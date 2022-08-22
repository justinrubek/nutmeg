{
  inputs,
  self,
  ...
} @ part-inputs: {
  perSystem = {
    config,
    pkgs,
    system,
    inputs',
    ...
  }: let
    rust-stable = self.lib.rust-stable system;
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

    allBuildInputs = self.lib.allBuildInputs pkgs;
  in rec {
    devShells = {
      default = devShells.nightly;
      stable = pkgs.mkShell rec {
        buildInputs = allBuildInputs [rust-stable] ++ devTools;
        nativeBuildInputs = self.lib.bevyNativeBuildInputs pkgs;
        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
        inherit (self.checks.${system}.pre-commit) shellHook;
      };
      nightly = pkgs.mkShell rec {
        buildInputs = allBuildInputs [rust-nightly] ++ devTools;
        nativeBuildInputs = self.lib.bevyNativeBuildInputs pkgs;
        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
        inherit (self.checks.${system}.pre-commit) shellHook;
      };
      wasm = pkgs.mkShell rec {
        buildInputs = allBuildInputs [rust-wasm pkgs.wasm-bindgen-cli] ++ devTools;
        nativeBuildInputs = self.lib.bevyNativeBuildInputs pkgs;
        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
        inherit (self.checks.${system}.pre-commit) shellHook;
      };
      cli = pkgs.mkShell rec {
        buildInputs = allBuildInputs [rust-nightly] ++ devTools;
        nativeBuildInputs = self.lib.bevyNativeBuildInputs pkgs;
        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
      };
    };
  };
}
