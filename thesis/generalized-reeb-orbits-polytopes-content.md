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
theory. This material should invoke it rather than restate it, after showing
that the abstract nonsmooth Hamiltonian inclusion becomes the convex hull of
the pure facet directions. Move only the shoelace identity here, near its first
substantive use in the simple-minimizer argument.

Preserve the settled proof boundary and status recorded in
`PROJECT_COMPLETION.md`: cite dual-minimizer existence and the nonsmooth
multiplier/Euler--Lagrange input; derive the free-period uncentered equation,
its coefficient, translation/rescaling correspondence, and \(I_K=T=A\) in the
thesis convention. This candidate has passed source and internal mathematical
review; the remaining gate is integrated reader review, not a new decision
about whether to replace the derivation by citations.

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

## Limit Of Smooth Bodies

- Active draft now includes a compact CH2021 background subsection:
  `subsec:generalized-reeb-orbits-polytopes-ch2021`.
- Source: `papers/ch2021/s1_introduction_and_main_results.tex`, especially
  the definitions of combinatorial Reeb orbits and Types 1/2/3, Theorems 1.8
  and 1.9, Corollary 1.13, and `papers/ch2021/s2_type_1_reeb_orbits.tex`,
  Proposition 2.2 for the Type 1 flow-graph correspondence.
- Claim strength in active prose: CH2021 is background for four-dimensional
  symplectic polytopes and smoothing/combinatorial limits. It is not used as a
  replacement for the HK quadratic program and does not import our flow-graph
  implementation into this checkpoint.
- Proposed ownership change: move this CH2021 subsection to the flow-graph
  story, where its narrower symplectic-polytope hypotheses and its relation to
  the project theorem can be explained together. It is not on the dependency
  path from generalized orbits to the Haim--Kislev formula.
- Review risk: check theorem numbering against the compiled/published CH2021
  version if the bibliography or source numbering changes. The current source
  has Theorem 1.8, Theorem 1.9, Corollary 1.13, and Proposition 2.2.
- 2026-06-20 source-paper comparison fixed three CH2021 summary hazards in
  active prose: Theorem 1.9 needs a sequence \(\varepsilon_i\to0\), Theorem 1.8
  gives eventual equality of rotation/CZ rather than mere convergence, and the
  Corollary 1.13 rotation bound applies to Type 1 orbits only; Type 2 has no
  defined combinatorial rotation number in CH2021.

## Simple Minimizers

- State and prove, or cite and supplement, the existence of simple
  minimum-action generalized Reeb orbits on polytopes.
- Current checkpoint status: the active text now proves the theorem using the
  five-operation legacy proof spine: piecewise affine approximation, splitting
  mixed velocities, merging repeated velocities, rescaling, and compactness.
- Existing thesis text defines "simple" as a generalized Reeb orbit whose
  derivative can be represented by finitely many pure facet Reeb velocities
  with each facet direction occurring in one connected interval, possibly not
  at all. The active prose uses active words with positive dwell times and
  mentions padded all-facet permutations only as an alternate encoding.
- This supports the finite `(sigma, tau, b)` representation while preserving
  the caveat that closure is only the closedness equation for the broken path;
  base-point recovery is a separate finite linear-feasibility condition, even
  though it is not part of the velocity/action data.
- The full proof may later move to an appendix if it interrupts the reading
  path, but it should not be deleted or replaced by a bare HK citation.
- 2026-06-20 HK source comparison fixed two active-statement qualifications:
  the simple-minimizer theorem now states \(K\subset\mathbb R^4\),
  full-dimensional with \(0\in\operatorname{int}(K)\), and the prose explicitly
  says that HK's positive multiples of \(J_0n_i\) are reparametrized into the
  contact-normalized directions \(R_i=2J_0a_i\).
