set -eu

if test "${1:-signal}" = quiet; then
  printf '%s' "$$" >"$2"
  exec cat
fi

if test "${1:-signal}" = no-redraw; then
  dd if=/dev/zero bs=8192 count=1 2>/dev/null
  exec cat
fi

if test "${1:-signal}" = await-exit-command; then
  stty raw -echo
  printf 'size:%s\n' "$2"
  received="$(dd bs=1 count=6 2>/dev/null)"
  expected="$(printf '/exit\r')"
  if test "$received" = "$expected"; then
    exit 23
  fi
  exit 64
fi

if test "${1:-signal}" = exit-command; then
  on_resize() {
    dimensions="$(stty size)"
    trap - WINCH
    exec /bin/sh "$0" await-exit-command "$dimensions"
  }

  trap on_resize WINCH
  stty size
  while :; do
    read -r ignored || :
  done
fi

if test "${1:-signal}" = ignore-exit; then
  stty raw -echo
  printf '%s' "$$" >"$2"
  printf 'ready\n'
  dd bs=1 count=6 >/dev/null 2>&1
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
