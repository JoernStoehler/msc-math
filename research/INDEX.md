<!--
Purpose: index and conventions for research interpretation and proof-route notes.
Context: research notes are first-class epistemic artifacts, not task trackers.

Index maintenance:
- Source truth is topic research notes, proof-bearing sources, experiment
  outputs, task bundles, and accepted Jörn/Kai decisions.
- To check staleness, compare affected rows against the referenced research
  notes and task files; do not treat this index as a proof or task-status owner.
- To refresh, update synthesis and routing only; keep full reasoning in topic
  notes and work obligations in `tasks/*.md`.
- Keep entries short; point to source files instead of duplicating details.
-->

# Research Index

## Role

Files in `research/` record expensive interpretation, proof-route state,
decision history, negative results, and topic summaries. They are source
artifacts for thesis planning: `tasks/*.md` should link here instead of
embedding full reasoning, and thesis-facing work obligations should point to
the relevant research note.

Research files do not have to mirror `tasks/*.md` filenames. Topic interactions
in research can differ from task-bundle interactions. Prefer clear names over
filename isomorphism.

This index is retained synthesis, not a purely regenerable directory listing.
It aggregates the thesis-facing stories and points to the notes and task
bundles that own interpretation, value, cost, state, and remaining obligations.

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
  task-bundle obligation and the Jörn review or verification gate.

## Current Index

### Thesis Story Index

This table is the top-level entrypoint for the thesis-facing research stories.
It is not a separate claim register. The research note owns interpretation; the
task bundle owns remaining proof, writeup, verification, and cut/weaken
obligations.

| Thesis story | Interpretation source | Work obligations |
| --- | --- | --- |
| HKO2024 local maximality | `hko-local-maximum.md`, `hko-local-maximum-status.md`, `hko-local-maximum-exact-clarke.md` | `tasks/hko.md`, `tasks/writing.md` |
| Hostile sys-search landscape | `sys-landscape.md`, `sys-landscape-toolbox-audit.md`, `sys-landscape-datascience/` | `tasks/landscape.md`, `tasks/writing.md` |
| Crosspolytope capacity | `crosspolytope.md` | `tasks/writing.md`, `tasks/reproducibility.md` if thesis cites the computation |
| Visualization negative exploration | `visualization.md` | `tasks/landscape.md`, `tasks/writing.md` if included as standalone or supporting material |
| Pentagon rotation formula | `sys-landscape.md`, future dedicated note if promoted | `tasks/landscape.md`, `tasks/writing.md` if promoted from future work |
| Numerical and algorithmic method story | `numerics.md`, `numerics-error-bounds.md`, `verification.md`, `verification-orbit-recovery.md` | `tasks/numerics.md`, `tasks/reproducibility.md`, `tasks/writing.md` |
| Repo/software/process story | `finish-current-state.md`, `verification.md`, `visualization.md`, architecture and submission maps | `tasks/infrastructure.md`, `tasks/reproducibility.md`, `tasks/writing.md`, `tasks/submit-thesis.md` |

### Research Note Index

| Area | Start here | Role |
| --- | --- | --- |
| Current finish state | `finish-current-state.md` | repo-state and closeout context |
| HKO local maximum | `hko-local-maximum.md`, `hko-local-maximum-status.md`, `hko-local-maximum-exact-clarke.md` | HKO proof route, exact-Clarke state, and blockers |
| Hostile sys landscape | `sys-landscape.md`, `sys-landscape-toolbox-audit.md`, `sys-landscape-datascience/` | negative-search interpretation and data-science method state |
| Sys first-order local behavior | `sys-first-order-local-behavior.md` | generic smooth case, non-generic active-germ classification, and semialgebraic fallback status |
| Numerics | `numerics.md`, `numerics-error-bounds.md` | numerical-method status and error-bound interpretation |
| Verification | `verification.md`, `verification-orbit-recovery.md` | validation evidence and orbit-recovery interpretation |
| Standalone topics | `crosspolytope.md`, `visualization.md`, `combinatorial-cells.md` | topic-local result interpretation |

## Handshake With Other Surfaces

- `tasks/*.md`: routes work, records steering decisions, links here for proof
  and interpretation content, and owns remaining obligations caused by desired
  thesis stories.
- `tasks/verify-thesis-done.md`: owns final done gates; update or flag it when
  a research note changes what must be checked before declaring the thesis done.
- `formal/`: owns developer-facing proof text; research notes may explain proof
  route state but should not replace proof-bearing TeX.
