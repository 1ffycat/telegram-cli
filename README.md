# Telegram CLI

[![Crates.io](https://img.shields.io/crates/v/telegram-cli.svg)](https://crates.io/crates/telegram-cli)
[![License: MPL 2.0](https://img.shields.io/badge/License-MPL_2.0-brightgreen.svg)](https://opensource.org/licenses/MPL-2.0)

A simple, fast, and powerful command-line tool written in Rust to interact with the Telegram Bot API. This tool allows you to send messages, files, photos, videos, and audio directly from your terminal.

## Features

*   Send text messages, documents, photos, videos, and audio.
*   Supports glob patterns for sending multiple files at once.
*   Read message content from standard input.
*   Format messages with Markdown or HTML.
*   Wrap messages in code blocks with syntax highlighting.
*   Works with a config file, environment variables, CLI arguments, or any mix of them. The config file is optional.
*   Secrets can be read from files, so [agenix](https://github.com/ryantm/agenix) / [sops-nix](https://github.com/Mic92/sops-nix) work out of the box.
*   Honours `HTTPS_PROXY` / `HTTP_PROXY` / `ALL_PROXY` / `NO_PROXY`, with explicit HTTP and SOCKS5 proxy options.
*   Configure the prefix and postfix to automatically add to your text messages.
*   Overrides for bot token and chat ID via command-line options.
*   Available as a Nix package and flake for easy installation on NixOS and other systems.
*   Includes a `home-manager` module for declarative configuration.

## Installation

### From Crates.io

If you have Rust and Cargo installed, you can install `telegram-cli` directly from crates.io:

```bash
cargo install telegram-cli
```

### With Nix

For NixOS users, you can install `telegram-cli` declaratively.

**Via Flakes:**

1.  Add the flake to your `flake.nix` inputs:

    ```nix
    {
      description = "My NixOS configuration";
      inputs = {
        nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
        telegram-cli = {
          url = "github:1ffycat/telegram-cli";
          inputs.nixpkgs.follows = "nixpkgs";
        };
      };
      # ...
    }
    ```

2.  Add it to your `environment.systemPackages` in your `configuration.nix`:

    ```nix
    environment.systemPackages = [
      inputs.telegram-cli.packages.${system}.telegram-cli
    ];
    ```

**home-manager Module:**

The flake also exposes a `home-manager` module for more granular, user-level configuration.

1.  Import the module in your `home.nix`:

    ```nix
    { inputs, ... }: {
      imports = [
        inputs.telegram-cli.homeManagerModules.default
      ];
    }
    ```

2.  Configure the program:

    ```nix
    programs.telegram-cli = {
      enable = true;
      botTokenFile = config.age.secrets.telegram-bot-token.path; # Recommended: runtime path of a secret (agenix / sops-nix), see below
      defaultChatId = "<your-telegram-chat-id>";   # Optional: Default chat ID (or defaultChatIdFile)
      # botToken = "<your-telegram-bot-token>";    # Works, but ends up in the world-readable Nix store
      # proxy = "socks5h://127.0.0.1:1080";        # Optional: see "Proxy" below (or proxyFile)
      prefix = "NixOS Alert:\n";                   # Optional: Prefix for text messages
      postfix = "\n-- Sent from my NixOS machine"; # Optional: Postfix for text messages
      defaultFormat = "MarkdownV2";                # Optional: "Html", "MarkdownV2", or "No". Defaults to "MarkdownV2".
    };
    ```

**Secrets with agenix / sops-nix:**

Point `botTokenFile` (and, if you like, `defaultChatIdFile` / `proxyFile`) at the *runtime path* of the decrypted secret. Use a string, not a Nix path literal, so nothing is copied into the store. The file is read every time `telegram-cli` runs.

```nix
# agenix (home-manager)
age.secrets.telegram-bot-token.file = ./secrets/telegram-bot-token.age;
programs.telegram-cli.botTokenFile = config.age.secrets.telegram-bot-token.path;

# sops-nix (home-manager)
sops.secrets.telegram-bot-token = { };
programs.telegram-cli.botTokenFile = config.sops.secrets.telegram-bot-token.path;
```

## Configuration

Every setting can come from three places. When several are present, the first one wins:

1.  **CLI arguments**
2.  **Environment variables**
3.  **Config file** (`~/.config/telegram-cli/config.json`, or the file given by `--config` / `TELEGRAM_CLI_CONFIG`)

The config file is **optional**: `telegram-cli` works with nothing but environment variables or flags (handy for containers and CI), and never creates files or directories on its own.

| Setting      | CLI                | Environment                          | Config file key     |
| :----------- | :----------------- | :----------------------------------- | :------------------ |
| Bot token    | `--bot-token`      | `TELEGRAM_BOT_TOKEN`                 | `BotToken`          |
| Token file   | `--bot-token-file` | `TELEGRAM_BOT_TOKEN_FILE`            | `BotTokenFile`      |
| Chat ID      | `--chat-id`        | `TELEGRAM_CHAT_ID`                   | `DefaultChatId`     |
| Chat ID file | `--chat-id-file`   | `TELEGRAM_CHAT_ID_FILE`              | `DefaultChatIdFile` |
| Proxy        | `--proxy`          | `TELEGRAM_PROXY` (or `HTTPS_PROXY`)  | `Proxy`             |
| Proxy file   |                    |                                      | `ProxyFile`         |
| Config path  | `--config`         | `TELEGRAM_CLI_CONFIG`                |                     |

Within one source, a direct value beats a `...File` value. Files are trimmed, so a trailing newline is fine. Empty values are ignored.

Other config keys: `Prefix`, `Postfix` and `DefaultFormat` (`Html`, `MarkdownV2` or `No`).

Example `config.json`:

```json
{
  "BotTokenFile": "/run/agenix/telegram-bot-token",
  "DefaultChatId": "-1001234567890",
  "Prefix": "🚀 ",
  "DefaultFormat": "MarkdownV2"
}
```

> **Tip:** command-line arguments are visible to other users via `ps`. Prefer `--bot-token-file`, the environment, or the config file for the bot token.

### Proxy

`telegram-cli` respects the standard proxy environment variables (`HTTPS_PROXY`, `HTTP_PROXY`, `ALL_PROXY`, `NO_PROXY`). Telegram is reached over HTTPS, so `HTTPS_PROXY` is the relevant one:

```bash
HTTPS_PROXY=http://proxy.example:3128 telegram-cli "Hello"
```

To set it explicitly, use `--proxy`, `TELEGRAM_PROXY` or the `Proxy` / `ProxyFile` config keys. An explicit proxy takes priority over the standard variables. `http://`, `https://`, `socks5://` and `socks5h://` (proxy-side DNS) URLs are supported, and credentials can be embedded (`http://user:pass@host:3128`; keep those in `ProxyFile`). Use `--no-proxy` to ignore every proxy setting and connect directly.

## Usage

```
telegram-cli [OPTIONS] [MESSAGE]
```

### Arguments

*   `[MESSAGE]` The message to send. This can also be piped from stdin when using the `-s` flag.

### Options

| Flag | Long Flag                 | Description                                                                    |
| :--- | :------------------------ | :----------------------------------------------------------------------------- |
|      | `--chat-id <CHAT_ID>`     | Override the default chat ID from the config file.                             |
|      | `--chat-id-file <PATH>`   | Read the chat ID from a file.                                                  |
|      | `--bot-token <BOT_TOKEN>` | Override the default bot token from the config file.                           |
|      | `--bot-token-file <PATH>` | Read the bot token from a file (agenix / sops-nix).                            |
|      | `--proxy <URL>`           | Use this HTTP(S) / SOCKS5 proxy.                                               |
|      | `--no-proxy`              | Ignore all proxy settings.                                                     |
|      | `--config <PATH>`         | Use a different config file.                                                   |
|      | `--format <FORMAT>`       | Select message format. Possible values: `html`, `md`, `no`.                    |
| `-c` | `--code <CODE>`           | Wrap the message in a code block for a given language (e.g., `rust`).          |
| `-f` | `--file <FILE>`           | Send file(s) as documents. Can be used multiple times. Supports glob patterns. |
| `-p` | `--photo <PHOTO>`         | Send image(s) as photos. Can be used multiple times. Supports glob patterns.   |
| `-v` | `--video <VIDEO>`         | Send video(s). Can be used multiple times. Supports glob patterns.             |
| `-a` | `--audio <AUDIO>`         | Send audio file(s). Can be used multiple times. Supports glob patterns.        |
| `-s` | `--stdin`                 | Take the message text from stdin.                                              |
| `-h` | `--help`                  | Print help information.                                                        |
| `-V` | `--version`               | Print version information.                                                     |

### Examples

**Sending a simple text message:**

```bash
telegram-cli "Hello, world!"
```

**Sending a message with Markdown formatting:**

```bash
telegram-cli --format md "This is a *bold* and _italic_ message."
```

**Piping a message from another command:**

```bash
echo "This message comes from stdin" | telegram-cli -s
```

**Sending a file:**

```bash
telegram-cli -f ./my-document.pdf "Here is the document you requested."
```

**Sending multiple photos using a glob pattern:**

```bash
telegram-cli -p "images/*.jpg" "A collection of beautiful photos."
```

**Sending a code snippet:**

```bash
telegram-cli -c rust 'fn main() { println!("Hello from Rust!"); }'
```

**Using a different bot token and chat ID:**

```bash
telegram-cli --bot-token "YOUR_OTHER_BOT_TOKEN" --chat-id "ANOTHER_CHAT_ID" "Message to a different chat."
```

**Using only environment variables (no config file):**

```bash
TELEGRAM_BOT_TOKEN="..." TELEGRAM_CHAT_ID="..." telegram-cli "Deploy finished"
```

## Building

Requires Rust 1.88 or newer: `cargo build --release`.

## License

This project is licensed under the **Mozilla Public License 2.0 (MPL-2.0)**.

This license is a "weak copyleft" license. It requires that any modifications to MPL-licensed source code files must also be licensed under the MPL-2.0. However, it allows you to combine the MPL-licensed code with code under other licenses (including proprietary ones) in a larger work.

A full copy of the license is available in the [./LICENSE](LICENSE) file in this repository.

## Contributing

Contributions are welcome! Please feel free to open an issue or submit a pull request.