{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    rustc
    cargo
    pkg-config
    openssl
  ];

  # Optional: Set environment variables if needed
  # shellHook = ''
  #   export SOME_VAR="some_value"
  # '';
}
