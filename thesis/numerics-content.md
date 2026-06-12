# Numerics Content Notes

Section-local content companion for `thesis/numerics.tex`.
Source truth for empirical numerics is `experiments/numerics/README.md`.
Source truth for the generic-case numerical contract is
`formal/hk2017-qp-core.tex` and `formal/hk2017-qp-precision.tex`.

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

## Numerical Mechanisms And Thesis Use

| Mechanism | Why it appears | What it gives | Cost or alternative | Best context |
| --- | --- | --- | --- | --- |
| Exact rational or algebraic computation | Theorem-level finite checks cannot depend on f64 signs near discontinuities. | Proof-bearing finite certificates and exact predicate values. | Slow and specialized; not a high-throughput search engine. | HKO theorem packets, regular-product certificates, small exact audits. |
| SageMath exact verification | Some finite certificates are easier to express and audit in a CAS than in Rust, especially with algebraic fields and root isolation. | Independent exact reconstruction, algebraic comparisons, and theorem-facing verification artifacts. | Slower and less integrated into high-throughput Rust workflows; best used for selected proof packets. | HKO feasible-section verification, regular-product executable proofs, final exact checks. |
| Pure f64 computation with rejection | Large data-science and method-development runs need many well-behaved rows more than they need answers on every input. | Fast datasets whose accepted rows satisfy numerical preconditions. | Rejects uncertain polytopes; unsuitable when the interesting objects are degenerate or near predicate boundaries. | Broad search, regression, and method-table experiments on mostly generic inputs. |
| Trinary predicate logic | Positivity, rank, and sign predicates are discontinuous, so near-zero f64 values should not become mathematical facts. | Decided values can be used safely; `indeterminate` records that the computation did not decide. | Propagating indeterminacy complicates consumers; some workflows need rejection or exact fallback. | Capacity and search code that branches on numerical predicates. |
| Lazy exact fallback | Some consumers need an answer on a delicate input instead of rejecting it. | Resolves selected indeterminate cases without running exact arithmetic everywhere. | Only useful where exact fallback exists for the needed predicate or quantity; not a current broad guarantee. | Small or load-bearing cases, especially when a thesis claim depends on the result. |
| Error bounds | A proof or a certificate needs a quantitative link between f64 residuals and exact quantities. | Conditional statements such as "under these margins, this sign/value is stable." | Requires generic hypotheses and constants; broad non-generic coverage is harder. | Appendix-level generic-case numerical contract. |
| Empirical f64-vs-exact measurement | Readers need to know whether the implemented f64 path behaves as expected on representative emitted contexts. | Error magnitudes, predicate disagreement counts, and conditioning diagnostics. | Empirical support is not a theorem and depends on context selection. | Numerics audit reports and compact thesis support tables. |

The useful thesis message is the combination, not one mechanism alone. Rust
exact arithmetic and SageMath exact verification carry theorem-level finite
checks. Pure f64 with rejection is good for high-throughput, well-behaved data.
Trinary logic and lazy fallback describe what happens when f64 branch decisions
are delicate. Error bounds and empirical measurements explain why the accepted
f64 computations are trusted.

The current numerics audit supports the empirical part on an emitted context
bank. The exact-rational simplex/hypercube contexts currently show no predicate
disagreements. The HKO rows are same-binary64-input diagnostics and currently
expose beta-positivity disagreements; they are not algebraic HKO evidence.

Good main-text asset candidate: a compact table with the rows above and columns
for source truth, thesis use, and caveat. Good explanatory figure candidate: a
small flow diagram showing f64 computation, trinary decision/rejection,
optional exact fallback, and exact-oracle audit. Poor asset candidates: raw
JSONL/CSV screenshots, histograms from four contexts, or a standalone HKO
disagreement plot.

## Claim Boundaries

- Supported by current source truth: emitted-context f64-vs-oracle diagnostics,
  exact-rational agreement for the retained rational fixtures, same-binary64
  HKO diagnostics, and the generic-case proof framework in the formal notes.
- Not supported by current source truth: broad public-solver certification,
  algebraic HKO validation from the numerics audit, old gradient-validation
  aggregates, old unknown-predicate aggregate evidence, old Sage feasibility
  packets, and broad packet-style error-bound claims.
