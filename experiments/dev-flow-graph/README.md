# Dev Flow-Graph

This package is the active algorithm-development surface for the flow-graph
algorithm. It contains frontier counts, endpoint and closed-word representation
spikes, case-finding tools, unresolved-word diagnostics, and tube visualization
for understanding the current algorithm.

The current binaries intentionally mix counters, f64 failures, exact
cross-checks, and HK2017 comparisons when those fields help diagnose the
changing flow-graph algorithm. Do not treat that mixture as a routing precedent
for durable evidence packets.

The boundary is not "mentions numerics" versus "does not mention numerics".
Keep numerical analysis here when it should move with changing flow-graph
algorithm design, representation choices, supported cases, or failure
diagnostics. Move or copy it into `experiments/numerics/` when the important
thing is reusable f64/exact methodology that should improve together across
algorithms.

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
cargo run -p exp-dev-flow-graph --release --bin flow-graph-frontier -- --max-facets 5 --output /tmp/flow-graph-frontier-smoke.jsonl
cargo run -p exp-dev-flow-graph --release --bin flow-graph-endpoint-spike -- --max-facets 5 --max-rows 1 --output /tmp/flow-graph-endpoint-spike-smoke.jsonl
cargo run -p exp-dev-flow-graph --release --bin flow-graph-visualize-tube-data -- --output /tmp/flow-graph-attempt31-visualization.json
uv run --script experiments/dev-flow-graph/visualize-tube/render.py --input /tmp/flow-graph-attempt31-visualization.json --output /tmp/flow-graph-attempt31-visualization.png
```

## Artifact Policy

- Current commands default to stdout unless `--output` is provided.
- Do not treat scratch JSONL as thesis evidence until the producing command,
  input path, commit, and interpretation are recorded.
- The default input dataset is currently owned by
  `experiments/combinatorial-cells/polytopes.jsonl`; this package only reads it.

## Current Experiment Families

- `frontier/`: word-frontier and f64 tube count measurements.
- `endpoint-spike/`: exact endpoint-set representation spike.
- `closed-word-spike/`: exact closed-word resolver spike for selected generated
  closed words.
- `discover-e2e/`: case-finding tool for high-value flow-graph examples before
  selected rows are promoted into fixed checks.
- `unresolved-diagnostic/`: diagnostic for unresolved f64 closed words using
  exact tube, exact one-sigma QP, geometric recovery, and HK2017-style
  references as debugging evidence.
- `visualize-tube/`: Rust JSON producer plus Python matplotlib renderer for
  tube-debugging and future thesis figures.

## Promotion Targets

- Move or copy cleaned f64/exact numerical behavior audits to
  `experiments/numerics/` once the question is reusable numerical methodology
  rather than flow-graph design triage.
- Move or copy runtime, memory, counter, and profiling targets to
  `experiments/performance/` once the measured algorithm path is stable enough
  that counters or timings are the result.
- Move or copy correctness, HK2017 agreement, literature-value, or error-path
  regression packets to `experiments/verification/` once selected cases are
  intended as evidence rather than case-finding diagnostics.
