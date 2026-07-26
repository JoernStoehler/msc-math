# Quadratic Program Content Notes

Status: maintenance companion for the integrated chapter candidate in
`thesis/04-haim-kislev-quadratic-program.tex`. Not source truth.

Purpose: gather the writing inventory for the Haim--Kislev quadratic-program
section.

Overruled by: source papers, `formal/`, `crates/symplectic/`, revalidated
legacy thesis material, experiment artifacts, and Jörn/Kai review.

Lifecycle: retain through Jörn/Kai review because the product-enumeration gate,
source hierarchy, and implementation boundaries are not recoverable cheaply
from the chapter alone. Reduce it after those gates close.

## Integrated Audit, 2026-07-14

The chapter was audited from Main commit `3dcc1efd` against HK2017 Theorems
1.1 and 1.5, the active Clarke/simple-minimizer foundation, the current formal
QP notes, Rudolf's published Theorem 1, the rotated-pentagon dependency, and
the current Rust QP surfaces. The HK convention translation, factor
`c_EHZ = 1/(2 Q_max)`, and global-maximizer realization are supported.

Accepted chapter boundary:

- a feasible or positive KKT point is not a fixed-word maximum or a global
  capacity maximizer;
- complete exact support/word coverage makes KKT candidates sufficient for the
  scalar maximum, but one KKT solve does not solve a nonconcave fixed-word QP;
- the restricted Lagrangian-product family is existential and exhaustive for
  the capacity value, not a classification of every minimizing orbit;
- the general Rust search is an f64-first candidate path with exact rational
  fallback/certification surfaces; an exact KKT witness is global only when
  the candidate family and resolution policy establish coverage;
- the HKO and rotated-pentagon theorem packets own their exact Sage
  certificates. Numerical discovery and crate tests remain supporting evidence.

The product finite-enumeration proof was reconstructed independently from
Rudolf's short strong billiard, the thesis splitting/merging argument, dual
minimality, and Clarke realization. No contradiction was found. On 2026-07-14
Jörn accepted the conceptual reduction and specifically confirmed that the
splitting/merging step produces a distinct-facet dual minimizer without
increasing the inherited three-block bound. He emphasized that this surgery is
in the dual problem: containment in the factor polygons follows only after
Clarke reconstruction returns the dual minimizer to a primal orbit on the
product boundary. Blocks may disappear under merging, but this only decreases
the paired block counts; primal simplicity then bounds every surviving block
by two adjacent facets, while closure excludes a single block of each type.
Thus the retained cases have two or three blocks of each type and at most
twelve distinct facets. Jörn did not personally line-check the convention
calculation. Kai/expert line review remains appropriate because this is a
project-derived, theorem-critical reduction. Reopen the chapter if that review
changes the block family, convention lift, or use of the simple-minimizer
theorem.

Corrections made during this audit: the fixed-word KKT wording now preserves
support-face/global distinctions; the reversed-lift action sign is explicit;
the merging invariant is block count rather than block length; and stale
contradictory statements in `formal/ehz-kkt-system.tex` and
`formal/hk2017-qp-core.tex` were repaired.

Update rule: add or change a claim only with a source pointer or an explicit
`needs source` marker.

## Production algorithm update, 2026-07-26

The chapter now records two product reductions with deliberately different
contracts.

- The twelve-facet alternating block theorem remains the source for the
  rotated-pentagon certificate's exhaustive branch comparison. It retains
  block and adjacency information and must not be replaced by a scalar-only
  sparsity statement.
- The newer six-facet theorem states that some global product-QP maximizer
  uses at most three facets from each factor, with total factor weights
  \(1/2\). Its proof factors the objective as
  \(Q=t(1-t)B(\alpha,\gamma)\), chooses vertex maximizers in the two planar
  normalized closure polytopes, and uses their support bound of three. This is
  an existence and capacity-value theorem, not a classification of all
  minimizers. Source: `formal/product-qp-six-facet-reduction.tex`.

