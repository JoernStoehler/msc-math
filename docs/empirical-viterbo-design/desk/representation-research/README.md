# Complementary representations of four-dimensional geometry

First research window, 18 September 2026. Owner: `broad_discovery_agenda`.
No capacity evaluations. These are controlled geometric observations and exact
calibration identities, not new capacity bounds or a claim of mathematical novelty.

## Scientific result

**Bulk fourth-order symplectic pairings contain information absent from both
existing vertex covariance and continuous-volume covariance. They nevertheless
miss some orientation distinctions that supporting-facet pairings detect.**

We constructed a heterogeneous collection of 15 four-polytopes and investigated
128 orientations per body. Eleven bodies are non-control examples: Gaussian
vertex hulls, centrally symmetric nonproduct hulls, halfspace intersections, and
an asymmetrically truncated cube. Simplex, cube, crosspolytope and 24-cell provide
different exact/symmetry controls. Facet counts range from 5 to 64. These are
selected scientific examples, not 1,920 independent draws from a population.

Each body was first centered at its uniform-volume centroid and whitened using
its **uniform-volume** covariance. This is generally a non-symplectic change of
body, not removal of a capacity-preserving gauge. Original vertices, centroids,
covariances and whitening maps remain in the artifact. Subsequent rotations
preserve its entire volume covariance, now the identity, and volume.

An initial comparison would still leave the historical extreme-vertex
covariance free to vary. We therefore constructed additional paired orientations
that also preserve **both Williamson frequencies of centered extreme-vertex
covariance**. Every one of the eleven non-control bodies has a retained pair
with different bulk pairing kurtosis, by 1.61–7.79%. Vertex-rho discrepancies
are at most 4.9e-15. The construction preserves the frequencies analytically;
the listed differences are floating-point evaluations. Thus this is not merely
another correlation with covariance or a change of covariance measure.

| Source body | Bulk pairing kurtosis increase at matched covariance spectra |
|---|---:|
| Gaussian hull, 8 input points, replicate 0 | 7.79% |
| Gaussian hull, 8 input points, replicate 1 | 2.00% |
| Gaussian hull, 12 input points, replicate 0 | 2.43% |
| Gaussian hull, 12 input points, replicate 1 | 3.42% |
| Gaussian hull, 18 input points, replicate 0 | 2.52% |
| Gaussian hull, 18 input points, replicate 1 | 3.95% |
| Symmetric hull, 16 input points | 7.51% |
| Symmetric hull, 24 input points | 3.47% |
| Intersection of 12 halfspaces | 2.72% |
| Intersection of 16 halfspaces | 1.61% |
| Asymmetrically cut cube | 5.05% |

The matched pairs were selected from four prescribed magnitude patterns and
eight sign choices for each pattern. This is a disclosed search for separating
examples, not a statistical test or an estimate of typical effect size.

The exact controls expose a different limitation. For a regular simplex,
bulk pairing kurtosis is **6723/1568 in every orientation**. For the regular
24-cell it is **3645000/1399489 in every orientation**. Nevertheless, the
normalized reciprocal dual-pairing extremum changes by 20.4% and 34.3%,
respectively, over the sampled orientations. Those spans are sampled; the
kurtosis identities hold for the whole orientation sphere. Merely adding a
fourth-order scalar does not solve representation sufficiency.

## Definitions and what is actually invariant

Use omega(x,y)=x^T J y, with the repository q,p convention. For independent
uniform-volume points X,Y in the centered body, define

    kappa(K,omega) = E[omega(X,Y)^4] / E[omega(X,Y)^2]^2.

These are exact polynomial integrals evaluated in f64, not Monte Carlo estimates.
The second moment is -tr(C J C J); for C=I it equals 4. All odd pairing moments
vanish under exchange of X and Y. The fourth moment contracts two copies of the
full fourth-moment tensor with four copies of J.

For polar vertices a_i of the centroid-centered body, retain

    d(K,omega) = 1 / (sqrt(vol K) max_ij |omega(a_i,a_j)|).

