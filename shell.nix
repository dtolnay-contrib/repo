let
  sources = import ./nix/sources.nix;
  rust = import ./nix/rust.nix { inherit sources; };
  pkgs = import sources.nixpkgs {};

  app-deps = with pkgs; [
    rust
    openssl
    pkg-config
  ];

  dev-deps = with pkgs; [
    nixpkgs-fmt
    bashInteractive
  ];

in
pkgs.mkShell {
  buildInputs = app-deps ++ dev-deps;
}
