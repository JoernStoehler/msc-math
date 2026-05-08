---
name: thesis-conventions
description: Use when Codex writes, edits, reviews, or delegates thesis-facing LaTeX/prose work in `thesis/`, especially publication-facing mathematical writing, experiment exposition, figures, or self-contained thesis assets.
---

# Thesis Conventions

## LaTeX in thesis/

- the audience for which we write is Kai, Elizabeth, and the hypothetical master students who build upon this thesis in the future.
- Jörn reviews for correctness, clarity, and presentation style.
- We target a professional, publication-ready, pure mathematics style when we write about symplectic geometry from a pure mathematician's perspective, and a more applied/data-science style when we write about experiments.
- Software engineers are not part of the audience, so we don't focus on code.
- Formatting of figures, including fonts and size and colors, are owned by the Python code. LaTeX simply includes the images/PDFs.
- `thesis/` is self-contained and does not `\input` files from `formal/`, `experiments/`, or `crates/`. We deliberately copy assets into `thesis/` when we need them for publication.
