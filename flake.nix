{
  inputs = {
    # Nix Packages collection & NixOS
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

    # Pure Nix flake utility functions
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    { nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (
      system: with nixpkgs.legacyPackages.${system}; {
        devShells.default = mkShell {
          packages = [
            cargo
            rustfmt
          ];
        };
      }
    );
}
