<!--
Purpose: fix the data contract for the hostile-landscape feature/pattern-search closure task.
Context: this note chooses the dataset shape, row IDs, source files, and staged enrichment plan
before implementation so later analysis does not have to redesign persisted artifacts.
-->

# Feature Pattern Search Dataset Plan

## Goal

Close the hostile-landscape caveat in `RESULTS.md` and `TASKS.md` with a bounded
standard-method pass over existing random-sample and ascent artifacts, while
laying the data foundation in a form that still makes sense if the analysis is
later expanded for publication.

Implementation status:

- 2026-04-18: Stage 1 converter landed as
  `cargo run -p exp-sys-landscape --release --bin sys-normalized-dataset`
- verified normalized output counts on the current default source surface:
  `282` states, `282` capacity rows, and `287` fixed-`F` step events
- the converter currently relies on exact cache rows for `vertices_rational`,
  so the shared `experiments/sys-landscape/cache.jsonl` must already contain the
  fixed-`F` ascent endpoints; this session refreshed that cache contract
- 2026-04-18: `states.jsonl` gained an explicit `root_group_id` ancestry key so
  grouped CV and leakage control no longer depend on Python-side lineage
  heuristics
- 2026-04-18: first bounded modeling pass landed at
  `experiments/sys-landscape/feature-pattern-search/analyze.py`
- current first-pass result:
  cheap dual-vertex geometry summaries help within the random regime
  (`R^2 ≈ 0.43` with ridge, vs `0.19` for the metadata baseline),
  but they fail within the endpoint regime (`R^2 ≈ -0.11`) and show
  no random/endpoint transfer (`R^2 < -5` on both transfer surfaces)
- 2026-04-18: bounded `feature_skeleton.jsonl` enrichment landed via
  `sys-feature-skeleton`
- current skeleton result:
  pure skeleton features also fail on the endpoint regime
  (`R^2 ≈ -0.06` with ridge), and the full
  `metadata + geometry + skeleton` block still underperforms metadata alone on
  the endpoint surface
- 2026-04-18: bounded `feature_omega.jsonl` enrichment landed via
  `sys-feature-omega`
- current omega result:
  ridge-local `omega_0` summaries and directed transition-graph features help
  within the random regime (`R^2 ≈ 0.43` with ridge), but they still add only
  a small endpoint-side signal (`R^2 ≈ 0.09`) and do not transfer across
  regimes
- 2026-04-18: fixed-`F` ascent cache writes now preserve cached `best_sigma`
  payloads for endpoint rows, and a bounded `feature_orbit.jsonl` enrichment
  landed via `sys-feature-orbit`
- current orbit result:
  sigma-local support-size, geometry, `omega_0`, and transition features help
  the endpoint regime more than geometry/skeleton/omega alone
  (`R^2 ≈ 0.11` with ridge), but they still underperform the metadata baseline
  (`R^2 ≈ 0.44`) and remain strongly non-transferable
- 2026-04-18: the orbit packet is now interpreted as three sub-blocks
  (`orbit_combinatorics`, `orbit_geometry`, `orbit_search`) with the merged
  `orbit` packet kept only as a reference aggregate
- current implication:
  there is still no evidence for a cheap transferable pattern; the next
  enrichment step, if pursued, should move past cached-`best_sigma` summaries
  toward richer face-level symplectic data or full orbit/KKT payloads rather
  than more provenance metadata

The design question here is not "which regressor should we try first?" The
durable question is "what persisted dataset shape lets us try many methods
without rebuilding the whole data surface each time?"

## Current Research Surface

The closure task wants to distinguish two possibilities:

1. There is a transferable signal that predicts higher `sys` across random
   samples and ascent-found local maxima.
2. Any signal is weak, non-transferable, or reducible to dataset/family/facet
   count effects, which supports the hostile-landscape interpretation.

Main modeling-surface rule:

- exclude packets deliberately constructed in the neighborhood of HKO2024 from
  the default hostile-landscape dataset
- use them only as separately labeled control/sensitivity packets if later
  needed

Reason:

