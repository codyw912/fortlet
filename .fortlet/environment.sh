rust_version=1.97.1
rust_date=2026-07-16
jj_version=0.43.0

case "$FORTLET_TARGET" in
  linux/aarch64)
    rust_target=aarch64-unknown-linux-gnu
    rust_sha256=9a7a2c336b4787f1b72f6bab7c35d5b7af2fd03cbd39b4fc721466a70d402a7d
    jj_target=aarch64-unknown-linux-musl
    jj_sha256=289197b6bec60b4e57d47260624b617716f737eb02cdfd9155791b2576aa5862
    ;;
  linux/x86_64)
    rust_target=x86_64-unknown-linux-gnu
    rust_sha256=88f28fa9af20594179f85d6df67078dfd6fa93e2f6da5e1e9b0ac4997988ca4f
    jj_target=x86_64-unknown-linux-musl
    jj_sha256=59e5588583ac82b623239929368c65b90735931c0f26b5a16c1f04d5bb97643d
    ;;
  *)
    echo "unsupported Fortlet target: $FORTLET_TARGET" >&2
    exit 1
    ;;
esac

temporary="$(mktemp -d)"
trap 'rm -rf "$temporary"' EXIT

(
  cd "$temporary"
  apt-get update -qq
  apt-get download "libcap-ng0=0.8.3-1+b3" "libcap-ng-dev=0.8.3-1+b3"
  for package in ./*.deb; do
    dpkg-deb -x "$package" "$FORTLET_OUTPUT"
  done
)
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
