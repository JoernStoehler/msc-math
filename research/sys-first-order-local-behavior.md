<!--
Purpose: classify the local first-order behavior object needed for sys in a fixed row chart.
Context: draft research note for `tasks/planning-notes.md`; no task status change is implied.
-->

# Sys First-Order Local Behavior

Research question: what object stores all locally relevant HK branches near a
fixed row-chart polytope and evaluates first-order behavior of `sys` without
re-solving the full HK problem for every perturbation direction?

Epistemic status: draft theorem classification from local source reading on
2026-04-30. The positive route below depends on external semialgebraic
geometry theorems that are not yet cited in the repo; before theorem-facing
use, check a source such as Bochnak-Coste-Roy, *Real Algebraic Geometry*, or
Basu-Pollack-Roy, *Algorithms in Real Algebraic Geometry*, for the exact forms
of Tarski-Seidenberg quantifier elimination, semialgebraic curve selection,
Puiseux/monotonicity for one-variable semialgebraic germs, and cylindrical
algebraic decomposition.

Evidence read: historical task notes for sys first-order and HKO,
`thesis/legacy/general-case-algorithm.tex`,
`thesis/legacy/general-case-algorithm-proof.tex`,
`formal/hk2017-qp-core.tex`, `formal/hk2017-qp-precision.tex`,
`formal/capacity-smoothness-classification.tex`,
`formal/capacity-boundary-subdifferential.tex`,
`formal/hko-local-maximality-conditions.tex`,
`formal/hko-symmetry-gradient-structure.tex`,
`research/hko-local-maximum-exact-clarke.md`,
`research/combinatorial-cells.md`, and `papers/hk2017/EHZ-polytopes.tex`.

## 1. Problem model

Work in a fixed listed-row chart
`K(a) = {x in R^4 : a_i^T x <= 1, i = 1,...,F}` with `a` near a base point
`a0` where `K(a0)` is a bounded convex body with nonempty interior. The HK
input from symplectic geometry is finite: for every cyclic listed sequence or
subset-permutation `sigma`, define an action matrix `H_sigma(a)` with entries
from `omega_0(a_i,a_j)` and a constraint matrix
`C_sigma(a) beta = e`, where `sigma` is an ordered support and the first four
rows encode `sum_i beta_i a_{sigma(i)} = 0`, while the last row encodes
`sum_i beta_i = 1`.
For that `sigma`,

```text
V_sigma(a) = max 1/2 beta^T H_sigma(a) beta
             subject to C_sigma(a) beta = e, beta >= 0.
```

The HK value is `Qmax(a) = max_sigma V_sigma(a)` and
`c_EHZ(a) = 1/(2 Qmax(a))` when `Qmax(a) > 0`. The systolic ratio is then
`sys(a) = c_EHZ(a)^2 / (2 Vol(a))`, so first-order `sys` needs the same local
object for `Qmax` plus a matching local object for `Vol`.

The symplectic/HK-specific parts are the HK formula, the finite list of
combinatorial words, the symplectic matrix entries in `H_sigma`, and the
normalization connecting `Qmax` to `c_EHZ`. The rest is algebraic finite-branch
optimization: finitely many parametric quadratic programs with polynomial
equalities, weak inequalities, and a finite max, followed by rational smooth
operations wherever `Qmax` and `Vol` stay positive.

Listed rows are not the same as actual facets. Repeated or redundant listed
rows must stay in the algebraic model because a row can be unused at `a0`,
become used along a perturbation, or be dismissed only after an exact dominance
certificate. The minimum-support HK value argument proves value completeness,
but by itself it does not prove first-order completeness.

## 2. Branch theorem candidate

For one fixed branch `sigma`, the strongest defensible local theorem is not a
gradient theorem. It is a semialgebraic active-germ theorem.

Define the branch graph by the first-order formula

```text
(a, y) in Graph(V_sigma)
iff exists beta:
  C_sigma(a) beta = e,
  beta >= 0,
  y = 1/2 beta^T H_sigma(a) beta,
  and for all gamma with C_sigma(a) gamma = e, gamma >= 0:
      y >= 1/2 gamma^T H_sigma(a) gamma.
```

Tarski-Seidenberg makes this graph semialgebraic. A branch active germ at
`a0` in direction `h` is a semialgebraic arc
`t -> (a0 + t h, beta(t))`, `t > 0`, such that `beta(t)` is feasible and
attains `V_sigma(a0+t h)`, with the value converging to a finite limiting
value as `t -> 0+`. The branch first-order object is a finite semialgebraic
cell decomposition of direction space together with, on each cell, a
quantifier-free description of the right derivative

