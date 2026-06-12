# Numerics Content Notes

Status: section-local content companion for `thesis/numerics.tex`. Not source
truth.

Purpose: gather the high-level numerical reliability story and its boundaries.

Overruled by: `crates/`, `experiments/`, exact verification artifacts, formal
proofs, task files, and Jörn/Kai review.

Lifecycle: keep while the numerics section is being assembled. After the
section is stable, delete this file or reduce it to a short maintenance note.

Update rule: add or change a claim only with a source pointer or an explicit
`needs source` marker.

## Pacing

- Kai preference: numerics is interesting for about one high-level paragraph in
  the main text, not more.
- Detailed proofs and intermediate bounds belong in the appendix.
- Do not mix numerical-analysis language into symplectic definitions or exact
  algorithm proofs.
- Treat numerics as a support layer after the exact mathematical computation
  story.
- The main reader problem is separating theorem-level exact computation from
  empirical f64 diagnostics. Do not spend the paragraph on implementation
  mechanics unless they change claim strength.

## Content Inventory

- Active source truth: `experiments/numerics/README.md` for the empirical audit
  and `formal/hk2017-qp-*.tex` for the generic-case numerical contract. Old
  packet-style numerics artifacts and deleted research notes are historical;
  revalidate any old claim before thesis use.
- Exact arithmetic path: rational or algebraic data is used to implement
  mathematically meaningful helper operations slowly but without numerical
  error.
- These helpers are separated so later computations can reuse them instead of
  re-encoding the same mathematics.
- Floating-point fast path: the same mathematical algorithms are mapped to
  `f64` linear algebra where practical.
- Discontinuous predicates are treated as trinary `true`, `false`, or
  `indeterminate` decisions with error margins.
- `indeterminate` means the numerical evidence is not strong enough to decide
  the mathematical predicate; this differs from invalid input errors and
  unrecovered assertion failures.
- Logical use of indeterminate values: use cancellations such as
  `false and indeterminate = false`, and simplify searches only from decided
  values.
- Do not claim a relational abstract interpreter unless a retained proof adds
  it. Relations such as two individually indeterminate predicates whose
  disjunction is forced true are outside the current method.
- Include empirical error measurements and exact comparisons where the f64 path
  is used to rerun experiments.
- Include proven error bounds only at the strength needed by retained thesis
  claims.

## Thesis-Useful Content

- Main message: exact/Sage/rational computations carry theorem-level finite
  checks where the thesis needs them; `f64` computations are fast diagnostics
  and experiment drivers, with explicit indeterminate/caveat boundaries.
- Current numerics audit support: fixed context bank, structured JSONL events,
  f64-vs-oracle observations, and predicate-disagreement summaries. The
  exact-rational simplex/hypercube contexts currently show no predicate
  disagreements. The HKO rows are same-binary64-input diagnostics and currently
  expose beta-positivity disagreements; they are not algebraic HKO evidence.
- Good main-text asset candidate: a compact claim-support table with rows such
  as exact finite certificates, f64 solver diagnostics, predicate-disagreement
  audit, and generic-case bounds; columns should be source truth, support
  strength, thesis use, and caveat.
- Good explanatory figure candidate: a small provenance diagram showing
  `P_exact`, `P_f64`, f64 solver output, exact oracle comparison, and report.
  This can replace several sentences explaining `input_pair_kind` and
  `oracle_kind`.
- Poor asset candidates: raw JSONL/CSV screenshots, histograms from four
  contexts, or a standalone HKO disagreement plot. They would overemphasize a
  small diagnostic bank and distract from claim strength.

## Claim Audit Before Writing

- For every retained sentence, record: thesis sentence, source truth, support
  strength, and caveat.
- Do not claim broad public-solver certification.
- Do not claim algebraic HKO validation from the numerics audit.
- Do not reuse old gradient-validation, unknown-predicate aggregate, Sage
  feasibility, or packet-style error-bound claims without a fresh source.
