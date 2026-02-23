#!/usr/bin/env bash
# Local devcontainer post-create setup (Jörn's Ubuntu desktop).

set -euo pipefail

echo "Local devcontainer post-create..."

# Ensure user directories exist
sudo mkdir -p \
  "${HOME}/.config" \
  "${HOME}/.local" \
  "${HOME}/.cache"
sudo chown -R "${USER}:${USER}" \
  "${HOME}/.config" \
  "${HOME}/.local" \
  "${HOME}/.cache"

# Configure npm paths and install global packages
if command -v npm >/dev/null 2>&1; then
  mkdir -p "${HOME}/.local/bin" "${HOME}/.cache/npm"
  npm config set prefix "${HOME}/.local"
  npm config set cache "${HOME}/.cache/npm"
  # pyright LSP for Claude Code code intelligence plugin
  npm install -g pyright
fi

# Configure git credentials via GitHub CLI
if command -v gh >/dev/null 2>&1; then
  gh auth setup-git || true
fi

# Install Claude Code CLI
curl -fsSL https://claude.ai/install.sh | bash

# Ensure TeX directories exist
sudo mkdir -p \
  "${HOME}/.texlive2023" \
  "${HOME}/.texmf-var" \
  "${HOME}/.texmf-config"
sudo chown -R "${USER}:${USER}" \
  "${HOME}/.texlive2023" \
  "${HOME}/.texmf-var" \
  "${HOME}/.texmf-config"
mkdir -p "${HOME}/.cache/LaTeXML"

# Verify tools
echo "code-tunnel: $(code-tunnel --version 2>/dev/null || echo 'not found')"
if command -v latexmk >/dev/null 2>&1; then
  echo "latexmk: $(latexmk --version 2>/dev/null | head -1 || echo 'available')"
else
  echo "WARNING: latexmk not found (TexLive may not be installed)" >&2
fi

# Pre-warm TeX formats in user tree if missing
if [ ! -d "${HOME}/.texlive2023/texmf-var/web2c" ]; then
  echo "Pre-warming TeX formats..."
  TEXMFVAR="${HOME}/.texlive2023/texmf-var" fmtutil-user --all >/dev/null 2>&1 || true
fi

echo "Local post-create complete."
