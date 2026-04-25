<!--
Purpose: index and conventions for research interpretation and proof-route notes.
Context: research notes are first-class epistemic artifacts, not task trackers.
-->

# Research Notes

## Role

Files in `research/` record expensive interpretation, proof-route state,
decision history, negative results, and topic summaries. They are source
artifacts for thesis planning: `tasks/*.md` should link here instead of
embedding full reasoning, and `RESULTS.md` should extract thesis-facing claims
from here when a note changes claim strength.

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
  `RESULTS.md` entry and the Jörn review or verification gate.

## Current Index

| Area | Start here | Role |
| --- | --- | --- |
| Current finish state | `finish-current-state.md` | repo-state and closeout context |
| HKO local maximum | `hko-local-maximum.md`, `hko-local-maximum-status.md`, `hko-local-maximum-exact-clarke.md` | HKO proof route, exact-Clarke state, and blockers |
| Hostile sys landscape | `sys-landscape.md`, `sys-landscape-toolbox-audit.md`, `sys-landscape-datascience/` | negative-search interpretation and data-science method state |
| Numerics | `numerics.md`, `numerics-error-bounds.md` | numerical-method status and error-bound interpretation |
| Verification | `verification.md`, `verification-orbit-recovery.md` | validation evidence and orbit-recovery interpretation |
| Standalone topics | `crosspolytope.md`, `visualization.md`, `combinatorial-cells.md` | topic-local result interpretation |

## Handshake With Other Surfaces

- `tasks/*.md`: routes work and records steering decisions; links here for
  proof and interpretation content.
- `RESULTS.md`: owns the compressed thesis-facing claim and interpretation
  cache; update or flag it when a research note changes what the thesis may say.
- `FINAL-VERIFICATION.md`: owns final done gates; update or flag it when a
  research note changes what must be checked before archive.
- `formal/`: owns developer-facing proof text; research notes may explain proof
  route state but should not replace proof-bearing TeX.
