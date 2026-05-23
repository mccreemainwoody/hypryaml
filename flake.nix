{
  description = "hypryaml flake definition";

  inputs = {
    systems.url = "systems";
    nixpkgs.url = "nixpkgs/nixos-unstable";
    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs.systems.follows = "systems";
    };
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    ...
  } @ _: let
    globals = {
      overlays.default = import ./nix/overlay.nix {};
    };
    systemSpecific = flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {inherit system;};
        hypryaml = pkgs.callPackage ./default.nix {};
      in {
        formatter = pkgs.alejandra;
        packages = rec {
          inherit hypryaml;
          default = hypryaml;
        };
        devShell = pkgs.callPackage ./shell.nix {};
      }
    );
  in
    globals // systemSpecific;
}
