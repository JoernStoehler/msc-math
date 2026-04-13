#!/usr/bin/env bash
# codex-cloud-setup.sh — configure the Codex cloud v1 environment for msc-math.
# Context: The default Codex cloud image already provides Rust, uv, git-lfs,
# and common build tooling. This script adds the repo-specific missing pieces
# and pre-caches the common Python analysis stack so normal phone-driven tasks
# do not waste time on repeated downloads.

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

echo "[codex-cloud-setup] repo root: ${ROOT_DIR}"

if [[ "$(id -u)" -ne 0 ]]; then
  SUDO="sudo"
else
  SUDO=""
fi

echo "[codex-cloud-setup] Installing qhull..."
${SUDO} apt-get update
${SUDO} apt-get install -y --no-install-recommends qhull-bin
${SUDO} rm -rf /var/lib/apt/lists/*

echo "[codex-cloud-setup] Initializing Git LFS..."
git lfs install --skip-repo

echo "[codex-cloud-setup] Pre-caching Python analysis packages..."
uv --quiet run --with numpy --with matplotlib --with scipy python - <<'PY'
import matplotlib
import numpy
import scipy

print("Python analysis imports warmed:", numpy.__version__, matplotlib.__version__, scipy.__version__)
PY

echo "[codex-cloud-setup] Verifying toolchain..."
python3 --version
cargo --version
rustc --version
uv --version
git lfs version
qconvex - >/dev/null 2>&1 <<'EOF' || true
EOF

echo "[codex-cloud-setup] Done."
