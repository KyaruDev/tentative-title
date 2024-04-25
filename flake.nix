{
  description = "tentative-title's development shell.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {nixpkgs, flake-utils, ...}:
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          pkgs = import nixpkgs {
            inherit system;
          };
        in
          with pkgs; {
            devShells.default = mkShell rec {
              nativeBuildInput = [ pkg-config ];
              buildInputs = [
                alsa-lib udev vulkan-loader
                xorg.libX11 xorg.libXcursor xorg.libXi xorg.libXrandr
                libxkbcommon wayland
              ];
              LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
            };
          }
      );
}
