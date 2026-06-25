# Sys-Datascience Feature-Space Coverage Ledger

Purpose: control surface for feature-space closure in the trusted random/product
`sys` datascience slice. This ledger measures feature coverage by object family,
not by scalar-column count. It should be updated whenever prepare features,
method inputs, dataset filters, or thesis-facing datascience claims change.

Epistemic status: initial ledger created from current code inspection, current
method-packet READMEs, and the adversarial review of the `/tmp` feature
shortlist. Existing prepare columns are treated as candidate code paths until
their object definition, invariance, symmetry, scaling, truncation, numerical
stability, and method use are audited.

## Status Vocabulary

- `absent`: no current reusable implementation found.
- `candidate-existing-unaudited`: current code has similarly named columns or
  method logic, but this ledger does not yet trust them as thesis evidence.
- `audited-existing`: current implementation has been checked against the
  ledger row's validity requirements and can be used with the recorded caveats.
- `implemented-new`: implemented after this ledger row was created.
- `rejected`: intentionally not pursued, with reason.
- `deferred`: worthwhile but lower value than current work, with reopen trigger.

## Current High-Priority Rows

### Omega Matrix Invariants

- feature object: skew matrix `Omega_ij = omega_0(a_i,a_j)` on the dual facet
  list.
- thesis question: do cheap symplectic matrix invariants explain or predict the
  high `sys` tail in trusted random/product rows?
- current status: `implemented-new` for Frobenius norm, spectral norm, stable
  rank, and numerical rank/nullity at threshold `1e-10`; `absent` for
  characteristic-polynomial coefficients and Pfaffian-derived features.
  Current-schema scratch tables have been regenerated with these columns;
  reviewed retained-table closure remains pending.
- valid transforms: singular values; numerical rank/nullity; Frobenius norm;
  spectral norm; stable rank; characteristic-polynomial coefficients if stable;
  `abs(pfaffian)` or `pfaffian^2` for even facet count.
- invalid/risky transforms: raw matrix entries; raw Pfaffian, because it changes
  sign under odd facet permutations; principal minors indexed by arbitrary
  facet labels; reading numerical rank/nullity as rich independent invariants.
- invariance/symmetry: spectra/rank/norm features are invariant under facet
  relabeling `Omega -> P Omega P^T`; raw magnitudes still depend on the chosen
  support representative under translations, so this row is not translation
  canonized unless a separate representative choice is added.
- numerical caveat: since `Omega = A^T J A` for four-dimensional dual vectors,
  its rank is bounded by `4`; nullity is therefore strongly facet-count-driven.
  The current rank/nullity columns use the explicitly named absolute threshold
  `1e-10` and should be treated as diagnostics unless a relative-threshold
  variant is added.
- data/leakage status: pre-capacity geometry feature; applicable to
  random/product rows.
- methods expected to consume it: `statistical-associations`,
  `projection-structure`, `prediction-ranking`, and possibly
  `random-tail-eda`.
- evidence commands/artifacts: implemented in `prepare/features_omega.rs`,
  `prepare/features.rs`, and `prepare/rows.rs`; `cargo check -p
  exp-sys-landscape` passed; `cargo fmt --check -p exp-sys-landscape` passed
  after formatting; temp-output `experiments/sys-datascience/pipeline.local.sh smoke` passed and produced
  smoke rows containing the new omega matrix columns. The shared method feature
  selector was updated to include `omega_*` columns as geometry features, and a
  selector smoke on `/tmp/tmp.tgQtP6g6T7/tables` found representative new omega
  columns among `105` geometry features.
- disposition: implementation, temp-output schema smoke, full scratch rebuild,
  and full method rerun passed; method/statistics review remains pending.
- reopen triggers: translation canonization changes; omega computation changes;
  a method promotes an omega matrix invariant to candidate-proposer evidence.

### Omega Sign-Orientation Invariants

- feature object: sign/orientation structure of `omega_0(a_i,a_j)`, optionally
  restricted by facet intersections/ridges.
- thesis question: does the symplectic sign pattern contain structure lost by
  absolute-value omega summaries?
