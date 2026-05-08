---
name: licca
description: Use when Codex prepares, reviews, or edits LICCA/cluster/external-execution work, including Slurm scripts, resource choices, handoff instructions for Jörn, retrieval instructions, or local-vs-cluster execution boundaries.
---

# LICCA

## Cluster and external execution

- agents do not have LICCA SSH access; prepare scripts, binaries, resource
  choices, and retrieval instructions for Jörn instead
- Jörn submits cluster jobs and retrieves external results unless the files are
  already present locally
- resource choices need a short justification
