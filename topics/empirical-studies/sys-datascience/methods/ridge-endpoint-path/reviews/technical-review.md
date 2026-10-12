# Technical review: ridge endpoint target pass

## Disposition: accept the eight numerical values; repair the packet before reuse

The eight target rows are internally consistent and should be retained as a
reproducible numerical observation.  An independent exact-orbit-set review
run confirms the interior `3x6/q01` capacity on the submitted f64 geometry.
The saved packet is nevertheless not provenance-complete for reuse because
its `capacity_contract_source_sha256` hashes a *proposer* program, not the
capacity implementation actually linked by the evaluator, and it does not
retain the capacity guarantee diagnostics/certificate.

This is a repairable evidence/provenance defect, not a numerical discrepancy.
Until the packet is repaired, use the saved target JSONL only as an exploratory, deliberately
feature-targeted construction; do not treat it as an independent validation
sample, a random-tail frequency statement, or evidence that low ridge sum by
itself predicts near-one `sys`.

## Checks that pass

### Identity and freeze boundary

* Current SHA-256 values agree with every target row and summary:
  `candidates.jsonl = 889fd728923465269e2cb0587c834ef253d84edc6af8ed9eaa070360852c7c61`,
  `api-verification.jsonl = eae432539d71f1ea82c426933d551108fbbc255eccd21ccbde4504438ee1a1bc`, and
  `src/main.rs = 1d980cd345a6917ed8cd5e279307f18645197c748bce5b0a2f8fc55df2324ec0`.
* Original timestamps place candidate/design output at `16:46:29`, API
  verification at `16:46:36`, evaluator source at `16:52:01`, and the saved
  first target output at `16:52:09` UTC.  Together with the matching hashes,
  this is good evidence that the actual candidate geometry and the API
  verification artifact were not altered after the target call.
* The frozen-CDF artifact records the cache digest and contains exactly
  `100000` rows in each bucket.  Its two endpoint counts are zero; this is
  correctly a censored lower-tail statement, not an estimated population tail
  probability.

### Rows, geometry, and arithmetic

* `target-evaluation.jsonl` has exactly eight unique expected IDs, one for
  each `3x6`/`4x4` times `q01`/`q001`/`q0001`/`endpoint` combination.  All
  fields needed to recompute `sys` are finite.
* The saved API verification has the expected product combinatorics in every
  row: `3x6` gives facets/vertices/edges/ridges `9/18/36/27`; `4x4` gives
  `8/16/32/24`.  It reports simple products, ordered fraction one, no ordering
  failures, all support heights in `[0.8,1.2]`, and edge-formula/feature
  agreement within `3.56e-15`.
* Recomputed `capacity^2/(2*volume)` agrees with each recorded `sys` to
  better than `1e-14`; no row has `sys > 1`.  The endpoint assertions are met:
  `3x6 = 0.75` and `4x4 = 0.5` within `1e-8`.
* Bounce counts and sigma lengths fit the declared product solver structure:
  every `3x6` row has three bounces and a six-facet sigma; every `4x4` row has
  two bounces and a four-facet sigma.  The high row is
  `3x6/q01`: volume `18`, capacity `5.851067096898951`,
  `sys 0.9509718381225976`, bounces `3`, sigma `[0,8,2,6,1,4]`.

### Fresh producer-path replay

I reran the current evaluator once, thereby executing only the eight target
capacity calls.  It reproduced every semantic field above, every `poly_id`,
and all frozen candidate/API/evaluator hashes.  Only elapsed-time fields (and
therefore the byte hash of the generated target JSONL) are expected to change.

## Capacity guarantee boundary

The evaluator does call the real product path:

`SysLandscapePolytopeCache::from_lagrangian_product` -> exact-incidence volume
-> `exp_sys_landscape::capacity_billiard` -> billiard sigma enumeration ->
`aggregate_orbits_with_dual_vertices_exact(..., OrbitGuaranteeMode::MinimaSafe)`.

Thus the reported scalar is not merely a feature surrogate.  Exact rational
arithmetic is used for the incidence volume and for resolving *indeterminate*
near-minimum orbit candidates.  However, `MinimaSafe` permits a clearly
admissible f64 best orbit to remain f64; the target schema discards
`iterations`, `min_action_lower`, `min_action_upper`, orbit admissibility,
beta margin, and any exact-resolution count.  It therefore does not itself
provide a durable exact certificate for the displayed capacity.  Also, the
exact rationals encode the submitted f64 normals/heights, rather than an
independent symbolic model of the intended trigonometric polygons.

