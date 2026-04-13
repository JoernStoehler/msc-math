#!/usr/bin/env bash
# codex-cloud-maintenance.sh — refresh cached Codex cloud containers for msc-math.
# Context: Codex runs this script when resuming a cached cloud container. It
# re-warms the Rust validation path against the task's checked-out branch so
# the next interactive step does not absorb the cold compile cost again.

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

echo "[codex-cloud-maintenance] repo root: ${ROOT_DIR}"
echo "[codex-cloud-maintenance] Refreshing Rust validation cache..."
bash "${ROOT_DIR}/scripts/codex-cloud-rust-warmup.sh"
echo "[codex-cloud-maintenance] Done."
