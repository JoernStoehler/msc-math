#!/usr/bin/env bash
# codex-cloud-rust-warmup.sh — precompile the Rust paths used by the Codex web environment.
# Context: setup and maintenance both call this script so fresh and resumed
# web containers reuse the same Rust warm-up logic.

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

echo "[codex-cloud-rust-warmup] Warming Rust release test artifacts..."
(
  cd "${ROOT_DIR}/library"
  cargo test --release --lib --no-run
)

echo "[codex-cloud-rust-warmup] Warming Rust clippy artifacts..."
(
  cd "${ROOT_DIR}/library"
  cargo clippy --lib --no-deps -- -D warnings
)

echo "[codex-cloud-rust-warmup] Warming representative experiment binary..."
(
  cd "${ROOT_DIR}"
  cargo build -p exp-sys-landscape --release --bin sys-random-sample
)
