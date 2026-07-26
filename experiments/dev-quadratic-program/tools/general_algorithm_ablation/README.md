# Four-dimensional QP route review packet

Status: retained ablation, verification, numerics, and performance evidence for
the selected general production route.

Start a review here, then read:

1. `RESULTS.md` for the bounded claims and current measurements.
2. `formal/hk2017-qp-precision.tex` for the numerical and curvature lemmas.
3. `../../src/selected_route/general.rs` for the instrumentable selected copy
   and `crates/symplectic/src/algorithms/capacity_4d/general.rs` for production.
4. The named executable matching the algorithm under review. `src/lib.rs` is
   the short index for the shared harness:
   - `src/harness/general_route.rs`: selected algorithm and certified numerics;
   - `src/harness/numerical_audits.rs` and `exact_agreement.rs`: exact checks;
   - `src/harness/benchmarks.rs`: matched timing and ablations;
   - `src/harness/adversarial.rs`: optional falsification searches;
   - `src/harness/commands.rs`: cohorts and executable entry points; and
   - `src/harness/shared.rs` and `tests.rs`: common data and focused regressions.
5. `run.sh` for the canonical build-and-run command.

## Directly runnable algorithms

Each stable general-route variant has a no-argument executable. The executable
prints its exact algorithm identity, numerical status, curvature status,
production status, cohort, work counts, phase timings, and total timing.
There are no factorization, guard, or cutoff flags to interpret.

This directory is a dedicated experiment crate. Its one library compiles the
shared machinery once; the small binaries fix one algorithm or evidence packet
each. Keeping the crate separate prevents this large harness from entering
unrelated `exp-dev-quadratic-program` library builds. See `RESULTS.md` for the
measured comparison with per-binary local modules and a parent-library module.

| Executable | Numerical decision | Curvature handling |
| --- | --- | --- |
| `qp-general-legacy-symmetric-eigen` | known-unsound legacy thresholds | none |
| `qp-general-unchecked-lu` | unchecked | none |
| `qp-general-empirical-inverse` | heuristic inverse radius | none |
| `qp-general-verified-scalar-lu` | certified scalar enclosure | none |
| `qp-general-verified-scalar-lblt` | certified scalar enclosure | certified pruning |
| `qp-general-verified-batched` | certified entrywise enclosure | certified pruning |
| `qp-general-verified-normwise` | certified normwise enclosure | certified pruning |
| `qp-general-verified-hybrid` | normwise, entrywise, then exact | certified pruning |
| `qp-general-pruned-empirical` | heuristic inverse radius | certified pruning |

Only `qp-general-verified-hybrid` is the selected production algorithm.
`qp-general-algorithm-comparison` runs all nine on the matched long-word
cohort; it interleaves the seven routes that share the common route machinery
and separately takes seven-round medians for the two legacy baselines. The
separate no-argument
`qp-general-selected-verification`, `qp-general-selected-numerics`, and
`qp-general-end-to-end` executables provide the retained correctness,
intermediate-error, and whole-route packets.

The algorithms outside the general batch comparison are also direct:

- `qp-product-closure-profile` profiles the selected KKT-free product
  algorithm on the standard fixtures;
- `qp-product-closure-route` performs its slower exact and adversarial audit;
- `qp-product-legacy-billiard-kkt` profiles the old product billiard/KKT
  control; and
- `qp-exact-sigma-comparison` compares the fraction-free exact one-word path
  with the generic exact rank/kernel solver on the same retained word.

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
- `algorithms.txt`: interleaved candidate-processing ablations for every
  stable general-route variant;
- `end-to-end.txt`: validation, exact transition/cycle construction, route,
  and total timings for the empirical control, tighter entrywise route, and
  selected staged route.

Focused tests and the formal build:

```bash
cargo test -p exp-qp-general-algorithms --release --lib
(cd formal && latexmk)
```

Use `cargo run`, rather than invoking `target/release` after source changes:

```bash
cargo run -p exp-qp-general-algorithms --release \
  --bin qp-general-verified-hybrid

cargo run -p exp-qp-general-algorithms --release \
  --bin qp-general-selected-verification
```

The older `qp-general-algorithm-ablation` flag interface remains only for
exploratory falsification searches that do not correspond to a stable
algorithm, including `--adversarial-predicate-search` and
`--beta-boundary-search`.
