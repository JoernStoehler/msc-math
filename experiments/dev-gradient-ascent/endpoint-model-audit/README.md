# Endpoint branch-model audit

Status: completed three-case derivative/KKT decomposition for a systematic
affine-model failure. It localizes the tested failures but does not classify a
local maximum or present a finished optimizer.

## Question and downstream use

The highest endpoint selected from the retained 128-start four-anchor tuning
run has the recorded evaluator field

```text
sys(a0) = 0.9999624776406894.
```

The continuation diagnostic proposed moves that its affine branch model said
would improve that field, but full evaluator recomputation decreased it in
both signs of the proposed direction. This packet asks:

1. What exactly was modeled and tested?
2. Which conclusions follow from the retained data?
3. Is the failure caused by a change of polytope combinatorics, a change of
   minimizing or admissible branch, an incorrect derivative, or ordinary
   finite-distance error?
4. What is the least expensive next diagnostic that distinguishes those
   explanations?

The answer determines whether to repair the local model, try a nonsmooth
gradient-sampling step, retain the endpoint as a numerical local-maximum
candidate, or stop spending optimizer effort on this state.

## Mathematical objects

For labelled dual vertices

```text
a = (a_0, ..., a_{F-1}) in (R^4)^F,
```

the primal polytope is

```text
K_a = {x in R^4 : <a_i, x> <= 1 for every i}.
```

The intended mathematical objective has the branch-envelope form

```text
sys(a) = c(a)^2 / (2 V(a))
       = min_{sigma admissible at a} s_sigma(a),
```

where `c(a)` is the mathematical EHZ capacity, `V(a)` is the Euclidean
four-volume, and

```text
s_sigma(a) = action_sigma(a)^2 / (2 V(a)).
```

The retained evaluations do not certify that complete mathematical envelope.
They use the legacy `MinimaSafe` orbit-search aggregation, whose f64 action
windows are diagnostics rather than proved error bounds and whose
`AdmissibleF64` rows are not rechecked. Write

```text
hat(c)(a)   = minimum action returned by that implemented search,
hat(sys)(a) = hat(c)(a)^2 / (2 hat(V)(a)),
```

where `hat(V)` is the configured f64 reconstructed volume. The JSON field is
named `sys`, but the empirical statements in this packet concern `hat(sys)`.
Equating it with the mathematical capacity needs both certified named-orbit
resolution and a separately justified candidate-family coverage argument.

For one smooth KKT branch, the code uses

```text
D s_sigma
  = (action_sigma / V) D action_sigma
    - (s_sigma / V) D V.
```

`D action_sigma` is an envelope derivative evaluated from one KKT payload
`(beta, q, mu, sigma)` at the base point. The tested model does not
differentiate the KKT solution map.
`D V` comes from facet volumes and centroids computed using the reconstructed
primal vertex--facet incidence matrix.

There are three distinct sources of nonsmoothness, and they must not be
conflated:

- **Fixed-incidence boundary:** the set of primal vertices or their incident
  facets changes.
- **Branch-domain boundary:** a transition predicate, `beta > 0`, or another
  KKT admissibility predicate changes.
- **Minimum switch:** two smooth admissible functions `s_sigma` exchange which
  one realizes `sys`, even though geometry and admissibility remain unchanged.

The earlier phrase “geometry cell” meant an open **fixed-incidence region**:
an open subset of `(R^4)^F` on which every four-facet intersection used as a
primal vertex, and its incident-facet set, remains constant. More explicitly,
for every four-set `I`, solve

```text
A_I(a) x_I(a) = 1.
```

Away from `det A_I = 0`, the signs of

```text
<a_j, x_I(a)> - 1
```

decide whether `x_I` is a vertex and whether an additional facet contains it.
A connected region on which all relevant signs are strict and fixed has
constant incidence. It need not be a polyhedral cell in the coordinates `a`,
so this packet avoids the word “cell.”

The transition graph and KKT branch domains add further boundaries; they are
not part of the definition of a fixed-incidence region.

## The tested finite branch model

At `a0`, the continuation code:

1. fully reconstructs geometry, volume, and the minimizing orbit;
2. runs a branch search retaining admissible branches with
   `action_sigma <= min_action * (1 + w)` for `w = 0.1` and `w = 1.0`;
