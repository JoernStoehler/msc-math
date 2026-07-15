# Generalized Reeb Orbits Content Notes

Status: section-local content companion for
`thesis/03-generalized-reeb-orbits-polytopes.tex`. Not source truth.

Purpose: gather the writing inventory for explaining why finite polytope
computations compute the relevant symplectic quantity.

Use `theory-authoring-map.md` for the cross-chapter reader questions and
ownership decisions that determine what enters this section.

Overruled by: `formal/`, source papers, revalidated legacy thesis material,
active algorithm sections, and Jörn/Kai review.

Lifecycle: keep while the generalized Reeb orbit section is being assembled.
After the section is stable, delete this file or reduce it to a short
maintenance index.

Update rule: add or change a claim only with a source pointer or an explicit
`needs source` marker.

## Session Scope

- In scope: generalized Reeb orbits as the polytope object that connects
  EHZ/minimum action to the finite HK quadratic-program statement.
- In scope: the general AAO/HK nonsmooth duality and generalized-orbit route
  needed to explain why polytopes carry the capacity-minimizing object used by
  both finite methods.
- Out of scope: CH2021's narrower four-dimensional symplectic-polytope
  smoothing/combinatorial model and the project flow-graph algorithm. Move the
  existing CH2021 comparison material to the flow-graph story rather than use
  it to justify the general definition here.
- Out of scope: project-original algorithmic improvements on top of HK or
  CH2021, except for small notation choices needed to state the literature in
  thesis notation.

## Reader Role And Proposed Boundary

This section should answer why a nonsmooth polytope has the right dynamical
object and why the capacity problem contains a finite pure-facet minimizer. It
should begin from the common action/capacity and normalized-facet language in
the preliminaries, then own:

- the generalized Reeb inclusion and contact-normalized facet directions;
- active words, dwell times, closure, cyclic redundancy, and base-point
  recovery;
- the specialization of the preliminary Clarke correspondence to the polytope
  inclusion;
- the piecewise-affine action identity and the simple-minimizer proof.

Keep the detailed Clarke principle in the preliminaries as general convex
theory. The active definition now derives the polytope subdifferential and
shows that the abstract nonsmooth Hamiltonian inclusion becomes the convex
hull of the pure facet directions. The shoelace identity now lives with the
finite word data, before its use in the simple-minimizer argument.

Preserve the settled proof boundary and status recorded in
`PROJECT_COMPLETION.md`: cite dual-minimizer existence and the nonsmooth
multiplier/Euler--Lagrange input; derive the free-period uncentered equation,
its coefficient, translation/rescaling correspondence, and \(I_K=T=A\) in the
thesis convention. In the active proof, the reconstruction scale satisfies
\(c^2=\nu=I_K(z)/T\). The limiting simple dual curve has
\(I_K(z_*)=T\), so its reconstruction scale is one and only a translation is
needed; no separate equality calculation is required. This candidate has
passed source and internal mathematical review; the remaining gate is Jörn's
integrated review, not a new decision about whether to replace the derivation
by citations.

The section should leave the reader ready for either finite method. It should
not explain the Haim--Kislev objective, KKT machinery, flow tubes, or CH2021 as
if they were part of the same proof.

## Definition

- Define generalized Reeb orbits on polytopes in the notation used by the
  algorithms.
- Ingredients to include: piecewise-linear curve, facets or dual vertices
  touched, beta/dwell times, closing condition, action, and what counts as an
  orbit word.
- This checkpoint now absorbs the main useful simple-minimizer proof material
  from `thesis/legacy/simple-minimizer-existence.tex`: `K = {x : <a_i,x> <= 1}`,
  `R_i = 2 J_0 a_i`, generalized Reeb inclusion, closure for pure facet
  words, base-point recovery, the action/period convention, and the five-step
  proof of simple-minimizer existence.
- Absorbed from `thesis/legacy/simple-minimizer-existence.tex`: simple orbit
  means piecewise-constant pure facet Reeb velocities with each facet direction
  used in one connected interval; finite word/dwell/base-point data determine
  an orbit only after the closure equation and base-point recovery construction
  are satisfied. Current thesis prose uses an active-word convention with
  positive dwell times; the legacy proof also uses a padded all-facet
  permutation with zero-duration unused facets.

## CH2021 Boundary

CH2021's Type 1/2/3 and smoothing material now lives in
`05-flow-graph-ch2021-background-comparison.tex`. It is not on the dependency
path from generalized characteristics and simple minimizers to the
Haim--Kislev formula. The source and statement hazards remain owned by the
flow-graph companion.

## Simple Minimizers

- State and prove, or cite and supplement, the existence of simple
  minimum-action generalized Reeb orbits on polytopes.
- Current checkpoint status: the active text now proves the theorem using the
  five-operation legacy proof spine: piecewise affine approximation, splitting
  mixed velocities, merging repeated velocities, rescaling, and compactness.
  The proof introduction states how these operations interact, and the final
  argument uses explicitly indexed approximation and rescaling sequences; the
  former unindexed notation obscured which limit dual minimality controlled.
- Existing thesis text defines "simple" as a generalized Reeb orbit whose
  derivative can be represented by finitely many pure facet Reeb velocities
  with each facet direction occurring in one connected interval, possibly not
  at all. The active prose uses active words with positive dwell times and
  mentions padded all-facet permutations only as an alternate encoding.
- This supports the finite `(sigma, tau, b)` representation while preserving
  the caveat that closure is only the closedness equation for the broken path;
  base-point recovery is a separate finite linear-feasibility condition, even
  though it is not part of the velocity/action data.
- The active text reserves "simple Reeb orbit" for a boundary orbit and calls
  the intermediate position-free objects "simple pure-velocity dual loops".
  The final Clarke reconstruction explicitly recovers the prescribed facet
  labels by extremality of the irredundant polar rows and therefore supplies a
  valid base point.
- The full proof may later move to an appendix if it interrupts the reading
  path, but it should not be deleted or replaced by a bare HK citation.
- 2026-06-20 HK source comparison fixed two active-statement qualifications:
  the simple-minimizer theorem now states \(K\subset\mathbb R^4\),
  full-dimensional with \(0\in\operatorname{int}(K)\), and the prose explicitly
  says that HK's positive multiples of \(J_0n_i\) are reparametrized into the
  contact-normalized directions \(R_i=2J_0a_i\).
- The active free-period proof was rechecked against the proof of HK2017
  Theorem 1.5. Its splitting and merging action inequalities are the same two
  rearrangements, while the local rescaling and compactness argument replaces
  HK's fixed-period normalization without changing the existential theorem.

## Explanatory Assets

`figures/foundations/generate.py` is the thesis-native producer for the two
assets owned by this section:

- `word-closure-basepoint.pdf` separates closure in displacement space from
  translated realization on prescribed facets. It is an exact planar
  Hamiltonian analogue, not a projection or a four-dimensional example. The
  adjacent square counterexample in the prose shows that closure need not
  imply realizability.
- `simple-minimizer-pipeline.pdf` externalizes the five operations, marks all
  intermediate objects as dual loops, and displays the dual-minimality squeeze
  that makes the action increases vanish.

Both assets explain mathematics already proved in the text; neither is source
truth or proof evidence. Run
`uv run --script thesis/figures/foundations/generate.py` from the repository
root to regenerate all foundation figures.
