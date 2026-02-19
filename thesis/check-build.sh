#!/usr/bin/env bash
# check-build.sh — post-compilation check for layout and reference warnings
#
# Usage: cd thesis/ && latexmk && ./check-build.sh
# Exit code: 0 if clean, 1 if significant warnings found
#
# Checks:
#   - Overfull hboxes > THRESHOLD pt (default 1pt, matching \hfuzz in main.tex)
#   - Undefined references

set -uo pipefail

LOG="build/main.log"
THRESHOLD="${OVERFULL_THRESHOLD:-1}"

if [[ ! -f "$LOG" ]]; then
    echo "ERROR: $LOG not found. Run latexmk first."
    exit 1
fi

TMPFILE=$(mktemp)
trap 'rm -f "$TMPFILE"' EXIT

# Collect overfull hboxes exceeding threshold
grep "Overfull.*hbox.*(.*pt too wide)" "$LOG" | sort -u | while IFS= read -r line; do
    pt=$(echo "$line" | grep -oP '\(\K[\d.]+(?=pt)') || continue
    if awk "BEGIN {exit !($pt > $THRESHOLD)}" 2>/dev/null; then
        echo "$line"
    fi
done > "$TMPFILE"

EXIT=0

overfull_count=$(wc -l < "$TMPFILE")
if [[ "$overfull_count" -gt 0 ]]; then
    echo "OVERFULL hboxes exceeding ${THRESHOLD}pt ($overfull_count total):"
    sed 's/^/  /' "$TMPFILE"
    EXIT=1
fi

# Check undefined references
undef_count=$(grep -c "undefined on input line" "$LOG" || true)
if [[ "$undef_count" -gt 0 ]]; then
    echo "UNDEFINED references ($undef_count):"
    grep "undefined on input line" "$LOG" | sort -u | sed 's/^/  /'
    EXIT=1
fi

if [[ "$EXIT" -eq 0 ]]; then
    echo "Build clean."
fi

exit $EXIT
