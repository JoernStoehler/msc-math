# Planned TOC

Purpose:
plan what the thesis should look like, while making explicit what is doable and
what relates to what.

Target:
after reading this file, Jorn or an agent should know the intended thesis shape,
which claims/sections support each other, and which parts are still uncertain
because of claim strength rather than prose polish.

Verification gate:
the file is detailed enough when each leaf section has a clear place, a clear
claim or role, known proof/data/notation dependencies, and no unresolved
question of basic writability. If the remaining uncertainty is only page count
or niceness of exposition, stop expanding that leaf.

Non-goal:
this is not an annotated thesis draft. A thesis can always be expanded in more
detail; that expansion has no value here once writability and relations are
clear.

For each prospective section, this file should make clear:

- what would be written here?
- what proof/data/notation does it need?
- is this leaf too large and should be split?
- what other parts does this leaf depend on or support?
- can a mediocre writeup of this leaf damage another part of the thesis?

Derived agent guidance:

- add detail when it changes placement, dependency, claim strength, or whether
  the leaf is writable;
- do not add detail merely because the eventual thesis section could be written
  in more detail;
- preserve messy formulas, caveats, and proof sketches when they are the reason
  a leaf is or is not writable;
- route execution status, ownership, commands, and final verification gates to
  `tasks/*.md`, `ROADMAP.md`, or the relevant research note instead.

The main things to optimize for:

- time: do not spend a week for low marginal gain;
- readability: Kai and Elizabeth need to understand what work was done;
- completeness: do not silently lose results already obtained;
- correctness: do not make false or overstrong claims;
- proof completeness: close proof gaps only when the thesis claim needs them,
  and otherwise state the gap honestly.

Because there is too much content, sections should spoil their point early.
Readers should be able to skip detailed arguments after seeing the claim,
support, and caveat.

Use hand sketches where they explain definitions, theorems, or edge cases
faster than polished figures. Use matplotlib-style figures when visual reasoning
is part of the evidence.

## Title

Probing Viterbo's Conjecture

## Abstract

Write last.

Probably a long paragraph, paper style, with results spoiled:

- Viterbo's conjecture and HKO2024 motivate the project.
- We study computable polytope models in dimension four.
- We locally analyze HKO2024.
- We run bounded computational searches and find no second transferable
  `sys > 1` regime.
- We state the algorithmic and numerical trust boundary.

## 1. Introduction

Purpose: make the reader know the problem, the scope, and the contributions.

Content:

- Brief history/context of Viterbo's conjecture.
- State Viterbo's conjecture.
- HKO2024 as the surprising counterexample.
- Computational approaches such as HK2017 and CH2021, and how their search
  settings differ from HKO2024.
- Why the thesis narrows to polytopes:
  computable, dense, and already used in the existing algorithms.
- Why dimension four:
  higher dimensions have much worse complexity and computational cost.
- Why both generic and non-generic cases matter:
  HKO2024 is highly symmetric/non-generic, but generic random search is still
  the obvious baseline.
- Why implementation and numerics matter:
  a numerical counterexample story is worthless if the solver is not trusted
  near thresholds.

Contributions to mention:

- HKO2024 looks like a local maximum of the systolic ratio.
- Strongest desired version:
  HKO2024 is locally maximal in `M_10` modulo the natural symmetries of `sys`.
- If the exact certificate is not closed, state the weaker exact-route and
  numerical evidence honestly.
- Broad bounded searches, local optimization, continuation, and standard
  data-science-style probes found no second transferable `sys > 1` regime.
- We developed a Rust implementation of the HK2017/CH2021-style finite
  computation and improved it using the combinatorial structure of the problem.
- We developed subgradient / local-ascent machinery for the nonsmooth problem.
- We hardened the computations with numerical checks and exact fallbacks where
  the thesis relies on them.
- We flesh out proof details around existence of a simple minimum-action Reeb
  orbit on a polytope.
