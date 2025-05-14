let
  pkgs = import <nixpkgs> { };
in
pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    gcc
    nixd
    rustc
    cargo
    rustfmt
    rust-analyzer
  ];
  buildInputs = with pkgs; [ ];

  # Important for runtime library discovery
  shellHook = ''
    export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath [ ]}:$LD_LIBRARY_PATH
  '';
}
