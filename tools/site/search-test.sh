#!/bin/sh
# Runs the built search page in a headless browser for a few questions and
# prints how many results each gives and the first pages it points to.
#   sh tools/site/search-test.sh
set -eu
here=$(cd "$(dirname "$0")/../.." && pwd)
CHROME=${CHROME:-$(ls "$HOME"/.cache/ms-playwright/chromium_headless_shell-*/chrome-headless-shell-linux64/chrome-headless-shell | head -1)}
for q in wifi wi-fi printer lock "install app" xyzzy; do
  "$CHROME" --no-sandbox --disable-gpu --virtual-time-budget=3000 --dump-dom \
    "file://$here/site/dist/search.html?q=$(printf '%s' "$q" | sed 's/ /%20/g')" 2>/dev/null |
  python3 -I -c "
import sys, re
d = sys.stdin.read()
st = re.findall(r'id=\"status\"[^>]*>([^<]*)', d)
hits = re.findall(r'class=\"title\" href=\"([^\"]+)\"', d)[:4]
print('$q', '|', st, hits)"
done
