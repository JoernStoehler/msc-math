# Theory Source Reorganization Plan

Status: implementation plan derived from `theory-authoring-map.md` and the
active TeX input graph. It records safe source moves and unresolved placement
decisions; it is not publication prose or mathematical source truth.

Purpose: turn the accepted explanatory ownership into a dependency-safe TeX
editing sequence. The plan deliberately does not preserve existing files or
paragraph order merely because they are active.

## Target Reading Path

The early theory should establish this dependency chain:

1. symplectic and action conventions;
2. convex Hamiltonian language, including the general nonsmooth
   characteristic inclusion;
3. normalized polytope input language and the basic Lagrangian-product
   definition;
4. EHZ capacity and the systolic objective;
5. Clarke's general dual variational principle;
6. specialization to polytope facet directions, finite word data, and the
   existence of a simple minimizer;
7. the Haim--Kislev finite quadratic program;
8. the separate flow-graph construction and its relation to CH2021.

This order removes the current forward dependency in the Clarke passage. A
reader meets

\[
  -J_0\dot\gamma(t)\in\partial g_K^2(\gamma(t))
\]

as the general convex-body notion before Clarke's theorem. The generalized
polytope passage then derives

\[
  \partial g_K^2(x)
  =2\operatorname{conv}\{a_i:x\in F_i\},
  \qquad R_i=2J_0a_i,
\]

rather than defining an apparently unrelated object after Clarke has already
referred to it.

## Preliminaries Source Split

Replace the current mixed ownership with these semantic units. Filenames are
provisional; purposes and boundaries are the decision.

| Proposed source unit | Owns | Deliberately does not own |
|---|---|---|
| `02-preliminaries-convex-symplectic-notation.tex` | Coordinates, \(J_0\), \(\omega_0\), \(\lambda_0\), action, smooth characteristic/Reeb normalization, and \(A=T\) | Piecewise-affine word action |
| `02-preliminaries-convex-hamiltonian-language.tex` | Convex bodies containing the origin, support and gauge functions, \(H_K=g_K^2\), convex subdifferentials, the general smooth/nonsmooth Hamiltonian inclusion, and its contact normalization | Polytope algorithms or finite face data |
| `02-preliminaries-polytope-input-language.tex` | Normalized halfspaces \(a_i=n_i/h_i\), polarity, full dimensionality, boundedness, irredundancy, and labelled versus unlabelled presentations | Vertex enumeration, incidence extraction, kernel tests, or local charts |
| `02-preliminaries-lagrangian-products.tex` | The geometric \(q/p\)-product definition, factor placement, and distinction from a symplectic product | Enumeration order, pentagon labels, family symmetries, or volume computations |
| `02-preliminaries-ehz-capacity.tex` | Least-action EHZ capacity, compact capacity axioms, the four-dimensional systolic ratio, Viterbo's threshold, and only the immediate translation/scaling consequences needed to understand them | HKO quotient machinery, a polygon limit lemma, or the unused symplectic-product formula |
| `02-preliminaries-clarke-dual-action-principle.tex` | The current detailed free-period, uncentered dual principle and its settled citation/proof boundary | Its finite polytope/QP specialization |

The present `02-preliminaries-polytope-representations-finite-geometry.tex`
is therefore a source reservoir, not a unit to revise in place. Its opening
normalized-halfspace discussion feeds the polytope-input unit; its final
support/gauge/Hamiltonian discussion feeds the convex-Hamiltonian unit; and
the short geometric core of its Lagrangian-product definition feeds the
Lagrangian-product unit.

## Generalized-Orbit Source

Keep `03-generalized-reeb-orbits-definition.tex` focused on the specialization
of the general inclusion to polytopes. It should calculate the active
subdifferential, obtain the pure facet directions, and explain why the same
normalization gives action equal to elapsed time.

Rename or reshape
`03-generalized-reeb-orbits-words-dwell-times-closure.tex` as the finite-word
data unit. It should own active words, dwell times, closure, cyclic
redundancy, base-point feasibility, and the piecewise-affine action formula.
Move the symplectic shoelace lemma and proof out of
`02-preliminaries-convex-symplectic-notation.tex` into this unit before its
first substantive use. Preserve the current label during the structural move;
rename it only in a later cleanup if the reference churn buys clearer
navigation.

Keep `03-generalized-reeb-orbits-simple-minimizers.tex` after the finite-word
unit. It may then use the action formula and base-point condition without
pulling algorithm-facing material into the preliminaries.

