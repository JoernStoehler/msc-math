#!/usr/bin/env bash
# codex-cloud-rust-warmup.sh — precompile the Rust paths used by the Codex web environment.
# Context: setup and maintenance both call this script so fresh and resumed
# web containers reuse the same Rust warm-up logic.

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
if [[ -z "${CARGO_TARGET_DIR:-}" ]]; then
  cat >&2 <<'EOF'
[codex-cloud-rust-warmup] CARGO_TARGET_DIR is not set.

Set CARGO_TARGET_DIR as a Codex web environment variable so setup, maintenance,
and later ordinary cargo commands use the same cached target directory.
EOF
  exit 101
fi
mkdir -p "${CARGO_TARGET_DIR}"
if command -v realpath >/dev/null 2>&1; then
  CARGO_TARGET_DIR="$(realpath -m "${CARGO_TARGET_DIR}")"
  export CARGO_TARGET_DIR
fi

echo "[codex-cloud-rust-warmup] Cargo target dir: ${CARGO_TARGET_DIR}"

echo "[codex-cloud-rust-warmup] Warming Rust release test artifacts..."
(
  cd "${ROOT_DIR}/library"
  cargo test --release --lib --no-run
)

echo "[codex-cloud-rust-warmup] Warming Rust debug test artifacts..."
(
  cd "${ROOT_DIR}/library"
  cargo test --lib --no-run
)

echo "[codex-cloud-rust-warmup] Warming Rust clippy artifacts..."
(
  cd "${ROOT_DIR}/library"
  cargo clippy --lib --no-deps -- -D warnings
)

echo "[codex-cloud-rust-warmup] Warming workspace release binaries..."
(
  cd "${ROOT_DIR}"
  cargo build --workspace --release --bins
)

echo "[codex-cloud-rust-warmup] Warming workspace debug binaries..."
(
  cd "${ROOT_DIR}"
  cargo build --workspace --bins
)
