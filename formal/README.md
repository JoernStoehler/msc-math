# Formal proof development

This directory develops mathematical statements and proof routes. It is not
the publication surface. `main.tex` builds the proof-development document and
shows which `.tex` files are currently included; inclusion does not itself
mean that Jörn or Kai accepted the statement or that the active thesis uses it.

## Main groups

| Group | Principal files |
| --- | --- |
| Symplectic/polytope foundations | `symplectic-polytope-geometry.tex`, `combinatorial-boundary-regularity.tex` |
| HK2017/QP conventions and core | `hk2017-qp-conventions.tex`, `hk2017-qp-core.tex`, `hk2017-qp-precision.tex`, `ehz-kkt-system.tex` |
| Capacity and orbit algorithms | `capacity-algorithms.tex`, `billiard-capacity-algorithm.tex`, `reeb-orbit-recovery.tex`, `finite-reeb-orbit-verification.tex` |
| Flow graph | `flow-graph-real-algorithm.tex`, `flow-graph-capacity.tex`, `flow-graph-ch2021-comparison.tex`, `flow-graph-proof-risk.tex` |
| First-order behavior | `capacity-derivatives.tex`, `capacity-smoothness-classification.tex`, `capacity-boundary-subdifferential.tex`, `active-orbit-facet-coverage.tex`, `sys-first-order-local-behavior.md` |
| HKO local maximality | `hko-symmetry-gradient-structure.tex`, `hko-feasible-section-upper-branches.tex`, `hko-local-maximality-conditions.tex` |
| Lagrangian products | `lagrangian-product-rotation-symmetry.tex`, `product-two-bounce-class.tex`, `product-triangle-bounce-classification.tex`, `pentagon-rotation-capacity.tex` |
| Numerical and representation audits | `f64-prefilter-bound.tex`, `f64-orientation-sign-filters.tex`, `f64-near-singular-vertices.tex`, `near-redundant-facet-removal-bounds.tex`, `rational-integer-scaling.tex`, `admissibility-precision-audit.tex` |
| Search and canonization | `random-polytope-boundedness.tex`, `search-pruning-correctness.tex`, `generic-coordinate-canonization.tex` |

## Status interpretation

Read the statement and its local status markers. Distinguish:

- derivation or conjectural route;
- agent-checked proof candidate;
- exact certificate plus mathematical implication;
- active thesis theorem;
- superseded comparison or risk note.

Do not infer project-wide absence from one formal file or from this README.
Related proof material may be theorem-local under `experiments/`, active under
`thesis/`, or imported from a paper.

Known examples requiring care:

- `pentagon-rotation-capacity.tex` is retained for calculations but is not the
  current theorem-facing proof owner; start at
  `experiments/regular-products/README.md`.
- Flow-graph comparison/risk notes do not replace the project's current
  simple-minimizer and exact-word routes.
- Generic smooth-branch first-order results do not settle the full
  arbitrary-polytope behavior described in
  `sys-first-order-local-behavior.md`.

## Build

```bash
cd formal
latexmk
```

The build checks LaTeX integration, not mathematical correctness or thesis
inclusion.
