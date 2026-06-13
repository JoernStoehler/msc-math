# Dev Quadratic Program

Status: lightweight coordination packet for unresolved QP/HK2017 route,
library-surface, and cleanup questions. This directory currently owns no Rust
package, generated artifacts, or thesis evidence. It does not imply that there
is ongoing development of the QP method itself.

The durable implementation surface is still `crates/symplectic/`. The reusable
evidence homes remain:

- `experiments/numerics/` for f64/exact behavior, tolerance policy, ambiguity
  handling, and agreement with exact or certified values;
- `experiments/performance/` for runtime, memory, counters, pruning wins, and
  scaling;
- `experiments/verification/` for reusable correctness and regression evidence,
  including capacity axioms, agreement tests, minimum-set semantics, and
  error-path checks;
- topic folders for theorem-local or thesis-slice use.

Use this packet when a QP question is still about route naming, reusable API
shape, or cleanup ownership, not when the question already has a clear evidence
home. If a future branch develops a f64-only route without exact arithmetic,
this README can point to that branch or record the decision after it lands.

Use `experiments/algorithm-comparison/README.md` for cross-algorithm comparison
reasoning that points to performance, numerics, correctness, topic, or thesis
evidence homes.

## Algorithm Labels

The relevant labels are defined in `experiments/MAP.md`:

- `QP/enumerate/unpruned`
- `QP/enumerate/pruned`
- `QP/enumerate/billiard`
- `QP/solve/kkt/f64`
- `QP/solve/kkt/exact`
- `QP/capacity/f64`
- `QP/capacity/fallback`
- `QP/capacity/certified`
- `QP/capacity/exact`
- `QP/recover-orbit`

`QP/capacity/exact` is reserved for a full exact/CAS-backed capacity search.
It is not the ordinary crate path. Current exact crate support includes
one-sigma exact KKT solving and f64-candidate fast paths with exact fallback or
certified exact aggregation.

## Questions For This Packet

- Which QP capacity route or routes should be exposed as the ordinary reusable
  library API?
- Which expert controls should remain public for experiments, and which deep
  module paths are accidental imports?
- What names should distinguish f64-only, exact-fallback, certified
  postprocessing, one-sigma exact solve, and reserved full-exact/CAS routes?
- What result semantics should the library promise for minimizers, gap-window
  orbit sets, rejected ambiguities, and exact fallback counts?
- Which QP cleanup questions should stay coupled until the answer is known, and
  which should move immediately to numerics, performance, or correctness homes?

## Not Owned Here

- Agreement of QP variants with certified values belongs in `experiments/numerics/`
  when the question is numerical behavior, and in `experiments/verification/`
  when the question is reusable correctness evidence.
- Capacity axioms and stable regression suites belong in
  `experiments/verification/` or cheap crate tests.
- Runtime and memory comparisons belong in `experiments/performance/`.
- HKO-specific QP use belongs in `experiments/hko-local-maximum/` unless a
  generic QP cleanup decision is being extracted.
- Method-local datascience use belongs under
  `experiments/sys-datascience/methods/<packet>/README.md`.

## Promotion Rule

Keep this directory README-only unless future coupled QP development work does
not yet have a better home. If code is added here, record why the work should
move together here rather than in `crates/`, `numerics/`, `performance/`,
`verification/`, or a topic folder.
