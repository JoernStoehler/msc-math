# Four-dimensional QP route review packet

Status: retained development evidence before production migration. This branch
does not change Main and does not propose a merge.

Start a review here, then read:

1. `RESULTS.md` for the bounded claims and current measurements.
2. `formal/hk2017-qp-precision.tex` for the numerical and curvature lemmas.
3. `main.rs` for the executable correspondence. Its major sections match the
   algorithm below.
4. `run.sh` for the canonical build-and-run command.

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
   curvature. A proved obstruction is cached.
4. A survivor reuses the same factorization for its solution and approximate
   inverse. Ordinary binary64 matrix products are enlarged by analytic
   dot-product, KKT-assembly, subtraction, and underflow terms. A verified
   inverse-defect argument then encloses every solution component.
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
| batched residual and defect enclosure | `lem:kkt-batched-defect-enclosure` | `certify_direct_solution_batched_profiled` |
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

Exact Lagrangian products require a separate outer branch. Their exact zero
blocks select the smaller billiard word family, but also make many augmented
KKT matrices singular. The general inverse-based route is retained as a
correctness control on products, not as their performance route.

The existing null-space-capable billiard solver is the retained performance
control and agrees with exact aggregation on the three fixtures. This packet
does not upgrade its legacy f64 predicates to a general certificate. A trusted
production product branch must exact-resolve its output-relevant candidates or
add a product-specific certified wrapper without discarding its singular-system
handling.

Near-products are not silently rounded into products. That would change the
input and needs an explicit preprocessing contract.

## What is and is not certified

The batched route, scalar outward control, curvature rejection, exact fallback,
and final capacity interval are theorem-backed under their stated arithmetic
and candidate-stream assumptions.

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
  and total timings for the empirical control and selected route.

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

Optional older falsification searches remain available as
`--adversarial-predicate-search` and `--beta-boundary-search`. They target the
discarded heuristic, so they are not part of the normal reviewer path.