- current status: `implemented-new` for all-pair omega-sign out-degree summary
  statistics; `candidate-existing-unaudited` for zero fractions and transition
  density/out-degree summaries in `features_omega.rs`; `absent` for sorted full
  degree multisets, triad/cycle counts, and richer restricted sign summaries.
  Current-schema scratch tables have been regenerated with the new columns;
  reviewed retained-table closure remains pending.
- valid transforms: zero fractions; permutation-invariant orientation graph
  summaries; sorted out-degree/in-degree multisets; triad/cycle counts;
  summaries restricted to facet-intersection or ridge pairs.
- invalid/risky transforms: raw signed entries; "positive fraction over
  `i < j`"; any signed statistic depending on arbitrary facet order.
- invariance/symmetry: sign graph is invariant under linear `Sp(4)` and
  positive support rescalings; scalar summaries must be invariant under facet
  relabeling.
- data/leakage status: pre-capacity geometry feature; applicable to
  random/product rows.
- methods expected to consume it: `statistical-associations`,
  `projection-structure`, `prediction-ranking`.
- evidence commands/artifacts: new out-degree summaries implemented in
  `prepare/features_omega.rs`, `prepare/features.rs`, and `prepare/rows.rs`;
  `cargo check -p exp-sys-landscape` passed; `cargo fmt --check -p
  exp-sys-landscape` passed after formatting; temp-output `experiments/sys-datascience/pipeline.local.sh smoke` passed and produced
  smoke rows containing the new omega-sign out-degree columns. The shared
  method feature selector was updated to include `omega_*` columns as geometry
  features. Existing zero/transition fields still need audit before use as
  coverage.
- disposition: implementation, temp-output schema smoke, full scratch rebuild,
  and full method rerun passed for the first all-pair sign summary slice;
  method/statistics review remains pending.
- reopen triggers: canonical facet ordering is introduced; transition graph
  semantics change; method evidence suggests sign-orientation structure matters.

### Normalized Omega Alignment

- feature object: normalized pair feature
  `omega_0(a_i,a_j)/(||a_i|| ||a_j||)`.
- thesis question: does relative symplectic alignment in the chosen Euclidean
  representative explain high `sys` rows better than raw omega magnitudes?
- current status: `implemented-new` for all-pair and ridge-restricted
  mean/std/min/max of absolute normalized omega; `absent` for quantiles and
  concentration summaries. Current-schema scratch tables have been regenerated
  with these columns; reviewed retained-table closure remains pending.
- valid transforms: all-pair and ridge-restricted absolute summaries; quantiles
  and concentration over complete pair/ridge sets; sign-sensitive summaries only
  through permutation-invariant graph/matrix summaries.
- invalid/risky transforms: claiming full `Sp(4)` invariance; order-dependent
  signed summaries.
- invariance/symmetry: cancels global scale and positive rescaling of the two
  vectors entering a pairwise comparison. It is invariant under orthogonal
  symplectic changes of coordinates, but not under general `Sp(4)` because
  Euclidean norms change, and it is not a translation-canonized feature.
- data/leakage status: pre-capacity geometry feature; applicable to
  random/product rows.
- methods expected to consume it: `statistical-associations`,
  `projection-structure`, `prediction-ranking`.
- evidence commands/artifacts: implemented in `prepare/features_omega.rs`,
  `prepare/features.rs`, and `prepare/rows.rs`; `cargo check -p
  exp-sys-landscape` passed; `cargo fmt --check -p exp-sys-landscape` passed
  after formatting; temp-output `experiments/sys-datascience/pipeline.local.sh smoke` passed and produced
  smoke rows containing the new normalized omega columns. A tiny
  `statistical-associations` smoke against that temp table ran with
  `--max-features 20 --permutations 2`; it verifies schema consumption only,
  not thesis evidence.
- disposition: implementation, temp-output schema smoke, full scratch rebuild,
  and full method rerun passed; non-`Sp(4)` caveat review and method/statistics
  review remain pending.
- reopen triggers: translation canonization or Euclidean representative changes.

