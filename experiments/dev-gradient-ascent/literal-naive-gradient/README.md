# Literal Naive Branch-Gradient Ascent

This owner preserves the literal baseline

```text
sigma = deterministic currently minimizing admissible branch
da = eta * grad_a sys_sigma(a)
a = a + da
```

The update is unconditional. The producer does not normalize the gradient,
project the state, construct a near-active set, use a maximin or negative
direction, line-search, reject a decrease, or stop for small improvement. It
records full exact `sys` after every update and tracks best-so-far only for
analysis. A trajectory stops early only if the raw updated dual vertices do not
define a valid state or the exact state/branch-gradient computation fails.

## Retained run

The retained packet runs 100 updates from the ordinary six-facet
`3daddfde...104e` start at learning rates
`1e-5,1e-4,1e-3,1e-2,1e-1,1`. Inspect
[`artifacts/summary.json`](artifacts/summary.json) for the compact trajectory
comparison and the JSONL files for every raw update. In this single-start
packet, every learning rate below `1` remained valid for all 100 updates and
found a new best after iteration 20; `eta=1` made the first updated geometry
invalid. The viable trajectories include frequent decreases and branch
switches, so final-state and best-so-far behavior must not be conflated.

[`figures/iteration-vs-sys.png`](figures/iteration-vs-sys.png) plots both raw
full-`sys` and best-so-far trajectories. Regenerate it with
`uv run --script experiments/dev-gradient-ascent/literal-naive-gradient/plot.py`.

This establishes that eight-step prefixes cannot evaluate the literal baseline
on this start. It does not select a general learning rate, establish population
behavior, or support a local-maximality claim.

## Regeneration

From the repository root:

```bash
cargo run --release -p exp-dev-gradient-ascent \
  --bin dev-gradient-ascent-literal-naive-gradient -- \
  --polytope-table experiments/dev-sys-prediction/facet-scale-baseline-error/polytope-panel.jsonl \
  --out-dir experiments/dev-gradient-ascent/literal-naive-gradient/artifacts \
  --etas 1e-5,1e-4,1e-3,1e-2,1e-1,1 \
  --updates 100
```

Expected complete trajectories contain 101 rows: the initial state plus 100
updates. The `eta=1` failure trajectory contains the initial row and its first
failed update. `run-provenance.json` records source, input, implementation, and
command identity.
