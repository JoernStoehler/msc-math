# Sys-Landscape Reasoning

This topic is now documented as a three-file local note set to support future work without opening `research/sys-landscape/design/*.md`.

## Current Scientific State

`sys-landscape` now has a stable experimental surface across five families: random generic polytopes, random Lagrangian products, fixed-F gradient ascent (general and Lagrangian products), and F-continuation. The evidence is consistent with the current thesis framing: random IID sampling hits only moderate `sys` values, and structured continuation can improve endpoints but has not moved above `sys=1`.

### Evidence surface

- Random generic polytopes (`random-sample`): `70` rows, max `sys=0.739`, no `sys>1`.
- Random products (`random-product-sample`): `100` rows, max `sys=0.794`, no `sys>1`.
- Fixed-F general ascent (`gradient-ascent-general`): `10` seeds, max `sys=0.9030`; all seeds used escape logic; no `sys>1`.
- Fixed-F Lagrangian product ascent (`gradient-ascent-products`): `12` seeds, max `sys=0.8727`; no `sys>1`.
- Variable-F continuation (`variable-f-ascent`): `90` trials total, including random-seed RQ2 and `10` RQ1 local maxima starts; improvements from F=10 to F=11 are common, but still below the conjecture threshold.
- Regular polygon structure evidence (`rotated-regular-products`, `pentagon-rotation-formula`): confirmed `sys>1` at the known pentagon-pentagon rotation (`theta=18°`) and no further tested regular family violations.

### Key current implications

- The hostile-landscape hypothesis is still favored by current bounded experiments: signal that predicts high `sys` on random baselines does not transfer to fixed-F ascent endpoints, and endpoint-side gains remain largely regime-specific.
- The local continuation line (`general`, `products`, and `variable-f-ascent`) suggests optimization behavior is highly local in the labeled dual-vertex coordinates used by current code.
- The known counterexample remains isolated to the special pentagon rotation geometry; raw sampling plus naive local ascent has not discovered a second comparable regime.
- `random-f->11` continuation is useful for benchmarking but weak as a generic mechanism for surpassing known bounds.

## Data and Interface Commitments

- `feature-pattern-search` set a durable schema for normalizing outputs:
  - geometry core: `experiments/sys-landscape/normalized-dataset/polytopes.jsonl`
  - occurrences: `states.jsonl`
  - scalar target table: `capacity_results.jsonl`
  - step-level fixed-F trace table: `step_events.jsonl`
- `states` and `polytope` identities are split (`state_id` and `poly_id`) with explicit lineage metadata (`root_group_id`, `lineage_id`, `parent_state_id`) when available.
- HKO-local continuation/evidence in `experiments/hko-local-maximum/cut-and-ascent/` is intentionally not in the default hostile-landscape modeling surface.

## Current Package Surface

- `random-sample`, `random-product-sample`, `rejection-calibration`, `rotated-regular-products`, `gradient-ascent-general`, `gradient-ascent-products`, `variable-f-ascent`, `feature-pattern-search`, `normalized-dataset`, and the `feature-*` extractor binaries are the stable local evidence surface.
- The former `feature-*` style notes are now consolidated here: `feature-pattern-search` is the only place needed to recover modeling implications, and `witness-search-program` maps directly to the successor program.

## Why this is enough for a next agent

The current files preserve: exact commands for smoke/LICCA production behavior, current packet dimensions and maxima, why certain packets are treated as bounded/local vs historical, and which modeling lines have already been closed by results. A future agent can continue either (a) normalizing additional output packets into the same schema, or (b) implementing witness-aware continuation using current local files as constraints.
