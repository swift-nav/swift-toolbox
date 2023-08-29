{ pkgs ? import <nixpkgs> {} }:

(pkgs.buildFHSUserEnv {
  name = "swift-toolbox";
  targetPkgs = pkgs: (with pkgs;
    [ imagemagick
      libxcrypt-legacy
      glib
      capnproto
      openssl
      pkg-config
      cmake
      clang
      libglvnd
      libxkbcommon
      nss
      nspr
      wayland
      fontconfig
      freetype
      expat
      alsa-lib
      dbus
      libkrb5
      zlib
      gdb
    ]) ++ (with pkgs.xorg;
    [ libX11
      libXcursor
      libXrandr
      libxkbfile
      libXcomposite
      libXdamage
      libXext
      libXfixes
      libXrender
      libXtst
      libxcb
      xcbutilkeysyms
      xcbutilimage
      xcbutilwm
      xcbutilrenderutil
      libXi
      libxshmfence
    ]);
  profile = ''
    unset QT_QPA_PLATFORMTHEME
    unset QT_STYLE_OVERRIDE
    unset QTWEBKIT_PLUGIN_PATH
    unset QT_PLUGIN_PATH

    export PKG_CONFIG_PATH="${pkgs.openssl.dev}/lib/pkgconfig"
    export LIBCLANG_PATH="${pkgs.llvmPackages_11.libclang.lib}/lib"
  '';
  runScript = "bash";
}).env
