{
  description = "A very basic rust flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    flake-utils.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let pkgs = nixpkgs.legacyPackages.${system};
      in {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [ cargo rustc rust-analyzer rustfmt ];
          env = {
            POLARS_FMT_MAX_ROWS = 50;
            # POLARS_FMT_MAX_COLS = 50;
          };
        };
      });
}
