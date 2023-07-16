{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  inputs.rust-overlay.url = "github:oxalica/rust-overlay";
  inputs.rust-overlay.inputs.nixpkgs.follows = "nixpkgs";

  outputs =
    { self, nixpkgs, rust-overlay }:
    {
      devShells.x86_64-linux.default = let
        pkgs = import nixpkgs {
          system = "x86_64-linux";
          overlays = [ rust-overlay.outputs.overlays.default ];
        };
      in pkgs.mkShell {
        nativeBuildInputs = [
          (pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml.example)
        ];
      };
    };
}