```text
DV_sigma(a0; h) =
lim_{t -> 0+} (V_sigma(a0 + t h) - V_sigma(a0+)) / t
```

for the relevant right-hand branch germ. This object is an active-germ
catalogue or semialgebraic cell object. It is not a single gradient. It is not
the Clarke subgradient, although at locally Lipschitz smooth-envelope points
the Clarke object is a coarse outer description.

This theorem covers `beta_i = 0` by not treating the zero coordinate as a
failure. A zero coordinate simply places the optimizer on a face of
`beta >= 0`; the germ records which zero coordinates stay zero, which become
positive as `beta_i(t) > 0`, and which signs block the branch. It also covers
`beta_i(t) > 0` limiting to `beta_i(0) = 0`, since that is exactly an arc in
the graph closure. The correct feasibility test is ray feasibility in the
semialgebraic graph, not the linearized inequality `dot beta_i >= 0` alone.
Linearized feasibility is a useful local certificate only under an explicit
regularity condition; singular rank changes can make the linearized cone too
large or can hide arcs that require higher-order terms.

Singular KKT systems and active continua are handled by quantifying over the
optimizer set instead of solving a nonsingular KKT matrix. Repeated or
redundant listed rows are handled because they remain ordinary coordinates in
the formula. Exact-real claims use the semialgebraic formula over the ordered
real field. Floating-point diagnostics can sample cells, reveal likely active
patterns, and detect conditioning failures, but they do not certify the
quantified branch statement.

## 3. Aggregation theorem candidate

For the HK value, aggregate by finite max:

```text
Qmax(a) = max_sigma V_sigma(a).
```

A branch is value-active at `a0` if its right-hand branch value has limiting
value `Qmax(a0)`. A branch with an exact zeroth-order gap cannot affect the
first-order derivative. A branch that is infeasible or below the max at the
base algebraic point can still become first-order active along a direction if
its right-hand germ has limiting value `Qmax(a0)`. This is why the finite list
to keep is not just the set of base KKT solutions with `beta > 0`; it is the
set of right-active semialgebraic germs.

The aggregation object is the finite union of all right-active branch-germ
catalogues, cell-refined so that the winning branch list is constant on each
direction cell. On such a cell,

```text
DQmax(a0; h) = max over right-active sigma on that cell of DV_sigma(a0; h).
```

For capacity, use `c_EHZ = 1/(2 Qmax)`, so

```text
Dc_EHZ(a0; h) = - DQmax(a0; h) / (2 Qmax(a0)^2).
```

For `sys = c_EHZ^2/(2 Vol)`, combine this with the matching directional
derivative of `Vol`. In a fixed row chart, `Vol` is also finite
semialgebraic: enumerate vertex supports, keep ray-feasible vertex germs, and
cell-decompose direction space. On a cell where `DVol(a0; h)` is known,

```text
Dsys(a0; h)
  = sys(a0) * (2 Dc_EHZ(a0; h)/c_EHZ(a0) - DVol(a0; h)/Vol(a0)).
```

Inactive branches can become active only if the exact zeroth-order gap closes
along their right-hand germ. Since the branch list is finite, a cell
decomposition can record exactly which branches have zero limiting gap and
which are separated by a positive gap. Without that certificate, pruning a
base-inactive branch is an assumption.

## 4. Efficiency/pruning implication

Later code can avoid looping over all `sigma` for every direction `h`, but only
after a one-time exact catalogue has been built. The catalogue must contain:

- the exact base value `Qmax(a0)` and `Vol(a0)`;
- every branch/zero-pattern/right-germ whose limiting value is `Qmax(a0)`;
- a direction-space cell decomposition;
- on each direction cell, the branch derivative formula and dominance relation;
- for every discarded `sigma`, either a positive zeroth-order gap certificate
  or a first-order dominance certificate by a kept germ on every cell.

The branch-pruning rule for unused beta coordinates is therefore conditional.
If a `sigma` has `beta_k = 0` at the base point, the smaller
`sigma \ {k}` gives the same base value, and the current HK algorithm can use
that for value computation. For first-order behavior this is not enough:
`sigma` may have a right-hand germ with `beta_k(t) > 0` and a different slope.
It can be pruned only if the catalogue proves that this appearing-coordinate
germ is absent, has a positive gap, or is first-order dominated by a retained
germ on every direction cell.

If the target is a practical exact-real implementation, the first pass may
still loop over all `sigma`, all supports, and all zero-patterns to build the
certificate. Evaluation after that can be cell lookup plus finitely many
polynomial/rational comparisons. If no such certificate is built, the
obstruction is real: a base-value minimum-support reduction is not a
first-order branch-completeness proof.

## 5. HKO relevance

