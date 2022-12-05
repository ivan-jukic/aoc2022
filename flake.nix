{
    inputs = {
        cargo2nix.url           = "github:cargo2nix/cargo2nix/release-0.11.0";
        flake-utils.follows     = "cargo2nix/flake-utils";
        nixpkgs.follows         = "cargo2nix/nixpkgs";
        oxalica-overlay.url     = "github:oxalica/rust-overlay";
        dev-shell               = {
            url = "path:./nixpkgs/dev-shell.nix";
            flake = false;
        };
    };

    outputs = inputs: with inputs;
        # Build the output set for each default system and map system sets into
        # attributes, resulting in paths such as:
        # nix build .#packages.x86_64-linux.<name>
        flake-utils.lib.eachDefaultSystem (system:
            let

                # create nixpkgs that contains rustBuilder from cargo2nix overlay
                overlays    = [cargo2nix.overlays.default (import oxalica-overlay)];
                pkgs        = import nixpkgs { inherit system overlays; };
                devShell    = (import dev-shell { inherit pkgs system; }).devShell;

            in rec {
                inherit devShell;
            }
        );
}
