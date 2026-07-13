# Quadratic Program Content Notes

Status: section-local content companion for
`thesis/04-haim-kislev-quadratic-program.tex`. Not source truth.

Purpose: gather the writing inventory for the Haim--Kislev quadratic-program
section.

Overruled by: source papers, `formal/`, `crates/symplectic/`, revalidated
legacy thesis material, experiment artifacts, and Jörn/Kai review.

Lifecycle: keep while the section is being assembled. After the section is
stable, delete this file or reduce it to a short maintenance index.

Update rule: add or change a claim only with a source pointer or an explicit
`needs source` marker.

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
  minimizer, and Clarke's correspondence realizes it, after translation and
  positive rescaling, as a minimum-action simple generalized Reeb orbit.
  Base-point recovery is then an explicit reconstruction method rather than an
  extra existence hypothesis.
- For the Lagrangian-product enumeration, state only a capacity-search result:
  the restricted alternating \(q/p\)-block family contains a capacity
  minimizer. Do not claim it classifies all minimum-action simple Reeb orbits;
  the billiard bounce bound gives existence of one minimizer with at most three
  bounces, not nonexistence of longer minimizers.
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
