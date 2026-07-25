# Four-dimensional QP route review packet

Status: retained ablation, verification, numerics, and performance evidence for
the selected general production route.

Start a review here, then read:

1. `RESULTS.md` for the bounded claims and current measurements.
2. `formal/hk2017-qp-precision.tex` for the numerical and curvature lemmas.
3. `../../src/selected_route/general.rs` for the instrumentable selected copy
   and `crates/symplectic/src/algorithms/capacity_4d/general.rs` for production.
4. `main.rs` for alternatives, counters, and executable correspondence.
5. `run.sh` for the canonical build-and-run command.

## Intended route

The general route is restricted to four dimensions, at most sixteen facets,
and validated binary64 dual vertices. Validation rejects a dual or primal
vertex whose infinity norm lies outside `[1e-3, 1e3]`; it also retains the
existing finiteness, origin-interior, extremality, and degeneracy checks.

The retained experiment obtains its candidate stream from the exact-binary64
HK transition graph and processes words from shortest to longest. Completeness
of that stream is an imported HK/enumeration obligation, not reproved here.

1. Length-three and length-four words use an outward determinant rejection
   followed, when necessary, by an exact rational constraint solve.
2. A later word is rejected if a cached positive-curvature obstruction embeds
   in it preserving cyclic order.
3. Otherwise the augmented KKT matrix is factored once with pivoted
   Bunch--Kaufman `LBL^T`. Its inertia and a numerical null-space vector may
   propose a positive-curvature direction. The proposal is rejected unless an
   outward calculation proves that an exact direction in `ker(C)` has positive
   curvature. A proved obstruction is cached without first constructing the
   unused solution and inverse.
4. A survivor reuses the factorization for its solution and approximate
   inverse. A cheap normwise calculation first encloses the residual and
   inverse defect. If that is inconclusive, a tighter entrywise calculation is
   tried. Both enlarge ordinary binary64 products by proved dot-product,
   KKT-assembly, subtraction, and underflow terms.
5. The beta signs and `Q` are decided from that enclosure. The identity
   `Q = -xi/2` avoids a second quadratic-form error analysis. Any
   indeterminate case is solved in exact rational arithmetic.
6. Accepted `Q` intervals are maximized, then reciprocated outward, yielding a
   certified interval for the capacity. The printed `best_action` is only a
   convenient central value; `best_action_lower` and `best_action_upper` carry
   the certificate.

The proof-to-code labels are:

| Claim | Formal label | Main implementation |
| --- | --- | --- |
| verified inverse and solution radius | `lem:kkt-verified-inverse-defect` | `decision_from_certified_norms` |
| cheap normwise residual and defect enclosure | `lem:kkt-normwise-defect-enclosure` | `certify_direct_solution_normwise_profiled` |
| batched residual and defect enclosure | `lem:kkt-batched-defect-enclosure` | `certify_direct_solution_batched_profiled` |
| normwise, then batched, then exact staging | `rem:kkt-staged-defect-enclosure` | `certify_direct_solution_hybrid_profiled` |
| beta, `Q = -xi/2`, and action intervals | `cor:kkt-beta-q-from-xi` | `decision_from_certified_norms`, `record_case_capacity_interval` |
| certified tangent curvature | `lem:kkt-certified-curvature-direction` | `certify_curvature` |
| cyclic obstruction inheritance | `lem:kkt-cyclic-obstruction-inheritance` | `contains_certified_subword` |

The numerical theorem assumes the arithmetic contract recorded in
`rem:kkt-batched-binary64-contract`: pinned `nalgebra 0.33.3` for certified
products, `nalgebra 0.35.0` for Bunch--Kaufman factorization,
`matrixmultiply 0.3.10`, finite values, runtime-checked gradual underflow, and
the current matrix dimensions. A dependency, target, compiler-arithmetic, or
word-length change requires that contract to be reviewed again.

## Product route

Exact Lagrangian products use the separate KKT-free closure-vertex route in
`../product_closure_route/`. The six-facet reduction proves that a scalar
capacity maximizer exists with at most three facets from each factor; direct
closure-vertex enumeration therefore avoids the singular augmented KKT systems
that made the old product branch difficult to certify.

The old billiard/KKT solver remains a performance and behavioral control. It is
not the selected production design. The closure-vertex route returns the exact
binary64-rational scalar capacity and sparse exact maximizing witnesses; it
does not classify every minimizing or near-minimizing branch.

Near-products are not silently rounded into products. That would change the
input and needs an explicit preprocessing contract.

## What is and is not certified

Both floating-point enclosure stages, the scalar outward control, curvature
rejection, exact fallback, and final capacity interval are theorem-backed
under their stated arithmetic and candidate-stream assumptions.

The following are controls only:

- the residual/inverse-norm heuristic called `EmpiricalThenExact`;
- the legacy `q_error_bound` and static beta/Q thresholds;
- unchecked direct solves and the symmetric-eigen route;
- cutoff and LU variants retained for ablation.

Exact fallback on only the heuristic's indeterminate cases cannot repair an
incorrect determinate heuristic decision. The heuristic is therefore never the
recommended route, even though it is faster on the current cohort.

The packet checks that pruning preserves the exact capacity and tied minimizing
cyclic classes on small complete streams. It does not make the production API
return an exact complete orbit set. Production migration must keep the scalar
capacity interval contract separate from any exact orbit-set contract.

## Reproduce

The canonical command builds first, preventing stale-binary evidence:

```bash
experiments/dev-quadratic-program/tools/general_algorithm_ablation/run.sh \
  /tmp/qp-current-route-evidence
```

It writes:

- `verification.txt`: complete-route exact agreement, capacity containment,
  product controls, and invalid-input controls;
- `numerics.txt`: exact binary64 predicate and enclosure audits on generic,
  scaled, near-singular, product, and edge cohorts;
- `profile.txt`: interleaved candidate-processing ablations and separate
  product timings;
- `end-to-end.txt`: validation, exact transition/cycle construction, route,
  and total timings for the empirical control, tighter entrywise route, and
  selected staged route.

Focused tests and the formal build:

```bash
cargo test -p exp-dev-quadratic-program --release \
  --bin qp-general-algorithm-ablation
(cd formal && latexmk)
```

Use `cargo run`, rather than invoking `target/release` after source changes, for
one packet:

```bash
cargo run -p exp-dev-quadratic-program --release \
  --bin qp-general-algorithm-ablation -- --verification-packet
```

The bounded rich-output architecture spike is:

```bash
cargo run -p exp-dev-quadratic-program --release \
  --bin qp-general-algorithm-ablation -- --rich-output-spike
```

It compares exact minimizers, an exact `11/10` capacity window, and all exact
admissible records after certified pruning against a complete exact solve of
the retained F5--F7 streams. It also measures the exact-resolution overhead;
it is not part of the normal scalar performance packet.

Optional older falsification searches remain available as
`--adversarial-predicate-search` and `--beta-boundary-search`. They target the
discarded heuristic, so they are not part of the normal reviewer path.
