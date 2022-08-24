{ pkgs ? import <nixpkgs> { } , ... }:
pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    gcc
    pkg-config
    lldb
    rustup
  ];
  buildInputs = with pkgs; [
    clippy
    pango
    gtk4
    glib
    cairo
    graphene
    gdk-pixbuf
    gir-rs
  ];

  RUSTC_VERSION = "nightly-2022-08-24";
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
