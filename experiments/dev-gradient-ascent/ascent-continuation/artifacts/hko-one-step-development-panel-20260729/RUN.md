# HKO one-step development panel run record

The raw producer was commit `219e50aaad54020a9e9946c91392679bd17019a0`.
Its frozen input was
`experiments/dev-gradient-ascent/ascent-continuation/inputs/hko-panel.json`,
SHA-256 `1f1666e4c5964ac76825aaa60e684c2220d769e0b95d10733a5b84fbc8668f48`.

```bash
cargo run --release -p exp-dev-gradient-ascent \
  --bin dev-gradient-ascent-ascent-continuation -- \
  --mode full \
  --states-json experiments/dev-gradient-ascent/ascent-continuation/inputs/hko-panel.json \
  --out-dir experiments/dev-gradient-ascent/ascent-continuation/artifacts/hko-one-step-development-panel-20260729/raw
```

The successful run used 140.10 user CPU seconds, 0.18 system seconds, 140.29
wall seconds, and at most 57,648 KiB resident memory. The producer's own timer
records 140.125 seconds in `raw/summary.json`.

The analyzer is:

```bash
uv run --script \
  experiments/dev-gradient-ascent/ascent-continuation/analyze_hko_calibration.py \
  experiments/dev-gradient-ascent/ascent-continuation/artifacts/hko-one-step-development-panel-20260729/raw \
  experiments/dev-gradient-ascent/ascent-continuation/artifacts/hko-one-step-development-panel-20260729/analysis
```

The four directions and four distances are a development calibration panel.
They were frozen before the run. The other retained HKO random rays were not
read by the input generator and remain available for held-out evaluation after
the proposal rules are frozen.
