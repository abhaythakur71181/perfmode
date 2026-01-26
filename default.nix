{ pkgs ? import <nixpkgs> { } }:

pkgs.rustPlatform.buildRustPackage {
  pname = "perfmode";
  version = "0.1.0";

  src = pkgs.fetchFromGitHub {
    owner = "abhaythakur71181";
    repo = "perfmode";
    rev = "main";
    sha256 = "sha256-puswpidWJfurMo+8HD6++XesO4zEmqadZVIPq0j9mBs=";
  };
  cargoHash = "sha256-puswpidWJfurMo+8HD6++XesO4zEmqadZVIPq0j9mBs=";
}
