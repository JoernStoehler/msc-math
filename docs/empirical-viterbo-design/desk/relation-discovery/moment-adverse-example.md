# A finite moment inequality fails on a controlled convex family

2026-09-18. Exact calculation independently checked. The original finite-data
inequality is false; no replacement inequality or literature novelty is claimed.
This is target-free geometry using the partner owner's new measurements. No
capacity was computed on the adverse examples.

For uniform-area X in a planar convex body, whiten the centered distribution
to Z with E Z=0 and E ZZ^T=I. Define

    b3 = sum_{i,j,k} (E[Z_i Z_j Z_k])^2,
    b4 = E |Z|^4.

These are `affine_skew_tensor_norm2` and `affine_radial_fourth_moment` in the
partner collection. The disk has b3=0 and b4=16/3. The triangle control has
b3=32/25 and b4=32/5. These affine-invariant quantities measure ordinary
geometry; their usefulness need not be judged by capacity prediction.

## From an empirical frontier to an adverse example

In the one-parameter bound family

    b4 >= 16/3 + alpha b3,

all 49 supplied partner rows allow alpha up to approximately 0.6904229218;
the limiting row is `smooth_36_sym_0.125`. This is a finite-data envelope, not
a universal coefficient. The rows contain 33 heterogeneous base bodies and
16 deliberately related symmetrization descendants, not 49 independent draws.

The simple candidate alpha=2/3 survives every row, with minimum slack
9.859261657e-5. It also agrees with the leading-order relation for a pure
third-harmonic perturbation of the disk. That agreement is not enough:
higher-order terms have the opposite sign.

Consider the radial body

    K_a = {(r cos(theta),r sin(theta)): 0 <= r <= 1+a cos(3 theta)},
    0 < a < 1/10.

It is strictly convex. Writing c=cos(3 theta), the curvature numerator is

    R^2 + 2(R')^2 - R R'' = 1+11ac+18a^2-8a^2 c^2
                         >= (1-a)(1-10a) > 0.

Threefold rotational symmetry forces zero centroid and scalar covariance.
Let D=1+a^2/2, m2=E r^2, m4=E r^4 and H3=E[r^3 cos(3 theta)]. Direct polar
integration gives

    area(K_a) = pi D,
    m2 = (1+3a^2+3a^4/8)/(2D),
    m4 = (1+15a^2/2+45a^4/8+5a^6/16)/(3D),
    H3 = a(1+3a^2/2+a^4/8)/D.

The covariance is (m2/2)I. Reflection symmetry and the third-harmonic tensor
give

    b4 = 4m4/m2^2,
    b3 = 2H3^2/m2^3.

With t=a^2 and U=1+3t+3t^2/8, exact simplification yields

    b4 - 16/3 - (2/3)b3
      = -(16/3)t^2 [11/8+(15/2)t+(3/4)t^2+t^3/32+5t^4/512]/U^3 < 0.

Thus the whole strictly convex family refutes the proposed bound. The leading
slack is -(22/3)a^4: a second-order calculation would miss its failure.

At a=1/20, the exact slack is

    -852161933360 / 18758376094322967
    = -4.5428342468e-5 approximately.

This is a proof for the smooth body, not a floating-point sign test. Moment
and covariance continuity under inscribed polygon approximation also proves
the existence of polygon counterexamples. Direct integration on regular
angular samples with 96, 384 and 1536 vertices gives slacks approximately
-4.40506e-5, -4.53828e-5 and -4.54257e-5. These rounded polygon calculations
are numerical checks, not exact certificates for the stored binary vertices.

## Evidence and replay

`moment-frontier-check.json` retains every input-row check, source hashes,
exact rational smooth-body values, and the polygon convergence check.
`moment-adverse-polygons.jsonl` retains the actual 96-vertex polygon, body ID,
construction and numerical status for inexpensive cross-line reuse. All
larger polygons are reproducible from their explicit angular construction;
no expensive feature producer or capacity backend was used.

```sh
PYTHONDONTWRITEBYTECODE=1 uv run --with numpy==2.5.3 --with scipy==1.18.1 python \
  docs/empirical-viterbo-design/desk/relation-discovery/moment_frontier.py \
  --collection docs/empirical-viterbo-design/desk/pentagon-arbitrary-partner/collection-v1.jsonl \
  --moment-code docs/empirical-viterbo-design/desk/pentagon-arbitrary-partner/collection.py \
  --out docs/empirical-viterbo-design/desk/relation-discovery/moment-frontier-check.json
```

The replay imports the other owner's pure moment routine and does not call
its collection or evaluator entrypoints. The exact radial calculation uses
rational arithmetic independently of polygon integration.

## Interpretation and limits

This is a completed discovery cycle: a simple finite-data inequality, a
mathematically selected missing family, and an explicit disproof. The source
collection's smooth bodies mix harmonics; their success did not test the pure
third-harmonic near-disk direction. Acquiring this direction was useful in a
way that adding more generic bodies would not necessarily be.

It does not show that the two measurements are uninformative, or justify
fitting an unrestricted nonlinear bound. A corrected extremal moment question
would be a separate project, requiring a literature check and adversarial
families beyond this one. No such project is automatically launched.

A second representation limit is retained in the JSON: regular pentagons and
heptagons have zero third-moment tensor but positive difference-body area
excess. Third moments detect one kind of asymmetry, not all asymmetry.

The wider distributional context is Mardia's multivariate skewness and
kurtosis, as defined for example in
[Chowdhury et al., Sub-dimensional Mardia measures](https://arxiv.org/abs/2111.14441).
The exact counterexample above is self-contained; no convex-body extremal
theorem was inferred from that reference.
