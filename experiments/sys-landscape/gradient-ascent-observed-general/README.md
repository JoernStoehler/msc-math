# Observed General Gradient Ascent: Retained Fixed Panel

This packet retains a completed fixed-`F=10` panel for the current
`iterative_observed_multi_direction_probe` candidate. It is a method-result
packet, not a local-maximality certificate or a source of new `sys > 1`
examples.

## Retention and validation

The tracked raw artifact is
[`artifacts/retained-panel.jsonl`](artifacts/retained-panel.jsonl): the exact,
unmodified concatenation in seed order of the twelve external one-row inputs
`/tmp/observed-general-retained-panel-42-53/seed-{42..53}.jsonl`.

Regenerate the tracked artifact and summary from those raw inputs with:

```bash
python3 analyze_retained_panel.py \
  --input-dir /tmp/observed-general-retained-panel-42-53 \
  --out-dir artifacts
```

The standard-library analyzer validates exactly seeds 42 through 53, one
newline-terminated JSON row per source file, shared schema and parameters
(except each row's one-element seed list), successful completion, trace length
and accepted moves, trace/endpoint statuses, finite attempt statuses, and the
aggregate costs. It writes [`artifacts/summary.json`](artifacts/summary.json).

## Fresh-session reproduction contract

This is a source-to-raw-to-retained procedure.  Run it from a clean checkout
at the source identity recorded below; do **not** write a fresh run into the
tracked `artifacts/` directory.  The retained panel was introduced in commit
`dadf7aa824e3d0cd237f45d666ae894b86c600fe`.  At that commit, and at the
current version of this packet, the producer and analyzer SHA-256 digests are
respectively
`2642d4390939ba9d43f6515bfdd0add16263d65564c2a45d4a1fd015220fa9e9` and
`703212db0d3d501f7bdced290c29e024022b034ca84b335fa6558939ace806cb`.

First record the checkout identity and reject local changes to the producer,
analyzer, or dependency lockfile.  If the commit is not the retained-panel
commit, matching the two source hashes above establishes the relevant
producer/analyzer identity; also retain the recorded `Cargo.lock` hash and
full commit ID with the new raw directory.

```bash
git rev-parse HEAD
git status --short -- \
  Cargo.lock \
  experiments/sys-landscape/gradient-ascent-observed-general/main.rs \
  experiments/sys-landscape/gradient-ascent-observed-general/analyze_retained_panel.py
sha256sum \
  Cargo.lock \
  experiments/sys-landscape/gradient-ascent-observed-general/main.rs \
  experiments/sys-landscape/gradient-ascent-observed-general/analyze_retained_panel.py
```

Build the exact binary once, then make one independently named, one-row raw
file for every seed.  `--out` is mandatory here so the output location and
input naming are independent of the binary's timestamped default.

```bash
set -euo pipefail
packet=experiments/sys-landscape/gradient-ascent-observed-general
run_dir=/tmp/observed-general-retained-panel-42-53-reproduction
raw_dir="$run_dir/raw"
retained_dir="$run_dir/retained"
rm -rf "$run_dir"
mkdir -p "$raw_dir"

cargo build -p exp-sys-landscape --release \
  --bin sys-gradient-ascent-observed-general
for seed in $(seq 42 53); do
  cargo run -p exp-sys-landscape --release \
    --bin sys-gradient-ascent-observed-general -- \
    --seed "$seed" --retained-preflight --out "$raw_dir/seed-$seed.jsonl"
done
python3 "$packet/analyze_retained_panel.py" \
  --input-dir "$raw_dir" --out-dir "$retained_dir"
```

The analyzer is the retention gate: it rejects a missing, extra, misnamed, or
non-single-row raw file before it writes the concatenation.  Check both that
the analyzer's retained JSONL is the exact byte concatenation of these raw
inputs and that its summary describes this run directory and all twelve seeds.

```bash
cat "$raw_dir"/seed-{42..53}.jsonl >"$run_dir/manual-concatenation.jsonl"
cmp -- "$run_dir/manual-concatenation.jsonl" "$retained_dir/retained-panel.jsonl"
test "$(wc -l <"$retained_dir/retained-panel.jsonl")" -eq 12
python3 - "$retained_dir/summary.json" "$raw_dir" <<'PY'
import json
import sys

summary = json.load(open(sys.argv[1], encoding="utf-8"))
assert summary["input_directory"] == sys.argv[2]
assert summary["validated_seeds"] == list(range(42, 54))
assert summary["run_count"] == summary["completed_runs"] == 12
assert summary["failed_runs"] == 0
assert summary["operational_failure_count"] == 0
PY
sha256sum "$raw_dir"/seed-{42..53}.jsonl \
  "$retained_dir/retained-panel.jsonl" "$retained_dir/summary.json"
```

Do not require byte identity with the tracked panel or summary: elapsed-time
fields (and the summary's absolute input-directory field) are run-specific.
The required identities are (1) the recorded source/lockfile identity, and
(2) within the fresh run, the exact raw-to-retained concatenation above.  Use
the analyzer's validation and its resulting statuses—not a byte comparison to
the historical artifact—to decide whether a fresh panel satisfies the same
retention contract.

## Fixed run parameters

Every row records `run_mode = retained_preflight`, a one-element seed list,
branch threshold `1e-3`, action window `1e-2`, observed acceptance threshold
`max(0, 1e-3 * abs(base_sys))`, trace steps `1e-3,1e-4`, trace cap `8`, and
endpoint steps `1e-3,1e-4,1e-5,1e-6`. Each accepted trace move recomputes the
branch-derived directions at the new state. The endpoint scan tests every
generated direction against every endpoint step.

## Direct results

All 12 runs completed without an operational failure, and all 12 accepted all
eight trace moves. The mean `sys` increase was `0.011565`; its seedwise range
was `0.003579` to `0.042455`. Measured total compute was `400.889 s`, with 204
finite-step evaluations and 405,772 capacity-orbit iterations.

All 12 traces stopped at the iteration cap rather than the candidate's
all-generated-directions stop condition. Consequently every endpoint result is
`not_evaluable_trace_did_not_stop`, and all 12 exhaustive endpoint scans found
at least one above-threshold finite move. The panel therefore supports
systematic finite-step ascent progress and an observed bounded cost profile;
it refutes, and does not support, a heuristic endpoint claim at cap 8.

The full per-seed values, statuses, and costs are in
[`artifacts/summary.json`](artifacts/summary.json), not duplicated here.

## Raw naming defect

The retained raw rows still use schema
`gradient_ascent_observed_general_smoke_v1` and run IDs of the form
`observed-general-smoke-seed-N`, although their raw `purpose` is
`retained_mode_one_seed_preflight`. This is a naming defect in the producer
surface. The analyzer flags it in `naming_defects`; the artifact preserves it
rather than silently relabeling historical run identity.

## Boundary

The finite endpoint condition would concern only the candidate-generated
directions and checked steps after the trace actually stops. It does not cover
all nearby directions or branches, and it is weaker than local maximality.
Here it is not even evaluable, because every run exhausted cap 8 while finite
above-threshold moves remained. See
[`../../dev-gradient-ascent/METHOD-CANDIDATE.md`](../../dev-gradient-ascent/METHOD-CANDIDATE.md)
and [`../../dev-gradient-ascent/PROMOTION-READINESS.md`](../../dev-gradient-ascent/PROMOTION-READINESS.md)
for the broader candidate limits.
