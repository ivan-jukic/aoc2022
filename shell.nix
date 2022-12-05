{ environ ? import ./nixpkgs }:
import ./nixpkgs/dev-shell.nix { pkgs = environ.pkgs; system = builtins.currentSystem; }
    