### Two-Face Symplectic Area Tails

- feature object: volume-normalized symplectic areas of reconstructed primal
  two-face polygons.
- thesis question: do natural two-dimensional symplectic face areas explain the
  high `sys` tail?
- current status: `implemented-new` for incidence-cycle ordering diagnostics,
  median, q90, q95, and top-3 share; `candidate-existing-unaudited` for the
  older mean/std/min/max/sum/max-share and small-threshold fractions until full
  retained-data review. Current-schema scratch tables have been regenerated
  with the new columns; reviewed retained-table closure remains pending.
- valid transforms: median; quantiles; top-k shares; concentration; small-area
  fractions; comparison to Euclidean two-face area if added.
- invalid/risky transforms: using area tails without validating two-face
  reconstruction and vertex ordering; treating a degenerate fallback order as
  reliable geometry.
- invariance/symmetry: primal symplectic area is translation invariant and
  linear-symplectic invariant; dividing by `sqrt(volume)` removes global scale
  in dimension four.
- data/leakage status: pre-capacity geometry feature; applicable to
  random/product rows.
- methods expected to consume it: `random-tail-eda`,
  `statistical-associations`, `projection-structure`, `prediction-ranking`.
- evidence commands/artifacts: `prepare/features_face_symplectic.rs` now orders
  two-face vertices by the incidence-induced cycle instead of a coordinate-angle
  fallback, records ordered-face count, ordering-failure count, and ordered
  fraction, and computes median/q90/q95/top-3 share over successfully ordered
  faces. `cargo check -p exp-sys-landscape` passed; `cargo fmt --check -p
  exp-sys-landscape` passed; temp-output `experiments/sys-datascience/pipeline.local.sh smoke` passed. The smoke table
  had `0` ordering failures and ordered fraction `1.0` for all `4` rows. Shared
  method feature selection excludes ordering-diagnostic columns from geometry
  inputs and excludes `ridge_symp_area_volnorm_*` inputs whenever any loaded row
  has `ridge_symp_area_ordered_fraction != 1.0`.
- disposition: implementation, temp-output schema/ordering smoke, full scratch
  rebuild, and full method rerun passed; method/statistics review remains
  pending.
- reopen triggers: face reconstruction code changes; degenerate two-face cases
  appear; thesis wording relies on area-tail interpretation.

### Product And Source Parameter Stratification

- feature object: generator/source metadata, including generic `facet_count`,
  random/product source, product `(k,m)` bucket, `h_min`, `h_max`, seed/attempt,
  and product `bounces`.
- thesis question: are random/product conclusions stable across obvious
  generator parameters, or are mixture-level statements hiding source effects?
- current status: `implemented-new` for optional provenance fields
  `sample_seed`, `sample_attempt`, `sample_h_min`, `sample_h_max`,
  `product_k`, `product_m`, and `product_bounces`; availability depends on the
  producer row type and loader path. Canonical old producer rows expose height
  range and product `(k,m,bounces)` but not seed/attempt; newer run-local
  producer metadata can expose seed/attempt. Current-schema scratch tables have
  been regenerated with these columns; reviewed retained-table closure remains
  pending.
- valid transforms: explicit join/encoding from producer/provenance rows;
  source/facet-count/product-bucket stratification; within-stratum summaries and
  residuals; grouped validation.
- invalid/risky transforms: treating source metadata as intrinsic geometry;
  mixture-level thesis claims without checking source/facet-count/product
  strata; using metadata as a clean proposer without leakage discussion.
- invariance/symmetry: generator-dependent, not geometric invariant.
- data/leakage status: pre-capacity metadata; applicable to trusted
  random/product rows if joined from producer/provenance tables.
- methods expected to consume it: `random-tail-eda` consumes availability and
  stratification diagnostics; `statistical-associations` now reports
  categorical source/provenance factor tests; `prediction-ranking` now reports
  metadata-only source/facet/product baselines. `projection-structure` still
  needs an explicit provenance join if metadata overlays are desired.
