{ config, lib, pkgs, ... }:
let
  inherit (lib) mkEnableOption mkIf mkOption types;
  cfg = config.programs.telegram-cli;
  telegramCli = pkgs.callPackage ../default.nix { };

  # Unset (null) options are left out so that the CLI falls back to the environment.
  configFile = pkgs.writeText "telegram-cli-config.json" (builtins.toJSON (lib.filterAttrs (_: v: v != null) {
    Prefix = cfg.prefix;
    Postfix = cfg.postfix;
    DefaultChatId = cfg.defaultChatId;
    DefaultChatIdFile = cfg.defaultChatIdFile;
    BotToken = cfg.botToken;
    BotTokenFile = cfg.botTokenFile;
    Proxy = cfg.proxy;
    ProxyFile = cfg.proxyFile;
    DefaultFormat = cfg.defaultFormat;
  }));
in
{
  options.programs.telegram-cli = {
    enable = mkEnableOption "telegram-cli";

    package = mkOption {
      type = types.package;
      default = telegramCli;
      description = "Package to use for telegram-cli";
    };

    prefix = mkOption {
      type = types.str;
      default = "";
      description = "The prefix to use in text messages";
    };

    postfix = mkOption {
      type = types.str;
      default = "";
      description = "The postfix to use in text messages";
    };

    defaultChatId = mkOption {
      type = types.nullOr types.str;
      default = null;
      description = "The default chat ID for telegram-cli";
    };

    defaultChatIdFile = mkOption {
      type = types.nullOr types.str;
      default = null;
      example = "/run/agenix/telegram-chat-id";
      description = ''
        Runtime path of a file containing the default chat ID, e.g.
        `config.sops.secrets.telegram-chat-id.path` or `config.age.secrets.telegram-chat-id.path`.
        Use a string, not a Nix path, so the file is never copied to the store.
      '';
    };

    botToken = mkOption {
      type = types.nullOr types.str;
      default = null;
      description = ''
        The Telegram bot token to send messages with.
        Warning: this ends up in the world-readable Nix store. Prefer `botTokenFile`.
      '';
    };

    botTokenFile = mkOption {
      type = types.nullOr types.str;
      default = null;
      example = "/run/agenix/telegram-bot-token";
      description = ''
        Runtime path of a file containing the bot token, e.g.
        `config.sops.secrets.telegram-bot-token.path` (sops-nix) or
        `config.age.secrets.telegram-bot-token.path` (agenix).
        Use a string, not a Nix path, so the file is never copied to the store.
      '';
    };

    proxy = mkOption {
      type = types.nullOr types.str;
      default = null;
      example = "socks5h://127.0.0.1:1080";
      description = ''
        Proxy URL (http, https, socks5 or socks5h). When null, telegram-cli honours the
        HTTPS_PROXY / HTTP_PROXY / ALL_PROXY / NO_PROXY environment variables.
        Do not put credentials here (Nix store); use `proxyFile`.
      '';
    };

    proxyFile = mkOption {
      type = types.nullOr types.str;
      default = null;
      example = "/run/agenix/telegram-proxy";
      description = "Runtime path of a file containing the proxy URL (may include credentials).";
    };

    defaultFormat = mkOption {
      type = types.enum [ "Html" "MarkdownV2" "No" ];
      default = "MarkdownV2";
      description = "The default format to use when sending text messages";
    };
  };

  config = mkIf cfg.enable {
    assertions = [
      {
        assertion = cfg.botToken == null || cfg.botTokenFile == null;
        message = "programs.telegram-cli: set only one of botToken and botTokenFile.";
      }
      {
        assertion = cfg.defaultChatId == null || cfg.defaultChatIdFile == null;
        message = "programs.telegram-cli: set only one of defaultChatId and defaultChatIdFile.";
      }
      {
        assertion = cfg.proxy == null || cfg.proxyFile == null;
        message = "programs.telegram-cli: set only one of proxy and proxyFile.";
      }
    ];

    home.packages = [ cfg.package ];
    home.file.".config/telegram-cli/config.json".source = configFile;
  };
}