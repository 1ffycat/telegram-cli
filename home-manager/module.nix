{ config, lib, pkgs, ... }:
let
  inherit (lib) mkEnableOption mkIf mkOption types;
  cfg = config.programs.telegram-cli;
  telegramCli = pkgs.callPackage ../default.nix { };

  configFile = pkgs.writeText "telegram-cli-config.json" (builtins.toJSON {
    Prefix = cfg.prefix;
    Postfix = cfg.postfix;
    DefaultChatId = cfg.defaultChatId;
    BotToken = cfg.botToken;
    DefaultFormat = cfg.defaultFormat;
  });
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

    botToken = mkOption {
      type = types.nullOr types.str;
      default = null;
      description = "The Telegram bot token to send messages with";
    };

    defaultFormat = mkOption {
      type = types.enum [ "Html" "MarkdownV2" "No" ];
      default = "MarkdownV2";
      description = "The default format to use when sending text messages";
    };
  };

  config = mkIf cfg.enable {
    home.packages = [ cfg.package ];
    home.file.".config/telegram-cli/config.json".source = configFile;
  };
}