use anyhow::{Context, Result};
use clap::ValueEnum;
use serde::Deserialize;
use std::path::{Path, PathBuf};

/// Contents of `config.json`. Every field is optional, and so is the file itself.
#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Config {
    pub bot_token: Option<String>,
    /// Path to a file containing the bot token (agenix, sops-nix, systemd credentials, ...).
    pub bot_token_file: Option<PathBuf>,
    pub default_chat_id: Option<String>,
    /// Path to a file containing the default chat ID.
    pub default_chat_id_file: Option<PathBuf>,
    /// Proxy URL. Overrides HTTPS_PROXY / HTTP_PROXY / ALL_PROXY.
    pub proxy: Option<String>,
    /// Path to a file containing the proxy URL (useful when it embeds credentials).
    pub proxy_file: Option<PathBuf>,
    pub prefix: Option<String>,
    pub postfix: Option<String>,
    pub default_format: Option<Format>,
}

#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum Format {
    Html,
    #[clap(name = "md")]
    MarkdownV2,
    #[clap(name = "no")]
    No,
}

impl std::fmt::Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Format::Html => write!(f, "HTML"),
            Format::MarkdownV2 => write!(f, "MarkdownV2"),
            Format::No => write!(f, "None"),
        }
    }
}

/// Loads the config file. A missing *default* file is not an error: the tool works
/// with CLI arguments and environment variables alone. An explicitly requested
/// file (`--config` / `TELEGRAM_CLI_CONFIG`) must exist.
pub fn load_config(explicit: Option<&Path>) -> Result<Config> {
    let (path, required) = match explicit {
        Some(p) => (p.to_path_buf(), true),
        None => match default_config_path() {
            Some(p) => (p, false),
            None => return Ok(Config::default()),
        },
    };

    let config_str = match std::fs::read_to_string(&path) {
        Ok(s) => s,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound && !required => {
            return Ok(Config::default());
        }
        Err(e) => {
            return Err(e).with_context(|| format!("Failed to read config file at {path:?}"));
        }
    };
    serde_json::from_str(&config_str)
        .with_context(|| format!("Failed to parse config file at {path:?}"))
}

fn default_config_path() -> Option<PathBuf> {
    Some(
        dirs::home_dir()?
            .join(".config")
            .join("telegram-cli")
            .join("config.json"),
    )
}

/// One place a setting can come from, in precedence order.
pub enum Source<'a> {
    Value(Option<&'a str>),
    File(Option<&'a Path>),
}

/// Returns the first non-empty source. Files are read at call time and trimmed, so
/// secrets managed by agenix/sops-nix never need to be stored in the config itself.
pub fn resolve(name: &str, sources: &[Source]) -> Result<Option<String>> {
    for source in sources {
        match source {
            Source::Value(Some(v)) if !v.trim().is_empty() => {
                return Ok(Some(v.trim().to_string()));
            }
            Source::File(Some(path)) if !path.as_os_str().is_empty() => {
                let content = std::fs::read_to_string(path)
                    .with_context(|| format!("Failed to read the {name} file at {path:?}"))?;
                let content = content.trim();
                if content.is_empty() {
                    anyhow::bail!("The {name} file at {path:?} is empty");
                }
                return Ok(Some(content.to_string()));
            }
            _ => {}
        }
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legacy_config_still_parses() {
        let c: Config = serde_json::from_str(
            r#"{"BotToken":"1:A","DefaultChatId":"-100","Prefix":"x","Postfix":null,"DefaultFormat":"MarkdownV2"}"#,
        )
        .unwrap();
        assert_eq!(c.bot_token.as_deref(), Some("1:A"));
        assert_eq!(c.default_format, Some(Format::MarkdownV2));
        assert!(c.bot_token_file.is_none() && c.proxy.is_none());
    }

    #[test]
    fn empty_values_fall_through_to_next_source() {
        let got = resolve("t", &[Source::Value(Some("")), Source::Value(Some(" b "))]).unwrap();
        assert_eq!(got.as_deref(), Some("b"));
    }

    #[test]
    fn file_secrets_are_trimmed_and_missing_files_error() {
        let path = std::env::temp_dir().join("telegram-cli-test-secret");
        std::fs::write(&path, "123:ABC\n").unwrap();
        let got = resolve("t", &[Source::File(Some(&path))]).unwrap();
        assert_eq!(got.as_deref(), Some("123:ABC"));
        std::fs::remove_file(&path).unwrap();
        assert!(resolve("t", &[Source::File(Some(&path))]).is_err());
    }

    #[test]
    fn nothing_configured_resolves_to_none() {
        assert_eq!(
            resolve("t", &[Source::Value(None), Source::File(None)]).unwrap(),
            None
        );
    }
}
