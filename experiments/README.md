# Experiments

Experiments are organized by the question whose code, data, outputs, and
interpretation should change together. A topic README is the entry point; the
producer and retained artifacts are the evidence.

## Main owners

| Owner | Role |
| --- | --- |
| `sys-datascience/` | retained hostile-`sys` producer tables, method packets, and thesis search evidence |
| `sys-landscape/` | legacy hostile-landscape producers and search surfaces |
| `hko-local-maximum/` | HKO theorem certificate tooling and supporting empirical checks |
| `regular-products/` | rotated regular-product sweeps, pentagon figures, and exact formula packet |
| `combinatorial-cells/` | boundary/cell exploration and negative results |
| `verification/` | reusable correctness, regression, minimum-set, orbit-recovery, and Sage checks |
| `performance/` | reusable runtime, memory, profiling, and scaling measurements |
| `numerics/` | reusable numerical-stability and exact-versus-f64 questions |
| `visualization/` | visualization producers and browser-rendered assets |
| `ai-use/` | AI-use provenance reports and rerun tooling |
| `crosspolytope/` | one-off crosspolytope computation |
| `local-maxima-check/` | selected-body comparison of local behavior |
| `algorithm-comparison/` | routing note for cross-algorithm comparison questions; no producer or evidence of its own |

Active method-development packets use `dev-<method>/` while their diagnostics,
performance, correctness, and design still need to move together:

- `dev-quadratic-program/`
- `dev-flow-graph/`
- `dev-gradient-ascent/`
- `dev-sys-prediction/`
- `dev-canonization-t-search/`

Promotion to `crates/`, `verification/`, `performance/`, or another owner is a
deliberate ownership change, not a documentation cleanup.

## Choose by question

| Question | Usual owner |
| --- | --- |
| Is reusable implementation behavior correct? | crate tests, then `verification/` for expensive or cross-cutting evidence |
| How stable is f64 behavior? | method owner while coupled; otherwise `numerics/` |
| How fast or memory-heavy is it? | `performance/` |
| Does this support one theorem or thesis slice? | the topic owner, such as `hko-local-maximum/` or `regular-products/` |
| Does this method help on the retained hostile-search data? | `sys-datascience/` |
| Is the method itself still being designed? | matching `dev-<method>/` |

## Owner README pattern

A topic entry point should make these facts recoverable near the top:

1. what the directory owns and does not own;
2. current status or disposition;
3. established results and their scope;
4. authoritative source/artifact paths;
5. superseded or misleading alternatives;
6. safe smoke commands versus tracked-output producers.

Do not interpret absence from this top-level README as evidence that no
experiment exists. Search topic READMEs and stable mathematical/algorithm
terms before making a project-wide negative claim.

## Artifacts

Keep inputs, producer code, outputs, and interpretation together when they form
one reproducible packet. Generated outputs are not hand-edited. README commands
must distinguish:

- cheap compile or smoke checks;
- full producers writing disposable output;
- commands intentionally refreshing tracked evidence.

Generated build trees, temporary outputs, and large raw data are not navigation
surfaces.
