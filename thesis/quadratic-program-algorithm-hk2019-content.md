# Quadratic Program Algorithm Content Notes

Status: section-local content companion for
`thesis/quadratic-program-algorithm-hk2019.tex`. Not source truth.

Purpose: gather the writing inventory for the HK2019 finite optimization
algorithm section.

Overruled by: source papers, `formal/`, `crates/symplectic/`, revalidated
legacy thesis material, experiment artifacts, and Jörn/Kai review.

Lifecycle: keep while the section is being assembled. After the section is
stable, delete this file or reduce it to a short maintenance index.

Update rule: add or change a claim only with a source pointer or an explicit
`needs source` marker.

## Session Scope

- In scope: the Haim-Kislev quadratic-program statement and the mathematical
  reason it computes the EHZ capacity/minimum action for polytopes.
- In scope: notation choices and conversion notes needed to state the standard
  result in the thesis convention.
- Out of scope: the algorithms we developed to solve the quadratic programs.
  Leave pruning, KKT solvers, exact arithmetic, f64 behavior, performance
  optimization, and implementation tests to later algorithm/numerics sessions.
- Out of scope: project-original improvements or insights on top of the HK
  finite formulation, except where a small convention is needed to make the
  standard result readable.

## Purpose And Naming

- Present the standard finite quadratic-program formulation for the minimizer
  search. This is Haim-Kislev's work; the thesis work in this section is
  explanatory and not a new algorithmic contribution.
- First use can call this "Quadratic Program Algorithm Based On HK2019"; after
  first use, use the shorter algorithmic name.
- The bibliography currently also has an `HK2017` entry for the same title; the
  journal-version GAFA citation is 2019 and should be preferred in
  reader-facing naming unless a writer chooses otherwise.
- Open naming decision: choose one reader-facing convention among `HK2017`,
  `HK2019`, or "Haim-Kislev formula/algorithm", and make the bibliography match
  that choice.

## Definition

- Define the finite optimization problem, variables, constraints, objective,
  and reconstruction of the orbit.
- Include sigma/orbit word, beta constraints, quadratic objective including
  normalization/factor convention, admissibility conditions, and orbit
  reconstruction formula.
- State the input contract explicitly: the dual rows have already been checked
  to be extremal and bounded in the sense of the polytope preliminaries.
- The capacity algorithm may use the computed primal vertex set and
  vertex-facet incidence for feasibility and pruning, but mathematical
  correctness is the HK2019 finite-search statement plus the preceding validity
  checks.
- Record the chosen thesis convention for dual vertices versus
  normals/heights, the action matrix, `Q`, `Qmax`, and the factor relating
  `Qmax` to `c_EHZ`. Existing formal notes use `c_EHZ = 1/(2 Qmax)` for their
  `Q`; legacy prose may use a different-looking but equivalent convention and
  must be converted deliberately.
- Convention source: use `formal/hk2017-qp-conventions.tex` for the
  HK2017-to-project translation layer, especially normals/heights to dual
  vertices, the project `J_0`/`omega_0` convention, fixed-word QP orientation,
  and the current sign/order audit target. Cite HK2017/Haim--Kislev separately
  for the theorem itself.

## Correctness

- State and justify that the finite problem computes the intended minimum
  action/capacity in the polytope setting, at the proof depth chosen for this
  thesis section.
- Likely named claims: finite candidate reduction, equivalence between beta
  problem and generalized orbit action, existence of an optimum, and recovery
  of capacity/sys.

## Deferred Algorithmic Material

- Partial-word enumeration, pruning/admissibility checks, KKT solving,
  repeated/tied minimizer handling, accumulator/certainty status, and empirical
  tests belong to a later session about our algorithmic work.
- When this section is drafted, mention implementation only as far as needed to
  explain why later thesis sections can call this standard finite problem a
  computational target.