3. adds transition-blocked candidate words from its extension enumeration;
4. assigns each represented branch an affine model

   ```text
   m_sigma(delta) = gap_sigma + <g_sigma, delta>,
   ```

   where the implemented gap is the branch ratio at `a0` minus
   `hat(sys)(a0)`, and `g_sigma` is the implemented envelope gradient;
5. solves

   ```text
   maximize_delta  min_sigma m_sigma(delta)
   subject to      delta perpendicular to the 15 symmetry directions,
                   ||delta||_2 <= r ||a0||_2;
   ```

6. fully recomputes the evaluator's `sys` field at `a0 + delta` and accepts
   only a measured gain greater than `1e-12`.

In parallel, it takes the projected, normalized gradient of the evaluator's
one displayed winning orbit at `a0` and proposes that direction at the same
five radii. This is a separate single-branch model; it does not solve the
max--min problem across near-active branches.

For `F = 10`, the ambient dimension is 40 and the locally constructed
symmetry-transverse slice has dimension 25. Thus one stopped state first
receives 15 proposals: two action-window max--min directions and one
single-winning-branch-gradient direction at each of five radii. If all 15
proposals fail, the diagnostic fully evaluates both signs of each of the 25
slice-basis vectors at normalized radius `1e-5`.

This continuation is not identical to the optimizer that produced the
endpoint. The producer used a four-anchor branch history; the endpoint
diagnostic discards that history and rebuilds two models only at the current
state.

## Population and retained observations

The population is the eight highest outcomes of one selected algorithm in the
128-start **tuning** dataset. It is outcome-selected discovery evidence, not a
held-out optimizer comparison.

- Five of eight endpoints had one validated improving model move.
- The gains were `1.57114391e-6` to `4.60807882e-5`.
- None crossed `sys = 1`.
- The top endpoint had no improving max--min model, winning-branch-gradient,
  or signed-basis move.
- The top-eight run used 278 full evaluator calls and 20.09 seconds.
- The top endpoint alone used 66 evaluations; the mirrored-direction rerun
  used 76.

For the top endpoint, the `w = 0.1` positive model direction gave:

| normalized radius | affine delta for the target winner | measured delta sys | measured slope |
| ---: | ---: | ---: | ---: |
| `1e-3` | `+2.727e-4` | `-8.340e-4` | `-0.1591` |
| `3e-4` | `+8.193e-5` | `-2.513e-4` | `-0.1598` |
| `1e-4` | `+2.743e-5` | `-8.396e-5` | `-0.1602` |
| `3e-5` | `+8.296e-6` | `-2.523e-5` | `-0.1604` |
| `1e-5` | `+2.799e-6` | `-8.293e-6` | `-0.1582` |

The opposite direction had measured slopes from `-0.3913` to `-0.3888`.
Thus the mismatch per unit distance stayed roughly constant over the tested
range. This is evidence against an ordinary second-order Taylor remainder for
the complete implemented affine model along this sequence of directions; it
is not a limit as the radius tends to zero.

The implemented evaluator's displayed base winning word was

```text
[0, 2, 6, 7, 5, 4, 8, 1].
```

The positive target winner was

```text
[0, 2, 4, 6, 7, 5, 8, 1],
```

and the negative target winner was

```text
[0, 2, 7, 5, 6, 8, 1].
```

Both target winners were already present in the base candidate set. Therefore
missing the target word is not the immediate explanation. It remains possible
that the target-winning KKT solution changes its smooth solution branch or
admissibility regime. The base full evaluation returned two orbit rows from
its zero-action-gap minima-safe search, so the displayed base winner must not
be read as evidence for a unique active branch. Those two rows may merely have
overlapping numerical action intervals; they do not establish an exact tie or
a complete minimizing set.

The displayed base winner has recorded `beta_margin =
0.0323338906286978`, so that one payload is not itself close to the
implemented beta-zero cutoff. The corresponding margins and KKT ranks for the
second returned row and the two one-sided target words were not retained.

The positive target had a different recorded incidence matrix only at radii
`1e-3` and `3e-4`; it had the same recorded incidence at `1e-4`, `3e-5`, and
`1e-5`. The negative targets retained the same incidence at all five radii.
A changed-incidence explanation therefore does not account for the persistent
small-radius mismatch by itself.

