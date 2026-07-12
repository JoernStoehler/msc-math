# Frozen endpoint `sys` predictions before target evaluation

## Purpose and non-use of target data

This memo freezes the mathematical and numerical interpretation of the proposed
`3x6` and `4x4` ridge-endpoint smoke before any target call on its candidates.
I did not evaluate the planned endpoints or perturbations with a capacity/`sys`
routine, and I did not inspect any target output for them.

The target normalization is

```text
sys(P x_L Q) = c_EHZ(P x_L Q)^2 / (2 area(P) area(Q)).
```

Throughout, `J(x,y)=(-y,x)` is counterclockwise rotation by 90 degrees. Thus
`h_{J K}(v)=h_K(-Jv)`. Replacing `J` by `-J` reflects the induced norm and does
not change either endpoint value, but coordinate checks must use one sign
consistently.

The capacity is the minimum `Q`-billiard length in `P`, where an increment `v`
has length `h_Q(v)`. A minimizing closed polygon may be chosen with two or three
bounce points. These are the repo's stated Artstein--Avidan--Ostrover / Rudolf
and Bezdek--Bezdek interfaces.

## Frozen endpoint predictions

| bucket and endpoint | ridge value `R` | predicted `sys` | status |
|---|---:|---:|---|
| `4x4`, centrally symmetric quadrilateral `P`, `Q` a quarter-turn of `P` up to scale | `8` | `1/2` exactly | proved from affine reduction to the hypercube and its known capacity |
| `3x6`, triangle `P`, `Q = J(P-P)` up to sign/scale | `4 sqrt(6)` | `3/4` exactly | tight billiard derivation below; no target evaluation |

Thus the exact proxy minima do **not** have one common target value. The
`4x4` endpoint is lower in `sys` than the `3x6` endpoint by `1/4`, despite both
being exact equality cases for the same ridge lower-bound argument.

## Proved formula: the `4x4` equality family has `sys = 1/2`

Every centrally symmetric quadrilateral is a parallelogram. Write it, after
translation, as `P = A S`, where `S = [-1,1]^2`. If `Q = J P` (the harmless
choice of `-J`, translation, or an independent positive factor scale does not
change `sys`), apply the linear symplectic map

```text
(q,p) -> (A^{-1} q, A^T p).
```

The identity `A^T J A = det(A) J` in the plane sends the product to

```text
S x_L det(A) J S.
```

Since `JS=S=-S`, the second factor is the set `|det(A)|S`. Independent positive
scaling of one Lagrangian factor leaves `sys` unchanged, so this is
`sys`-equivalent to `S x_L S = [-1,1]^4`. The repo records the literature value

```text
c_EHZ([-1,1]^4) = 4,    volume = 16,
sys = 4^2 / (2*16) = 1/2.
```

Consequently `sys=1/2` holds on the entire nondegenerate `4x4` ridge-equality
family, not only for the centered square representative.

This endpoint is already a known example: the canonical square pair is exactly
the hypercube fixture, whose capacity is cited in the repo to HK2019 Example
4.6 / Rudolf. A successful target call would be regression evidence for the
smoke pipeline, not a new mathematical example or a new `sys` value.

## Tight derivation: the `3x6` endpoint has `sys = 3/4`

The construction is invariant under affine normalization of the triangle via
the corresponding block symplectic map and factor scale. Normalize

```text
P = conv{(0,0),(1,0),(0,1)},    area(P)=1/2,
D = P-P,                        area(D)=3,
Q = J D.
```

For this triangle and `v=(x,y)`, the stated convention gives

```text
h_Q(v) = width_P(-Jv)
       = max{0,y,-x} - min{0,y,-x}
       = N(x,y) := max(|x|, |y|, |x+y|).
```

The medial triangle with vertices

```text
(1/2,0), (1/2,1/2), (0,1/2)
```

