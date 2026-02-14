#!/usr/bin/env bash
# Generate visualization JSON data and start HTTP server.
# Usage: ./serve.sh [port]
set -euo pipefail

PORT="${1:-8080}"
DIR="$(cd "$(dirname "$0")" && pwd)"
DATA="$DIR/../../docs/viz/data"
CRATES="$DIR/../../crates"

mkdir -p "$DATA"

echo "Building datasets binary..."
cargo build --release --manifest-path "$CRATES/Cargo.toml" -p datasets --bin datasets -q

BIN="$CRATES/target/release/datasets"

POLYTOPES=(simplex hypercube crosspolytope hko_pentagon
           lagrangian_triangle_product symplectic_triangle_product
           lagrangian_tri_sq symplectic_tri_sq)

for name in "${POLYTOPES[@]}"; do
    out="$DATA/${name}.json"
    if [ -f "$out" ]; then
        echo "  $name — cached"
    else
        "$BIN" export-viz "$out" "$name" 2>&1
    fi
done

# Kill any stale server on this port
if lsof -ti:"$PORT" >/dev/null 2>&1; then
    echo "Killing stale process on port $PORT..."
    lsof -ti:"$PORT" | xargs kill 2>/dev/null
    sleep 0.5
fi

echo ""
echo "Serving at http://localhost:$PORT"
echo "  (Ctrl-C to stop)"
cd "$DIR"
exec python3 -m http.server "$PORT"
