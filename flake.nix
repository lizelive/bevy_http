{
  description = "Application packaged using poetry2nix";

  inputs = {
    nix-ai.url = "github:lizelive/nix-ai";
    flake-utils.follows = "nix-ai/flake-utils";
  };

  outputs = { self, nix-ai, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        formatter = nix-ai.formatter.${system};
        pkgs = nix-ai.legacyPackages.${system};
        devShells = nix-ai.devShells.${system};
        # packages = nix-ai.packages.${system};
      in
      {
        devShells.default = devShells.aio;
        inherit formatter;
      });
}
