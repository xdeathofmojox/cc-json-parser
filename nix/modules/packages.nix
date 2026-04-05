{ ... }:
{
  perSystem =
    {
      pkgs,
      craneLib,
      commonArgs,
      cargoArtifacts,
      ...
    }:
    {
      packages = {
        default = craneLib.buildPackage (commonArgs // { inherit cargoArtifacts; });
        cc-json-parser = craneLib.buildPackage (commonArgs // { inherit cargoArtifacts; });
      };
      legacyPackages = pkgs;
    };
}