- otherwise the easiest "pattern" to learn may be "start near the one known
  counterexample"
- that would weaken the thesis-level question, which is whether data science can
  find useful structure without already encoding the unique known `sys > 1`
  spoiler

Current committed source surfaces already cover:

- random generic samples: `experiments/sys-landscape/random-sample/random-sweep.jsonl`
  with 70 rows
- random Lagrangian products:
  `experiments/sys-landscape/random-product-sample/random-product-sweep.jsonl`
  with 100 rows
- fixed-`F` ascent endpoints:
  `gradient-ascent-general.jsonl` with 10 rows and
  `gradient-ascent-products.jsonl` with 12 rows
- fixed-`F` ascent event logs:
  `gradient-ascent-general-trace.jsonl` with 146 rows and
  `gradient-ascent-products-trace.jsonl` with 141 rows
- variable-`F` continuation endpoints:
  `experiments/sys-landscape/variable-f-ascent/variable-f-ascent.jsonl`
  with 90 rows
- separately labeled HKO-specific control packet, excluded from the default
  modeling surface:
  `experiments/hko-local-maximum/cut-and-ascent/cut-and-ascent.jsonl`
  with 20 rows
- reusable rational polytope caches:
  `experiments/sys-landscape/cache.jsonl` with 214 rows and
  `experiments/combinatorial-cells/polytopes.jsonl` with 953 rows
- existing orbit/symplectic feature packet:
  `experiments/combinatorial-cells/omega-hypothesis/omega-obstacle.jsonl`
  with 953 rows

## Approach Options Compared

### Option A: One wide ad hoc table built directly from current JSONL files

Pros:

- fastest to start
- one Python script can consume it immediately

Cons:

- geometry identity and search provenance get fused into one row
- later orbit or derivative enrichments require rewriting the core table
- duplicates the same polytope if it appears in several roles
- poor fit for transfer tests and future publication-grade comparisons

### Option B: Normalized endpoint/state core plus joinable enrichment tables

Pros:

- stable geometry identity via `poly_id`
- stable provenance identity via `state_id`
- current random and ascent artifacts fit this shape without inventing missing
  intermediate geometry
- later orbit, derivative, or novel symplectic features can land as separate
  `poly_id`-keyed tables
- works for the thesis closure pass and still scales to publication follow-up

Cons:

- more joins and more files than a one-off wide table
- requires explicit ID and table conventions up front

### Option C: Full state graph now, with stored geometry for every intermediate

Pros:

- best long-term shape for trajectory-aware methods
- makes every ascent move a first-class object

Cons:

- current trace artifacts do not store intermediate polytope geometry
- retrofitting this now would require new experiment output, not just
  conversion of committed data
- LICCA-scale intermediate geometry would increase output size and cache policy
  complexity immediately

### Choice

Choose **Option B now**.

This is the right "do it properly now" compromise:

- it locks the durable IDs and joins now
- it does not pretend the current trace logs contain intermediate states that
  were never persisted
- it leaves a clean pivot path to Option C later if future runs decide to emit
  intermediate geometries explicitly

## Dataset Contract

### Identity Rules

- `poly_id`: stable geometry ID defined as a content hash of the ordered
  rational dual-vertex list used by `PolytopeRecord`
- `state_id`: stable provenance ID for one occurrence of a polytope in a
  dataset or search lineage

`poly_id` deliberately preserves facet order. This is not an attempt to quotient
out by relabeling or combinatorial isomorphism. The experiment surfaces already
use facet-indexed objects such as `sigma`, `beta`, ridge pairs, and ascent
directions, so ordered dual vertices are the correct persisted identity.

The authoritative geometry payload is still the exact rational dual-vertex list
itself. The hash is the join key, not a replacement for storing the exact
geometry.

Recommended canonicalization rule for `poly_id`:

- serialize `dual_vertices_rational` as a JSON array of 4-tuples of
  `"numerator/denominator"` strings in the stored facet order
- hash that canonical byte string with a stable content hash such as `blake3`
- store both `poly_id` and the exact rational payload in `polytopes.jsonl`

### Core Tables

#### `polytopes.jsonl`

