{ pkgs ? import <nixpkgs> {} }:

with pkgs;

mkShell {
  buildInputs = [
    llvmPackages.libclang
  ];

  LIBCLANG_PATH = "${llvmPackages.libclang.lib}/lib";
}
