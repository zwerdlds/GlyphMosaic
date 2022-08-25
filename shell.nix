{ pkgs ? import <nixpkgs> { }, ... }:
pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    gcc
    pkg-config
    lldb
    rustup
    gsettings-desktop-schemas
    kde-gtk-config
    glib
    gtk4
    wrapGAppsHook
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
    gtk3
  ];

  RUSTC_VERSION = "nightly-2022-08-24";
  NIX_ENFORCE_PURITY = 0;
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
  shellHook = ''
    XDG_DATA_DIRS=$GSETTINGS_SCHEMAS_PATH
  '';
}
