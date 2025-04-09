[Nix](https://nixos.org/) is a package manager that uses a purely functional approach to dependency management. Packages in Nix are built and run in isolated, reproducible environments. This tutorial walks you through setting up a development environment for Onefetch using Nix.

> This guide assumes you already have Nix [installed](https://nixos.org/download.html#nix-quick-install) on your system.

## Setup

To begin, create a `flake.nix` file with the following content:

```nix
{
  description = "onefetch";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = {
    self,
    nixpkgs,
    ...
  }: let
  forAllSystems = fn: nixpkgs.lib.genAttrs [
    "aarch64-darwin"
    "aarch64-linux"
    "i686-linux"
    "x86_64-linux"
    "x86_64-darwin"
  ] (system: fn nixpkgs.legacyPackages.${system});
  in {
    devShells = forAllSystems (pkgs: {
      default = pkgs.mkShell {
        name = "onefetch";
        packages = with pkgs; [
          cargo
          rustc
          clippy
          rustfmt
          rust-analyzer
          cmake
        ];
        RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
      };
    });
  };
}
```

then enter the development shell:

```bash
nix develop
```
