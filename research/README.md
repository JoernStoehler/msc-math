<!--
Purpose: index and conventions for research interpretation and proof-route notes.
Context: research notes are first-class epistemic artifacts, not task trackers.
-->

# Research Notes

## Role

Files in `research/` record expensive interpretation, proof-route state,
decision history, negative results, and topic summaries. They are source
artifacts for thesis planning: `tasks/*.md` should link here instead of
embedding full reasoning, and thesis-facing work obligations should point to
the relevant research note.

Research files do not have to mirror `tasks/*.md` filenames. Topic interactions
in research can differ from task-bundle interactions. Prefer clear names over
filename isomorphism.

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
| Repo/software/process story | `finish-current-state.md`, `verification.md`, `visualization.md`, architecture and submission maps | `tasks/infrastructure.md`, `tasks/reproducibility.md`, `tasks/writing.md`, `tasks/submission.md` |

### Research Note Index

| Area | Start here | Role |
| --- | --- | --- |
| Current finish state | `finish-current-state.md` | repo-state and closeout context |
| HKO local maximum | `hko-local-maximum.md`, `hko-local-maximum-status.md`, `hko-local-maximum-exact-clarke.md` | HKO proof route, exact-Clarke state, and blockers |
| Hostile sys landscape | `sys-landscape.md`, `sys-landscape-toolbox-audit.md`, `sys-landscape-datascience/` | negative-search interpretation and data-science method state |
| Numerics | `numerics.md`, `numerics-error-bounds.md` | numerical-method status and error-bound interpretation |
| Verification | `verification.md`, `verification-orbit-recovery.md` | validation evidence and orbit-recovery interpretation |
| Standalone topics | `crosspolytope.md`, `visualization.md`, `combinatorial-cells.md` | topic-local result interpretation |

## Handshake With Other Surfaces

- `tasks/*.md`: routes work, records steering decisions, links here for proof
  and interpretation content, and owns remaining obligations caused by desired
  thesis stories.
- `FINAL-VERIFICATION.md`: owns final done gates; update or flag it when a
  research note changes what must be checked before archive.
- `formal/`: owns developer-facing proof text; research notes may explain proof
  route state but should not replace proof-bearing TeX.
