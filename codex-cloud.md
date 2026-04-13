# Codex Cloud

Purpose: define the low-complexity Codex cloud environment for `msc-math`.
This environment is for phone-first, parallel, session-sized coding work:
code inspection, code changes, Rust validation, Python analysis, and PR
creation. It is not a replacement for the local devcontainer.

## V1 Guarantees

- Rust binaries, tests, and clippy should run in cloud.
- Normal Python analysis scripts should run in cloud.
- Git LFS data may be used in cloud.
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

## Canonical Smoke Test

Use the committed smoke script directly:

```bash
bash scripts/codex-cloud-smoke.sh
```

This script checks:

- `cargo`, `uv`, `git lfs`, `qconvex`
- `cd crates/library/ && cargo test --release --lib`
- `cd crates/library/ && cargo clippy --lib -- -D warnings`
- one representative experiment build
- one representative `uv run analyze.py`

## What The Setup Adds

The default Codex `universal` image already covers most of the common stack.
The repo-specific addition we currently need is `qhull-bin`, because the Rust
library shells out to `qconvex` in `crates/library/src/geom/qhull.rs`.

The setup script also pre-caches:

- `numpy`
- `matplotlib`
- `scipy`

so normal Python analysis does not repeatedly spend time downloading them at
the start of a session.

## What V1 Does Not Cover

- `thesis/` TeX builds
- `cd crates/ && latexmk`
- LICCA / slurm submission and retrieval
- local host workflows documented in `.devcontainer/`

If a task needs those, use the local devcontainer workflow instead.

## Known Risk

The local environment currently uses Rust `1.94.0`. The published Codex
`universal` image currently advertises Rust versions up to `1.92.0`. Treat the
smoke script as the actual compatibility check before trusting cloud Rust
execution for a task.

## What You Still Need To Configure In Codex Cloud

I can commit the scripts and docs here, but I cannot click the Codex web UI
for you. In the cloud environment configuration:

1. Select the default `universal` environment.
2. Enable unrestricted internet during agent execution.
3. Set the setup command to:

```bash
bash scripts/codex-cloud-setup.sh
```

4. After the environment is created, run:

```bash
bash scripts/codex-cloud-smoke.sh
```

If that passes, the environment is ready for travel/mobile code sessions.
