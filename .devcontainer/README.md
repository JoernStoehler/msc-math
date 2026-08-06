# Local Devcontainer

Local devcontainer on Jörn's Ubuntu desktop. Provides OS-level isolation for
Codex CLI sessions (`danger-full-access` is safe because Docker
is the security boundary, not Codex's in-tool sandbox rules).

## Architecture

### Layers

**Host** (Ubuntu desktop) runs:
- Tailscale for mesh networking across devices
- SSH server (socket-activated — starts on first connection)
- `devcontainer` CLI to build/start/exec into the container
- gnome-terminal for host-direct Codex sessions and tunnel management

**Container** (Docker) runs:
- VS Code tunnel server (for browser IDE access via vscode.dev)
- bash sessions (entered via `devcontainer exec` from host, or VSCode terminal)
- tmux (session persistence across disconnects; `set -g mouse on` for scroll)
- Codex CLI processes
- the Codex mobile companion and its loopback app-server, when the sibling
  `/workspaces/codex-gui` checkout and its dependencies are present
- Everything Codex spawns: shell commands, cargo, python, latexmk, etc.
- SageMath via a baked Miniforge/conda-forge install, exposed as `sage`

Version policy in this container:
- pinned for reproducibility: base image, Rust toolchain, Node.js package version, `uv`, Miniforge, SageMath
- intentionally latest on recreate: `code-tunnel`, Codex CLI, Claude Code

### Access paths

1. **VS Code tunnel (desktop):** Chrome → vscode.dev → tunnel → container → bash → tmux → codex
2. **SSH (mobile):** Android → Termux → SSH → Tailscale → host → devcontainer exec → container → bash → tmux → codex

### Why this stack

| Component | Why |
|-----------|-----|
| Docker container | OS-level isolation — Codex can't touch host filesystem, SSH keys, etc. |
| `danger-full-access` | Safe because Docker provides isolation. Eliminates in-tool permission prompts. |
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
codex                 # start Codex
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
codex
```

Access methods (see Architecture section above):
- Chrome on Ubuntu desktop (vscode.dev tunnel)
- Termux on Android (`ssh joern@<tailscale-ip>`)

## Codex mobile companion

On each container start, `postStartCommand` runs
`.devcontainer/start-codex-gui.sh`. When `/workspaces/codex-gui` and its
`node_modules` are present, the script idempotently starts both the HMR server
and the loopback Codex app-server in their detached tmux sessions.

Run the same startup explicitly from the `msc-math` entry point with:

```bash
bash .devcontainer/start-codex-gui.sh
```

The hook prints a warning and leaves the container usable when the companion
checkout or dependencies are absent. After a devcontainer recreation, clone
the private `codex-gui` repository into `/workspaces/codex-gui`, run `npm ci`
there, and rerun the startup script. Inspect or stop the services with the
`npm run dev:*` and `npm run app-server:*` commands in that checkout.

## Host scripts

| Script | Purpose |
|--------|---------|
| `host-devcontainer-rebuild.sh` | Rebuild image + recreate container |
| `host-update-vscode-tunnel.sh` | Refresh `code-tunnel` inside the existing container without rebuild/recreate |
| `host-vscode-tunnel.sh` | Start VS Code tunnel into container |
| `warmup-cache.sh` | Background cache warmer (Rust + Python deps) |

## Bind mounts

Persistent state survives container rebuilds via bind mounts from `/srv/devhome/` on the host and the named Docker volume listed below:

| Host path | Container path | Purpose |
|-----------|---------------|---------|
| `/srv/devhome/.codex` | `~/.codex` | Codex config, sessions, credentials |
| `/srv/devhome/.config/gh` | `~/.config/gh` | GitHub CLI auth |
| `/srv/devhome/.cache/uv` | `~/.cache/uv` | Python dependency cache |
| `/srv/devhome/.texlive2023` | `~/.texlive2023` | TeX cache |
| `/srv/devhome/.texmf-var` | `~/.texmf-var` | TeX user generated files |
| `/srv/devhome/.texmf-config` | `~/.texmf-config` | TeX user config |
| `/srv/devhome/.bash_history_dir` | `~/.bash_history_dir` | Shell history |
| Docker volume `msc-math-vscode` | `~/.vscode` | VS Code tunnel auth/state |

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

## Maintenance Decisions

### Before changing this environment

Treat this devcontainer as shared infrastructure, not a scratch setup.

Before editing `devcontainer.json`, `Dockerfile`, `post-create.sh`, or host
scripts:

- recover intent from this README, git history, and the repo commands in
  `AGENTS.md`;
- identify whether the change affects image rebuild cost, post-create runtime,
  host filesystem state, Docker volumes, credentials, cache behavior, or tools
  agents rely on;
- compare at least the no-change option, Dockerfile option, post-create option,
  host-bind option, and Docker-volume option when persistence or freshness is
  involved;
- do not remove or weaken existing setup behavior unless its reason is known or
  the change is explicitly accepted after documenting the risk;
- document accepted tradeoffs here, including rejected alternatives when the
  wrong alternative would be tempting to future agents.

### VS Code tunnel refresh

`code-tunnel` is installed in the Dockerfile and refreshed again in
`post-create.sh`.

Reason:

- the Dockerfile uses the VS Code "latest" URL, but Docker can reuse the cached
  image layer and therefore keep an old `code-tunnel` binary;
- forcing a Docker cache bust before the `code-tunnel` layer would also
  invalidate later heavy layers such as Node, Sage, Rust, and cargo tools;
- `postCreateCommand` runs when the container is recreated, so refreshing
  `/usr/local/bin/code-tunnel` there updates the tunnel CLI without rebuilding
  TeX/Sage/Rust layers.

Cost:

- each recreate downloads the VS Code CLI tarball once;
- if that download fails, post-create fails loudly instead of silently keeping a
  stale tunnel binary.

The host rebuild script prints `code-tunnel --version` after recreate so the
update is visible in the rebuild log.

For a VS Code tunnel CLI update without rebuilding or recreating the container,
run from the host:

```bash
bash .devcontainer/host-update-vscode-tunnel.sh
```

The script discovers the existing devcontainer by its Docker label, copies the
latest stable VS Code CLI to `/usr/local/bin/code-tunnel`, and verifies the
installed binary by copying it back out. It works for stopped containers. If a
tunnel process is already running, restart that tunnel process after the update;
the container itself is not started, stopped, rebuilt, or recreated.

### Cache persistence

Do not add new `/srv/devhome` host paths or Docker volumes just to preserve
ordinary rebuild caches such as Cargo registry, npm cache, pre-commit hook
envs, or Matplotlib cache.

Reason:

- those caches are convenience state, not valuable auth/runtime state;
- extra host paths or Docker volumes add hidden state that future agents must
  understand, clean up, and debug;
- local Cargo build artifacts already persist in the repo-local ignored
  `target/` directory unless the workspace itself is deleted.

Current persistence is intentionally limited to auth/runtime state and caches
that were already part of the host contract: Claude, Codex, GitHub CLI, uv,
TeX user trees, bash history, and the existing VS Code state volume.

### Agent CLI Tool Bundle

The Dockerfile installs a broad but ordinary set of small command-line tools in
the common CLI layer: PDF/text inspection, shell linting/formatting, benchmarking
and profiling, plotting, SQLite, Pandoc, process monitoring, and interactive
navigation tools.

Reason:

- these tools are repeatedly useful for local agent work and should survive
  container rebuilds;
- installing them in the Dockerfile is simpler than post-create installation or
  host bind mounts, because they are ordinary OS packages and not user state;
- they live after the heavyweight TeX layer, so adding or removing them does
  not invalidate the slowest dependency layers.

Cost:

- the image is larger and this layer takes longer to rebuild;
- versions follow Ubuntu 24.04 apt repositories unless a tool is explicitly
  pinned elsewhere.

Ubuntu names `fd` and `bat` as `fdfind` and `batcat`; the Dockerfile adds
`/usr/local/bin/fd` and `/usr/local/bin/bat` symlinks because agents commonly
try the upstream command names.

### DuckDB CLI

DuckDB CLI is installed from the official prebuilt GitHub release because
Ubuntu 24.04 does not package the CLI. The Dockerfile pins
`DUCKDB_VERSION` and checks the downloaded zip against
`DUCKDB_CLI_LINUX_AMD64_SHA256`.

Reason:

- DuckDB is useful for inspecting tabular experiment artifacts and method
  packets without adding Python dependencies or ad hoc parsers;
- a pinned Dockerfile install is more reproducible than a post-create download
  or container-local manual install.

Cost:

- the build depends on GitHub release availability;
- updating DuckDB requires changing both the version and SHA256.

## SageMath In The Local Devcontainer

The local image now installs SageMath in the Dockerfile via the official
conda-forge / Miniforge route, not via Ubuntu `apt`.

Reason:

- on this Ubuntu 24.04 base, `apt-cache policy sagemath` currently has no
  installable candidate even with the standard `main universe restricted
  multiverse` components enabled;
- the official Sage installation guide documents conda-forge as a supported
  installation route.

Practical consequences:

- after a normal rebuild, `sage --version` should work immediately inside the
  container;
- `mamba` and `conda` are also exposed on `PATH` via `/usr/local/bin`;
- the image pins `SAGE_VERSION=10.8` in `.devcontainer/Dockerfile`;
- the Sage environment is large, so rebuilds will take noticeably longer and
  the image will be larger than before.

Minimal acceptance check after rebuild:

```bash
sage --version
cd experiments/hko-local-maximum/theorem/exact-witness
python3 build_widened_representative_witness.py
sage verify_widened_representative_witness.sage
```
