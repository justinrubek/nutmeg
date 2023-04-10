{
  inputs,
  self,
  ...
} @ part-inputs: {
  imports = [];

  perSystem = {
    config,
    pkgs,
    lib,
    system,
    inputs',
    self',
    ...
  }: let
    devTools = [
      self'.packages.rust-toolchain
      pkgs.rustfmt
      pkgs.bacon
      pkgs.cocogitto
      inputs'.bomper.packages.cli
      pkgs.miniserve
      pkgs.wasm-bindgen-cli
    ];

    ciTools = [
      self'.packages.rust-toolchain
      pkgs.cocogitto
    ];

    bevyNativeBuildInputs = [
      pkgs.pkg-config
      pkgs.llvmPackages.bintools
      pkgs.udev
      pkgs.alsaLib
      pkgs.vulkan-loader
      pkgs.xlibsWrapper
      pkgs.xorg.libXcursor
      pkgs.xorg.libXrandr
      pkgs.xorg.libXi
      pkgs.libxkbcommon
      pkgs.wayland
      pkgs.clang
    ];
    withExtraPackages = base: base ++ bevyNativeBuildInputs;

    craneLib = inputs.crane.lib.${system}.overrideToolchain self'.packages.rust-toolchain;

    common-build-args = rec {
      src = lib.cleanSourceWith {
        src = ../.;
      };
      pname = "nutmeg";

      nativeBuildInputs = withExtraPackages [];
      LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath nativeBuildInputs;
    };
    deps-only = craneLib.buildDepsOnly ({} // common-build-args);

    checks = {
      pre-commit = import ./checks/pre-commit.nix part-inputs system;

      clippy = craneLib.cargoClippy ({
          cargoArtifacts = deps-only;
          cargoClippyExtraArgs = "--all-features -- --deny warnings";
        }
        // common-build-args);

      tests = craneLib.cargoNextest ({
          cargoArtifacts = deps-only;
          partitions = 1;
          partitionType = "count";
        }
        // common-build-args);
    };

    packages = {
      client = craneLib.buildPackage ({
          pname = "nutmeg-client";
          cargoArtifacts = deps-only;
          cargoExtraArgs = "--bin nutmeg_client";
        }
        // common-build-args);

      server = craneLib.buildPackage ({
          pname = "nutmeg-server";
          cargoArtifacts = deps-only;
          cargoExtraArgs = "--bin nutmeg-server";
        }
        // common-build-args);

      wasm = craneLib.buildPackage (rec {
          pname = "nutmeg-wasm";
          cargoArtifacts = deps-only;
          cargoExtraArgs = "--bin nutmeg_wasm";
          buildInputs = [
            pkgs.xorg.libxcb
            pkgs.wasm-bindgen-cli
          ];
          nativeBuildInputs = withExtraPackages [pkgs.wasm-pack pkgs.wasm-bindgen-cli];
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
          buildPhase = ''
            # required to enable web_sys clipboard API
            export RUSTFLAGS=--cfg=web_sys_unstable_apis

            cargo build --release --target wasm32-unknown-unknown --manifest-path=crates/client/Cargo.toml

            ${pkgs.wasm-bindgen-cli}/bin/wasm-bindgen --out-dir $out/wasm --target web target/wasm32-unknown-unknown/release/nutmeg_client.wasm
          '';
          installPhase = ''
            echo 'Skipping installPhase'
          '';
          doCheck = false;
        }
        // common-build-args);
    };
  in rec {
    inherit checks packages;

    devShells = {
      default = pkgs.mkShell rec {
        packages = withExtraPackages devTools;
        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath packages;
        inherit (self.checks.${system}.pre-commit) shellHook;
      };
      ci = pkgs.mkShell rec {
        packages = withExtraPackages ciTools;
        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath packages;
      };
    };

    apps = {
      client = {
        type = "app";
        program = "${self.packages.${system}.client}/bin/nutmeg_client";
      };
      default = apps.client;
    };
  };
}
