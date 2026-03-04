{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs  }:
    let
      inherit self;
      system = "x86_64-linux";

      pkgs = import nixpkgs {
        inherit system;

        config = {
          allowUnfree = true;
          android_sdk.accept_license = true;
        };
      };
    in
    {
      devShells.${system} = rec {
        default = desktop;

        desktop = pkgs.mkShell rec {
          nativeBuildInputs = with pkgs; [
            git

            cargo
            rust-analyzer
            clippy
            rustfmt
            rustc
          ];

          # runtime dependencies
          buildInputs = with pkgs; [
            pkg-config
            wayland-scanner
            clang
            pkg-config
            udev
            vulkan-headers
            vulkan-loader
            libGL
            libusb1
            libayatana-appindicator
            libdrm
            mesa
            wayland
            wayland-protocols
            pipewire
            libpulseaudio
            alsa-lib
            dbus
            libxkbcommon
            xorg.libX11
            xorg.libXScrnSaver
            xorg.libXcursor
            xorg.libXext
            xorg.libXfixes
            xorg.libXi
            xorg.libXrandr
          ];

          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
        };
      };
    };
}
