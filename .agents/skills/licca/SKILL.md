---
name: licca
description: Use before writing any command Jörn should run on, to, or from LICCA, including SSH login, SCP retrieval, Slurm scripts, sbatch/squeue/sacct commands, resource choices, handoff instructions, and local-vs-cluster execution boundaries.
---

# LICCA

Agents do not have LICCA SSH access. Prepare scripts, binaries, resource
choices, and handoff/retrieval commands for Jörn. Jörn submits LICCA jobs and
retrieves external results unless the files are already local.

## Command Handoffs

- Keep local-machine commands, LICCA login-node commands, Slurm submission
  commands, Slurm monitoring commands, and local retrieval commands in separate
  command blocks. Label the execution context. Do not bundle cleanup, checkout,
  submission, monitoring, result checks, promotion, or retrieval into one pasted
  block unless Jörn explicitly asks for a combined script.
- Do not give interactive or indefinitely blocking LICCA handoff commands such
  as `tail -f`, `watch`, pagers, interactive `srun`, shell loops, or commands
  that wait for completion without a bounded result. Use bounded snapshots.
- For submitted jobs, prefer `sbatch --parsable` assigned to a descriptive shell
  variable such as `produce_jid` or `table_jid`, then use that variable in the
  next monitoring and validation commands. This reduces retyping errors while
  keeping each command block bounded and inspectable.
- Login nodes are for light editing, transfers, job submission, and monitoring.
  Nontrivial computation goes through Slurm.
- Do not ask Jörn to push from LICCA. LICCA can pull from GitHub for this
  project; generated artifacts should be retrieved and committed locally.
- If LICCA `git checkout` or `git pull` is blocked by dirty generated artifacts,
  resolve only the exact paths Git reports as blockers. For tracked blockers,
  restore those paths. For untracked blockers, move those paths aside. Do not
  broad-clean experiment/data directories, create a new clone/worktree/export,
  or switch to tarball transfer unless Jörn asks or the blocker changes.

## Current Facts

Default SSH/SCP route: use the gateway `ProxyCommand` form below unless Jörn
supplies a current working alias.

```bash
ssh -t \
  -o IdentitiesOnly=yes \
  -o PubkeyAuthentication=no \
  -o PreferredAuthentications=password,keyboard-interactive \
  -o 'ProxyCommand=ssh -o IdentitiesOnly=yes -o PubkeyAuthentication=no -o PreferredAuthentications=password,keyboard-interactive -W %h:%p stoehljo@xlogin.uni-augsburg.de' \
  stoehljo@licca-li-01.rz.uni-augsburg.de
```

The no-pubkey options avoid "Too many authentication failures" when Jörn's
local SSH agent offers too many keys before password authentication. This form
asks for the password twice: first for `xlogin.uni-augsburg.de`, then for
`licca-li-01.rz.uni-augsburg.de`.

LICCA ED25519 host key fingerprint, observed in Augsburg HPC docs and confirmed
by Jörn on 2026-06-04:

```text
SHA256:ZKi0w4Cc24qHbrLQKXX/ifYQ92208g2yhCVPHvgxWz8
```

Other current facts:

- Rust builds: use `CARGO_TARGET_DIR=/hpc/gpfs2/scratch/u/stoehljo/cargo-target`
  unless Jörn says the storage layout changed.
- Python: LICCA had system `python3` 3.12.3 on 2026-06-04 and no `uv`; use
  `python3 script.py` for standard-library helper scripts.
- Normal LICCA checkout: `"$HOME/msc-math"`.
- Jörn's host checkout was observed at `~/workspaces/msc-math/`; inside the
  devcontainer the checkout path is `/workspaces/msc-math/`. Match retrieval
  destinations to where the `scp` command is actually running.

Example retrieval command when running `scp` inside the devcontainer:

```bash
scp \
  -o IdentitiesOnly=yes \
  -o PubkeyAuthentication=no \
  -o PreferredAuthentications=password,keyboard-interactive \
  -o 'ProxyCommand=ssh -o IdentitiesOnly=yes -o PubkeyAuthentication=no -o PreferredAuthentications=password,keyboard-interactive -W %h:%p stoehljo@xlogin.uni-augsburg.de' \
  stoehljo@licca-li-01.rz.uni-augsburg.de:~/artifact.tgz \
  /workspaces/msc-math/.worktrees/<worktree>/
```

## Slurm Handoffs

Prefer self-contained `*.slurm.sh` scripts for reviewed jobs. Put resource
choices, seed ranges, output paths, resume rules, and exact binary commands in
the script.

Path and output rules for Slurm scripts:

