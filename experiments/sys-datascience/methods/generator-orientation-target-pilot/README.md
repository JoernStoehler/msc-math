# Frozen orientation target pilot

This is a narrow salvage from the stopped breadth wave. It retains only the
orientation smoke, this target pilot, and a 40-row orientation-only feature
snapshot. The 808-row `generator-exact-feature-augmenter` owner, its tangential
inputs/replay, and all other generator breadth folders are deliberately not
copied. `extract_orientation_features.py` records the original full-feature
and report hashes and copies the reviewed orientation lines without
re-serializing or recomputing them.

Transplant inventory:

- retained files: `generator-orientation-smoke/README.md`,
  `generator-orientation-smoke/src/main.rs`,
  `generator-orientation-smoke/artifacts/panel-2-per-bucket/report.json`,
  `generator-orientation-smoke/artifacts/panel-2-per-bucket/rows.jsonl`;
  `generator-orientation-target-pilot/README.md`, `analyze.py`, `design.json`,
  `src/main.rs`, `protocol-history.json`, `select.py`, `selection-manifest.json`,
  `test_analyze.py`, `extract_orientation_features.py`,
  `artifacts/report.json`, `artifacts/target-manifest.json`,
  `artifacts/target-rows.jsonl`,
  `artifacts/orientation-feature-manifest.json`, and
  `artifacts/orientation-features.jsonl`; plus the two binary registrations in
  the historical binary registrations in
  `experiments/sys-landscape/Cargo.toml` (the current registrations are in
  `experiments/sys-datascience/Cargo.toml`);
- explicitly excluded: `generator-exact-feature-augmenter/` (apart from its
  bound 40-row snapshot), `generator-tangential-matchability/`,
  `generator-zoo-smoke/`, `generator-quality-atlas/`,
  `generator-smoke-integration/`, `generator-sys-effects/`,
  `generator-target-quotient-distance/`, and all other stopped generator-law,
  distribution, quality, and transfer breadth owners.

This packet tests one mechanism question: on the eight reviewed Euclidean
product witnesses, does a Haar `SO(4)` orientation change `sys` through
symplectic alignment? The same source bodies are evaluated under exactly one
`identity`, one Haar `U(2)` control, and one Haar `SO(4)` map per base: 24 rows
in buckets `3x3`, `4x4`, `4x6`, and `6x6`, two bases per bucket. Pairing is only
by `base_id`; facet order is the source order.

The source panel and report are target-free. The repaired HEAD warns about
source or implementation byte drift and verifies the exact 8×3 grid, statuses,
IDs, buckets, and absence of target fields before any target path could be opened.
It is validation-only and makes no reconstruction or capacity call. The
historical evaluator contract (local reconstruction, empty method-local cache,
and `CapacityBackend::Auto`) is bound to the retained pre-rerun commit
`a59441c0ecde29ac667745e02aac4bedb8ca7d14`. The retained artifact is complete;
failed/partial/incomplete manifests are rejected as non-interpretable rather
than treated as results.

## Freeze and provenance

`design.json` binds the source/report hashes, selection grid, evaluator source
and implementation closure, the narrowed feature snapshot and its original
full-artifact hashes, backend, formulas, gates, and no-enlargement rule.
Run target-free checks and commit this implementation/design cleanly before
the target command. The target artifact is post-freeze and must record that
pre-target commit. No shared target cache is used.

The durable machine-readable protocol history is `protocol-history.json`.
The design/evaluator were first committed at
`dfbcc400b8cc39c60f8e8c22f8e9ed95acc229be`; source-variant validation was
repaired before the run at `f5f38f351576e7eccb5a51242ff95211ed7b8761`.
Twenty-four values were exposed after that commit, but those outputs were
rejected and deleted: the design had one mistyped
`experiments/sys-landscape/src/lib.rs` SHA-256 nibble and the analyzer used
the wrong repository-parent depth. They were not analyzed or retained.
`a59441c0ecde29ac667745e02aac4bedb8ca7d14` is therefore the valid
pre-retained-rerun commit, not the first absolute pre-target freeze; it changed
only those two provenance/audit lines, not selection, evaluator, formulas,
gates, or interpretation. The retained full 24-row rerun occurred after
`a59441c0`; `artifacts/target-rows.jsonl` is its only retained/analyzed target
artifact and has its retained SHA-256 recorded in the manifest as provenance.

