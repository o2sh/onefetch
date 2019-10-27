{
  system ? builtins.currentSystem,
  pkgs ? import <nixpkgs> { inherit system; },
  ...
}:

with pkgs;
let
  rustPlatform = makeRustPlatform {
    rustc = cargo;
    cargo = cargo;
  };
in
  rustPlatform.buildRustPackage rec {
    name = "onefetch-${version}";
    version = "1.6.5";
    
    src = ./.;
    cargoSha256 = "0saxy27m08lcb9gr71n2m283frdhaixiac6d7fvdwj5qv228kjdb";
    buildInputs = [ ];
    CARGO_HOME = "$(mktemp -d cargo-home.XXX)";

    meta = with lib; {
      homepage = https://github.com/o2sh/onefetch;
      description = ''displays information about your Git project directly on your terminal'';
      license = licenses.mit;
      # maintainers = with maintainers; [ kloenk ]; # I'm not a maintainer yet, but maybe I will submit this package, and then I will be a maintainer
    };
  }