One row per unique geometry.

Required columns:

- `poly_id`
- `dual_vertices_rational`
- `vertices_rational`
- `facet_count`
- `geometry_source`

Primary source:

- direct copy/conversion from `PolytopeRecord` rows in
  `experiments/sys-landscape/cache.jsonl`
- direct copy/conversion from `PolytopeRecord` rows in
  `experiments/combinatorial-cells/polytopes.jsonl`
- optionally direct copy/conversion from
  `experiments/sys-landscape/variable-f-ascent/cache.jsonl` when a later
  packet decides that its larger local cache is worth consuming explicitly
- additional rows reconstructed from endpoint `dual_vertices` or
  `final_dual_vertices` fields when no cache row exists yet

Current exact-cache counts:

- `sys-landscape/cache.jsonl`: 214 rows
- `combinatorial-cells/polytopes.jsonl`: 953 rows
- `variable-f-ascent/cache.jsonl`: 20061 rows, but with weaker provenance
  metadata than the other two caches

#### `states.jsonl`

One row per dataset/search occurrence of one polytope.

Required columns:

- `state_id`
- `poly_id`
- `dataset`
- `family`
- `role`
- `search_space`
- `optimizer`
- `backend`
- `source_name`
- `seed_index` when present
- `lineage_id` when present
- `parent_state_id` when an explicit parent exists

Current role vocabulary:

- `random_sample`
- `ascent_endpoint`
- `continuation_endpoint`
- `hko_control` for excluded control/sensitivity packets only

#### `capacity_results.jsonl`

One row per `poly_id` carrying the scalar target values and cheap search
summaries.

Required columns:

- `poly_id`
- `capacity`
- `volume`
- `sys`
- `iterations`
- `search_result_source`

### Event Table

#### `step_events.jsonl`

One row per logged ascent event, not one row per persisted intermediate state.

Required columns:

- `state_id`
- `phase`
- `iteration`
- `step_type`
- `t_fraction`
- `t_actual`
- `sys_before`
- `sys_after`
- `delta_sys`
- `gradient_norm`

Reason for this table shape:

- the current fixed-`F` trace files are event logs keyed by ascent name
- they do not contain intermediate geometry, so they cannot populate a genuine
  `from_state_id -> to_state_id` geometry graph
- if future runs emit intermediate geometries, those can add a new
  `state_transitions.jsonl` table without changing the endpoint contract

## Enrichment Tables

### `feature_geometry.jsonl`

Cheap-to-medium Euclidean/symplectic black-box features computed from the
polytope geometry alone.

Examples:

- dual-vertex norm summaries
- centered dual-matrix singular/eigenvalue summaries
- pairwise cosine summaries
- all-pair `|omega0|` summaries
- q/p energy-split sparsity summaries

### `feature_skeleton.jsonl`

Features that require `Skeleton::compute(polytope)` but not a fresh orbit solve.

Examples:

- vertex, edge, ridge counts
- simple/non-simple flags
- ridge-degree summaries
- ridge `|omega0|` summaries
- threshold counts for small ridge `|omega0|`

### `feature_orbit.jsonl`

Orbit-sensitive enrichment keyed by `poly_id`, not part of the core endpoint
contract.

Examples:

- best `sigma`
- best `beta`
- orbit length
- orbit `omega` summaries
- admissibility / interval metadata from `OrbitSearchResult`

Why separate:

- current random caches often already store one best `sigma`
- current ascent endpoint summaries do **not** store the richer orbit payload
- treating orbit data as a joinable enrichment lets the closure pass start from
  geometry-only features and add orbit-sensitive features later without
  redesigning the dataset
- the current complete orbit/symplectic feature packet is
  `experiments/combinatorial-cells/omega-hypothesis/omega-obstacle.jsonl`,
  which covers 953 cached polytopes but does not yet cover the sys-landscape
  ascent endpoints

### Future Optional Tables

- `feature_derivatives.jsonl`
- `feature_family_specific.jsonl`
- `state_transitions.jsonl` if future runs emit intermediate geometries

## Source Mapping

### Source Priority Rule

