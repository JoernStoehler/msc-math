# Matching covariance and fourth moments still leaves different geometry

2026-09-18 bounded follow-up. These are controlled numerical witnesses on the
representation owner's eleven non-control bodies, not exact certificates or
capacity computations. The existing exact cube collision remains a calibration;
it is not counted again as a new result.

**Result:** for each of the eleven bodies, two orientations preserve the entire
centered-vertex Williamson spectrum and the bulk fourth symplectic-pairing
moment, while the normalized reciprocal of the maximum dual pairing differs
by 5.31–30.31%. Uniform-volume covariance and volume also stay fixed. Thus the
already-established new information in fourth moments does not mean that
adding that scalar to covariance closes these representations.

The largest differences are selected from a bounded, disclosed search. They
are existence witnesses on named bodies, not estimates of typical effects.
The reciprocal is c_J only where the body's central-symmetry hypothesis holds;
on nonsymmetric bodies it remains the named centered-polar descriptor. No
conclusion about capacity follows from its change.

## Construction

The native collection supplies a fixed body K, its uniform-volume fourth
tensor, its centered-vertex covariance C_v, and a three-form basis Omega_i.
Every unit u in R^3 specifies an orthogonal symplectic orientation
Omega(u)=sum u_i Omega_i. Uniform-volume covariance has been whitened to I.

The vertex second-pairing moment is the quadratic form

    q(u) = -tr(C_v Omega(u) C_v Omega(u)) = u^T A u,
    A_ij = -tr(C_v Omega_i C_v Omega_j).

It equals 2(nu1^2+nu2^2); also nu1^2 nu2^2=det(C_v), which stays fixed.
Therefore fixing q fixes both Williamson frequencies, not only their ratio.

Diagonalize A=E diag(lambda0,lambda1,lambda2) E^T, with increasing eigenvalues.
Put

    alpha^2=(lambda2-lambda1)/(lambda2-lambda0),
    beta^2=(lambda1-lambda0)/(lambda2-lambda0),
    u(theta)=E(alpha cos(theta), sin(theta), +/- beta cos(theta)).

Both great circles satisfy |u|=1 and q(u)=lambda1 identically. The four
controls with scalar A are excluded here: their behavior is already covered
by the representation owner's exact formulas.

The bulk fourth-pairing moment is homogeneous quartic in u. Its restriction
to either circle has the five-term form

    c0+c2 cos(2theta)+s2 sin(2theta)+c4 cos(4theta)+s4 sin(4theta).

The script recovers those coefficients from sixteen samples, checks them at
31 different angles per circle, and solves for repeated values at seven
interior levels. The levels are the 1/8 through 7/8 empirical quantiles on
two fixed 128-angle circle grids. Roots are bracketed and refined; antipodal
duplicates are removed. Native tensor contractions and covariance eigenvalues
then check the matched outputs directly.

This construction is algebraic at the covariance level and numerical at the
quartic-root and input-geometry levels. It is stronger than nearest-neighbor
matching, but it is not an exact certificate for the underlying convex body.

## Retained observations

Percent difference below means high/low minus one for the normalized dual
reciprocal. One strongest level is selected per body; all seven levels and
their extrema are retained in the JSON.

| Body | Difference |
|---|---:|
| gaussian_hull_8_0 | 22.57% |
| gaussian_hull_8_1 | 17.51% |
| gaussian_hull_12_0 | 15.01% |
| gaussian_hull_12_1 | 30.31% |
| gaussian_hull_18_0 | 19.45% |
| gaussian_hull_18_1 | 23.64% |
| symmetric_hull_16 | 11.16% |
| symmetric_hull_24 | 5.31% |
| facet_intersection_12 | 15.82% |
| facet_intersection_16 | 17.92% |
| asymmetrically_cut_cube | 17.64% |

Across the selected levels, the maximum rho discrepancy is 6.7e-15 and the
maximum bulk-kurtosis discrepancy is 8.9e-15. The quartic representation's
maximum independent-angle evaluation discrepancy is 1.21e-13 in the raw
fourth moment. These are numerical consistency diagnostics, not certified
error bounds.

Ridge sums also differ in these selected pairs, with signed changes ranging
from approximately -14.60% to +21.98%. Their direction does not uniformly
follow the dual reciprocal. Vertex fourth-pairing kurtosis is **not** held
fixed in this generic experiment; claiming all fourth-order representations
agree would overstate it.

## Evidence and consequence

`joint-level-contrasts.json` retains native input/code SHA256 identities,
quadratic matrices, eigenframes, quartic coefficients, orientation pairs,
matched measurements and all tested level outcomes. Native raw geometry and
moment tensors stay with `../representation-research/artifacts/bodies.jsonl`.

```sh
PYTHONDONTWRITEBYTECODE=1 uv run --with numpy==2.5.3 --with scipy==1.18.1 python \
  docs/empirical-viterbo-design/desk/relation-discovery/joint_level_contrasts.py \
  --bodies docs/empirical-viterbo-design/desk/representation-research/artifacts/bodies.jsonl \
  --measurement-code docs/empirical-viterbo-design/desk/representation-research/orientation_panel.py \
  --out docs/empirical-viterbo-design/desk/relation-discovery/joint-level-contrasts.json
```

The executed pass took roughly 1.5 seconds using cached numerical packages.
It regenerates neither native bodies nor capacity targets.

The scientific increment is robustness beyond the exact symmetric controls:
the joint descriptor loses information on nonproduct, nonsymmetric bodies,
including halfspace intersections and an asymmetrically cut cube. This
supports retaining extremal/field measurements alongside covariance and bulk
moments. It does not motivate endlessly matching more scalar columns or claim
that a complete geometric representation should have been possible from two
numbers. No further producer or classifier is queued from this result.
