{ pkgs, nixpkgs, guestSystem }:

let
  lib = pkgs.lib;
  guest = nixpkgs.legacyPackages.${guestSystem};
  contract = "fip0013-1";
  imageReference = "fortlet-runtime:${contract}-${guestSystem}";
  runtimeLibraryPath = lib.makeLibraryPath [ guest.stdenv.cc.cc.lib guest.glibc guest.zlib ];

  platform = {
    aarch64-linux = {
      architecture = "arm64";
      codexTarget = "aarch64-unknown-linux-musl";
      codexHash = "sha256-icv3m9Wub5xY2kfoB58xHIQhk1DJxDwHDULz6bKoFAE=";
      fhsLoader = "/lib/ld-linux-aarch64.so.1";
      tactTarget = "aarch64-unknown-linux-gnu";
      tactHash = "sha256-TLKA5jv6+NboXqUQIuHLsaV+hizQ2XzJorDVLqrG6Wg=";
    };
    x86_64-linux = {
      architecture = "amd64";
      codexTarget = "x86_64-unknown-linux-musl";
      codexHash = "sha256-vXWNU9VuQdxl4EX0WJ33mgOO0ZegEa3LUqJY5q1kz9o=";
      fhsLoader = "/lib64/ld-linux-x86-64.so.2";
      tactTarget = "x86_64-unknown-linux-gnu";
      tactHash = "sha256-SCQONX12JOcW6b2OxxVP4TyIEJccooOI4CXczS85G54=";
    };
  }.${guestSystem} or (throw "unsupported Fortlet guest system ${guestSystem}");

  runtimeSupport = pkgs.runCommand "fortlet-runtime-support-${contract}" { } ''
    mkdir -p "$out/bin" "$out/etc/fortlet" "$out/etc/nix"
    cat > "$out/bin/fortlet-hold" <<'EOF'
    #!${guest.bash}/bin/bash
    trap 'exit 0' TERM INT
    while :; do
      ${guest.coreutils}/bin/sleep 3600 &
      wait $!
    done
    EOF
    chmod 0555 "$out/bin/fortlet-hold"

    cat > "$out/etc/fortlet/bash-env" <<'EOF'
    case ":$PATH:" in
      *:/.msb/scripts:*) PATH="/.msb/scripts:$FORTLET_MANAGED_PATH" ;;
      *) PATH="$FORTLET_MANAGED_PATH" ;;
    esac
    export PATH
    EOF
    chmod 0444 "$out/etc/fortlet/bash-env"

    cat > "$out/etc/nix/nix.conf" <<'EOF'
    accept-flake-config = false
    experimental-features = nix-command flakes
    flake-registry =
    require-sigs = false
    sandbox = false
    EOF
    chmod 0444 "$out/etc/nix/nix.conf"
  '';

  runtimeRoot = pkgs.buildEnv {
    name = "fortlet-runtime-root-${contract}-${guestSystem}";
    paths = [
      guest.bash
      guest.bubblewrap
      guest.cacert
      guest.coreutils
      guest.curl
      guest.findutils
      guest.gawk
      guest.gitMinimal
      guest.gnugrep
      guest.gnused
      guest.gnutar
      guest.gzip
      guest.libarchive
      guest.nix
      guest.xz
      runtimeSupport
    ];
    pathsToLink = [ "/bin" "/etc" ];
  };

  runtimeClosure = pkgs.closureInfo { rootPaths = [ runtimeRoot ]; };

  runtimeNar = pkgs.runCommand "fortlet-runtime-seed-${contract}-${guestSystem}"
    { nativeBuildInputs = [ pkgs.coreutils pkgs.gzip pkgs.nix ]; }
    ''
      mkdir -p "$out"
      ${pkgs.nix}/bin/nix-store --export \
        $(cat ${runtimeClosure}/store-paths) \
        | gzip -n > "$out/runtime.nar.gz"
      sha256sum "$out/runtime.nar.gz" | cut -d ' ' -f 1 > "$out/runtime.nar.gz.sha256"
    '';

  runtimeImage = pkgs.dockerTools.buildLayeredImage {
    name = "fortlet-runtime";
    tag = "${contract}-${guestSystem}";
    contents = [ runtimeRoot ];
    extraCommands = ''
      mkdir -p bin etc home/agent tmp usr/bin ".$(dirname ${platform.fhsLoader})"
      ln -sf ${guest.bash}/bin/bash bin/bash
      ln -sf ${guest.bash}/bin/bash bin/sh
      ln -sf ${guest.coreutils}/bin/env usr/bin/env
      ln -sf ${guest.stdenv.cc.bintools.dynamicLinker} ".${platform.fhsLoader}"
      printf 'root:x:0:0:root:/root:/bin/sh\n' > etc/passwd
      printf 'root:x:0:\n' > etc/group
      chmod 1777 tmp
    '';
    config = {
      Cmd = [ "${runtimeSupport}/bin/fortlet-hold" ];
      Env = [
        "PATH=${runtimeRoot}/bin"
        "NIX_SSL_CERT_FILE=${guest.cacert}/etc/ssl/certs/ca-bundle.crt"
        "SSL_CERT_FILE=${guest.cacert}/etc/ssl/certs/ca-bundle.crt"
        "LD_LIBRARY_PATH=${runtimeLibraryPath}"
      ];
      WorkingDir = "/home/agent";
    };
  };

  codexVersion = "0.147.0";
  codexSource = pkgs.fetchurl {
    url = "https://github.com/openai/codex/releases/download/rust-v${codexVersion}/codex-package-${platform.codexTarget}.tar.gz";
    hash = platform.codexHash;
  };
  codex = pkgs.runCommand "fortlet-codex-${codexVersion}-${guestSystem}"
    { nativeBuildInputs = [ pkgs.gnutar pkgs.gzip pkgs.patchelf ]; }
    ''
      mkdir -p "$out"
      tar -xzf ${codexSource} -C "$out"
      patchelf \
        --set-interpreter ${guest.stdenv.cc.bintools.dynamicLinker} \
        --set-rpath ${lib.makeLibraryPath [ guest.glibc guest.ncurses ]} \
        "$out/codex-resources/zsh/bin/zsh"
      chmod -R a-w "$out"
    '';

  tactVersion = "0.3.7";
  tactSource = pkgs.fetchurl {
    url = "https://github.com/clabby/tact/releases/download/v${tactVersion}/tact-${platform.tactTarget}-v${tactVersion}.tar.gz";
    hash = platform.tactHash;
  };
  tact = pkgs.runCommand "fortlet-tact-${tactVersion}-${guestSystem}"
    { nativeBuildInputs = [ pkgs.gnutar pkgs.gzip pkgs.patchelf ]; }
    ''
      mkdir -p "$out/bin"
      tar -xzf ${tactSource} -C "$out" --strip-components=1 \
        "tact-${platform.tactTarget}-v${tactVersion}/tact"
      mv "$out/tact" "$out/bin/tact"
      patchelf \
        --set-interpreter ${guest.stdenv.cc.bintools.dynamicLinker} \
        --set-rpath ${lib.makeLibraryPath [ guest.stdenv.cc.cc.lib guest.glibc ]} \
        "$out/bin/tact"
      chmod 0555 "$out/bin/tact"
    '';

  mkClosureBundle = name: version: root: executable:
    let closure = pkgs.closureInfo { rootPaths = [ root ]; };
    in pkgs.runCommand "fortlet-${name}-closure-${version}-${guestSystem}"
      { nativeBuildInputs = [ pkgs.coreutils pkgs.gzip pkgs.jq pkgs.nix ]; }
      ''
        mkdir -p "$out"
        ${pkgs.nix}/bin/nix-store --export \
          $(cat ${closure}/store-paths) \
          | gzip -n > "$out/closure.nar.gz"
        archive_sha256="$(sha256sum "$out/closure.nar.gz" | cut -d ' ' -f 1)"
        jq -n \
          --arg contract "${contract}" \
          --arg name "${name}" \
          --arg version "${version}" \
          --arg system "${guestSystem}" \
          --arg executable "${executable}" \
          --arg archive_sha256 "$archive_sha256" \
          --slurpfile store_paths <(jq -R . < ${closure}/store-paths) \
          '{
            schema: 1,
            contract: $contract,
            name: $name,
            version: $version,
            system: $system,
            executable: $executable,
            archive_sha256: $archive_sha256,
            store_paths: $store_paths
          }' > "$out/manifest.json"
      '';

  codexBundle = mkClosureBundle "codex" codexVersion codex "${codex}/bin/codex";
  tactBundle = mkClosureBundle "tact" tactVersion tact "${tact}/bin/tact";

  bundle = pkgs.runCommand "fortlet-runtime-artifacts-${contract}-${guestSystem}"
    { nativeBuildInputs = [ pkgs.coreutils pkgs.gnutar pkgs.jq pkgs.skopeo ]; }
    ''
      mkdir -p "$out/harnesses/codex" "$out/harnesses/tact" "$out/runtime"
      skopeo --insecure-policy copy \
        "docker-archive:${runtimeImage}" \
        "oci-archive:$out/runtime/image.oci.tar:${imageReference}"
      cp ${runtimeNar}/runtime.nar.gz "$out/runtime/runtime.nar.gz"
      cp ${codexBundle}/closure.nar.gz "$out/harnesses/codex/closure.nar.gz"
      cp ${codexBundle}/manifest.json "$out/harnesses/codex/manifest.json"
      cp ${tactBundle}/closure.nar.gz "$out/harnesses/tact/closure.nar.gz"
      cp ${tactBundle}/manifest.json "$out/harnesses/tact/manifest.json"

      inspection="$(mktemp -d)"
      tar -xf "$out/runtime/image.oci.tar" -C "$inspection" index.json
      image_digest="$(jq -er '.manifests | if length == 1 then .[0].digest else error("expected one image manifest") end' "$inspection/index.json")"
      image_archive_sha256="$(sha256sum "$out/runtime/image.oci.tar" | cut -d ' ' -f 1)"
      seed_archive_sha256="$(sha256sum "$out/runtime/runtime.nar.gz" | cut -d ' ' -f 1)"
      jq -n \
        --arg contract "${contract}" \
        --arg system "${guestSystem}" \
        --arg architecture "${platform.architecture}" \
        --arg image_reference "${imageReference}" \
        --arg image_digest "$image_digest" \
        --arg image_archive_sha256 "$image_archive_sha256" \
        --arg seed_archive_sha256 "$seed_archive_sha256" \
        --arg runtime_root "${runtimeRoot}" \
        --arg nix_version "${guest.nix.version}" \
        --arg nix "${guest.nix}/bin/nix" \
        --arg nix_store "${guest.nix}/bin/nix-store" \
        --arg shell "${guest.bash}/bin/bash" \
        --arg hold "${runtimeSupport}/bin/fortlet-hold" \
        --arg managed_bash_env "${runtimeSupport}/etc/fortlet/bash-env" \
        --arg runtime_library_path "${runtimeLibraryPath}" \
        --arg ca_bundle "${guest.cacert}/etc/ssl/certs/ca-bundle.crt" \
        --slurpfile store_paths <(jq -R . < ${runtimeClosure}/store-paths) \
        '{
          schema: 1,
          contract: $contract,
          system: $system,
          architecture: $architecture,
          image_reference: $image_reference,
          image_digest: $image_digest,
          image_archive_sha256: $image_archive_sha256,
          seed_archive_sha256: $seed_archive_sha256,
          runtime_root: $runtime_root,
          nix_version: $nix_version,
          nix: $nix,
          nix_store: $nix_store,
          shell: $shell,
          hold: $hold,
          managed_bash_env: $managed_bash_env,
          runtime_library_path: $runtime_library_path,
          ca_bundle: $ca_bundle,
          store_paths: $store_paths
        }' > "$out/runtime/manifest.json"
    '';
in
{
  inherit bundle codex codexVersion guestSystem imageReference runtimeRoot tact tactVersion;
}
