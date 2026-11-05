#!/usr/bin/env bash
set -euo pipefail
if [[ $# != 1 ]]; then echo "Usage: SAGE_PYTHON=/path/to/sage/python $0 NEW_REPLAY_DIRECTORY" >&2; exit 2; fi
: "${SAGE_PYTHON:?Set SAGE_PYTHON to the Sage Python interpreter}"
packet=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
mkdir -- "$1"
run=$(cd -- "$1" && pwd)
for f in common.py exact_attainer.py exact_geometry.py exact_curvature.py exact_extend_curvature.py exact_leaves.py verify_result.py coverage_replay.cpp proposed-exclusions.txt; do
 cp -- "$packet/$f" "$run/$f"
done
cd -- "$run"
export PYTHONDONTWRITEBYTECODE=1
"$SAGE_PYTHON" exact_attainer.py > attainer.log
"$SAGE_PYTHON" exact_geometry.py > geometry.log
"$SAGE_PYTHON" exact_curvature.py > curvature.log
"$SAGE_PYTHON" exact_extend_curvature.py > curvature-extension.log
g++ -O2 -std=c++17 coverage_replay.cpp -o coverage_replay
./coverage_replay --generate > generation-summary.json
"$SAGE_PYTHON" exact_leaves.py > leaves.log
./coverage_replay > coverage-summary.json
"$SAGE_PYTHON" verify_result.py | tee verification.log
