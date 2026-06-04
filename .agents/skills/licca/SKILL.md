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

## Practical LICCA workflow notes

- Prefer self-contained Slurm scripts named `*.slurm.sh` for reviewed jobs.
  Put resource choices, seed ranges, output paths, resume rules, and exact
  binary commands in the script. This is easier to audit than important
  `sbatch` settings living only in chat or CLI flags.
- Use `/hpc/gpfs2/scratch/u/stoehljo/cargo-target` as the LICCA
  `CARGO_TARGET_DIR` for Rust builds unless Jörn says the storage layout
  changed.
- LICCA currently has system `python3` available, observed as Python 3.12.3 on
  2026-06-04. It did not have `uv` available then. For standard-library helper
  scripts on LICCA, use `python3 script.py`, not `uv run --script`.
- GitHub password authentication is not supported for Git operations. Unless
  LICCA GitHub SSH/token auth has been explicitly set up, do not ask Jörn to
  push from LICCA. Retrieve artifacts with `scp` through the gateway and commit
  from the local/devcontainer environment.
- Example retrieval from the local host, using the same gateway:

```bash
scp -o IdentitiesOnly=yes -o PubkeyAuthentication=no \
  -o ProxyJump=stoehljo@xlogin.uni-augsburg.de \
  stoehljo@licca-li-01.rz.uni-augsburg.de:~/artifact.tgz \
  ~/workspaces/msc-math/.worktrees/<worktree>/
```

- Be careful with host paths versus devcontainer paths. Jörn's host checkout was
  observed at `~/workspaces/msc-math/`; the devcontainer path is
  `/workspaces/msc-math/`.

## Slurm and data-output rules

- Shard outputs should be separate files. Avoid concurrent writes to one JSONL
  or cache file.
- Rust ascent shard resume works only for the same output summary file. A rerun
  of the same shard can skip completed rows from that file; a different output
  directory or seed layout is not a global cache.
- Do not delete partial shard files after Slurm timeout if resume is desired.
  Rerun the same script with the same array index and constants.
- If `squeue` shows `TIME` such as `0:16`, that is seconds/minutes formatting
  from Slurm, not necessarily minutes. Confirm elapsed time with `sacct` when
  reporting wall time.
- For completed job timing and core-hours, ask Jörn to run `sacct` on LICCA,
  for example:

```bash
sacct -j <jobid> --format=JobID,JobName%24,Partition,State,Elapsed,AllocCPUS,CPUTime,MaxRSS,ExitCode
```

- For logs, use `tail`, `grep`, and row counts before interpreting results:

```bash
grep -R "\*\*\* VITERBO VIOLATION" logs/*.out 2>/dev/null || true
wc -l path/to/shards/*.jsonl
```

- Login-node table builds or local postprocessing can be CPU-active for many
  minutes with no new log lines. Check `ps -o pid,etime,pcpu,pmem,rss,cmd`
  before calling a quiet process hung.
