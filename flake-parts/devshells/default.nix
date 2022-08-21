{
  inputs,
  self,
  ...
} @ part-inputs: {
  perSystem = {
    config,
    pkgs,
    system,
    ...
  }: let
    rust-stable = self.lib.rust-stable system;
    rust-nightly = self.lib.rust-nightly system;
    devTools = with pkgs; [
      rustfmt
      bacon
      cocogitto
    ];

    bevyNativeBuildInputs = with pkgs; [pkgconfig llvmPackages.bintools];
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

    allBuildInputs = base: base ++ devTools ++ bevyBuildInputs;
  in rec {
    devShells = {
      default = devShells.nightly;
      stable = pkgs.mkShell rec {
        # buildInputs = [rust-stable] ++ devTools ++ bevyBuildInputs;
        buildInputs = allBuildInputs [rust-stable];
        nativeBuildInputs = bevyNativeBuildInputs;
        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
        inherit (self.checks.${system}.pre-commit) shellHook;
      };
      nightly = pkgs.mkShell rec {
        # buildInputs = [rust-nightly] ++ devTools ++ bevyBuildInputs;
        buildInputs = allBuildInputs [rust-nightly];
        nativeBuildInputs = bevyNativeBuildInputs;
        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
        inherit (self.checks.${system}.pre-commit) shellHook;
      };
    };
  };
}
