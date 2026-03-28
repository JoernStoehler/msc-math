# Devcontainer Setup

Local devcontainer on Jörn's Ubuntu desktop. Provides OS-level isolation for
Claude Code sessions (`--dangerously-skip-permissions` is safe because Docker
is the security boundary, not CC's permission rules).

## Architecture

### Layers

**Host** (Ubuntu desktop) runs:
- Tailscale for mesh networking across devices
- SSH server (socket-activated — starts on first connection)
- `devcontainer` CLI to build/start/exec into the container
- gnome-terminal for host-direct CC sessions and tunnel management

**Container** (Docker) runs:
- VS Code tunnel server (for browser IDE access via vscode.dev)
- bash sessions (entered via `devcontainer exec` from host, or VSCode terminal)
- tmux (session persistence across disconnects; `set -g mouse on` for scroll)
- Claude Code CLI processes
- Everything CC spawns: Bash() commands, cargo, python, latexmk, etc.

### Access paths

1. **VS Code tunnel (desktop):** Chrome → vscode.dev → tunnel → container → bash → tmux → claude
2. **SSH (mobile):** Android → Termux → SSH → Tailscale → host → devcontainer exec → container → bash → tmux → claude

### Why this stack

| Component | Why |
|-----------|-----|
| Docker container | OS-level isolation — CC can't touch host filesystem, SSH keys, etc. |
| `--dangerously-skip-permissions` | Safe because Docker provides isolation. Eliminates all permission prompts. |
| tmux | Session persistence across disconnects. Bell passthrough configured for notifications. |
| SSH + Tailscale | Access from any device (currently: Termux on Android). |
| VS Code tunnel | Browser IDE for file browsing + terminal. Primary access path from desktop. |
| trash-cli + rm wrapper | `rm` → `trash-put` inside container. Use `/bin/rm` for real deletes. |

## Access via VSCode tunnel (primary)

From host, start the tunnel in a gnome-terminal tab:

```bash
cd ~/workspaces/msc-math
.devcontainer/host-vscode-tunnel.sh
```

Then open Chrome → vscode.dev/tunnel/msc-math → open terminal:

```bash
tmux new -s cc        # create a tmux session
claude                # start CC
# /resume, /add-dir, etc.
```

Reconnect after disconnect:

```bash
tmux attach -t cc     # reattach to existing session
```

## Access via SSH

The `dc` shell function (in host `~/.bashrc`) gives a bash shell in any
project's devcontainer:

```bash
dc() {
  local root
  root="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
  devcontainer up --workspace-folder "$root" >/dev/null
  devcontainer exec --workspace-folder "$root" -- bash -l
}
```

From any device with SSH:

```bash
ssh joern@<tailscale-ip>
cd ~/workspaces/msc-math && dc

# Inside the container:
tmux new -s cc
claude
```

Access methods (see Architecture section above):
- Chrome on Ubuntu desktop (vscode.dev tunnel)
- Termux on Android (`ssh joern@<tailscale-ip>`)

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
