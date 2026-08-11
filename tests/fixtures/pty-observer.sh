set -eu

if test "${1:-signal}" = quiet; then
  printf '%s' "$$" >"$2"
  exec cat
fi

if test "${1:-signal}" = no-redraw; then
  dd if=/dev/zero bs=8192 count=1 2>/dev/null
  exec cat
fi

if test "${1:-signal}" = resize; then
  on_resize() {
    dimensions="$(stty size)"
    printf 'size:%s\n' "$dimensions"
    trap - WINCH
    exec cat
  }

  trap on_resize WINCH
  stty size
  while :; do
    read -r ignored || :
  done
fi

printf 'ready\n'
exec cat