- Possible side result:
  formula/status for `P_5 x_L R(theta) P_5`.
- Possible side result:
  high-symmetry-family negative checks.

Writability note:

- The intro can be drafted before all proof details are perfect.
- The exact final sentence for the HKO result waits until the local-maximum
  chapter knows whether it is theorem-strength or evidence-strength.

## 2. Background

Purpose: define the objects used later. Prove only what the main text needs.

### 2.1 Polytopes and Euclidean geometry

Content:

- Convex bodies and polytopes in `R^4`.
- `VPolytope` and `HPolytope`.
- Correspondence between vertices/facets when needed.
- Closed `k`-faces.
- Dual polytopes.
- Support functions and gauge functions.
- Space of convex bodies.
- Space of polytopes.
- `M_F`: polytopes containing the origin in their interior with `F`
  irredundant facets.
- Embedding of `M_F` into an open subset of `R^{4F}` by dual vertex
  coordinates.
- Generic properties:
  introduce only the finite list actually used later.

Writability note:

- This is mostly standard notation and should be writable mediocrely.
- The `M_F` definition matters later for HKO local maximality, so do not hide it
  in an appendix.

### 2.2 Smooth symplectic geometry setting

Content:

- Notation `J_0`, `omega_0`, `lambda_0`, `Sp(4)`.
- Action.
- Minimum action of a convex body with smooth boundary.
- Reeb vector field.
- Reeb orbits.
- Action of a Reeb orbit.
- Cite existence of at least one Reeb orbit.
- Cite existence of a minimum.
- Symplectic capacity axioms:
  monotonicity, conformality, normalization.
- Cite that minimum action is a symplectic capacity, called EHZ capacity.
- Viterbo's conjecture in the chosen normalization.
- Brief relation to the capacity-equality story if it helps motivation.

Writability note:

- This section can be mostly definitions plus citations.
- Do not try to teach all of symplectic geometry.

### 2.3 Symplectic geometry on polytopes

Content:

- Main method:
  polytopes as limits of smooth convex bodies.
- Generalized Reeb orbits on polytopes.
- CH2021-style compactness statement:
  if `K_n -> K` and the actions are bounded, a subsequence of orbits converges
  to a generalized Reeb orbit on `K`.
- TODO:
  look up the exact CH2021 statement and topology.
- Corollary or cited fact:
  EHZ capacity behaves correctly under the relevant polytope limit.
- Simple Reeb orbits on polytopes.
- HK2017 theorem:
  for any polytope, there is a simple Reeb orbit with minimum action.

Writability note:

- The detailed proof of simple-minimizer existence can be long because it uses
  Clarke dual action principle material.
- Main text must still state why the finite computation is legitimate.
- Long functional-analytic proof material can move to an appendix only after
  the main text states the claim and why it applies here.

## 3. Finite Computation Methods

Purpose: explain the finite optimization problem and the algorithms used later.

### 3.1 HK2017 finite optimization problem

Content:

- Recall the minimum action optimization problem.
- Define the HK2017 problem in `(sigma,beta)`.
- State equivalence to the original problem.
- Include shoelace-style formula for the symplectic action of a piecewise
  linear curve if this is the clean way to explain the objective.
- Give formulas for recovering `gamma` from `(sigma,beta)` and conversely.

Candidate problem statement:

- `sigma` is a partial permutation / orbit word.
- `beta_i >= 0`.
- Constraints:
  `sum_i beta_i a_{sigma(i)} = 0`,
  `sum_i beta_i = 1`.
- Objective:
  `Q(sigma,beta) = sum_{i<j} beta_i beta_j
  omega_0(a_{sigma(i)}, a_{sigma(j)})`,
  with final factor convention checked against thesis/code notation.
- Minimum action / capacity is recovered from this objective with the chosen
  normalization.

Writability note:

- This section needs enough formulas to make later HKO/landscape text readable.
- It does not need every implementation optimization.

### 3.2 HK2017 solver

Content:

