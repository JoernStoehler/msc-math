# Flow-Graph Experiments

This package currently contains flow-graph frontier-count measurements and an
exact endpoint-set representation spike. It is the intended home for later
flow-graph profiling, HK2017 comparison, and numerical-stability checks.

The algorithm contract and result-control surface live in
`crates/symplectic/src/algorithms/flow_graph/README.md`.

## Current Commands

- `flow-graph-frontier`
  - Reads `../combinatorial-cells/polytopes.jsonl` by default.
  - Writes JSONL to stdout, or to `--output PATH`.
  - Use `--build-f64-tubes` to include f64 tube construction counters.

- `flow-graph-endpoint-spike`
  - Reads `../combinatorial-cells/polytopes.jsonl` by default.
  - Writes JSONL to stdout, or to `--output PATH`.
  - This is an exact endpoint-set spike, not a supported exact implementation.

- `flow-graph-visualize-tube-data`
  - Emits one JSON object for a generated polytope and one closed tube word.
  - Defaults to the current mismatch row:
    `facet_count=7`, `master_seed=20260605`, `attempt=31`, `sigma=0,4,2,6`.
  - `visualize-tube/render.py` renders that JSON to a PNG with matplotlib.
  - Two-face panels use ordered local frames, so `F_i cap F_j` and
    `F_j cap F_i` are separate panels when both are needed.

## Smoke Checks

Use release mode for timing or count interpretation.

```bash
cargo run -p exp-flow-graph --release --bin flow-graph-frontier -- --max-facets 5 --output /tmp/flow-graph-frontier-smoke.jsonl
cargo run -p exp-flow-graph --release --bin flow-graph-endpoint-spike -- --max-facets 5 --max-rows 1 --output /tmp/flow-graph-endpoint-spike-smoke.jsonl
cargo run -p exp-flow-graph --release --bin flow-graph-visualize-tube-data -- --output /tmp/flow-graph-attempt31-visualization.json
uv run --script experiments/flow-graph/visualize-tube/render.py --input /tmp/flow-graph-attempt31-visualization.json --output /tmp/flow-graph-attempt31-visualization.png
```

## Artifact Policy

- Current commands default to stdout unless `--output` is provided.
- Do not treat scratch JSONL as thesis evidence until the producing command,
  input path, commit, and interpretation are recorded.
- The default input dataset is currently owned by
  `experiments/combinatorial-cells/polytopes.jsonl`; this package only reads it.

## Planned Experiment Families

- `frontier/`: word-frontier and f64 tube count measurements.
- `endpoint-spike/`: exact endpoint-set representation spike.
- `visualize-tube/`: Rust JSON producer plus Python matplotlib renderer for
  tube-debugging and future thesis figures.
- `profiling/`: release-mode performance and operation counts.
- `hk2017-comparison/`: same-polytope comparison against HK2017.
- `numerical-stability/`: f64 rejection and near-degenerate transition checks.
