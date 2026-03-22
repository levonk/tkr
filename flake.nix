{
  description = "A modern Rust CLI ticket management system";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils/v1.0.0";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        
        # Package definition for tkr
        tkr = pkgs.rustPlatform.buildRustPackage {
          pname = "tkr";
          version = "0.1.0";
          src = ./.;
          
          cargoLock = {
            lockFile = ./Cargo.lock;
          };
          
          nativeBuildInputs = with pkgs; [
            pkg-config
          ];
          
          buildInputs = with pkgs; [
            openssl
          ];
          
          meta = with pkgs.lib; {
            description = "A modern Rust CLI ticket management system";
            homepage = "https://github.com/levonk/tkr";
            license = licenses.mit;
            platforms = platforms.all;
            mainProgram = "tkr";
          };
        };
      in
      {
        packages.default = tkr;
        
        devShells.default = pkgs.mkShell {
          packages = [
            pkgs.git
            pkgs.gnumake
            pkgs.rustc
            pkgs.cargo
            pkgs.clippy
            pkgs.rustfmt
            pkgs.openssl
            pkgs.pkg-config
          ];
        };
      }
    );
}
