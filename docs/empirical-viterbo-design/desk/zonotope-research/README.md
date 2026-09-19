# Zonotopes: pairing area, volume, and segment operations

First research window, 18 September 2026. Owner: `toolbox_brainstorm` under
the empirical desk. No symplectic capacity was evaluated. The numerical work
used a few seconds of producer time; interpretation, derivation, source checks,
and an independent mathematical audit were the substantive work.

## Result and current judgment

The retained candidate is a sharp, capacity-free inequality on **all
four-dimensional zonotopes**, with no bound on the number of generators:

\[
\boxed{\operatorname{vol}_4(Z)\le 4\Phi(Z)^2},\qquad
Z=\sum_i[-v_i,v_i],\quad
\Phi(Z)=\sum_{i<j}|\omega(v_i,v_j)|.
\]

**Status: conjectural in general.** A bounded literature check did not identify
this exact inequality, which is not a novelty certificate. Its product cases
are already explained by elementary/classical inequalities. The data support
the candidate but do not justify increasing the random sample.

There is a useful distinction from the historical ridge scout: this quantity
has a presentation-independent body meaning, its scale and symmetries are
understood, and the proposed bound is not fitted to capacity or systolic ratio.
It is an area-like linear-symplectic invariant, **not an EHZ capacity** and not
claimed invariant under nonlinear symplectomorphisms. No consequence for
Viterbo's inequality has been established.

The meaningful next crux is the rank-four skew-matrix inequality in
[mathematics.md](mathematics.md), or an adverse nonproduct configuration.
More successful product examples would add little: the entire planar-product
slice is classical mixed-area mathematics.

The subsequent [bounded rank-four follow-up](followup-rank-four.md) ruled out
a cancellation-free two-simple-bivector proof even at equality and derived a
zero-pairing condition for extremizing forms. The universal conjecture remains
open in this desk; the updated recommendation is to pause its scalar attack
unless a concrete cancellation-control method becomes available.

## Why this is a body measurement

The convention is coordinates `(q1,q2,p1,p2)` and half-segments `[-v_i,v_i]`.
Reordering or reversing generators changes nothing. Splitting a generator into
positive collinear pieces changes neither the body nor Phi. Uniform scaling by
`r` multiplies Phi by `r^2` and four-volume by `r^4`; linear symplectic maps
preserve both.

