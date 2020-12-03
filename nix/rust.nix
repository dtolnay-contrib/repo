{ sources = import ./sources.nix }:
let
  stable = pks.rustChannelOf {
    rustToolchain = "stable";
  };

  nightly = pkgs.rustChannelOf {
    rustToolchain = ./rust-toolchain;
  };

  naersk = pkgs.callPackage sources.naersk {
    cargo = nightly;
    rust = stable;
  };
in
{
  inherit naersk stable nightly
}
# { sources ? import ./sources.nix }:
# let
#   pkgs = import sources.nixpkgs { overlays = [ (import sources.nixpkgs-mozilla) ]; };
#   channel = "stable";
#   chan = pkgs.latest.rustChannels.stable.rust;
# in chan
