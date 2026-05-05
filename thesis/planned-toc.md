# Planned TOC

Purpose:
record the thesis outline discussed with Kai, while making explicit what each
part is for and what still needs local decisions during writing.

Status:
meeting-derived outline. This is not polished prose and not yet a complete
section-by-section writing plan.

Target:
after reading this file, Jorn or an agent should know the intended thesis shape,
which claims and methods are main text, which details belong in appendices, and
which parts should be polished only lightly.

Derived agent guidance:

- add detail when it changes placement, dependency, claim strength, or whether
  the section is writable;
- do not add detail merely because the eventual thesis section could be written
  in more detail;
- preserve caveats and proof sketches when they determine what can honestly be
  claimed;
- put execution status, ownership, commands, and final verification gates in
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

Real Audience: Kai and Elizabeth

Imaginary Audience: master students who know
- what polytopes are
- symplectic geometry lecture
- basic optimization lectures (e.g. gradient ascent, critical points, search algorithms)

## Meeting-Derived TOC

### Title

Probing Viterbo's Conjecture.

### Abstract

Write last. Standard abstract content: motivation, problem, methods, results, and significance.

Main results worth stating:
- HKO2024 is a local maximum among ten-facet polytopes up to symmetry.
- HKO2024 is the global maximum among lagrangian products of two rotated regular pentagons.
- Black-box data-science tools and related search methods did not yield any new polytopes with `sys>1`.

Main methods:
- The thesis reduces to computable problems.
- The thesis develops, improves, tests and numerically analyzes algorithms.
- We used AI for most of the labor that went into the thesis.

### Introduction

Write late. Standard introduction content: broad problem and interest, progress so far, motivation for the thesis, argue the scope restrictions, the thesis contribution, and the presented structure.

Content:

- Quickly defined Viterbo's conjecture and systolic ratio.
- It relates to fundamental questions about symplectic capacities.
- Historical attempts in either direction, partial results and related progress.
- HKO2024 settles the question, but is surprising and interestingly unique.
- The thesis resumes the computational efforts.
- For tractability, we restrict to polytopes, to 4d, to low facet counts, but retain non-generic cases such as lagrangian products.
- Structure of the thesis, chaining what is needed for each main/side result.

## Preliminaries

Purpose:
- define the mathematical objects needed by later chapters.
- show the simple, robust way to think about the domain, not hacks.
- collapse notation and conventions that the literature isn't consistent about.

### Polytopes

Content:

- polytopes in `R^4`
- duality between convex hulls of finite sets and bounded intersections of half-spaces
- polytopes containing zero: the dual polytope with dual vertices
- support and gauge functions
- the face lattice, names
- Convex bodies, Convex smooth bodies, Hausdorff distance
- The topological space of polytopes with a fixed number of facets.
- Example: HKO2024 as a ten-facet polytope.
- Algorithm: Volume of a polytope

### Smooth Symplectic Geometry

Content:

- Standard symplectic structure and notation on `R^4`.
- Reeb vector fields for smooth convex bodies.
- Reeb trajectories and Reeb orbits.
- Action of a curve.
- Minimum action
- Cited without proof: existence of a Reeb orbit, existence of a minimum action
- Symplectic capacities.
- Cited without proof: Minimum action is a symplectic capacity.
- Viterbo's conjecture for smooth convex bodies.
- Continuity argument via scaling and Hausdorff distance.

### Clarke dual action principle

Content:

- State the principle at the level needed for the thesis.
<!-- TOC DECISION: Include the fleshed-out polished proof here. This is not just
     motivational background. The section should name the function space,
     action functional, minimizer/existence statement, and the conclusion used
     later for polytopes. -->
- Use it to justify the later finite computation story.
<!-- OUTLINE GAP: Say which later step it justifies: existence of a
     minimum-action orbit, reduction to a closed curve/action minimization
     problem, or the HK2017 finite optimization problem. -->
- Keep functional-analytic details only where they are needed for correctness.
<!-- TOC DECISION: Because the polished proof already exists, do not hide all
     functional-analytic details by default. Decide only how to pace the proof
     so it remains readable. -->

## Generalized Reeb Orbits On Polytopes

Purpose:
explain why the finite polytope computations compute the relevant symplectic
quantity.

### Definition

Define generalized Reeb orbits on polytopes in the notation used by the
algorithms.
<!-- OUTLINE GAP: Replace this with the actual definition ingredients:
     piecewise-linear curve, facets/dual vertices touched, beta/dwell times,
     closing condition, action, and what counts as an orbit word. -->

### Limit of smooth bodies

