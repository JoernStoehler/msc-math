# Frozen orientation target pilot

This packet tests one mechanism question: on the eight reviewed Euclidean
product witnesses, does a Haar `SO(4)` orientation change `sys` through
symplectic alignment? The same source bodies are evaluated under exactly one
`identity`, one Haar `U(2)` control, and one Haar `SO(4)` map per base: 24 rows
in buckets `3x3`, `4x4`, `4x6`, and `6x6`, two bases per bucket. Pairing is only
by `base_id`; facet order is the source order.

The source panel and report are target-free. The repaired HEAD verifies both
source hashes, the exact 8×3 grid, statuses, IDs, buckets, implementation
closure, and absence of target fields before any target path could be opened.
It is validation-only and makes no reconstruction or capacity call. The
historical evaluator contract (local reconstruction, empty method-local cache,
and `CapacityBackend::Auto`) is bound to the retained pre-rerun commit
`a59441c0ecde29ac667745e02aac4bedb8ca7d14`. The retained artifact is complete;
failed/partial/incomplete manifests are rejected as non-interpretable rather
than treated as results.

## Freeze and provenance

`design.json` binds the source/report hashes, selection grid, evaluator source
and implementation closure, backend, formulas, gates, and no-enlargement rule.
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
artifact and has the immutable SHA-256 recorded in the manifest.

The retained rows necessarily carry the evaluator and design hashes from that
rerun. The repaired producer binds the current evaluator/design hashes for
future validation, while the manifest records both current and retained hashes;
the analyzer checks each side against its corresponding commit/artifact.

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

`--validate-only` performs the complete target-free producer freeze check,
including implementation-closure hashes and absence of every forbidden target
key (even a JSON `null`); it never reconstructs or calls capacity. The analyzer
rejects every failed/partial/incomplete manifest and raises on corrupt partial
or complete artifacts.

## Commands

Target-free, before the freeze commit:

```bash
cargo fmt --check
cargo check -p exp-sys-landscape --bin sys-datascience-generator-orientation-target-pilot
cargo run -p exp-sys-landscape --bin sys-datascience-generator-orientation-target-pilot -- --validate-only
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
cargo run -p exp-sys-landscape --bin sys-datascience-generator-orientation-target-pilot -- --validate-only
```

The retained analyzer is the only command that reads the 24-row target
artifact. Any timeout, row failure, duplicate, substitution, or incomplete
grid in a newly produced historical artifact is rejected as non-interpretable.
