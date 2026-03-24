---
name: slurm
description: LICCA cluster job submission for this project. Load when an experiment needs cluster compute. Covers the handoff workflow (agent writes script, Jörn submits) and project-specific template.
---

## When to use

- Experiment exceeds the devcontainer's 10-minute CPU limit
- Inherently expensive computation (large sweeps, dataset generation)

**Agents must NEVER have SSH access to LICCA.** No SSH keys, no ControlMaster sockets, no stored credentials. Claude Code runs as the same devcontainer user, so any open connection would give it access to a shared university cluster. Agent writes the job script + Rust binary; Jörn submits on LICCA and retrieves results manually.

## How to write a job script

Copy `references/experiment.sh` to `experiments/<experiment>/job.sh`. Fill in the TODOs. The template is pre-configured for this repo's layout (`experiments/Cargo.toml` binaries, `$HOME/msc-math` repo path on LICCA).

Key project-specific details already in the template:
- `cargo build/run` paths use `--manifest-path experiments/Cargo.toml`
- Partition is `epyc` (LICCA's general CPU partition, AMD EPYC 128-core nodes)
- Cargo env sourced from `$HOME/.cargo/env` (rustup, not a system module)

## Resource justification (MANDATORY)

Every job script must include a table justifying every resource flag:

| Flag | Value | Why |
|------|-------|-----|
| `--partition` | ... | Why this partition, what's the time limit |
| `--cpus-per-task` | ... | Why this many CPUs for this workload |
| `--mem` | ... | Why this much memory for this workload |
| `--time` | ... | Expected runtime + safety margin reasoning |

Never pick resource values without explaining why they're correct for this specific job.

## Handoff to Jörn

Include in your message:
1. What the job computes
2. The resource justification table (above)
3. Output file path(s)

Jörn's commands for building/submitting/retrieving: `references/licca-setup.md`.