cannot be translated into `int(P)`. Each of its three increments has
`h_Q`-length `1/2`, so it is a three-bounce candidate of total length `3/2`.
Indeed, a translation `(a,b)` putting all three vertices in `int(P)` would
simultaneously require `a>0`, `b>0`, and `a+b<0`. The non-translatable-polygon
characterization therefore gives the upper bound `c_EHZ <= 3/2`.

For the matching lower bound, let `(x_i,y_i)` be the vertices of any closed
polygonal curve and put

```text
x_min = min_i x_i,
y_min = min_i y_i,
s_max = max_i (x_i+y_i).
```

The curve can be translated into `int(P)` exactly when there are `a,b` such
that

```text
a > -x_min,    b > -y_min,    a+b < 1-s_max.
```

Such `a,b` exist exactly when

```text
F := s_max - x_min - y_min < 1.
```

Thus every non-translatable curve has `F>=1`. Translate it so that
`x_min=y_min=0`, and define

```text
X = max_i x_i,
Y = max_i y_i,
m = min_i(x_i+y_i),
M = max_i(x_i+y_i) = F.
```

For every increment `(dx,dy)`,

```text
N(dx,dy) = (|dx| + |dy| + |dx+dy|)/2.
```

Writing `TV` for cyclic total variation, the curve's `N`-length satisfies

```text
L_N = [TV(x) + TV(y) + TV(x+y)]/2
    >= X + Y + (M-m).
```

Here `X+Y>=M`. A vertex with `x=0` gives `m<=Y`, and a vertex with `y=0`
gives `m<=X`; hence `m<=min(X,Y)<=(X+Y)/2`. Therefore

```text
L_N >= X+Y+M-m
    >= M + (X+Y)/2
    >= 3M/2
    >= 3/2.
```

This lower bound holds for every closed polygonal curve, so together with the
medial triangle it proves

```text
c_EHZ(P x_L J(P-P)) = 3/2 = 3 area(P).
```

For completeness, if an arbitrary triangle is `P=A P_0+t`, the block
symplectic map `(q,p)->(A^{-1}q,A^T p)` and
`A^T J A=det(A)J` reduce its product to
`P_0 x_L det(A)J(P_0-P_0)`. The billiard length and capacity scale by
`|det(A)|` (reflection handles negative determinant), so

```text
c_EHZ = (3/2)|det(A)| = 3 area(P).
```

Since `area(Q)=6 area(P)`,

```text
sys = (3 area(P))^2 / [2 area(P) * 6 area(P)] = 3/4.
```

Epistemic status: the billiard correspondence and non-translatable-polygon
characterization are cited theorem-level inputs in the thesis; the complete
elementary total-variation lower bound is written above. I did not locate a
repo source that names this exact triangle--difference-body product as a
published example. The numerical value is nevertheless proved from the stated
inputs and is frozen as exact for interpreting the smoke. A thesis use should
retain this argument or replace it with a checked published source.

This is not a new high-`sys` value: `3/4` is already achieved by repo examples
such as the standard four-simplex and the crosspolytope. The present `3+6`
facet Lagrangian product is not linearly identical to either of those fixtures.
I did not find it explicitly registered as a known polytope in the repo.
Therefore a target computation may identify a useful additional realization,
but it must not be reported as discovering the value `3/4`, and the analytic
prediction predates the call.

## Predictions for the three interior perturbations on each path

What is proved for either path is continuity only:

```text
sys(K_epsilon) -> endpoint sys as epsilon -> 0.
```

The endpoint has several tied billiard branches, and the direct-optimization
packet does not specify the perturbation coordinates in enough detail to fix a
one-sided derivative. Neither monotonicity nor the side from which `sys`
approaches the endpoint is a theorem. Exact per-row predictions before the
geometry is frozen would be false precision.

The following qualitative predictions are frozen:

1. **`4x4`: reversal toward `1/2` is the central expectation.** The independent
   pre-target selections had mean `sys` about `0.6311` at ridge depth `.01` and
   `0.5583` at depth `10^-4`; the latter was lower by about `0.0728`. The exact
   endpoint is `0.5`. For a path whose outer perturbation is near the observed
   extreme-ridge scale, I expect a continued decline or flattening followed by
   decline as `R -> 8`, ending exactly at `0.5`. I assign low confidence to
   step-by-step monotonicity because branch switching can create a shallow
   bump and the path direction matters.

