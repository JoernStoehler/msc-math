---
name: thesis-assets
description: Use when Codex creates, edits, reviews, or decides whether to create thesis-facing or experiment-support non-figure assets requiring design judgment beyond normal prose, including tables, code listings, and compact fact displays. Use thesis-figures instead for figures, plots, diagrams, sketches, screenshots presented as figures, visual data displays, and figure-like composites.
---

# Thesis Assets

Create or retain an asset when it solves a specific reader problem better than
prose. Useful design questions are: who needs it, where it will be used, what
owns its facts, how it is regenerated, and what would make it correct,
readable, and non-misleading. Ask Jörn only when an unresolved choice depends
materially on his thesis taste.

## Ownership And Status

- Experiment assets stay beside their producer unless deliberately copied into
  the self-contained `thesis/` tree.
- Regenerate generated data and tables; do not patch their outputs.
- Record whether a mentioned asset is a draft, candidate, rejected, or
  thesis-ready, and whether it is source truth, theorem evidence, a sanity
  check, or explanation only.
- Do not present empirical or explanatory assets as proof input unless the
  proof actually uses them.

## Tables And Listings

- Organize tables around the reader's mathematical or proof obligation unless
  source order is itself relevant.
- Treat code listings as explanatory excerpts, not source truth. State whether
  a table or listing supports proof, verifier audit, empirical orientation, a
  caveat, or drafting.
- Add witness excerpts, compact fact displays, or listings only when current
  prose establishes a concrete reader need.

Before reporting completion, rerun the producer, inspect the rendered result,
and check paths, commands, status, emphasis, and epistemic claims.
