# Shared Context For Research-Note Migration

This repo has already moved to a new local-note convention for topic and experiment folders.

Use these file meanings exactly:

- `REASONING.md`
  - Current reasoning about the visible surface.
  - Why the current code, data, or experiment state is interpreted this way.
  - What the current artifacts imply upward to the parent topic.

- `DECISIONS.md`
  - Retained non-obvious decisions.
  - Rejected routes, constraints, Jörn instructions, and choices that are no longer obvious from the visible files.
  - Not a chronology, not a progress log, not a task tracker.

- `NEXT-STEPS.md`
  - The current forward-looking work packet.
  - Active objective, blockers, stop condition, and exact next commands or files when known.
  - Not broad history, not general interpretation.

Do not recreate the old `research/` tree in a new location.

The repo direction already agreed in the main session:

- `crates/` owns durable Rust implementations.
- `formal/` owns developer-facing math.
- `experiments/` owns exploratory and validation work.
- `experiments/verification/sage/` is the conceptual home for durable Sage validation.
- Rust files should stay small and concern-focused.
- Tests should move to `test_*.rs` files; bigger validation belongs in experiments.
- Data stays with the producer.
- Experiment trees use semantic paths, not balanced-tree symmetry.

Local-note writing rules:

- Prefer short, information-dense bullets or paragraphs.
- Keep only information that would save a future agent real time.
- Omit obvious code paraphrase.
- Omit low-value chronology.
- Omit stale plans that are not current next steps and are cheap to recover.
- Preserve exact filenames, commands, dataset names, and external constraints when they still matter.

When in doubt:

- `REASONING.md`: "What does the current visible surface mean?"
- `DECISIONS.md`: "What invisible choice or rejected route is still worth remembering?"
- `NEXT-STEPS.md`: "What should the next agent do now?"
