#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"

for stem in nested baseline repaired; do
  pandoc "${stem}-passage.md" \
    --from=markdown+tex_math_dollars+tex_math_single_backslash \
    --standalone --mathml --embed-resources --css passage.css \
    --metadata pagetitle='Ridge-area writing pilot' \
    -o "${stem}-passage.html"
done