An independent Sol review reconstructed the six-facet argument and found no
gap. Jörn did not line-check it, but accepted its use as a production risk
after all 10,240 retained random products had a literal winning word of length
at most six. Source:
`experiments/dev-quadratic-program/tools/product_closure_route/RESULTS.md`.

The former implementation-status paragraphs were superseded. The production
general route now has complete transition-pruned candidate coverage, certified
curvature rejection with cyclic-order inheritance, verified inverse-defect
enclosures, exact fallback for every indeterminate decision, and outward
capacity bounds. The production product route uses the six-facet
closure-vertex enumeration, interval filtering, and exact resolution of every
possible winner; it returns the exact capacity of the binary64-rational input.
Sources:

- `crates/symplectic/src/algorithms/capacity_4d/`;
- `formal/hk2017-qp-precision.tex`;
- `experiments/dev-quadratic-program/tools/general_algorithm_ablation/RESULTS.md`;
- `experiments/dev-quadratic-program/tools/product_closure_route/RESULTS.md`.

The active numerics section states the binary64-input interpretation, exact
geometry validation, applicability limits, fixed-word enclosure theorem,
fallback policy, route completeness, empirical comparisons, and remaining
source-coordinate-uncertainty limitation. The Sage theorem packets remain the
authority for proof-facing algebraic claims.

## Session Scope

- In scope: the Haim--Kislev quadratic-program statement and the mathematical
  reason it computes the EHZ capacity/minimum action for polytopes.
- In scope after the 2026-06-25 review: a compact bridge from the HK theorem to
  the project computation layers that later sections use: unpruned word
  enumeration, transition pruning, product enumeration, active-word KKT solves,
  and the distinction between f64 exploration and exact/Sage theorem evidence.
- In scope: notation choices and conversion notes needed to state the standard
  result in the thesis convention.
- Out of scope here: detailed algorithms for solving the quadratic programs.
  Leave KKT solver internals, exact arithmetic implementation, f64 error
  analysis, performance optimization, and implementation tests to later
  algorithm/numerics sections.
- Out of scope: project-original improvements or insights on top of the HK
  finite formulation, except where a small convention is needed to make the
  standard result readable.

## Purpose And Naming

- Present the standard finite quadratic-program formulation for the minimizer
  search. This is Haim--Kislev's work; the thesis work in this section is
  explanatory and not a new algorithmic contribution.
- The reader-facing section title is "The Haim--Kislev Quadratic Program".
  After first attribution, use "the quadratic program" or "QP" when the
  antecedent is clear.
- The active thesis prose cites `HK2017` because the local source and citation
  index use that anchor for the 2017 arXiv source. The single thesis
  bibliography entry with key `HK2017` carries the published GAFA 2019 metadata.
  Do not reintroduce a separate `HK2019` key for the same paper, and do not use
  year-based naming in reader-facing prose merely because the filename still
  contains `hk2019`.

## Definition

- Active draft now defines the finite optimization problem, variables,
  constraints, objective, and normalization/factor convention in dual-vertex
  coordinates.
- Current convention: for a fixed word
  \(\sigma=(\sigma_1,\ldots,\sigma_m)\),
  \(M_\sigma=\{\beta\ge0:\sum\beta_k=1,\sum\beta_ka_{\sigma_k}=0\}\) and
  \(Q_\sigma(\beta)=\sum_{j<k}\beta_j\beta_k
  \omega_0(a_{\sigma_j},a_{\sigma_k})\). Here \(\sigma\) is the active Reeb
  traversal order.
- The active draft explains the comparison with HK2017 by reversing the ordered
  word in the HK display. This should be reviewed carefully because fixed-word
  orientation errors are high-impact for later solver sections.
- State the input contract explicitly: the dual rows have already been checked
  to be extremal and bounded in the sense of the polytope preliminaries.
- The capacity computation may use the computed primal vertex set and
  vertex-facet incidence for feasibility and pruning, but mathematical
  correctness is the Haim--Kislev finite-search statement plus the preceding
  validity checks.
