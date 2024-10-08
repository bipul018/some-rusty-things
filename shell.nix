{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
  buildInputs = with pkgs; [
    SDL2 SDL2_ttf SDL2_gfx SDL2_image SDL2_mixer
    rustc cargo
    linuxKernel.packages.linux_latest_libre.perf
  ];
}