- evidence commands/artifacts: producer schemas in `produce/rows.rs`; provenance
  loaders in `prepare/load_caches.rs` and `prepare/prepare.rs`; provenance
  schema in `prepare/rows.rs` and `prepare/write_database.rs`; random-tail EDA
  availability summary in `methods/random-tail-eda/analyze.py`;
  categorical-factor summaries in `methods/statistical-associations/analyze.py`;
  metadata-only baselines in `methods/prediction-ranking/analyze.py`. `cargo
  check -p exp-sys-landscape` passed; `cargo fmt --check -p exp-sys-landscape`
  passed; temp-output `experiments/sys-datascience/pipeline.local.sh smoke` passed. Smoke provenance
  rows contained explicit `sample_h_min`, `sample_h_max`, `product_k`,
  `product_m`, and `product_bounces` where available. Temp method smokes
  verified EDA availability diagnostics, association factor-test output, and
  metadata-only prediction baselines.
- disposition: implementation, temp-output schema/method smokes, full scratch
  rebuild, and full stratified method rerun passed; method/statistics review
  remains pending.
- reopen triggers: generator parameters change; new random family added; thesis
  wording makes source- or parameter-stability claims.

### Product-Likeness / Block-Structure Diagnostics

- feature object: exact or approximate decomposition of generic rows into
  product-like coordinate/facet blocks.
- thesis question: do high `sys` rows exhibit product-like or candidate-proposer
  structure beyond known product-source labels?
- current status: `absent`.
- valid transforms: known-product labels; approximate facet partition score;
  block off-diagonal omega/Gram mass; coordinate-block sparsity after known
  product orientation; null/permutation baselines for partition scores.
- invalid/risky transforms: overfitting a best partition without a null model;
  using product-source labels as proof of generic product-likeness.
- invariance/symmetry: depends on chosen coordinate/product convention unless
  formulated invariantly; must account for facet permutation.
- data/leakage status: pre-capacity geometry/metadata feature if computed from
  row geometry and source metadata; applicable to random/product rows.
- methods expected to consume it: `statistical-associations`,
  `projection-structure`, `prediction-ranking`, possibly a future product
  structure packet.
- evidence commands/artifacts: none yet.
- disposition: prototype after omega/metadata audit if value remains high.
- reopen triggers: high-tail rows concentrate in product buckets; model
  importances point to product/source features; Jörn asks for product-structure
  explanation.

### Translation-Canonized Robustness

- feature object: feature families recomputed after choosing a translation
  convention for the support representation.
- thesis question: do dual-coordinate and omega-magnitude findings survive
  support-representative changes?
- current status: `absent` on main. A prior spike outside this branch proposed
  translating by an interior center `c` via `a_i' = a_i/(1 - <a_i,c>)`, but no
  default canonization is merged.
- valid transforms: parallel centered feature families; rank-correlation and
  method-importance comparisons between uncentered and centered variants.
- invalid/risky transforms: silently replacing current columns before center
  choice review; treating vertex-average center as a settled geometric center.
- invariance/symmetry: target is translation robustness; center choice controls
  mathematical strength. Volume normalization should remain explicit.
- data/leakage status: pre-capacity feature transformation; applicable to
  random/product rows after center computation.
- methods expected to consume it: all geometry-feature methods after
  canonization choice.
- evidence commands/artifacts: none in current branch.
- disposition: defer until first omega/area/metadata round is planned, unless
  dual-coordinate conclusions become thesis-critical sooner.
- reopen triggers: strong findings depend on support-representative-sensitive
  dual magnitudes; a center convention is approved.

### Transition-Digraph Extras

- feature object: directed graph from facet intersections plus omega signs.
- thesis question: does symplectic-incidence graph structure explain high
  `sys` rows or orbit compatibility?
- current status: `candidate-existing-unaudited` for density, bidirectionality,
  and out-degree summaries in `features_omega.rs`; `absent` for SCCs,
  cycle/triad motifs, graph spectra, and orbit-compatibility summaries.
- valid transforms: SCC count/largest share; source/sink counts; cycle/triad
  counts; spectra if cost/interpretability is acceptable; orbit-overlap
  summaries only in post-capacity interpretation packets.
