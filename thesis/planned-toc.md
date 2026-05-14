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
  `tasks/*.md`, `tasks/MAP.md`, or the relevant research note instead.

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
- validity checks for a proposed finite dual-row set:
  `{a_k}` must be exactly the extremal points of `conv {a_k}`, and
  the intersection `{x : <a_k,x> <= 1}` is bounded exactly when
  `0 in int conv {a_k}`.
- algorithmic construction of the primal vertex set from valid dual vertices.
  Incidence of a primal vertex with a facet is then the defining equality
  `<a_i,x> = 1`; precomputing incidence is useful for later algorithms but is
  not a separate mathematical assumption.
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

## Quadratic Program Algorithm Based On HK2019

Purpose:
present the first finite algorithmic formulation for the minimizer search.
<!-- NAMING DECISION: At first use, call this "Quadratic Program Algorithm
     Based On HK2019". After first use, drop "Based On HK2019" and use the
     algorithmic name. The bibliography currently also has an `HK2017` entry
     for the same title; the journal-version GAFA citation is 2019 and should
     be preferred in reader-facing naming unless a writer chooses otherwise. -->

### Definition

Define the HK2017 finite optimization problem, variables, constraints,
objective, and reconstruction of the orbit.
<!-- OUTLINE GAP: Spell out the problem in the ToC: sigma/orbit word, beta
     constraints, quadratic objective including normalization/factor convention,
     admissibility conditions, and orbit reconstruction formula. -->
The input contract should be explicit: the dual rows are already checked to be
extremal and bounded in the sense of the polytope preliminaries. The capacity
algorithm may use the computed primal vertex set and vertex--facet incidence
for feasibility and pruning, but mathematical correctness is the HK2019
finite-search statement plus the preceding validity checks.

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
Separate correctness from implementation speed: incidence tables, cached
symplectic products, ordered candidate generation, tracing, profiling, and
benchmarking explain why the experiments can be run, not why the theorem is
true.

### Empirical tests

State the tests that support the implementation. Put detailed test outputs and
knobs outside the main reading path.
<!-- OUTLINE GAP: Name the test families and what each is supposed to catch:
     known polytopes, HK2017-vs-CH2021 comparison, exact/f64 spot checks,
     orbit recovery, and regression tests for past bugs. -->
State the testing philosophy once: whenever a believed statement is precise
enough to be formalized and cheap enough to exercise, write it as an automated
test. Typical patterns are known examples, finite sampled families, randomized
polytopes satisfying a predicate, exact-vs-f64 comparisons, and regression
tests for previously found bugs.

## Flow-Graph Algorithm Based On CH2021

Purpose:
present the second minimizer-search algorithm. This is expected to substitute
for HK2017 in practical minimizer search once finished.
<!-- NAMING DECISION: At first use, call this "Flow-Graph Algorithm Based On
     CH2021". After first use, drop "Based On CH2021" and use the algorithmic
     name. The separate tube picture may still be useful in the definition,
     because it describes the objects operated on, but it does not by itself
     say that the algorithm intersects tubes or chooses which tubes to build. -->
<!-- WRITER-SESSION QUESTION: After the flow-graph/tube branch lands, decide the
     final name, what tube objects appear in the exposition, and which proof
     and test claims are included. -->

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
<!-- WRITER-SESSION QUESTION: Decide while writing the chapter which
     non-generic cases matter for HKO2024. Do not attempt to cover every
     a-priori possible degeneracy unless it appears in, or is needed for, the
     HKO argument. -->

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

Define the decision problem and explain the SageMath computation that decides
it.

Content:

- Define the decision problem for local maximality among ten-facet polytopes up
  to symmetry.
- Define the chained computational steps/subroutines that decide the problem.
- Prove each step correct as it is introduced.
- Mention early that the exact field is `Q[tan(pi/5)] = Q[t]/p(t)` and give the
  polynomial `p(t)`. SageMath may use `QQbar`, which is a larger exact field;
  that is acceptable because performance is not the bottleneck here.
- Include selected Sage/Python code snippets where they make the computation
  more checkable or concrete.
- Include selected printed data snippets where they help sanity-check the
  computation, for example the first few and last few lines of a long printed
  sigma list.
- State the active orbit/row set, active-gradient rank/kernel comparison, and
  equality of flat first-order directions with the symmetry tangent space.

Writing note:
do not write pseudocode by default. Prefer a chain of defined subroutines or
checks, each with its own meaning and correctness explanation. Full code and
full data live in the repo. Passages of code and data may go in the main text
when they are logically important or useful as sanity checks. Kai should be able
to treat code snippets visually if he does not want to read them.
<!-- WRITER-SESSION QUESTION: Decide while writing how much code/data belongs in
     main text versus appendix versus repo-only. Current leaning: full code and
     full data in the repo; important code/data passages in main text; appendix
     only if it improves readability beyond what selected passages already do. -->

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
<!-- TOC DECISION: Rows are polytopes. Most rows come from random families and
     variants of gradient ascent applied to random polytopes; some may come
     from enumeration instead of randomization. Avoid mixing the non-black-box
     known HKO2024 `n=1` positive sample into the black-box data-science table,
     because methods can then memorize that one case without teaching us
     anything new. -->
