#!/usr/bin/env bash
# Return a label's rendered number from the existing compiled thesis.
set -euo pipefail

if [[ $# -ne 1 || -z "$1" ]]; then
    echo "Usage: $0 LABEL" >&2
    exit 2
fi

cd "$(dirname "$0")"
if [[ ! -r build/main.aux ]]; then
    echo "Missing build/main.aux; build the thesis with latexmk first." >&2
    exit 1
fi

# Compare a literal prefix: labels can contain regex metacharacters. Pass the
# label through the environment so awk does not interpret backslash escapes.
if ! LOOKUP_LABEL="$1" awk '
    BEGIN { prefix = "\\newlabel{" ENVIRON["LOOKUP_LABEL"] "}{{" }
    index($0, prefix) == 1 {
        rest = substr($0, length(prefix) + 1)
        end = index(rest, "}")
        if (end) { print substr(rest, 1, end - 1); found = 1; exit }
    }
    END { if (!found) exit 1 }
' build/main.aux; then
    echo "Label not found in build/main.aux: $1" >&2
    exit 1
fi
