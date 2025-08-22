# Telegram CLI

A simple, fast, and powerful command-line tool written in Rust to interact with the Telegram Bot API. This tool allows you to send messages, files, photos, videos, and audio directly from your terminal.

[![License: MPL 2.0](https://img.shields.io/badge/License-MPL_2.0-brightgreen.svg)](https://opensource.org/licenses/MPL-2.0)

## Features

*   Send text messages, documents, photos, videos, and audio.
*   Supports glob patterns for sending multiple files at once.
*   Read message content from standard input.
*   Format messages with Markdown or HTML.
*   Wrap messages in code blocks with syntax highlighting.
*   Configure default bot token and chat ID in a config.json file under `~/.config/telegram-cli/`.
*   Configure the prefix and postfix to automatically add to your text messages.
*   Overrides for bot token and chat ID via command-line options.
*   Available as a Nix package and flake for easy installation on NixOS and other systems.
*   Includes a `home-manager` module for declarative configuration.

## Installation

### With Nix

For users of the Nix package manager, you can install `telegram-cli` declaratively.

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
      botToken = "<your-telegram-bot-token>";      # Optional: Default bot token
      defaultChatId = "<your-telegram-chat-id>";   # Optional: Default chat ID
      prefix = "NixOS Alert:\n";                   # Optional: Prefix for text messages
      postfix = "\n-- Sent from my NixOS machine"; # Optional: Postfix for text messages
      defaultFormat = "MarkdownV2";                # Optional: "Html", "MarkdownV2", or "No". Defaults to "MarkdownV2".
    };
    ```

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
|      | `--bot-token <BOT_TOKEN>` | Override the default bot token from the config file.                           |
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

## License

This project is licensed under the **Mozilla Public License 2.0 (MPL-2.0)**.

This license is a "weak copyleft" license. It requires that any modifications to MPL-licensed source code files must also be licensed under the MPL-2.0. However, it allows you to combine the MPL-licensed code with code under other licenses (including proprietary ones) in a larger work.

A full copy of the license is available in the [./LICENSE](LICENSE) file in this repository.

## Contributing

Contributions are welcome! Please feel free to open an issue or submit a pull request.