The retained rows necessarily carry the evaluator and design hashes from that
rerun. The repaired producer binds the current evaluator/design hashes for
future validation, while the manifest records both current and retained hashes;
the analyzer checks each side against its corresponding commit/artifact.

The target row schema is `generator-orientation-target-pilot-row-v1` and has
source IDs/metadata, `poly_id`, backend, volume, capacity, `sys`, sigma/orbit
scalars, timing, transformed f64 payload, and source/design/evaluator hashes.
`target-manifest.json` records complete/failed status and counts.

## Analysis

`analyze.py` warns when source, source-report, orientation-feature snapshot,
design, target, or pre-target revision identities drift, then applies blocking
semantic checks before reading `sys`. It computes for each base
`delta_so4 = sys(so4-haar)-sys(identity)`, `delta_u2`, and the retained exact
feature `delta_ridge` from `symplectic_ridge_area_mean`. It reports every pair.
The generated `generator-orientation-target-pilot-transplant-report-v1` keeps
all decision-relevant pair values and disposition byte/field-equivalent to the
accepted `a92ca3e6` report; only provenance fields and the report schema are
versioned for the narrowed closure.

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

The result is witness-level evidence only: Haar `U(2)` preserves `sys` to the
numerical control tolerance, Haar `SO(4)` changes it materially on 7/8
witnesses, and the selected ridge mediator fails its predeclared directional
gate (5/8 opposite signs). It supports no population, common-sign, causal,
proposer-transfer, or general-generator claim.

`--validate-only` performs the complete target-free producer check, warning on
implementation-closure byte drift and rejecting every forbidden target key
(even a JSON `null`); it never reconstructs or calls capacity. The analyzer
rejects every failed/partial/incomplete manifest and raises on corrupt partial
or complete artifacts.

## Commands

To rebuild only the retained orientation feature closure (the stopped
breadth-wave full artifact must be supplied by its owner; it is not a packet
input), run from this directory:

```bash
python3 extract_orientation_features.py \
  --full-features /path/to/generator-exact-feature-augmenter/artifacts/full-panels/features.jsonl \
  --full-report /path/to/generator-exact-feature-augmenter/artifacts/full-panels/report.json
```

Target-free, before the freeze commit:

```bash
cargo fmt --check
cargo check -p exp-sys-datascience --bin sys-datascience-generator-orientation-target-pilot
cargo run -p exp-sys-datascience --bin sys-datascience-generator-orientation-target-pilot -- --validate-only
python3 -m unittest discover -s experiments/sys-datascience/methods/generator-orientation-target-pilot -p 'test_*.py'
python3 analyze.py --self-test
python3 experiments/sys-datascience/methods/generator-orientation-target-pilot/select.py
```

The repaired HEAD is deliberately validation-only: it rejects target execution
before opening output or calling capacity. The historical target evaluator and
its manifest-writing path are reproducible only by checking out the retained
pre-rerun commit `a59441c0ecde29ac667745e02aac4bedb8ca7d14`; do not run that
historical command against this worktree or regenerate retained rows.

For provenance review, the current branch command is:

```bash
cargo run -p exp-sys-datascience --bin sys-datascience-generator-orientation-target-pilot -- --validate-only
```

The retained analyzer is the only command that reads the 24-row target
artifact. Any timeout, row failure, duplicate, substitution, or incomplete
grid in a newly produced historical artifact is rejected as non-interpretable.