- invalid/risky transforms: generic graph feature dumping without a question;
  mixing orbit-selected summaries into clean proposer features.
- invariance/symmetry: graph summaries must be facet-permutation invariant;
  graph construction depends on omega signs and facet intersections.
- data/leakage status: pre-capacity for graph-only features; post-capacity if
  compared to selected orbits.
- methods expected to consume it: `projection-structure`, `prediction-ranking`,
  `statistical-associations`; orbit comparisons only in interpretation packets.
- evidence commands/artifacts: current partial code in `prepare/features_omega.rs`
  needs audit before use as coverage.
- disposition: defer until tied to SCC/cycle/orbit-compatibility question.
- reopen triggers: omega sign graph shows signal; orbit interpretation becomes
  thesis-facing.

## Medium/Lower-Priority Rows

### Euclidean Dual/Primal Shape Controls

- feature object: Euclidean norms, pairwise distances/cosines, centered SVD,
  edge lengths, facet volumes, vertex covariance/inertia.
- thesis question: do ordinary Euclidean shape controls explain source effects
  or high-tail rows?
- current status: `candidate-existing-unaudited` for many dual, edge, and facet
  volume summaries in `features_geometry.rs` and `features_face_geometry.rs`;
  `absent` for primal vertex inertia/covariance.
- valid transforms: distribution/concentration summaries; source/facet-count
  stratified residuals; controls in predictive models.
- invalid/risky transforms: presenting Euclidean features as `Sp(4)` invariants;
  coordinate-wise claims without canonization.
- invariance/symmetry: `O(4)`/Euclidean and generator-sensitive, not
  `Sp(4)`-invariant. Translation invariance depends on centered or
  difference-based definitions.
- data/leakage status: pre-capacity geometry features.
- methods expected to consume it: existing geometry-only methods.
- disposition: keep as controls after audit; do not prioritize over symplectic
  features unless model evidence demands it.

### Incidence Complex And Face-Lattice Structure

- feature object: vertex-facet incidence, face counts, graph spectra, flag
  counts, combinatorial type proxies.
- thesis question: do combinatorial types or incidence patterns explain the
  high `sys` tail independently of metric/symplectic magnitudes?
- current status: `candidate-existing-unaudited` for counts and degree/size
  summaries in `features_skeleton.rs`; `absent` for spectra, flag counts, and
  richer face-lattice summaries.
- valid transforms: facet/vertex graph spectra; flag counts; f-vector
  refinements; facet-count-normalized ratios.
- invalid/risky transforms: assuming graph-only signal is a symplectic
  mechanism.
- invariance/symmetry: facet/vertex relabeling invariant if summarized
  correctly.
- data/leakage status: pre-capacity geometry/combinatorics.
- disposition: defer until high-priority symplectic/product gaps are addressed.

### Action Spectrum, Sigma, Orbit, And KKT Outputs

- feature object: post-capacity sigma/action lists, selected orbit facets,
  cycle diagnostics, and KKT/orbit-search scalars.
- thesis question: how should computed capacity evidence be interpreted and
  audited?
- current status: `candidate-existing-unaudited` for current orbit fields in
  `features_orbit.rs`.
- valid transforms: cutoff-aware availability, counts, first values, gaps,
  selected-vs-global comparisons, KKT diagnostics, and leakage-audit summaries.
- invalid/risky transforms: generic quantiles of truncated/censored action
  lists; using post-capacity fields as clean pre-evaluation proposer features.
- invariance/symmetry: depends on orbit representation and search output; must
  record censoring/cutoff semantics.
- data/leakage status: post-capacity interpretation/leakage only.
- disposition: keep separate from clean proposer feature sets; audit only when
  interpretation packet needs it.

### Raw Coordinates And Flattened Dual Vertices

- feature object: raw `dual_vertices_f64` and `dual_vertices_flat_f64`.
- thesis question: baseline black-box input or leakage/generator audit, not a
  direct geometric explanation.
