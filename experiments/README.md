# Experiments

An experiment packet owns one empirical question together with the material
needed to reproduce and interpret it: producer code, input and comparison
contracts, retained outputs, analysis, limitations, and current use.

Read `ARCHITECTURE.md` for the repository-wide packet boundary. Agents changing
packet structure use
`.agents/skills/empirical-research/references/experiment-packets.md`.

## Candidate physical layout

Packets are direct children of `experiments/` by default:

```text
experiments/
|-- README.md
|-- qp-intermediate-numerics/
|   |-- README.md
|   |-- Cargo.toml
|   |-- src/
|   |-- retained outputs
|   `-- analysis/report
|-- optimizer-intermediate-numerics/
|-- volume-intermediate-numerics/
|-- flow-graph-proof-risk/
|-- hko-local-maximum/
`-- ...
```

This is packet-level flattening, not file-level flattening. Each packet keeps
whatever internal directories make its producers and evidence clear.

The current repository still contains category owners such as `numerics/`,
`verification/`, and `performance/`, plus `dev-*` aggregations. This disposable
prototype does not move their executable paths. Treat them as transitional
exceptions while evaluating the packet model, not as the target taxonomy.

## Packet entry point

Near the beginning, a packet README should make recoverable:

1. status;
2. original question;
3. current decision or consumer;
4. what the packet owns and does not establish;
5. systems under study and comparisons actually supported;
6. authoritative producer, evidence, interpretation, and rerun commands;
7. relations that make another change likely to require reassessment;
8. commands that are cheap checks versus commands that refresh evidence.

The original question remains visible when the current consumer, mainline
algorithm, or comparison set changes. Retained evidence identifies the actual
algorithm and configuration it measured rather than relying on a mutable
`mainline` label.

## Relations are not parents

A packet can simultaneously be:

- about a QP implementation;
- an exact-versus-f64 numerical analysis;
- a comparison of current and candidate algorithms;
- evidence for a thesis claim;
- methodologically related to optimization and volume experiments;
- inactive but retained for future regression.

Those are independent links. Record exact paths and stable terms in the packet
README so `rg` can find them. Do not encode one relation as the physical parent
and make the others implicit.

Useful views in this README may group packets by measured system, method,
thesis decision, or status. A view states its scope and points to packet
READMEs; it is navigation, not evidence and not a completeness certificate.

## Current packet views

These are examples of overlapping views over the existing tree, not exclusive
ownership categories.

### By thesis or mathematical object

| Packet | Current role |
| --- | --- |
| `hko-local-maximum/` | HKO theorem certificate tooling and supporting empirical checks |
| `regular-products/` | rotated regular-product sweeps, pentagon figures, and exact formula evidence |
| `combinatorial-cells/` | boundary/cell exploration and bounded negative results |
| `crosspolytope/` | crosspolytope computation |
| `local-maxima-check/` | selected-body comparison of local behavior |
| `sys-datascience/` | retained hostile-`sys` producers, tables, and thesis search evidence |
| `sys-landscape/` | legacy hostile-landscape producers and search surfaces |

### By implementation under study

| Current path | Current role |
| --- | --- |
| `dev-quadratic-program/` | QP route design plus coupled diagnostics |
| `dev-flow-graph/` | flow-graph design and diagnostic studies |
| `dev-gradient-ascent/` | gradient-ascent development studies |
| `dev-sys-prediction/` | prediction experiments over `sys` |
| `dev-canonization-t-search/` | canonicalization-parameter search |
| `dev-f64-capacity/` | f64-capacity route development and diagnostics |

### By empirical method

| Current path | Current role |
| --- | --- |
| `numerics/qp-error-bounds/` | wide QP intermediate-variable numerical evidence |
| `dev-quadratic-program/numerics-audit/` | QP/KKT exact-versus-f64 audit coupled to route design |
| `verification/flow-graph-proof-risk/` | exact flow-graph public-output falsifiers |
| `verification/` | current aggregate package for correctness, minimum-set, recovery, and Sage checks |
| `performance/` | shared current performance package and measurements |
| `visualization/` | visualization producers and browser-rendered assets |
| `algorithm-comparison/` | relation/routing note; no producer or evidence of its own |
| `ai-use/` | AI-use provenance reports and rerun tooling |

## Artifacts and commands

Generated outputs are not hand-edited. Packet commands distinguish:

- cheap compile or smoke checks;
- full producers writing disposable output;
- commands intentionally refreshing tracked evidence.

Generated build trees, temporary outputs, and large raw data are not navigation
surfaces.

Absence from a view or search result does not establish that no experiment
exists. Search packet READMEs, producer names, exact algorithm paths, and stable
mathematical terms before making a project-wide negative claim.
