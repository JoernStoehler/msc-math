# Repository capabilities

This is a compact answer to “what can the repository currently rely on?”
Every row is a navigation claim. Confirm its scope and current status at the
named sources before using it in a proof, thesis claim, or experiment decision.
The table is intentionally selective. Absence of a row is not evidence that
the repository lacks a capability; use the relevant domain README and search
the source tree before making a negative claim.

| Capability | Current scope | Confirm at |
| --- | --- | --- |
| Exact algebraic arithmetic and dense linear algebra | one statically chosen real algebraic field; exact order, row reduction, rank, kernel, solve, and negative-definiteness checks; not a general CAS | `crates/algebraic-numbers/README.md`, source and tests |
| Convex-polytope geometry | current four-dimensional point, polar, incidence, face, and volume workflows; not symplectic capacity | `crates/euclidean-polytopes/README.md`, source and tests |
| Symplectic capacity and orbit search | current four-dimensional HK2017/QP, billiard/product, KKT, orbit, derivative, random, and persistence workflows; f64 behavior is not exact proof | `crates/symplectic/README.md`, `crates/symplectic/src/`, tests |
| Capacity/orbit validation | selected fixtures, capacity axioms, minimum sets, and orbit recovery; not exhaustive correctness for all polytopes | `experiments/verification/README.md` |
| HKO local-maximum certificate | exact theorem-local feasible-section certificate and supporting formal route for the stated quotient-local result; broader HKO maximality remains conjectural | `experiments/hko-local-maximum/README.md`, theorem packet, HKO formal notes |
| First-order local behavior | generic smooth-branch exposition is available; a full arbitrary-polytope evaluator remains substantially heavier and must not be replaced by the generic statement | `formal/sys-first-order-local-behavior.md`, first-order thesis sources |
| QP/KKT numerical diagnostics | structured f64-versus-reference evidence and caveated solver diagnostics; not universal certification of every public wrapper | `experiments/dev-quadratic-program/README.md`, numerics packets |
| Hostile `sys` search data science | retained producer tables and reviewed method packets support bounded search/usefulness claims; not density or impossibility theorems | `experiments/sys-datascience/README.md` |
| Rotated regular-polygon product result | exact pentagon open-domain executable proof plus endpoint/symmetry mathematical close and empirical exposition assets | `experiments/regular-products/README.md`, active thesis section |
| Combinatorial-cell exploration | bounded evidence about boundary behavior, widths, convexity failures, crossings, and gradient discontinuity; no global convexity or monotonicity claim | `experiments/combinatorial-cells/README.md` |
| Crosspolytope computation | retained four-dimensional candidate and bounded search evidence; global optimality depends on the stated search boundary | `experiments/crosspolytope/README.md` |
| Visualization producers | scripts and browser-rendered assets for selected four-dimensional polytope/orbit views; visualization is explanatory, not theorem evidence | `experiments/visualization/README.md`, topic-local asset sources |
| Formal proof-development document | integrated mathematical statements, derivations, audits, and proof routes; inclusion does not imply thesis use or stakeholder acceptance | `formal/README.md`, `formal/main.tex` |
| Thesis PDF and native assets | active LaTeX builds and thesis-native figures; build success alone is not mathematical or submission readiness | `thesis/README.md`, `thesis/main.tex`, build checks |

## Maintenance rule

Keep this table small. Add a row only for a repeated, high-level project
question whose reconstruction is expensive or commonly overclaimed. When a row
needs detailed status, commands, metrics, or exclusions, put those details in
the named local source rather than expanding this file.
