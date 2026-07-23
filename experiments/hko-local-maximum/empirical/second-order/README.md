# Second-Order HKO Checks

Question: along the numerically first-order-flat directions found in the
fixed-`F=10` dual-vertex model at HKO2024, does sampled symmetric finite-step
behavior show positive or negative curvature?

`main.rs` starts from the hardcoded HKO polytope and writes:

- `second-order-base.jsonl`: the active-gradient matrix, numerical rank, and
  flat-direction basis;
- `second-order-curves.jsonl`: paired epsilon probes along the basis
  directions;
- `second-order-random.jsonl`: sampled unit directions in the numerical flat
  space.

The retained base row reports numerical rank 25 and 15 flat directions. The
retained analysis reports negative median symmetric curvature for all 15 basis
directions and negative curvature for 100 sampled flat-space directions.
Individual smallest-scale finite-step deltas include near-zero positive values,
so the result is resolution-dependent numerical support, not negative
definiteness or a proof over every flat direction.

Safe smoke mode runs the phase-one calculation and one curvature probe without
writing tracked outputs:

```bash
cargo run -p exp-hko-local-maximum --release --bin hko-second-order -- --smoke
```

Full mode rewrites all three JSONL files. The analyzer rewrites the tracked
figures and TeX table:

```bash
cargo run -p exp-hko-local-maximum --release --bin hko-second-order
uv run --script analyze.py
```

The older mathematical framing and its caveats are in
`../../../../formal/hko-local-maximality-conditions.tex`; the active
theorem-facing route is instead `../../theorem/`. Changes to active-orbit
selection, SVD tolerance, epsilon grids, derivative conventions, or the HKO
fixture require reassessing this packet.
