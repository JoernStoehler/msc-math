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

## Login path for Jörn

- Do not guess a local alias such as `ssh licca`.
- For external access from home, use the University of Augsburg gateway with
  SSH `ProxyJump`:

```bash
ssh -t -o IdentitiesOnly=yes -o PubkeyAuthentication=no \
  -J stoehljo@xlogin.uni-augsburg.de \
  stoehljo@licca-li-01.rz.uni-augsburg.de
```

- The no-pubkey options avoid "Too many authentication failures" when Jörn's
  local SSH agent offers too many keys before password authentication.
- On first connection, the LICCA ED25519 host key fingerprint observed in the
  Augsburg HPC docs and confirmed by Jörn on 2026-06-04 is:

```text
SHA256:ZKi0w4Cc24qHbrLQKXX/ifYQ92208g2yhCVPHvgxWz8
```

- Once Jörn is on the LICCA login node, give ordinary LICCA-side commands such
  as `sinfo`, `squeue`, `sbatch`, `git`, `cargo`, and retrieval commands.
- Login nodes are for light editing, transfers, job submission, and monitoring;
  serious computation must go through Slurm.
