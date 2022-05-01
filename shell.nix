{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
  buildInputs = [
    pkgs.rustc
    pkgs.rust-analyzer
    pkgs.cargo
    pkgs.cargo-edit
    pkgs.cargo-feature
    pkgs.rustfmt
    pkgs.lldb_9
  ];
}