Move `03-generalized-reeb-orbits-ch2021-smoothings-limits.tex` out of this
reading path. Its hypotheses, Type 1/2/3 distinction, smoothing results, and
finite-segment statement are specific to the CH2021 comparison, not to the
general polytope-to-QP reduction.

## Later Consumer Moves

These moves should occur only while revising the named consumer, because the
surrounding explanation and hypotheses need to be rewritten together.

| Current material | Destination and integration decision |
|---|---|
| Systolic symmetry group and invariance | Move from the EHZ source to the HKO local-maximality opening/chart reduction. Update its three active references there. The general scaling and symplectic-invariance facts remain with EHZ. |
| Hausdorff continuity proposition | Move immediately before its endpoint use in `09-rotated-regular-polygons-exact-certificate.tex`; verify the capacity-continuity citation while integrating it. |
| Lagrangian product volume and rotation invariance | State with the pentagon-profile proof in `09-rotated-regular-polygons-pentagon-profile-theorem.tex`, where it converts capacity branches into the systolic formula. |
| Lifted \(q/p\) facet blocks and product enumeration | Explain beside the Lagrangian-product enumeration in `04-haim-kislev-quadratic-program.tex`; retain only the geometric product definition earlier. |
| Fixed-facet Hausdorff/chart conditions | Own in the HKO chart reduction and later repeat only the first-order hypotheses actually needed there. Do not present a generic preliminary chart theorem before its valid scope is settled. |
| CH2021 smoothing and Type 1/2/3 material | Integrate into the existing “Relation to CH2021” passage in `05-flow-graph-algorithm-ch2021.tex`, after the project correctness and genericity results and before implementation evidence. Preserve the distinction between the published CH2021 route and the project's simple-word correctness proof. |
| Detailed polygon label order | Own with the exact polygon certificate or its data interface, not with the definition of Lagrangian product. |

The symplectic-product capacity formula in
`02-preliminaries-ehz-capacity.tex` has no active consumer. Remove it from the
publication path rather than relocate it.

## Material Without A Settled Publication Owner

Do not transplant these paragraphs merely to make the preliminary split
complete:

- four-hyperplane vertex enumeration;
- vertex--facet incidence extraction and the incidence-only edge/two-face
  rules;
- the triple-kernel boundedness test;
- simplex triangulation as a general volume recipe.

For each item, first identify the actual theorem, algorithm, or evidence
interface that requires it. Then choose among a restricted input contract, a
consumer-local explanation, an appendix audit, or no publication claim. In
particular, do not repair the incidence rule until its input class is explicit;
the rule may disappear, become a deliberately restricted pipeline contract,
or need replacement by an exact face-dimension test.

The relevant current source owners to inspect at that point include
`crates/symplectic/src/exact/polytope.rs`,
`crates/euclidean-polytopes/src/faces.rs`, and
`formal/random-polytope-boundedness.tex`.

## Implementation Order And Gates

1. Establish the convex-Hamiltonian seam before Clarke, then read Clarke and
   the generalized-orbit opening continuously. This is the first pilot because
   it tests the hardest conceptual dependency.
2. Split the mixed finite-geometry source. Keep deferred material recoverable
   through Git and this plan; do not create `.bak` files.
3. Move the shoelace lemma into the finite-word data unit and recheck the
   generalized-orbit proof spine and QP references.
4. Integrate the CH2021 source beside the flow-graph comparison and remove its
   old input from the generalized-orbit wrapper.
5. Integrate product-specific geometry while revising the QP material. Do not
   disturb the current finite theorem spine merely to complete a file move.
6. Move HKO symmetry material only during an HKO coherence pass, and move the
   continuity/product-volume material only during the polygon proof pass.
7. Leave first-order geometry and the finite-incidence contract deferred until
   their required claims and input classes are settled.

After each structural stage, rebuild the thesis, check labels and references,
and reread the affected transition in the rendered PDF. Passing LaTeX is only
a structural check. The substantive gate is whether the resulting reader can
answer the generating question for that passage without importing a later
algorithm or silently filling a forward dependency.

The active Clarke source may pass through structural moves, but its settled
proof boundary in `PROJECT_COMPLETION.md` remains fixed. Delete
`legacy/clarkedual-action-principle.tex` only after the integrated active
passage has passed Jörn's review and its useful source decisions have been
absorbed.
