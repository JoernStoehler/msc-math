# Boundary Gradient Discontinuity

Question: how does the `sys` gradient direction change at sampled first
combinatorial boundaries, and how does that change relate to the sampled cell
geometry?

This is a consuming analysis, not a Rust producer. `analyze.py` reads:

- `../boundary-characterization/combinatorial-boundaries-gradient.jsonl`;
- `../boundary-characterization/combinatorial-boundaries-anatomy.jsonl`;
- `../cell-widths/combinatorial-boundaries-profiling.jsonl`.

It rewrites the tracked figures in this directory:

```bash
uv run analyze.py
```

The plots are diagnostic views of the retained sampled transitions. They do
not prove continuity or discontinuity of `sys`, identify a universal boundary
mechanism, or remove the selected-gradient ambiguity at symmetric points.
Changes to either producer's direction keys, schema, epsilon policy, or
gradient convention require rechecking this join.
