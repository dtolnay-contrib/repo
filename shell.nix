let
    sources = import ./nix/sources.nix;
    rust = import ./nix/rust.nix { inherit sources; };
    pkgs = import sources.nixpkgs {};
in
pkgs.mkShell {
  buildInputs = with pkgs; [
    rust
    openssl
    pkg-config

    # keep this line if you use bash
    pkgs.bashInteractive
  ];
}
