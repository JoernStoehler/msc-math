<!--
Purpose: index and conventions for research interpretation and proof-route notes.
Context: research notes are first-class epistemic artifacts, not task trackers.

Index maintenance:
- Source truth is topic research notes, proof-bearing sources, experiment
  outputs, task progress files, and accepted Jörn/Kai decisions.
- To check staleness, compare affected rows against the referenced research
  notes and task files; do not treat this index as a proof or task-status owner.
- To refresh, update synthesis and routing only; keep full reasoning in topic
  notes and work obligations in the relevant task progress file.
- Keep entries short; point to source files instead of duplicating details.
-->

# Research Index

## Role

Files in `research/` record expensive interpretation, proof-route state,
decision history, negative results, and topic summaries. They are source
artifacts for thesis planning: task progress files should link here instead of
embedding full reasoning, and thesis-facing work obligations should point to
the relevant research note.

Research files do not have to mirror task-progress filenames. Topic
interactions in research can differ from task-progress interactions. Prefer
clear names over filename isomorphism.

This index is retained synthesis, not a purely regenerable directory listing.
It aggregates the thesis-facing stories and points to the notes and task
progress files that own interpretation, value, cost, state, and remaining
obligations.

## Map Type And Authority

- Type: convention/index map.
- Agent question: which research note owns the interpretation or proof-route
  state for a thesis-facing story?
- Authority: topic research notes, proof-bearing sources, and accepted
  Jörn/Kai decisions overrule this index.
- Non-authority: this file does not own full proof arguments, experiment
  interpretations, task status, or final thesis-done gates.

## Writing Rules

- Start with the research question, result, or interpretation being cached.
- Separate observation, inference, speculation, and Jörn-only judgment.
- Name the evidence read: code paths, datasets, figures, formal labels, papers,
  commits, or commands.
- State current epistemic status in precise prose, e.g. "Epistemic status:
  exact proof sketch not yet reviewed by Jörn" or "Epistemic status: negative
  interpretation of completed exploratory runs".
- Say what would refresh or invalidate the note.
- If a theorem-strength claim depends on the note, link the corresponding
  task-progress obligation and the Jörn review or verification gate.

## Current Index

### Thesis Story Index

This table is the top-level entrypoint for the thesis-facing research stories.
It is not a separate claim register. The listed source owns interpretation; the
task progress files own remaining proof, writeup, verification, and cut/weaken
obligations.

| Thesis story | Interpretation source | Work obligations |
| --- | --- | --- |
| HKO2024 local maximality | `hko-local-maximum.md`, `hko-local-maximum-status.md`, `hko-local-maximum-proof-control-packet.md`, `hko-local-maximum-exact-witness.md`, `hko-local-maximum-proof-route-note.md` | `tasks/definition-of-success.md`, `tasks/current-state.md`, `tasks/planning-notes.md` |
| Hostile sys-search landscape | `experiments/sys-datascience/README.md`, `experiments/sys-datascience/methods/README.md`, `sys-landscape.md`, `sys-landscape-toolbox-audit.md` | local datascience README files first; research/task files only for older context or cross-thesis audit |
| Crosspolytope capacity | `crosspolytope.md` | `tasks/current-state.md`, `tasks/planning-notes.md` if thesis cites the computation |
| Visualization negative exploration | `visualization.md` | `tasks/planning-notes.md` if included as standalone or supporting material |
| Pentagon rotation formula | `experiments/regular-products/README.md`; `thesis/rotated-regular-polygons-content.md`; `experiments/regular-products/pentagon-rotation-formula-proof/` | `thesis/rotated-regular-polygons.tex` and final thesis review gates |
| Numerical and algorithmic method story | `experiments/numerics/README.md`, `formal/hk2017-qp-core.tex`, `formal/hk2017-qp-precision.tex`, `verification.md`, `verification-orbit-recovery.md` | `tasks/definition-of-success.md`, `tasks/current-state.md`, `tasks/planning-notes.md` |
| Tube algorithm import | `tube-algorithm-raw-jorn-2026-05-04.md`, `tube-algorithm.md` | `tasks/planning-notes.md` if promoted into thesis text |
| Repo/software/process story | `finish-current-state.md`, `verification.md`, `visualization.md`, architecture and submission maps | `tasks/definition-of-success.md`, `tasks/current-state.md`, `tasks/planning-notes.md`, `tasks/submit-thesis/README.md` |

### Research Note Index

| Area | Start here | Role |
| --- | --- | --- |
| Current finish state | `finish-current-state.md` | repo-state and closeout context |
| HKO local maximum | `hko-local-maximum.md`, `hko-local-maximum-status.md`, `hko-local-maximum-proof-control-packet.md`, `hko-local-maximum-exact-witness.md`, `hko-local-maximum-proof-route-note.md` | HKO proof route, exact-witness state, proof-control packet, selected-branch checkpoint, and blockers |
| Hostile sys landscape | `experiments/sys-datascience/README.md`, `experiments/sys-datascience/methods/README.md`, `sys-landscape.md`, `sys-landscape-toolbox-audit.md` | local datascience state first; research notes are context |
| Pentagon rotation formula / regular products | `experiments/regular-products/README.md`, `thesis/rotated-regular-polygons-content.md` | regular-product side-result inventory, writing companion, proof packet routing, and source-truth boundaries |
| Sys first-order local behavior | `sys-first-order-local-behavior.md` | generic smooth case, non-generic active-germ classification, and semialgebraic fallback status |
| Tube algorithm | `tube-algorithm-raw-jorn-2026-05-04.md`, `tube-algorithm.md` | raw Jörn source note plus routing/clarification note before rewriting old TeX or Rust surfaces |
| Verification | `verification.md`, `verification-orbit-recovery.md` | validation evidence and orbit-recovery interpretation |
| Standalone topics | `crosspolytope.md`, `visualization.md`, `combinatorial-cells.md` | topic-local result interpretation |

## Handshake With Other Surfaces

- Task progress files route work, record steering decisions, link here for
  proof and interpretation content, and own remaining obligations caused by
  desired thesis stories.
- `tasks/definition-of-success.md`: owns final done gates; update or flag it
  when a research note changes what must be checked before declaring the thesis
  done.
- `formal/`: owns developer-facing proof text; research notes may explain proof
  route state but should not replace proof-bearing TeX.
