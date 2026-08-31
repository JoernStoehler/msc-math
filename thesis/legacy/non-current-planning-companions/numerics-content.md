# Numerics Content Notes

Status: maintenance companion for `thesis/11-numerics.tex`. The active section
is the publication candidate; this file records source ownership, claim
boundaries, review state, and reopen conditions rather than duplicating its
mathematics or metrics.

## Section role and source hierarchy

The section explains the certified scalar-capacity routes for exact dyadic
polytopes supplied in binary64 coordinates and separates them from the exact
SageMath packets used by theorem-facing algebraic claims. Read it together
with the QP formulation in `thesis/04-haim-kislev-quadratic-program.tex`.

Current owners, in descending order for their respective claims:

- Public contract and production implementation:
  `crates/symplectic/README.md` and
  `crates/symplectic/src/algorithms/capacity_4d/`, especially `mod.rs`,
  `geometry.rs`, `general.rs`, and `product.rs`.
- Fixed-word enclosure and certified curvature arguments:
  `formal/hk2017-qp-precision.tex`, especially
  `lem:kkt-verified-inverse-defect`,
  `lem:kkt-batched-defect-enclosure`,
  `lem:kkt-normwise-defect-enclosure`,
  `lem:kkt-certified-curvature-direction`, and
  `lem:kkt-cyclic-obstruction-inheritance`. That file also retains explicitly
  incomplete legacy perturbation proposals; its status remark and the named
  direct lemmas must be read before reuse.
- General-route verification, numerical comparison, and performance evidence:
  `experiments/dev-quadratic-program/tools/general_algorithm_ablation/README.md`
  and `RESULTS.md`, backed by the packet's named binaries and harness.
- Product-route intermediate audit, exact comparisons, adversarial cases, and
  performance evidence:
  `experiments/dev-quadratic-program/tools/product_closure_route/README.md`,
  `RESULTS.md`, and the retained producer output `sample5.jsonl`.
- Six-facet product completeness:
  `formal/product-qp-six-facet-reduction.tex` and the active QP theorem. The
  product audit and retained random-product check are falsification evidence,
  not the proof.
- Proof-facing HKO and rotated-pentagon arithmetic:
  `experiments/hko-local-maximum/theorem/README.md` with
  `thesis/07-hko-local-maximum-exact-certificate.tex`, and
  `experiments/regular-products/pentagon-rotation-formula-proof/README.md`
  with `thesis/09-rotated-regular-polygons-exact-certificate.tex`.

The two route packets were written before production migration and still
contain historical “pre-production” or “not yet migrated” language. Use them
as the owners of their retained measurements and audit interpretation, but use
the crate README and `capacity_4d` sources for current implementation and API
status. `experiments/dev-quadratic-program/README.md` records this ownership
split explicitly.

## Claim boundaries that must survive editing

- The input coordinates are interpreted as exact binary64 dyadic rationals.
  The contract does not recover algebraic or measured source coordinates or
  propagate uncertainty introduced before the API call.
- The general route returns outward capacity bounds over its complete retained
  word family; the product route returns an exact rational capacity and sparse
  exact winners in the closure-vertex family. Neither route classifies every
  minimizing or near-minimizing geometric branch.
- Facet count, primal/dual norm bounds, exact geometry validation, and the
  general candidate-count limit are applicability conditions. Rejection
  outside them is not a theorem about the excluded polytope.
- A certified fixed-word KKT enclosure does not by itself establish a
  fixed-word maximum or candidate-family completeness. The general and product
  route arguments supply those separate obligations.
- Audit metrics belong to the two retained `RESULTS.md` files and their
  producer artifacts. Thesis metrics may summarize them with a precise cohort
  and comparison contract, but this companion must not become a second metric
  ledger.
- Finite exact-versus-binary64 comparisons test the implementation. Numerical
  soundness rests on the proved outward bounds, the inspected arithmetic
  contract, complete-candidate arguments, and exact fallback policy. The Sage
  packets remain the authority for algebraic theorem claims.

## Review state and reopen conditions

`docs/project-status.md` currently treats the QP/numerics material as an
integrated draft with certified production algorithms and numerical evidence;
Kai/expert convention review and integrated reader review remain appropriate.
No separate final Jörn/Kai acceptance of `thesis/11-numerics.tex` is recorded
here.

Reopen this companion when the public capacity contract, input limits,
candidate coverage, fallback semantics, arithmetic environment, or retained
evidence packet changes. A dependency, compiler, target, matrix-evaluation, or
underflow-contract change also reopens the floating-point certificate checks
named in the active section and formal precision note. Reassess propagated
summary wording through `thesis/central-claim-control.md`; do not add an
independent abstract or conclusion inventory here.
