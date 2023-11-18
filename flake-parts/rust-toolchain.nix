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
      sha256 = "sha256-PZjjJ2eeY5keN6NtGH9V555p3qpiVWxYtZj/LFkQ+DA=";
    };
  in rec {
    packages = {
      rust-toolchain = fenix-toolchain;
    };
  };
}
