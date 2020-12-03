# import niv sources and the pinned nixpkgs
{ sources ? import ./nix/sources.nix, pkgs ? import sources.nixpkgs { } }:
let
  # import rust compiler
  # rust = import ./nix/rust.nix { inherit sources; };
  rust = import ./nix/rust.nix;

  # configure naersk to use our pinned rust compiler
  # naersk = pkgs.callPackage sources.naersk {
  #   rustc = rust;
  #   cargo = pkgs.latest.rustChannels.nightly.rust;
  # };

  # tell nix-build to ignore the `target` directory
  src = builtins.filterSource
    (path: type: type != "directory" || builtins.baseNameOf path != "target") ./.;
in rust.naersk.buildPackage {
  inherit src;

  name = "repo";
  version = "0.1.2";

  buildInputs = with pkgs; [
    openssl
    pkg-config
  ];

  # The default is [ "-Z" "unstable-options" ] but we are on stable
  cargoOptions = v: ["--locked"];

  cargoBuildOptions = v: [ "$cargo_release" ''-j "$NIX_BUILD_CORES"''
                           "--message-format=$cargo_message_format"];
  cargoBuild = v: ''
    OUT_DIR=out cargo $cargo_options build $cargo_build_options >> $cargo_build_output_json
  '';

  # remove nix store references for a smaller output package
  remapPathPrefix = true;
}

