# Generator target-quotient distance

Status: bounded exact research packet, 2026-07-15. The mathematical note is
agent-written and not reviewed by Jörn.

## Scientific question and disposition

Does the full symplectic Gram matrix of volume-one, analytic-center-normalized
facet covectors give a distance that removes exactly translation, positive
scale, linear symplectic maps, and facet relabeling?

Yes, after two restrictions:

1. The metric is local to one fixed irredundant facet count $F$. Unequal
   facet counts are not compared.
2. Exact permutation minimization is factorial. This prototype certifies only
   $F\le8$, by visiting every permutation, and returns no answer on timeout.

The proof is in
[`formal/symplectic-gram-quotient.md`](../../../../formal/symplectic-gram-quotient.md).
It proves that the full Gram matrix separates every spanning labeled
configuration up to a unique linear symplectic map; no genericity condition is
needed. The finite-permutation Frobenius quotient is therefore a genuine
metric, not merely a pseudometric, on each fixed-$F$ quotient.

## Exact object and input contract

Coordinates are `(q1,q2,p1,p2)`. Covectors are represented as four-entry
columns in the mathematics and four-entry tuples in Python. For normalized
duals $a_1,\ldots,a_F$, the packet computes

\[
\Omega_{ij}=a_i^TJa_j,
\qquad
d_\Omega(K,L)=F^{-1}\min_{P\in S_F}
\|\Omega(K)-P\Omega(L)P^T\|_F.
\]

There are two accepted entry routes.

- `validate_normalized_configuration` accepts exact `Fraction` covectors whose
  inequalities are already analytic-center translated and volume-one scaled.
  It checks distinct rows, rank four, exact positive spanning, exact vertices,
  and that every row supports an affine three-dimensional facet.
- `normalize_parallelotope` accepts eight exact inequalities and a declared
  center. It requires four exact opposite normalized facet pairs. This
  certifies that the declared center is the symmetry center and hence analytic
  center. If the pair representatives are the rows of $U$, it derives exact
  volume as $16/|\det U|$, requires a rational exact fourth root, applies the
  volume-one normalization, and runs the generic validation above.

The second route is intentionally narrow: it makes translation, scale,
symplectic, orthogonal, and general-linear controls exact without introducing a
numerical analytic-center or volume solver. A future caller with exact geometry
from `euclidean-polytopes` may use the first route after its owner certifies the
center and volume normalization. Constructing `NormalizedConfiguration`
directly bypasses validation and is an internal/caller-contract operation.

The result records:

- `status`: `exact` or `timeout`;
- the number evaluated and the required total $F!$;
- exact minimum squared Frobenius objective and exact squared metric distance;
- the distance as a rational or symbolic square root, plus a display-only f64;
- number of minimizing permutations and whether the minimizer is nonunique;
- exact second-distinct objective and a declared near-symmetry flag.

No stochastic permutation search is used. A timeout result has no distance.

## Controls

`test_packet.py` and the retained smoke report cover the current known set of
discriminating controls:

- labeled identity and an explicit facet permutation give exact zero;
- `diag(2,1/3,1/2,3)` is symplectic but not orthogonal and gives exact zero;
- a translated positive scaling by three is normalized back to the identical
  covectors, with exact pre-normalization volume `81`;
- a rational determinant-one `SO(4)` product of Givens rotations is orthogonal
  but not symplectic and gives squared distance `2664/1625`;
- the controlled map `diag(2,1,1,8)` has determinant `16`; volume-one
  normalization does not remove its nonsymplectic anisotropy, and squared
  distance is `5/2`;
- unequal facet count, a duplicate/redundant row, a degenerate lower-rank
  presentation, malformed opposite pairs, and $F>8$ fail closed;
- a rational perturbation of a symmetric fixture has an exact near-optimal
  second permutation and is flagged `near_symmetry`;
- a zero-second timeout reports `timeout`, `exact: false`, and no distance;
- all ordered triangle inequalities on the three-fixture set `base`,
  `so4_outside_u2`, and `nonsymplectic_gl` pass by exact rational comparison.

The controls check this implementation. The completeness and metric claims
come from the proof, not the finite fixtures.

## Reproduction and measured cost

From the repository root:

```bash
uv run --script experiments/sys-datascience/methods/generator-target-quotient-distance/test_packet.py

/usr/bin/time -f 'wall_seconds=%e max_rss_kb=%M' \
  uv run --script \
  experiments/sys-datascience/methods/generator-target-quotient-distance/quotient_distance.py \
  --out experiments/sys-datascience/methods/generator-target-quotient-distance/artifacts/smoke-report.json

python3 -m compileall -q \
  experiments/sys-datascience/methods/generator-target-quotient-distance
```

On the development container, the nine focused tests took `72.797 s`. The
retained report took `56.82 s` wall time and `21,376 KiB` maximum RSS. Its
individual exact $F=8$ comparisons took about `6.48-7.83 s`, each recording
all `40,320` permutations. These are smoke measurements, not a stable
benchmark. The supported bound is $F\le8$; the implementation deliberately
does not imply that repeated cloud-scale pair distances at $F=8$ are cheap.

## Failure modes and cost envelope

- Different facet counts: error `unequal_facet_counts` before search.
- More than eight facets: error `facet_count_exceeds_exact_bound`.
- Timeout: explicit incomplete result with no objective or distance.
- Redundant or degenerate inequalities: validation error.
- General exact volume whose fourth root is not rational: the local
  parallelotope normalizer rejects it. This is an implementation boundary, not
  a mathematical restriction of the metric.
- Symmetry: several exact minimizers are valid and reported. `near_symmetry`
  means the exact gap from the best objective to the second *distinct*
  objective is at most `1e-6` times the declared Gram-norm scale; it is a search
  diagnostic, not a geometric theorem.
- Factorial scaling: exhaustive search is transparent but unsuitable for
  production facet counts or large pair-distance matrices. No branch-and-bound
  scalability claim is made.

## Allowed and prohibited claims

Allowed: for two validated analytic-center/volume-one exact configurations with
the same $F\le8$, an `exact` result is the stated quotient metric evaluated
over every facet permutation. The formal theorem applies to all fixed-$F$
spanning real configurations independently of this computational bound.

Prohibited:

- a distance between different facet-count or combinatorial strata;
- an exact result after timeout or from a partial/stochastic search;
- polynomial-time or production-scale matching;
- using the numerical controls as proof of separation, invariance, or triangle
  inequality;
- claiming topology, components, holes, support coverage, or agreement with a
  Hausdorff/support-function topology on the union of polytope strata;
- treating the retained parallelotope fixtures as representative generator
  data or as evidence about `sys`.

The retained artifact is
[`artifacts/smoke-report.json`](artifacts/smoke-report.json). It contains exact
control outcomes, search counts and timings, timeout/near-symmetry state, the
producer hash, Git provenance, and the reproduction command. There are no
plots because this packet has no display question.
