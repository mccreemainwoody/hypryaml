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
  } @ _:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {inherit system;};
        hypryaml = import ./default.nix pkgs;
      in {
        formatter = pkgs.alejandra;
        packages = rec {
          inherit hypryaml;
          default = hypryaml;
        };
        devShell = import ./shell.nix pkgs;
      }
    );
}
