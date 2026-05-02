{ inputs, ... }:
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
      checks = {
        clippy = craneLib.cargoClippy (
          commonArgs
          // {
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "--all-targets -- --deny warnings";
          }
        );

        tests = craneLib.cargoNextest (
          commonArgs
          // {
            inherit cargoArtifacts;
            src = pkgs.lib.cleanSourceWith {
              src = pkgs.lib.cleanSource inputs.self;
              filter =
                path: type:
                (craneLib.filterCargoSources path type) || (builtins.match ".*/test-data/.*" path != null);
            };
          }
        );

        machete = craneLib.mkCargoDerivation (
          commonArgs
          // {
            inherit cargoArtifacts;
            nativeBuildInputs = [ pkgs.cargo-machete ];
            buildPhaseCargoCommand = "cargo machete";
          }
        );

        audit = craneLib.cargoAudit (
          commonArgs
          // {
            advisory-db = inputs.advisory-db;
          }
        );

        deny = craneLib.cargoDeny (commonArgs // { });
      };
    };
}
