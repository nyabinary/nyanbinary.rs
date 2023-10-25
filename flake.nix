{
  inputs = {
    nixpkgs = {
      url = "github:NixOS/nixpkgs/nixos-unstable";
    };
    flake-utils = {
      url = "github:numtide/flake-utils";
    };
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    fenix,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {inherit system;};
    in {
      formatter = pkgs.alejandra;
      devShell = pkgs.mkShell {
        buildInputs = with pkgs; [
          trunk
          (with fenix.packages.${system};
            combine [
              complete.rustc
              complete.cargo
              complete.rustfmt
              complete.rust-analyzer
              complete.clippy
              complete.miri
              complete.rust-src
              targets.wasm32-unknown-unknown.latest.rust-std
            ])
        ];
      };
    });
}
