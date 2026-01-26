{
  description = "A Nix flake for perfmode (Fan/Performance Control for ASUS TUF Gaming laptops)";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils }:
    let
      supportedSystems = [ "x86_64-linux" "aarch64-linux" ];
    in
    utils.lib.eachDefaultSystem (system:
      if nixpkgs.lib.elem system supportedSystems then
        let
          pkgs = import nixpkgs { inherit system; };
        in
        {
          packages.default = pkgs.rustPlatform.buildRustPackage {
            pname = "perfmode";
            version = "0.1.0";
            src = ./.;
            cargoHash = "sha256-puswpidWJfurMo+8HD6++XesO4zEmqadZVIPq0j9mBs=";
            nativeBuildInputs = [ pkgs.pkg-config ];
          };

          devShells.default = pkgs.mkShell {
            buildInputs = with pkgs; [
              cargo
              rustc
              rustfmt
              clippy
            ];
          };
        }
      else
        { } # Return empty set for unsupported systems
    );
}
