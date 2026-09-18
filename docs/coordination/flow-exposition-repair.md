# Chapter 5 exposition repair — 18 September 2026

Scope: active `thesis/candidate/05-flow-graph.tex`; no theorem statement or
hypothesis changed. All Chapter 5 findings in `whole-exposition-assessment.md`
were checked against the primitive formula and the retained sources before edits.

The genericity proof now defines its independent row parameter space and
nonzero-denominator domain; states and proves the repeated-section contraction
with its incoming/outgoing planes; explains cyclic-cut invariance; gives the
complete nonzero pairing table; and distinguishes repeated algebraic witness
rows from simple searched words. The odd/even reductions and chartwise density
argument are explicit. Source: `formal/flow-graph-real-algorithm.tex`, especially
the repeated-section and long-word-determinant lemmas. No unresolved mathematical
concern was found in this repair.

Local edits explain the two appended closure labels, omission of zero-time
passages from the active word, and the two pruning arguments. Example 5.2 now
names `experiments/dev-flow-graph/visualize-tube/flow-graph-f6-tube.json`, whose
metadata confirms six facets, seed 20260605, attempt 3. Rounded illustration
versus exact membership/closure decisions remains explicit.

Verification: ran the existing exact SymPy checker
`docs/review-evidence/pro-review-2026-09-15/thesis_review/verification/check_flow.py`
using `uv run --with sympy python`; it returns `-10643/600` for ABCDE and
ABABCDE, and `1/168` for ACDE, ABACDE, and ABABACDE. An additional in-memory
check verified all ten displayed pairings, odd witness lengths 5–13, even
witness lengths 6–14, and the corresponding odd cyclic-cut invariance. These
finite checks corroborate the displayed algebra; the contraction proof supplies
the arbitrary-length implication. `git diff --check` passed. Full compilation
and rendered-page verification are left to the integration owner, as assigned.
No temporary files or worktrees were created.
