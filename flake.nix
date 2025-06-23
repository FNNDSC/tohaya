{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, flake-utils, nixpkgs }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = (import nixpkgs) {
          inherit system;
        };
      in
      with pkgs;
      {
        packages.default = rustPlatform.buildRustPackage {
          pname = "tohaya";
          version = self.shortRev or self.dirtyShortRev;
          src = ./.;
          cargoLock = {
            lockFile = ./Cargo.lock;
            outputHashes = {
              "biblib-0.2.4" = "sha256-+hxrnmTZFFx9Xifxy4am7FRs4HFQ3ejhmdpAVh+Y7F8=";
            };
          };
          nativeBuildInputs = [ pkg-config ];
          doCheck = false;
        };
      });
}
