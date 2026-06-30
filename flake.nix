{
    description = "fsize: file size tool";

    inputs = {
        nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
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
        overlays = [ (import rust-overlay) ];
    };

    rustToolchain = pkgs.rust-bin.stable.latest.default.override {
        extensions = [ "rust-src" "rust-analyzer" ];
        targets = [ "x86_64-pc-windows-gnu" ];
    };

    commonArgs = {
        pname = "fsize";
        version = "0.1.0";
        src = ./.;
        cargoLock = { lockFile = ./Cargo.lock; };
        nativeBuildInputs = with pkgs; [ pkg-config ];
        buildInputs = [ ];
    };

    linuxPackage = pkgs.rustPlatform.buildRustPackage (commonArgs // {
        meta.mainProgram = "fsize";
    });

    windowsPackage = pkgs.pkgsCross.mingwW64.rustPlatform.buildRustPackage (commonArgs // {
        nativeBuildInputs = commonArgs.nativeBuildInputs ++ [
            pkgs.pkgsCross.mingwW64.windows.pthreads
        ];
        buildInputs = commonArgs.buildInputs ++ [
            pkgs.pkgsCross.mingwW64.windows.pthreads
        ];

        cargoBuildFlags = [ "--target" "x86_64-pc-windows-gnu" ];

        CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER = "${pkgs.pkgsCross.mingwW64.stdenv.cc}/bin/${pkgs.pkgsCross.mingwW64.stdenv.cc.targetPrefix}cc";
    });

    in {
        packages = {
            default = linuxPackage;
            fsize = linuxPackage;
            fsize-windows = windowsPackage;
        };

        devShells.default = pkgs.mkShell {
            buildInputs = [
                rustToolchain
                pkgs.pkgsCross.mingwW64.windows.pthreads
                pkgs.pkgsCross.mingwW64.stdenv.cc
            ];
            shellHook = ''
    echo "Done"
    '';
        };
    }
    );
}
