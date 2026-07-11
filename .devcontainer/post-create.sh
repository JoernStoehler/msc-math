#!/usr/bin/env bash
# Local devcontainer post-create setup (Jörn's Ubuntu desktop).

set -euo pipefail

echo "[post-create] Local devcontainer post-create..."

# Ensure user directories exist
sudo mkdir -p \
  "${HOME}/.config" \
  "${HOME}/.local" \
  "${HOME}/.cache"
sudo chown -R "${USER}:${USER}" \
  "${HOME}/.config" \
  "${HOME}/.local" \
  "${HOME}/.cache"

# Fix ownership of Docker volume mounts (created as root by default)
sudo chown "${USER}:${USER}" "${HOME}/.vscode" 2>/dev/null || true

# Refresh VS Code tunnel CLI on every container recreate. The Dockerfile also
# bakes a copy into the image, but that layer can be cached across rebuilds.
tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT
curl -fsSL "https://update.code.visualstudio.com/latest/cli-linux-x64/stable" -o "$tmpdir/vscode-cli.tar.gz"
tar -xzf "$tmpdir/vscode-cli.tar.gz" -C "$tmpdir"
sudo install -m 0755 "$tmpdir/code" /usr/local/bin/code-tunnel
rm -rf "$tmpdir"
trap - EXIT

# Configure npm paths and install global packages
if command -v npm >/dev/null 2>&1; then
  mkdir -p "${HOME}/.local/bin" "${HOME}/.cache/npm"
  npm config set prefix "${HOME}/.local"
  npm config set cache "${HOME}/.cache/npm"
  # pyright LSP for Claude Code code intelligence plugin
  npm install -g pyright
  # Codex CLI (OpenAI). Runtime state (auth.json, history, sessions, log)
  # lives in ~/.codex, which is bind-mounted from /srv/devhome/.codex so it
  # persists across rebuilds and stays outside any git tree. This project's
  # IDE-owned settings remain there; repo-local skills and optional custom
  # agents live in their tracked project directories.
  npm install -g @openai/codex
fi

# Codex: idempotently seed a trust entry for the msc-math project root in the
# machine-local ~/.codex/config.toml. Trust enables project-scoped Codex
# customization. The file
# is inside the /srv/devhome/.codex bind mount so this append survives rebuilds
# and stays out of any git tree. Append-if-not-present for idempotency.
mkdir -p /home/vscode/.codex
CODEX_USER_CONFIG=/home/vscode/.codex/config.toml
touch "$CODEX_USER_CONFIG"
if ! grep -qF 'projects."/workspaces/msc-math"' "$CODEX_USER_CONFIG"; then
  printf '\n[projects."/workspaces/msc-math"]\ntrust_level = "trusted"\n' >> "$CODEX_USER_CONFIG"
fi

# Configure git credentials via GitHub CLI
if command -v gh >/dev/null 2>&1; then
  gh auth setup-git || true
fi

# Initialize Git LFS (git-lfs is installed via Dockerfile; this sets up the
# smudge/clean filters so LFS-tracked files are handled on checkout/commit)
git lfs install

# Pre-commit hooks (check-added-large-files blocks files >10 MB)
uv tool install pre-commit
export PATH="${HOME}/.local/bin:${PATH}"
pre-commit install

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
echo "[post-create] code-tunnel: $(code-tunnel --version 2>/dev/null || echo 'not found')"
if command -v latexmk >/dev/null 2>&1; then
  echo "[post-create] latexmk: $(latexmk --version 2>/dev/null | head -1 || echo 'available')"
else
  echo "[post-create] WARNING: latexmk not found (TexLive may not be installed)" >&2
fi
if command -v sage >/dev/null 2>&1; then
  echo "[post-create] sage: $(sage --version 2>/dev/null || echo 'available')"
else
  echo "[post-create] WARNING: sage not found" >&2
fi

# Pre-warm TeX formats in user tree if missing
if [ ! -d "${HOME}/.texlive2023/texmf-var/web2c" ]; then
  echo "[post-create] Pre-warming TeX formats..."
  TEXMFVAR="${HOME}/.texlive2023/texmf-var" fmtutil-user --all >/dev/null 2>&1 || true
fi

# tmux config for Claude Code TUI compatibility
# Based on https://github.com/sethdford/tmux-claude-code
cat > ~/.tmux.conf << 'TMUXCONF'
set -g mouse on
set -g status off
set -g set-titles on
set -g set-titles-string "[#S] #{pane_title}"
set -g @scroll-down-exit-copy-mode off

# Claude Code fixes
set -g allow-passthrough on
set -sg escape-time 0
set -g extended-keys always
set -as terminal-features 'xterm*:extkeys'
set -as terminal-features 'xterm-kitty:extkeys'
set -g set-clipboard on
set -g history-limit 250000
set -g focus-events on
set -g default-terminal "tmux-256color"
set -ag terminal-overrides ",xterm-256color:RGB"

# Bell passthrough — lets CC terminal_bell reach the outer terminal
set -g bell-action any
set -g visual-bell on
set -g monitor-bell on

# Copy mode styling (readable on light background)
set -g mode-style "bg=#a8d1ff,fg=#000000"
TMUXCONF

# Safe delete wrapper — redirects rm to trash-put (use /bin/rm for real deletes)
# Requires .Trash-1000/ in .gitignore to avoid committing trashed files.
if ! grep -q 'trash-put' ~/.bashrc 2>/dev/null; then
  cat >> ~/.bashrc << 'BASHRC'

# Safe delete: redirect rm to trash-put (use /bin/rm for real deletes)
rm() { trash-put "$@"; }
export -f rm
BASHRC
fi

echo "[post-create] Local post-create complete."
