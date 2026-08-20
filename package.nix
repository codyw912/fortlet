{ lib
, fetchurl
, makeWrapper
, rustPlatform
, stdenv
,
}:

let
  microsandboxVersion = "0.6.8";

  artifacts = {
    aarch64-darwin = {
      agentdFilename = "agentd-aarch64";
      agentdHash = "sha256-0sg6HAeFg/FHqGib+a5NRbYxg5CWxdaFtQUooNCZBZ0=";
      runtimeFilename = "microsandbox-darwin-aarch64.tar.gz";
      runtimeHash = "sha256-+SrK7uH+/VJxmXvw5SJgDTK0o3J5bR4PAOIp+aj/CJA=";
      libkrunfwFilename = "libkrunfw.5.dylib";
    };
    x86_64-linux = {
      agentdFilename = "agentd-x86_64";
      agentdHash = "sha256-pLQmXA4zSL9/0wVZPzd54iKvrfDPisuMm3w6QDk5FNU=";
      runtimeFilename = "microsandbox-linux-x86_64.tar.gz";
      runtimeHash = "sha256-mSvmbOimGWWzrHczvOWNapioKSoXLoXIdRB0oq0W9p0=";
      libkrunfwFilename = "libkrunfw.so.5.6.1";
    };
  };

  platform = artifacts.${stdenv.hostPlatform.system} or
    (throw "fortlet does not support ${stdenv.hostPlatform.system}");

  releaseUrl = "https://github.com/superradcompany/microsandbox/releases/download/v${microsandboxVersion}";
  agentd = fetchurl {
    url = "${releaseUrl}/${platform.agentdFilename}";
    hash = platform.agentdHash;
  };
  runtimeBundle = fetchurl {
    url = "${releaseUrl}/${platform.runtimeFilename}";
    hash = platform.runtimeHash;
  };
in
rustPlatform.buildRustPackage {
  pname = "fortlet";
  version = "0.1.0";

  src = lib.fileset.toSource {
    root = ./.;
    fileset = lib.fileset.unions [
      ./.fortlet
      ./.github/pull_request_template.md
      ./.github/workflows/verify.yml
      ./AGENTS.md
      ./Cargo.lock
      ./Cargo.toml
      ./flake.nix
      ./OVERVIEW.md
      ./README.md
      ./WORKFLOW.md
      ./arch
      ./docs/RUNBOOK.md
      ./examples
      ./experiments
      ./governance/CHARTER.md
      ./package.nix
      ./src
      ./tests
    ];
  };

  cargoLock.lockFile = ./Cargo.lock;

  postPatch = ''
    patch -d "$cargoDepsCopy" -p1 < ${./nix/microsandbox-reproducible-build.patch}
  '';

  MICROSANDBOX_AGENTD = agentd;
  MICROSANDBOX_SKIP_RUNTIME_INSTALL = 1;

  nativeBuildInputs = [ makeWrapper ];

  dontStrip = true;

  postInstall = ''
    runtime="$out/libexec/fortlet/microsandbox"
    shims="$out/libexec/fortlet/shims"
    mkdir -p "$runtime"
    mkdir -p "$shims"
    tar -xzf ${runtimeBundle} -C "$runtime"

    for harness in codex tact; do
      ln -s "$out/bin/fortlet" "$shims/$harness"
    done

    wrapProgram "$out/bin/fortlet" \
      --set MSB_PATH "$runtime/msb" \
      --set MSB_LIBKRUNFW_PATH "$runtime/${platform.libkrunfwFilename}" \
      --set FORTLET_SHIM_DIR "$shims"
  '';

  doInstallCheck = true;
  installCheckPhase = ''
    shims="$out/libexec/fortlet/shims"
    runtime="$out/libexec/fortlet/microsandbox"
    test_home="$(mktemp -d)"
    runtime_check="$(mktemp -d)"
    trap 'rm -rf "$test_home" "$runtime_check"' EXIT

    tar -xzf ${runtimeBundle} -C "$runtime_check"
    cmp "$runtime_check/msb" "$runtime/msb"
    cmp \
      "$runtime_check/${platform.libkrunfwFilename}" \
      "$runtime/${platform.libkrunfwFilename}"

    env -i HOME="$test_home" PATH="$out/bin" \
      "$out/bin/fortlet" --help >/dev/null

    for harness in codex tact; do
      test -L "$shims/$harness"
      test "$shims/$harness" -ef "$out/bin/fortlet"
      discovered="$(env -i PATH="$shims" ${stdenv.shell} -c "command -v $harness")"
      test "$discovered" = "$shims/$harness"
      if env -i HOME="$test_home" PATH="$shims" \
        "$shims/$harness" --version >/dev/null 2>"$test_home/$harness.err"; then
        echo "$harness unexpectedly launched without test credentials" >&2
        exit 1
      fi
      grep -q '^fortlet: credentials stage failed;' "$test_home/$harness.err"
    done
  '';

  strictDeps = true;

  meta = {
    description = "Project-scoped isolation for coding-agent CLIs";
    license = with lib.licenses; [ mit asl20 ];
    mainProgram = "fortlet";
    platforms = builtins.attrNames artifacts;
  };
}
