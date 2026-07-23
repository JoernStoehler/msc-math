# Theory Source Reorganization Plan

Status: implementation plan derived from `theory-authoring-map.md` and the
active TeX input graph. It records safe source moves and unresolved placement
decisions; it is not publication prose or mathematical source truth.

Purpose: turn the accepted explanatory placement into a dependency-safe TeX
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

This order removed the former forward dependency in the Clarke passage. A
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

The active split uses these semantic units. The purposes and boundaries, not
the filenames by themselves, are the durable decision.

| Active source unit | Contains | Deliberately excludes |
|---|---|---|
| `02-preliminaries-convex-symplectic-notation.tex` | Coordinates, \(J_0\), \(\omega_0\), \(\lambda_0\), action, smooth characteristic/Reeb normalization, and \(A=T\) | Piecewise-affine word action |
| `02-preliminaries-convex-hamiltonian-language.tex` | Convex bodies containing the origin, support and gauge functions, \(H_K=g_K^2\), convex subdifferentials, the general smooth/nonsmooth Hamiltonian inclusion, and its contact normalization | Polytope algorithms or finite face data |
| `02-preliminaries-polytope-input-language.tex` | Normalized halfspaces \(a_i=n_i/h_i\), polarity, full dimensionality, boundedness, irredundancy, and labelled versus unlabelled presentations | Vertex enumeration, incidence extraction, kernel tests, or local charts |
| `02-preliminaries-lagrangian-products.tex` | The geometric \(q/p\)-product definition, factor placement, and distinction from a symplectic product | Enumeration order, pentagon labels, family symmetries, or volume computations |
| `02-preliminaries-ehz-capacity.tex` | Least-action EHZ capacity, compact capacity axioms, the four-dimensional systolic ratio, Viterbo's threshold, and only the immediate translation/scaling consequences needed to understand them | HKO quotient machinery, a polygon limit lemma, or the unused symplectic-product formula |
| `02-preliminaries-clarke-dual-action-principle.tex` | The current detailed free-period, uncentered dual principle and its settled citation/proof boundary | Its finite polytope/QP specialization |

The former
`02-preliminaries-polytope-representations-finite-geometry.tex` was used as a
source reservoir during this split and is now recoverable through Git. Its
opening normalized-halfspace discussion fed the polytope-input unit; its final
support/gauge/Hamiltonian discussion fed the convex-Hamiltonian unit; and the
short geometric core of its Lagrangian-product definition fed the
Lagrangian-product unit.

## Generalized-Orbit Source

`03-generalized-reeb-orbits-definition.tex` is focused on the specialization
of the general inclusion to polytopes. It should calculate the active
subdifferential, obtain the pure facet directions, and explain why the same
normalization gives action equal to elapsed time.

`03-generalized-reeb-orbits-words-dwell-times-closure.tex` is the finite-word
data unit. It contains active words, dwell times, closure, cyclic redundancy,
base-point feasibility, the piecewise-affine action formula, and the relocated
symplectic shoelace lemma. The former preliminary label was retained to avoid
reference churn; rename it only if a later cleanup buys clearer navigation.

Keep `03-generalized-reeb-orbits-simple-minimizers.tex` after the finite-word
unit. It may then use the action formula and base-point condition without
pulling algorithm-facing material into the preliminaries.

The former `03-generalized-reeb-orbits-ch2021-smoothings-limits.tex` is no
longer in this reading path. Its useful hypotheses, Type 1/2/3 distinction,
smoothing results, and finite-segment statement moved to
`05-flow-graph-ch2021-background-comparison.tex`, because they are specific to
the CH2021 comparison rather than the general polytope-to-QP reduction.

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

## Material Without A Settled Publication Location

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

The relevant implementation and formal sources to inspect at that point include
`crates/symplectic/src/exact/polytope.rs`,
`crates/euclidean-polytopes/src/faces.rs`, and
`formal/random-polytope-boundedness.tex`.

## Implementation Order And Gates

The first four stages are implemented in the active source: the
convex-Hamiltonian seam before Clarke, the mixed preliminary source split, the
shoelace relocation, and the CH2021 move to the flow-graph comparison. The
unused symplectic-product digression was also removed. These changes passed a
clean build and local rendered inspection but not Jörn's integrated review.
Git and this plan retain the deferred source decisions; there are no `.bak`
copies.

The remaining integration gates are:

1. Integrate product-specific geometry while revising the QP material. Do not
   disturb the current finite theorem spine merely to complete a file move.
2. Move HKO symmetry material only during an HKO coherence pass, and move the
   continuity/product-volume material only during the polygon proof pass.
3. Leave first-order geometry and the finite-incidence contract deferred until
   their required claims and input classes are settled.

After each structural stage, rebuild the thesis, check labels and references,
and reread the affected transition in the rendered PDF. Passing LaTeX is only
a structural check. The substantive gate is whether the resulting reader can
answer the generating question for that passage without importing a later
algorithm or silently filling a forward dependency.

The active Clarke source may pass through structural moves, but its settled
proof boundary in `docs/project-status.md` remains fixed. Delete
`legacy/clarkedual-action-principle.tex` only after the integrated active
passage has passed Jörn's review and its useful source decisions have been
absorbed.