- Record the chosen thesis convention for dual vertices versus
  normals/heights, the action matrix, `Q`, `Qmax`, and the factor relating
  `Qmax` to `c_EHZ`. Existing formal notes use `c_EHZ = 1/(2 Qmax)` for their
  `Q`; legacy prose may use a different-looking but equivalent convention and
  must be converted deliberately.
- Convention source: use `formal/hk2017-qp-conventions.tex` for the
  HK2017-to-project translation layer, especially normals/heights to dual
  vertices, the project `J_0`/`omega_0` convention, active-word QP orientation,
  and the word-reversal comparison to HK2017. Cite HK2017/Haim--Kislev
  separately for the theorem itself.

## Correctness

- The active theorem states HK2017's capacity formula, translated from
  normals/heights to dual vertices, and gives the direct variable substitution.
- The active explanatory paragraph checks normalization against the thesis
  Reeb/action conventions: an actual simple orbit with period \(T\) gives
  \(Q=1/(2T)\) in active traversal order.
- Do not strengthen this into "every feasible QP point is already an actual
  Reeb orbit." Arbitrary finite feasible points are part of HK's dual finite
  formula; base-point recovery and later feasibility/pruning are separate
  computational questions for such non-global candidates. A certified global
  QP maximizer is different: HK's construction makes it a global dual
  minimizer. In the active free-period normalization, assigning dwell times
  \(T\beta_k\), with \(T=1/(2Q_{\max})\), gives
  \(A=I_K=T\) and multiplier \(\nu=1\). Clarke's correspondence therefore
  realizes it by translation alone as a minimum-action simple generalized
  Reeb orbit. The active word is preserved because
  \(-J_0R_i=2a_i\) belongs to the reconstructed polytope subdifferential and
  irredundancy makes \(a_i\) an extreme polar vertex, forcing facet \(F_i\) to
  be active on that piece. Base-point recovery is then an explicit
  reconstruction method rather than an extra existence hypothesis.
- For the Lagrangian-product enumeration, state only a capacity-search result:
  the restricted alternating \(q/p\)-block family contains a capacity
  minimizer. Do not claim it classifies all minimum-action simple Reeb orbits;
  the billiard bounce bound gives existence of one minimizer with at most three
  bounces, not nonexistence of longer minimizers.
- The former proof incorrectly combined the existence of a simple capacity
  minimizer with the existence of a capacity-minimizing billiard having at
  most three bounces; those statements need not initially select the same
  minimizer. The repaired proof starts with Rudolf's short strong billiard,
  reverses its polygonal lift to the thesis symplectic convention, and applies
  the already proved dual splitting and merging operations. Merging repeated
  facet directions cannot increase the number of alternating blocks. Rescaling
  would produce a dual-feasible value below the capacity unless every action
  increase was zero, so the resulting distinct-facet word is itself globally
  maximizing. Clarke reconstruction then makes this same word physical; only
  at that point does the planar incidence argument force every maximal block
  to have one or two adjacent facets. Source: Rudolf, Theorem 1, arXiv
  2203.01718 / published DOI `10.1007/s10884-022-10228-0`; local proof details
  are in the active thesis theorem.
- 2026-06-20 legacy/PDF comparison found the old KKT linear system,
  well-definedness, non-maximizer warning, and unpruned solver correctness as
  substantial old material not represented here. This is deliberate for this
  checkpoint: the current section states the HK finite QP interface and a
  compact formula-to-computation bridge, while detailed optimizing/solving
  internals belong to later QP/numerics work.
- 2026-06-25 convention decision: use active traversal / earlier-entry-first
  order throughout the project and thesis. HK2017's displayed theorem uses the
  reversed ordered word; active prose marks the reversal explicitly and uses
  deletion of zero-weight facets as the enumeration convention.

## Deferred Algorithmic Material

- Partial-word enumeration, pruning/admissibility checks, KKT solving,
  repeated/tied minimizer handling, accumulator/certainty status, and empirical
  tests belong to a later session about our algorithmic work.
- The active subsection
  `subsec:quadratic-program-algorithm-hk2019-computation-layers` is intended to
  be the high-level reader bridge, not the detailed solver story. It should not
  grow into a software manual.
