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
    #[arg(short, long)]
    pub chat_id: Option<String>,

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
