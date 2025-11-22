# TEAM_556: Flake for gutter-bar GTK4 Rust dev shell
{
  description = "GTK4 gutter-bar status bar / launcher";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.outputs.overlays.default ];
        };

        rust-bin = rust-overlay.outputs.lib.mkRustBin { } pkgs;
        rustToolchain = rust-bin.stable.latest.default.override {
          extensions = [
            "rust-analyzer"
            "rust-src"
            "clippy-preview"
            "rustfmt-preview"
          ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            pkg-config
            wrapGAppsHook4
            rustToolchain
          ];

          buildInputs = with pkgs; [
            gtk4
            glib
            glib.dev
            pango
            pango.dev
            cairo
            cairo.dev
            gdk-pixbuf
            gdk-pixbuf.dev
          ];
        };
      }
    );
}
