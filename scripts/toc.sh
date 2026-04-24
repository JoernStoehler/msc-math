#!/bin/bash
# Extract a table-of-contents from Markdown files with line ranges.
# Usage: bash scripts/toc.sh <file> [<file> ...]
# Output: "start-end  heading" for each #/##/### section.
# Agents: use the line ranges with Read(file, offset=start, limit=end-start+1).

set -euo pipefail

if [ "$#" -eq 0 ]; then
    echo "Usage: bash scripts/toc.sh <file> [<file> ...]" >&2
    exit 2
fi

for FILE in "$@"; do
    if [ ! -f "$FILE" ]; then
        echo "Error: $FILE not found" >&2
        exit 1
    fi

    if [ "$#" -gt 1 ]; then
        echo "==> $FILE <=="
    fi

    awk '
/^#{1,3} / {
    if (NR > 1 && start > 0) {
        printf "%4d-%4d  %s\n", start, NR-1, title
    }
    start = NR
    title = $0
}
END {
    if (start > 0) {
        printf "%4d-%4d  %s\n", start, NR, title
    }
}
' "$FILE"
done