HKO is not a shortcut around this theory. It is a hard instance of it.
Current HKO evidence has many active orbits, symmetry quotients, exact
representative rows, endpoint/midpoint families, and unresolved seven-facet
representative classes. The formal HKO second-order route uses a
smooth/min-envelope argument that the task progress files already reject as a
substitute for arbitrary closed-value behavior. The gradient-analysis note
also records that current bookkeeping has 150 exact-action orbits, 20 visited
facet subsets, and 28 distinct gradient patterns, so the active surface is not
a single smooth branch.

The `beta_k = 0` case is central for HKO because some listed combinatorics
collapse to a shorter support at the base point while nearby or symmetric
representatives may use the coordinate. Symmetry helps compress the catalogue,
but it does not prove that no asymmetric right-active germ was missed.

An HKO-specific certificate could avoid proving the full arbitrary-polytope
theorem only if it supplies the same facts for the HKO base point: exact
right-active branch coverage, exact row formulas for all kept germs, exact
dominance or gap certificates for discarded germs, and exact comparison of the
resulting Clarke-flat or direction-cell object with the 15-dimensional symmetry
tangent space. That is the same theorem content with HKO data substituted for a
global `a0`.

## 6. Thesis-readable explanation

The HK formula turns the EHZ capacity of a polytope into a finite list of
quadratic optimization problems. For one listed order of facets, the variables
are the dwell-time weights `beta`, constrained by closure, normalization, and
`beta >= 0`. At a regular point with one positive optimizer, this branch has an
ordinary gradient. The difficulty is that HKO and other important polytopes do
not sit only in that regular case. A weight can be zero at the base point and
become positive under perturbation. Several optimizers can tie, a KKT matrix
can be singular, and a listed row can be repeated or unused. Dropping a zero
weight is valid for computing the base HK value, but it can discard a branch
that has a different first-order slope nearby. Therefore the local object must
record germs of active branches, not only gradients of currently positive
branches. A germ means a small one-sided family of feasible optimizers as the
polytope is perturbed in a chosen direction. Because all branch equations and
inequalities are polynomial, the set of such germs is semialgebraic and can in
principle be decomposed into finitely many direction cells. On each direction
cell, the derivative is computed by a finite formula and the active branches
are known. The capacity derivative is obtained from the derivative of the HK
maximum, and the systolic-ratio derivative also includes the volume derivative.
This gives a mathematically finite evaluator, but the proof uses heavy real
algebraic geometry rather than a short smooth-envelope argument. For HKO, a
shorter certificate may be possible only because symmetry reduces the finite
catalogue; it must still prove that every right-active germ has been included.

## 7. Verdict

ONLY-HEAVY.

The compute-once evaluator exists at the level of semialgebraic
cell-decomposition machinery: define all branch value graphs by quantified
polynomial formulas, eliminate quantifiers, decompose direction space into
cells, and store the resulting active-germ catalogue and derivative formulas.
This handles the hard cases, but it is too large to be the clean thesis proof
one would want for the main HKO story. A thesis-readable section can explain
the model and why the smooth route fails; a theorem-strength proof would need
either a checked semialgebraic-geometry citation chain or an HKO-specific exact
certificate with the same active-germ coverage.

Hard-case coverage:

| hard case | classification |
| --- | --- |
| `beta_i = 0` at an optimizer | Included as a boundary face in the semialgebraic optimizer graph; not discarded unless a dominance/gap certificate exists. |
| `beta_i(t) > 0` limiting to `beta_i = 0` | Included as a right-active arc/germ; this is the main reason base support pruning is unsafe for derivatives. |
| ray feasibility vs linearized feasibility | Ray feasibility in the semialgebraic graph is the theorem object; linearized feasibility is only a certificate under a separate regularity hypothesis. |
| singular KKT systems or active continua | Covered by quantifying over optimizer sets and value graphs, not by inverting a KKT matrix. |
| repeated/redundant listed rows | Remain coordinates in the listed-row formula; they can be pruned only by exact dominance/gap certificates. |
| exact-real theorem vs f64 diagnostic behavior | Exact theorem lives over ordered real closed fields; f64 can find candidate cells and conditioning failures but cannot certify branch coverage. |
| branch pruning when a `sigma` has an unused beta coordinate | Safe for base value through minimum-support reduction; unsafe for first-order behavior unless appearing-coordinate germs are certified absent or dominated. |

Smallest tempting substitute that fails: "keep only base active branches with
strictly positive beta and apply the directional derivative of a finite minimum
of smooth functions." It fails precisely at beta-boundary appearances,
singular KKT systems, and branches whose base value is duplicated by a smaller
support but whose nearby first-order slope differs.
