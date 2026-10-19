mod cli;
mod config;
mod telegram;

use anyhow::{Context, Result};
use clap::Parser;
use cli::Cli;
use config::{Format, Source, load_config, resolve};
use glob::glob;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use telegram::{Media, ProxyMode, TelegramClient};

/// Expands a user-supplied path: an existing path is used as-is (so names containing
/// glob characters like `[` still work), anything else is treated as a glob pattern.
fn expand(path: &Path) -> Result<Vec<PathBuf>> {
    if path.exists() {
        return Ok(vec![path.to_path_buf()]);
    }
    let pattern = path.to_string_lossy();
    let mut found = Vec::new();
    for entry in
        glob(&pattern).with_context(|| format!("Failed to read glob pattern: {pattern}"))?
    {
        match entry {
            Ok(p) => found.push(p),
            Err(e) => eprintln!("Warning: Skipping invalid path from glob: {e}"),
        }
    }
    if found.is_empty() {
        eprintln!("Warning: No files match {pattern}");
    }
    Ok(found)
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    rustls::crypto::ring::default_provider()
        .install_default()
        .map_err(|_| anyhow::anyhow!("Failed to install the TLS crypto provider"))?;

    let cli = Cli::parse();
    let config = load_config(cli.config.as_deref())?;

    // Precedence: CLI argument > environment variable (both handled by clap) > config file.
    let bot_token = resolve(
        "bot token",
        &[
            Source::Value(cli.bot_token.as_deref()),
            Source::File(cli.bot_token_file.as_deref()),
            Source::Value(config.bot_token.as_deref()),
            Source::File(config.bot_token_file.as_deref()),
        ],
    )?
    .context(
        "No bot token provided. Use --bot-token, --bot-token-file, TELEGRAM_BOT_TOKEN, \
         TELEGRAM_BOT_TOKEN_FILE, or set BotToken/BotTokenFile in the config file.",
    )?;

    let chat_id = resolve(
        "chat ID",
        &[
            Source::Value(cli.chat_id.as_deref()),
            Source::File(cli.chat_id_file.as_deref()),
            Source::Value(config.default_chat_id.as_deref()),
            Source::File(config.default_chat_id_file.as_deref()),
        ],
    )?
    .context(
        "No chat ID provided. Use --chat-id, --chat-id-file, TELEGRAM_CHAT_ID, \
         TELEGRAM_CHAT_ID_FILE, or set DefaultChatId/DefaultChatIdFile in the config file.",
    )?;

    let proxy = if cli.no_proxy {
        ProxyMode::Disabled
    } else {
        let explicit = resolve(
            "proxy",
            &[
                Source::Value(cli.proxy.as_deref()),
                Source::Value(config.proxy.as_deref()),
                Source::File(config.proxy_file.as_deref()),
            ],
        )?;
        explicit.map_or(ProxyMode::System, ProxyMode::Url)
    };

    let client = TelegramClient::new(bot_token, proxy)?;
    let chat_id = chat_id.as_str();
    let format = cli.format.or(config.default_format);

    let message_from_cli = if cli.stdin {
        let mut buffer = String::new();
        io::stdin()
            .read_to_string(&mut buffer)
            .context("Failed to read from stdin")?;
        (!buffer.trim().is_empty()).then_some(buffer)
    } else {
        cli.message
    };

    // Process the message content if it exists
    let (final_text, final_parse_mode) = if let Some(content) = message_from_cli {
        let content = format!(
            "{}{}{}",
            config.prefix.as_deref().unwrap_or_default(),
            content,
            config.postfix.as_deref().unwrap_or_default()
        );

        if let Some(lang) = cli.code {
            if format == Some(Format::Html) {
                let escaped_content = content
                    .replace('&', "&amp;")
                    .replace('<', "&lt;")
                    .replace('>', "&gt;");
                (
                    Some(format!(
                        "<pre><code class=\"language-{lang}\">{escaped_content}</code></pre>"
                    )),
                    Some("HTML"),
                )
            } else {
                (
                    Some(format!("```{lang}\n{content}\n```")),
                    Some("MarkdownV2"),
                )
            }
        } else {
            let parse_mode = match format {
                Some(Format::Html) => Some("HTML"),
                Some(Format::MarkdownV2) => Some("MarkdownV2"),
                _ => None,
            };
            (Some(content), parse_mode)
        }
    } else {
        (None, None)
    };

    let caption = final_text.as_deref();
    let mut files_sent_count = 0;

    for (media, paths) in [
        (Media::Document, &cli.file),
        (Media::Photo, &cli.photo),
        (Media::Video, &cli.video),
        (Media::Audio, &cli.audio),
    ] {
        for path in paths {
            for file in expand(path)? {
                client
                    .send_media(media, chat_id, &file, caption, final_parse_mode)
                    .await?;
                files_sent_count += 1;
            }
        }
    }

    // If NO files were sent and we have text, send it as a standard message.
    if files_sent_count == 0
        && let Some(text_to_send) = final_text
    {
        client
            .send_message(chat_id, &text_to_send, final_parse_mode)
            .await?;
    }

    Ok(())
}
