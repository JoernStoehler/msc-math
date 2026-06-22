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
diagnostics. Move or copy it into `experiments/dev-quadratic-program/numerics-audit/` when the important
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

- `flow-graph-closed-word-spike`
  - Runs an exact closed-word resolver spike on one selected generated
    polytope/word.
  - This is an exploratory representation spike, not the supported crate
    implementation.

- `flow-graph-discover-e2e`
  - Searches generated polytopes and buckets them by f64/FG/QP behavior.
  - Use selected reviewed rows as candidates for fixed crate or verification
    checks; do not treat discovery output as thesis evidence by itself.

- `flow-graph-visualize-tube-data`
  - Emits one JSON object for a generated polytope and one closed tube word.
  - Defaults to the current mismatch row:
    `facet_count=7`, `master_seed=20260605`, `attempt=31`, `sigma=0,4,2,6`.
  - `visualize-tube/render.py` renders that JSON to a PNG with matplotlib.
  - Two-face panels use ordered local frames, so `F_i cap F_j` and
    `F_j cap F_i` are separate panels when both are needed.

- `flow-graph-unresolved-diagnostic`
  - Diagnoses unresolved f64 closed words using exact tube resolution, exact
    one-sigma QP summaries, and geometric recovery references.
  - Keep this here while the failure taxonomy is still part of flow-graph
    algorithm development.

## Smoke Checks

Use release mode for timing or count interpretation.

```bash
cargo run -p exp-dev-flow-graph --release --bin flow-graph-frontier -- --max-facets 5 --output /tmp/flow-graph-frontier-smoke.jsonl
cargo run -p exp-dev-flow-graph --release --bin flow-graph-endpoint-spike -- --max-facets 5 --max-rows 1 --output /tmp/flow-graph-endpoint-spike-smoke.jsonl
cargo run -p exp-dev-flow-graph --release --bin flow-graph-discover-e2e -- --facet-counts 5 --max-attempts-per-f 1 --wanted-per-bucket 1
cargo run -p exp-dev-flow-graph --release --bin flow-graph-unresolved-diagnostic -- --facet-count 5 --attempts 1 --output /tmp/flow-graph-unresolved-diagnostic-smoke.jsonl
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

## Promotion Ledger

Treat this table as a routing ledger, not as a statement that evidence has
already been promoted. Keep mixed diagnostics here while they explain the
current algorithm state. Promote selected cases, commands, or summaries only
after the question has a stable evidence home.

| Surface | Keep here while | Promote or retire when | Destination |
| --- | --- | --- | --- |
| `frontier/` | word-frontier, f64 tube counts, and polygon-operation counters still guide search/representation choices | the measured path is stable enough that counts or runtime are the result | `experiments/performance/` |
| `endpoint-spike/` | exact endpoint-set representation remains useful historical context for the current exact implementation | the crate exact path fully supersedes it and no active diagnostic reads it | git history, or a short note here if future agents still need the clue |
| `closed-word-spike/` | selected generated words are still being used to understand exact closed-word behavior | a selected word becomes a stable regression or proposition witness | crate tests or `experiments/verification/` |
| `discover-e2e/` | rows are being searched and bucketed for high-value examples | a row has a reviewed expected label and should stop being rediscovered | crate tests for cheap cases, `experiments/verification/` for slower artifact-backed suites |
| `unresolved-diagnostic/` | f64 errors, exact tube resolution, exact one-sigma QP, geometric recovery, and HK2017 references are jointly needed for failure taxonomy | the question becomes reusable f64/exact methodology or stable error-path evidence | `experiments/dev-quadratic-program/numerics-audit/` or `experiments/verification/` |
| `visualize-tube/` | tube geometry needs inspection to debug a word or explain a mismatch | a selected image/data packet supports thesis exposition | owning thesis/topic asset packet |

## Promotion Targets

- Move or copy cleaned f64/exact numerical behavior audits to
  `experiments/dev-quadratic-program/numerics-audit/` once the question is reusable numerical methodology
  rather than flow-graph design triage.
- Move or copy runtime, memory, counter, and profiling targets to
  `experiments/performance/` once the measured algorithm path is stable enough
  that counters or timings are the result.
- Move or copy correctness, HK2017 agreement, literature-value, or error-path
  regression packets to `experiments/verification/` once selected cases are
  intended as evidence rather than case-finding diagnostics.
