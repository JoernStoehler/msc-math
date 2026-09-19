#!/usr/bin/env bash
# Supported-v2 commands. Existing frozen/run outputs are immutable.
set -euo pipefail
cd /workspaces/msc-math
export CARGO_TARGET_DIR="$PWD/.git/codex/ds-first-wave/readiness/target" CARGO_BUILD_JOBS=2
export RAYON_NUM_THREADS=2 OMP_NUM_THREADS=2 OPENBLAS_NUM_THREADS=2
PACKET=experiments/sys-datascience/methods/diagonal-cem-pilot
cargo build --offline --locked --release --manifest-path "$PACKET/Cargo.toml"
flock /workspaces/msc-math/.git/codex/ds-evaluator.lock /usr/bin/time -v -o "$PACKET/artifacts-supported-v2/run.time" timeout 240 python3 "$PACKET/run.py"
python3 "$PACKET/analyze.py"

