{
  self,
  inputs,
  lib,
  ...
} @ part-inputs: system:
inputs.pre-commit-hooks.lib.${system}.run {
  src = lib.cleanSourceWith {
    src = ../../.;
  };
  hooks = {
    alejandra.enable = true;
    # clippy.enable = true;
  };
}
