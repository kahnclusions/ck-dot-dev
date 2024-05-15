{
  description = "Build a cargo project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    fenix = {
      url = "github:nix-community/fenix/monthly";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.rust-analyzer-src.follows = "";
    };

    flake-utils.url = "github:numtide/flake-utils";

    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, crane, fenix, flake-utils, advisory-db, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};

        inherit (pkgs) lib;

        name = "ck-dot-dev";

        rustNightly = fenix.packages.${system}.fromToolchainFile {
          file = ./rust-toolchain.toml;
          sha256 = "sha256-bMKEd1LmaGfG8ntGJoqdrDFiH5XXBYoPDCaY82g9roY=";
        };

        craneLib = crane.lib.${system}.overrideToolchain rustNightly;
        src = craneLib.cleanCargoSource (craneLib.path ./.);

        # Common arguments can be set here to avoid repeating them later
        commonArgs = {
          inherit src;
          strictDeps = true;

          buildInputs = with pkgs; [
            cargo-leptos
            binaryen
            tailwindcss
            # Add additional build inputs here
          ] ++ lib.optionals pkgs.stdenv.isDarwin [
            # Additional darwin specific inputs can be set here
            pkgs.libiconv
          ];

          # Additional environment variables can be set directly
          # MY_CUSTOM_VAR = "some value";
        };

        craneLibLLvmTools = craneLib.overrideToolchain
          (fenix.packages.${system}.complete.withComponents [
            "cargo"
            "llvm-tools"
            "rustc"
          ]);

        # Build *just* the cargo dependencies, so we can reuse
        # all of that work (e.g. via cachix) when running in CI
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        srcFilter = path: type:
                (lib.hasSuffix "\.html" path) ||
                (lib.hasSuffix "tailwind.config.js" path) ||
                # Example of a folder for images, icons, etc
                (lib.hasInfix "/public/" path) ||
                (lib.hasInfix "/style/" path) ||
                # Default filter from crane (allow .rs files)
                (craneLib.filterCargoSources path type)
              ;

        # Build the actual crate itself, reusing the dependency
        # artifacts from above.
        ck-dot-dev = craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;
          src = lib.cleanSourceWith {
            src = craneLib.path ./.; # The original, unfiltered source
            filter = srcFilter;
          };

          buildPhaseCargoCommand = "cargo leptos build --release -vv";
          nativeBuildInputs = with pkgs; [
            cargo-leptos
            tailwindcss
            binaryen
            makeWrapper
          ];
          installPhaseCommand = ''
            mkdir -p $out/bin
            cp target/release/${name} $out/bin/
            cp -r target/site $out/bin/
            wrapProgram $out/bin/${name} \
              --set LEPTOS_SITE_ROOT $out/bin/site
          '';
        });
      in
      {
        checks = {
          # Build the crate as part of `nix flake check` for convenience
          inherit ck-dot-dev;

          # Run clippy (and deny all warnings) on the crate source,
          # again, reusing the dependency artifacts from above.
          #
          # Note that this is done as a separate derivation so that
          # we can block the CI if there are issues here, but not
          # prevent downstream consumers from building our crate by itself.
          ck-dot-dev-clippy = craneLib.cargoClippy (commonArgs // {
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "--all-targets -- --deny warnings";
          });

          ck-dot-dev-doc = craneLib.cargoDoc (commonArgs // {
            inherit cargoArtifacts;
          });

          # Check formatting
          ck-dot-dev-fmt = craneLib.cargoFmt {
            inherit src;
          };

          # Audit dependencies
          ck-dot-dev-audit = craneLib.cargoAudit {
            inherit src advisory-db;
          };

          # Audit licenses
          ck-dot-dev-deny = craneLib.cargoDeny {
            inherit src;
          };

          # Run tests with cargo-nextest
          # Consider setting `doCheck = false` on `ck-dot-dev` if you do not want
          # the tests to run twice
          ck-dot-dev-nextest = craneLib.cargoNextest (commonArgs // {
            inherit cargoArtifacts;
            partitions = 1;
            partitionType = "count";
          });
        };

        packages = {
          default = ck-dot-dev;
        } // lib.optionalAttrs (!pkgs.stdenv.isDarwin) {
          ck-dot-dev-llvm-coverage = craneLibLLvmTools.cargoLlvmCov (commonArgs // {
            inherit cargoArtifacts;
          });
        };

        apps.default = flake-utils.lib.mkApp {
          drv = ck-dot-dev;
        };

        devShells.default = craneLib.devShell {
          # Inherit inputs from checks.
          checks = self.checks.${system};

          # Additional dev-shell environment variables can be set directly
          # MY_CUSTOM_DEVELOPMENT_VAR = "something else";

          # Extra inputs can be added here; cargo and rustc are provided by default.
          packages = with pkgs; [
            cargo-leptos
            binaryen
            tailwindcss
            just
            cargo-watch
          ];
        };
      });
}
