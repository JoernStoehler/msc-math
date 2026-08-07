# Development environment

This directory owns the msc-math development image. The surrounding files own
different parts of the executable declaration:

- [`../compose.yaml`](../compose.yaml) owns runtime mounts, limits, networking,
  process identity, and temporary filesystems.
- [`Dockerfile`](Dockerfile) owns system packages and toolchains. Its layer
  comments state the expected change frequency, invalidators, and why related
  installs share a layer.
- [`locks/`](locks/) owns exact package graphs where exact resolution has enough
  value to justify maintaining a lock.
- [`../Justfile`](../Justfile) owns lifecycle operations that Compose cannot
  express by itself: host validation, the constrained builder, smoke tests,
  authentication bootstrap, and the optional Codex app-server.
- [`../.env.example`](../.env.example) owns the small set of machine-local
  inputs. The ignored `.env` is not a secret store.

Read those files for current behavior. This document records why the pieces
exist, which alternatives were rejected, and where the reasoning should be
rechecked. It is a navigation and decision aid, not authority over a
contradicting executable configuration or runtime observation.

## Intended workflow

The host supplies Docker, persistent storage, and external connectivity. Agents
develop inside one shared container at `/workspaces/msc-math`; they do not treat
the host as a second development environment.

The runtime user is the host-matching, non-root `developer`. The container has a
writable overlay and passwordless `sudo` so an agent can follow the normal
development loop:

1. install a missing system tool experimentally;
2. use it to complete or diagnose the task;
3. add the package to the appropriate Dockerfile layer;
4. rebuild when durable reproduction is needed.

Experimental root changes survive container stop/start but disappear when the
container is replaced. A package is not part of the declared environment until
the Dockerfile contains it.

Docker remains the outer isolation boundary. Codex's inner Linux sandbox is
disabled deliberately: the container exposes only the project and two explicit
state directories, does not mount the Docker socket, and retains Docker's
ordinary unprivileged seccomp/capability boundary. Passwordless root inside this
container is not host root.

## Persistence model

| Material | Storage | Lifetime and reason |
|---|---|---|
| Repository, project data, nested worktrees, durable notes, build outputs | Host bind at `/workspaces/msc-math` | Valuable project state; visible to host backup |
| Codex sessions, configuration, and OAuth | Narrow host bind at `$CODEX_HOME` | Sessions are the highest-value operational record |
| GitHub CLI authentication | Narrow host bind at `~/.config/gh` | Small but annoying to recreate |
| Ordinary home state and caches | Container writable layer | Useful across stop/start; intentionally discarded on replacement |
| System-package experiments | Container writable layer | Promote useful changes to the Dockerfile |
| `/tmp` and `/var/tmp` | Size-bounded tmpfs | Immediate disposable work; never a durable evidence location |
| `/run` and per-user runtime state | Small tmpfs | Process-lifetime state |
| Docker build cache | BuildKit-managed cache | Replaceable acceleration, inspected/pruned explicitly |

There are no named volumes. No selected state needs Docker-managed portability
or sharing, while the valuable state benefits from direct host inspection and
backup.

Do not invent separate `data`, `notes`, `repos`, or `worktrees` filesystems
without a concrete ownership or lifecycle requirement. Repository-local data
often needs to vary with a worktree, and one filesystem keeps Git operations,
relative paths, and agent navigation compositional. Use `/tmp` for disposable
notes and the relevant repository/worktree for durable ones.

## Worktrees and concurrency

Worktrees live under `.worktrees/` and are created and used inside the
container. Git records absolute paths in worktree metadata; host-created
worktrees therefore require a one-time `git worktree repair` from inside the
container. Host-side worktree usability is not a requirement.

Agents share the container because per-agent containers would add startup,
copying, and communication cost without protecting valuable state from agents
that already need repository access. Git worktrees provide source-change
isolation; the shared process namespace and resource budget are deliberate.

