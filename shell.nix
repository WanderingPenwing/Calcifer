{ pkgs ? import <nixpkgs> { overlays = [ (import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz)) ]; },}:
with pkgs;

mkShell {
  nativeBuildInputs = with xorg; [
    libxcb
    libXcursor
    libXrandr
    libXi
    pkg-config
  ] ++ [
    cargo
    rustc
    atk
    gdk-pixbuf
    webkitgtk
    glib
    libGL
    libGLU
    libxkbcommon
    gtk3-x11
    gnome.zenity
  ];
  buildInputs = [
    latest.rustChannels.stable.rust
    xorg.libX11
    wayland
    libxkbcommon
    python3Packages.virtualenv
    python3Packages.plyer
    python3Packages.pygobject3
    python3Packages.pillow
  ];

  shellHook = ''
      export LD_LIBRARY_PATH=/run/opengl-driver/lib/:${lib.makeLibraryPath ([libGL libGLU libxkbcommon])}
  '';
}
