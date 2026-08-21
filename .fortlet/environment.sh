rust_version=1.97.1
rust_date=2026-07-16
jj_version=0.43.0
zig_version=0.15.2

case "$FORTLET_TARGET" in
  linux/aarch64)
    rust_target=aarch64-unknown-linux-gnu
    rust_sha256=9a7a2c336b4787f1b72f6bab7c35d5b7af2fd03cbd39b4fc721466a70d402a7d
    jj_target=aarch64-unknown-linux-musl
    jj_sha256=289197b6bec60b4e57d47260624b617716f737eb02cdfd9155791b2576aa5862
    debian_triplet=aarch64-linux-gnu
    debian_arch=arm64
    libcap_runtime_sha256=24e74ad29a37d2a3940b8977d11298a7afc77379ef414b561d79c64147d740e0
    libcap_development_sha256=92ac2d723583ac9a34340f00c61adbf6a3ae613ec395541bc32d428f6c16c092
    zig_target=aarch64-linux
    zig_sha256=958ed7d1e00d0ea76590d27666efbf7a932281b3d7ba0c6b01b0ff26498f667f
    ;;
  linux/x86_64)
    rust_target=x86_64-unknown-linux-gnu
    rust_sha256=88f28fa9af20594179f85d6df67078dfd6fa93e2f6da5e1e9b0ac4997988ca4f
    jj_target=x86_64-unknown-linux-musl
    jj_sha256=59e5588583ac82b623239929368c65b90735931c0f26b5a16c1f04d5bb97643d
    debian_triplet=x86_64-linux-gnu
    debian_arch=amd64
    libcap_runtime_sha256=b4b54769c77e4a71c8b33aee4d600ba28a9994a1c6f60d55d4ebe7fc44882e07
    libcap_development_sha256=50674ccc126009f8d640a9230db4600d6fe552b68077193f234ea892784db5d5
    zig_target=x86_64-linux
    zig_sha256=02aa270f183da276e5b5920b1dac44a63f1a49e55050ebde3aecc9eb82f93239
    ;;
  *)
    echo "unsupported Fortlet target: $FORTLET_TARGET" >&2
    exit 1
    ;;
esac

temporary="$(mktemp -d "$FORTLET_OUTPUT/.fortlet-work.XXXXXX")"
trap 'rm -rf "$temporary"' EXIT

extract_debian_artifact() {
  package=$1
  sha256=$2
  url="https://deb.debian.org/debian/pool/main/libc/libcap-ng/${package}_0.8.3-1+b3_${debian_arch}.deb"
  archive="$temporary/$package.deb"
  curl --proto '=https' --tlsv1.2 -LsSf -o "$archive" "$url"
  printf '%s  %s\n' "$sha256" "$archive" | sha256sum -c -
  bsdtar -xOf "$archive" data.tar.xz | tar -xJf - -C "$FORTLET_OUTPUT"
}
extract_debian_artifact libcap-ng0 "$libcap_runtime_sha256"
extract_debian_artifact libcap-ng-dev "$libcap_development_sha256"

zig_archive="zig-$zig_target-$zig_version.tar.xz"
zig_url="https://ziglang.org/download/$zig_version/$zig_archive"
curl --proto '=https' --tlsv1.2 -LsSf -o "$temporary/$zig_archive" "$zig_url"
printf '%s  %s\n' "$zig_sha256" "$temporary/$zig_archive" | sha256sum -c -
mkdir -p "$FORTLET_OUTPUT/zig"
tar -xJf "$temporary/$zig_archive" -C "$FORTLET_OUTPUT/zig" --strip-components=1
mkdir -p "$FORTLET_OUTPUT/bin"
cat > "$FORTLET_OUTPUT/bin/cc" <<'EOF'
#!/bin/sh
exec /opt/fortlet/project/zig/zig cc "$@"
EOF
cat > "$FORTLET_OUTPUT/bin/c++" <<'EOF'
#!/bin/sh
exec /opt/fortlet/project/zig/zig c++ "$@"
EOF
chmod 755 "$FORTLET_OUTPUT/bin/cc" "$FORTLET_OUTPUT/bin/c++"

make_layer_relative() {
  link=$1
  expected_target=$2
  link_target="$(readlink "$link")"
  if [ "$link_target" != "$expected_target" ] || [ ! -f "$FORTLET_OUTPUT$expected_target" ]; then
    echo "unexpected Debian development link: $link" >&2
    exit 1
  fi
  ln -snf "../../..$expected_target" "$link"
}
make_layer_relative \
  "$FORTLET_OUTPUT/usr/lib/$debian_triplet/libcap-ng.so" \
  "/lib/$debian_triplet/libcap-ng.so.0.0.0"
make_layer_relative \
  "$FORTLET_OUTPUT/usr/lib/$debian_triplet/libdrop_ambient.so" \
  "/lib/$debian_triplet/libdrop_ambient.so.0.0.0"

mkdir -p "$FORTLET_OUTPUT/lib"
cp -L "$FORTLET_OUTPUT"/usr/lib/*/libcap-ng.so* "$FORTLET_OUTPUT/lib/"

rust_archive="rust-$rust_version-$rust_target.tar.xz"
rust_url="https://static.rust-lang.org/dist/$rust_date/$rust_archive"
curl --proto '=https' --tlsv1.2 -LsSf -o "$temporary/$rust_archive" "$rust_url"
printf '%s  %s\n' "$rust_sha256" "$temporary/$rust_archive" | sha256sum -c -
tar -xJf "$temporary/$rust_archive" -C "$temporary"
"$temporary/rust-$rust_version-$rust_target/install.sh" \
  --prefix="$FORTLET_OUTPUT" \
  --without=rust-docs \
  --disable-ldconfig

jj_archive="jj-v$jj_version-$jj_target.tar.gz"
jj_url="https://github.com/jj-vcs/jj/releases/download/v$jj_version/$jj_archive"
curl --proto '=https' --tlsv1.2 -LsSf -o "$temporary/$jj_archive" "$jj_url"
printf '%s  %s\n' "$jj_sha256" "$temporary/$jj_archive" | sha256sum -c -
tar -xzf "$temporary/$jj_archive" -C "$temporary" ./jj
install -m 755 "$temporary/jj" "$FORTLET_OUTPUT/bin/jj"

"$FORTLET_OUTPUT/bin/rustc" --version
"$FORTLET_OUTPUT/bin/cargo" --version
"$FORTLET_OUTPUT/bin/jj" --version
