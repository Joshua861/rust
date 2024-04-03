let 
 nixpkgs = import (fetchTarball "channel:nixos-unstable") {};
in
 with nixpkgs;
 stdenv.mkDerivation {
    name = "rust";
    buildInputs = [ 
      cargo
      rustc
      pkg-config
      openssl.dev 
      nix
    ];
    OPENSSL_DEV=openssl.dev;
 }