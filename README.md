# color-picker

A color picker using hyprpicker written in rust

## Dependencies

- [hyprpicker](https://github.com/hyprwm/hyprpicker)

## Usage

```text
A simple color picker wrapper for hyprpicker

Usage: color-picker [OPTIONS]

Options:
  -u, --usage
  -h, --help     Print help
  -V, --version  Print version
```

Just run this and let it work it's magick!

```bash
color-picker
```

## Installation

### On other distributions

```sh
git clone https://github.com/Daru-san/color-picker
cd color-picker
cargo build --release
```

### On NixOS

Run without installing

```sh
nix run github:Daru-san/color-picker
```

Add to flake for home-wide or system-wide installation

```nix
# In flake.nix
{
    description = "Your flake";
    inputs = {
        nixpkgs.url = "nixpkgs/nixos-unstable";
        color-picker = {
            url = "github:Daru-san/color-picker";
            inputs.nixpkgs.follows = "nixpkgs";
        };
    };
}
```

```nix
# In home.nix
{pkgs,inputs,...}:{
    home.packages = [inputs.color-picker.packages.${pkgs.system}.default];
}
```
