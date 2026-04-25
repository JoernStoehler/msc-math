# Sys-Landscape Research Note

## Scope
- Capture the current status of the `sys-landscape` experimentation branch in a single research-facing note.
- Keep the experimental artifact and code locations in `experiments/` as canonical storage.
- Track evidence quality, modeling implications, decisions, and execution order without introducing runbook-level process instructions.
- Keep tool-by-tool verdicts in `research/sys-landscape-toolbox-audit.md`; this note stays as the topic-level narrative and decision surface.

## Current State
- Topic evidence is now documented as one note in `research/sys-landscape.md`.
- Stable experimental surface spans five families: random generic polytopes, random Lagrangian products, fixed-F gradient ascent (general and products), fixed-F continuation, and continuation variants that reach `F=11`.
- `sys-landscape` evidence is bounded at present and currently has no `sys>1` discovery beyond the known isolated pentagon-pentagon rotation geometry.
- Core dataset contract uses a normalized package shape with `poly_id` and `state_id` split identities, plus lineage metadata (`root_group_id`, `lineage_id`, `parent_state_id`) when present.
- Current local modeling surface includes the legacy experiment families
  `random-sample`, `random-product-sample`, `rejection-calibration`,
  `rotated-regular-products`, `gradient-ascent-general`,
  `gradient-ascent-products`, and `variable-f-ascent`, plus the maintained
  datascience pipeline under `experiments/sys-landscape/datascience/`.

## Evidence And Interpretation
- Random generic polytopes (`random-sample`): 70 rows, max `sys=0.739`, no `sys>1`.
- Random Lagrangian products (`random-product-sample`): 100 rows, max `sys=0.794`, no `sys>1`.
- Fixed-F general ascent (`gradient-ascent-general`): 10 seeds, max `sys=0.9030`, no `sys>1`; all seeds used escape logic.
- Fixed-F Lagrangian product ascent (`gradient-ascent-products`): 12 seeds, max `sys=0.8727`, no `sys>1`.
- Variable-`F` continuation (`variable-f-ascent`): 90 trials incl. random-seed RQ2 and 10 RQ1 local maxima starts; gains from `F=10` to `F=11` are common but still below `1`.
- Regular polygon probes (`rotated-regular-products`, `pentagon-rotation-formula`): confirmed `sys>1` at `theta=18 deg`; no further tested regular-family violation is known.
- Interpretation stays with hostile-landscape framing: bounded search and continuation improve local endpoints but have not surfaced a second transferable `sys>1` regime.

## Decisions
- Keep normalized, joinable dataset normalization (`poly_id` + `state_id`) as the stable ingestion contract. Avoid one-table redesign that would break provenance and force rewrites.
- Preserve ordered dual-vertex geometry under `poly_id` and store exact rational payload (`dual_vertices_rational`) with each `poly_id`.
- Keep HKO-specific packets outside the principal hostile-landscape surface and treat `hko-local-maximum/cut-and-ascent/cut-and-ascent.jsonl` as optional control.
- Use experiment-specific seed streams consistently in `gradient-ascent-*`, including `--fresh`, per-seed RNG streams, and `--no-db-update` under parallel execution.
- Do not run large local reruns (for example `N=1000`) as defaults; scale on LICCA with wall-time overrides.
- Carry LICCA tripwire (`#SBATCH --time=00:00:01`) as hard guardrail and require CLI `--time` for production.

- Rejected approaches:
  - One-off wide table rebuild from committed JSONL.
  - Raw witness-permutation transfer as a global cache across unrelated seeds.
  - Default continuation strategy that increases `F` before fixed-F ascent (`add+F+1` first).
  - Treating reduced-model outcomes as final claims before exact witness verification.

- Open constraints:
  - Keep local topic work in `experiments/` while algorithmic.
  - Use additive schema changes where feasible in the maintained datascience
    table stage under `experiments/sys-landscape/datascience/tables/`.
  - Treat direct continuation replacement as a measured comparison, not a blind rewrite.

## History
- A single-trajectory baseline established in `gradient-ascent-general`, `gradient-ascent-products`, and `variable-f-ascent`, plus calibrated random baselines and rotated-product checks.
- The dominant unresolved experimental objective is structured witness-guided continuation: replace naive random-F increase with controlled local-structure re-use.
- Exact-evaluated rows now support reduced-model prefilter comparison, with attention focused on exact-call reduction, hit rate, and safe rejection (`U_A(K)<1`).
- Feature-pattern experiments now include geometric and orbit trajectory blocks over canonicalized packets and show that cheap random-regime signal does not transfer cleanly to endpoints.
- A direct continuation successor has been identified as witness-guided `F+1` continuation, benchmarked against existing `variable-f-ascent` and HKO-style `cut-and-ascent` baselines.

## Next Steps
- Implement witness-oracle instrumentation in fixed-F ascent paths; persist top-`m` and within-gap witness sets plus diagnostics with each exact evaluation.
- Benchmark before new continuation methods: use `variable-f-ascent` endpoints plus existing exact rows and compare minimizer-only, top-`m`, within-gap, parent-cache, and hybrid witness sets.
- Add reduced-model descent loop as first warm replacement candidate, with exact-check fallback on candidate acceptance.
- Add witness-guided `F+1` continuation with lifted parent witnesses in child runs; do not restart witness state from scratch.
- Respect blocking conditions: instrumentation must be explicit before comparisons are considered complete, and continuation changes are blocked until fixed-seed benchmarks are running.
- Stop conditions: stop witness-guided continuation if exact-call reduction or safe pruning does not improve on the fixed benchmark bank; retain exact-failing outcomes as negative evidence.
