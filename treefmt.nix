{ pkgs, ... }:
{
  projectRootFile = "flake.nix";

  programs = {
    taplo.enable = true;
    rustfmt = {
      enable = true;
      edition = "2021";
    };
    nixfmt.enable = true;
  };
}
