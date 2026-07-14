# Adaptive direction-model ablation

This packet compares three normalized adaptive branch-gradient directions on
the frozen six-start generic-random `F=6` panel: the deterministic current
minimizer branch, zero-gap near-active maximin, and the finite-radius
gap-aware candidate-window maximin.  Every proposal is an exact full-`sys`
evaluation and is retained in raw JSONL with branch-set and prediction
telemetry.  The generated `analysis.json`, `DISCUSSION.md`, and figures are
the claim-bearing outputs; `run-provenance.json` records source and parameter
identity.

The retained comparison is descriptive: two role-labelled fixtures (one
mechanism-disagreement case and one equality/easy control), three policies,
three initial radii, and six proposals per cell (108 exact target proposals).
It is not a convergence or stationarity claim. The canonical six-start
generic-random screening is retained separately under `artifacts/screening`.

The canonical six-start generic-random smoke is retained separately. The
claim-bearing slice uses `inputs/mechanism_disagreement.jsonl` (f6be75…f1b8)
and `inputs/equality_easy_control.jsonl` (43d243…dec8cc), with six proposals
per policy/radius cell to exercise the mechanism within the bounded packet
cost; this is not a replacement for a 100-evaluation census.

The primary branch windows are the established near-active relative threshold
`1e-3` and candidate action window `1e-2`; both are recorded in
`artifacts/run-provenance.json`.

## Reproduce

```bash
cargo run --release -p exp-dev-gradient-ascent \
  --bin dev-gradient-ascent-adaptive-direction-ablation -- \
  --budget 6 --facet-count 0 --start-count 2 --exclude-start-ids never \
  --polytope-table experiments/dev-gradient-ascent/adaptive-direction-ablation/inputs/selected-fixtures.jsonl \
  --out-dir experiments/dev-gradient-ascent/adaptive-direction-ablation/artifacts

uv run --script experiments/dev-gradient-ascent/adaptive-direction-ablation/analyze.py \
  experiments/dev-gradient-ascent/adaptive-direction-ablation/artifacts
```

The screening command was:

```bash
cargo run --release -p exp-dev-gradient-ascent \
  --bin dev-gradient-ascent-adaptive-direction-ablation -- \
  --smoke --budget 1 --facet-count 6 --start-count 6 \
  --exclude-start-ids random_F6_s0_1 \
  --polytope-table experiments/sys-datascience/produce/random.jsonl \
  --out-dir experiments/dev-gradient-ascent/adaptive-direction-ablation/artifacts/screening
python3 experiments/dev-gradient-ascent/adaptive-direction-ablation/analyze_screening.py \
  experiments/dev-gradient-ascent/adaptive-direction-ablation/artifacts/screening
```

The producer's default 100-evaluation budget is not the retained command; use
the explicit bounded `--budget 6` above when regenerating this packet.

The separate generic panel uses the same command with `--facet-count 6
--start-count 6 --exclude-start-ids random_F6_s0_1`, source
`experiments/sys-datascience/produce/random.jsonl`, and output directory
`artifacts/generic-panel`; validate it with
`python3 .../analyze.py artifacts/generic-panel generic`. Its six starts are
the exact screening IDs recorded in `inputs/generic-start-manifest.json`.