- Input:
  `F` dual vertices `a_i`.
- Output:
  a partial permutation `sigma` and weights `beta`, or all tied best candidates
  when needed.
- Enumerate partial permutations / orbit words.
- For each `sigma`, solve the linear constraints:
  `sum_i beta_i a_{sigma(i)} = 0`,
  `sum_i beta_i = 1`.
- This gives an affine solution space for `beta`, possibly empty.
- Project the quadratic objective to the constraint space:
  `Q'(sigma,beta') = 1/2 beta'^T H'(sigma) beta'
  + b'(sigma)^T beta' + c'(sigma)`.
- Check negative definiteness, or record semidefinite cases if the retained
  version needs them.
- Find the critical point when it exists.
- Recover `beta` in the original coordinates.
- Check positivity/admissibility.
- Update the best solution.

Correctness proof sketch:

- The admissible `beta` sets are compact.
- There are finitely many `sigma`.
- The objective is continuous, so a maximum exists.
- Decompose by faces where some `beta_i = 0`.
- Choose a maximum on a minimal face.
- Since it lies inside that face, it is a critical point of the projected
  objective.
- If the projected Hessian had a positive eigenvalue, a better nearby point
  would exist.
- If it had a zero eigenvalue, one could move to the boundary and contradict
  minimal face dimension.
- The positivity check excludes critical points outside the admissible face.

Writability note:

- This proof sketch is probably writable.
- Check the sign and factor conventions before finalizing formulas.

### 3.3 Implementation variants

Content:

- Track one best solution or all tied best solutions.
- Track semidefinite cases that define positive-dimensional families of maxima.
- Expand partial permutations to full permutations by padding with zeros when
  that is notationally convenient.
- Use pruning / combinatorial structure where it is needed for performance.
- State what the Rust implementation returns and what the status/certainty
  fields mean.

Writability note:

- Keep implementation detail only where it affects correctness,
  reproducibility, or interpretation of experiment outputs.
- Align this section with `thesis/migration-findings.md`:
  multiplier names, KKT signs, `Q` factor, beta/eigen thresholds, accumulator
  wording, and pruning assumptions.

### 3.4 First-order branches and subgradients

Purpose:
separate the formal first-order behavior from the practical fact that we can run
gradient ascent.

Content:

- To analyze a local neighborhood of a polytope `K`, fix `sigma` and look at
  the corresponding action branch as a function of dual vertices.
- For fixed `sigma`, define a branch such as `A_min(a;sigma)` when the
  constrained critical point exists.
- Generic behavior:
  the constraint matrix has maximum rank and the constrained quadratic problem
  has an isolated negative-definite critical point.
- Open conditions such as `beta_i > 0` and negative definiteness are not enough
  for describing limits.
- For limit arguments, use closed replacements:
  `beta_i >= 0`, negative semidefinite conditions, and possibly sets of
  critical `beta`.
- If `K_n -> K`, then after passing to a subsequence,
  `sigma_n` and `beta_n` should converge to a limiting minimizer.
- `sys(K)` is not smooth in the dual vertices, but it is controlled by the
  active branches.
- Define/describe:
  `Sigma_min(a)`,
  active `beta(a;sigma)`,
  branch action values,
  and the active subgradients.
- Expected behavior:
  active sets and active `beta` sets are semicontinuous in the relevant sense.
- Generic uniqueness of `beta(a;sigma)` is a dense/open-type statement; higher
  dimensional `beta` sets are special closed cases.

Writability note:

- This is the section most likely to sprawl.
- It interacts strongly with HKO local maximality at detail level.
- It interacts only weakly with the search-landscape narrative.
- If the proof gets ugly, keep the main conceptual statement here and move
  routine algebra/case checking to an appendix.

## 4. HKO2024 Local Maximum

Purpose: state and support the strongest honest local-maximality claim.

Content:

