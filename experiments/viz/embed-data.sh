#!/usr/bin/env bash
# Generate data.js with all polytope data embedded
set -euo pipefail

DIR="$(cd "$(dirname "$0")" && pwd)"
DATA_DIR="$DIR/../../docs/viz/data"

echo "// Auto-generated polytope data - DO NOT EDIT"
echo "// Generated from JSON files in data/"
echo ""
echo "window.POLYTOPE_DATA = {"

first=true
for json_file in "$DATA_DIR"/*.json; do
    name=$(basename "$json_file" .json)

    if [ "$first" = true ]; then
        first=false
    else
        echo ","
    fi

    echo -n "  \"$name\": "
    cat "$json_file"
done

echo ""
echo "};"
