{
  description = "Dev environment for learn-bevy";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {
          inherit system;
        };
      in {
        devShells.default = pkgs.mkShell {
          name = "alsa-udev-shell";
          buildInputs = [
            pkgs.alsa-lib
            pkgs.systemd
            pkgs.pkg-config
          ];
          LD_LIBRARY_PATH = "$LD_LIBRARY_PATH:${
            with pkgs;
              pkgs.lib.makeLibraryPath [
                xorg.libX11
                xorg.libXcursor
                xorg.libXi
                libxkbcommon
                xorg.libxcb
                vulkan-loader
                glfw
              ]
          }";
        };
      }
    );
}
