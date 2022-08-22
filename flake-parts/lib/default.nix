{
  inputs,
  self,
  ...
}: let
  inherit (inputs.gitignore.lib) gitignoreSource;
in {
  flake.lib = rec {
    flake_source = gitignoreSource ../..;
    cargo_lock = ../../Cargo.lock;
    rust-stable = system: inputs.rust-overlay.packages.${system}.rust;
    rust-nightly = system: inputs.rust-overlay.packages.${system}.rust-nightly;

    bevyNativeBuildInputs = pkgs: with pkgs; [pkgconfig llvmPackages.bintools];
    bevyBuildInputs = pkgs:
      with pkgs; [
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
    allBuildInputs = pkgs: base: base ++ bevyBuildInputs pkgs;
  };
}
