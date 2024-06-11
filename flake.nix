{
  description = "Color picker for wayland written in rust";

  inputs = {
    utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "nixpkgs/nixos-unstable";
  };

  outputs = {
    self,
    nixpkgs,
    utils,
    ...
  }:
    utils.lib.eachDefaultSystem
    (
      system: let
        pkgs = import nixpkgs {inherit system;};
        toolchain = pkgs.rustPlatform;
      in rec
      {
        packages.default = toolchain.buildRustPackage {
          pname = "color-picker-rs";
          version = "0.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
          nativeBuildInputs = [pkgs.makeWrapper];
          postIntall = ''
            wrapProgram $out/bin/color-picker \
              --prefix PATH : ${
              nixpkgs.lib.makeBinPath [pkgs.hyprpicker]
            }
          '';
        };

        apps.default = utils.lib.mkApp {drv = packages.default;};
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            (with toolchain; [
              cargo
              rustc
              rustLibSrc
            ])
            hyprpicker
            clippy
            rustfmt
            pkg-config
          ];

          # Specify the rust-src path (many editors rely on this)
          RUST_SRC_PATH = "${toolchain.rustLibSrc}";
        };
      }
    );
}
