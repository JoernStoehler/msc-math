# Numerics Appendix Content Notes

Status: section-local content companion for
`thesis/appendix-numerics-proofs.tex`. Not source truth.

Purpose: gather appendix-level numerics proof and intermediate-bound material.

Overruled by: `crates/`, `experiments/`, formal proof files, exact verification
artifacts, and Jörn/Kai review.

Lifecycle: keep while the appendix is being assembled. After the appendix is
stable, delete this file or reduce it to a short maintenance note.

Update rule: add or change a claim only with a source pointer or an explicit
`needs source` marker.

## Content Inventory

- Active source truth: `formal/hk2017-qp-core.tex`,
  `formal/hk2017-qp-precision.tex`, and `experiments/numerics/README.md`.
  Deleted packet-style numerics artifacts are historical only.
- Exact algebraic fallback details.
- Empirical error-measurement details.
- Proven error bounds.
- Intermediate inequalities and constants.

## Thesis-Useful Content

- Use the appendix only for details that the main paragraph cannot responsibly
  compress: the exact/f64/indeterminate contract, the generic-case hypotheses,
  and the reason diagnostics do not amount to a public certified-solver claim.
- If empirical numbers are cited, prefer a compact table derived from the
  current audit report: context count, oracle kind, predicate-disagreement
  count, and largest absolute errors. State that the table is an emitted-context
  audit, not a coverage theorem.
- If a visual asset is needed, prefer a claim-support matrix or provenance
  diagram over plots. A plot is justified only if a future audit has enough
  contexts to reveal a distributional pattern.
- Keep old gradient-validation, unknown-predicate aggregate, Sage feasibility,
  and broad packet-style error-bound material out of the appendix unless a new
  source revalidates the exact claim being used.

## Asset Acceptance Notes

- Any thesis-facing asset must be copied deliberately into `thesis/`; do not
  include files directly from `experiments/`.
- The caption or nearby prose must state whether the asset is theorem evidence,
  empirical support, a diagnostic, or an explanation.
- The asset should make a reader understand claim strength faster than prose
  alone; otherwise use a sentence or table instead.
