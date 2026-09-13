# Prediction cache and evaluator provenance

The current `src/sysext_cache.rs` keys scalar results by ordered rationalized
geometry only. Rows store volume, minimum action, `sys`, candidate summaries,
iteration count and optional geometry, but no evaluator or volume-method
identity. Duplicate keys with differing scalar payloads are rejected; mixed
methods on different keys are not detected.

## Source-backed volume boundaries

- `prediction_cloud.rs::compute_base_state_from_polytope` explicitly calls
  `reference::exact_volume_as_f64`: rational volume, then binary64 rounding.
- `SysextCache::compute` calls `sys-landscape::compute_sys_computation` on a
  miss. Since `1fbf9809` (July 26), that helper uses f64 volume from exact-derived
  incidence. Capacity uses the retained legacy frontend, not the certified
  scalar API.
- Hits return stored scalars. The cache is shared by local probes, step-ranking
  audits, traces, line searches and endpoint diagnostics in `prediction_cloud`.
  The panel runner passes its paths; the standalone sigma-line probe does not
  use this cache.

The July change was not a change to the prediction-cache schema. Treating an
unlabelled row as automatically reference-volume or automatically current f64
would invent provenance. The error decomposition still telescopes under mixed
methods, but this does not isolate prediction error from evaluator differences.

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

## Boundary needed for a future cache migration

A scalar cache hit must match both geometry and the requested evaluator/volume
identity. Newly computed rows can carry those identities; old rows without
provenance must remain distinguishable. Geometry reuse need not imply scalar
reuse. Merely adding labels while continuing geometry-only lookup would leave
mixed evaluations possible.

The reference-versus-production policy for new runs must be explicit before
migrating the cache. A new cache namespace can recompute unknown scalar rows
without overwriting old evidence; a provenance import for a retained artifact
must be tied to that particular artifact, not to all files lacking metadata.