- current status: `candidate-existing-unaudited`; raw arrays are retained in
  `polytope-table.jsonl`.
- valid transforms: grouped-CV black-box baselines; PCA/exploratory diagnostics;
  invariant/equivariant models if later designed.
- invalid/risky transforms: coordinate-level univariate explanation; treating
  arbitrary facet order or coordinate basis as meaningful geometry.
- invariance/symmetry: not facet-permutation invariant; coordinate/generator
  dependent; requires canonicalization or equivariant handling for stronger use.
- data/leakage status: pre-capacity geometry input but high interpretation risk.
- disposition: audit/baseline only.

## Current State

The active datascience slice is restricted to random/product polytopes. The
old ascent, continuation, local-behavior, and perturbation panels were removed
from the active surface during the random-slice cleanup. Feature coverage here
therefore concerns reusable columns on retained random/product rows.

Remaining next work:

1. Run method/statistics review on the full scoped artifacts and packet
   interpretation.
2. Decide whether the scoped prepared table should be retained in repo/LFS or
   kept as reproducible generated data.
3. Only after review, promote selected claims into thesis prose. Do not claim
   closure over arbitrary random polytope distributions from this retained
   random/product sample.

## Review Log

### 2026-06-25 Random-Slice Cleanup And Current-Schema Rerun

- milestone: cleanup the datascience slice so the active surface focuses on
  random polytopes and random Lagrangian-product polytopes.
- removed abandoned fixed-F ascent, continuation, endpoint-diagnostic,
  local-behavior, and perturbation files from the active
  `experiments/sys-datascience/` surface.
- simplified canonical and run-local prepare provenance so random/product
  tables no longer expose trajectory/ascent columns.
- retained compatibility output `computed-polytope-observation-table.jsonl` as
  an empty file because scan/fingerprint tooling expects the filename.
- full current-schema scratch prepare:
  `/tmp/sys-ds-random-only-full-current`;
  `14336` polytope rows, `14336` provenance rows, `0`
  computed-polytope observation rows, max `sys = 0.86258589584944`, and `0`
  rows with `sys > 1`.
- full current-schema method artifacts:
  `/tmp/sys-ds-full-current/`.
  The active method reruns found no `sys > 1` row and no validated
  candidate-proposer. Geometry-only prediction retained a strong in-table
  signal, but it did not rank unevaluated generated rows before `sys`
  computation.
- checks during cleanup:
  - `cargo check -p exp-sys-landscape`;
  - full random/product prepare and active method reruns against hydrated
    canonical producer data from `/workspaces/msc-math`.
- remaining gates: method/statistics review, packet README integration, and a
  retention decision for the full prepared table.

### 2026-06-22 Checkpoint Review

- local review: inspected current branch diffs for omega features, two-face
  symplectic-area ordering/tails, provenance metadata plumbing, shared method
  selection, and affected method READMEs.
- adversarial review: subagent reviewed the same checkpoint for feature-family
  gaps, overstrong invariance claims, leakage, stale documentation, and
  statistical-methodology risks.
- issues patched from review:
  - normalized omega wording now states that it is a Euclidean-representative
    diagnostic, not a general `Sp(4)` invariant or translation-canonized
    feature;
  - omega rank/nullity now carries the caveat that `Omega = A^T J A` has rank
    bounded by `4`, so nullity is mostly facet-count-driven and the current
    `1e-10` threshold is diagnostic;
  - two-face ordering diagnostics are excluded from geometry-feature inputs;
  - two-face symplectic-area summaries are excluded from clean method inputs if
    any loaded row reports incomplete two-face ordering;
  - clean univariate association screening excludes post-capacity `orbit_*`
    fields and reports them separately as available-but-not-tested;
  - product-bucket EDA labels multi-bucket provenance explicitly instead of
    silently selecting the first bucket;
  - product/source metadata consumption is recorded as implemented for
    `random-tail-eda`, with factor tests/provenance joins still pending for
    other methods.
