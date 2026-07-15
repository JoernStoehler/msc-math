# Generator support-process atlas

This target-free packet asks one bounded question: on the same random normal
fans, how do facetwise IID support variation and low-frequency correlated
support fields change factor geometry, acceptance, diversity, and tails?

It fills a coverage gap between the reviewed support-law implementations in
`alternative-generator-smoke` at source commit `6cc64c8f` and the later scaled
shape atlas. The formulas are copied locally because experiment interfaces are
not stable enough to import.

## Frozen design

The retained panel uses side counts `4,6,8`, deterministic independent seeds
`1201,2203,3209`, and 48 frozen IID-uniform normal fans per seed/side-count.
Each fan has one immutable `fan_id`. Each arm has one immutable latent ID on
that fan; neither the fan nor latent is redrawn after failure.

The six arms are exhaustive for this packet:

- equal supports `h_i=1`;
- current supports `h_i iid Uniform[0.8,1.2)`;
- clipped-Gaussian centered log supports at `sigma=0.1` and `sigma=0.2`;
- inverse-frequency Fourier log fields with `R=2` and `R=3`, centered and
  rescaled to empirical sampled log-support SD `0.1`.

`sigma=0` is tested as the equal-support identity and is not emitted as a
separate population. Accepted polygons are area-normalized and translated by
their area centroid before shape distances. A 64-direction centered support
signature provides the common distance view.

Two sampling objects remain separate:

1. The arm-marginal table contains every requested arm attempt on every fan,
   including failures. Its acceptance rates describe this finite panel under
   each original arm law and the shared fan panel.
2. The complete paired subset contains only fan IDs on which all six arms
   succeeded. It supports same-fan comparisons but is explicitly conditioned
   on every arm succeeding and is not an estimate of the original marginal
   laws.

## Measures and predeclarations

Each accepted attempt records actual support CV, sampled log-support SD,
cyclic log-support adjacency correlation and roughness, angular-gap CV,
isoperimetric ratio, sampled width anisotropy, maximum centered vertex radius,
and centered vertices/support signatures. Same-fan equal-support distances use
support L2, support L-infinity, and corresponding-vertex RMS views.

The analyzer reports marginal and complete-conditioned summaries separately:
tail quantiles, source-to-arm paired distances, within-arm support-signature
diversity, and directed nearest-cross overlap with same-fan targets excluded.
Every summary is stratified by side count and seed.

Before generation, the matched-CV comparisons were fixed as IID
`sigma=0.1` versus each of smooth `R=2` and `R=3`. Matching passes when the
accepted mean support CVs differ by at most `0.02`; a failure is reported and
is not repaired by post-hoc amplitude tuning. The sigma ladder is fixed as
equal, IID `0.1`, IID `0.2`; median support CV, log roughness, source support
distance, and width anisotropy are checked for nondecreasing behavior in both
sampling objects.

## Commands and artifacts

Source validation:

```text
python3 -m unittest experiments/sys-datascience/methods/generator-support-process-atlas/test_run.py
python3 -m py_compile experiments/sys-datascience/methods/generator-support-process-atlas/run.py
```

Retained run, executed only from the clean source commit:

```text
python3 experiments/sys-datascience/methods/generator-support-process-atlas/run.py
cd experiments/sys-datascience/methods/generator-support-process-atlas/artifacts
sha256sum -c checksums.sha256
```

`fans.jsonl` owns frozen normal fans; `attempts.jsonl` owns every marginal
attempt and failure; `complete-fans.jsonl` owns the conditioned paired IDs.
The remaining JSONL files are generated comparison summaries. `report.json`
contains the design, counts, calibration dispositions, and interpretation
boundary, including zero-row complete strata rather than silently omitting
them. `manifest.json` pins source commit/tree, Python version, source
hashes, and artifact hashes. No volatile timing is retained.

The producer captures repository provenance before artifact creation and
fails closed after writing diagnostic outputs if tracked source is dirty or
row/failure/linkage contracts fail. The packet uses Python standard library
only; the pinned Python implementation/version and the local Box--Muller
normal algorithm are recorded for replay.

## Interpretation boundary

Allowed: finite-panel target-free statements about these explicit support
processes, their geometry, conditioning, and which later atlas comparisons may
be redundant.

Prohibited: `sys` or capacity claims, population support/density claims, law
ranking, causal claims, or theorem-level conclusions. A clean smoke or a
complete paired contrast does not strengthen those prohibited claims.
