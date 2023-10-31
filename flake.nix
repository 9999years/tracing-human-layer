{
  description = "Human-friendly console formatter for Rust's tracing logging library";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
    systems.url = "github:nix-systems/default";
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
  };

  nixConfig = {
    extra-substituters = ["https://cache.garnix.io"];
    extra-trusted-substituters = ["https://cache.garnix.io"];
    extra-trusted-public-keys = ["cache.garnix.io:CTFPyKSLcx5RMJKfLo5EEPUObbA78b0YQ2DTCJXqr9g="];
  };

  outputs = inputs @ {
    self,
    nixpkgs,
    systems,
    crane,
    advisory-db,
  }: let
    eachSystem = nixpkgs.lib.genAttrs (import systems);
  in {
    packages = eachSystem (system: let
      pkgs = nixpkgs.legacyPackages.${system};
      inherit (pkgs) lib;
      packages = pkgs.callPackage ./nix/makePackages.nix {inherit inputs;};
    in
      (lib.filterAttrs (name: value: lib.isDerivation value) packages)
      // {
        default = packages.tracing-human-layer;
      });

    checks = eachSystem (system: self.packages.${system}.tracing-human-layer.checks);

    devShells = eachSystem (system: {
      default = self.packages.${system}.tracing-human-layer.devShell;
    });
  };
}
