{
  description = "Color picker for wayland written in rust, using hyprpicker";

  inputs = {
    utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "nixpkgs/nixos-unstable";
  };

  outputs =
    {
      self,
      nixpkgs,
      utils,
      ...
    }:
    let
      inherit (nixpkgs.lib) makeBinPath licenses maintainers;
    in
    utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
        toolchain = pkgs.rustPlatform;
      in
      rec {
        packages = {
          default = packages.color-picker;
          color-picker = toolchain.buildRustPackage {
            pname = "color-picker";
            version = "0.1.2";
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
            nativeBuildInputs = [ pkgs.makeWrapper ];
            postInstall = ''
              wrapProgram $out/bin/color-picker \
                --prefix PATH : ${makeBinPath [ pkgs.hyprpicker ]}
            '';
            meta = {
              description = "A simple wrapper program for hyprpicker with notifications";
              homepage = "https://github.com/Daru-san/color-picker";
              license = licenses.mit;
              maintainers = [ maintainers.daru-san ];
              mainProgram = "color-picker";
            };
          };
        };
        formatter = pkgs.nixfmt-rfc-style;
        apps = {
          default = apps.color-picker;
          color-picker = utils.lib.mkApp { drv = packages.color-picker; };
        };
        devShells.default = pkgs.mkShell {
          buildInputs = [
            (with toolchain; [ rustLibSrc ])
            (with pkgs; [
              rustc
              cargo
              hyprpicker
              clippy
              rustfmt
              pkg-config
            ])
          ];

          # Specify the rust-src path (many editors rely on this)
          RUST_SRC_PATH = "${toolchain.rustLibSrc}";
        };
      }
    );
}
