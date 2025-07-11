{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, flake-utils, nixpkgs }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = (import nixpkgs) { inherit system; };
      in
      with pkgs;
      {
        packages.default = rustPlatform.buildRustPackage {
          pname = "tohaya";
          version = self.shortRev or self.dirtyShortRev;
          src = ./.;
          cargoLock = {
            lockFile = ./Cargo.lock;
            allowBuiltinFetchGit = true;
          };
          cargoBuildFlags = [ "--bins" ];
        };
        devShell = mkShell {
          buildInputs = [
            # you are assumed to have `rustup` installed globally
            just
            wasm-pack
            live-server
          ];
        };
      });
}
