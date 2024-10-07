{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
  
  buildInputs = with pkgs; [
    SDL2
    zig_0_12
  ];
}
