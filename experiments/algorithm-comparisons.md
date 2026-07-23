# Algorithm Comparison

Status: README-only routing and reasoning note for capacity-algorithm
comparisons. This directory owns no commands, generated artifacts, or thesis
evidence.

The old `dev-algorithm-comparison` crate, benchmark/ablation binaries,
generated JSONL, figures, and profiling artifacts were retired from the active
tree. Git history is the archive for those historical artifacts.

Algorithm comparison is not a separate evidence home. Record only the reasoning
that relates algorithm units to each other here; route fresh evidence to the
appropriate lens home. If a future comparison becomes a thesis-section asset,
move or summarize the relevant reasoning in the owning `thesis/*-content.md`
companion instead of making thesis prose depend on this note.

## Evidence Routing

| Comparison question | Evidence home |
| --- | --- |
| Which route is faster, uses less memory, or scales better? | `experiments/performance/` |
| Do f64/exact/fallback/certified values agree within the intended numerical contract? | the owning development packet while the numerical question is method-coupled; QP/KKT uses `experiments/dev-quadratic-program/numerics-audit/`, and reusable correctness evidence uses `experiments/verification/` |
| Does a route satisfy capacity axioms, literature values, minimum-set semantics, or stable regression properties? | `experiments/verification/` or cheap crate tests |
| Which QP route should be the ordinary library API? | `experiments/dev-quadratic-program/` while the decision is unsettled, then `crates/symplectic/` docs/API |
| Which theorem or thesis slice needs a particular route? | the owning topic folder or thesis companion |

## Comparison Ledger

Treat this table as a pointer list, not as source truth. Do not record benchmark
numbers, local timing summaries, or other easy-staling measurements here when a
reader can rerun a command or read a current output in the evidence home. Before
using a row in thesis prose or API decisions, rerun or read the current evidence
in the named home and check whether the reasoning still applies.

| Comparison | Current reasoning breadcrumb | Fresh evidence pointer | Re-check before use |
| --- | --- | --- | --- |
| `QP/enumerate/pruned` vs `QP/enumerate/unpruned` | Pruned enumeration is the ordinary general-polytope path; unpruned enumeration remains useful as a reference and ablation path. | `experiments/performance/` for speed/scaling; `experiments/verification/` or crate tests for agreement/regression; labels in `experiments/README.md`. | Check the current performance target or add one that runs both routes on the same input selector. For correctness language, check agreement/minimum-set evidence separately from speed. |
| `QP/enumerate/billiard` vs general QP enumeration | Billiard/Lagrangian-product enumeration is a specialized sigma source feeding the shared QP/KKT solve and aggregation layer, not a replacement for the general-polytope route. | `experiments/regular-products/` for product use; `experiments/hko-local-maximum/` for HKO-local use; label `QP/enumerate/billiard` in `experiments/README.md`. | Check whether the comparison question is about product structure, theorem-local HKO use, or general capacity routing; route evidence accordingly. |
| `QP/capacity/certified` vs `FG/capacity/exact` | Certified/exact QP values are useful comparison targets for exact flow-graph capacity tests, but the algorithms have different route structure. | `crates/symplectic/src/algorithms/flow_graph/README.md`, flow-graph tests, QP/KKT numerics in `experiments/dev-quadratic-program/numerics-audit/`, or reusable correctness targets in `experiments/verification/`. | Check whether the claim is numerical agreement, a reusable correctness property, or API-route reasoning; those belong in different lens homes. |
| f64 route vs fallback/certified route for QP capacity | The ordinary fast route may use f64 candidate generation/filtering; exact fallback or certified aggregation changes ambiguity handling and output trust, not necessarily every internal step. | `experiments/dev-quadratic-program/README.md` for route/API design; `experiments/dev-quadratic-program/numerics-audit/` for QP/KKT f64-vs-exact behavior; `experiments/verification/` for reusable correctness checks. | Name the path precisely (`QP/capacity/f64`, `QP/capacity/fallback`, `QP/capacity/certified`, or reserved `QP/capacity/exact`) before comparing. |
| QP/HK2017 vs FG/CH2021 capacity routes | This is a family-level route/API comparison, not one benchmark. QP and FG evidence may differ by supported inputs, exactness route, performance profile, and theorem relevance. | `experiments/README.md` algorithm-unit inventory; `experiments/dev-quadratic-program/`; `experiments/dev-flow-graph/`; lens homes for numerics/performance/verification. | First decide which property is being compared: speed, exactness, supported input class, regression stability, or thesis-section relevance. Then use the matching evidence home. |
| Historical A-axis pruning ablation | Historical output existed, but it is no longer an active evidence producer. | Git history for the retired `experiments/verification/algorithm-comparison/` folder; `formal/search-pruning-correctness.tex` for the mathematical pruning discussion. | Use only as context for reconstructing a question. Fresh evidence should be produced in the owning development packet, `experiments/performance/`, or `experiments/verification/` depending on the question. |

## Adding A New Comparison

Do not recreate a comparison crate by default. Add or update a target in the
lens home that owns the evidence, then record the comparison reasoning here.
Prefer pointers to commands, target names, and output locations over copied
measurements; those pointers stay useful after the measurements change.

Example shape:

```text
Claim: `QP/enumerate/pruned` is faster than `QP/enumerate/unpruned` on the
current random-fixture profile target.
Evidence: `experiments/performance/README.md`, target `hk2017-pruned-f64`
or a future target that explicitly compares both routes.
Use: supports ordinary-library default choice; not a correctness claim.
```