- checks after review patches:
  - `cargo fmt --check -p exp-sys-landscape`;
  - `cargo check -p exp-sys-landscape`;
  - `python3 -m py_compile` on changed method scripts;
  - `git diff --check`;
  - selector smoke on `/tmp/tmp.Hitoz0vCai/tables`, verifying ordering
    diagnostics are present in rows but not selected as geometry features;
  - `random-tail-eda` smoke on `/tmp/tmp.Hitoz0vCai/tables`;
  - `statistical-associations` smoke on `/tmp/tmp.Hitoz0vCai/tables` with
    `--max-features 20 --permutations 2`, verifying `orbit_*` fields are
    reported as available but not tested.
- still not thesis evidence: full retained-table rebuild and full method reruns
  remain pending; after the later 2026-06-22 local rebuild attempt, this gate
  is routed to the LICCA handoff rather than local full rebuild.

### 2026-06-22 Source/Metadata Method Patch

- implemented shared product-bucket labeling in
  `methods/_shared/random_only.py`, including explicit `multi:` labels for
  rows with conflicting product provenance buckets.
- added `source_factor_tests` to `statistical-associations`; it summarizes and,
  where group sizes permit, runs ANOVA/Kruskal-style tests for source family,
  dataset label, facet count, source-by-facet, product bucket, product bounce
  count, and height range.
- added metadata-only ridge/random-forest baselines to `prediction-ranking`,
  using source/facet/product provenance labels as leakage/source diagnostics
  separate from geometry-only proposer inputs.
- checks:
  - `python3 -m py_compile` on changed method scripts;
  - `random-tail-eda` smoke on `/tmp/tmp.Hitoz0vCai/tables`;
  - `statistical-associations` smoke on `/tmp/tmp.Hitoz0vCai/tables` with
    `--max-features 20 --permutations 2`;
  - `prediction-ranking` smoke on `/tmp/tmp.Hitoz0vCai/tables` with
    `--max-features 20 --forest-trees 20 --permutations 1`;
  - `git diff --check`.
- still not thesis evidence: the smokes use a two-row temp table and only prove
  schema/runtime behavior. Full retained-table reruns remain pending.

### 2026-06-22 Cross-Method Dashboard

- added `methods/random-only-closure-summary.md` as the current cross-method
  dashboard for the trusted random/product method slice.
- linked the dashboard from `experiments/sys-datascience/README.md` and
  `methods/README.md`.
- the dashboard records:
  - the trusted random/product fingerprint used by current method packets;
  - method group, packet, data slice, test performed, current result, caveat,
    and disposition;
  - coarse mapping from the method checklist to current random-only handling;
  - the retained-table rebuild/full-rerun gate;
  - thesis claims supported by old retained artifacts versus claims not yet
    supported for the updated branch.
- evidence status: documentation/control-surface improvement only; it does not
  replace packet artifacts or full method reruns.

### 2026-06-22 Projection Metadata Overlays

- added source/facet/product metadata overlay summaries to
  `methods/projection-structure/analyze.py`.
- overlays summarize PCA coordinates by metadata labels and cluster composition
  by the same labels; metadata labels are not used to construct the geometry
  projection.
- updated `projection-structure/README.md` and
  `methods/random-only-closure-summary.md` to record the overlay behavior and
  full-rerun caveat.
- evidence status: schema/runtime smoke only until retained tables are rebuilt
  and the projection packet is rerun on full data.

### 2026-06-22 Random-Only Method Disposition Ledger

- added `methods/random-only-method-dispositions.md` to record
  run/defer/reject/out-of-scope dispositions for the checklist families in the
  trusted random/product scope.
- linked the disposition ledger from `experiments/sys-datascience/README.md`,
  `methods/README.md`, and `methods/random-only-closure-summary.md`.
- this closes a control-surface gap: future agents no longer need to infer
  from the recall checklist why broad optimization, sequence/trajectory,
  endpoint/attractor, and usually rejected method families are not part of the
  current clean random-only method evidence.
- evidence status: disposition/control-surface improvement only. Full retained
  method reruns and method/statistics review remain required before closure.

### 2026-06-22 Old-Schema Full-Table Method Reruns

