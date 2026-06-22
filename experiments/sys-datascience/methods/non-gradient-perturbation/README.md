# non-gradient-perturbation

## Research Question

Does the bounded random-direction perturbation route run on trusted
random/product basepoints, and what happened in the current tiny smoke panel?

## Method

Use the existing `sys-local-behavior-produce` binary with a trusted random-only
polytope table and `--max-top-basepoints 0`, so basepoints are hash-selected
controls rather than chosen by high `sys`. The producer currently emits both
random directions and gradient-derived diagnostic directions. This packet
counts only `random_unit_direction_*` rows as non-gradient perturbation
evidence. Radii are fixed before evaluation.

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
- The producer panel rows are not retained in this packet. The summary records
  the scratch `panel_dir` used for the smoke run; regenerate the panel with the
  command above before rerunning the analyzer.

## Observation

Current smoke run:

- basepoints: `2`;
- total producer sample rows: `18`;
- successful samples: `18`;
- failures: `0`;
- direction labels:
  `single_near_active_gradient` (`4`),
  `negative_single_near_active_gradient` (`4`),
  `near_active_maximin_direction` (`2`),
  `random_unit_direction_0` (`4`),
  `random_unit_direction_1` (`4`);
- non-gradient random-direction rows counted for this packet: `8`;
- random-direction target rows with `sys > 1`: `0`;
- random-direction max target `sys`: `0.3443338973225082`;
- random-direction positive-delta samples: `4`;
- random-direction max observed `sys` increase: `0.00022450716950711547`.

This smoke panel verifies that the producer can evaluate bounded random
directions from hash-selected trusted basepoints and records no positive row in
the admissible random-direction subset. It is intentionally too small to serve
as broad perturbation coverage by itself. The gradient-derived direction rows
are retained in the artifact only as producer diagnostics and are not counted
as non-gradient evidence.

## Validity Guards

- This is not a gradient-ascent, local-maximum, attractor, or basin experiment.
- Rows whose direction label is gradient-derived are not counted for the
  random-only thesis claim.
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
