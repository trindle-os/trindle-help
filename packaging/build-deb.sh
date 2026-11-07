#!/usr/bin/env bash
# Builds the trindle-help Debian package into OUT (first argument, default deb-out).
set -euo pipefail

here="$(cd "$(dirname "$0")" && pwd)"
root="$(dirname "$here")"
out="${1:-deb-out}"
version="$(sed -n 's/^version = "\(.*\)"/\1/p' "$root/Cargo.toml" | head -1)"
arch="$(dpkg --print-architecture)"
stage="$out/trindle-help_${version}_${arch}"
# The first version of the main package that no longer holds Help's files; older ones are replaced.
main_without_help="1:0.9.0~beta1+local285"

# No path of the build computer may end up in the program; the last matching mapping wins.
export RUSTFLAGS="${RUSTFLAGS:-} --remap-path-prefix=$HOME=/src --remap-path-prefix=${CARGO_HOME:-$HOME/.cargo}=/cargo --remap-path-prefix=$root=/src/trindle-help"

cargo build --release --offline --locked --manifest-path "$root/Cargo.toml" --target-dir "$root/target"

needs="$(ldd "$root/target/release/trindle-help" | awk '/=>/ {print $1}' | sort | tr '\n' ' ')"
for lib in $needs; do
    case "$lib" in
        libc.so.6|libgcc_s.so.1|libm.so.6|libxkbcommon.so.0|libwayland-client.so.0|libffi.so.8) ;;
        *) echo "trindle-help now needs $lib: add its package to the Depends line below and to this list ($needs)" >&2; exit 1 ;;
    esac
done

rm -rf "$stage" "$stage.deb"
install -d "$stage/DEBIAN" "$stage/usr/bin" "$stage/usr/share/applications" "$stage/usr/share/trindle/help" "$stage/usr/share/doc/trindle-help"
install -m 0755 "$root/target/release/trindle-help" "$stage/usr/bin/trindle-help"
install -m 0644 "$root/share/applications/trindle-help.desktop" "$stage/usr/share/applications/trindle-help.desktop"
install -m 0644 "$root"/share/help/*.png "$stage/usr/share/trindle/help/"
install -m 0644 "$root/LICENSE" "$stage/usr/share/doc/trindle-help/copyright"
for lang in "$root"/share/lang/*; do
    install -d "$stage/usr/share/trindle/lang/$(basename "$lang")"
    install -m 0644 "$lang"/*.ftl "$stage/usr/share/trindle/lang/$(basename "$lang")/"
done

cat > "$stage/DEBIAN/control" <<CONTROL
Package: trindle-help
Version: $version
Section: doc
Priority: optional
Architecture: $arch
Depends: libc6, libxkbcommon0, libwayland-client0, trindle-toolkit-lang
Replaces: trindle (<< $main_without_help)
Breaks: trindle (<< $main_without_help)
Maintainer: Trindle <help@trindle.org>
Description: Help for Trindle: searchable pages that open offline
 Opens at the page a message points to, or on its own, with no network.
 This works only on a Wayland desktop.
CONTROL
cat > "$stage/DEBIAN/postinst" <<'POSTINST'
#!/bin/sh
set -e
if [ "$1" = configure ] && command -v update-desktop-database >/dev/null 2>&1; then
    update-desktop-database -q /usr/share/applications || true
fi
POSTINST
chmod 0755 "$stage/DEBIAN/postinst"
dpkg-deb --root-owner-group --build "$stage" "$stage.deb" >/dev/null
echo "$stage.deb"
