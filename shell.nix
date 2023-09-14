{ stable ? import <nixpkgs> { }, ... }:
stable.mkShell rec {
  developmentInputs = (with stable; [ ]);
  buildInputs =
    (with stable; [
      # Enable rust
      rustup

      # Used for building and debugging rust elements
      gcc
      pkg-config
      lldb
      cargo
      rustc
      clippy
      openssl
      freetype
      git-lfs
      rust-analyzer
      cargo-watch
      cargo-llvm-cov
      llvmPackages_16.bintools-unwrapped
      just
      rustfmt
      trunk
      moreutils
      expect
      tmux
    ]);

  RUST_SRC_PATH = "${stable.rust.packages.stable.rustPlatform.rustLibSrc}";
  shellHook = ''
    export RUSTUP_TOOLCHAIN="nightly";
    export PATH="$CARGO_HOME:$PATH";
    export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${stable.lib.makeLibraryPath buildInputs}";
  '';
}
