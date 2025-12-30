{ pkgs ? import <nixpkgs> { } }:

pkgs.stdenv.mkDerivation {
  pname = "perfmode";
  version = "0.1.0";

  src = pkgs.fetchFromGitHub {
    owner = "abhaythakur71181";
    repo = "perfmode";
    rev = "0.1.0";
    sha256 = "sha256-Fpw1Olg3JB2gDwvQM7SFqyjjF9IMnJv90HFpGaTLxa0=";
  };

  buildInputs = [ pkgs.rustc pkgs.cargo ];

  buildPhase = ''
    cargo build --release
  '';

  installPhase = ''
    mkdir -p $out/bin
    cp target/release/perfmode $out/bin/
  '';
}