Explain generalized Reeb orbits as limits of smooth convex bodies.
<!-- OUTLINE GAP: Name the exact limit statement needed: what converges
     (bodies, curves, actions), in which topology, and what conclusion is used
     later. -->

Writing note:
check the exact CH2021 statement and topology before final prose.

### Existence of simple minimizers

State and prove, or cite and supplement, the existence of simple
minimum-action generalized Reeb orbits on polytopes.
<!-- TOC DECISION: Prove this in the thesis. Existing thesis text defines
     "simple" as a generalized Reeb orbit whose derivative is piecewise
     constant, each constant value is a pure facet Reeb vector, and each facet
     velocity occurs on a single interval, possibly empty. This is the option
     that wins because it gives the finite `(sigma, tau, b)` representation. -->

Placement note:
the main text must state the result and why it applies. Long proof details can
move to an appendix if they interrupt the reading path.

## Algorithm Based On HK2017

Purpose:
present the first finite algorithmic formulation for the minimizer search.

### Definition

Define the HK2017 finite optimization problem, variables, constraints,
objective, and reconstruction of the orbit.
<!-- OUTLINE GAP: Spell out the problem in the ToC: sigma/orbit word, beta
     constraints, quadratic objective including normalization/factor convention,
     admissibility conditions, and orbit reconstruction formula. -->

### Correctness

Prove that the finite problem computes the intended minimum action/capacity in
the polytope setting.
<!-- OUTLINE GAP: Split the correctness proof into named claims: finite
     candidate reduction, equivalence between beta problem and generalized
     orbit action, existence of an optimum, and recovery of capacity/sys. -->

### Performance optimization

Explain only the optimizations that matter for making the computation feasible
or for understanding later experiments.
<!-- OUTLINE GAP: Name the optimizations to discuss. Candidate list:
     partial-word enumeration, pruning/admissibility checks, KKT solve,
     repeated/tied minimizer handling, and accumulator/certainty status. -->

### Empirical tests

State the tests that support the implementation. Put detailed test outputs and
knobs outside the main reading path.
<!-- OUTLINE GAP: Name the test families and what each is supposed to catch:
     known polytopes, HK2017-vs-CH2021 comparison, exact/f64 spot checks,
     orbit recovery, and regression tests for past bugs. -->

## Algorithm Based On CH2021

Purpose:
present the second minimizer-search algorithm. This is expected to substitute
for HK2017 in practical minimizer search once finished.
<!-- NAMING QUESTION: "Tube algorithm" is probably the better algorithmic
     picture for the objects we compute and operate with. "Algorithm based on
     CH2021" is less informative, and CH2021 itself is less sophisticated than
     the current tube/intersection/search algorithm. Decide final section title
     after the implementation/source text lands. -->

### Definition

Define the algorithm in the current thesis notation.
<!-- OUTLINE GAP: Replace this with the concrete CH2021/tube objects:
     input polytope, face graph, tube, primitive tube, tube intersection,
     action restriction, closed-loop fixed points, and output orbit/capacity. -->

Writing note:
use the current mathematical source for the tube/CH2021-style algorithm, not
stale old thesis text.

### Correctness

Prove the algorithm computes the same target as the HK2017 formulation under
the stated assumptions.
<!-- OUTLINE GAP: Name the assumptions and proof steps: exhaustive simple-word
     search, pruning claims used or not used, fixed-point solving, and
     comparison with the generalized Reeb orbit definition. -->

### Performance optimization

Explain the performance improvements that make the algorithm useful in the
thesis computations.
<!-- OUTLINE GAP: Name the concrete improvements once the implementation is
     finished: what is asymptotically or practically avoided compared to
     HK2017, and which pruning/search choices are actually used. -->

### Empirical tests

State comparison tests against HK2017 and any targeted tests for the algorithm's
own objects.
<!-- OUTLINE GAP: Name expected tests: primitive maps, empty intersections,
     action restriction, fixed points, small examples against HK2017, and any
     HKO/regular-polygon cases used in the thesis. -->

## Algorithm For First-Order Perturbations

Purpose:
explain how local perturbations of a polytope are evaluated for gradient-like
search and for the HKO2024 local-maximum computation.

### Definition

Define the first-order objects in the notation used by the rest of the thesis.
<!-- OUTLINE GAP: Name the objects: row-coordinate perturbation h, active
     orbit/word set, branch action, beta derivative or subgradient, volume
     derivative, and derivative of sys. -->

Writing note:
introduce the generic case first because it is readable. Handle edge cases only
where the HKO computation or the thesis claims need them.
<!-- OUTLINE GAP: Decide the exact generic hypotheses to list here:
     positive dwell times, full-rank constraints, negative-definite reduced
     Hessian, unique active minimizer or finite active set, and fixed face
     combinatorics for volume. -->

