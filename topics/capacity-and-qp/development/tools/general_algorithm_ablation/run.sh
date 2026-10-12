#!/usr/bin/env bash
set -euo pipefail

output_dir="${1:-/tmp/qp-current-route-evidence}"
mkdir -p "$output_dir"

cargo build -p exp-qp-general-algorithms --release \
  --bin qp-general-algorithm-comparison \
  --bin qp-general-selected-verification \
  --bin qp-general-selected-numerics \
  --bin qp-general-end-to-end

target/release/qp-general-selected-verification >"$output_dir/verification.txt"
target/release/qp-general-selected-numerics >"$output_dir/numerics.txt"
target/release/qp-general-algorithm-comparison >"$output_dir/algorithms.txt"
target/release/qp-general-end-to-end >"$output_dir/end-to-end.txt"

printf '%s\n' \
  "$output_dir/verification.txt" \
  "$output_dir/numerics.txt" \
  "$output_dir/algorithms.txt" \
  "$output_dir/end-to-end.txt"
