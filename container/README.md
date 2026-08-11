# Development environment

This directory owns the msc-math development image. The surrounding files own
different parts of the executable declaration:

- [`../compose.yaml`](../compose.yaml) owns mounts, limits, networking,
  process identity, and temporary filesystems.
- [`Dockerfile`](Dockerfile) owns system packages and toolchains. Its layer
  comments state the expected change frequency and cache invalidators.
- [`locks/`](locks/) owns exact package inputs and realizations.
- [`common.sh`](common.sh), [`image.sh`](image.sh),
  [`lifecycle.sh`](lifecycle.sh), and [`app-server.sh`](app-server.sh) own the
  narrow host guards that Compose cannot express safely.
- [`../Justfile`](../Justfile) is the small public command index.
- [`../.env.example`](../.env.example) owns the four machine-local inputs.

Executable files and observed runtime state overrule this rationale if they
disagree. Update both when an accepted decision changes.

## Host contract and normal workflow

The host supplies a local Unix-socket Docker Engine, Compose, `just`, Git,
`jq`, `flock`, `realpath`, persistent storage, and external
connectivity. Docker and its socket are deliberately absent inside the
workspace. The host CPU must be x86-64-v3 because the current Sage lock uses
that target.

Create mode-`0600` `.env` from `.env.example`; its two credential-state
directories must be absolute, canonical, mode-private, owned by the configured
UID/GID, and already exist. Lifecycle commands fix the Compose project, file,
env file, and machine inputs; they also disable ambient orphan removal.

The ordinary sequence is:

1. `just validate` checks the host inputs and renders the Compose model.
2. `just image-build` builds a per-invocation candidate with Docker's default
   BuildKit backend, smoke-tests that immutable image ID, and only then promotes
   `msc-math-workspace:local`. It never replaces the running workspace.
3. `just dev-start` starts an existing stopped container, or creates one only when no
   workspace container exists. It never reconciles or recreates an existing
   container, then starts the pinned app-server used by the tracked
   SubagentStop wake hook.
4. `just dev-status` proves the container and app-server contract.
5. `just shell` opens an interactive login shell.

`just container-recreate` is the explicit destructive transition after an accepted image
or Compose change. It stops the app-server, force-recreates the workspace,
and discards its writable overlay. Run it only after inspecting the existing container and deciding
that its replaceable overlay contains nothing to promote.

Normal lifecycle does not own or start a persistent builder container. Image
promotion and lifecycle mutations share one host lock.

## Runtime boundary

Agents develop inside one shared container at `/workspaces/msc-math`; the host
is not a second development environment. The image's `developer` UID/GID is the
single runtime identity authority. Labels record the baked values, and start
fails if either the image or an existing container disagrees with `.env`.

The container has a writable overlay and passwordless `sudo` for the normal
install–try–promote loop:

1. install a missing tool experimentally;
2. use it to complete or diagnose the task;
3. add it to the appropriate late Dockerfile layer;
4. rebuild only when durable reproduction is worth the cost.

Experimental root changes survive stop/start and disappear on replacement. A
package is declared only when the Dockerfile contains it. Docker remains the
outer isolation boundary: the container has no Docker socket, mounts only the
project and two state directories, and retains Docker's ordinary unprivileged
seccomp/capability boundary.

## Persistence and recovery

| Material | Storage | Lifetime |
|---|---|---|
| Repository, `.git`, nested `.worktrees`, durable artifacts | Host bind at `/workspaces/msc-math` | Survives stop, replacement, rebuild, and host reboot |
| Codex sessions, configuration, OAuth, SQLite state | Narrow host bind at `$CODEX_HOME` | Survives replacement; highest-value operational record |
| GitHub CLI authentication | Narrow host bind at `~/.config/gh` | Survives replacement |
| Ordinary home, `.local`, caches, package experiments | Container writable layer | Survives stop/start; discarded on replacement |
| `/tmp`, `/var/tmp`, `/run` | Size-bounded tmpfs | Process/container-lifetime disposable state |
| Build cache | BuildKit-managed | Replaceable acceleration |

The recoverable machine state is the repository, Codex-state directory, and
GitHub-state directory named by `.env`. Back up all three together.
A clone is not equivalent: it omits local-only Git history, ignored `.env` and
token files, Codex sessions, GitHub auth, and dirty/untracked worktree state.
Verify off-machine backup coverage and perform a read-only restore/listing test;
"host-visible" alone does not prove disaster recovery.

Back up the repository root, `.git`, and `.worktrees` as one unit. `.worktrees`
is ignored but may contain valuable dirty checkouts, while `.git/worktrees`
contains their reciprocal administration. Restoring at a different host path is
supported while Compose still binds it to `/workspaces/msc-math`, ownership
matches `.env`, and Git's linked-worktree metadata is repaired as described
below.

Worktrees are created and used inside the container. A worktree created outside
the repository bind is invisible and cannot be repaired from the container. If
a visible worktree was created or moved on the host, quiesce every agent using
it and repair from the healthy primary checkout, not the broken worktree:

```bash
git -C /workspaces/msc-math worktree repair \
  /workspaces/msc-math/.worktrees/<name>
git -C /workspaces/msc-math worktree list --porcelain
git -C /workspaces/msc-math/.worktrees/<name> rev-parse \
  --show-toplevel --git-dir --git-common-dir
```