### Correctness

Prove the generic case cleanly. Add non-generic cases afterward as needed for
HKO2024 or for honest caveats.
<!-- OUTLINE GAP: Split into theorem statements: differentiability of one
     branch, derivative formula for capacity/action, derivative formula for
     volume, derivative/subgradient statement for sys, and separate caveat for
     ties or semidefinite cases. -->

### Empirical tests

State the numerical checks that the implementation behaves as expected on the
examples used in the thesis.
<!-- OUTLINE GAP: Name the checks: finite-difference agreement, active-set
     stability, comparison to ascent behavior, and HKO-specific row/rank
     bookkeeping. -->

## Main Result: HKO2024 Is A Local Maximum Among Ten-Facet Polytopes Up To Symmetry

Purpose:
state and prove the main local-maximum theorem.

### Decision problem

Define the exact local question:
HKO2024 among ten-facet polytopes, modulo the natural symmetries.

Content:

- The HKO2024 configuration.
- The ten-facet local model.
- Translations, scaling, and linear symplectic maps as symmetries.
- Why raw strict local maximality in the ambient coordinates is not the right
  statement.

### Computation with SageMath

State the exact computation used to prove local maximality:

- exact coordinate field;
- active orbit/row set;
- active-gradient rank/kernel comparison;
- equality of flat first-order directions with the symmetry tangent space.

Writing note:
the main text should explain the conceptual reduction and the result of the
calculation. Long row lists, generated certificates, and verification details
belong in the SageMath appendix.

### Empirical tests

State the supporting checks:

- first-order numerical bookkeeping;
- second-order checks;
- perturbation checks;
- facet-splitting / cut-and-ascent / neighborhood checks if retained.

Writing note:
these are support and sanity checks, not substitutes for the exact calculation.

## Main Result: Black-Box Data-Science Tools Are Insufficient

Purpose:
state the negative search result: the attempted black-box and standard
data-science methods did not produce a second transferable high-`sys` regime.

Writing note:
negative results can be stated in batches because many rows have the same
conclusion. The main text can use a table or bullet list.

### Algorithm for random polytopes

State how random polytopes were generated and what search question this tests.
<!-- OUTLINE GAP: Name the generator, parameters, row counts, retained facets,
     normalization, and exact negative claim licensed by this sample. -->

### Algorithm for gradient ascent

State how local ascent was run and what search question this tests.
<!-- OUTLINE GAP: Name fixed-F/product/continuation variants, seed counts,
     stopping rules, escape logic, and whether the claim is only local-search
     negative evidence. -->

### Rows: polytopes

Define the table rows used in the data-science attempts:
random polytopes, products, ascent endpoints, continuation endpoints, and any
other retained row families.
<!-- OUTLINE GAP: Finalize the row families from committed artifacts. Do not
     leave "other retained row families" in the final ToC. -->

### Columns: symplectic invariants and metadata

Define the columns at the level needed to understand the result:
symplectic quantities, geometric features, orbit data, and metadata.
<!-- OUTLINE GAP: Name the actual column groups that appear in the thesis table
     or appendix, especially which metadata columns are caveats rather than
     geometry. -->

### Result types

Use these result types in the main table or bullet list:

- inapplicable;
- failed;
- negative;
- positive.

Meaning:

- `inapplicable`: the method does not meaningfully apply to the available data
  or search question.
- `failed`: the method could not be implemented cheaply enough, scaled enough,
  or supplied with enough data for a thesis-facing result.
- `negative`: the method ran and did not produce a useful search rule or new
  transferable high-`sys` regime.
- `positive`: the method produced a useful signal or candidate. Use only where
  the evidence actually supports this.
<!-- TOC DECISION: If a real positive result appears, report it honestly and
     escalate to Jorn. It may falsify the current "insufficient" main result
     and justify spending about another week to follow up toward a better
     result. Also filter obvious false positives, e.g. a model finding signal
     because `sys` was regressed against `sys`. -->

### Methods: data-science toolbox

Batch the attempted, failed, and inapplicable methods. Put detailed figures,
tables, parameters, and method-specific notes in the data-science appendix.
<!-- OUTLINE GAP: Name the method families in the main text: random sampling,
     ascent, continuation, regression, classification, clustering/PCA,
     omitted/deferred families, and any positive or inapplicable rows. -->

## Side Result: Products Of Rotated Regular Polygons

Purpose:
record the structured-family side result around products of rotated regular
polygons.

### Empirical curves