The 10 GiB memory/no-extra-swap and PID limits protect the host. They are policy,
not machine-local tuning knobs. The dedicated Buildx `docker-container` builder
applies the same memory policy to image builds; Compose runtime limits do not
constrain BuildKit.

## Toolchain and rebuild policy

Pinning follows expected value rather than one universal rule:

- The Ubuntu base is digest-pinned.
- Ubuntu packages use the ordinary live 24.04 archive. Pinning every APT
  package is brittle; the attempted dated-snapshot route failed at repository
  trust/bootstrap. Preserve an image digest if an exact built realization is
  needed.
- Standalone downloaded tools are version- and checksum-pinned.
- Rust follows the repository toolchain pin.
- Pre-commit uses a hash lock.
- Sage 10.9/Python 3.12 uses an explicit Conda package lock because its large,
  expensive dependency graph benefits from exact resolution.
- Vendor Codex intentionally follows `@latest` and is installed by `just up`.
  Its fast release cadence makes an image pin counterproductive; sessions and
  authentication persist independently under `$CODEX_HOME`.

Large, stable layers precede small or frequently changed layers. The sudo/user
configuration is intentionally after Sage, Rust, and pre-commit so development
ergonomics and host UID changes do not rebuild expensive toolchains.

The current image is Linux/AMD64 and the Sage lock requires x86-64-v3.
`just validate` fails early on an incompatible host. Regenerate the lock only
when supporting such a host is an actual requirement; doing so invalidates the
large Sage layer and everything after it.

## Codex app-server

The app-server is optional and explicitly started in tmux by
`just app-server-up`. Automatic startup would add hidden process state to every
workspace lifecycle even when no client needs it.

Clients have two supported routes:

- peer container: join external network `msc-math-dev` and connect to
  `ws://msc-math:4500`;
- host: connect to `ws://127.0.0.1:4500`.

Docker publishes only loopback. Tailscale, SSH, Cloudflare, or another trusted
host mechanism owns deliberate external exposure and TLS. The native
capability-token file lives under `$CODEX_HOME`; a client that bind-mounts that
single file must be recreated after atomic token rotation.

## Rejected architectures

- **Dev Containers, Codespaces, and VS Code tunnel:** they add an editor control
  plane not used by the normal SSH/container/tmux workflow. They can be
  reintroduced later as a thin consumer of this environment.
- **Host development:** it loses package-manager, process, filesystem, and
  resource isolation.
- **A VM:** it adds startup and synchronization cost without improving the
  selected persistence boundary.
- **One container per agent:** it makes lifecycle and communication expensive
  while Git worktrees already isolate source changes.
- **A persistent full home:** it promotes uncontrolled state from every tool
  into backup-worthy state.
- **A tmpfs full home and read-only root:** it made the environment clean but
  reversed the normal install-try-promote development loop and spent RAM on
  ordinary caches.
- **Named volumes for credentials, tools, caches, data, or notes:** they either
  obscure valuable state from host backup or add lifecycle machinery for
  replaceable state.
- **Baking Codex or maintaining a Codex binary volume:** both add rebuild or
  seeding/version-switch machinery to an installation that takes seconds.
- **An app-server Compose service or startup daemon:** it creates another
  lifecycle for an optional process already operable through the shared
  container and tmux.
- **A synthetic multi-repository workspace:** Codex and Codex GUI are external
  tools developed elsewhere; this image belongs to msc-math alone.

## How to reassess this design

Do not trust these conclusions merely because they are documented. When a
requirement changes or a sharp edge appears:

1. identify the concrete actor and workflow being helped or blocked;
2. inspect the executable owner above and the observed container state;
3. compare the smallest standard alternatives, including maintenance,
   rebuild, wall-time, persistence, and recovery costs;
4. test the highest-risk lifecycle boundary before polishing secondary details;
5. update the executable declaration and then update this reasoning if the
   decision changed.

Use `just validate` before building, `just doctor` after starting, and
`just status` for the container, builder, network, cache, and disk overview.
Do not treat a passing static render as proof that worktrees, sudo, Codex,
authentication, or app-server networking work; those require their runtime
checks.
