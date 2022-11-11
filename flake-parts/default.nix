{
  inputs,
  self,
  ...
} @ part-inputs: {
  imports = [
    ./lib
  ];

  perSystem = {
    config,
    pkgs,
    system,
    inputs',
    ...
  }: let
    rust-nightly = self.lib.rust-nightly system;
    rust-wasm = rust-nightly.override {
      targets = ["wasm32-unknown-unknown"];
    };
    devTools = with pkgs; [
      rustfmt
      bacon
      cocogitto
      inputs'.bomper.packages.cli
      miniserve
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
  in rec {
    devShells = {
      default = devShells.nightly;
      nightly = pkgs.mkShell rec {
        buildInputs = allBuildInputs [rust-nightly] ++ devTools;
        nativeBuildInputs = bevyNativeBuildInputs;
        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
        inherit (self.checks.${system}.pre-commit) shellHook;
      };
      wasm = pkgs.mkShell rec {
        buildInputs = allBuildInputs [rust-wasm pkgs.wasm-bindgen-cli] ++ devTools;
        nativeBuildInputs = bevyNativeBuildInputs;
        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
        inherit (self.checks.${system}.pre-commit) shellHook;
      };
      ci = pkgs.mkShell rec {
        buildInputs = allBuildInputs [rust-nightly] ++ devTools;
        nativeBuildInputs = bevyNativeBuildInputs;
        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
      };
    };

    packages = {
      default = packages.cli;
      client = pkgs.rustPlatform.buildRustPackage rec {
        pname = "nutmeg-client";
        version = "0.9.0";

        buildAndTestSubdir = "crates/client";
        src = self.lib.flake_source;
        cargoLock = {
          lockFile = self.lib.cargo_lock;
        };
        buildInputs = allBuildInputs [rust-nightly];
        nativeBuildInputs = allNativeBuildInputs [];
        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
      };
      server = pkgs.rustPlatform.buildRustPackage rec {
        pname = "nutmeg-server";
        version = "0.9.0";

        buildAndTestSubdir = "crates/server";
        src = self.lib.flake_source;
        cargoLock = {
          lockFile = self.lib.cargo_lock;
        };
        buildInputs = allBuildInputs [rust-nightly];
        nativeBuildInputs = allNativeBuildInputs [];
        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
      };
      client-wasm = pkgs.rustPlatform.buildRustPackage rec {
        pname = "nutmeg-client-wasm";
        version = "0.9.0";

        buildAndTestSubdir = "crates/client";
        src = self.lib.flake_source;
        cargoLock = {
          lockFile = self.lib.cargo_lock;
        };
        buildInputs = allBuildInputs [
          pkgs.xorg.libxcb
          pkgs.wasm-bindgen-cli
        ];
        nativeBuildInputs = allNativeBuildInputs [rust-wasm pkgs.wasm-pack pkgs.wasm-bindgen-cli];
        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
        buildPhase = ''
          # required to enable web_sys clipboard API
          export RUSTFLAGS=--cfg=web_sys_unstable_apis

          cargo build --release --target wasm32-unknown-unknown --manifest-path=crates/client/Cargo.toml

          wasm-bindgen --out-dir $out/wasm --target web target/wasm32-unknown-unknown/release/nutmeg_client.wasm
        '';
        installPhase = ''
          echo 'Skipping installPhase'
        '';
        doCheck = false;
      };
    };

    apps = {
      client = {
        type = "app";
        program = "${self.packages.${system}.client}/bin/client";
      };
      default = apps.client;
    };

    checks = {
      pre-commit = import ./checks/pre-commit.nix part-inputs system;
    };
  };
}
