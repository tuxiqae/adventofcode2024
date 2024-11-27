{
  description = "Automatic EC2 provisioner";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
    nci.url = "github:yusdacra/nix-cargo-integration";
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    devshell = {
      url = "github:numtide/devshell";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    git-hooks-nix = {
      url = "github:cachix/git-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs @ {...}:
    inputs.flake-parts.lib.mkFlake {inherit inputs;} {
      systems = [
        # systems for which you want to build the `perSystem` attributes
        "x86_64-linux"
        "aarch64-darwin"
        # ...
      ];
      imports = [
        inputs.nci.flakeModule
        inputs.devshell.flakeModule
        inputs.treefmt-nix.flakeModule
        inputs.git-hooks-nix.flakeModule
      ];
      perSystem = {
        pkgs,
        config,
        ...
      }: let
        crateName = "aoc_2024_rs";
      in {
        nci = {
          projects.${crateName}.path = ./.;
          crates.${crateName} = {
            export = true;
          };
        };
        devshells.default = {
          devshell.startup.pre-commit.text = config.pre-commit.installationScript;
          packages = with pkgs; [
            cargo
            rustc
            aoc-cli
          ];
        };
        treefmt = {
          projectRootFile = "flake.nix";
          programs = {
            alejandra.enable = true;
            typos.enable = true;
            rustfmt.enable = true;
          };
        };
        pre-commit.settings = {
          hooks = {
            alejandra.enable = true;
            typos.enable = true;
            deadnix.enable = true;
            detect-aws-credentials.enable = true;
            detect-private-keys.enable = true;
            rustfmt.enable = true;
          };
        };
      };
    };
}