<!-- WRITER-SESSION QUESTION: Finalize row families only when the data-science
     writer session starts or the branch stabilizes. The set is still dynamic. -->

### Columns: symplectic invariants and metadata

Define the columns at the level needed to understand the result:
symplectic quantities, geometric features, orbit data, and metadata.
<!-- TOC DECISION: Columns are features of the polytopes. They include
     symplectic invariants, geometric/orbit features, and metadata. Metadata
     columns are useful caveats because a method may learn data provenance
     rather than geometry. -->

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
<!-- TOC DECISION: Methods come from the data-science toolboxes we actually use
     and assign to Codex agents. The full set is still dynamic, so the final
     method list is needed before writing the table, not for the current ToC. -->
<!-- WRITER-SESSION QUESTION: Finalize the method list during the data-science
     chapter writer session. For now, keep the ToC generic. -->

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
<!-- TOC DECISION: Final structure: state the formula; define the computable
     decision problem; prove that the decision problem is equivalent to "the
     formula is right"; explain how each step was done in SageMath with
     intermediate hand sanity checks; report the final `True` printed by
     SageMath. One step is also checked by hand: the formula computes
     `sys_sigma` for the chosen sigma. The minimum over all sigma is certified
     by the decision problem, not by a full hand proof. -->
<!-- WRITER-SESSION QUESTION: Set final wording after the SageMath/proof text is
     ready. The structure is settled; exact claim phrasing can wait. -->

### Computation with SageMath

Explain the exact/SageMath computation that supports the pentagon-product
formula or status.
<!-- TOC DECISION: The SageMath subsection owns the exact inputs, checked
     identities/inequalities, intermediate sanity checks, and final Boolean
     result for the pentagon-product decision problem. -->

## Side Result: Visualization In 3D

Purpose:
record visualization as mathematical exploration.

Writing note:
one top-level chapter is acceptable even if it is short: roughly half a page of
text plus one page of figures. It is unusual, but the material does not
naturally belong elsewhere. Some figures may also be used elsewhere for
explanation, but most explanatory illustrations should be hand-drawn sketches.

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
<!-- WRITER-SESSION QUESTION: Choose the figure set during the visualization
     writer session. This should not block the global ToC. -->

## Numerics

Purpose:
give the high-level numerical reliability story.

Kai preference:
Numerics is interesting for about one high-level paragraph in the main text,
not more. Detailed proofs and intermediate bounds belong in the appendix.

Content:

- Exact arithmetic path: rational or algebraic data is used to implement the
  mathematically meaningful helper operations slowly but without numerical
  error. These helpers are separated so that later computations can reuse them
  instead of re-encoding the same mathematics.
- Floating-point fast path: the same mathematical algorithms are mapped to
  `f64` linear algebra where this is practical, but discontinuous predicates
  are treated as trinary `true`, `false`, or `indeterminate` decisions with
  error margins.
- Error handling boundary: `indeterminate` means the numerical evidence is not
  strong enough to decide the mathematical predicate. This is distinct from
  invalid input errors and from unrecovered assertion failures.
- Logical use of indeterminate values: use cancellations such as
  `false and indeterminate = false`, and simplify searches only from decided
  values. Do not claim a relational abstract interpreter; relations such as
  two individually indeterminate predicates whose disjunction is forced true
  are outside the current method unless a retained proof adds them.
- Empirical error measurements and exact comparisons, especially where the f64
  path is used to rerun experiments.
- Proven error bounds only at the strength needed by retained thesis claims.

Writing note:
do not mix numerical-analysis language into symplectic definitions or exact
algorithm proofs. Treat numerics as a later support layer after the exact
mathematical computation story. The current goal is trusted enough numerics to
rerun the experiments properly; stronger numerical certification should be
written and implemented only when the settled thesis text needs it.

## Published Code And Data

Purpose:
state what code and data are published and how they support reproducibility.

Content:

- Repository structure at a high level.
<!-- TOC DECISION: State the live GitHub repository, with the caveat that it may
     retire in a few years, and the permanent uploads on the chosen archive
     sites once known. -->
- Which experiment artifacts support thesis claims.
<!-- TOC DECISION: State that data is committed, the git history is not pruned
     and covers roughly half the thesis lifetime, the thesis PDF is rebuildable,
     and documentation explains how to read and run the repo. -->
- Which commands or archived outputs are promised.
<!-- TOC DECISION: The thesis should not become the run manual. "How to read
     this" and "how to run this" live in the repo. The repo promises
     reproducibility via Docker. -->
- Maintenance philosophy: code clarity wins by default; optimize only when
  tracing, profiling, benchmarking, or final consumers show that performance is
  material for a retained thesis computation. Maintenance after writeup should
  repair thesis/code mismatches, missing tests, reproducibility gaps, and
  profiling evidence that matters for the final experiments.
<!-- WRITER-SESSION QUESTION: Fill archive-site names and exact reproducibility
     promise when submission/archive mechanics are known. -->

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
<!-- TOC DECISION: Decide later. A short version may be about four pages with a
     few statistical figures. A detailed version may be about twelve pages and
     discuss prompts plus how AI use changed over six months. The agent-log
     analysis has not been designed yet. -->
<!-- WRITER-SESSION QUESTION: Decide length and evidence after the agent-log
     analysis exists. -->

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
