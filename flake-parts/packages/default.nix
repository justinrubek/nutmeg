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
  in rec {
    packages = {
      default = packages.cli;
      client = pkgs.rustPlatform.buildRustPackage {
        pname = "nutmeg-client";
        version = "0.1.0";

        buildAndTestSubdir = "crates/client";
        src = self.lib.flake_source;
        cargoLock = {
          lockFile = self.lib.cargo_lock;
        };
        buildInputs = [rust-nightly];
      };
      server = pkgs.rustPlatform.buildRustPackage {
        pname = "nutmeg-server";
        version = "0.1.0";

        buildAndTestSubdir = "crates/server";
        src = self.lib.flake_source;
        cargoLock = {
          lockFile = self.lib.cargo_lock;
        };
        buildInputs = [rust-nightly];
      };
    };
  };
}
