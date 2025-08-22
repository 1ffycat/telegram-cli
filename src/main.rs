mod cli;
mod config;
mod telegram;

use anyhow::{Context, Result};
use clap::Parser;
use cli::Cli;
use config::load_config;
use glob::glob;
use std::io::{self, Read};
use std::process;
use telegram::TelegramClient;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = load_config()?;

    // Check for valid chat_id, exit if both CLI and config default are missing or empty
    let bot_token = match (cli.bot_token.as_deref(), &config.bot_token) {
        (Some(bot_token), _) if !bot_token.is_empty() => bot_token,
        (_, Some(default_id)) if !default_id.is_empty() => default_id,
        _ => {
            eprintln!(
                "Error: No valid bot token provided. Please specify a bot token via CLI arguments or set a non-empty default in the config file."
            );
            process::exit(1);
        }
    };

    let client = TelegramClient::new(bot_token.to_string());

    // Check for valid chat_id, exit if both CLI and config default are missing or empty
    let chat_id = match (cli.chat_id.as_deref(), &config.default_chat_id) {
        (Some(chat_id), _) if !chat_id.is_empty() => chat_id,
        (_, Some(default_id)) if !default_id.is_empty() => default_id,
        _ => {
            eprintln!(
                "Error: No valid chat ID provided. Please specify a chat ID via CLI arguments or set a non-empty default in the config file."
            );
            process::exit(1);
        }
    };
    let format = cli.format.or(config.default_format);

    let message_from_cli = if cli.stdin {
        let mut buffer = String::new();
        io::stdin()
            .read_to_string(&mut buffer)
            .context("Failed to read from stdin")?;
        if buffer.trim().is_empty() {
            None
        } else {
            Some(buffer)
        }
    } else {
        // Collect all positional arguments into one message.
        // This allows `telegram-cli hello world` without quotes.
        cli.message
    };

    // Process the message content if it exists
    let (final_text, final_parse_mode) = if let Some(mut content) = message_from_cli {
        let mut processed_content = String::new();
        if let Some(prefix) = &config.prefix {
            processed_content.push_str(prefix);
        }
        processed_content.push_str(&content);
        if let Some(postfix) = &config.postfix {
            processed_content.push_str(postfix);
        }
        content = processed_content;

        if let Some(lang) = cli.code {
            let (formatted_content, parse_mode) = if format == Some(config::Format::Html) {
                let escaped_content = content
                    .replace('&', "&amp;")
                    .replace('<', "&lt;")
                    .replace('>', "&gt;");
                (
                    format!(
                        "<pre><code class=\"language-{}\">{}</code></pre>",
                        lang, escaped_content
                    ),
                    Some("HTML"),
                )
            } else {
                (format!("```{}\n{}\n```", lang, content), Some("MarkdownV2"))
            };
            (Some(formatted_content), parse_mode)
        } else {
            let parse_mode = match format {
                Some(config::Format::Html) => Some("HTML"),
                Some(config::Format::MarkdownV2) => Some("MarkdownV2"),
                _ => None,
            };
            (Some(content), parse_mode)
        }
    } else {
        (None, None)
    };

    let caption = final_text.as_deref();
    let mut files_sent_count = 0;

    macro_rules! send_files {
        ($paths:expr, $sender:ident) => {
            for path in $paths {
                let path_str = &path.to_string_lossy();
                for entry in glob(path_str)
                    .with_context(|| format!("Failed to read glob pattern: {}", path_str))?
                {
                    match entry {
                        Ok(p) => {
                            client
                                .$sender(chat_id, &p, caption, final_parse_mode)
                                .await?;
                            files_sent_count += 1;
                        }
                        Err(e) => eprintln!("Warning: Skipping invalid path from glob: {}", e),
                    }
                }
            }
        };
    }

    send_files!(&cli.file, send_document);
    send_files!(&cli.photo, send_photo);
    send_files!(&cli.video, send_video);
    send_files!(&cli.audio, send_audio);

    // If NO files were sent and we have text, send it as a standard message.
    if files_sent_count == 0 {
        if let Some(text_to_send) = final_text {
            client
                .send_message(chat_id, &text_to_send, final_parse_mode)
                .await?;
        }
    }

    Ok(())
}
