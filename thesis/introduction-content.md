# Introduction Companion

Status: section-local maintenance and claim-propagation map for
`thesis/01-introduction.tex`. Not mathematical source truth.

Purpose: preserve the introduction's explanatory hierarchy, source routes,
claim boundaries, and narrow reopen conditions. Active theorem statements,
experiment owners, `FACTSHEET.md`, and accepted Jörn/Kai decisions overrule
this file.

## Explanatory structure

The introduction makes four moves:

1. state the four-dimensional EHZ form of Viterbo's conjecture and explain how
   the HKO counterexample changes the motivating question;
2. name the local, empirical, and finite-method questions and justify the
   restrictions to dimension four, polytopes, low facet counts, and selected
   Lagrangian products;
3. rank the contributions: HKO ten-facet local maximality is the central
   theorem; the finite algorithms and first-order upper functions are enabling
   mathematical contributions; the data-science search is bounded empirical
   evidence; the rotated-pentagon theorem and visualization are side results;
4. give a dependency-based reader path from generalized Reeb dynamics to
   finite methods, local variation, exact certificates, empirical search, and
   the final evidence/research-process discussions.

## Claim routes and boundaries

- Problem and normalization:
  `thesis/02-preliminaries-ehz-capacity.tex`, `Viterbo2000`, and
  `HaimKislevOstrover2024`. The introduction states only the EHZ version in
  four dimensions used by the thesis. The HKO paper's raw $-\pi/2$
  representative uses the opposite symplectic-form convention; direct
  convention translation gives $+\pi/2$, and a further symplectic reflection
  identifies the displayed $-\pi/2$ representative. The introduction calls it
  symplectically equivalent rather than the direct translation.
- HKO theorem and certificate:
  `thesis/07-hko-local-maximum*.tex` and
  `experiments/hko-local-maximum/theorem/`. The theorem is local in the
  exactly-ten-facet Hausdorff stratum modulo translations, scaling, and linear
  symplectic maps. It is not a changing-facet or all-convex-body theorem.
- Finite foundations and algorithms:
  `thesis/03-generalized-reeb-orbits*.tex`,
  `thesis/04-haim-kislev-quadratic-program.tex`, and
  `thesis/05-flow-graph-algorithm-ch2021*.tex`. HK's finite formula is cited;
  the exact flow-graph theorem is restricted to flow-graph regular
  presentations, with chamber-relative genericity.
- First-order method:
  `thesis/06-first-order-perturbations.tex`. Nondegenerate optimizing branches
  and arbitrary smooth feasible upper functions are distinct. No complete
  non-generic branch catalogue or numerical endpoint claim is propagated.
- Bounded search:
  `thesis/08-black-box-datascience.tex`,
  `thesis/a-datascience-results.tex`, and
  `experiments/sys-datascience/`. Counts and outcomes refer only to the
  retained table and frozen generated-candidate packet. The introduction does
  not claim exhaustive search, random nonexistence, or a validated general
  proposer.
- Rotated pentagons:
  `thesis/09-rotated-regular-polygons*.tex` and
  `experiments/regular-products/pentagon-rotation-formula-proof/`. This is an
  exact one-parameter side result, not a classification of polygon products or
  all minimizing orbits.
- Evidence and final surfaces:
  `thesis/10-visualization-3d.tex`, `thesis/11-numerics.tex`,
  `thesis/12-published-code-data.tex`, and `thesis/13-use-of-ai.tex`.
  Visualization remains qualitative; exact and floating-point evidence remain
  distinct; the later AI-process discussion is provisional and is not
  promoted to a central mathematical contribution.

## Narrow reopen conditions

- Recheck the data-science paragraph if the retained thesis chapter changes
  its row counts, evaluated generated-candidate packet, or validated-proposer
  conclusion. This is the anticipated moving update point.
- Recheck theorem-strength wording if integrated review changes the HKO
  theorem, the exact flow-graph regularity boundary, the first-order
  upper-function implication, or the rotated-pentagon exhaustive reduction.
- Recheck the final reader-path paragraph if the numbered AI-process section
  is substantially rewritten, removed, or given a different thesis role.
- Recheck citations if the introduction expands its historical account beyond
  the Viterbo and HKO papers currently cited.