The exterior zonoid algebra provides a stronger interpretation than these
finite checks. In its normalization,
`h_(Z wedge Z)(omega) = 4 Phi(Z)`. Its product extends continuously to zonoids
and preserves ordinary set inclusion; thus Phi extends as a continuous,
inclusion-monotone body functional. This is **not** the paper's mixed J-volume,
which uses absolute complex determinants. The usual Alexandrov–Fenchel
inequality does not apply to this pairing functional: its mixed value vanishes
between orthogonal symplectic planes although both self-values are positive.
See Theorems 4.1 and 4.5, equations (4.7)–(4.8), and §6 of
[Breiding–Bürgisser–Lerario–Mathis](https://arxiv.org/html/2109.14996).

The formula `volume = 16 sum |Pfaffian_4|` is standard determinant algebra and
is used for computation, not counted as a discovery. Pairing matrices retain
more information than Phi; future failures of the scalar bound should be
investigated at that level rather than hidden by a different aggregate.

## What was derived

Details and proofs are in [mathematics.md](mathematics.md).

* The conjecture is equivalent to `4 sum |Pf W_I| <= (sum |W_ij|)^2` for
  arbitrary real skew matrices of rank at most four. It is proved here for
  at most five generators using the weighted triangle-free graph bound.
* For every planar Lagrangian product `P x_L Q`, it is exactly the classical
  mixed-area inequality. Equality includes `P x_L J_2 P` for **arbitrary**
  centrally symmetric planar polygons, not only squares or regular examples.
* For `P x_L J_2 P` plus any **one or two arbitrary segments**, a direct
  support-strip/determinant argument proves the bound for all nonnegative
  segment lengths. With one segment the deficit is exactly quadratic.
  This explains the controlled equality-neighborhood evidence; it is not
  evidence of an unexplained new product phenomenon.
* Dropping the rank-four restriction yields the easy but weaker constant 8.
  The improvement to 4 in arbitrary generator count is the unresolved step.

The proof audit was independent of the numerical producer. These are retained
mathematical derivations, not a formal proof artifact or independently
established novel theorems.

## Evidence and interpretation

| Packet | Scientific role | Outcome |
|---|---|---|
| [scout.json](scout.json) | 640 Gaussian, integer, near-symplectic-split and near-Lagrangian presentations; four short local searches | No violation; best search ratio about 0.9984; cube equality |
| [adversarial.json](adversarial.json) | 4,000 integer cases, 1,536 perturbations around a connected six-cycle equality presentation, A4/D4 root controls | No violation; four integer cases are rank-deficient controls, explicitly flagged |
| [segment_paths.json](segment_paths.json) | 351 two-segment response polynomials on five bases; raw added directions retained | 326 sufficient exact certificates for the entire nonnegative parameter quadrant; 25 other paths pass the finite grid |
| [validation.json](validation.json) | Independent Python-integer determinant replay, separate from numpy/Pfaffian producer | All 4,000 integer cases and all 351 coefficient packets checked |

The path grid has 68,796 evaluations but they are highly dependent points on
351 polynomials, not 68,796 independent bodies. Values exceeding 1 by roughly
`4e-16` are floating-point evaluation of equality. The exact certificates use
integer polynomial coefficients and an explicitly sufficient decomposition;
they do not rely on a tolerance or on grid coverage.

The deliberately reproduced volume-interaction example of
[Skorupinski, Theorem 3.1](https://arxiv.org/html/2608.07702v1)
has original-convention volumes `(14,30,26,56)`. We retain the same integer
vectors as half-generators, multiplying four-volume by 16. Along its two added
segments,

```
volume(t,s) = 224 + 256 t + 192 s + 224 ts
Phi(t,s)    = 10  + 6 t   + 4 s
```

The numerator of the mixed derivative of `log(volume)` is positive (1024),
whereas that of `log(Phi)` is negative (-24). This is a concrete contrast
between symplectic pairing response and volume response. The volume
counterexample itself is published prior work, and Phi's response follows
directly from its definition; neither is advertised as a new theorem.

Across the retained paths there are 23 strict opposite-sign responses.
These are examples of information the representation can distinguish, not
a statistical significance claim or evidence of causality.

## Sources and coverage boundaries

The exterior-zonoid source was checked for the actual product, continuity,
monotonicity and mixed-J-volume definitions. A bounded independent audit also
checked the [Handbook of zonoid calculus](https://iris.sissa.it/handle/20.500.11767/129410):
its generalized Grassmann-zonoid and mixed-J Alexandrov–Fenchel statements are
conjectures, not theorems that settle the proposed inequality. This is not an
exhaustive search of related norm or exterior-algebra inequalities.

Skorupinski's explicit matrix and its four volumes were source-checked and
replayed exactly. The separate
[Fradelizi et al. paper](https://arxiv.org/html/2608.12681v1)
connects zonoid volume polynomials to Rayleigh properties and provides further
adverse constructions. Its abstract/introduction were read in this window;
its construction sections were not fully audited or used as data.

## Reproduction and next decision

All scripts use Python and numpy except the independent validator, which uses
only the standard library. Run from any directory:

```
python /path/to/zonotope-research/scout.py
python /path/to/zonotope-research/adversarial.py
python /path/to/zonotope-research/segment_paths.py
python /path/to/zonotope-research/validate.py
```

Scripts overwrite only their own adjacent JSON outputs. Seeds, geometry,
accepted local-search steps, and failed-to-certify paths remain in the packets.
The `body_id` is a SHA-256 of the literal generator payload, **not** a canonical
identifier modulo alternate presentations or symplectic transformations.
Equality of rounded coordinates is never used to merge records.

The recommended bounded mathematical/source assessment has now been completed;
see [the follow-up](followup-rank-four.md). It found a proof obstruction and a
constrained extremal locus, but neither a general proof nor a violation.
Recommendation: keep Phi as a semantically grounded measurement and move the
zonotope line to richer pairing arrangements/other operations unless a concrete
cancellation-control method justifies reopening this scalar conjecture. The
single/two-segment lemma is a calibration result, not by itself a reason to
prioritize a narrow theorem paper. Capacity computation should enter only if a
specific geometric distinction warrants that added cost.
