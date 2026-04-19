# Sys-Landscape Decisions

## Durable Decisions Worth Keeping

- Use normalized, joinable dataset normalization (`poly_id` + `state_id`) before adding more methods.
  - Reason: fixed-F trace artifacts currently mix provenance and geometry roles; one-table redesign would force repeated rewrites.
  - Implementation consequence: `normalized-dataset` remains the only required ingestion contract.
- Preserve ordered dual-vertex geometry as the primary `poly_id` source and store exact rational payload (`dual_vertices_rational`) with each `poly_id`.
  - Reason: many downstream features (`omega`, skeleton, ridge summaries, continuation IDs) rely on consistent facet indexing.
- Exclude HKO-specific control/sensitivity packets from the default hostile-landscape model surface.
  - `hko-local-maximum/cut-and-ascent/cut-and-ascent.jsonl` is held for optional control, not in the principal closure.
- Use experiment-specific seeds and CLI seeds consistently in `gradient-ascent-*` binaries (including `--fresh`, per-seed RNG streams, `--no-db-update` under parallel execution).
  - No large local reruns (`N=1000`) are allowed; production scaling belongs on LICCA with wall-time overrides.
- Carry LICCA `tripwire` (`#SBATCH --time=00:00:01`) as a hard guardrail and require CLI `--time` for actual runs.

## Retained Rejections / Non-Choices

- Reject rebuilding a wide one-off table from committed JSONLs.
  - The temporary gain is outweighed by immediate forked schema changes once new enrichment joins arrive.
- Reject treating raw witness permutations as a transferable global cache across unrelated seeds.
  - Current evidence suggests near-canonical geometry mismatch makes raw labels weak without local lineage.
- Reject F-increase before optimization in continuation (`add+F+1` before F-level ascent) as a default continuation strategy.
  - Variable-F calibration shows it is usually dominated by optimize-first paths.
- Reject interpreting safe reduced-model outcomes as final claims.
- New candidate values must still pass exact witness checks before claims of best exact `sys` are accepted.

## Open Constraints to Respect

- The local topic should stay in the experiments package even for algorithmic changes.
- Existing artifact schema should be additive: new packets and enrichers should fit the `normalized-dataset` contract where feasible.
- A direct continuation replacement from random addition to witness-guided splitting is pending; do it as a measured comparison, not a blind replacement.