## Toolchain and rebuild policy

Pinning follows expected value rather than one universal rule:

- Ubuntu is digest-pinned; APT packages use the live Ubuntu 24.04 archive.
- Standalone downloads are version- and checksum-pinned.
- Rust follows `rust-toolchain.toml` and exact Cargo-tool versions.
- Pre-commit has a source constraint and hash lock.
- Sage 10.9/Python 3.12 has source constraints plus an explicit Conda
  realization because resolving it is expensive.
- Codex is explicitly pinned by `CODEX_VERSION` and baked into a late,
  inexpensive image layer shared with the GUI/app-server protocol.

The existing TeX, standalone-CLI, and Sage layer keys stay stable across this
migration. General `PATH` is reset immediately after Sage, so Rust and
pre-commit rebuild without inheriting the Conda toolchain; small diagnostics and
the explicit tool symlinks remain late. The Sage and pre-commit environments
are not put on general `PATH`: only `sage`, `conda`, `mamba`, and `pre-commit`
are exposed. Ordinary Python, compilers, `pkg-config`, `curl`, and `pandoc`
therefore remain the Ubuntu tools.

The source files beside the generated locks own refresh recipes. Conda-forge
and Ubuntu are live repositories, so regeneration is an intentional reviewable
update, not a promise to reconstruct a past solve byte-for-byte.

### Codex upgrades

Change `CODEX_VERSION` deliberately, rebuild and smoke-test the image, then
test the app-server and every affected GUI target before explicitly recreating
the container. When a concrete regression or schema/configuration change
appears, use `codex doctor --json`, a SubagentStop wake integration, and an
affected GUI handshake to localize it.
Refresh the live model catalog or base-instruction comparison only when those
surfaces changed, and obtain exact review before replacing prompt-bearing/user
configuration. Reinstall a prior explicit npm version only when a concrete
regression makes rollback cheaper than immediate follow-up repair.

App-server WebSocket transport and generated schemas are version-specific.
`just dev-status` deliberately checks process and listener readiness,
not a synthetic RPC. The relay and app-server read the same repository token;
the relay's focused test covers its authorization header, and an actual
SubagentStop completion is the end-to-end wake-path check.

The relocation left many derived `state_5.sqlite` rollout locators rooted at
the old home. Rollout JSONL, history, logs, memories, and snapshots remain
unchanged, and current GUI/session use is healthy. Leave the stale derived rows
alone unless a focused resume failure demonstrates a repair is needed; do not
rewrite source history or rebuild the healthy database merely to normalize
paths.

## Codex app-server

The app-server is optional for a plain shell but required for the tracked root
wake harness. `just dev-start` is the normal agent-ready path. The server runs in
detached tmux inside the workspace; failure leaves a retained pane for
`just app-server-logs`. Stop and workspace-stop paths send it an interrupt and
wait for readiness to disappear before container shutdown.
The tracked relay wakes an idle root after one of its direct children stops.
Codex 0.147 multi-agent v2 intentionally rejects direct input to spawned
subagent threads, so a nested child cannot use this route to wake its immediate
subagent parent; nested owners must remain active while waiting for descendants.

Clients have two routes:

- a peer container joins the shared `joern-dev` bridge and
  connects to `ws://msc-math:4500`;
- a host client connects to `ws://127.0.0.1:4500`.

Both WebSocket upgrades send the bearer capability token; `/readyz` alone is an
unauthenticated listener check. Docker publishes only loopback. Another trusted
host mechanism owns deliberate external exposure and TLS. The wake relay uses
its fixed loopback endpoint, so configuration cannot redirect the capability
token.

This repository reads its ignored mode-`0600` `.app-server-token`; `codex-gui`
currently owns an identical ignored copy. General bootstrap and core doctor do
not require that optional secret. Token replacement is coordinated maintenance:
stage and verify every consumer copy, replace them with rollback available,
restart app-servers and GUI backends, and prove an authenticated handshake
without printing the token.

## Rejected alternatives and reassessment

- Dev Containers, Codespaces, and VS Code tunnel add an unused editor control
  plane. They may return later only as a thin consumer of this Compose setup.
- Host development loses package, process, filesystem, and resource isolation.
- A VM adds startup/synchronization cost without improving selected state.
- One container per agent adds lifecycle and communication cost while Git
  worktrees already isolate source.
- A full-home bind promotes uncontrolled tool state into backup-worthy state.
- A read-only root/tmpfs home blocks the normal install–try–promote loop.
- Named volumes obscure valuable state from host backup or add machinery for
  replaceable state.
- Installing unpinned Codex into the writable overlay makes app-server protocol
  compatibility depend on whichever startup last contacted npm.
- Making the optional app-server PID 1 would couple its crash to every shell;
  a second Compose service would need a separate Codex installation lifecycle.

Reassess a choice only after identifying a concrete blocked actor/workflow,
observing the executable owner and runtime, and comparing the smallest standard
alternatives by submission wall time, recovery, rebuild cost, and maintenance.
Use `just image-status` for stopped as well as running state; do not mistake static
Compose rendering or `/readyz` for an end-to-end acceptance test.
