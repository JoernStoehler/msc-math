# Sys-Datascience LICCA Status

No sys-datascience LICCA job is selected. The retained scripts are dormant
infrastructure, not execution handoffs and not evidence.

| Surface | Status | Reactivation gate |
| --- | --- | --- |
| `produce/licca-datascience-produce.slurm.sh` | dormant run-local producer infrastructure | a fresh C3 decision names the producer axis, claim, budget, outputs, stopping rule, and review gate |
| `prepare/licca-datascience-prepare.slurm.sh` | dormant run-local prepare infrastructure | a selected C3 producer run or a distinct reproduction task supplies a reviewed input/output contract |
| `produce/licca-refresh-random.slurm.sh` and `produce/promote-licca-random-refresh.py` | dormant standalone retained-producer refresh/promotion helpers | a selected reproduction/refresh task states comparison and promotion rules, or C3 explicitly selects a new retained sample |
| `licca-build-dataset.slurm.sh` | dormant legacy in-place table rebuild helper | an explicit retained-table reproduction/schema-refresh task; prefer the run-local prepare script for new work |
| `produce/plans/` | dormant producer designs | status and gates are recorded in `produce/plans/README.md` |

The obsolete post-feature-rebuild handoff was deleted because its branch,
worktree, archive names, and direct submission sequence were stale.

When a gate above is satisfied, prepare a new job-specific handoff using the
repo LICCA conventions. Recheck the current commit, hydrated inputs, execution
model, resource estimate, output destination, validation, retrieval, and
promotion boundary. The presence of a Slurm script does not authorize a job.
