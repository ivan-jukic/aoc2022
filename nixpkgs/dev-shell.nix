args@{ 
    pkgs,
    system,
    rustVer     ? "1.65.0",
    ...
}:
let
    localDevTools       = with pkgs; [ zlib which ];
    linuxSpecificTools  = with pkgs; lib.optionals stdenv.isLinux [ gcc11 ];
    macOsSpecificTools  = with pkgs; lib.optionals stdenv.isDarwin [ clang ];
    rustSrc             = pkgs.rust-bin.stable.${rustVer}.rust-src;
    rustfmtNightly      = pkgs.rust-bin.nightly.latest.rustfmt;
in
{
    devShell = pkgs.mkShellNoCC {
        RUST_SRC_PATH       = "${rustSrc}/lib/rustlib/src/rust/library";   # Fixes rust-analyzer!
        PROJECT_PLATFORM    = system;
        nativeBuildInputs = with pkgs; [
            rust-bin.stable.${rustVer}.default
            # Using nightly rustfmt to allow unstable features!
            rustfmtNightly
            cargo-sort
            libiconv
            gnumake
            jq
            lsof
            ];
        buildInputs = with pkgs; [
            # For fun
            figlet
            lolcat
            ]
            ++ localDevTools
            ++ macOsSpecificTools
            ++ linuxSpecificTools;

        shellHook = ''
            PATH = "${rustfmtNightly}/bin:$PATH"; # Use the nightly `rustfmt`!
            clear
            figlet "AOC 2022" | lolcat --freq 0.5
        '';
    };
}
