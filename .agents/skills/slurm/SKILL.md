---
name: slurm
description: LICCA cluster job submission. Load when an experiment needs more compute than the devcontainer provides, such as runs longer than 10 minutes, large sweeps, or dataset generation.
---

# LICCA Cluster Workflow

**Agents NEVER have SSH access to LICCA.** Agent writes the job script + binary; Jörn submits and retrieves results.

## Steps

1. **Write/update the experiment entrypoint** in `experiments/<topic>/<experiment>/main.rs`
2. **Copy the template** from `references/experiment.sh` to `experiments/<topic>/<experiment>/job.sh`
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

## After Jörn retrieves results

Jörn scps result files into the repo, then commits: `git add <file> && git commit -m "Add <experiment> results from LICCA"`. Git LFS handles the upload on push (transparent — .jsonl files are LFS-tracked via `.gitattributes`).
