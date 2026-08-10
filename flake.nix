{
  description = "Project-scoped isolation for coding-agent CLIs";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }:
    let
      systems = [
        "aarch64-darwin"
        "x86_64-linux"
      ];
      forAllSystems = nixpkgs.lib.genAttrs systems;
    in
    {
      packages = forAllSystems (system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
          fortlet = pkgs.callPackage ./package.nix { };
        in
        {
          default = fortlet;
          inherit fortlet;
        });

      apps = forAllSystems (system: {
        default = {
          type = "app";
          program = "${self.packages.${system}.fortlet}/bin/fortlet";
        };
      });

      checks = forAllSystems (system: {
        package = self.packages.${system}.fortlet;
      });

      devShells = forAllSystems (system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in
        {
          default = pkgs.mkShell {
            packages = with pkgs; [
              cargo
              clippy
              jq
              nixpkgs-fmt
              rustc
              rustfmt
            ];
            shellHook = nixpkgs.lib.optionalString pkgs.stdenv.isDarwin ''
              export SDKROOT="$(xcrun --show-sdk-path)"
              export LIBRARY_PATH="$SDKROOT/usr/lib"
            '';
          };
        });

      formatter = forAllSystems (system:
        nixpkgs.legacyPackages.${system}.nixpkgs-fmt);
    };
}
