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
| Do f64/exact/fallback/certified values agree within the intended numerical contract? | `experiments/numerics/` for numerical behavior, or `experiments/verification/` when the claim is reusable correctness evidence |
| Does a route satisfy capacity axioms, literature values, minimum-set semantics, or stable regression properties? | `experiments/verification/` or cheap crate tests |
| Which QP route should be the ordinary library API? | `experiments/dev-quadratic-program/` while the decision is unsettled, then `crates/symplectic/` docs/API |
| Which theorem or thesis slice needs a particular route? | the owning topic folder or thesis companion |

## Current Comparison Claims

Treat this table as a pointer list, not as source truth. Refresh each claim from
the named evidence home before using it in thesis prose or API decisions.

| Claim | Current pointer |
| --- | --- |
| Pruned HK2017 enumeration is the ordinary general-polytope path; unpruned enumeration is retained as a comparison/reference path. | `experiments/MAP.md` labels `QP/enumerate/pruned` and `QP/enumerate/unpruned`; current performance targets are in `experiments/performance/`. |
| Billiard/Lagrangian-product enumeration is a specialized sigma enumeration feeding the shared QP/KKT solve and aggregation layer. | `experiments/MAP.md` label `QP/enumerate/billiard`; product/topic use lives in `experiments/regular-products/` and HKO-specific use in `experiments/hko-local-maximum/`. |
| Exact/certified QP results are the comparison target for exact flow-graph capacity tests. | `crates/symplectic/src/algorithms/flow_graph/README.md` and flow-graph tests. |
| Historical A-axis pruning ablation output existed, but it is no longer an active evidence producer. | Git history for the retired `experiments/verification/algorithm-comparison/` folder; formal note `formal/search-pruning-correctness.tex` records the mathematical pruning discussion. |

## Adding A New Comparison

Do not recreate a comparison crate by default. Add or update a target in the
lens home that owns the evidence, then record the comparison reasoning here.

Example shape:

```text
Claim: `QP/enumerate/pruned` is faster than `QP/enumerate/unpruned` on the
current random-fixture profile target.
Evidence: `experiments/performance/README.md`, target `hk2017-pruned-f64`
or a future target that explicitly compares both routes.
Use: supports ordinary-library default choice; not a correctness claim.
```
