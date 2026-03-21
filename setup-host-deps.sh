#!/usr/bin/env bash
# Throwaway script: install missing dependencies on host.
# Run with: sudo bash setup-host-deps.sh
# Safe to re-run (apt and rustup are idempotent).
set -euo pipefail

echo "=== APT packages ==="
apt-get update
apt-get install -y \
  biber chktex lacheck latexml \
  texlive-bibtex-extra texlive-fonts-extra texlive-science texlive-xetex \
  universal-ctags inotify-tools entr qhull-bin \
  libssl-dev libclang-dev

echo ""
echo "=== Rust toolchain (as user joern) ==="
# rustup must run as joern, not root
su - joern -c '
  set -euo pipefail
  if command -v rustup &>/dev/null; then
    echo "rustup already installed, updating..."
    rustup update stable
  else
    curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain stable
  fi
  export PATH="$HOME/.cargo/bin:$PATH"
  rustup component add rustfmt clippy rust-analyzer

  echo "Installing cargo-nextest..."
  curl -LsSf https://get.nexte.st/latest/linux | tar zxf - -C "$HOME/.cargo/bin"

  echo "Installing cargo-watch..."
  cargo install cargo-watch
'

echo ""
echo "=== Done. Verify with: rustc --version && biber --version ==="
