{
  config,
  lib,
  pkgs,
  ...
}:
with lib; let
  cfg = config.services.tuxedo-rs;
in {
  options = {
    services.tuxedo-rs = {
      enable = mkEnableOption ''
        Rust utilities for interacting with hardware from TUXEDO Computers.
      '';

      tailor_gui.enable = mkEnableOption ''
        Alternative to Tuxedo Control Center, written in Rust.
      '';
    };
  };

  config = mkIf cfg.enable {
    hardware.tuxedo-keyboard.enable = true;

    services.dbus.packages = [pkgs.tailord];

    systemd.packages = [pkgs.tailord];

    environment = {
      etc = {
        "tailord/keyboard".source = ../tailord/default_configs/keyboard;
        "tailord/fan".source = ../tailord/default_configs/fan;
        "tailord/profiles".source = ../tailord/default_configs/profiles;
      };

      systemPackages = with pkgs; [
        tailord
        (mkIf cfg.tailor_gui.enable pkgs.tailor_gui)
      ];
    };
  };
}
