{
  # run `nix develop` to use!!
  description = "Flake for setting up my SSG";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";   # overrides rust-overlay's input for named nixpkgs vs top-level
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem(system: 
      let
        # overlays -> fix point; allows adding pkgs to
        #             nixpkgs instance
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      {
        # mkShell -> fn that generates derivation
        #            (shell env) -> shell script that
        #            declares variables (cargo, rust
        #            compiler, etc. in $PATH)
        devShells.default = with pkgs; mkShell {
          buildInputs = [
            rust-bin.beta.latest.default
          ];
        };
      }
    );
}
