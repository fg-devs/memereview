{
  description = "Memereview virtual env";

  inputs.devshell.url = "github:numtide/devshell";
  inputs.flake-utils.url = "github:numtide/flake-utils";

  outputs = { self, flake-utils, devshell, nixpkgs }:
    flake-utils.lib.eachDefaultSystem (system: {
      devShell =
        let
          pkgs = import nixpkgs {
            inherit system;

            overlays = [ devshell.overlays.default ];
          };
        in
        pkgs.devshell.mkShell {
          packages = [
            pkgs.openssl
            pkgs.pkg-config
            pkgs.protobuf
            pkgs.rustup
            pkgs.zlib
          ];
        };
    });
}
