{ ... }:
{
  perSystem =
    { pkgs, ... }:
    {
      packages = {
        default = pkgs.cc-json-parser;
        inherit (pkgs)
          cc-json-parser
          ;
      };
      legacyPackages = pkgs;
    };
}