For the most consequential row I additionally ran an independent exact
orbit-set aggregation from the same reconstructed input.  It examined `468`
billiard sigma candidates and performed `68` exact KKT resolutions.  It found
two exact minimizers, `[0,7,1,3,2,5]` and `[0,8,2,6,1,4]`, with identical
exact action

```text
4805970830822047804630130065382950860590066355953200082737243253217998571649657875633147020836864
/ 821383646987948252599476974935743855580962501797711025874405257825978773070633687065300311459187
```

and hence capacity `5.85106709689895155` and `sys 0.95097183812259800`.
This independently confirms the saved value to the displayed precision and
explains why either minimizer may appear as the stored `best_sigma`.  The
review program and output are `/tmp/ridge-endpoint-cert-review/{src/main.rs,output-final.txt}`
(source SHA-256 `66026a6cf6c635569eec4567b7011f859930cd0dac873c9d496c9097ceb45488`,
output SHA-256 `7b32be1266df9dd28a766c6dd451925511a640a228b684087c2b1f18bbd377b7`).

The exact aggregation certifies the submitted f64-derived rational geometry
and the f64-solver candidate stream; the packet should retain that distinction
rather than call it a symbolic certificate for the ideal trigonometric family.

## Required exact repairs

1. Add a capacity provenance manifest to the target packet.  Pin the git
   revision (`8eb3c66d2a38830f7b6e44f08e9c0b984eb9474d` for this review), the
   target `Cargo.lock` hash, and hashes (or one deterministic combined digest)
   for the actual implementation closure: at minimum
   `experiments/sys-landscape/src/lib.rs`,
   `crates/symplectic/src/algorithms/billiard/{mod.rs,block_enumeration.rs,facet_classification.rs}`,
   `crates/symplectic/src/algorithms/{facet_adjacency.rs,orbit_search.rs}`, and
   the exact KKT solver used by the aggregation.  Replace the misleading
   `capacity_contract_source_sha256` field; the currently hashed
   `methods/extreme-scalar-rejection-proposer/src/main.rs` only imports the
   capacity API and does not identify its implementation.
2. Promote the successful `3x6/q01` review certificate above into the packet
   (rather than leaving it in reviewer scratch).  Retain its input `poly_id`,
   candidate/implementation manifest digests, capacity/exact action, both
   minimizer sigmas, `468` candidate iterations, `68` exact resolutions, and
   its f64-derived-rational boundary.  It agrees with the target capacity/sys
   to a stated tolerance.
3. Extend every target row (or a linked audit JSONL) with the ordinary search
   guarantee fields: `iterations`, `min_action_lower`, `min_action_upper`,
   best-orbit `admissibility`, `beta_margin`, and any exact-fallback count.
   This makes a capacity claim reviewable without rerunning hidden internals.
4. Make the summary check schema-complete: expected eight IDs, unique IDs,
   all finite, no `sys > 1`, arithmetic tolerance, and links/digests for the
   target JSONL plus its frozen CDF and geometry verification.  Include the
   `3x6/q01` target result explicitly rather than only endpoint controls.
5. Preserve the interpretation boundary in the owner-facing handoff: rotations
   were chosen after inspecting the frozen feature CDF but before evaluating
   `sys`.  The packet can support the conditional observation that this
   constructed `q=.01` placement produced the recorded `sys`; it cannot
   support a holdout/generalization or causal feature claim.

## Recheck rule

The numerical values may now be used for further research and a guarded
thesis-facing discussion.  Complete repairs 1--4 before handing the packet to
another session as reproducible evidence.  The interpretation guardrail in
repair 5 remains mandatory.

## Final recheck of repairs 1--4 (2026-07-12): REJECT AS COMPLETE

No new target point was evaluated for this recheck.  I inspected the newly
generated manifest, v2 rows, q01 certificate, summary, their cross-links, and
the current source that generated them.

* **Repair 2 — accepted.** `q01-certified-minimizers.json` is now in the
  packet.  Its candidate and manifest digests link to the v2 q01 row; its
  `poly_id`, ordinary capacity, `468` iterations, and the two minimizer sigmas
  agree.  It records `68` exact resolutions, the exact action, an
  `8.88e-16` ordinary/certified-capacity difference below its `1e-12`
  tolerance, and correctly limits itself to the submitted f64-enumerated
  stream.