- Define the HKO pentagon-pentagon configuration.
- State its `sys > 1` role.
- Define the `M_10` local model in dual-vertex coordinates.
- Explain the natural symmetries of `sys`:
  translations, scaling, and linear symplectic maps.
- State that strict local maximality in raw `R^40` is the wrong statement.
- Desired theorem:
  HKO2024 is locally maximal in `M_10` modulo the natural symmetries.
- Exact first-order route:
  - exact geometry and symmetry tangent space;
  - active orbit/prototype reduction;
  - active-gradient rank/kernel certificate.
- Current caveat:
  Packet 3 is the blocker if we want theorem-strength wording.
- Current caveat:
  exact field is `Q(tan(pi/5))`, not `Q(sqrt(5))`.
- Current caveat:
  old `44`-orbit / `10`-gradient language must be replaced or caveated against
  current `150` exact action orbits, `20` visited subsets, and `28` gradients.
- Supporting evidence:
  first-order numerical bookkeeping,
  second-order samples,
  perturbation checks,
  facet-splitting,
  cut-and-ascent,
  neighborhood checks.

Writability note:

- This chapter owns the detailed first-order story.
- If the exact certificate closes, this becomes the main theorem chapter.
- If it does not close, this chapter still exists as a carefully weakened
  evidence/exact-route chapter.
- Do not let gradient-ascent exposition carry the theorem proof.

## 5. Search Landscape

Purpose: state what was searched, what was found, and what this does not prove.

Content:

- Search question:
  can standard computational methods find another high-`sys` regime?
- Random generic polytopes.
- Random Lagrangian products.
- Fixed-`F` general gradient ascent.
- Fixed-`F` product ascent.
- Variable-`F` continuation.
- Rotated regular products.
- Pentagon-pentagon as the known positive control.
- Data-science feature/regression/classifier work if retained.
- Volumes of local maxima / dimensionality heuristic if it helps explain why
  interesting cases are rare.
- Negative interpretation:
  bounded searches found no second transferable `sys > 1` regime.
- Caveat:
  this is not a density theorem.
- Caveat:
  this is not an impossibility theorem.
- Caveat:
  this is not proof that better nonlocal methods cannot find more examples.

Writability note:

- This chapter mostly does not depend on the HKO proof details.
- It uses HKO as baseline/control geometry.
- It does depend on method-ledger/audit status for whatever data-science methods
  are mentioned.

## 6. Numerics and reproducibility

Purpose: state the trust boundary for computations cited in the thesis.

Content:

- What the default f64 solver computes.
- What exact or stronger verification paths exist.
- Which path is used for which thesis claim.
- KKT/numerical error caveats only where cited.
- Interval/exact fallback story only as far as the thesis uses it.
- Orbit recovery evidence if cited.
- Which datasets and figures are thesis-supporting.
- Which repo promises are actually made.

Writability note:

- This is not a full numerical-analysis thesis.
- Do not reopen broad solver development here.
- Verify only artifacts cited or promised by the final thesis text.

## Appendices

Use appendices for material that is needed for correctness but interrupts the
main reading path.

Appendix A: notation glossary.

Appendix B: Clarke dual action principle and simple-minimizer proof details.

Appendix C: finite algorithm proof details that are too heavy for Chapter 3.

Appendix D: numerical implementation details.

Appendix E: optional figures / visualization / pentagon-rotation material, only
if it helps and is cheap.

Appendix rule:

- Main text must keep the claim, key reduction, and reason the result applies.
- Appendices may hold routine algebra, long case splits, lookup notation, and
  implementation detail.

## Current cut/default-not-mainline material

- Tube algorithm, unless the missing proof/formula becomes available and worth
  the time.
- New LICCA-scale runs, unless results already exist and are cheap to cite.
- Broad solver unification.
- New data-science methods.
- Publication-grade higher-`F` HKO checks.

## Leaf test

For every leaf, first write the four-line version:

1. Claim.
2. Support.
3. Caveat.
4. Pointer to proof, artifact, or appendix.

Only polish notation after this exists.
