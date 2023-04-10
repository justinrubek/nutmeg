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
      sha256 = "sha256-Xf9G2PXaLF/qAIB0ifePSmoPkkOPT2Ic6PkFJwDcZf0=";
    };
  in rec {
    packages = {
      rust-toolchain = fenix-toolchain;
    };
  };
}
