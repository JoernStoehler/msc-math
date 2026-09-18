# Whole-manuscript consistency audit

18 September 2026. Inspected candidate commit `476facfd` (87-page integration
checkpoint), with source selection from `thesis/candidate/main.tex` and its
`legacy/` fallback. This is a bounded source/known-feedback audit, not a new
mathematical proof audit, visual inspection or prose acceptance prediction.
No manuscript source was changed and no computational producer was run.

## Corrections ready without another human judgment

Locations refer to the frozen sources; line numbers may move during integration.

1. **Self-contained claim exceeds the actual proof contract.**
   `thesis/candidate/recovered/thesis-candidate/01-introduction.tex:121`
   promises a “self-contained presentation.” The selected
   `02-duality.tex` explicitly imports the nonsmooth multiplier theorem and
   fixed-period attainment from AAO, then explains normalization and reconstruction.
   That use is legitimate; the introduction should promise the actual expanded
   derivation rather than self-containment. A minimal replacement is in the patch.

2. **Two different matrices are called `J_0` across chapters.**
   `recovered/thesis-candidate/02-preliminaries-convex-symplectic-notation.tex`
   defines `J_0 = [[0,-I],[I,0]]` in `(q1,q2,p1,p2)` coordinates.
   `product-rotation-draft/section.tex:56,96–98` uses the same symbol for
   `diag(J2,-J2)`, the theta-zero member of its factor-plane family. The local
   conjugation identities are not the problem; the undocumented change of
   convention is. Name the initial product matrix `J_sym` and state that this
   section uses factor-adapted coordinates. The proposal does this without
   changing any formula or rotation theorem. The integrator may move the
   `J_sym := J_theta|theta=0` definition next to the first matrix display.

3. **Flow chapter promises a destination that has no promised inventory.**
   `05-flow-graph.tex:415` says input data/test entry points are listed in
   Availability. The selected `legacy/12-published-code-data.tex` contains no
   flow test/input entry. Either provide them there or link the real owners.
   The patch takes the latter option: `crates/symplectic/src/algorithms/flow_graph/exact_search.rs`
   contains search and regression tests; `experiments/dev-flow-graph/visualize-tube/`
   contains the worked example and exact retained input. Both resolve in this
   candidate checkout. The upstream closure report is at
   `/workspaces/msc-math/.worktrees/flow-runtime-closure/docs/flow-runtime-closure/README.md`
   but is **not** currently copied into the candidate, so do not add a relative
   candidate link to that report unless adopting it too.

4. **Keep the conclusion's local quantifier explicit.**
   `recovered/thesis-candidate/14-conclusion.tex:7` says “Every sufficiently
   nearby body” after a ten-facet sentence. The antecedent permits the intended
   reading, but the stand-alone universal phrasing is broader than the theorem.
   Replace “body” by “ten-facet polytope.” This is scope hardening, not evidence
   of a false main theorem.

The [minimal patch](minimal-corrections.patch) passes `git apply --check` against
`476facfd`. It is unapplied and unbuilt; the owning integrator should adapt it to
concurrent edits and run its usual candidate build. No tests are needed for the
unapplied report itself.

## Ready literature correction still omitted

The introduction's local-maximality context mentions smooth Zoll work only;
there is no selected discussion of Haim–Kislev's *Dynamical extensions of Zoll
to nonsmooth convex bodies* (arXiv:2511.16644v1). This was already identified
in the preserved Pro examiner report R03 and independently source-checked in
`/workspaces/msc-math/.worktrees/open-thesis-literature/docs/open-thesis-literature/extrema.md`.

A short comparison is ready: the cited paper investigates cutting the HKO
body; Proposition 1.13 proves capacity additivity for specified near-boundary
cuts. Cutting generally changes the facet count, while the present theorem
moves ten facets. Capacity loss under a cut by itself does not compare the
capacity loss with volume loss. Neither result supplies the other's missing
implication. Add this relation and a bibliography entry near the existing
smooth-Zoll paragraph or the local theorem's discussion; no new literature
campaign is needed. Do not promote the paper's numerical cut-extremality
observations to a theorem, or claim that the all-convex local question is solved.
This report reuses the existing source audit; it does not claim an additional
independent reading of the external paper.

## Evidential wording to reconcile

Introduction `01-introduction.tex:132` says the flow method's performance “in
our tests made it less useful for large searches.” The selected flow chapter
instead says no common-input benchmark is claimed. Those statements need not
be logically inconsistent (informal historical experience could support the
first), but the candidate does not identify the supporting comparison. The
existing flow-runtime closure verifies correctness, not relative performance.
Either supply an appropriate retained performance source or remove the
comparative phrase and describe the independent cross-check role. Do not turn
finite correctness tests into a speed comparison. This item is a source gap in
this bounded read, not a finding that no timing evidence exists anywhere.

## Known feedback and disclosure checks

- The human-approved HKO chapter's repairs remain present in its included
  `hko-lemma-*` files: worked-bound purpose comes first, CAS attribution is
  explicit, and the previously criticized certificate-entry/negative-alternative
  wording has not been reintroduced. No wholesale HKO rewrite is recommended.
- The separate disclosure and reflection are both wired into the candidate.
  The disclosure attributes the new affine argument to Pro/agent review and
  retains the dated personal-verification limitation. It does not falsely extend
  Jörn's prior personal verification to the new argument.
- The reflection's rotation-versus-fresh-sampling example agrees with the
  retained orientation-allocation README: fixed rotations improved three of four
  starting bodies; fresh draws won all four maximum comparisons. Its defective
  regression-test example agrees with the sign-replay case matrix. These checks
  support those anecdotes, not all first-person experiential assertions.
- The abstract does not mention the newly selected product-position result.
  This is an optional synthesis choice, not a demonstrated factual error.
- General prose concerns (qualification density, chapter length and relative
  emphasis) remain judgments. This audit has not converted such preferences
  into mandatory deletions, or treated the narrow flaggers as acceptance gates.

## Coordination

Actionable findings were sent directly to the candidate integrator before this
report. Candidate edits, final source convention choice, literature paragraph
and build remain under that owner's control. This worktree retains a coherent
review packet and is not disposable scratch; no temporary producer outputs or
raw session/secret reads were created.
