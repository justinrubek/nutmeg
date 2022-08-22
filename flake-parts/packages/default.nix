{
  inputs,
  self,
  ...
}: {
  perSystem = {
    config,
    pkgs,
    system,
    ...
  }: let
    rust-stable = self.lib.rust-stable system;
    rust-nightly = self.lib.rust-nightly system;
    rust-wasm = rust-nightly.override {
      targets = ["wasm32-unknown-unknown"];
    };

    allBuildInputs = base: self.lib.allBuildInputs pkgs base;
    nativeBuildInputs = self.lib.bevyNativeBuildInputs pkgs;
  in rec {
    packages = {
      default = packages.cli;
      client = pkgs.rustPlatform.buildRustPackage {
        pname = "nutmeg-client";
        version = "0.4.7";

        buildAndTestSubdir = "crates/client";
        src = self.lib.flake_source;
        cargoLock = {
          lockFile = self.lib.cargo_lock;
        };
        buildInputs = [rust-nightly];
      };
      server = pkgs.rustPlatform.buildRustPackage {
        pname = "nutmeg-server";
        version = "0.4.7";

        buildAndTestSubdir = "crates/server";
        src = self.lib.flake_source;
        cargoLock = {
          lockFile = self.lib.cargo_lock;
        };
        buildInputs = [rust-nightly];
      };
      client-wasm = pkgs.rustPlatform.buildRustPackage {
        pname = "nutmeg-client-wasm";
        version = "0.4.7";

        buildAndTestSubdir = "crates/client";
        src = self.lib.flake_source;
        cargoLock = {
          lockFile = self.lib.cargo_lock;
        };
        buildInputs = allBuildInputs [
          pkgs.xorg.libxcb
          pkgs.wasm-bindgen-cli
        ];
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
        nativeBuildInputs = [rust-wasm pkgs.wasm-pack pkgs.wasm-bindgen-cli] ++ nativeBuildInputs;
      };
    };
  };
}