For the first converter packet, use explicit live-vs-historical priority rather
than assuming every committed JSONL is equally current.

Recommended rule:

1. exact geometry/capacity data comes from rational caches first
2. current explicit experiment outputs come next
3. historical fallback JSONLs are allowed only when no current higher-priority
   artifact exists locally

Immediate consequence:

- `gradient-ascent-general.jsonl` and `gradient-ascent-products.jsonl` are
  usable for the first local scaffold, but they must be labeled as historical
  root artifacts because their logbooks treat them as superseded by
  `data/smoke.jsonl` and future `data/licca.jsonl`

### `polytopes.jsonl`

Use these sources in order:

1. exact `PolytopeRecord` rows already present in `sys-landscape/cache.jsonl`
2. exact `PolytopeRecord` rows in `combinatorial-cells/polytopes.jsonl`
3. reconstructed polytopes from endpoint `dual_vertices` / `final_dual_vertices`
   when neither cache already contains the geometry

Exact join available now:

- rational caches join exactly by ordered `dual_vertices_rational`

Exact join not available yet:

- summary/event packets that only carry `dual_vertices` or `final_dual_vertices`
  as `f64` require new code to match or reconstruct against the rational cache

### `states.jsonl`

Create one `state_id` row from each source packet:

- `random-sample/random-sweep.jsonl`
- `random-product-sample/random-product-sweep.jsonl`
- `gradient-ascent-general/gradient-ascent-general.jsonl`
- `gradient-ascent-products/gradient-ascent-products.jsonl`
- `variable-f-ascent/variable-f-ascent.jsonl`
- exclude `hko-local-maximum/cut-and-ascent/cut-and-ascent.jsonl` from the
  default converter output; add it only in a separately labeled control mode if
  needed later

Current committed counts:

- `random-sweep.jsonl`: 70
- `random-product-sweep.jsonl`: 100
- `gradient-ascent-general.jsonl`: 10
- `gradient-ascent-products.jsonl`: 12
- `variable-f-ascent.jsonl`: 90
- `cut-and-ascent.jsonl`: 20, excluded by default from main modeling

Recommended stable `state_id` scheme:

- `random_sample::random_F5_0`
- `random_product::random_3x3_0`
- `ga_general::general_0`
- `ga_products::products_0`
- `variable_f::rq1_general_0_p0`
- `cut_and_ascent::hko_p0`

Recommended `lineage_id` rules:

- `general_7` for `rq1_general_7_p*`
- `rq2_seed_<base>` for the four-way comparison paths in variable-`F`
- `products_5` or `general_3` for fixed-`F` ascent endpoints
- `None` for random baselines

Known caveat:

- `cut-and-ascent` is a preliminary HKO-specific control packet and is excluded
  by default from the generic-search evidence surface

### `step_events.jsonl`

Populate only from:

- `gradient-ascent-general-trace.jsonl`
- `gradient-ascent-products-trace.jsonl`

Variable-`F` currently does not emit step-level event logs in the same format,
so it contributes endpoint states only.

Current committed event counts:

- `gradient-ascent-general-trace.jsonl`: 146
- `gradient-ascent-products-trace.jsonl`: 141

Exact join available now:

- general/products summary rows join to their event rows exactly by `name`

Known gap:

- variable-`F` and cut-and-ascent currently have no parallel event export

## Column Ownership And Cost

### Already present in current committed artifacts

Low code cost, no rerun needed:

- source/dataset names
- endpoint dual vertices
- scalar `capacity`, `volume`, `sys` on random packets
- ascent summary metrics such as `best_strategy`, `n_ascent_phases`,
  `n_gradient_iters_total`, and `total_delta`
- fixed-`F` step-event columns from the trace JSONLs

### Cheap to add from cached geometry

Low-to-medium code cost, low rerun cost:

- rational geometry copy into `polytopes.jsonl`
- exact `poly_id` creation
- Euclidean dual-vertex summaries
- skeleton counts and ridge summaries

### Medium-cost enrichments

Medium code cost, medium compute only on cache misses:

