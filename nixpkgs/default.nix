let
    oxalicaRust = builtins.fetchTarball {
        name    = "oxalica-overlay";
        url     = https://github.com/oxalica/rust-overlay/archive/f0ddbb639e2f23529ced79602cee6eb3903fc888.tar.gz;
        sha256  = "1k8lbawzvjls9by2fsw8gbdbmgqfmif9fay5bi1wf14nwm6654q4";
    };
 
    overlays = [
        (import "${oxalicaRust}")
    ];
in
    {   
        pkgs = import <nixpkgs> { inherit overlays; };
    }
