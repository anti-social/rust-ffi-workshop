{ pkgs ? import <nixpkgs> {} }:

with pkgs;

mkShell {
  buildInputs = [
    llvmPackages.libclang
    glibc.dev
    python3
  ];

  LIBCLANG_PATH = "${llvmPackages.libclang.lib}/lib";

  # NIX_CFLAGS_COMPILE has glibc include paths, but bindgen doesn't
  # read it — it only respects BINDGEN_EXTRA_CLANG_ARGS.
  shellHook = ''
    export BINDGEN_EXTRA_CLANG_ARGS="$NIX_CFLAGS_COMPILE"
  '';
}
