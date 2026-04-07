#!/bin/bash
# Extract a table-of-contents from TASKS.md with line ranges.
# Usage: bash scripts/tasks-toc.sh
# Output: "start-end  heading" for each #/##/### section.
# Agents: use the line ranges with Read(file, offset=start, limit=end-start+1).

set -euo pipefail

FILE="${1:-TASKS.md}"

if [ ! -f "$FILE" ]; then
    echo "Error: $FILE not found" >&2
    exit 1
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
