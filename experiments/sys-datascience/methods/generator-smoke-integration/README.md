# Generator-smoke integration bridge

This is an opt-in, method-local bridge for the reviewed
`alternative-generator-smoke` rows. It creates sidecars beside this packet;
it does not invoke or modify the retained producer/prepare path and never
writes `polytope-table.jsonl`.

## Command

Inputs are named explicitly so a later method cannot silently mix populations:

```bash
python3 experiments/sys-datascience/methods/generator-smoke-integration/bridge.py \
  --input geometry=experiments/sys-datascience/methods/alternative-generator-smoke/artifacts/smoke-rows.jsonl \
  --out-dir /tmp/generator-smoke-geometry-sidecar

python3 experiments/sys-datascience/methods/generator-smoke-integration/bridge.py \
  --input target-pilot=experiments/sys-datascience/methods/alternative-generator-smoke/artifacts/target-pilot/smoke-rows.jsonl \
  --out-dir /tmp/generator-smoke-target-sidecar
```

Repeat `--input NAME=PATH` to combine distinct, non-overlapping smoke files.
The bridge rejects duplicate sample IDs, repeated generator identities,
conflicting pairing identities, malformed/non-finite numeric values, and
incompatible row schema versions. Source paths and SHA-256 fingerprints are
recorded in every provenance row and in `integration-report.json`.
Canonical `altgen-v2/...` IDs are checked against the row law, resolved
parameter, seed, row/attempt, and `kxm` bucket; the bucket side counts must sum
to `facet_count`.

Outputs are:

- `generator-smoke-provenance.jsonl`: one row for every input row, including
  rejected or target-censored rows. It records law/version, parameter, seed,
  row/attempt identity, bucket/facet count, pairing, validation, acceptance,
  target status, source fingerprint, and capability flags.
- `generator-smoke-prepared.jsonl`: one method-local row per input row. The
  `geometry` object retains all non-target fields present in the source row;
  `sys`, `capacity`, `iterations`, and `target_ms` are present only when the
  target was actually evaluated. `missing_geometry_fields` and
  `censored_fields` make absent and target-censored values explicit. Use
  the exact shared `provenance_id` field to join to the provenance sidecar;
  `join_key` is the generator sample identity within that source.
- `integration-report.json`: deterministic counts, fingerprints, and the
  blocked feature families.

`runtime_cap` rows are reported as `target_status=skipped_runtime_cap`, not as
failed targets. Rows with the target backend disabled are
`target_status=not_requested`. The small populations remain named generator
smokes and are not aliases for the retained current-law random/product data.

## Method-local consumer example

The following recipe is deliberately separate from existing methods:

```bash
python3 experiments/sys-datascience/methods/generator-smoke-integration/consume_sidecar.py \
  --prepared /tmp/generator-smoke-target-sidecar/generator-smoke-prepared.jsonl \
  --law baseline --require-target
```

It filters the sidecar by law and reports evaluated rows only. A method should
keep its own input flag pointing at a sidecar path (for example
`--generator-smoke-prepared PATH`), and must retain the report's named-population
and censoring boundary in its interpretation.

## Available and blocked features

Available without inventing values: law/parameter and sampling identity,
pair/facet bucket, acceptance and validation status, factor areas/support-gap
CVs/isoperimetric ratios, normalized product volume, and `sys`/capacity only
for the 42 target-evaluated target-pilot rows. The geometry smoke has 270
accepted rows and no evaluated targets; its target fields are explicitly
`not_requested`.

The standard prepared feature families blocked by this schema are face-lattice
combinatorics, ordered two-face symplectic-area summaries, exact dual-vertex
volume reconstruction, and cached exact capacity fields. The rows contain no
full exact/cache payload, so a cleanly missing feature is preferable to a
fabricated value.

The smallest upstream extension that unlocks the current prepare families is a
producer payload per accepted row containing `poly_id`, ordered
`dual_vertices` (f64), `dual_vertices_rational`, `vertices_rational`, and the
exact-validation/cache identity used for volume/capacity. This should be a
future generator-smoke producer addition, not a change to this bridge or to
the retained tables.

## Checks and deterministic demonstration

```bash
python3 -m unittest discover -s experiments/sys-datascience/methods/generator-smoke-integration -p 'test_*.py'
python3 experiments/sys-datascience/methods/generator-smoke-integration/bridge.py \
  --input geometry=experiments/sys-datascience/methods/alternative-generator-smoke/artifacts/smoke-rows.jsonl \
  --out-dir experiments/sys-datascience/methods/generator-smoke-integration/artifacts/geometry
python3 experiments/sys-datascience/methods/generator-smoke-integration/bridge.py \
  --input target-pilot=experiments/sys-datascience/methods/alternative-generator-smoke/artifacts/target-pilot/smoke-rows.jsonl \
  --out-dir experiments/sys-datascience/methods/generator-smoke-integration/artifacts/target-pilot
```

The checked-in artifact reports show 272 geometry rows (270 accepted) and 68
target-pilot rows (42 evaluated, 25 runtime-capped, one exhausted row). These
are plumbing and censoring demonstrations, not transfer evidence. Reopen this
bridge when the smoke producer adds the exact/cache payload or a named method
needs a stronger feature family; do not promote these sidecars into retained
tables.
