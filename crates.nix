{
  perSystem = { pkgs, lib, ... }:
    let
      inherit (pkgs)
        darwin
        stdenv
        ;
    in
    {
      nci = {
        toolchainConfig = ./rust-toolchain.toml;
        projects.efty = rec {
          path = ./.;
          drvConfig = {
            mkDerivation.nativeBuildInputs = [
              pkgs.tailwindcss
              pkgs.leptosfmt
              pkgs.cargo-leptos
              pkgs.sass
            ] ++
            lib.optionals stdenv.isDarwin [
              darwin.apple_sdk.frameworks.SystemConfiguration
            ];
          };
          depsDrvConfig = drvConfig;

        };
      };
    };
}
