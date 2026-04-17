{ pkgs ? import <nixpkgs> { } }:

pkgs.rustPlatform.buildRustPackage {
  pname = "perfmode";
  version = "0.1.2";

  src = pkgs.fetchFromGitHub {
    owner = "abhaythakur71181";
    repo = "perfmode";
    rev = "main";
    sha256 = "sha256-EKn1sw8m0UAoYqsKKmRmbLOrFjyropFwwM681W5XHCw=";
  };
  cargoHash = "sha256-EKn1sw8m0UAoYqsKKmRmbLOrFjyropFwwM681W5XHCw=";
}
