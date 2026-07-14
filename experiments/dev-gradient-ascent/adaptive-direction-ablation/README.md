# Adaptive direction-model ablation

This packet compares three normalized adaptive branch-gradient directions on
the frozen six-start generic-random `F=6` panel: the deterministic current
minimizer branch, zero-gap near-active maximin, and the finite-radius
gap-aware candidate-window maximin.  Every proposal is an exact full-`sys`
evaluation and is retained in raw JSONL with branch-set and prediction
telemetry.  The generated `analysis.json`, `DISCUSSION.md`, and figures are
the claim-bearing outputs; `run-provenance.json` records source and parameter
identity.

The comparison is descriptive (`n=6` paired starts), not a convergence or
stationarity claim.  Initial radii are `1e-4,1e-3,1e-2`; each trajectory has
100 post-initial target evaluations unless a concrete invalid/evaluator/radius
stop occurs.

The canonical six-start generic-random smoke was single-branch at one
proposal per start, so the retained claim-bearing slice is the separately
labelled narrow-gap hard state in `inputs/narrow-gap-hard-state.jsonl` (source
state `43d243…dec8cc`, selected from the existing narrow-gap diagnostic). It
uses six proposals per policy/radius cell to exercise the mechanism within the
bounded packet cost; this is not a replacement for a 100-evaluation census.

## Reproduce

```bash
cargo run --release -p exp-dev-gradient-ascent \
  --bin dev-gradient-ascent-adaptive-direction-ablation -- \
  --polytope-table experiments/sys-datascience/produce/random.jsonl \
  --facet-count 6 --start-count 6 --exclude-start-ids random_F6_s0_1 \
  --out-dir experiments/dev-gradient-ascent/adaptive-direction-ablation/artifacts

uv run --script experiments/dev-gradient-ascent/adaptive-direction-ablation/analyze.py \
  experiments/dev-gradient-ascent/adaptive-direction-ablation/artifacts
```

The retained hard-state packet was produced with `--budget 6`,
`--facet-count 12`, `--start-count 1`, and the three default radii against
`inputs/narrow-gap-hard-state.jsonl`; the canonical six-start smoke used
`--smoke --budget 1` against the random source.