## The same failure across the endpoint cohort

The top endpoint is not the only failure. Across the eight selected endpoints:

- 52 of 80 action-window max--min proposals decreased recomputed `hat(sys)`;
- in all 52 losses, the target winner was represented in the base or extension
  set and its recorded affine prediction was positive;
- 46 of the losses retained the same recorded incidence;
- 40 had determinate geometry at both points and unchanged incidence; and
- all 40 current-winning-branch-gradient proposals decreased.

| rank | max--min gains | max--min losses | losses with unchanged clean geometry | winning-gradient losses |
| ---: | ---: | ---: | ---: | ---: |
| 1 | 0/10 | 10/10 | 0/10 | 5/5 |
| 2 | 10/10 | 0/10 | 0/0 | 5/5 |
| 3 | 2/10 | 8/10 | 6/8 | 5/5 |
| 4 | 0/10 | 10/10 | 10/10 | 5/5 |
| 5 | 6/10 | 4/10 | 4/4 | 5/5 |
| 6 | 8/10 | 2/10 | 2/2 | 5/5 |
| 7 | 2/10 | 8/10 | 8/8 | 5/5 |
| 8 | 0/10 | 10/10 | 10/10 | 5/5 |

Rank 2 is an internal positive control for the max--min model. Ranks 4 and 8
are clean failure controls that do not involve any geometry counter. This
cohort evidence makes a systematic derivative, KKT-identity/admissibility, or
bookkeeping problem more plausible than an explanation special to the top
endpoint.

## What the geometry counter actually means

The base evaluation recorded:

```text
vertex_indeterminate_count                 = 1
bounded_near_singular_vertex_count         = 0
ambiguous_vertex_incidence_count           = 0
facet_intersection_indeterminate_count     = 0
omega_indeterminate_count                  = 0
```

The counter `vertex_indeterminate_count` is the sum of:

1. four-facet systems with `abs(det) <= 1e-12`; and
2. accepted vertices with an additional facet within `1e-9`.

Here the second count is zero. The one nearly singular system uses facets
`[0, 1, 2, 8]`. A numerical SVD audit estimates its intersection at infinity:

```text
abs(det)                    = 7.59e-18
||x||_infinity              = 4.54e9
maximum inequality violation = 1.06e10
```

It therefore fails both the `||x||_infinity <= 1e3` and polytope-inequality
tests used for a bounded near-singular candidate. The Rust route did not add
this four-set as a possible primal vertex and did not mark any facet
intersection uncertain.

Consequently, the current data do **not** establish that `a0` lies on a
fixed-incidence boundary. The previous “geometry-wall” interpretation was too
strong. The near-singular four-set may be a harmless relation among dual
facets whose formal intersection is far outside `K_a`.

Reproduce the explanatory audit:

```bash
uv run --script \
  experiments/dev-gradient-ascent/endpoint-model-audit/inspect_four_facet_systems.py \
  experiments/dev-gradient-ascent/ascent-continuation/artifacts/top8-tuning-endpoints-one-step-20260729/input.json \
  /tmp/top-endpoint-four-facet-systems.json
```

The retained output is
`artifacts/top-endpoint-20260729/four-facet-systems.json`.

## Hypotheses before the directional decomposition

These are provisional explanations, not mutually exclusive statistical
models. Their ordering is the current priority for falsification after seeing
the cohort-wide failures.

1. **The envelope derivative or its implementation/bookkeeping is wrong for a
   systematic class of branches.** Clean unchanged-incidence failures occur
   across the cohort, while rank 2 supplies a positive control.
2. **The target word is represented but its KKT solution identity,
   rank, or admissibility regime changes.**
   The same cyclic word can fail to denote one smooth admissible function
   through the base point if a KKT system is singular or a beta component
   crosses zero.
3. **The f64 volume derivative or reconstructed volume is wrong on some
   incidence patterns.** Stable recorded incidence does not prove that the
   reconstruction or facet-volume derivative is correct.
4. **The implemented evaluator and branch model compare different orbit
   quantities.** A normalization, gap, canonical-word, or selected-solution
   defect could reproduce the observation.
