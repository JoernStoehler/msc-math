---
name: licca
description: Use before writing any command Jörn should run on, to, or from LICCA, including SSH login, SCP retrieval, Slurm scripts, sbatch/squeue/sacct commands, resource choices, handoff instructions, and local-vs-cluster execution boundaries.
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
- Keep local commands and LICCA-side commands in separate code blocks. Never put
  `ssh`/`scp` in the same command block as commands intended to run after login.
- For handoff commands, label each execution context: local machine, LICCA login
  node, or Slurm job.
- For external access from home, use the University of Augsburg gateway with
  an explicit `ProxyCommand`. This form is currently preferred over `-J`
  because Jörn observed the `ProxyJump` form still failing with "Too many
  authentication failures" on 2026-06-11:

```bash
ssh -t \
  -o IdentitiesOnly=yes \
  -o PubkeyAuthentication=no \
  -o PreferredAuthentications=password,keyboard-interactive \
  -o 'ProxyCommand=ssh -o IdentitiesOnly=yes -o PubkeyAuthentication=no -o PreferredAuthentications=password,keyboard-interactive -W %h:%p stoehljo@xlogin.uni-augsburg.de' \
  stoehljo@licca-li-01.rz.uni-augsburg.de
```

- The no-pubkey options avoid "Too many authentication failures" when Jörn's
  local SSH agent offers too many keys before password authentication. This
  command asks for the password twice: once for `xlogin.uni-augsburg.de` and
  once for `licca-li-01.rz.uni-augsburg.de`.
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
- Example retrieval from the local host, using the same gateway style:

```bash
scp \
  -o IdentitiesOnly=yes \
  -o PubkeyAuthentication=no \
  -o PreferredAuthentications=password,keyboard-interactive \
  -o 'ProxyCommand=ssh -o IdentitiesOnly=yes -o PubkeyAuthentication=no -o PreferredAuthentications=password,keyboard-interactive -W %h:%p stoehljo@xlogin.uni-augsburg.de' \
  stoehljo@licca-li-01.rz.uni-augsburg.de:~/artifact.tgz \
  ~/workspaces/msc-math/.worktrees/<worktree>/
```

- Be careful with host paths versus devcontainer paths. Jörn's host checkout was
  observed at `~/workspaces/msc-math/`; the devcontainer path is
  `/workspaces/msc-math/`.

## Slurm and data-output rules

- For nontrivial resource choices or changes, do not guess limits from vibes.
  Before recommending `--time`, `--cpus-per-task`, memory, array shape, or
  cancel/resubmit decisions, state:
  - the objective: calendar time, core-hours, timeout risk, Jörn intervention
    cost, and correctness/topology risk
  - a runtime BOTEC from job units, parallelism, and per-unit budget or measured
    timings
  - the cost of timeout, including lost core-hours, delayed downstream work, and
    expected Jörn follow-up loops
  - the lowest-risk variable to change first; prefer wall time or CPU count
    changes before changing shard/output topology
  - scheduler evidence when available, using `sbatch --test-only`,
    `squeue --start`, or `sacct` before asking Jörn to cancel/resubmit
- For CPU-parallel production jobs whose inner work uses Rayon or many
  independent CPU-bound units, treat `64` CPUs as the normal first LICCA
  production candidate, not `32`, unless the job is known small or scheduler
  tests penalize 64 CPUs. Still compare with `sbatch --test-only` when changing
  resources. This rule is based on the 2026-06-11 datascience table scheduler
  checks where `32 CPU / 6h`, `64 CPU / 4h`, and `64 CPU / 6h` had essentially
  identical start estimates.
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