- Do not derive run-local output directories from `BASH_SOURCE[0]` inside a
  Slurm job unless you have checked that LICCA is executing the repo copy.
  LICCA may execute a spool copy under `/var/spool/slurmd/...`, so
  `BASH_SOURCE[0]` can point outside the checkout.
- For scripts submitted from a specific repo directory, prefer anchoring
  run-local outputs to `SLURM_SUBMIT_DIR` and fail clearly if it is not the
  expected directory. For scripts with a fixed checkout convention, `cd
  "$HOME/msc-math"` and repo-relative paths are also acceptable.
- Print the resolved repo root, output directory, resource request, and git
  commit at job start so path mistakes are visible in the bounded log tail.

Before recommending `sbatch` for a nontrivial job:

- classify the execution model: serial process, Rayon/CPU-parallel process,
  Slurm array of serial tasks, Slurm array of CPU-parallel tasks, or mostly
  I/O-bound merge/table/postprocessing;
- inspect existing Slurm scripts before reusing them, especially
  `--cpus-per-task`, `--time`, memory, array shape, output paths, resume
  behavior, and side effects against the current job size;
- avoid `#SBATCH --mem=0` unless the job truly needs full-node memory and that
  cost is justified. On LICCA, `--mem=0` can request all node memory and make
  small `test` partition smoke jobs pend with `QOSGrpMemLimit`. Use bounded
  memory for smoke submissions, and prefer bounded production defaults unless
  current evidence says otherwise;
- give a short resource BOTEC: work units, parallelism, ETA/range, timeout or
  failed-run cost, Jörn active waiting time if relevant, next bounded check,
  and cancel/resubmit condition;
- treat `64` CPUs as a first LICCA candidate for Rayon or otherwise
  CPU-parallel production jobs, not as a rule. Use `1` CPU for serial tasks and
  I/O-bound merge/postprocess jobs unless the job evidence says otherwise;
- remember `sbatch --test-only` checks scheduling, not script-body correctness;
  `bash -n` checks syntax, not Slurm execution semantics;
- if a Slurm edit changes cwd/path resolution, environment assumptions, output
  topology, or resource behavior, run/provide a tiny Slurm smoke path before
  production, or label production as un-smoked and higher risk.
- if the production path depends on cache hits, resume files, deduplication, or
  a base-cache argument, smoke both the cold path and the hot/resume path before
  treating the LICCA run as production-ready.

For `sbatch --export`, remember that commas separate exported variables. Do not
inline comma-containing values such as `--export=ALL,FOO=a,b`; Slurm can parse
that as `FOO=a` plus another export item. Export the value in the shell first,
then pass the variable name, for example:

```bash
export DATASCIENCE_PRODUCERS='random,random-product'
sbatch --export=ALL,DATASCIENCE_PRODUCERS ...
```

For concurrent data-producing jobs, write per-task output files and merge after
validation. Avoid concurrent writes to one JSONL or cache file. If resume
matters, preserve partial outputs and check the producer's resume semantics
before changing output paths, seeds, or array constants.

After submission, carry the ETA forward. When reading `sacct`, logs, row
counts, or silence, compare observed elapsed/progress with the estimate. If an
already-running job used weaker resources than current guidance suggests,
compare continuing versus cancel/resubmit including lost elapsed work, expected
queue delay, core-hours, and active Jörn waiting time.

## Ask Or Stop

Ask Jörn before proceeding when the decision depends on:

- whether Jörn is actively waiting/babysitting and that can change the resource
  or cancel/resubmit decision;
- deleting, moving, or re-downloading large generated data whose value is not
  clear from local evidence;
- changed LICCA environment, storage layout, checkout path, or authentication;
- job topology that remains unclear after inspecting the relevant code and
  Slurm script;
- a resource or cleanup decision that can waste LICCA queue time, redownload
  large data, or require active Jörn babysitting, with no bounded check to
  reduce uncertainty first.

Stop and escalate when task-specific stop conditions from the handoff or
experiment docs fire, or when scheduler/runtime evidence makes the planned
production run implausible.

When a Slurm job fails in the first few seconds with tiny `MaxRSS`, treat it as
script setup, path, environment, or scheduler configuration evidence before
reasoning about the actual compute workload. Inspect the bounded log tail
before resubmitting.

## Monitoring Snippets

Use bounded commands such as:

```bash
sacct -j <jobid> --format=JobID,JobName%24,Partition,State,Elapsed,AllocCPUS,CPUTime,MaxRSS,ExitCode
tail -n 80 logs/*.out
wc -l path/to/shards/*.jsonl
ps -o pid,etime,pcpu,pmem,rss,cmd -p <pid>
```

If `squeue` shows `TIME` such as `0:16`, that is Slurm seconds/minutes
formatting, not necessarily minutes. Confirm elapsed time with `sacct` before
reporting wall time.
