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
      toolchain = with fenix.packages.${system};
            combine [
              minimal.rustc
              minimal.cargo
              targets.wasm32-unknown-unknown.latest.rust-std
            ];
    in {
      devShells.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          trunk
          toolchain
        ];
      };
    });
}