- used hydrated checked-in retained tables from
  `/workspaces/msc-math/experiments/sys-datascience/prepare` because the
  feature-closure worktree contains LFS pointer files for `prepare/*.jsonl`.
- regenerated branch method artifacts for:
  - `methods/random-tail-eda/artifacts`;
  - `methods/statistical-associations/artifacts`;
  - `methods/projection-structure/artifacts`;
  - `methods/prediction-ranking/artifacts`.
- current old-schema full-table results:
  - `random-tail-eda`: `14336` trusted random/product rows, `0` rows with
    `sys > 1`, max `sys = 0.86258589584944`;
  - `statistical-associations`: `89` eligible scalar covariates, `79`
    nonconstant tested, max absolute Spearman `0.9384368671850424`,
    family-max permutation p-value `0.004975124378109453`, and source/facet/
    product factor-test output;
  - `projection-structure`: `88` geometry features, five PCA components,
    top-25 isolation-forest anomaly overlap with top-2% `sys` rows is `0`, and
    metadata overlays are present;
  - `prediction-ranking`: `88` geometry features, `26` metadata one-hot
    features, geometry random forest `R^2 = 0.921983825923774`, metadata-only
    random forest `R^2 = -0.04953269595337506`.
- updated the affected packet READMEs, `random-only-closure-summary.md`, and
  `random-only-method-dispositions.md` to distinguish these old-schema
  full-table artifacts from post-rebuild evidence for new prepare columns.
- evidence status: this is real full-table method evidence for the old retained
  prepare schema plus branch method-side diagnostics. It is not evidence for
  the new omega matrix/sign/alignment, two-face-tail, or explicit provenance
  prepare columns until prepare is rebuilt and the packets are rerun.

### 2026-06-22 Scoped Random/Product Full Rerun

- fixed the prepare architecture so `sys-dataset --random-only` filters before
  feature construction and skips ascent/continuation rows, computed-polytope
  observation rows, and post-capacity orbit cache loading.
- added explicit local prepare tiers:
  - `prepare/build-random-only-slice.sh smoke`: `8` random + `10` product rows;
  - `prepare/build-random-only-slice.sh method`: `512` random + `1024`
    product rows;
  - `prepare/build-random-only-slice.sh full`: all `4096` random + `10240`
    product rows.
- observed timings with hydrated canonical producer files from
  `/workspaces/msc-math/experiments/sys-datascience/produce`:
  - `smoke`: loaded `18` rows in `0.4s`, feature table rounded to `0.0s`;
  - `method`: loaded `1536` rows in `0.5s`, feature table `1.2s`;
  - `full`: loaded `14336` rows in `0.5s`, feature table `408.3s`, total
    table build `409.1s`; memory stayed low in spot checks, so the remaining
    cost is CPU-heavy geometry/face feature construction.
- full scoped fingerprint:
  - `14336` polytope rows, `14336` provenance rows;
  - `0` computed-polytope observation rows and `0` ascent-run rows;
  - source counts `random_sample = 4096`, `random_product_sample = 10240`;
  - max `sys = 0.86258589584944`, `0` rows with `sys > 1`.
- reran full method packets against `/tmp/sys-ds-random-only-full`:
  - `random-tail-eda`: no positives, max `sys = 0.86258589584944`;
  - `statistical-associations`: `110` eligible scalar covariates, `99`
    nonconstant tested, max absolute Spearman `0.9384368671850424`,
    family-max permutation p-value `0.004975124378109453`;
  - `projection-structure`: `109` geometry features, PC1/`sys` correlation
    `-0.4636669884808957`, top-25 anomaly overlap with top-2% `sys` rows `0`;
  - `prediction-ranking`: `109` geometry features, `27` metadata features,
    geometry random forest `R^2 = 0.9266078877149259`, metadata-only random
    forest `R^2 = 0.0019535588595060993`.
- evidence status: current full random/product method evidence for the new
  prepare columns exists. Remaining gates are method/statistics review,
  final disposition review, and deciding whether to retain the scoped prepared
  table output or keep it as reproducible generated data.
