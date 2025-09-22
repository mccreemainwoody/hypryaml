{
    description = "hypryaml flake definition";

    inputs = {
        systems.url = "systems";

        nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
        flake-utils = {
            url = "github:numtide/flake-utils";
            inputs.systems.follows = "systems";
        };
    };

    outputs = { self, nixpkgs, flake-utils, ... } @ _:
        flake-utils.lib.eachDefaultSystem (system: 
            let
                pkgs = nixpkgs.legacyPackages.${system};
            in
            {
                packages = rec {
                    hypryaml = pkgs.rustPlatform.buildRustPackage
                    {
                        pname = "hypryaml";
                        version = "0.2.0";

                        src = ./.; 
                        cargoLock = { lockFile = ./Cargo.lock; };

                        meta = {
                            description = "Dynamically set Hypr values using YAML !";
                            license = "MIT";
                            homepage = "https://github.com/mccreemainwoody/hypryaml";
                            systems = flake-utils.defaultSystems;
                        };
                    };
                    default = hypryaml;
                };
                devShell = pkgs.mkShell {
                    packages = with pkgs;
                        [ rustc cargo rustfmt rust-analyzer ];
                };
            }
        );
}