On centrally symmetric bodies this is the volume-normalized c_J quantity in
[Berezovik's convention](https://arxiv.org/html/2310.14998). On nonsymmetric
bodies it is explicitly named a dual reciprocal descriptor; the symmetric
capacity-comparison theorem is not imported. The maximizing facet pair is retained.

Also retain primal pairing extrema, ridge-area sum/concentration, and vertex
pairing kurtosis. The last uses vertices centered at the **volume centroid**;
the separate historical covariance fields use vertices centered at their own
vertex mean. These measures must not be conflated.

All normalized measurements above are invariant under translations and linear
symplectic maps, and under common scaling. They are not claimed invariant under
arbitrary nonlinear symplectomorphisms.

Rather than recomputing hulls after each rotation, hold coordinates fixed and
use Omega(u)=u_1 Omega_1+u_2 Omega_2+u_3 Omega_3, |u|=1. The retained orthogonal
complex structures anticommute, satisfy Omega(u)^2=-I, and parameterize the
SO(4)/U(2) orientation sphere containing J. This is equivalent to rotating the
body: Omega=R^T J R. Every row retains u and Omega; body geometry plus Omega
specifies the experiment without a choice of representative R.

The fourth-moment numerator is a homogeneous quartic in u. In contrast, the
dual extremum is the maximum of absolute linear functions of u, and ridge sum
is a sum of absolute linear functions. Their different mathematical forms are
one reason to retain their directional fields and witnesses instead of selecting
a single scalar winner. No useful capacity relationship follows automatically.

## How the matched covariance construction works

Let C_v be centered extreme-vertex covariance of a whitened body, and set

    A_ij = -tr(C_v Omega_i C_v Omega_j).

The vertex second-pairing moment is u^T A u. Flip signs of coordinates of u in
an orthonormal eigenframe of A, retaining their magnitudes. This preserves both
|u| and u^T A u. The determinant of C_v is unchanged by orientation. The sum
of squared Williamson frequencies and their product are therefore unchanged,
so both frequencies are fixed. Volume covariance remains I throughout.

The fourth moment generally changes under those sign choices. The artifact
retains A, its eigenframe, chosen magnitudes and both measurements. This is
a reproducible descriptor-fiber construction; rounded-nearby geometries were
not declared equal.

## Independent exact calibrations

The cube calculation has a direct probability derivation. After isotropization,
coordinates are independent, have variance 1 and fourth moment 9/5. Expansion gives

    kappa_cube(u) = 27/10 + (9/25)(u_1^4+u_2^4+u_3^4),
    d_cube(u) = 1 / (4 max_i |u_i|),
    normalized_ridge_sum_cube(u) = 8 sum_i |u_i|.

Thus its kurtosis ranges exactly from 141/50 to 153/50. Even the same cube
kurtosis need not determine d: u=(1/sqrt(2),1/sqrt(2),0) and
u=(sqrt(2/3),1/sqrt(6),1/sqrt(6)) both give 72/25, but give different d.

For the simplex, the independent symbolic calculation uses isotropic vertices
with Gram matrix 30(I-11^T/5). Its fourth moment is
(15/28)D+(1/280)sum_i v_i^tensor4, where D is the sum of the three Kronecker
pairing tensors. The exact identity
sum_ij omega_u(v_i,v_j)^4=1166400 |u|^4 yields the stated constant kurtosis.

The 24-cell is [-1,1]^4 intersect {sum |x_i|<=2}. Orthant inclusion-exclusion
integrals give volume 8, C=(13/60)I, E[X_1^4]=3/28 and
E[X_1^2 X_2^2]=1/28. Sign/permutation symmetry then makes its fourth tensor
isotropic; the pairing-kurtosis constant follows. This calculation does not
depend on the panel's Qhull triangulation or tensor integration code.

## Evidence, validation and reproduction

* [bodies.jsonl](artifacts/bodies.jsonl): recoverable original and whitened
  geometry, maps, polar vertices, incidence, ridge vectors, covariance and full
  fourth-moment tensor. Body IDs hash original f64 vertex bytes.
* [measurements.jsonl](artifacts/measurements.jsonl): 1,920 orientation rows,
  including original historical vertex-covariance frequencies.
* [matched covariance contrasts](artifacts/matched-covariance-contrasts.json):
  eleven constructions and their paired measurements.
* [native measurements](artifacts/native-measurements.jsonl): original centered
  source bodies measured before whitening; a separate geometric view.
* [definitions](artifacts/definitions.json), [summary](artifacts/summary.json),
  [integration checks](artifacts/checks.json), [validation](artifacts/validation.json)
  and [exact identities](artifacts/exact-calibrations.json).

The largest relative error under an independently chosen linear symplectic map,
translation and scaling was 3.8e-15. Independent exact cube/simplex/24-cell
formulas agree within 7.2e-15. Whitened covariance differs from I by at most
3.2e-15. Qhull topology and coplanar-facet merging are floating point; these
checks are numerical validation, not exact geometry certification.

Reproduce from the repository root, each command separately:

```sh
OPENBLAS_NUM_THREADS=1 OMP_NUM_THREADS=1 uv run --script docs/empirical-viterbo-design/desk/representation-research/orientation_panel.py
uv run --script docs/empirical-viterbo-design/desk/representation-research/derive_calibrations.py
OPENBLAS_NUM_THREADS=1 OMP_NUM_THREADS=1 uv run --script docs/empirical-viterbo-design/desk/representation-research/matched_covariance.py
OPENBLAS_NUM_THREADS=1 OMP_NUM_THREADS=1 uv run --script docs/empirical-viterbo-design/desk/representation-research/validate_and_summarize.py
```

Scripts write only this directory's artifacts. The panel itself took about one
second on this host after dependencies were available; all runs were bounded,
single-threaded and far below the first-window resource limit. No historical
datasets were rebuilt or materialized and no capacity producer was launched.

## Research judgment

Keep bulk higher moments **and** dual extremal interactions as complementary
representations. The controlled examples justify this choice more clearly than
another pooled correlation. Do not expand the table merely because measurements
are cheap. Vertex and volume pairing kurtosis still correlate strongly on this
small panel (within-body Spearman roughly .88–1 outside constant cases), so their
apparent breadth should not be overstated.

The discovery-method owner has the structured fields and matched pairs. The next
bounded step is their cross-field analysis, then one adversarial construction
against an actual surviving relation. That should take roughly 30–60 minutes of
analysis and small geometry work; a broad capacity comparison or a theorem about
all polytopes would be a separate decision. If no interpretable relation survives,
the immediate result is still a clear representation distinction and an explicit
insufficiency finding, not evidence that any of these descriptors explains capacity.
