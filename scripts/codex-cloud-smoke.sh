#!/usr/bin/env bash
# codex-cloud-smoke.sh — acceptance smoke test for the Codex cloud v1 environment.
# Context: This script checks the minimum environment guarantees that make
# phone-first cloud sessions useful: Rust validation works, one experiment
# binary builds, one representative Python analysis runs on self-generated
# smoke data, and the required external tools are present.

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

echo "[codex-cloud-smoke] repo root: ${ROOT_DIR}"

require_cmd() {
  local cmd="$1"
  if ! command -v "${cmd}" >/dev/null 2>&1; then
    echo "[codex-cloud-smoke] missing command: ${cmd}" >&2
    exit 1
  fi
}

require_cmd cargo
require_cmd uv
require_cmd git

cleanup() {
  local files=(
    "${ROOT_DIR}/crates/exp-hko-local-maximum/perturbation-neighborhood/data/smoke-eps-0.001.jsonl"
    "${ROOT_DIR}/crates/exp-hko-local-maximum/perturbation-neighborhood/data/smoke-eps-0.01.jsonl"
    "${ROOT_DIR}/crates/exp-hko-local-maximum/perturbation-neighborhood/data/smoke-eps-0.1.jsonl"
    "${ROOT_DIR}/crates/exp-hko-local-maximum/perturbation-neighborhood/pentagon_perturb_sys_hist.png"
    "${ROOT_DIR}/crates/exp-hko-local-maximum/perturbation-neighborhood/pentagon_perturb_stats.md"
    "${ROOT_DIR}/crates/exp-hko-local-maximum/perturbation-neighborhood/pentagon_perturb_stats.tex"
    "${ROOT_DIR}/crates/exp-hko-local-maximum/perturbation-neighborhood/pentagon_perturb_pca.md"
    "${ROOT_DIR}/crates/exp-hko-local-maximum/perturbation-neighborhood/pentagon_perturb_pca.tex"
  )

  git -C "${ROOT_DIR}" restore --source=HEAD --worktree -- "${files[@]}" 2>/dev/null || true
  for file in "${files[@]}"; do
    if ! git -C "${ROOT_DIR}" ls-files --error-unmatch "${file#${ROOT_DIR}/}" >/dev/null 2>&1; then
      rm -f "${file}"
    fi
  done
}

trap cleanup EXIT

if ! command -v qconvex >/dev/null 2>&1; then
  cat >&2 <<'EOF'
[codex-cloud-smoke] missing command: qconvex

This repo's Rust validation path depends on qhull. Run
`bash scripts/codex-cloud-setup.sh` first in an environment where qhull can be
installed, or use an environment that already ships qconvex.
EOF
  exit 1
fi

echo "[codex-cloud-smoke] Verifying Git LFS..."
git lfs version >/dev/null

echo "[codex-cloud-smoke] Running library tests..."
(
  cd "${ROOT_DIR}/crates/library"
  cargo test --release --lib
)

echo "[codex-cloud-smoke] Running library clippy..."
(
  cd "${ROOT_DIR}/crates/library"
  cargo clippy --lib -- -D warnings
)

echo "[codex-cloud-smoke] Building representative experiment binary..."
(
  cd "${ROOT_DIR}/crates"
  cargo build -p exp-hko-local-maximum --release --bin hko-perturbation
)

echo "[codex-cloud-smoke] Running representative Python analysis..."
(
  cd "${ROOT_DIR}/crates/exp-hko-local-maximum/perturbation-neighborhood"
  mkdir -p data
  for eps in 0.001 0.01 0.1; do
    ../../target/release/hko-perturbation \
      --eps "${eps}" \
      --n 20 \
      --out "data/smoke-eps-${eps}.jsonl"
  done
  uv run analyze.py
)

echo "[codex-cloud-smoke] Smoke test passed."
