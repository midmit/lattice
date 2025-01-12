let
  pkgs = import <nixpkgs> {};
in
pkgs.mkShell {
  packages = [
    pkgs.cargo
    pkgs.rustc

    pkgs.rust-analyzer
    pkgs.rustfmt

    pkgs.libgcc
    pkgs.cmake

    pkgs.libclang
    pkgs.ccls

    pkgs.wayland
    pkgs.wayland-protocols
    pkgs.libxkbcommon
    pkgs.glfw
    pkgs.wayland-scanner
    pkgs.xwayland

    pkgs.xorg.libX11
    pkgs.mesa
  ];

  env = {
    RUST_BACKTRACE = "full";
    LD_LIBRARY_PATH="${pkgs.wayland}/lib:${pkgs.libxkbcommon}/lib:$LD_LIBRARY_PATH"; 
  };
}
