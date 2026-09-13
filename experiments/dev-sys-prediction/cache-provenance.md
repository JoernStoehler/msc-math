# Prediction cache and evaluator provenance

`src/sysext_cache.rs` keys scalar results by ordered rationalized geometry,
capacity method and volume method. Newly computed rows identify the current
producer as `legacy-orbit-search-v1` capacity with
`f64-from-exact-derived-incidence-v1` volume. These labels describe the existing
`compute_sys_computation` route; they do not select a different evaluator.

Rows predating these fields deserialize with both methods set to `unknown`.
They remain readable and may supply their stored exact geometry for polytope
reconstruction, but their scalar payload cannot hit a request for the current
methods. A miss appends a separately identified current row, so an unknown and
a current scalar payload for the same geometry can coexist without changing or
relabeling the old row. Conflicting payloads are still rejected within one
fully identified method tuple, and geometry conflicts are rejected across
tuples.

## Source-backed volume boundaries

- `prediction_cloud.rs::compute_base_state_from_polytope` explicitly calls
  `reference::exact_volume_as_f64`: rational volume, then binary64 rounding.
- `SysextCache::compute` calls `sys-landscape::compute_sys_computation` on a
  miss. Since `1fbf9809` (July 26), that helper uses f64 volume from exact-derived
  incidence. Capacity uses the retained legacy frontend, not the certified
  scalar API.
- Matching identified hits return stored scalars. The cache is shared by local
  probes, step-ranking
  audits, traces, line searches and endpoint diagnostics in `prediction_cloud`.
  The panel runner passes its paths; the standalone sigma-line probe does not
  use this cache.

The July change was not a change to the then-current prediction-cache schema.
Treating an unlabelled row as automatically reference-volume or automatically
current f64 would invent provenance. Such rows are now explicitly unknown and
are not scalar-hit eligible. The error decomposition still telescopes under
mixed methods, but this does not isolate prediction error from evaluator
differences.

## Retained June artifact

`facet-scale-baseline-error/local-decomp-cloud/sysext-cache.jsonl` entered at
`57bb7730` (June 30), when `compute_sys_computation` still used rational volume
rounded to f64. Its associated summary and compute-budget report record 111
misses, zero hits and 111 used rows. Source history therefore supports reference
volume for this particular retained run. The later `3fae5696` change concerns
Git LFS scope, not a documented scientific regeneration.

This is provenance evidence for that artifact, not a default rule for arbitrary
unlabelled cache files. These source/history checks did not recompute the panel.
Retained files remain unchanged.

## Enforced cache boundary

A scalar cache hit matches geometry, capacity method and volume method. Newly
computed rows carry those identities; old rows without provenance remain
distinguishable as unknown. Geometry reuse does not imply scalar reuse.

The cache migration itself does not choose reference-versus-production policy:
it records the production route that this producer already calls and recomputes
unknown scalar rows in that method namespace. A provenance import for a
retained artifact must still be tied to that particular artifact, not to all
files lacking metadata.
