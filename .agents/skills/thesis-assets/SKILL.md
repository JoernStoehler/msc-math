---
name: thesis-assets
description: Use when Codex creates, edits, reviews, or decides whether to create thesis-facing or experiment-support assets requiring design judgment beyond normal prose, including figures, plots, diagrams, sketches, tables, code listings, screenshots, compact fact displays, visual data excerpts, and generated publication assets.
---

# Thesis Assets

Create or retain an asset when it solves a specific reader problem better than
prose. Useful design questions are: who needs it, where it will be used, what
owns its facts, how it is regenerated, and what would make it correct,
readable, and non-misleading. Ask Jörn only when an unresolved choice depends
materially on his visual or thesis taste.

## Ownership And Status

- Experiment assets stay beside their producer unless deliberately copied into
  the self-contained `thesis/` tree.
- Regenerate generated data, figures, and tables; do not patch their outputs.
- Record whether a mentioned asset is a draft, candidate, rejected, or
  thesis-ready, and whether it is source truth, theorem evidence, a sanity
  check, or explanation only.
- Do not present empirical or explanatory assets as proof input unless the
  proof actually uses them.

## Figures

- Let producer code own fonts, sizes, colors, labels, and layout. Use
  `experiments/figure_config.py` as source truth for shared figure sizes and
  minimum font sizes when relevant.
- Run dependency-declaring Python generators with `uv run --script`.
- Keep captions and proof explanations out of the bitmap unless the text is a
  genuine figure label.
- Check readability at thesis width by inspecting the rendered output.
- Give each asset a clear purpose. Labels should identify the mathematical
  objects needed for that purpose rather than reproduce raw data.
- Assert relevant mathematical invariants in producers that plot recovered or
  computed data.

## Tables And Listings

- Organize tables around the reader's mathematical or proof obligation unless
  source order is itself relevant.
- Treat code listings as explanatory excerpts, not source truth. State whether
  a table or listing supports proof, verifier audit, empirical orientation, a
  caveat, or drafting.
- Add witness excerpts, compact fact displays, or listings only when current
  prose establishes a concrete reader need.

Before reporting completion, rerun the producer, inspect the rendered result,
and check paths, commands, status, visual emphasis, and epistemic claims.
