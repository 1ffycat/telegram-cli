use crate::config::Format;
use clap::{ArgAction, Parser};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    /// The message to send. Can be piped from stdin with the -s flag.
    #[arg(index = 1)]
    pub message: Option<String>,

    /// Override the default chat ID from the config file.
    #[arg(long, env = "TELEGRAM_CHAT_ID")]
    pub chat_id: Option<String>,

    /// Read the chat ID from a file (e.g. an agenix/sops-nix secret).
    #[arg(long, env = "TELEGRAM_CHAT_ID_FILE", value_name = "PATH")]
    pub chat_id_file: Option<PathBuf>,

    /// Override the default bot token from the config file.
    /// Prefer --bot-token-file or the environment: command-line arguments are visible to other users.
    #[arg(long, env = "TELEGRAM_BOT_TOKEN", hide_env_values = true)]
    pub bot_token: Option<String>,

    /// Read the bot token from a file (e.g. an agenix/sops-nix secret).
    #[arg(long, env = "TELEGRAM_BOT_TOKEN_FILE", value_name = "PATH")]
    pub bot_token_file: Option<PathBuf>,

    /// Route requests through this proxy (http://, https://, socks5:// or socks5h://).
    /// Overrides HTTPS_PROXY / HTTP_PROXY / ALL_PROXY and the config file.
    #[arg(
        long,
        env = "TELEGRAM_PROXY",
        hide_env_values = true,
        value_name = "URL"
    )]
    pub proxy: Option<String>,

    /// Ignore every proxy setting (CLI, config file and environment) and connect directly.
    #[arg(long, conflicts_with = "proxy")]
    pub no_proxy: bool,

    /// Use this config file instead of ~/.config/telegram-cli/config.json.
    #[arg(long, env = "TELEGRAM_CLI_CONFIG", value_name = "PATH")]
    pub config: Option<PathBuf>,

    /// Select message format.
    #[arg(long, value_enum)]
    pub format: Option<Format>,

    /// Wrap the message in a code block for a given language (e.g., `rust`).
    #[arg(short, long)]
    pub code: Option<String>,

    /// Send file(s) as documents. Can be used multiple times. Supports glob patterns.
    #[arg(short, long, action = ArgAction::Append)]
    pub file: Vec<PathBuf>,

    /// Send image(s) as photos. Can be used multiple times. Supports glob patterns.
    #[arg(short, long, action = ArgAction::Append)]
    pub photo: Vec<PathBuf>,

    /// Send video(s). Can be used multiple times. Supports glob patterns.
    #[arg(short, long, action = ArgAction::Append)]
    pub video: Vec<PathBuf>,

    /// Send audio file(s). Can be used multiple times. Supports glob patterns.
    #[arg(short, long, action = ArgAction::Append)]
    pub audio: Vec<PathBuf>,

    /// Take the message text from stdin.
    #[arg(short, long)]
    pub stdin: bool,
}
