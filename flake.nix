{
  description = "Rust dev shell with OpenSSL and libexif";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ rust-overlay.overlays.default ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustToolchain = pkgs.rust-bin.nightly.latest.default;
        openssl_combined = pkgs.symlinkJoin {
          name = "openssl-unified";
          paths = [ pkgs.openssl.dev pkgs.openssl.out];
        };
      in {
        devShells.default = pkgs.mkShell {
          buildInputs = [
            rustToolchain
            pkgs.pkg-config
            pkgs.openssl.dev
            pkgs.openssl.out
            pkgs.libexif
            pkgs.scc
          ];

          # Ensure pkg-config can find OpenSSL and libexif
          PKG_CONFIG_PATH = pkgs.lib.makeLibraryPath [
            pkgs.openssl.out
            pkgs.openssl.dev
            pkgs.libexif
          ] + "/lib/pkgconfig";

          # Optional: expose OpenSSL_DIR for crates like `openssl-sys`
          OPENSSL_DIR = openssl_combined;

          # Optional: improve reproducibility in crates like `openssl-sys`
          OPENSSL_NO_VENDOR = "1";
        };
      });
}
