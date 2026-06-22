# non-gradient-perturbation

## Research Question

Does the bounded non-gradient perturbation route run on trusted random/product
basepoints, and what happened in the current tiny smoke panel?

## Method

Use the existing `sys-local-behavior-produce` binary with a trusted random-only
polytope table and `--max-top-basepoints 0`, so basepoints are hash-selected
controls rather than chosen by high `sys`. Directions are random. Radii are
fixed before evaluation.

## Inputs

- `../trusted-random-dataset/artifacts/trusted-polytope-table.jsonl`

## Commands

Prepare the trusted table:

```bash
uv run --script experiments/sys-datascience/methods/trusted-random-dataset/analyze.py \
  --out-dir /tmp/sys-random-only-dataset \
  --write-filtered
```

Smoke panel:

```bash
cargo run -p exp-sys-landscape --release --bin sys-local-behavior-produce -- \
  --polytope-table /tmp/sys-random-only-dataset/trusted-polytope-table.jsonl \
  --out-dir /tmp/sys-random-only-perturbation-smoke \
  --max-top-basepoints 0 \
  --max-hash-basepoints 2 \
  --random-directions 2 \
  --radii 1e-5,1e-3

uv run --script experiments/sys-datascience/methods/non-gradient-perturbation/analyze.py \
  /tmp/sys-random-only-perturbation-smoke
```

## Retained Artifacts

- `artifacts/summary.json` from the analysis script.
- Producer panel rows live under the explicit `/tmp` or LICCA output directory
  named in the command that generated them.

## Observation

Current smoke run:

- basepoints: `2`;
- sample rows: `18`;
- successful samples: `18`;
- failures: `0`;
- target rows with `sys > 1`: `0`;
- max target `sys`: `0.3449017308020992`;
- positive-delta samples: `10`;
- max observed `sys` increase: `0.0007923406490981111`.

This smoke panel verifies that the non-gradient perturbation route runs and
records no positive row in this tiny sample. It is intentionally too small to
serve as broad perturbation coverage by itself.

## Validity Guards

- This is not a gradient-ascent, local-maximum, attractor, or basin experiment.
- Hash-selected basepoints avoid top-tail selection, but the panel remains a
  bounded sample.
- The existing producer recomputes capacity for target polytopes; failures are
  part of the panel result.

## Current Disposition

Use only as a smoke check of the non-gradient perturbation route. Do not use
as broad perturbation coverage without a larger panel.

## Remaining Worthwhile Questions

Scale the panel only if the smoke panel records meaningful finite improvements
or a near-threshold target.

## Predicted Stability Under Rerun

High with unchanged input table and command seeds.

## Thesis Use

Supports only the statement that a bounded non-gradient random perturbation
smoke panel was run separately from gradient ascent.

## Reopen Triggers

- perturbation radii or basepoint policy change;
- a target row with `sys > 1` appears;
- thesis wording asks about attractors or optimizer behavior.
