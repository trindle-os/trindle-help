#!/bin/sh
# Puts the git sources of the toolkit back into Cargo.lock. A checkout that patches
# it to a folder beside it (an untracked .cargo/config.toml) has cargo write those
# packages without a source, which a clean clone's `cargo build --locked` then
# refuses. Run before committing Cargo.lock.
set -eu

here=$(cd "$(dirname "$0")/.." && pwd)
rev=$(sed -n 's/^trindle-toolkit = .*rev = "\([0-9a-f]\{40\}\)".*/\1/p' "$here/Cargo.toml" | head -1)
[ -n "$rev" ] || { echo "Cargo.toml names no full commit for trindle-toolkit" >&2; exit 1; }
awk -v tk="git+https://github.com/trindle-os/trindle-toolkit?rev=$rev#$rev" '
    { line[NR] = $0 }
    END {
        for (i = 1; i <= NR; i++) {
            print line[i]
            if (line[i] ~ /^name = "trindle-(toolkit|common)"$/) {
                print line[++i]
                if (line[i + 1] !~ /^source = /) print "source = \"" tk "\""
            }
        }
    }' "$here/Cargo.lock" > "$here/Cargo.lock.new"
mv "$here/Cargo.lock.new" "$here/Cargo.lock"