Present the empirical curves for tested regular polygon products.
<!-- OUTLINE GAP: Name the curve parameters: polygon pair, rotation angle,
     plotted quantity, sampling resolution, and which curve supports which
     statement. -->

### Formula for the pentagon product

State the formula for the pentagon product if the proof/CAS writeup is ready.
Otherwise state the current status honestly.
<!-- OUTLINE GAP: Decide the exact thesis claim: proven formula, Sage-verified
     formula, empirical curve with conjectural formula, or future-work status. -->

### Computation with SageMath

Explain the exact/SageMath computation that supports the pentagon-product
formula or status.
<!-- OUTLINE GAP: Name the Sage computation inputs, exact field, checked
     identities/inequalities, and where the script/certificate is cited. -->

## Side Result: Visualization In 3D

Purpose:
record visualization as mathematical exploration.

Writing note:
one section is enough. It may fit best in an appendix if the main text is too
crowded.

Content:

- What was visualized.
<!-- OUTLINE GAP: Name the actual objects: which polytopes, which orbits,
     which projections/3D renderings, and which generated figures are candidates. -->
- What the visualization was meant to reveal.
<!-- OUTLINE GAP: State the search question: e.g. geometric pattern in Reeb
     dynamics, visible obstruction, cluster of high-sys cases, or sanity check. -->
- What was not found.
<!-- OUTLINE GAP: Phrase the negative result precisely enough: no useful visual
     search rule, no interpretable pattern, or no thesis-facing figure beyond
     illustration. -->
- Which figures are worth showing.
<!-- OUTLINE GAP: Decide main text vs appendix before figure polishing. -->

## Numerics

Purpose:
give the high-level numerical reliability story.

Kai preference:
Numerics is interesting for about one high-level paragraph in the main text,
not more. Detailed proofs and intermediate bounds belong in the appendix.

Content:

- Exact algebraic fallback.
- Empirical error measurements.
- Proven error bounds.

Writing note:
do not mix numerical-analysis language into symplectic definitions or exact
algorithm proofs. Treat numerics as a later support layer after the exact
mathematical computation story.

## Published Code And Data

Purpose:
state what code and data are published and how they support reproducibility.

Content:

- Repository structure at a high level.
<!-- OUTLINE GAP: Name the exact promises: source availability only, archived
     outputs, fresh-clone commands, or reproducible pipeline. -->
- Which experiment artifacts support thesis claims.
<!-- OUTLINE GAP: Link each retained claim to artifact families; avoid a broad
     repository tour. -->
- Which commands or archived outputs are promised.
<!-- OUTLINE GAP: Decide whether commands are smoke checks, full reruns, or
     provenance pointers that are not expected to be rerun before reading. -->

## Use Of AI

Purpose:
explain the role of AI tools honestly and concretely.

Content:

- Literature search.
- Proofs and formalization.
- Coding.
- Applied data science.
- Applied numerics.
- Writing.
- Project management.
<!-- OUTLINE GAP: For each category, decide whether to state only "AI helped"
     or give one concrete example and one limitation. Keep this factual and
     short unless Kai/Jorn want a fuller reflection. -->

Writing note:
keep this factual. Do not let it become the main thesis story unless there is a
specific reader-facing reason.

## Conclusion

Purpose:
summarize what the thesis establishes and what remains open.

Content:

- HKO2024 local maximality among ten-facet polytopes up to symmetry.
- The negative result for black-box data-science and related search tools.
- Algorithmic contributions.
- Side results.
- Future work.

## Bibliography

Use the normal thesis bibliography.

## Appendices

Use appendices for material that is needed for correctness or reproducibility
but interrupts the main reading path.

### Appendix: Data-Science Experiment Results

Content:

- Detailed method results.
- Figures and tables.
- Parameter choices and knobs.
- Method-specific caveats.

### Appendix: Numerics Proofs And Intermediate Bounds

Content:

- Exact algebraic fallback details.
- Empirical error-measurement details.
- Proven error bounds.
- Intermediate inequalities and constants.

### Appendix: Computations With SageMath

Content:

- Main result: HKO local maximum.
- Side result: products of regular pentagons.
- Algorithm based on CH2021.

## Section Test

For a prospective section or subsection, first try the four-line version:

1. Claim.
2. Support.
3. Caveat.
4. Pointer to proof, artifact, or appendix.

This is a check, not a rule for every section. If a section is not naturally
organized around one claim, say how to plan it instead. For example, notation,
reader orientation, summary tables, soft intuition, and appendix reference
material may need a purpose/dependency/placement check rather than a
claim/support/caveat/pointer check.

Only polish notation after this exists.
