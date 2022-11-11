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
  }: rec {
    apps = {
      client = {
        type = "app";
        program = "${self.packages.${system}.client}/bin/client";
      };
      default = apps.client;
    };
  };
}
