# Sys-Datascience LICCA Status

No sys-datascience LICCA job is selected. The retained scripts are dormant
infrastructure, not execution handoffs and not evidence.

| Surface | Status | Reactivation gate |
| --- | --- | --- |
| `experiments/polytope-datasets/licca-datascience-produce.slurm.sh` | dormant run-local producer infrastructure | a new research decision names the producer axis, claim, budget, outputs, stopping rule, and review gate |
| `experiments/polytope-invariant-table/licca-datascience-prepare.slurm.sh` | dormant run-local prepare infrastructure | a selected producer run or a distinct reproduction task supplies a reviewed input/output contract |
| `experiments/polytope-datasets/licca-refresh-random.slurm.sh` and `experiments/polytope-datasets/promote-licca-random-refresh.py` | dormant standalone retained-producer refresh/promotion helpers | a selected reproduction/refresh task states comparison and promotion rules, or new research explicitly selects a retained sample |
| `experiments/polytope-invariant-table/licca-build-retained-table.slurm.sh` | dormant legacy in-place table rebuild helper | an explicit retained-table reproduction/schema-refresh task; prefer the run-local table-construction script for new work |
| `experiments/polytope-datasets/plans/` | dormant producer designs | status and gates are recorded in `experiments/polytope-datasets/plans/README.md` |

The older `licca-post-feature-rebuild.md` handoff is preserved from `main` as a
historical plan. Its branch, worktree, archive names, and direct submission
sequence are not current instructions. Do not run it without satisfying a
reactivation gate above and preparing a new job-specific handoff.

When a gate above is satisfied, prepare a new job-specific handoff using the
repo LICCA conventions. Recheck the current commit, materialized inputs, execution
model, resource estimate, output destination, validation, retrieval, and
promotion boundary. The presence of a Slurm script does not authorize a job.
