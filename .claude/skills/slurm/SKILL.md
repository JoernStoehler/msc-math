---
name: slurm
description: LICCA cluster job submission. Load when an experiment needs more compute than the devcontainer provides (>10 min, large sweeps, dataset generation).
---

# LICCA Cluster Workflow

**Agents NEVER have SSH access to LICCA.** Agent writes the job script + binary; Jörn submits and retrieves results.

## Steps

1. **Write/update the experiment binary** in `crates/exp-<group>/<subdir>/run.rs`
2. **Copy the template** from `references/experiment.sh` to `crates/exp-<group>/<subdir>/job.sh`
3. **Fill in the TODOs** in the job script (binary name, resources, arguments)
4. **Write resource justification table** (mandatory):

| Flag | Value | Why |
|------|-------|-----|
| `--partition` | ... | Why this partition |
| `--cpus-per-task` | ... | Why this many CPUs |
| `--mem` | ... | Why this much memory |
| `--time` | ... | Expected runtime + safety margin |

5. **Present to Jörn:** what the job computes, the resource table, expected output paths

Jörn's submission/retrieval commands: `references/licca-setup.md`