- recompute or recover best orbit payload for endpoint rows whose geometry is
  not already represented by a cached `PolytopeRecord` with `sigmas`
- feature blocks that require `Skeleton::compute` plus orbit-sensitive joins
- matching `f64` endpoint geometries back into the rational cache in a way that
  is explicit about exact match versus reconstructed fallback

### High-cost future expansion

High code and artifact cost:

- storing intermediate ascent geometries for every step
- emitting full lineage graphs for LICCA-scale runs
- derivative-heavy enrichments across the whole endpoint packet

## Implementation Stages

### Stage 1: core dataset scaffold

Build:

- `polytopes.jsonl`
- `states.jsonl`
- `capacity_results.jsonl`
- `step_events.jsonl`

No model fitting yet. This stage proves the IDs, joins, and source selection.

### Stage 2: cheap enrichments for the closure pass

Build:

- `feature_geometry.jsonl`
- `feature_skeleton.jsonl`

These are the first feature blocks for the bounded hostile-landscape pass.

### Stage 3: optional orbit enrichment

Build:

- `feature_orbit.jsonl`

Only after the core tables and cheap features are working cleanly.

## Pivot Rules

Pivot away from the current plan if:

- the cache join rate for ascent endpoints is much worse than expected and
  orbit enrichment would require large fresh recomputation immediately
- the endpoint/state contract reveals that one source packet is too stale or
  structurally different to share the same table
- LICCA-returned data arrives with a different live-vs-historical priority than
  the current local artifacts

Do **not** pivot back to an ad hoc one-off wide table just because the first
implementation packet is slower than hoped. If simplification is needed, cut an
enrichment table or defer orbit features, but keep the `poly_id` / `state_id`
contract.

## Next Work Packets

The bounded closure pass is complete. The next decisions are no longer about
dataset shape; they are about where local developer time still has a good
feedback loop before large LICCA runs arrive.

### Unblocked Local Packets

These use the current exact geometry, normalized core tables, and smoke/local
verification loops. They do not need new cluster-scale data before
implementation starts.

1. **Trajectory aggregate features from `step_events.jsonl`**
   - Landed as `feature_trajectory.jsonl`, keyed by `state_id`, with scalar
     summaries of fixed-`F` trace availability, overshoot mix, phase restarts,
     step-size statistics, gradient norms, and gain concentration.
   - Coverage:
     `22 / 282` states (`10 / 10` general ascent, `12 / 12` product ascent,
     `0 / 90` variable-F continuation, `0` random baselines).
   - Result:
     near-null. Ridge `R^2` is `-0.0140` within random, `0.0026` within the
     endpoint union, and strongly negative on both transfer surfaces.
   - Interpretation:
     this closes the cheap scalar "maybe the signal is in fixed-`F` step-event
     dynamics" branch for the current dataset; a richer trajectory line would
     need either more traced endpoint rows or a more explicit state-graph
     question.

2. **Richer cached orbit/KKT scalar payload**
   - Landed as optional `orbit_scalars` on `PolytopeRecord`, currently filled by
     the fixed-`F` ascent endpoint writers and the canonical random baseline
     packets. `feature_orbit` now reads these cached scalars when present and
     falls back to one best-sigma KKT solve for older cache rows; at the moment
     that fallback still covers the `variable-f-ascent` packet.
   - Current scalar set:
     search iterations, retained-orbit count, best-orbit `beta_margin`,
     `q_error_bound`, and boolean `mu` / `xi` / exact-certification flags.
   - Result:
     the richer `orbit` block improves the random regime further
     (`R^2=0.3222` ridge, `0.3970` RF) but leaves the endpoint regime
     essentially unchanged (`R^2=0.1083` ridge, `0.0967` RF). Transfer remains
     strongly negative and becomes even more negative on the random-to-endpoint
     surface once the random packet carries search-level orbit scalars.
   - Interpretation:
     this strengthens the reading that cheap orbit/KKT structure is real in the
     random packet but still not the missing transferable endpoint signal.

