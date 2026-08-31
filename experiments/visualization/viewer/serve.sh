#!/usr/bin/env bash
# Generate visualization JSON data and start HTTP server.
# Usage: ./serve.sh [port]
set -euo pipefail

PORT="${1:-8080}"
DIR="$(cd "$(dirname "$0")" && pwd)"
MANIFEST="$DIR/../Cargo.toml"

# Do not disturb a process that this script does not own.
if lsof -nP -tiTCP:"$PORT" -sTCP:LISTEN >/dev/null 2>&1; then
    echo "Error: port $PORT is already in use; refusing to stop the existing process." >&2
    echo "Choose another port: ./serve.sh <port>" >&2
    exit 1
fi

echo "Building visualization binary..."
cargo build --release --manifest-path "$MANIFEST" --bin visualization -q

echo "Generating polytope data..."
VIZ_DATA="$DIR/data"
mkdir -p "$VIZ_DATA"
for name in simplex hypercube crosspolytope hko_pentagon \
            lagrangian_triangle_product symplectic_triangle_product \
            lagrangian_tri_sq symplectic_tri_sq; do
    cargo run --release --quiet --manifest-path "$MANIFEST" --bin visualization -- \
        "$name" "$VIZ_DATA/$name.json"
done

echo "Embedding data into viewer..."
bash "$DIR/embed-data.sh" > "$DIR/data.js"

echo ""
echo "Serving at http://localhost:$PORT"
echo "  (Ctrl-C to stop)"
cd "$DIR"
exec python3 -m http.server "$PORT"
