#!/usr/bin/env bash
set -euo pipefail

output_dir="${1:-/tmp/qp-current-route-evidence}"
mkdir -p "$output_dir"

cargo build -p exp-dev-quadratic-program --release \
  --bin qp-general-algorithm-ablation

binary=target/release/qp-general-algorithm-ablation
"$binary" --verification-packet >"$output_dir/verification.txt"
"$binary" --numerics-packet >"$output_dir/numerics.txt"
"$binary" --profile-packet >"$output_dir/profile.txt"
"$binary" --end-to-end-profile >"$output_dir/end-to-end.txt"

printf '%s\n' \
  "$output_dir/verification.txt" \
  "$output_dir/numerics.txt" \
  "$output_dir/profile.txt" \
  "$output_dir/end-to-end.txt"
