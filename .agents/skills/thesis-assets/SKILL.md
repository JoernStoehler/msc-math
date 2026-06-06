---
name: thesis-assets
description: Use when Codex creates, edits, reviews, or decides whether to create thesis-facing or experiment-support assets that require design judgment beyond normal prose, including figures, plots, diagrams, sketches, tables, code listings, screenshots, compact fact displays, visual data excerpts, and generated publication assets.
---

# Thesis Assets

Use this skill for reader-facing assets, not for ordinary theorem/prose editing.
Load it together with `thesis-conventions`, `python-conventions`, and
`research-experiments-data` when the asset involves thesis placement, Python
generation, or experiment artifacts.

## Before Creating Or Editing

Do not create an asset merely because source material exists. Create or keep an
asset only when it solves a specific reader problem better than prose alone.

For any nontrivial asset, first establish these five facts. Use `/tmp` scratch
for the draft when the answer is not trivial.

1. Reader problem: what should Kai/Jörn/future students understand faster or
   trust more because this asset exists?
2. Intended use: main thesis, appendix, companion note, experiment report, or
   scratch only.
3. Source truth: which code/data/proof note/companion file owns the facts?
4. Output contract: exact producer command, output path, and intended format.
5. Acceptance checks: how to tell whether the asset is correct, readable, and
   not misleading.

If high-judgment visual/design choices remain unresolved, ask Jörn before
coding. Do not replace that review gate with a large menu or an improvised
asset.

## Source And Status

- Thesis publication assets must be copied deliberately into `thesis/` before
  inclusion in the final PDF; do not `\input` or depend on `formal/`,
  `experiments/`, or `crates/` from thesis LaTeX.
- Experiment figures/data/reports live next to their producer unless they are
  deliberately promoted into `thesis/`.
- Generated `.jsonl`, `.csv`, figures, and tables must be regenerated, not
  patch-edited.
- A companion `.md` may mention a draft asset only after recording its status:
  draft/candidate/rejected/thesis-ready, and whether it is source truth,
  theorem evidence, sanity check, or explanation only.
- Do not describe empirical or explanatory assets as proof input unless the
  thesis proof actually uses them.

## Figures, Plots, Diagrams, Sketches

Use the repo figure conventions as hard constraints, not background advice.

- Python owns figure formatting: fonts, sizes, colors, labels, and layout.
- Run Python generators with `uv run --script` and PEP 723 dependencies. Do not
  use bare `python3` for scripts with undeclared packages.
- Use `experiments/figure_config.py` when relevant. Prefer `FIGSIZE_SINGLE`,
  `FIGSIZE_DUAL`, `FIGSIZE_TRIPLE`, `FIGSIZE_WIDE`, or `FIGSIZE_SQUARE` over
  manual figure sizes.
- Do not use font sizes below the configured minimums in `figure_config.py`.
- Keep captions/proof explanations out of the bitmap unless the text is a real
  label in the figure. Captions should state observations before interpretation
  in thesis prose or a companion note.
- A thesis-width figure must be readable at thesis width. Inspect the rendered
  output before calling it usable.
- Prefer one purpose per asset. Avoid combining geometry reminder, certificate
  facts, proof status, code provenance, and labels into one image.
- Labels should identify the mathematical objects the reader needs; omit labels
  that turn the asset into a raw data dump.
- If a mathematical plot uses recovered or computed data, assert the relevant
  invariants in the producer before saving the output.

## Tables And Listings

- Tables should be organized by the reader's mathematical/proof obligation, not
  by raw source-file order, unless source order is itself the point.
- A code listing is an explanatory excerpt, not source truth. Prefer stable
  pointers or quoted line ranges only after the source and wording are stable.
- Do not create raw witness excerpts, compact fact displays, or listings before
  the thesis prose shows a concrete need for them.
- State what the table/listing supports: theorem proof, verifier audit,
  empirical orientation, caveat, or writing aid.

## Review Checklist

Before reporting completion:

1. Rerun the producer command from a clean-enough state.
2. Inspect the rendered output, not only the script.
3. Check for label overlap, illegible text, excessive density, and wrong visual
   emphasis.
4. Check that paths, generation commands, and status notes are consistent.
5. Check that no source-truth or proof-evidence claim is stronger than the
   asset supports.
6. Report the producer command, output paths, status, and review passes.

Do not commit repo-local skill or asset changes without Jörn's explicit
approval.
