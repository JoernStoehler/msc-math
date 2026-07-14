# Generator quality atlas

## Decision and scope

This copy-local method packet asks whether a proposed planar factor generator is
similar enough for controlled transfer, expands geometric coverage, or merely
changes side counts. It keeps six purposes separate: controlled-transfer
similarity, coverage expansion, within-law diversity, combinatorial breadth,
compute/acceptance, and naturalness/provenance. There is deliberately no overall
quality score.

The retained synthetic artifact is a semantic and regression fixture, not
evidence about the current or any proposed random-polytope law. A future
generator should copy the analyzer into its method packet or emit the input
schema below; it should not import unstable generator code into this packet.

## Primary input contract

Input is newline JSON. Every nonempty line is a `factor-shape-row-v1` object
with these required fields:

- `schema`: exactly `factor-shape-row-v1`;
- `sample_id`: unique nonempty string;
- `law`: nonempty generator-law/version label;
- `population`: optional nonempty label identifying the actual distribution
  compared by the atlas. Use a stable `law + knob setting` label whenever one
  law has multiple parameters; otherwise the analyzer falls back to `law`;
- `side_count`: integer at least three;
- `vertices_ccw`: exactly `side_count` finite `[x,y]` pairs, in strict convex
  CCW cyclic order. The analyzer also accepts the fixture-only legacy alias
  `vertices`; new generators should emit `vertices_ccw`.

Optional fields are retained for separate descriptive views: `accepted`,
`attempts`, `rejections`, `generation_ms`, `validation_ms`, `target_ms`,
`provenance`, `f_vector`, `facet_count`, `pair_bucket`, and fields beginning
with `combinatorial_`. Put generator version, seed, selection rule, and other
source identity in `provenance` or additional row fields. Rejected attempts
without a valid polygon do not belong in this shape file; preserve them in the
generator owner's acceptance artifact and, when useful, repeat aggregate
attempt/rejection counts on accepted shape rows with documented semantics.

The coarse `alternative-generator-smoke-row-v2` rows contain scalar summaries,
not vertices. They cannot support this metric and are intentionally not adapted
as if they could. A generator owning those rows must emit a separate
`factor-shape-row-v1` file from its reviewed polygon representation.

## Geometry and metrics

For each polygon the analyzer validates positive nondegenerate area and strict
CCW convexity. It then computes the numerical Steiner point

`s(K) = (1/pi) integral h_K(theta) u(theta) dtheta`

from the exact exterior-angle formula for a polygon, subtracts it, divides
coordinates by the square root of polygon area, and samples the centered
support function on a declared uniform angular grid. The report also evaluates
the declared-grid support integral and records its Euclidean error from the
exact formula. It does not delete Fourier modes of `log h`.

The primary distance is root-mean-square support difference minimized over all
grid-seeded continuous rotation refinement. The report also gives an
independently rotation-minimized maximum support difference, a Hausdorff-like
view. Thus the metric quotients translation, positive scale, and rotation up to
floating-point optimization error. It does not quotient reflections. The
support integrals remain sampled, so both distances are numerical
approximations, not certified bounds. Defaults are 256 support angles and 4096
angles for the diagnostic numerical Steiner integral.

All comparisons are stratified by `side_count`. For every law and stratum the
report includes pairwise and nearest-neighbor diversity, duplicate pairs, and a
classical-MDS participation-ratio effective dimension. The accompanying
negative-eigenmass fraction exposes failure of the quotient distance to embed
cleanly as Euclidean at the observed sample size.

Against the named baseline law, it reports:

- empirical two-sample energy-like V-statistic using the L2 quotient distance;
- support-vector centroid distance after independently aligning every sample
  to the baseline empirical medoid (the named baseline-medoid gauge);
- fraction outside a finite-sample baseline central body.

The energy-like statistic is symmetric and self-zero, but the rotation-quotient
metric is not established to be of negative type, so the statistic is not
claimed to be nonnegative or identifying. The central body is the ball about
the baseline empirical medoid whose radius
is the sorted baseline medoid distance at index `ceil(q*m)-1`, with default
`q=0.9` and baseline count `m`. A candidate is outside exactly when its medoid
distance is strictly larger than that radius. This is a transparent descriptive
rule, not a calibrated population confidence region. Counts below five are
marked `small-sample`; duplicates, absent baseline laws, laws missing from a
side-count stratum, and degenerate support grids remain visible.

Every fixed-`n` convex planar polygon has the same combinatorial type. Optional
combinatorial fields are counted, but they cannot establish within-`n`
combinatorial breadth. The report separately records the side-count allocation
among analyzed accepted-shape rows and its total-variation difference from the
baseline accepted-row allocation. This is a pipeline/allocation diagnostic:
imposed sampling budgets, which side counts a population supports, and bounded
rejection all affect it. It is not an estimate of a natural generator-law
side-count distribution or combinatorial breadth. Naturalness is not inferred
from geometric distance; provenance remains descriptive.

## Reproduce the retained control

From this directory:

```bash
uv run --script analyze.py \
  --write-synthetic-fixture fixtures/synthetic.jsonl \
  --input fixtures/synthetic.jsonl \
  --out-dir artifacts
uv run --script test_analyze.py
```

`artifacts/report.json` is the detailed generated source. `artifacts/atlas.tsv`
is the compact investigation table, ordered by side count and law, for exact
repeated comparisons. The fixture deliberately contains translation,
grid-rotation, and scale copies that should have zero distance, plus narrow and
broad affine shape families. Regenerate both artifacts; do not hand-edit them.

The retained table should make one regression decision easy: the broad control
must show materially greater within-law diversity and baseline displacement
than the narrow control, while the baseline transformation copies remain
duplicates. It must not be read as generator evidence.

The retained generator-zoo smoke is reproduced separately after its producer:

```bash
uv run --script analyze.py \
  --input ../generator-zoo-smoke/artifacts/factor-shapes.jsonl \
  --baseline-law 'current-baseline[delta=0.2]' \
  --out-dir artifacts/generator-zoo
```

This real report is intentionally underpowered: it checks the pipeline and
suggests which populations merit a larger geometry-only sample. It is not a
law-ranking result.

## Interpretation and reopen boundary

This packet is ready as a copy-editable research assessment surface after its
focused tests and byte-deterministic regeneration check pass. A real-generator
report remains descriptive at its observed laws, side counts, sample sizes,
and selection rule. It does not prove homogeneity, naturalness, population
coverage, or downstream `sys` performance.

Reopen the metric design before consequential use if rotation-grid error is
comparable to the observed law separation, negative MDS eigenmass is material,
the baseline has fewer than five independent samples in a stratum, acceptance
metadata is repeated with incompatible semantics, or reflections should become
an explicit scientific equivalence. Larger samples should add resampling or
independent validation rather than treating these finite-sample summaries as
inferential tests.
