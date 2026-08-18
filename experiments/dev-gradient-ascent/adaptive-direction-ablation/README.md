# Adaptive direction-model ablation

This packet compares four adaptive directions: an L-infinity-scaled raw
current-minimizer gradient ray, the direct single-branch L-infinity
box-steepest sign control, zero-gap near-active box-LP maximin, and
finite-radius gap-aware candidate-window box-LP maximin. Every proposal is an exact full-`sys`
evaluation and is retained in raw JSONL with branch-set and prediction
telemetry.  The generated `analysis.json`, `DISCUSSION.md`, and figures are
the claim-bearing outputs; `run-provenance.json` records source and parameter
identity.

The retained comparison is descriptive: two role-labelled fixtures (one
mechanism-disagreement case and one equality/easy control), four policies,
three initial radii, and six proposals per cell (144 exact target proposals).
It is not a convergence or stationarity claim. The canonical six-start
generic-random screening is retained separately under `artifacts/screening`.

The canonical six-start generic-random smoke is retained separately. The
claim-bearing slice uses `inputs/mechanism_disagreement.jsonl` (f6be75…f1b8)
and `inputs/equality_easy_control.jsonl` (43d243…dec8cc), with six proposals
per policy/radius cell to exercise the mechanism within the bounded packet
cost; this is not a replacement for a 100-evaluation census. The generic
panel has 432 proposals and the screening smoke has 24 proposals. Near-active
versus single-branch comparisons use an operational descriptive tie tolerance
of `1e-8`; it is not a statistical significance threshold. Singleton
near-active and candidate windows are canonicalized to the same coordinatewise
sign direction as the direct control, avoiding arbitrary LP values on zero
gradient coordinates.

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

# Optional readable PNG/SVG comparison figures (requires matplotlib).
uv run --with matplotlib --no-project python3 \
  experiments/dev-gradient-ascent/adaptive-direction-ablation/plot_panels.py \
  experiments/dev-gradient-ascent/adaptive-direction-ablation/artifacts
```

The screening command was:

```bash
cargo run --release -p exp-dev-gradient-ascent \
  --bin dev-gradient-ascent-adaptive-direction-ablation -- \
  --smoke --budget 1 --facet-count 6 --start-count 6 \
  --exclude-start-ids random_F6_s0_1 \
  --polytope-table experiments/polytope-datasets/random.jsonl \
  --out-dir experiments/dev-gradient-ascent/adaptive-direction-ablation/artifacts/screening
python3 experiments/dev-gradient-ascent/adaptive-direction-ablation/analyze_screening.py \
  experiments/dev-gradient-ascent/adaptive-direction-ablation/artifacts/screening
```

The producer's default 100-evaluation budget is not the retained command; use
the explicit bounded `--budget 6` above when regenerating this packet.

The separate generic panel uses the same command with `--facet-count 6
--start-count 6 --exclude-start-ids random_F6_s0_1`, source
`experiments/polytope-datasets/random.jsonl`, and output directory
`artifacts/generic-panel`; validate it with
`python3 .../analyze.py artifacts/generic-panel generic`. Its six starts are
the exact screening IDs recorded in `inputs/generic-start-manifest.json`.

The compact ablation inputs are ordinary tracked files. Materialize the shared
random source before reproduction with:

```bash
scripts/artifacts.py materialize polytope-datasets
```

The retained compact trajectory JSONL is tracked normally; new producer output
is written under `artifacts/` and consumed there by the analyzers.
