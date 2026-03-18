# Devcontainer Setup

Local devcontainer on Jörn's Ubuntu desktop. Provides OS-level isolation for
Claude Code sessions (`--dangerously-skip-permissions` is safe because Docker
is the security boundary, not CC's permission rules).

## Architecture

```
Remote devices                        Host (Ubuntu desktop)              Container (Docker)
─────────────────                     ──────────────────────             ──────────────────

Browser                               Tailscale (100.70.188.20)
  └─ vscode.dev ──── tunnel ─────────── code-tunnel server ─────────────→ VS Code tunnel
                                                                           (IDE view)

Ghostty / Crostini /                  SSH server (:22, socket-activated)
PowerShell / Termux                     │
  └─ ssh joern@host ────────────────────┘
       └─ dc  ─── devcontainer exec ──────────────────────────────────→ bash
                                                                         └─ tmux new-session -A -s cc
                                                                              └─ claude --dangerously-skip-permissions
                                                                                   └─ Bash(), Edit(), ...
```

### Layers

**Host** runs:
- Tailscale for mesh networking across devices
- SSH server (socket-activated — starts on first connection)
- `devcontainer` CLI to build/start/exec into the container
- Utility scripts in `.devcontainer/` to manage the container

**Container** runs:
- VS Code tunnel server (for browser IDE access via vscode.dev)
- bash sessions (entered via `devcontainer exec` from host)
- tmux (session persistence across SSH disconnects; `set -g mouse on` for scroll)
- Claude Code CLI processes
- Everything CC spawns: Bash() commands, cargo, python, latexmk, etc.

**Remote devices** connect via:
- Browser → vscode.dev tunnel → IDE view (file browsing, editing)
- Terminal → SSH → host → `dc` function → container bash → tmux → claude

### Why this stack

| Component | Why |
|-----------|-----|
| Docker container | OS-level isolation — CC can't touch host filesystem, SSH keys, etc. |
| `--dangerously-skip-permissions` | Safe because Docker provides isolation. Eliminates all permission prompts. |
| tmux | Session persistence across disconnects. `set -g mouse on` in `~/.tmux.conf` fixes scroll in CC's TUI. |
| SSH + Tailscale | Access from any device. Works on Android (Termux), ChromeOS (Crostini), Windows (PowerShell), Linux. |
| VS Code tunnel | Browser IDE fallback for GUI file browsing. Not the primary workflow. |
| trash-cli + rm wrapper | `rm` → `trash-put` inside container. Use `/bin/rm` for real deletes. |

### Why not dtach

dtach doesn't preserve terminal buffer state. After detach/reattach, CC's TUI
renders incorrectly and cannot be recovered without restarting. tmux maintains
screen state across detach/reattach. dtach is still installed if needed for
other (non-TUI) processes.

## Access from host

The `dc` shell function (add to host `~/.bashrc`) gives a bash shell in any
project's devcontainer:

```bash
dc() {
  local root
  root="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
  devcontainer up --workspace-folder "$root" >/dev/null
  devcontainer exec --workspace-folder "$root" -- bash -l
}
```

From inside the container, start a persistent CC session:

```bash
tmux new-session -A -s cc 'claude --dangerously-skip-permissions'
# Ctrl+b d to detach
# Reattach later:
tmux attach -t cc
```

Or without persistence:

```bash
claude --dangerously-skip-permissions
```

## Access from remote devices

```bash
# From any device with SSH:
ssh joern@<tailscale-ip>
cd ~/workspaces/msc-math && dc

# Then inside the container:
tmux new-session -A -s cc 'claude --dangerously-skip-permissions'
# Ctrl+b d to detach; reattach from any device with:
# dc → tmux attach -t cc
```

Tested access methods:
- Chrome on Ubuntu desktop (trivial — just run `dc` in a terminal)
- PowerShell on Windows university PC (`ssh joern@100.70.188.20`)
- Crostini on ChromeOS (`ssh joern@100.70.188.20`)
- Termux on Android (`ssh joern@100.70.188.20`; optionally mosh for flaky connections)

## Host scripts

| Script | Purpose |
|--------|---------|
| `host-devcontainer-rebuild.sh` | Rebuild image + recreate container |
| `host-vscode-tunnel.sh` | Start VS Code tunnel into container |
| `warmup-cache.sh` | Background cache warmer (Rust + Python deps) |

## Bind mounts

Persistent state survives container rebuilds via bind mounts from `/srv/devhome/` on the host:

| Host path | Container path | Purpose |
|-----------|---------------|---------|
| `/srv/devhome/.claude` | `~/.claude` | CC config, sessions, credentials |
| `/srv/devhome/.config/gh` | `~/.config/gh` | GitHub CLI auth |
| `/srv/devhome/.cache/uv` | `~/.cache/uv` | Python dependency cache |
| `/srv/devhome/.texlive2023` | `~/.texlive2023` | TeX cache |
| `/srv/devhome/.bash_history_dir` | `~/.bash_history_dir` | Shell history |
| `/srv/devhome/.vscode-cli` | `~/.vscode-cli` | VS Code tunnel auth |

## Resource limits

- 10 GB RAM + 2 GB swap (protects host; OOM kills the process, not the container)
- `CARGO_BUILD_JOBS=4`
- SYS_ADMIN capability (for perf, strace)

## Rebuilding

```bash
# From host, in the repo directory:
bash .devcontainer/host-devcontainer-rebuild.sh
```

This rebuilds the image and recreates the container. Bind-mounted state
(`/srv/devhome/`) persists. Container-local state (installed packages not in
the Dockerfile) is lost.
