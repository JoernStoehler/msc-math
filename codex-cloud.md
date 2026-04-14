# Codex Cloud

Purpose: define the low-complexity Codex cloud environment for `msc-math`.
This environment is for phone-first, parallel, session-sized coding work:
code inspection, code changes, Rust validation, Python analysis, and PR
creation. It is not a replacement for the local devcontainer.

## V1 Guarantees

- Rust binaries, tests, and clippy should run in cloud.
- Python analysis should run in cloud on smoke-generated or otherwise hydrated inputs.
- Git LFS is installed in cloud, but this checkout mode does not guarantee hydrated LFS payloads.
- Cloud tasks must not assume committed LFS files are real data.
- Agent internet access should be unrestricted.
- TeX is intentionally out of scope in v1.

## Canonical Setup

Use the committed setup script directly:

```bash
bash scripts/codex-cloud-setup.sh
```

That script is the source of truth for:

- repo-specific package installs
- Python pre-caching
- environment verification

Do not maintain a second handwritten setup recipe in the Codex UI.

Use the committed maintenance script directly:

```bash
bash scripts/codex-cloud-maintenance.sh
```

That script is the source of truth for resumed cached-container refresh.

## Canonical Smoke Test

Use the committed smoke script directly:

```bash
bash scripts/codex-cloud-smoke.sh
```

This script checks:

- `cargo`, `uv`, `git lfs`, `qconvex`
- `cd library/ && cargo test --release --lib`
- `cd library/ && cargo clippy --lib -- -D warnings`
- one representative experiment build
- one representative Python analysis run on self-generated smoke data

## What The Setup Adds

The default Codex `universal` image already covers most of the common stack.
The repo-specific addition we currently need is `qhull-bin`, because the Rust
library shells out to `qconvex` in `library/src/geom/qhull.rs`.

The setup script also pre-caches:

- `numpy`
- `matplotlib`
- `scipy`

so normal Python analysis does not repeatedly spend time downloading them at
the start of a session.

The setup script pre-caches Python packages. It does not hydrate Git LFS
payloads.

The setup script also precompiles the Rust validation path used by the cloud
smoke workflow:

- library release test artifacts via `cargo test --release --lib --no-run`
- library clippy artifacts via `cargo clippy --lib --no-deps -- -D warnings`
- the representative experiment binary `sys-random-sample`
- the representative experiment binaries used by the cloud smoke workflow

This moves the expensive Rust cold-start cost into environment setup so the
first real cloud session is much closer to ready-to-use.

The maintenance script reruns that same Rust warm-up on resumed cached
containers after Codex checks out the task branch. This keeps follow-up tasks
from paying the full compile cost again after branch or dependency drift.

## Git LFS In This Cloud Mode

In the observed Codex cloud checkout mode for this repo:

- `git-lfs` is installed
- LFS tracking metadata is present
- committed LFS files can still appear as pointer files
- the checkout may have no git remote, so missing LFS payloads cannot be fetched in-session

So dataset-backed analysis on committed `.jsonl` files is not a safe cloud
assumption. The canonical smoke script therefore uses a self-generated Python
smoke dataset instead of relying on committed LFS payloads.

## What V1 Does Not Cover

- `thesis/` TeX builds
- `cd formal/ && latexmk`
- LICCA / slurm submission and retrieval
- local host workflows documented in `.devcontainer/`

If a task needs those, use the local devcontainer workflow instead.

## Known Risk

The local environment currently uses Rust `1.94.0`. The published Codex
`universal` image currently advertises Rust versions up to `1.92.0`. Treat the
smoke script as the actual compatibility check before trusting cloud Rust
execution for a task.

Some Codex cloud environment modes may also block `apt` access behind a proxy.
The setup script now checks for `qconvex` before touching `apt`. If `qconvex`
is already present, setup proceeds without package installation. If `qconvex`
is absent and `apt` is blocked, the script fails with a targeted message
explaining that Rust validation cannot meet the cloud smoke contract there.

## What You Still Need To Configure In Codex Cloud

I can commit the scripts and docs here, but I cannot click the Codex web UI
for you. In the cloud environment configuration:

1. Select the default `universal` environment.
2. Enable unrestricted internet during agent execution.
3. Set the setup command to:

```bash
bash scripts/codex-cloud-setup.sh
```

4. Set the maintenance command to:

```bash
bash scripts/codex-cloud-maintenance.sh
```

5. After the environment is created, run:

```bash
bash scripts/codex-cloud-smoke.sh
```

If that passes, the environment is ready for travel/mobile code sessions.
