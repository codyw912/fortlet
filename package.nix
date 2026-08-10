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
      ./Cargo.lock
      ./Cargo.toml
      ./arch
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

  postInstall = ''
    runtime="$out/libexec/fortlet/microsandbox"
    mkdir -p "$runtime"
    tar -xzf ${runtimeBundle} -C "$runtime"

    wrapProgram "$out/bin/fortlet" \
      --set MSB_PATH "$runtime/msb" \
      --set MSB_LIBKRUNFW_PATH "$runtime/${platform.libkrunfwFilename}"
  '';

  strictDeps = true;

  meta = {
    description = "Project-scoped isolation for coding-agent CLIs";
    license = with lib.licenses; [ mit asl20 ];
    mainProgram = "fortlet";
    platforms = builtins.attrNames artifacts;
  };
}