5. **A genuine fixed-incidence boundary supplies different one-sided
   derivatives.** This remains possible through an unrecorded or
   misclassified predicate, but the current counter is not evidence for it.
6. **Ordinary nonlinear remainder or f64 noise.** The nearly constant,
   order-one slope mismatch down to normalized radius `1e-5` makes this a poor
   leading explanation, though the current radius range is finite.

Those data did not distinguish action-envelope error from volume or
bookkeeping error. The next section records the experiment that made that
distinction.

## Directional decomposition result

The retained audit compares:

- the top endpoint's failed `gap-window-0.1` proposal;
- rank 4 as a clean unchanged-geometry failure; and
- rank 2 as a successful control.

It follows each saved proposal direction from normalized radius `1e-5` down
to `1e-8`. The last radius is only an infinitesimal-limit diagnostic, not a
proposed optimization step.

At radius `1e-5`, the two failures perturb the named KKT matrix by 218 and 67
times its smallest base eigenvalue magnitude. In both cases that eigenvalue
changes sign along one side of the tested interval. Their analytical
named-action derivatives predict the wrong sign at finite distance. By
contrast, the successful control has perturbation/gap ratio 1.53, does not
cross zero, and retains the correct sign.

The action derivative formula converges under smaller finite differences. At
radius `1e-8`, relative central-difference errors are:

| role | named action | named branch ratio |
| --- | ---: | ---: |
| top failure | `9.92e-3` | `1.88e-2` |
| rank-2 success | `4.16e-7` | `7.55e-7` |
| rank-4 clean failure | `7.83e-5` | `1.36e-4` |

The f64 volume derivative agrees with finite differences throughout. Across
all 39 audited points, f64 and exact-arithmetic reconstruction agree on
incidence, facet intersections, and omega signs; the largest relative volume
difference is below `1e-15`.

Thus the tested failures are finite-distance nonlinearities of ill-conditioned
named KKT branches. They are not explained by a sign error in the
infinitesimal derivative, volume, f64 geometry reconstruction, or a missing
target word. A Euclidean radius alone is therefore not a sufficient trust
scale for this affine branch model.

## Two different nearby-gradient diagnostics

The earlier proposal mixed two methods which must remain separate.

### Clarke-style gradient sampling

To test approximate stationarity, apply standard gradient sampling to
`-hat(sys)`, not to an arbitrary set of near-minimizing affine branches:
This interpretation requires evidence that the implemented evaluator agrees
locally with a Lipschitz branch envelope; gradient-sampling theory does not
justify heuristic discontinuities introduced by an incomplete or unstable
candidate search.

1. Construct an orthonormal matrix `U` whose 25 columns span the complement of
   the translation, scaling, and `Sp(4)` tangent directions at `a0`.
2. At deterministic seeded nearby points

   ```text
   a_j = a0 + epsilon ||a0||_2 U z_j,
   ```

   retain points where the implemented objective is differentiable enough to
   identify the actually realized winning branch and a trusted gradient.
3. Form sampled gradients

   ```text
   h_j = -U^T grad(hat(sys))(a_j)
   ```

   and find the minimum-norm vector in their convex hull. Its negative is the
   standard sampled descent direction for `-hat(sys)`, hence an ascent
   proposal for `hat(sys)`.
4. At exact ties, include every justified active branch gradient. Do not add
   merely near-minimizing inactive branches without a radius-dependent
   reachability argument: they can create a false stationary result.
5. Fully evaluate the proposal. A small sampled convex-hull norm is only
   finite-sample approximate-stationarity evidence; it is not a local-maximum
   certificate.

Random points here sample nearby realized smooth regimes. They are not being
used in the weaker hope that a random direction directly hits a thin improving
cone. A finite sample can still miss an adjacent regime.

### Transported affine branch models

If finite affine transport is separately validated, nearby branch evaluations
can instead define a semilocal predictor:

1. At a nearby point `a_j`, compute a branch value and trusted gradient for a
   justified named branch.
2. Express its gradient in the common labelled ambient coordinates and
   project it to the base slice:

   ```text
   gbar_{j,sigma} = U^T g_{j,sigma}.
   ```

