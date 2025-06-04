{
  description = ''
    Git repository summary on your terminal
  '';

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane.url = "github:ipetkov/crane";
    flake-utils.url = "github:numtide/flake-utils";

    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      crane,
      flake-utils,
      advisory-db,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};

        inherit (pkgs) lib;
        craneLib = crane.mkLib pkgs;

        # This filter prevent project from being rebuilded then changing
        # unrelated files ,e.g. README
        filter' =
          path: _type:
          builtins.match (lib.concatStringsSep "|" [
            ".*tera"
            ".*yaml"
            ".*zstd"
            ".*snap"
            ".*sh"
            ".+LICENSE.md"
          ]) path != null;
        filter = path: type: (filter' path type) || (craneLib.filterCargoSources path type);
        src = lib.cleanSourceWith {
        src = ./.;
          inherit filter;
          name = "source";
        };

        # Common arguments can be set here to avoid repeating them later
        common = {
          inherit src;
          strictDeps = true;

          # Bunch of libraries required for package proper work
          buildInputs =
            with pkgs;
            [
              # package dependencies
              zstd
            ]
            ++ lib.optionals pkgs.stdenv.isDarwin (
              with pkgs;
              [
              # additional dependencies on Darwin systems
              CoreFoundation
              libresolv
              Security
              ]
            );
          # Software required for project build
          nativeBuildInputs = with pkgs; [
            cmake
            pkg-config
          ];
          # Tools required for checks
          nativeCheckInputs = with pkgs; [ git ];

          # Additional environment variables can be set directly
          # MY_CUSTOM_VAR = "some value";
        };

        # Build dependencies only, so we will be able to reuse them further
        cargoArtifacts = craneLib.buildDepsOnly common;

        # Build the actual crate itself, reusing the dependency
        # artifacts from above.
        build = craneLib.buildPackage (common // { inherit cargoArtifacts; });
      in
      {
        checks = {
          # Build the crate as part of `nix flake check` for convenience
          inherit build;

          # Run clippy (and deny all warnings) on the crate source,
          # again, reusing the dependency artifacts from above.
          clippy = craneLib.cargoClippy (
            common
            // {
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "--all-targets -- --deny warnings";
            }
          );

          doc = craneLib.cargoDoc (common // { inherit cargoArtifacts; });

          # Check formatting
          fmt = craneLib.cargoFmt { inherit src; };

          tomlFmt = craneLib.taploFmt {
            src = pkgs.lib.sources.sourceFilesBySuffices src [ ".toml" ];
            # taplo arguments can be further customized below as needed
            # taploExtraArgs = "--config ./taplo.toml";
          };

          # Audit dependencies
          audit = craneLib.cargoAudit { inherit src advisory-db; };

          # Audit licenses
          deny = craneLib.cargoDeny { inherit src; };

          # Run tests with cargo-nextest
          # Consider setting `doCheck = false` on `my-crate` if you do not want
          # the tests to run twice
          nextest = craneLib.cargoNextest (
            common
            // {
            inherit cargoArtifacts;
            partitions = 1;
            partitionType = "count";
            cargoNextestPartitionsExtraArgs = "--no-tests=pass";
            }
          );
        };

        packages = rec {
          onefetch-debug = craneLib.buildPackage (
            common
            // {
              inherit cargoArtifacts;
              doCheck = false;
              CARGO_PROFILE = "dev";
            }
          );
          onefetch = craneLib.buildPackage (
            common
            // {
              inherit cargoArtifacts;
              doCheck = false;
            }
          );
          default = onefetch-debug;
        };

        apps.default = flake-utils.lib.mkApp { drv = build; };

        devShells.default = craneLib.devShell {
          # Inherit inputs from checks.
          checks = self.checks.${system};

          # Additional dev-shell environment variables can be set directly
          # MY_CUSTOM_DEVELOPMENT_VAR = "something else";

          # Extra inputs can be added here; cargo and rustc are provided by default.
          packages = with pkgs; [
            # pkgs.ripgrep
            nixd
            nixfmt-rfc-style
          ];
        };
      }
    );
  # Sets substituters to avoid locally building something already built
  nixConfig = {
    extra-substituters = [
      "https://crane.cachix.org"
      "https://cache.garnix.io"
    ];
    extra-trusted-public-keys = [
      "crane.cachix.org-1:8Scfpmn9w+hGdXH/Q9tTLiYAE/2dnJYRJP7kl80GuRk="
      "cache.garnix.io:CTFPyKSLcx5RMJKfLo5EEPUObbA78b0YQ2DTCJXqr9g="
    ];
  };
}
