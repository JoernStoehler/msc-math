# Intrinsic dimension, density body, and topology wishlist

This target-free packet asks a narrow preliminary question: in explicitly
declared coordinate views, do samples look grossly ambient-filling, or do they
instead have low-rank/local/concentrated/multicomponent signatures?  It is not
an attempt to assign a true dimension or topology to a polytope law.

## Views and quotient boundary

For a product represented by two CCW planar polygons, the analyzer translates
each factor to its vertex mean, reconstructs its H inequalities, and embeds the
dual facet vertices in `(q1,q2,p1,p2)`.  At fixed total facet count `F`, it
uses three separate vectors in `R^(4F)`:

- `fixed_order_dual`: q-factor cyclic order followed by p-factor cyclic order;
- `facet_permutation_canonical_dual`: lexicographically sorted facet vectors,
  removing only arbitrary facet ordering;
- `orthogonal_and_permutation_invariant_gram_spectrum`: sorted eigenvalues of
  the dual Gram matrix, which also removes common orthogonal coordinates but is
  many-to-one and therefore a feature view, not a chart.

No view quotients incidence strata, conditioning boundaries, linear symplectic
maps, or arbitrary factor permutations.  Different `F` values are never
pooled in the natural dual chart. Translation removal is an explicit adapter
choice, not an assertion about dual-coordinate gauge. Variable-`F` comparisons
would require a separately specified feature/stratification contract.

## Methods and calibration

`analyze.py` reports global PCA rank/participation ratio, local-PCA
participation ratios, TwoNN, Levina--Bickel kNN maximum-likelihood estimates
over a k range, exploratory correlation-dimension scaling windows, kNN graph
components, and empirical mass--radius/medoid summaries. It also creates a
split-calibrated radius around an independent training subset: its independent
holdout frequency is an empirical law-mass check only when rows are
exchangeable draws from one declared law. Separately, it draws independent
probes from an explicit coordinatewise training bounding-box reference measure
`Q` (expanded by 10% of each observed coordinate range). The fraction of `Q`
inside that same radius is **chart coverage under that arbitrary `Q`**, not law
mass, support coverage, or a density body. The synthetic suite
contains a known 2-plane, noisy circle, curved line, separated mixture,
duplicate-contaminated plane, anisotropic noisy line, and a square with
boundary. This makes known bias modes visible: PCA follows variance; duplicates
invalidate log-ratio estimators; mixtures split graphs; boundary and noise move
local/scaling estimates; a short apparent scaling window is not topology.

Persistent homology is explicitly deferred: this packet declares only NumPy,
and no lightweight PH dependency plus a filtration/coefficients/noise contract
has been calibrated. Connectivity is a neighborhood diagnostic, not a
topological result. UMAP/t-SNE are deliberately not included as evidence.
Generator-map Jacobian/local-rank and stable density-cluster trees are also
deferred: the line has no common differentiable generator/rejection contract,
nor a selected density/noise/level-set contract and per-law sample size.

## Reproduction

Run the synthetic calibration and tests from this directory:

```bash
uv run --script analyze.py --out-dir /tmp/generator-distribution-dimension/calibration
uv run --with numpy --script test_packet.py
```

For the deliberately small real generator-zoo smoke, pass the hydrated source
artifact from its owner worktree (the source hash is bound into the report):

```bash
uv run --script analyze.py \
  --factor-shapes /workspaces/msc-math/.worktrees/generator-transfer/experiments/sys-datascience/methods/generator-zoo-smoke/artifacts/factor-shapes.jsonl \
  --out-dir artifacts/generator-zoo-smoke
```

The real rows are a smoke only. In particular, many fixed-`F`/view strata are
below the estimator's k-range sample requirement, so their range/interval is a
diagnostic of insufficiency rather than a dimension estimate. Larger `n` is
valuable only after preserving the same law, selection and quotient contract:
it supports k- and resampling-stability checks within a stratum, distinguishes
local boundary/mixture behavior, and tests whether graph connectivity persists.
It also narrows holdout uncertainty for the law-mass radius and Monte Carlo
uncertainty for the declared-`Q` chart fraction. It cannot decide a quotient
left unspecified here or turn chart coverage into support coverage.

Agreement is informative only when it survives the synthetic analogue of the
real failure mode and spans a reasonable k/radius range. A PCA value agreeing
with a kNN value because both respond to anisotropic noise is not corroboration.
Mass-radius outputs describe finite sampled concentration around points or the
empirical medoid; they do not identify a population high-density body,
confidence region, or support. No output supports a claim about `sys`.