3. **Bounded face-level Euclidean features**
   - Landed as `feature_face_geometry.jsonl`, keyed by `poly_id`, with scalar
     summaries of edge lengths and facet 3-volumes only, evaluated after
     rescaling each polytope to the `vol(K)=1` convention.
   - Column set:
     `vertex_count`, `edge_count`, volume-normalized edge-length
     mean/std/min/max/max-share, and volume-normalized facet-volume
     mean/std/min/max/sum/max-share.
   - Result:
     helpful but still regime-specific. Within random it is strong for RF
     (`R^2=0.7009`) and now materially positive for ridge (`0.3847`); within
     endpoints it still adds only a small signal (`0.1030` ridge, `0.1218`
     RF), still well below the
     metadata baseline.
   - Interpretation:
     exact face-size summaries are not null, but they still behave like another
     endpoint-only/random-only packet rather than a transferable search clue.

4. **Bounded face-level symplectic features**
   - Landed as `feature_face_symplectic.jsonl`, keyed by `poly_id`, with
     summary-only ridge-polygon symplectic-area columns from ordered ridge
     vertex cycles, normalized by `vol(K)^(1/2)` so the packet is evaluated in
     the `vol(K)=1` convention.
   - Column set:
     ridge symplectic-area mean/std/min/max/sum/max-share, plus small-area
     threshold fractions.
   - Result:
     this is the strongest non-metadata endpoint-side block so far. Within
     random it reaches `R^2=0.5483` ridge and `0.8779` RF; within endpoints it
     reaches `0.4000` ridge and `0.2934` RF, clearly above the existing omega
     block but still below metadata.
   - Interpretation:
     symplectic face summaries look substantially more informative than
     Euclidean face summaries, but they still do not transfer across regimes.

5. **Symmetry-status bookkeeping for the bounded packet**
   - The generated summary now carries an explicit per-block table for:
     `vol(K)=1` normalization status, translation invariance, and `Sp(4)`
     invariance.
   - Current reading:
     `face_symplectic` and `skeleton` are the cleanest symmetry-aware blocks;
     `geometry` and `face_geometry` still keep Euclidean gauge dependence;
     `omega` is mixed because the transition/sign parts are symmetry-aware while
     the dual-coordinate magnitude packet still depends on translation gauge;
     `orbit` and `trajectory` are mixed/search-side rather than pure geometry
     quotients.

### LICCA-Blocked Packets

These are now blocked primarily by row count, not by missing local scaffolding.

1. **More fixed-`F` ascent endpoints**
   - Highest-value future dataset growth.
   - Priority order:
     `gradient-ascent-general`, then `gradient-ascent-products`, then
     `variable-f-ascent`.
   - Reason:
     the bounded pass already shows within-random signal; endpoint-side data is
     the main thin surface.

2. **Refresh the canonical main-surface packets from LICCA outputs**
   - Once new LICCA JSONLs arrive, keep using the current normalized converter
     and feature-pattern-search pipeline before inventing new methods.
   - Required rerun order:
     refresh canonical experiment JSONLs / caches, rerun
     `sys-normalized-dataset`, then rerun
     `experiments/sys-landscape/feature-pattern-search/analyze.py`.

### Current Prioritization

If local work continues before new LICCA rows arrive, use this order:

1. bounded face-level Euclidean features
2. bounded face-level symplectic features
3. only then revisit richer trajectory/state-graph methods

If new LICCA rows arrive first, pause local feature proliferation and refresh
the endpoint datasets before adding more model families.

### Residual Endpoint Check

- 2026-04-18: residual packet landed as
  `experiments/sys-landscape/feature-pattern-search/analyze_residual.py`.
- method:
  metadata-first additive grouped CV on the endpoint union, with the block
  model trained on the metadata residuals and grouped by the existing endpoint
  `root_group_id` / `source_name` fallback.
- current result:
  `face_symplectic` is the clearest endpoint-side residual gain beyond
  metadata (`Delta R^2 ≈ 0.12` ridge, `≈ 0.00` RF); `trajectory` is a small
  positive residual on both models; `face_geometry` is marginally positive for
  ridge only; `geometry`, `skeleton`, `omega`, `orbit`, and the full
  non-metadata union do not improve the metadata baseline.