3. Transport both value and gradient to the base:

   ```text
   b_{j,sigma} + <gbar_{j,sigma}, y> >= t,
   b_{j,sigma}
     = s_sigma(a_j) - hat(sys)(a0)
       - <g_{j,sigma}, a_j - a0>.
   ```

4. Maximize `t` over these affine inequalities subject to
   `||y||_2 <= r ||a0||_2`, then fully evaluate the proposal.

This is a finite branch-envelope predictor, not Clarke gradient sampling. Its
errors and branch reachability must be calibrated as such.

The gradient-sampling experiment should not be implemented yet as the primary
next step: the current evidence no longer supports the claimed
fixed-incidence-boundary premise, and sampling gradients that may themselves
be wrong would compound the defect.

## Executed diagnostic and reproduction

The producer uses three retained proposals:

- the top endpoint's failed smallest-radius direction and its target winner;
- one clean failed proposal from rank 4 or 8; and
- one successful rank-2 proposal as an internal positive control.

At six decreasing step lengths along each fixed direction it:

1. record the named KKT spectrum/rank/condition, residual, beta vector and
   margin;
2. compare the analytic action envelope derivative with one-sided and central
   finite differences of the named action;
3. compare analytic `D V[d]` with volume finite differences;
4. at the base and smallest `+/- h` points, compare f64 with exact-arithmetic
   reconstructed incidence and volume, retaining incidence cardinalities and
   predicate margins;
5. combine the checked action and volume derivatives and compare with named
   branch-ratio finite differences; and
6. audit normalization, canonical-word mapping, and selected-solution
   bookkeeping if both derivative components agree but the combined prediction
   does not.

This uses named KKT solves and 39 paired f64/exact-arithmetic geometry
reconstructions, with no full sigma search. The retained release run took
about 55 seconds wall time and 60 seconds user CPU on the development
container; exact-arithmetic geometry dominates that diagnostic cost.

The implemented producer is `main.rs`. Its three-case run is:

```bash
cargo run --release -p exp-dev-gradient-ascent \
  --bin dev-gradient-ascent-endpoint-model-audit -- \
  --input experiments/dev-gradient-ascent/ascent-continuation/artifacts/top8-tuning-endpoints-one-step-20260729/input.json \
  --candidates experiments/dev-gradient-ascent/ascent-continuation/artifacts/top8-tuning-endpoints-one-step-20260729/raw/candidates.jsonl \
  --out /tmp/endpoint-model-audit

uv run --script \
  experiments/dev-gradient-ascent/endpoint-model-audit/analyze.py \
  /tmp/endpoint-model-audit/audit.json \
  /tmp/endpoint-model-audit/analysis
```

The producer evaluates only named words. It does not run the full sigma search.

## Code and evidence map

- Endpoint producer and validation loop:
  `ascent-continuation/main.rs`.
- Endpoint packet selection:
  `ascent-continuation/make_optimizer_endpoint_packet.py`.
- Reader report and figures:
  `ascent-continuation/analyze_optimizer_endpoints.py`.
- Raw retained endpoint data:
  `ascent-continuation/artifacts/top8-tuning-endpoints-one-step-20260729/`.
- Affine branch construction and Euclidean max--min solve:
  `optimizer-runs/src/branch_model.rs`.
- Full `sys` evaluation and geometry/volume route:
  `optimizer-runs/src/evaluator.rs`.
- Symmetry-transverse basis:
  `optimizer-runs/src/quotient.rs`.
- f64 geometry predicates and thresholds:
  `experiments/dev-quadratic-program/src/geometry.rs`.
- Capacity, volume, and systolic-ratio derivatives:
  `crates/symplectic/src/derivatives.rs`.

## Claim boundary

The retained evidence shows that five selected endpoints admit a finite
improving move under the implemented evaluator. For the three audited
proposals, it localizes the affine-model discrepancy to finite-distance
nonlinearity of the named KKT action and shows that KKT perturbation relative
to the smallest eigenvalue gap separates the tested failures from the
infinitesimal regime. This is an outcome-selected diagnostic, not a calibrated
population rule. It does not show that the highest endpoint is a local
maximum, a saddle point, or a fixed-incidence-boundary point, and it does not
certify that the evaluator equals the mathematical capacity.
