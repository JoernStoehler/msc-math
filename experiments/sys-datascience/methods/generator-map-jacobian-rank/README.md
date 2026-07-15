# Generator-map local Jacobian rank

This target-free packet asks whether each copy-local planar factor generator is
locally full-dimensional in the similarity-quotiented polygon-body chart, whose
generic dimension is `2n-4`, or lies in a smaller structural family. It uses no
capacity, `sys`, UMAP, or t-SNE.

The formulas and conditioning boundaries are copied from the adjacent reviewed
generator-zoo owner. NumPy's pinned PCG64 stream supplies deterministic linked
latent bases; it reproduces the named probability laws, not the Rust producer's
ChaCha byte streams. This distinction is immaterial to a generic local-rank
claim but prohibits row-identity or exact-replay claims across the two owners.

The local chart subtracts the vertex mean, divides by RMS radius, removes a
spatial rotation, and quotients cyclic/reversal vertex labels. A base chooses a
deterministic dihedral representative; every central perturbation uses that
same linked local label. Exact polygon stabilizers are recorded as benign only
when their candidate chart vectors coincide. Nonidentical numerical ties,
nonconvexity, inactive facets, mutation clipping/sorting changes, and primal
hull active-set changes fail closed.

Implemented populations and agent-derived expected generic ranks are:

- IID angles/IID supports: `2n-4`;
- equal-support Dirichlet gaps (`alpha=1,4,16`): `n-1`;
- exact regular equal-support control: `0`;
- zonogon with `n/2` segment directions and lengths: `n-2`;
- four-step regular mutation: `2n-4` on the retained all-unclipped stratum;
  clipping also has open saturated lower-rank strata (the all-angle-saturated
  regression has rank `n-3`), so this is not a law-wide generic-rank claim;
- primal hull: `2n-4` only on a fixed `n`-vertex hull active-set stratum.

These counts are mathematical local-parameter arguments reviewed by an agent,
not Jörn-reviewed theorems. The analyzer retains singular spectra at five
central-difference steps and ranks at four relative thresholds. Exact linear
calibrations cover full rank, lower rank, duplicate outputs, gauge directions,
and a near-singular map whose reported rank must depend on threshold.
The primary relative threshold is `1e-6`; the report retains `1e-7`, `1e-8`,
and `1e-10` alternatives so threshold sensitivity remains visible.

Source and artifacts are committed in separate phases. Reproduce from this
directory:

```bash
uv run --script analyze.py --out-dir artifacts
uv run --with numpy==2.5.1 --script test_packet.py
uv run --with numpy==2.5.1 --script test_reproducibility.py
```

The deterministic report binds the source-only revision, analyzer SHA-256,
NumPy version, seeds, steps, and thresholds. Wall time is stdout-only.

A stable matched rank supports only a local statement about this implementation
at the retained generic bases. It does not establish global support, density,
topology, rare-mode mass, chart coverage, naturalness, or target value. A
failed perturbation is a boundary/discrete-state failure, not evidence of low
dimension. Strata remain separate; there is no global quality score.
