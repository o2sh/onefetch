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
