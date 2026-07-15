# Independent rare-region hit curves

This target-free packet measures online time-to-first-hit and generator-attempt
cost for explicit factor laws. It is built from accepted commit
`3f09eeebbcaae731d493317b63fc6ece127e804d` in worktree
`.worktrees/rare-hit-curves-next` (branch `research/rare-hit-curves-next`). The
source worktree name in the launch note was `generator-atlas-multiseed`; the
actual accepted checkout is `multiseed-atlas-confirmation`, at the same commit.

The three independent producer seeds have fixed roles: `20260716` is the pilot,
`20260717` confirmation-1, and `20260718` confirmation-2. The pilot pools rows
only within each side count to freeze lower/upper 0.10 quantile cells for
anisotropy, isoperimetric ratio, support roughness, and central-symmetry
residual. A separate support-shape region uses the upper 0.90 quantile of
pilot nearest-neighbour rotation-quotiented support distance. Confirmation
rows are read in the producer's original accepted order; no confirmation row
sets a threshold.

## Reproduce

From this directory, with the factor producer artifacts present at the sibling
atlas packet:

```bash
uv run --script rare_hit_curves.py \
  --pilot-input ../generator-distribution-atlas-confirmation/artifacts/raw/seed-20260716/core/factor-shapes.jsonl \
  --pilot-input ../generator-distribution-atlas-confirmation/artifacts/raw/seed-20260716/zonogon/factor-shapes.jsonl \
  --confirmation-input ../generator-distribution-atlas-confirmation/artifacts/raw/seed-20260717/core/factor-shapes.jsonl \
  --confirmation-input ../generator-distribution-atlas-confirmation/artifacts/raw/seed-20260717/zonogon/factor-shapes.jsonl \
  --confirmation-input ../generator-distribution-atlas-confirmation/artifacts/raw/seed-20260718/core/factor-shapes.jsonl \
  --confirmation-input ../generator-distribution-atlas-confirmation/artifacts/raw/seed-20260718/zonogon/factor-shapes.jsonl \
  --producer-report ../generator-distribution-atlas-confirmation/artifacts/raw/seed-20260716/core/factor-only-report.json \
  --producer-report ../generator-distribution-atlas-confirmation/artifacts/raw/seed-20260716/zonogon/factor-only-report.json \
  --producer-report ../generator-distribution-atlas-confirmation/artifacts/raw/seed-20260717/core/factor-only-report.json \
  --producer-report ../generator-distribution-atlas-confirmation/artifacts/raw/seed-20260717/zonogon/factor-only-report.json \
  --producer-report ../generator-distribution-atlas-confirmation/artifacts/raw/seed-20260718/core/factor-only-report.json \
  --producer-report ../generator-distribution-atlas-confirmation/artifacts/raw/seed-20260718/zonogon/factor-only-report.json \
  --out-dir artifacts/analysis
python3 -m unittest -v test_rare_hit_curves.py
```

The analyzer uses Python 3.12 and `numpy==1.26.4`. `report.json` records input
hashes, source revision/tree and dirty state, analyzer/test/README hashes,
seed roles, target-field scan, and transitive provenance for all six data shards
and six producer reports. Runtime is deliberately not serialized.
`pilot-regions.json` is the frozen selection boundary. `hit-curves.tsv`
contains aggregate accepted-prefix survival over exactly the two confirmation
seed streams per law/side, with very wide Wilson finite-sample intervals;
`stream-summary.tsv` keeps per-stream first-hit/censoring, accepted rows,
attempts, rejections, and exhaustion; and `stratum-findings.tsv` labels each
law/side region as replicating on both confirmation seeds, partial, or not
reobserved under right censoring.

## Measured packet

Each seed contributes 545 accepted rows from 552 requests. Seven primal-hull
triangle requests exhaust the 128-attempt cap. Accepted-row attempts plus the
seven charged exhaustions total 1,441 attempts per seed (896 rejections); the
producer reports measure 2.104945 ms, 2.601498 ms, and 2.570837 ms generation
time for the three seeds. The confirmation panel has 1,090 rows, 27
frozen regions, and 414 region/stream pairs: 174 have a first hit and 240 are
right-censored at 24 accepted rows.

At stratum level, 80 regions replicate on both confirmation seeds, 14 hit only
one confirmation seed, and 113 pilot-defined regions are not reobserved on both
confirmation seeds under right censoring. They are not zero-probability or
universal-artifact conclusions. These labels are in the generated TSV, not a
single aggregate law ranking. All scalar strata met the declared
finite-support-overlap eligibility threshold; generator knobs and side counts
remain separate.

The synthetic controls recover known Bernoulli hit rates (`p=0.10` mean
0.10092 over five streams and `p=0.01` mean 0.01032), preserve duplicate-stream
warnings, retain no-hit censoring, and show the expected first-hit movement
under seeded order permutations of the same multiset.

## Interpretation boundary

Allowed uses are finite-panel accepted-prefix curves, attempt/rejection/
exhaustion accounting, generator-versus-validation cost separation, and named
law-by-side contrasts when the frozen region and overlap field are visible.
These are descriptive factor-stream results. They do not estimate natural-law
probabilities, establish universal geometric extremes, pool facets or side
counts, rank laws by one score, transfer to `sys`/capacity/targets, infer a
mechanism, or authorize an online sampler. A censored stream is not evidence of
zero hit probability; a region with no confirmation event is not reobserved in
the two right-censored confirmation streams, not a universal negative.
