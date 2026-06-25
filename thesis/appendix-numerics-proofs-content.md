# Numerics Appendix Content Notes

Section-local content companion for `thesis/b-numerics-proofs.tex`.
Source truth is `formal/hk2017-qp-core.tex`,
`formal/hk2017-qp-precision.tex`, and `experiments/dev-quadratic-program/numerics-audit/README.md`.

## Content Inventory

- Exact algebraic fallback details.
- Empirical error-measurement details.
- Proven error bounds.
- Intermediate inequalities and constants.

## Thesis-Useful Content

- Use the appendix only for details that the main paragraph cannot responsibly
  compress: the exact/f64/indeterminate contract, the generic-case hypotheses,
  and the reason diagnostics do not amount to a public certified-solver claim.
- Separate the possible numerical workflows:
  Rust exact audit or certificate, SageMath exact verification, pure f64 with
  rejection, trinary propagation, lazy exact fallback, and conditional
  error-bound proof. They answer different needs and should not be presented as
  one universal solver architecture.
- If empirical numbers are cited, prefer a compact table derived from the
  current audit report: context count, oracle kind, predicate-disagreement
  count, row-level largest absolute errors, and no-oracle conditioning
  diagnostics. Use the report sections `Emitted Context Bank`, `Oracle-Backed
  f64 Measurements`, `Predicate Agreement Diagnostics`, and `Conditioning And
  Solver Diagnostics Without Oracle`. State that the table is an emitted-context
  audit, not a coverage theorem.
- Do not describe the numerics audit oracle as runtime fallback. Runtime exact
  resolution belongs only to code paths that implement it; flow-graph claims
  should be sourced from
  `crates/symplectic/src/algorithms/flow_graph/README.md`.
- If a visual asset is needed, prefer a claim-support matrix or provenance
  diagram over plots. A plot is justified only if a future audit has enough
  contexts to reveal a distributional pattern.
- Keep old gradient-validation, unknown-predicate aggregate, Sage feasibility,
  and broad packet-style error-bound material out of the appendix unless a new
  source revalidates the exact claim being used.

## Asset Boundaries

- Any thesis-facing asset must be copied deliberately into `thesis/`; do not
  include files directly from `experiments/`.
- The caption or nearby prose must state whether the asset is theorem evidence,
  empirical support, a diagnostic, or an explanation.
- The asset should make a reader understand claim strength faster than prose
  alone; otherwise use a sentence or table instead.
