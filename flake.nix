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
          shim-activation = pkgs.runCommand
            "fortlet-shim-activation-${fortlet.version}"
            { }
            ''
              mkdir -p "$out"
              ln -s "${fortlet}/libexec/fortlet/shims" "$out/bin"
            '';
        in
        {
          default = fortlet;
          inherit fortlet shim-activation;
        });

      apps = forAllSystems (system: {
        default = {
          type = "app";
          program = "${self.packages.${system}.fortlet}/bin/fortlet";
        };
      });

      checks = forAllSystems (system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
          fortlet = self.packages.${system}.fortlet;
          shim-activation = self.packages.${system}.shim-activation;
          native-codex = pkgs.writeShellScriptBin "codex" ''
            printf '%s\n' "$@"
          '';
        in
        {
          package = fortlet;
          shim-activation = pkgs.runCommand "fortlet-shim-activation-check"
            { }
            ''
              test -L "${shim-activation}/bin"

              for harness in codex tact; do
                test \
                  "${shim-activation}/bin/$harness" \
                  -ef \
                  "${fortlet}/libexec/fortlet/shims/$harness"
                discovered="$({ PATH="${shim-activation}/bin"; command -v "$harness"; })"
                test "$discovered" = "${shim-activation}/bin/$harness"
              done

              native_output="$(
                PATH="${shim-activation}/bin:${native-codex}/bin" \
                  "${fortlet}/bin/fortlet" native codex -- --probe "two words"
              )"
              expected="$(printf '%s\n' --probe "two words")"
              test "$native_output" = "$expected"

              touch "$out"
            '';
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

          agents = pkgs.mkShell {
            packages = [
              self.packages.${system}.fortlet
              self.packages.${system}.shim-activation
            ];
          };
        });

      formatter = forAllSystems (system:
        nixpkgs.legacyPackages.${system}.nixpkgs-fmt);
    };
}
