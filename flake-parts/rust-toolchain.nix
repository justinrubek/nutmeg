{
  inputs,
  self,
  ...
} @ part-inputs: {
  imports = [];

  perSystem = {
    pkgs,
    lib,
    system,
    inputs',
    ...
  }: let
    fenix-toolchain = inputs'.fenix.packages.fromToolchainFile {
      file = ../toolchain.toml;
      sha256 = "sha256-LnvKbTUeYmXkynJvVjTYmc1FsZaRiuLr8gviAac7D9Y=";
    };
  in rec {
    packages = {
      rust-toolchain = fenix-toolchain;
    };
  };
}