2. **`3x6`: weak rise or saturation toward `3/4` is the central expectation.**
   The corresponding selected means were about `0.7000` at depth `.01` and
   `0.7267` at depth `10^-4`, an increase of about `0.0267`; the exact endpoint
   is `0.75`. I expect the path to remain in the high-subthreshold band and to
   end at `0.75`, with a modest rise or plateau more likely than a large
   reversal. Monotone increase is not predicted strongly: the extreme sample
   already contained individual values above `0.75`, so a chosen path can
   overshoot and return.

3. **The cross-bucket prediction is stronger than either within-path shape.**
   Near their endpoints, the `3x6` path should be materially above the `4x4`
   path in `sys`, even though both continue to improve in `R`. This is the
   expected signature of bucket/equality-phenotype mixture.

These expectations use existing independent selected-band summaries only;
they are not fitted extrapolations, hit-rate estimates, or claims about
`sys>1`.

## Pre-registered interpretation of outcomes

- **First check the forced endpoint values.** If the computed endpoint differs
  materially from `0.5` (`4x4`) or `0.75` (`3x6`), stop scientific
  interpretation. The likely causes are construction/sign/scale mismatch,
  face-ordering loss, or a capacity pipeline error. It is not evidence against
  the formulas until those are excluded.

- **Expected mixture pattern:** `3x6` rises or saturates near `0.75`, while
  `4x4` falls toward `0.5`. This would strengthen bucket-specific
  saturation/Goodharting and reject a universal pooled proxy curve. It would
  not by itself distinguish a side-count effect from a more specific equality-
  phenotype effect.

- **Both paths fall as `R` improves:** stronger evidence that exact ridge
  equality is generally a symmetry-driven Goodhart endpoint. Because the
  endpoint levels differ, analyze changes relative to each endpoint rather
  than pooling raw `sys`.

- **Both paths rise monotonically to their forced endpoint values:** evidence
  that low `R` remains a useful local proposer on these two controlled paths.
  It still would not justify an asymptotic tail-dependence claim, a `sys>1` hit
  probability, or a general optimizer: both paths terminate at known values
  at most `0.75`.

- **`4x4` rises while `3x6` falls:** still supports mixture, but contradicts the
  direction predicted from the two observed selection depths. Reinspect how
  the deterministic paths relate to the generator's selected phenotypes before
  updating the broader tail model.

- **An endpoint-only jump or dip:** treat as equality-family/symmetry behavior,
  not continuation of the random tail. The three local perturbations are the
  relevant discriminator; add closer pre-target-frozen perturbations before
  making a limit-shape claim.

- **A perturbation exceeds `sys=1`:** potentially important, but it would not
  make the endpoint new and would not validate ridge minimization in general.
  It requires independent capacity verification, geometry checks, and a clear
  statement that the hit is an interior perturbation rather than either known
  endpoint.

## Scope boundary

This memo predicts only the canonical endpoint families and their proposed
small interior paths. It does not claim a formula for arbitrary centrally
symmetric `P x_L J P` beyond quadrilaterals, for incompatible side-count
buckets approaching a collapsed boundary, or for arbitrary perturbation
directions. No population rarity follows from the exact ridge endpoints.

## Response to independent review

Applied all requested repairs from `endpoint-predictions-review.md` before any
target call:

- fixed the literal convention `J(x,y)=(-y,x)` and its support-function sign;
- distinguished the matrix identity containing `det(A)` from the geometric
  scale `|det(A)|` in both endpoint reductions;
- replaced the compressed `3x6` lower-bound sentence with the complete
  non-translatability and cyclic-total-variation proof.

The forced endpoint values and qualitative perturbation predictions are
unchanged. No target output was used in this response.
