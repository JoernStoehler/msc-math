# Dev Flow-Graph

This package is the active algorithm-development surface for the flow-graph
algorithm. It contains exact-compatible frontier counts, endpoint and
closed-word representation spikes, and tube visualization for understanding
the current algorithm.

The first three binaries below are development probes, not durable evidence
packets. The exact flow-graph implementation and proof-risk verification
surfaces live in the crate and
`experiments/verification/flow-graph-proof-risk/`; the visualization packet is
retained for thesis exposition.

The boundary is not "mentions numerics" versus "does not mention numerics".
Keep exact computational analysis here when it should move with changing
flow-graph algorithm design, representation choices, or supported cases. QP
numerical methodology has its own package.

The algorithm contract and result-control surface live in
`crates/symplectic/src/algorithms/flow_graph/README.md`.

## Packet Inventory

Read the relevant child README before its source or artifacts. This is the
exhaustive current package inventory:

| Path | Current role |
| --- | --- |
| `frontier/` | Exact-compatible word-frontier counts before tube arithmetic. |
| `endpoint-spike/` | Exploratory exact endpoint-set representation and operation counters. |
| `closed-word-spike/` | Exact fixed-set/action classifications for selected generated words. |
| `visualize-tube/` | Exact-rational JSON producer and renderer for active explanatory thesis figures. |

The first three are algorithm-development probes without canonical retained
outputs. `visualize-tube/` is the retained explanatory artifact packet.

## Binary64 Prototype Retirement

The earlier binary64 flow-graph prototype lacked a sound
true/false/indeterminate predicate contract and was retired when project time
ended. The inventory below records what remains current and what was retired;
it is not a replacement evidence plan for the prototype.

The retired `flow-graph-discover-e2e` and
`flow-graph-unresolved-diagnostic` binaries depended on the f64 flow-graph
implementation to define their observations: the first bucketed approximate
capacity agreement and numerical rejection modes, and the second selected
words specifically from f64 unresolved/error outcomes. Neither has a coherent
exact-only replacement under the current public APIs.

| Command or output surface | Classification | Migration result |
| --- | --- | --- |
| `flow-graph-frontier`: transition edges, half-cache sizes, plus-depth counts, closed-cycle counts, split-missing counts, structural zero-ω predicate | Exact | Preserved in `frontier/main.rs`; the structural predicate now reads exact cache matrices directly. |
| `flow-graph-frontier`: binary64 tube live/empty/error counts, candidate actions, polygon inequalities, and operation counters | Retired prototype surface | Removed; an exact per-word tube/counter scan is not a bounded replacement for a combinatorial frontier command. |
| `flow-graph-discover-e2e` | Retired prototype binary | Deleted from Cargo and source; its approximate capacity/rejection question is historical only. |
| `flow-graph-unresolved-diagnostic` | Retired prototype binary | Deleted from Cargo and source; its input population was defined by the retired binary64 resolver. |
| `flow-graph-endpoint-spike` and `flow-graph-closed-word-spike` | Exact | Already exact representation spikes; left in their child directories. |
| `flow-graph-visualize-tube-data` | Active explanatory visualization | The active figure remains reproducible; its producer must use exact tube geometry and convert only at the JSON/rendering boundary. |
| `flow-graph-proof-risk` rows and exact FG search | Exact | Consumes public exact FG/tube APIs. The retained verifier rows are implementation evidence, not proof of the idealized theorem or CH2021's scope. |

## Artifact Policy

- Current Rust producers default to stdout unless `--output` is provided.
- Do not treat scratch JSONL as thesis evidence until the producing command,
  input path, commit, and interpretation are recorded.
- The default input dataset's physical home is
  `experiments/combinatorial-cells/polytopes.jsonl`; this package only reads it.

## Current Analysis Notes

### Exact singular fixed-set boundary

Current question: whether theorem-facing exact flow-graph search needs a
general lemma for nonpositive-action singular fixed sets, or whether the
ordinary generated singularities are a smaller structural class.

Scratch scan command used a temporary Rust probe over the modern exact resolver:
generated polytopes from `MASTER_SEED=20260605`, `H_MIN=0.5`, `H_MAX=2.0`,
transition-pruned exact closed words, bucketed by word length and exact
resolver outcome.

Observed selected cases:

- `F5` attempts `60`, `73`, `77`: every length-three nonempty word resolved as
  `length_three_zero_time`; length-four no-orbit words were regular; length-five
  words contained the positive orbit or empty tubes.
- `F6` attempt `3`: same pattern, plus length-six empty words.
- `F7` attempt `31`: length-three had 8 `length_three_zero_time` and 3 empty
  words; longer words had regular zero no-orbits, positives, or empty tubes.
- first 25 valid generated `F5` attempts scanned through attempt `429`:
  `len=3`: 101 `length_three_zero_time`; `len=4`: 88 regular zero no-orbits;
  `len=5`: 25 positive and 21 empty; zero other singular statuses and zero
  positive singular statuses.

Interpretation:

- Rejecting every singular fixed map is too strong: ordinary generated cases
  routinely contain structural length-three zero-time fixed lines.
- After splitting out the length-three zero-time lemma, the sampled generated
  cases showed no remaining nonpositive singular fixed-set statuses.
- The theorem-facing exact search therefore accepts only the proved
  `length_three_zero_time` singular no-orbit class, rejects positive-action
  singular fixed sets, and rejects any other singular no-orbit status until a
  broader lemma is actually needed.
- Stable regression evidence was promoted to
  `experiments/verification/flow-graph-proof-risk`: rows
  `length_three_word_is_zero_time_no_orbit` and
  `positive_singular_word_is_typed_unsupported`.

## Promotion Ledger

Treat this table as a routing ledger, not as a statement that evidence has
already been promoted. Keep mixed diagnostics here while they explain the
current algorithm state. Promote selected cases, commands, or summaries only
after the question has a stable evidence home.

| Surface | Keep here while | Promote or retire when | Destination |
| --- | --- | --- | --- |
| `frontier/` | word-frontier counts still guide search/representation choices | the measured path is stable enough that counts or runtime are the result | `experiments/performance/` |
| `endpoint-spike/` | exact endpoint-set representation remains useful historical context for the current exact implementation | the crate exact path fully supersedes it and no active diagnostic reads it | git history, or a short note here if future agents still need the clue |
| `closed-word-spike/` | selected generated words are still being used to understand exact closed-word behavior | a selected word becomes a stable regression or proposition witness | crate tests or `experiments/verification/` |
| `visualize-tube/` | tube geometry needs inspection to debug a word or explain a mismatch | a selected image/data packet supports thesis exposition | relevant thesis/topic asset packet |

## Promotion Targets

- Move or copy runtime, memory, counter, and profiling targets to
  `experiments/performance/` once the measured algorithm path is stable enough
  that counters or timings are the result.
- Move or copy correctness, HK2017 agreement, literature-value, or error-path
  regression packets to `experiments/verification/` once selected cases are
  intended as evidence rather than case-finding diagnostics.
