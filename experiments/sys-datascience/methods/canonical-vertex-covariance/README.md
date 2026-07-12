# Canonical extreme-vertex covariance eccentricity

This bounded diagnostic asks whether the symplectic eccentricity
`rho = nu2 / nu1` of the centered, uniform covariance of a polytope's
canonical distinct primal extreme vertices is associated with `sys` in the
retained sample, including descriptive bucket/vertex-count controls.

`analyze.py` deliberately does not use arbitrary point clouds. It decodes the
producer's exact `vertices_rational`, deduplicates rationally coincident entries,
requires numerical full dimension, and rejects a list containing any
non-extreme point by a convex-hull check. The output records the covariance
condition and symplectic-pairing error for each accepted row.

Run the implementation tests:

```bash
uv run --script experiments/sys-datascience/methods/canonical-vertex-covariance/analyze.py --self-test
```

Run the retained-data diagnostic (no new `sys` evaluations):

```bash
uv run --script experiments/sys-datascience/methods/canonical-vertex-covariance/analyze.py
```

Artifacts in `artifacts/current/` are generated, not hand edited.
`provenance.json` records input content hashes, tracked LFS object identities,
source revision/input status, commands, and output hashes. All four retained
inputs must be hydrated rather than Git-LFS pointer files; otherwise the
analysis stops rather than treating a pointer as evidence.

Selection protocol and claim boundary: ρ was selected post-target, after the
retained `sys` values were already available. Every association in the report
therefore reuses the full table; none is frozen validation. Per-stratum
Spearmans and pooled within-stratum rank-residualized correlations are
descriptive controls, not partial/causal estimates. This packet cannot
establish a symplectic-geometric mechanism, generalization beyond these
random/product generators, or a credible proposer. It should become a
persistent line only if it motivates a separate pre-target evaluation.
