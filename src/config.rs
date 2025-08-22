use anyhow::{Context, Result};
use clap::ValueEnum;
use serde::Deserialize;
use std::path::PathBuf;
use std::process;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Config {
    pub bot_token: Option<String>,
    pub default_chat_id: Option<String>,
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

pub fn load_config() -> Result<Config> {
    let config_path = get_config_path().context("Could not determine config path")?;
    if !config_path.exists() {
        anyhow::bail!(
            "Config file not found. Please create it at: {:?}\n\
            Example:\n\
            {{\n\
            \t\"BotToken\": \"123456:ABC-DEF1234ghIkl-zyx57W2v1u123ew11\",\n\
            \t\"DefaultChatId\": \"-1001234567890\",\n\
            \t\"Prefix\": \"🚀 \",\n\
            \t\"DefaultFormat\": \"MarkdownV2\"\n\
            }}",
            config_path
        );
    }
    let config_str = std::fs::read_to_string(&config_path)
        .with_context(|| format!("Failed to read config file at {:?}", config_path))?;
    let config: Config = serde_json::from_str(&config_str)
        .with_context(|| format!("Failed to parse config file at {:?}", config_path))?;

    // Check for empty BotToken or DefaultChatId
    if let Some(bot_token) = &config.bot_token {
        if bot_token.is_empty() {
            eprintln!(
                "Warning: BotToken is empty in config file at {:?}. \
                Please provide a valid default bot token or remove the empty BotToken field if intentional.",
                config_path
            );
            process::exit(1);
        }
    }
    if let Some(default_chat_id) = &config.default_chat_id {
        if default_chat_id.is_empty() {
            eprintln!(
                "Warning: DefaultChatId is empty in config file at {:?}. \
                Please provide a valid default chat ID or remove the empty DefaultChatId field if intentional.",
                config_path
            );
            process::exit(1);
        }
    }

    Ok(config)
}

fn get_config_path() -> Result<PathBuf> {
    let home_dir = dirs::home_dir().context("Failed to get home directory")?;
    let config_dir = home_dir.join(".config").join("telegram-cli");
    std::fs::create_dir_all(&config_dir)
        .with_context(|| format!("Failed to create config directory at {:?}", config_dir))?;
    Ok(config_dir.join("config.json"))
}
