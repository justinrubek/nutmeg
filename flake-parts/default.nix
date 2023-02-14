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
    ...
  }: let
    devTools = [
      fenix-toolchain
      pkgs.rustfmt
      pkgs.bacon
      pkgs.cocogitto
      inputs'.bomper.packages.cli
      pkgs.miniserve
      pkgs.wasm-bindgen-cli
    ];

    ciTools = [
      fenix-toolchain
      pkgs.cocogitto
    ];

    bevyNativeBuildInputs = [pkgs.pkg-config pkgs.llvmPackages.bintools];
    bevyBuildInputs = with pkgs; [
      udev
      alsaLib
      vulkan-loader
      xlibsWrapper
      xorg.libXcursor
      xorg.libXrandr
      xorg.libXi
      libxkbcommon
      wayland
      clang
    ];

    allBuildInputs = base: base ++ bevyBuildInputs;
    allNativeBuildInputs = base: base ++ bevyNativeBuildInputs;
    fenix-toolchain = inputs'.fenix.packages.fromToolchainFile {
      file = ../toolchain.toml;
      sha256 = "sha256-Xf9G2PXaLF/qAIB0ifePSmoPkkOPT2Ic6PkFJwDcZf0=";
    };

    craneLib = inputs.crane.lib.${system}.overrideToolchain fenix-toolchain;

    common-build-args = rec {
      src = lib.cleanSourceWith {
        src = ../.;
      };
      pname = "nutmeg";

      buildInputs = allBuildInputs [];
      nativeBuildInputs = allNativeBuildInputs [];
      LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
    };
    deps-only = craneLib.buildDepsOnly ({} // common-build-args);

    clippy-check = craneLib.cargoClippy ({
        cargoArtifacts = deps-only;
        cargoClippyExtraArgs = "--all-features -- --deny warnings";
      }
      // common-build-args);

    tests-check = craneLib.cargoNextest ({
        cargoArtifacts = deps-only;
        partitions = 1;
        partitionType = "count";
      }
      // common-build-args);

    client-package = craneLib.buildPackage ({
        pname = "nutmeg-client";
        cargoArtifacts = deps-only;
        cargoExtraArgs = "--bin nutmeg_client";
      }
      // common-build-args);

    server-package = craneLib.buildPackage ({
        pname = "nutmeg-server";
        cargoArtifacts = deps-only;
        cargoExtraArgs = "--bin nutmeg-server";
      }
      // common-build-args);

    wasm-package = craneLib.buildPackage (rec {
        pname = "nutmeg-wasm";
        cargoArtifacts = deps-only;
        cargoExtraArgs = "--bin nutmeg_wasm";
        buildInputs = allBuildInputs [
          pkgs.xorg.libxcb
          pkgs.wasm-bindgen-cli
        ];
        nativeBuildInputs = allNativeBuildInputs [pkgs.wasm-pack pkgs.wasm-bindgen-cli];
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
  in rec {
    devShells = {
      default = devShells.nightly;
      nightly = pkgs.mkShell rec {
        buildInputs = allBuildInputs devTools;
        nativeBuildInputs = bevyNativeBuildInputs;
        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
        inherit (self.checks.${system}.pre-commit) shellHook;
      };
      ci = pkgs.mkShell rec {
        buildInputs = allBuildInputs ciTools;
        nativeBuildInputs = bevyNativeBuildInputs;
        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
      };
    };

    packages = {
      default = packages.client;
      client = client-package;
      server = server-package;
      client-wasm = wasm-package;
      rust-toolchain = fenix-toolchain;
    };

    apps = {
      client = {
        type = "app";
        program = "${self.packages.${system}.client}/bin/nutmeg_client";
      };
      default = apps.client;
    };

    checks = {
      pre-commit = import ./checks/pre-commit.nix part-inputs system;
      clippy = clippy-check;
      tests = tests-check;
    };
  };
}