* **Repair 4 — accepted.** `target-summary.v2` records the expected and
  observed eight-ID sets, uniqueness, finiteness, no `sys > 1`, arithmetic
  tolerance, endpoint checks, q01 values, and hashes of the target rows,
  certificate, candidate/API/CDF artifacts, manifest, and evaluator source.
  Every saved digest was recomputed and matches its current file.
* **Repair 1 — not complete.** The manifest hashes its listed files and its
  commit and lockfile match the current worktree, but its claimed
  `implementation_closure_sha256` omits the ordinary f64 KKT hot path.
  `orbit_search.rs` directly calls
  `kkt::saddle_point_solver::solve_kkt_for_dual_vertices`; that solver in turn
  uses `kkt/{mod.rs,qp_assembly.rs,beta_feasibility.rs}` and
  `geom::symplectic_form.rs`.  The exact certificate also directly uses
  `geom::rational_arithmetic.rs` through `rational_solver.rs`.  None of these
  files is identified by the manifest digest.  A commit pin is useful, but it
  does not make a partial list an implementation *closure*, particularly in a
  working tree that can contain uncommitted changes.
* **Repair 3 — not complete.** Every v2 row now has the requested interval,
  iteration, best-admissibility, and beta-margin diagnostics.  But
  `returned_admissible_exact_orbit_count` is not an exact-fallback count: it is
  counted only after zero-gap trimming, and it omits exact resolutions that
  were rejected or trimmed.  It therefore cannot answer how many indeterminate
  f64 candidates MinimaSafe resolved.  The prior
  `f64_indeterminate_candidate_count_before_aggregation` is likewise only a
  pre-aggregation count.  Preserve an explicit
  `exact_fallback_resolution_count` (and ideally an exact-rejected count) from
  the ordinary MinimaSafe aggregation, or label and document an alternative
  counter that exactly has that meaning.

### Exact remaining fixes and recheck condition

1. Extend `capacity-implementation-manifest.json` and its deterministic
   closure digest with, at minimum,
   `crates/symplectic/src/kkt/{mod.rs,saddle_point_solver.rs,qp_assembly.rs,beta_feasibility.rs}`
   and `crates/symplectic/src/geom/{symplectic_form.rs,rational_arithmetic.rs}`
   (plus any other direct non-generated source dependency of the named paths).
   Regenerate the manifest, certificate, target rows, and summary together so
   their manifest digests agree.
2. Add the actual ordinary-MinimaSafe exact fallback resolution count to each
   v2 row, with its definition in the schema/source.  Regenerate the existing
   eight rows and summary; do not add target geometries.

Accept after these two mechanical repairs if the repaired artifacts retain the
same frozen candidate/API/CDF identities, the eight existing target values,
and the q01 certificate agreement.

## Final mechanical-repair recheck (2026-07-12): ACCEPT

No capacity evaluation was rerun for this recheck.  I checked the current
artifacts and the source whose SHA-256 is recorded by the summary.

* The implementation manifest now includes the previously missing ordinary
  f64 KKT route (`kkt/{mod.rs,saddle_point_solver.rs,qp_assembly.rs,
  beta_feasibility.rs}` and `geom/symplectic_form.rs`) as well as the exact
  route's rational arithmetic.  All 16 listed file hashes, the lockfile hash,
  the recorded commit, and the deterministic closure digest match the current
  sources.
* Each of the eight v2 target rows now records
  `exact_fallback_resolution_count` and `exact_fallback_rejected_count` before
  zero-gap trimming.  Counts satisfy `0 <= rejected <= resolutions <=
  f64_indeterminate_candidate_count_before_aggregation`: the `3x6` rows need
  no exact fallback in their MinimaSafe windows; the `4x4` rows record
  `(56,49)`, `(45,41)`, `(53,45)`, and `(86,79)` respectively.  The evaluator
  reconstructs the current private MinimaSafe selection/replacement logic for
  these diagnostics and asserts equality of its pre-trim instrumented minimum
  with the public `aggregate_orbits_with_dual_vertices_exact(..., MinimaSafe)`
  result within `1e-12`.
* The summary hashes match the current target rows, q01 certificate, frozen
  candidates, API verification, CDF artifacts, expanded manifest, and
  evaluator source.  The frozen identities, all eight existing numerical
  values, and the q01 certificate link/agreement remain unchanged.

The two remaining mechanical conditions from the prior rejection are
satisfied.  The packet is accepted for further research reuse and guarded
thesis-facing discussion, subject to the interpretation boundary already
stated above: it certifies the submitted f64-derived rational geometry and is
not independent population, holdout, or symbolic-family evidence.
