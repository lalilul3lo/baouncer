{
  description = "Flake for baouncer cli";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  inputs.rust-overlay.url = "github:oxalica/rust-overlay";

  inputs.flake-utils.url = "github:numtide/flake-utils";

  inputs.nil.url = "github:oxalica/nil/c8e8ce72442a164d89d3fdeaae0bcc405f8c015a";

  inputs.nil.flake = true;

  outputs =
    {
      nil,
      nixpkgs,
      rust-overlay,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        nix-lsp-server = nil.packages.${system}.nil;
      in
      {
        devShells.default =
          with pkgs;
          mkShell {
            buildInputs = [
              nix-lsp-server
              rust-analyzer
              openssl
              rust-bin.beta.latest.default
            ];
          };
      }
    );
}
