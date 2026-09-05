#!/usr/bin/env bash
# check-build.sh — force a fresh thesis build, then check its PDF and log
#
# Usage: cd thesis/ && ./check-build.sh
# Exit code: 0 if compilation succeeds and the selected checks are clean
#
# Checks:
#   - Overfull hboxes > THRESHOLD pt (default 1pt, matching \hfuzz in main.tex)
#   - Undefined references

set -euo pipefail

cd "$(dirname "$0")"
latexmk -g

LOG="build/main.log"
PDF="build/main.pdf"
THRESHOLD="${OVERFULL_THRESHOLD:-1}"

if [[ ! -s "$LOG" ]]; then
    echo "ERROR: fresh build did not produce a nonempty $LOG" >&2
    exit 1
fi

if [[ ! -s "$PDF" ]] || [[ "$(head -c 5 "$PDF")" != "%PDF-" ]]; then
    echo "ERROR: fresh build did not produce a valid-looking $PDF" >&2
    exit 1
fi

TMPFILE=$(mktemp)
trap 'rm -f "$TMPFILE"' EXIT

# Collect overfull hboxes exceeding threshold. Starting with awk keeps a clean
# log (no matching lines) from tripping `set -e`.
awk -v threshold="$THRESHOLD" \
    '/Overfull.*hbox.*\(.*pt too wide\)/ && match($0, /\([0-9]+\.[0-9]+pt/) {
        pt = substr($0, RSTART+1, RLENGTH-3)+0
        if (pt > threshold+0) print
    }' "$LOG" | sort -u > "$TMPFILE"

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
    echo "Fresh build passed the selected PDF, overfull-box, and reference checks."
fi

exit $EXIT
