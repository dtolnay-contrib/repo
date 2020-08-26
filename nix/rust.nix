{ sources ? import ./sources.nix }:

let
    pkgs = import sources.nixpkgs { overlays = [ (import sources.nixpkgs-mozilla) ]; };
    channel = "stable";
    chan = pkgs.latest.rustChannels.stable.rust;
in chan
