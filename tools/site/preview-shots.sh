#!/bin/sh
# Pictures of the built site for approving the look, taken with a headless
# browser (nothing opens on any screen).
#   sh tools/site/402-shoot.sh OUTDIR
# Needs a headless Chromium; set CHROME to its path if it is not the one in
# the Playwright cache.
set -eu
here=$(cd "$(dirname "$0")/../.." && pwd)
out=${1:?output folder}
CHROME=${CHROME:-$(ls "$HOME"/.cache/ms-playwright/chromium_headless_shell-*/chrome-headless-shell-linux64/chrome-headless-shell | head -1)}
mkdir -p "$out"
dist=$here/site/dist
shot() { # name url width height
  "$CHROME" --no-sandbox --disable-gpu --hide-scrollbars --virtual-time-budget=3000 \
    --window-size="$3,$4" --screenshot="$out/$1.png" "$2" > /dev/null 2>&1
}
shot 1-home "file://$dist/index.html" 1280 900
shot 2-docs-page "file://$dist/ring.html" 1280 900
shot 3-search "file://$dist/search.html?q=lock" 1280 900
shot 4-docs-narrow "file://$dist/apps.html" 420 900
ls "$out"
