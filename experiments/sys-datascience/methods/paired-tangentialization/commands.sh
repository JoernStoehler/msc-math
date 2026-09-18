#!/usr/bin/env bash
# Historical run commands; retained immutable outputs must not be overwritten.
set -euo pipefail
cd /workspaces/msc-math
export CARGO_TARGET_DIR="$PWD/.git/codex/ds-first-wave/readiness/target" CARGO_BUILD_JOBS=2
export RAYON_NUM_THREADS=2 OMP_NUM_THREADS=2 OPENBLAS_NUM_THREADS=2
PACKET=experiments/sys-datascience/methods/paired-tangentialization
cargo build --offline --locked --release --manifest-path "$PACKET/Cargo.toml"
/usr/bin/time -v -o "$PACKET/artifacts/generate.time" timeout 120 "$CARGO_TARGET_DIR/release/paired-tangentialization" "$PACKET/artifacts"
# freeze.json was written and checked after generation, before target execution.
flock /workspaces/msc-math/.git/codex/ds-evaluator.lock /usr/bin/time -v -o "$PACKET/artifacts/evaluate.time" timeout 120 python3 experiments/sys-datascience/methods/current-body-evaluator/evaluate.py --input "$PACKET/artifacts/inputs.jsonl" --output "$PACKET/artifacts/results.jsonl" --binary "$CARGO_TARGET_DIR/release/current-body-evaluator" --build-receipt experiments/sys-datascience/methods/current-body-evaluator/controls/build-receipt.json --timeout-seconds 5
python3 "$PACKET/analyze.py"
