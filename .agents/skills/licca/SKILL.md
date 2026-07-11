---
name: licca
description: Use before writing any command Jörn should run on, to, or from LICCA, including login, transfer, Slurm, resource, monitoring, and retrieval commands.
---

# LICCA

Agents do not receive unrestricted LICCA access. Prepare simple, typo-resistant
commands or reviewed scripts; Jörn skims and runs them. Read
`references/current-facts.md` for connection and environment facts and
`references/slurm-sharp-edges.md` when preparing Slurm work.

## Handoffs

- Separate and label local, login-node, submission, monitoring, and retrieval
  commands. Do not combine cleanup or state-changing steps unless requested.
- Give bounded commands, not interactive shells, pagers, `watch`, `tail -f`,
  unbounded loops, or commands that wait for completion.
- Prefer `sbatch --parsable` assigned to a descriptive job-id variable.
- Use login nodes only for light edits, transfers, submission, and monitoring.
- LICCA pulls repository changes; retrieve generated artifacts and commit them
  locally. Do not ask Jörn to push from LICCA.
- Maintain the one checkout at `~/msc-math`. If Git reports dirty blockers,
  resolve only those exact paths. Do not broad-clean data, make another clone or
  worktree, or switch to tarball transfer unless Jörn asks.

Prefer self-contained reviewed Slurm scripts that state resources, seeds,
outputs, resume rules, and the exact command. Print the resolved repository,
output directory, resources, and commit at job start. Inspect existing scripts
and current code before choosing resources. `sbatch --test-only` checks
scheduling and `bash -n` checks shell syntax; neither tests Slurm execution.
Smoke semantic changes to paths, environment, output topology, or resources.
When cache/resume behavior matters, check both cold and resumed paths.

Concurrent producers write separate shards and merge only after validation.
Preserve partial outputs and verify resume semantics before changing paths,
seeds, or array constants.

Ask Jörn before destructive data handling, changed connection/storage facts, or
an elevated decision whose topology remains unclear after local inspection.
Ask whether he is actively waiting whenever that would change resource or
cancel/resubmit choices. Stop when task-specific stop conditions fire.

A job that fails within seconds with tiny `MaxRSS` is evidence about setup,
paths, environment, or scheduler configuration before it is evidence about the
compute workload. Inspect a bounded log tail before resubmitting.
