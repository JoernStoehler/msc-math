#!/usr/bin/env bash
# Generate visualization JSON data and start HTTP server.
# Usage: ./serve.sh [port]
set -euo pipefail

PORT="${1:-8080}"
DIR="$(cd "$(dirname "$0")" && pwd)"
EXPERIMENTS="$DIR/../.."

echo "Building viz_export binary..."
cargo build --release --manifest-path "$EXPERIMENTS/Cargo.toml" --bin viz_export -q

echo "Generating polytope data..."
VIZ_DATA="$EXPERIMENTS/../docs/viz/data"
mkdir -p "$VIZ_DATA"
for name in simplex hypercube crosspolytope hko_pentagon \
            lagrangian_triangle_product symplectic_triangle_product \
            lagrangian_tri_sq symplectic_tri_sq; do
    "$EXPERIMENTS/target/release/viz_export" "$name" "$VIZ_DATA/$name.json"
done

echo "Embedding data into viewer..."
bash "$DIR/embed-data.sh" > "$DIR/data.js"

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
