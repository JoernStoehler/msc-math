# Frozen orientation target pilot

This packet tests one mechanism question: on the eight reviewed Euclidean
product witnesses, does a Haar `SO(4)` orientation change `sys` through
symplectic alignment? The same source bodies are evaluated under exactly one
`identity`, one Haar `U(2)` control, and one Haar `SO(4)` map per base: 24 rows
in buckets `3x3`, `4x4`, `4x6`, and `6x6`, two bases per bucket. Pairing is only
by `base_id`; facet order is the source order.

The source panel and report are target-free. The evaluator first verifies both
source hashes, the exact 8×3 grid, statuses, IDs, buckets, and absence of target
fields. It reconstructs each source `transformed_dual_vertices_f64` locally with
`SysLandscapePolytopeCache::from_f64_dual_vertices`, then uses an empty,
method-local `ComputedPolytopeCache::load(&[])` and `CapacityBackend::Auto`.
Rows are flushed incrementally. A failure or interruption is retained as a
failed/incomplete manifest and must analyze as `ambiguous_incomplete`.

## Freeze and provenance

`design.json` binds the source/report hashes, selection grid, evaluator source
and implementation closure, backend, formulas, gates, and no-enlargement rule.
Run target-free checks and commit this implementation/design cleanly before
the target command. The target artifact is post-freeze and must record that
pre-target commit. No shared target cache is used.

The target row schema is `generator-orientation-target-pilot-row-v1` and has
source IDs/metadata, `poly_id`, backend, volume, capacity, `sys`, sigma/orbit
scalars, timing, transformed f64 payload, and source/design/evaluator hashes.
`target-manifest.json` records complete/failed status and counts.

## Analysis

`analyze.py` verifies source, source-report, exact-feature, design, target, and
pre-target commit hashes before reading `sys`. It computes for each base
`delta_so4 = sys(so4-haar)-sys(identity)`, `delta_u2`, and the retained exact
feature `delta_ridge` from `symplectic_ridge_area_mean`. It reports every pair.

The U(2) control passes only when `max |delta_u2| <= 1e-8`. The primary frozen
disposition supports a material alignment role when all 24 targets are complete,
the control passes, and at least 6/8 SO(4) deltas have magnitude at least
0.01. It contradicts the role only when complete/control-pass and all SO(4)
deltas have magnitude below 0.005; otherwise it is ambiguous. The ridge-linked
secondary direction requires at least 6/8 opposite nonzero signs and Spearman
rho ≤ −0.5, with explicit average-rank tie handling. Heterogeneity reports
bucket-level two-row changes, leave-one-bucket-out signed means and
median-absolute-delta ranges, common-sign and largest-bucket absolute-share
flags. Heterogeneity or concentration prohibits a common signed-effect or
broad-transfer claim.

No p-value, bootstrap interval, population effect, causal mediation, or law
ranking is warranted at eight frozen witnesses. This packet does not establish
an invariant theorem, a population transfer, or a general mechanism.

## Commands

Target-free, before the freeze commit:

```bash
cargo fmt --check
cargo check -p exp-sys-landscape --bin sys-datascience-generator-orientation-target-pilot
python3 -m unittest discover -s experiments/sys-datascience/methods/generator-orientation-target-pilot -p 'test_*.py'
python3 analyze.py --self-test
python3 experiments/sys-datascience/methods/generator-orientation-target-pilot/select.py
```

After the pre-target commit, run the target under a bounded whole-command wall
timeout (20 minutes):

```bash
timeout 1200s cargo run -p exp-sys-landscape --release \
  --bin sys-datascience-generator-orientation-target-pilot -- \
  --source experiments/sys-datascience/methods/generator-orientation-smoke/artifacts/panel-2-per-bucket/rows.jsonl \
  --source-report experiments/sys-datascience/methods/generator-orientation-smoke/artifacts/panel-2-per-bucket/report.json \
  --design experiments/sys-datascience/methods/generator-orientation-target-pilot/design.json \
  --out experiments/sys-datascience/methods/generator-orientation-target-pilot/artifacts/target-rows.jsonl
python3 analyze.py --target artifacts/target-rows.jsonl --manifest artifacts/target-manifest.json --out artifacts/report.json
```

Expected local cost is minutes. Any timeout, row failure, duplicate,
substitution, or incomplete grid is retained and yields `ambiguous_incomplete`.
