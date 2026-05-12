#!/usr/bin/env bash
# codex-cloud-smoke.sh — acceptance smoke test for the Codex web environment.
# Context: This script checks the minimum environment guarantees that make
# phone-first web sessions useful: Rust validation works, one experiment
# binary builds, one representative Python analysis runs on self-generated
# smoke data, and the required external tools are present.

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
if [[ -z "${CARGO_TARGET_DIR:-}" ]]; then
  cat >&2 <<'EOF'
[codex-cloud-smoke] CARGO_TARGET_DIR is not set.

Set CARGO_TARGET_DIR as a Codex web environment variable so setup, maintenance,
and later ordinary cargo commands use the same cached target directory.
EOF
  exit 101
fi
if [[ "${CARGO_TARGET_DIR}" == *'$'* || "${CARGO_TARGET_DIR}" != /* ]]; then
  cat >&2 <<EOF
[codex-cloud-smoke] CARGO_TARGET_DIR must be an absolute expanded path.

Current value: ${CARGO_TARGET_DIR}

Set the Codex web environment variable to a literal absolute path such as
/home/oai/.cache/cargo-target/msc-math. Do not use \$HOME in the UI value.
EOF
  exit 101
fi
if command -v realpath >/dev/null 2>&1; then
  CARGO_TARGET_DIR="$(realpath -m "${CARGO_TARGET_DIR}")"
  export CARGO_TARGET_DIR
fi
case "${CARGO_TARGET_DIR}" in
  "${ROOT_DIR}" | "${ROOT_DIR}"/*)
    cat >&2 <<EOF
[codex-cloud-smoke] CARGO_TARGET_DIR must not be inside the repo checkout.

Current value: ${CARGO_TARGET_DIR}

Use a cached user directory such as /home/oai/.cache/cargo-target/msc-math.
EOF
    exit 101
    ;;
esac
mkdir -p "${CARGO_TARGET_DIR}"
TMP_DIR=""

echo "[codex-cloud-smoke] repo root: ${ROOT_DIR}"
echo "[codex-cloud-smoke] Cargo target dir: ${CARGO_TARGET_DIR}"

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
  if [[ -n "${TMP_DIR}" && -d "${TMP_DIR}" ]]; then
    rm -rf "${TMP_DIR}"
  fi
}

trap cleanup EXIT

if ! command -v qconvex >/dev/null 2>&1; then
  cat >&2 <<'EOM'
[codex-cloud-smoke] missing command: qconvex

This repo's Rust validation path depends on qhull. Run
`bash .devcontainer/codex-cloud-setup.sh` first in an environment where qhull can be
installed, or use an environment that already ships qconvex.
EOM
  exit 1
fi

echo "[codex-cloud-smoke] Verifying Git LFS..."
git lfs version >/dev/null

echo "[codex-cloud-smoke] Running symplectic library tests..."
(
  cd "${ROOT_DIR}"
  cargo test -p symplectic --release --lib
)

echo "[codex-cloud-smoke] Running symplectic library clippy..."
(
  cd "${ROOT_DIR}"
  cargo clippy -p symplectic --lib -- -D warnings
)

echo "[codex-cloud-smoke] Building representative experiment binary..."
(
  cd "${ROOT_DIR}"
  cargo build -p exp-hko-local-maximum --release --bin hko-perturbation
)

echo "[codex-cloud-smoke] Running representative Python analysis in temp workspace..."
TMP_DIR="$(mktemp -d)"
TMP_EXP_DIR="${TMP_DIR}/experiments/hko-local-maximum/perturbation-neighborhood"
mkdir -p "${TMP_EXP_DIR}/data"
mkdir -p "${TMP_DIR}/experiments"
cp "${ROOT_DIR}/experiments/figure_config.py" "${TMP_DIR}/experiments/figure_config.py"
cp "${ROOT_DIR}/experiments/hko-local-maximum/perturbation-neighborhood/analyze.py" "${TMP_EXP_DIR}/analyze.py"

for eps in 0.001 0.01 0.1; do
  "${CARGO_TARGET_DIR}/release/hko-perturbation" \
    --eps "${eps}" \
    --n 20 \
    --out "${TMP_EXP_DIR}/data/smoke-eps-${eps}.jsonl"
done

(
  cd "${TMP_EXP_DIR}"
  uv run analyze.py
)

echo "[codex-cloud-smoke] Smoke test passed."
