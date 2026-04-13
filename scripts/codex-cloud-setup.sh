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

if command -v qconvex >/dev/null 2>&1; then
  echo "[codex-cloud-setup] qconvex already present; skipping qhull install."
else
  echo "[codex-cloud-setup] qconvex missing; attempting to install qhull..."
  if ! ${SUDO} apt-get update; then
    cat >&2 <<'EOF'
[codex-cloud-setup] apt-get update failed before qhull could be installed.

This Codex cloud environment does not currently permit the repo-specific qhull
install path used by this setup script. Without qconvex, `cargo test --release
--lib` will not satisfy the cloud smoke contract for this repo.

Next step:
- use a Codex cloud environment mode that already includes qconvex, or
- use an environment where apt package installs succeed, or
- accept that this environment cannot run the Rust validation smoke suite.
EOF
    exit 100
  fi

  if ! ${SUDO} apt-get install -y --no-install-recommends qhull-bin; then
    cat >&2 <<'EOF'
[codex-cloud-setup] qhull-bin install failed.

The repo's Rust validation path depends on `qconvex`
(`crates/library/src/geom/qhull.rs`). This environment is therefore not ready
for the intended cloud smoke workflow.
EOF
    exit 100
  fi

  ${SUDO} rm -rf /var/lib/apt/lists/*
fi

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
qconvex - >/dev/null 2>&1 <<'EOF'
EOF

echo "[codex-cloud-setup] Done."
