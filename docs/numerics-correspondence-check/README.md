# Numerics production correspondence: bounded check

18 September 2026. Assignment N from `docs/technical-closure-plan/README.md`.

## Outcome

The named production correspondence gate **passes all four tests**. No solver,
formal precision-source, or test drift was found between base revision
`60f4fb0b386f43de57592886b674d47d40cf0760` and the interrupted production
checkout. This establishes a finite regression bridge from the selected
readable development routes to production. It does not establish equivalence
for every input, nor independently prove interval arithmetic or enumeration
soundness.

The chapter needs small attribution repairs, not a new solver project on the
basis of this check. `proposed-attribution.patch` makes only these repairs
against the exact source identified in `source-sha256.txt`; it has not been
applied to the interrupted production checkout. No original authoring was
replaced and no chapter build was performed.

## Receipt

Working directory: `/workspaces/msc-math/.worktrees/numerics-correspondence-check`.
Command:

```sh
cargo test -p exp-dev-quadratic-program --release --test selected_route_correspondence
```

Rust `1.94.0 (4a4ef493e 2026-03-02)`, Cargo
`1.94.0 (85eff7c80 2026-01-15)`. Fresh release build: 50.82 seconds.
Test execution: 1.25 seconds. Exit code 0. Results:

```text
running 4 tests
production_general_action_window_matches_complete_exact_control ... ok
readable_and_production_product_certificates_match ... ok
exact_window_derivatives_match_the_retained_f64_route_on_f10 ... ok
readable_and_production_general_bounds_match ... ok
4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

The test was inspected before execution: it constructs fixed fixtures in
memory and compares results, with no retained-evidence producer or external
file writes. Cargo writes ordinary ignored build products in this worktree's
`target/`. No full producer, broader test suite or secret contents were read.

## Precisely what was tested

Owner: `experiments/dev-quadratic-program/tests/selected_route_correspondence.rs`.

| Test | Actual comparison | Not established |
| --- | --- | --- |
| General bounds | Simplex and hypercube: identical lower and upper binary64 bounds from readable selected general route and production `general_capacity` | All-input code equivalence; independent correctness of shared enumeration/arithmetic |
| General action window | Retained F5 seed 99540836: exact capacity and ordered `(sigma, action)` lists at action multiple 101/100 against complete exact control | Physical placement of every witness; all-input window completeness |
| Derivatives | Retained F10 case: same word list as admissible retained route at factor 1.001; gradient differences below 1e-10 | A derivative theorem at arbitrary degeneracies; rigorous interval bounds on gradients |
| Products | Triangle product, triangle-square and HKO pentagon: exact capacity equality and same closure-family winning words | All physical orbit branches; every product; algebraic-input pentagon theorem (these fixtures are exact stored binary64 inputs) |

## Claim → mathematics → implementation/evidence map

Paths below are relative to the repository. The selected copies live in
`experiments/dev-quadratic-program/src/selected_route/{general,product}.rs`;
production lives in `crates/symplectic/src/algorithms/capacity_4d/`.

| Retained chapter claim | Mathematical support | Production and selected-route link | Evidence boundary |
| --- | --- | --- | --- |
| Exact target is supplied dyadic geometry | Exact rational halfspace geometry; public input contract | `mod.rs`: `capacity_from_dual_vertices`, `exact_binary64_polytope_geometry`; checked fixture helper invokes facet, finite, norm and geometry checks | This gate exercises valid fixtures, not every rejection case. No pre-rounding coordinate uncertainty is enclosed |
| Inverse-defect solution enclosure | `formal/hk2017-qp-precision.tex`: `lem:kkt-verified-inverse-defect`, normwise and batched defect lemmas, `cor:kkt-beta-q-from-xi` | `general.rs`: `certify_direct_solution_normwise_profiled`, `certify_direct_solution_batched_profiled`, `decision_from_certified_norms_with_inverse_norm`; selected general copy | General bounds fixture comparisons; old 249-system audit belongs to selected development route, not a new production-wide audit |
| Indeterminate predicates resolve exactly | Same precision contract; exact KKT linear system and strict positivity feasibility | `general.rs` guarded `Option` falls back to `exact_decision`; `kkt/rational_solver.rs::solve_kkt_exact` handles singular systems with exact linear algebra and Fourier–Motzkin positivity | Exact witness gives stationarity, closure, normalization, positive weights and Q; it is not by itself a fixed-word maximum or a physical orbit |
| Safe curvature pruning and inheritance | Precision lemmas `lem:kkt-certified-curvature-direction`, `lem:kkt-cyclic-obstruction-inheritance` | `general.rs`: `certify_curvature`, obstruction cache and `contains_certified_subword` | Floating inertia proposes only; rejection requires a certified tangent direction. Code inspection, not fresh curvature proof |
| Complete family yields scalar enclosure | HK formula, minimum-support argument and adjacency pruning in `formal/hk2017-qp-core.tex`; precision bounds | `mod.rs::general_words`, `general_capacity`; `general.rs` takes maxima of accepted Q lower/upper endpoints then outward reciprocal | Complete outer support coverage is logically separate from exact witness correctness. Passing finite fixture comparisons does not prove the outer theorem |
| Exact product capacity and tied sparse winners | `formal/product-qp-six-facet-reduction.tex`, `thm:product-qp-six-facet-maximizer` and scope remark | `product.rs::solve_product_closure_capacity_hybrid`, `mod.rs::product_capacity`, `product_qp_minimizers`; selected product copy | Three-fixture gate checks exact values and candidate-family word lists; no assertion that every geometric branch has sparse support |
| Scalar convenience value has requested tolerance | Public interval contract | `mod.rs::capacity_value` and `capacity_value_from_bounds` | Separate from the historical harness's printed central value; named correspondence gate does not directly test this wrapper |

Source inspection confirms the production general route bypasses **all** its
floating certificates if gradual underflow is unavailable, including short-word
rejection and curvature pruning. Otherwise indeterminate staged enclosures
fall back to exact rational decisions. Final aggregation maximizes the accepted
Q bounds before reciprocal endpoint conversion. These are the relevant code
paths; this bounded check does not replace the pinned compiler/dependency and
matrix-evaluation-order assumptions of the precision proof.

## Concrete chapter repairs

Interrupted source: `thesis/candidate/11-numerics.tex`; `main.tex` line 65
currently inputs this prepared chapter.

1. Its opening attributes both HKO and current pentagon proofs to Sage. The
   selected pentagon chapter is analytic; preserve Sage attribution for HKO.
2. Name `capacity_4d` instead of implying every ordinary Rust capacity or
   derivative entry point satisfies this contract. Legacy orbit routes retain
   weaker/different guarantees.
3. Attribute the 249-system, 88/1271-word and 88-product audits to selected
   development implementations. The new four-test gate supplies finite
   correspondence evidence, not a blanket transfer of every historical result.
4. Replace “reported midpoint” at 2.31e-14 with “printed central value”. The
   source RESULTS and harness README identify `best_action` as a convenient
   central value and identify separate endpoints as the certificate; midpoint
   identity was not established.
5. Replace the final universal Sage-family sentence with accurate separate
   descriptions of the HKO certificate and analytic pentagon proof.

These are encoded in the proposal patch. The integrator should also include
one concise sentence reporting the four-test production comparison at its
actual scope, rather than expanding the old audit populations to production.

## Remaining limits and stop decision

No newly observed mismatch warrants solver changes or expensive producers.
Still required at final integration: apply/reconcile the attribution patch,
verify final chapter references and assumptions, and maintain the distinction
between capacity bounds and derived systolic ratios using f64 volume. Current
production certificates do not retroactively certify historical DS target
values with missing evaluator receipts.

This check is deliberately not a complete formal verification, a new audit of
the 249-system/88-product evidence, a reproduction of Sage certificates, or a
mathematical acceptance review of the reduction theorems. Those unperformed
activities must not be inferred from the passing gate.
