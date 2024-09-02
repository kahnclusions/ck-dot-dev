{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    crane = {
      url = "github:ipetkov/crane";
    };
    rust-overlay = {
      url = "https://flakehub.com/f/oxalica/rust-overlay/0.1.tar.gz";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nix-filter.url = "github:numtide/nix-filter";
  };

  outputs = { self, nixpkgs, crane, nix-filter, flake-utils, rust-overlay }:
    flake-utils.lib.eachSystem [ "x86_64-linux" "aarch64-darwin" ] (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = (import nixpkgs) {
          inherit system overlays;
        };
        inherit (pkgs) lib;
        frameworks = pkgs.darwin.apple_sdk.frameworks;

        # filter the source to reduce cache misses
        # add a path here if you need other files, e.g. bc of `include_str!()`
        src = nix-filter {
          root = ./.;
          include = [
            (nix-filter.lib.matchExt "toml")
            ./Cargo.toml
            ./Cargo.lock
            ./public
            ./content
            ./src
            ./style
            ./tailwind.config.js
          ];
        };

        toolchain = pkgs.rust-bin.nightly."2024-09-01".minimal.override {
          targets = [ "wasm32-unknown-unknown" ];
        };
        dev-toolchain = pkgs.rust-bin.nightly."2024-09-01".default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
          targets = [ "wasm32-unknown-unknown" ];
        };

        # read leptos options from `Cargo.toml`
        leptos-options = builtins.elemAt (builtins.fromTOML (
          builtins.readFile ./Cargo.toml
        )).workspace.metadata.leptos 0;
        
        # configure crane to use our toolchain
        craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;

        # crane build configuration used by multiple builds
        common-args = {
          inherit src;

          # use the name defined in the `Cargo.toml` leptos options
          pname = leptos-options.bin-package;
          version = "0.1.0";

          doCheck = false;

          nativeBuildInputs = [
            pkgs.binaryen # provides wasm-opt
            pkgs.cargo-leptos
            pkgs.tailwindcss
          ] ++ pkgs.lib.optionals (system == "x86_64-linux") [
            pkgs.nasm # wasm compiler only for x86_64-linux
          ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
            # Additional darwin specific inputs can be set here
            pkgs.libiconv # character encoding lib needed by darwin
            frameworks.Security
            frameworks.CoreFoundation
            frameworks.CoreServices
            frameworks.SystemConfiguration
            frameworks.Accelerate
          ];

          buildInputs = [
            pkgs.pkg-config # used by many crates for finding system packages
            pkgs.openssl # needed for many http libraries
          ];

        };

        cargoArtifacts = craneLib.buildDepsOnly common-args;

        ck-dot-dev-frontend-deps = craneLib.mkCargoDerivation (common-args // {
          pname = "ck-dot-dev-frontend-deps";
          src = craneLib.mkDummySrc common-args;
          cargoArtifacts = null;
          doInstallCargoArtifacts = true;

          buildPhaseCargoCommand = ''
            cargo build \
              --package=${leptos-options.lib-package} \
              --lib \
              --target-dir=/build/source/target/front \
              --target=wasm32-unknown-unknown \
              --no-default-features \
              --profile=${leptos-options.lib-profile-release}
          '';
        });

        ck-dot-dev-server-deps = craneLib.mkCargoDerivation (common-args // {
          pname = "ck-dot-dev-server-deps";
          src = craneLib.mkDummySrc common-args;
          cargoArtifacts = ck-dot-dev-frontend-deps;
          doInstallCargoArtifacts = true;

          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [ pkgs.openssl ];

          buildPhaseCargoCommand = ''
            cargo build \
              --package=${leptos-options.bin-package} \
              --no-default-features \
              --release
          '';
        });


        # build the binary and bundle using cargo leptos
        ck-dot-dev-server = craneLib.buildPackage (common-args // {
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [ pkgs.openssl ];
          OPENSSL_DEV=pkgs.openssl.dev;

          # add inputs needed for leptos build
          nativeBuildInputs = common-args.nativeBuildInputs;

          buildPhaseCargoCommand = ''
            LEPTOS_HASH_FILES=true cargo leptos build --release -vvv
          '';

          installPhaseCommand = ''
            mkdir -p $out/bin
            cp target/release/ck-dot-dev-server $out/bin/
            cp target/release/hash.txt $out/bin/
            cp -r target/site $out/bin/
          '';

          cargoArtifacts = ck-dot-dev-server-deps;
        });

      in {
        checks = {
          # lint packages
          # app-hydrate-clippy = craneLib.cargoClippy (common-args // {
          #   cargoArtifacts = ck-dot-dev-server-deps;
          #   cargoClippyExtraArgs = "-p ck-dot-dev-app --features hydrate -- --deny warnings";
          # });
          # app-ssr-clippy = craneLib.cargoClippy (common-args // {
          #   cargoArtifacts = ck-dot-dev-server-deps;
          #   cargoClippyExtraArgs = "-p ck-dot-dev-app --features ssr -- --deny warnings";
          # });
          ck-dot-dev-server-clippy = craneLib.cargoClippy (common-args // {
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "-p ck-dot-dev-server -- --deny warnings";
          });
          ck-dot-dev-frontend-clippy = craneLib.cargoClippy (common-args // {
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "-p ck-dot-dev-frontend -- --deny warnings";
          });

          # make sure the docs build
          # ck-dot-dev-server-doc = craneLib.cargoDoc (common-args // {
          #   cargoArtifacts = ck-dot-dev-server-deps;
          # });

          # check formatting
          # ck-dot-dev-server-fmt = craneLib.cargoFmt {
          #   pname = common-args.pname;
          #   version = common-args.version;
          #   
          #   inherit src;
          # };

          # # audit licenses
          # ck-dot-dev-server-deny = craneLib.cargoDeny {
          #   pname = common_args.pname;
          #   version = common_args.version;
          #   inherit src;
          # };

          # run tests
          # ck-dot-dev-server-nextest = craneLib.cargoNextest (common-args // {
          #   cargoArtifacts = ck-dot-dev-server-deps;
          #   partitions = 1;
          #   partitionType = "count";
          # });
        };

        packages = {
          default = ck-dot-dev-server;
        };
        
        devShells.default = pkgs.mkShell {
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [ pkgs.openssl ];
          OPENSSL_DEV=pkgs.openssl.dev;
          nativeBuildInputs = (with pkgs; [
            dev-toolchain # rust toolchain
            just # command recipes
            dive # for inspecting docker images
            cargo-leptos # main leptos build tool
            bacon # cargo check w/ hot reload
            cargo-deny # license checking
            tailwindcss
          ])
            ++ common-args.buildInputs
            ++ common-args.nativeBuildInputs
            ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
              pkgs.darwin.Security
              frameworks.Security
              frameworks.CoreFoundation
              frameworks.CoreServices
              frameworks.SystemConfiguration
              frameworks.Accelerate
            ];
        };
      }
    );
}
