<!--
Purpose: define "the thesis project is done" as a pedantic sufficiency tree.
Context: rewritten on 2026-04-16 after the first gate-style draft proved too
summary-like. This file now breaks parent nodes into children until leaf nodes
become directly observable, even when the resulting execution plan is slow or
repetitive.
-->

# Verification Gates

## Context And Goal

This file answers one question:

> What has to be true before Jorn can truthfully say that the thesis project is
> finished?

The target is thesis completion plus truthful thesis-facing repo claims. It is
not "a cheap automated verification suite" and it is not "the smallest set of
checks". Actual execution efficiency is irrelevant here.

This file is allowed to be pedantic, overlapping, repetitive, and expensive to
execute. The point is coverage, not convenience.

This file does not replace:

- `RESULTS.md`: what the thesis should claim, and with what strength.
- `TASKS.md`: ownership, sequencing, and deadline choices.
- `formal/`: theorem statements, proof sketches, labels, and math notes.
- `library/`: stable reusable algorithms.
- `experiments/`: evidence-producing protocols and generated artifacts.
- `thesis/`: the actual submitted text.

## Reading Rule

Each parent node claims that its listed children are jointly sufficient for the
parent.

If the children do not actually suffice, the parent is incomplete and needs
more children. Overlap between children is allowed. Redundancy is allowed.

This file uses statement form, not question form. The point is to state
conditions whose truth would suffice, not to invite vague yes/no answers.

Node IDs are stable and literal, for example `T2.4.3` for an internal node or
`T2.4.3.a` for a leaf. The first letter `T` means "thesis done tree". Children
refine their parent. Leaf suffixes do not need to stay contiguous after edits.

Leaf-node tags used in this file:

- `[observable]`: a leaf that can be answered directly by inspecting a concrete
  artifact or running a concrete command.
- `[observable] [Name]`: a leaf that asks the named human for a specific
  judgment that is narrow enough to execute literally.
- `[todo-gap]`: a missing child that still has to be written down before the
  parent can honestly be treated as covered.

Default fallback:

- If no finer decomposition is available, a parent may end in a specific
  `[observable] [Name]` leaf.
- Unspecific leaves such as "Jorn checks thesis" are too vague.
- Specific leaves such as "Jorn verifies proof correctness" or "Jorn compares
  interpretation with the cited data and confirms the argument is valid and
  complete enough for submission" are acceptable.

## Top Layer

### T0. The thesis project is finished.

Intended meaning: Jorn can submit the thesis and truthfully stand behind the
mathematical, empirical, and software-facing claims that the thesis makes, and
the remaining mechanical submission steps are already complete.

Sufficient children:

- T1. The thesis contains the thesis-facing result surface from `RESULTS.md`
  that is still intended for inclusion.
- T2. Every claim made by the thesis has support of the right type and
  strength.
- T3. The thesis is understandable enough for its intended audience.
- T4. Every thesis-facing reference to proofs, code, data, experiments,
  algorithms, figures, and tables resolves to inspectable sources.
- T5. The repo artifact matches the promises that the thesis makes about it.
- T6. Work that is not finished is explicitly cut from the claim surface or
  labeled as future work or caveated open work.
- T7. The thesis project has been submitted and closed mechanically.
- T0.gap.a [todo-gap] Is any top-layer aspect of "the thesis project is done"
  still missing from T1-T7?

## T1 Tree

### T1. The thesis contains the thesis-facing result surface from `RESULTS.md`
that is still intended for inclusion.

Sufficient children:

- T1.1. Every main-result block in `RESULTS.md` that is still intended for
  inclusion is in the thesis.
- T1.2. Every supporting item in `RESULTS.md` that a retained thesis claim
  depends on is in the thesis.
- T1.3. Every non-required item in `RESULTS.md` that Jorn still intends to
  include is in the thesis.

### T1.1. Every main-result block in `RESULTS.md` that is still intended for
inclusion is in the thesis.

Sufficient children:

- T1.1.a [observable] For each `[main result]` entry in `RESULTS.md` that Jorn
  still intends to include, the final thesis contains corresponding thesis
  material.
- T1.1.b [observable] [Jorn] Jorn says no intended `[main result]` entry from
  `RESULTS.md` is missing and no included entry is only a placeholder.

### T1.2. Every supporting item in `RESULTS.md` that a retained thesis claim
depends on is in the thesis.

Sufficient children:

- T1.2.a [observable] For each retained thesis-facing claim in `RESULTS.md`,
  any supporting theorem statement, experiment description, figure, table,
  appendix, or explanation that the thesis text still depends on appears in the
  thesis or appendices.
- T1.2.b [observable] [Jorn] Jorn says no retained `RESULTS.md` item depends
  on support material that is missing from the thesis artifact.

### T1.3. Every non-required item in `RESULTS.md` that Jorn still intends to
include is in the thesis.

Sufficient children:

- T1.3.a [observable] For each non-main-result item in `RESULTS.md` that Jorn
  still intends to include, corresponding thesis material exists.
- T1.3.b [observable] [Jorn] Jorn says no optional `RESULTS.md` item that he
  still intends to include is missing.

## T2 Tree

### T2. Every claim made by the thesis has support of the right type and
strength.

Sufficient children:

- T2.1. HKO2024 local-maximality claims are licensed.
- T2.2. Hostile sys landscape claims are licensed.
- T2.3. Standalone result claims are licensed.
- T2.4. Method and algorithm claims are licensed.
- T2.5. Experiment and data support used by the thesis is interpreted
  correctly.
- T2.6. Global thesis wording does not outrun support.

### T2.1. HKO2024 local-maximality claims are licensed.

Sufficient children:

- T2.1.1. The HKO2024 statement uses the right quotient claim.
- T2.1.2. The first-order certificate or weaker replacement matches the claim.
- T2.1.3. The thesis-facing proof and evidence route for local maximality is
  coherent.
- T2.1.4. Empirical HKO stress-test wording matches available data.
- T2.1.5. HKO figures and tables support the text that cites them.

### T2.1.1. The HKO2024 statement uses the right quotient claim.

Sufficient children:

- T2.1.1.a [observable] The thesis states local maximality modulo
  translations, scaling, and symplectic linear maps.
- T2.1.1.b [observable] The thesis does not claim strict local maximality in
  raw `R^40`.
- T2.1.1.c [observable] The thesis statement matches the claim strength
  recorded in `RESULTS.md`.

### T2.1.2. The first-order certificate or weaker replacement matches the claim.

Sufficient children:

- T2.1.2.a [observable] An exact `Q(sqrt(5))` certificate exists for the
  15-dimensional flat-space / symmetry-space comparison, or the theorem
  statement is weakened so it no longer needs that certificate.
- T2.1.2.b [observable] [Jorn] Jorn says the available first-order
  certificate or weaker replacement licenses the exact thesis statement.

### T2.1.3. The thesis-facing proof and evidence route for local maximality is
coherent.

Sufficient children:

- T2.1.3.a [observable] The symmetry-space proof is written, or the thesis
  openly presents the result as weaker than a full proved theorem.
- T2.1.3.b [observable] If the second-order route remains in the thesis, the
  thesis says how it relates to the final argument; if it does not remain, it
  is omitted or demoted to supporting evidence.
- T2.1.3.c [observable] [Jorn] Jorn verifies proof correctness of the
  thesis-facing local-maximality argument as stated.

### T2.1.4. Empirical HKO stress-test wording matches available data.

Sufficient children:

- T2.1.4.a [observable] If the LICCA-scale F=10 perturbation run is cited, the
  returned artifacts exist and are integrated into the thesis-facing wording.
- T2.1.4.b [observable] If the LICCA-scale F=10 perturbation run is not
  integrated, the thesis cites only the smaller existing evidence and labels
  the larger run as pending/future or omits it.
- T2.1.4.c [observable] Higher-F validation appears only as future work unless
  supporting evidence is included.

### T2.1.5. HKO figures and tables support the text that cites them.

Sufficient children:

- T2.1.5.a [observable] Every HKO figure or table cited in support of the
  local-maximality story matches its generating artifact or preserved source.
- T2.1.5.b [observable] [Jorn] Jorn says the HKO interpretation in the thesis
  matches the cited figures, tables, and data.

### T2.2. Hostile sys landscape claims are licensed.

Sufficient children:

- T2.2.1. Baseline negative-search evidence is licensed.
- T2.2.2. Large-scale ascent-density wording is licensed.
- T2.2.3. Feature-regression or pattern-search wording is licensed.
- T2.2.4. Mechanism claims about cells, boundaries, and active orbits are
  licensed.
- T2.2.5. The hostile-landscape headline is worded as an empirical conclusion
  with stated limits.

### T2.2.1. Baseline negative-search evidence is licensed.

Sufficient children:

- T2.2.1.a [observable] Every baseline family cited in the hostile-landscape
  story (random generic polytopes, random products, rotated regular products,
  fixed-F ascent, variable-F ascent) has cited data or artifacts in the repo,
  or is removed from the claim surface.
- T2.2.1.b [observable] [Jorn] Jorn says the baseline evidence cited is enough
  for the exact baseline negative-search sentences that remain in the thesis.

### T2.2.2. Large-scale ascent-density wording is licensed.

Sufficient children:

- T2.2.2.a [observable] If LICCA-scale ascent data is cited, the returned
  artifacts exist and are integrated.
- T2.2.2.b [observable] If LICCA-scale ascent data is not cited, the thesis
  avoids strong density claims and states the current seed-limit caveat.

### T2.2.3. Feature-regression or pattern-search wording is licensed.

Sufficient children:

- T2.2.3.a [observable] If regression or classifier results are cited, the
  corresponding artifacts exist.
- T2.2.3.b [observable] If no such results are cited, the thesis does not
  claim that no learnable structure exists.

### T2.2.4. Mechanism claims about cells, boundaries, and active orbits are
licensed.

Sufficient children:

- T2.2.4.a [observable] Claims about narrow cells, frequent boundary
  crossings, continuity of sys, and orbit or gradient changes cite concrete
  experiments or are weakened.
- T2.2.4.b [observable] No all-minimum-orbit claim in this part of the thesis
  is supported only by a one-sigma cache.

### T2.2.5. The hostile-landscape headline is worded as an empirical conclusion
with stated limits.

Sufficient children:

- T2.2.5.a [observable] The abstract, introduction, experiment chapter, and
  conclusion use empirical language for the hostile-landscape result.
- T2.2.5.b [observable] The thesis states the main limits that remain
  relevant: random-model dependence, seed-count limits, and the absence of a
  proof that high-sys examples do not exist.
- T2.2.5.c [observable] [Jorn] Jorn says the hostile-landscape interpretation
  does not outrun the evidence.

### T2.3. Standalone result claims are licensed.

Sufficient children:

- T2.3.1. The crosspolytope claim is licensed.
- T2.3.2. The visualization claim is licensed.
- T2.3.3. The pentagon-rotation formula claim is licensed or omitted/future.

### T2.3.1. The crosspolytope claim is licensed.

Sufficient children:

- T2.3.1.a [observable] If the crosspolytope result is included, the thesis
  states `c_EHZ = 4` and `sys = 3/4` with the
  exhaustive-through-orbit-length-13 caveat.
- T2.3.1.b [observable] If timing is quoted, the quoted timing matches the
  cited artifact or is labeled historical console output.

### T2.3.2. The visualization claim is licensed.

Sufficient children:

- T2.3.2.a [observable] Visualization is presented as a negative exploration
  or communication artifact, not as proof of a theorem or algorithm guarantee.
- T2.3.2.b [observable] Any included visualization figure has a concrete
  source in the repo.

### T2.3.3. The pentagon-rotation formula claim is licensed or omitted/future.

Sufficient children:

- T2.3.3.a [observable] If a pentagon-rotation formula is claimed, a written
  proof or source exists.
- T2.3.3.b [observable] If no proof or source exists, the formula appears only
  as future work or is omitted.

### T2.4. Method and algorithm claims are licensed.

Sufficient children:

- T2.4.1. The EHZ capacity-method correctness story is licensed.
- T2.4.2. Rich minimum-orbit claims are licensed.
- T2.4.3. Numerical error-bound claims are licensed.
- T2.4.4. The projection-solver story matches the current code.
- T2.4.5. Tube-algorithm claims are licensed or removed.
- T2.4.6. Billiard and product-method claims are scoped.
- T2.4.7. Derivative-based claims are licensed.
- T2.4.8. Algorithm-surface authority is explicit.

### T2.4.1. The EHZ capacity-method correctness story is licensed.

Sufficient children:

- T2.4.1.a [observable] The thesis distinguishes the idealized capacity
  algorithm, implementation choices, pruning, numerical caveats, and empirical
  validation.
- T2.4.1.b [observable] The thesis does not use unresolved proof TODOs as
  silent support for a full correctness claim.
- T2.4.1.c [observable] [Jorn] Jorn says the thesis-facing EHZ correctness
  story is honest at the stated claim strength.

### T2.4.2. Rich minimum-orbit claims are licensed.

Sufficient children:

- T2.4.2.a [observable] If the thesis claims a reusable method returning all
  simple minimum-action orbits, the source of truth for that method is named
  and exists.
- T2.4.2.b [observable] If the thesis only claims experiment-level
  verification of all simple minimum-action orbits, the thesis says that and
  does not silently upgrade it to a library promise.
- T2.4.2.c [observable] Any claim about all minimum simple orbits is backed by
  a path that recomputes all tied or near-tied minimum orbits, not only one
  best sigma.

### T2.4.3. Numerical error-bound claims are licensed.

Sufficient children:

- T2.4.3.a [observable] The thesis states which numerical quantities have
  analytic error bounds, which have empirical cross-checks, and which depend
  on exact fallback or high-precision comparison.
- T2.4.3.b [observable] The thesis does not rely on stale solver-contract
  comments or stale numerical notes as if they were final support.

### T2.4.4. The projection-solver story matches the current code.

Sufficient children:

- T2.4.4.a [observable] If the projection solver is discussed, the thesis or
  formal notes describe the current solver path rather than stale
  experiment-local copies.

### T2.4.5. Tube-algorithm claims are licensed or removed.

Sufficient children:

- T2.4.5.a [observable] If the tube algorithm is used in the thesis-facing
  method story, the correct rotation formula and supporting text exist.
- T2.4.5.b [observable] If that formula or support does not exist, tube claims
  are cut, demoted, or clearly labeled experimental or historical.

### T2.4.6. Billiard and product-method claims are scoped.

Sufficient children:

- T2.4.6.a [observable] If billiard or product methods are discussed, the
  thesis says exactly which parts are proved, implemented, benchmarked, or used
  only as comparison methods.

### T2.4.7. Derivative-based claims are licensed.

Sufficient children:

- T2.4.7.a [observable] If derivative formulas are used as theorem-level
  support, a reviewed proof source exists.
- T2.4.7.b [observable] If no reviewed proof source exists, derivative-based
  plots are described as empirical diagnostics rather than theorem-level
  support.

### T2.4.8. Algorithm-surface authority is explicit.

Sufficient children:

- T2.4.8.a [observable] Every thesis-presented algorithm is clearly identified
  as a library API, experiment protocol, instrumented variant, or future work.

### T2.5. Experiment and data support used by the thesis is interpreted
correctly.

Sufficient children:

- T2.5.1. Every thesis-facing experiment has a named source of truth.
- T2.5.2. Every thesis-facing dataset has wording that matches its freshness
  status.
- T2.5.3. Caches are not overread.
- T2.5.4. Intermediate numerical data and search traces are not misinterpreted.
- T2.5.5. LICCA artifacts cited in support are either present or marked
  pending/future.

### T2.5.1. Every thesis-facing experiment has a named source of truth.

Sufficient children:

- T2.5.1.a [observable] For each thesis-facing experiment, the source of truth
  is named: generator or protocol, preserved artifact, recomputation command,
  or explicitly external artifact.

### T2.5.2. Every thesis-facing dataset has wording that matches its freshness
status.

Sufficient children:

- T2.5.2.a [observable] For each thesis-facing dataset, the thesis wording
  matches whether the dataset is current enough, rerun, or historical.
- T2.5.2.b [observable] No stale dataset is used without either rerun,
  historical labeling, or weaker wording.

### T2.5.3. Caches are not overread.

Sufficient children:

- T2.5.3.a [observable] One-sigma caches are used only for minimum-action or
  best-sigma claims, not for claims about all tied or near-minimum sigmas.
- T2.5.3.b [observable] The primitive source or preserved-artifact role of the
  shared 170-row catalog is stated somewhere the thesis-support story depends
  on it.

### T2.5.4. Intermediate numerical data and search traces are not misinterpreted.

Sufficient children:

- T2.5.4.a [observable] Data with per-`(S, sigma)` intermediate matrices,
  solver verdicts, or timings is not presented as a reusable polytope catalog.
- T2.5.4.b [observable] Search traces are cited as search-process evidence;
  final-body claims cite extracted final artifacts instead.

### T2.5.5. LICCA artifacts cited in support are either present or marked
pending/future.

Sufficient children:

- T2.5.5.a [observable] Any LICCA artifact used as support is present and
  identified.
- T2.5.5.b [observable] Any LICCA work not present is marked pending/future or
  omitted from the thesis-facing support story.

### T2.6. Global thesis wording does not outrun support.

Sufficient children:

- T2.6.a [observable] Every claim made by the thesis names or points to the
  support that is supposed to justify it.
- T2.6.b [observable] [Jorn] Jorn checks, claim by claim, that the support
  cited for each claim is enough for that exact claim as stated.
- T2.6.c [observable] [Jorn] Jorn checks that no claim is stated more strongly
  than its support licenses.

## T3 Tree

### T3. The thesis is understandable enough for its intended audience.

Sufficient children:

- T3.1. The mathematical part is understandable enough for the mathematical
  audience.
- T3.2. The computational and numerical part is understandable enough for the
  computational audience.
- T3.3. The implementation-facing part is understandable enough for a
  technically literate reader.
- T3.4. The thesis structure makes the role of each part clear.

### T3.1. The mathematical part is understandable enough for the mathematical
audience.

Sufficient children:

- T3.1.a [observable] [Kai] Kai reads the mathematical part and says it is
  clear enough to follow the definitions, theorem statements, and argument
  structure.
- T3.1.gap.a [todo-gap] If Kai is not the intended checker here, name the
  replacement checker explicitly.

### T3.2. The computational and numerical part is understandable enough for the
computational audience.

Sufficient children:

- T3.2.a [observable] [Elizabeth] Elizabeth reads the computational,
  numerical, and optimization part and says it is clear enough to follow the
  method and experiment logic.
- T3.2.gap.a [todo-gap] If Elizabeth is not the intended checker here, name
  the replacement checker explicitly.

### T3.3. The implementation-facing part is understandable enough for a
technically literate reader.

Sufficient children:

- T3.3.a [observable] [Jorn] Jorn says the implementation-facing explanations
  are clear enough that a technically literate reader can follow what the code
  and experiments do.

### T3.4. The thesis structure makes the role of each part clear.

Sufficient children:

- T3.4.a [observable] [Jorn] Jorn says the thesis structure makes clear how
  the background, methods, experiments, and conclusion support the two main
  result blocks.

## T4 Tree

### T4. Every thesis-facing reference to proofs, code, data, experiments,
algorithms, figures, and tables resolves to inspectable sources.

Sufficient children:

- T4.1. Bibliographic citations resolve.
- T4.2. Internal thesis cross-references resolve.
- T4.3. Theorem, definition, and proof-source references resolve.
- T4.4. Figure and table provenance resolves.
- T4.5. Experiment, dataset, code, and result-artifact references resolve.

### T4.1. Bibliographic citations resolve.

Sufficient children:

- T4.1.a [observable] Every citation key used in the thesis resolves to a
  bibliography entry.

### T4.2. Internal thesis cross-references resolve.

Sufficient children:

- T4.2.a [observable] Every section, equation, figure, and table reference in
  the thesis resolves, or the broken reference is removed before submission.

### T4.3. Theorem, definition, and proof-source references resolve.

Sufficient children:

- T4.3.a [observable] Every theorem, lemma, definition, or proof-source
  reference used by the thesis points to an existing labeled target, or the
  broken dependency is removed before submission.

### T4.4. Figure and table provenance resolves.

Sufficient children:

- T4.4.a [observable] Every figure and table included in the thesis has a
  concrete source: generated artifact, source dataset plus analysis script, or
  hand-drawn source.

### T4.5. Experiment, dataset, code, and result-artifact references resolve.

Sufficient children:

- T4.5.a [observable] Every experiment, dataset, code path, result artifact,
  or external resource named in the thesis exists in the repo or is explicitly
  marked external or not included.

## T5 Tree

### T5. The repo artifact matches the promises that the thesis makes about it.

Sufficient children:

- T5.1. Library-facing promises are truthful.
- T5.2. Experiment-pipeline promises are truthful.
- T5.3. Reproducibility and build promises are truthful.
- T5.4. Repo-internal semantics do not contradict thesis promises.
- T5.5. Math-code correspondence promised by the repo is truthful.

### T5.1. Library-facing promises are truthful.

Sufficient children:

- T5.1.a [observable] If the thesis says the project provides a Rust library,
  `library/` exists.
- T5.1.b [observable] Thesis-facing library claims match the APIs, modules, or
  explicitly named internal code paths that actually exist.

### T5.2. Experiment-pipeline promises are truthful.

Sufficient children:

- T5.2.a [observable] If the thesis says the project provides a reproducible
  experiment pipeline, `experiments/` contains the thesis-facing experiment
  packages, scripts, and outputs or preserved artifacts that the thesis relies
  on.
- T5.2.b [observable] Each thesis-facing experiment has either a rerunnable
  verification path or an explicitly named preserved artifact that the thesis
  treats as the computational record.

### T5.3. Reproducibility and build promises are truthful.

Sufficient children:

- T5.3.a [observable] Any build, test, smoke, regeneration, or formal-build
  promise that the thesis makes is backed by a concrete command or by explicit
  artifact-based wording instead.
- T5.3.b [observable] If the thesis promises library tests, experiment smoke
  runs, thesis builds, or formal builds, the corresponding command set is
  named and the promised scope matches reality.

### T5.4. Repo-internal semantics do not contradict thesis promises.

Sufficient children:

- T5.4.a [observable] No thesis claim silently depends on a cache, helper, or
  internal routine whose actual semantics are weaker than the thesis wording.
- T5.4.b [observable] Search traces, intermediate numerical data, and
  one-sigma caches are not used as if they proved stronger facts than they
  actually support.

### T5.5. Math-code correspondence promised by the repo is truthful.

Sufficient children:

- T5.5.a [observable] Thesis-critical Rust comments with `[lem:...]`,
  `[thm:...]`, or `[def:...]` do not point to nonexistent or misleading proof
  sources.
- T5.5.b [observable] If a thesis-critical algorithm lacks the formal source
  its code comments suggest, the comments are relabeled or the formal source is
  added.

## T6 Tree

### T6. Work that is not finished is explicitly cut from the claim surface or
labeled as future work or caveated open work.

Sufficient children:

- T6.1. Unfinished main-result-adjacent work is not presented as finished.
- T6.2. Optional standalone results are integrated only if supported.
- T6.3. Optional method and process material is integrated only if supported.
- T6.4. No silent placeholder remains in the submitted claim surface.

### T6.1. Unfinished main-result-adjacent work is not presented as finished.

Sufficient children:

- T6.1.a [observable] Higher-F HKO validation, larger LICCA stress tests,
  regression-pattern-search work, and other still-open strengthening work
  appear only as future work, caveated open work, or are omitted unless their
  support is actually included.
- T6.1.b [observable] The thesis does not claim broad coverage of all
  remaining research surfaces.

### T6.2. Optional standalone results are integrated only if supported.

Sufficient children:

- T6.2.a [observable] Crosspolytope and visualization results are included
  only at the modest claim strength their support licenses.
- T6.2.b [observable] Pentagon-rotation formula results are included only if
  the proof exists; otherwise they are future work or omitted.

### T6.3. Optional method and process material is integrated only if supported.

Sufficient children:

- T6.3.a [observable] Tube algorithm, extra numerical formalization, AI or
  process reflection, and other optional method or process material are
  included only if their support exists; otherwise they are cut or labeled
  future or historical.

### T6.4. No silent placeholder remains in the submitted claim surface.

Sufficient children:

- T6.4.a [observable] No unresolved thesis TODO marker stands in for missing
  mathematical content, empirical content, explanation, or citation in the
  submitted thesis.
- T6.4.b [observable] [Jorn] Jorn checks that wording does not smuggle
  deferred work back in as if it were finished.

## T7 Tree

### T7. The thesis project has been submitted and closed mechanically.

Sufficient children:

- T7.1. The submitted thesis PDF is identified and tied to source.
- T7.2. University thesis requirements are satisfied.
- T7.3. The repo state accompanying the thesis is identified.
- T7.4. The deadline condition is satisfied.
- T7.5. All mechanical handin steps are complete.

### T7.1. The submitted thesis PDF is identified and tied to source.

Sufficient children:

- T7.1.a [observable] The exact final thesis PDF that was submitted is
  identified.
- T7.1.b [observable] `cd thesis && latexmk && ./check-build.sh` rebuilds the
  submitted PDF from the recorded source, or Jorn records why the submitted PDF
  is treated as the preserved final artifact.

### T7.2. University thesis requirements are satisfied.

Sufficient children:

- T7.2.a [observable] The submitted thesis contains the thesis-side material
  required by the university.
- T7.2.b [observable] Any required form that must live in the thesis artifact
  is present.
- T7.2.c [observable] Any required printed copies or print-ready files exist.
- T7.2.gap.a [todo-gap] Name the exact university-side components if they are
  not yet written down elsewhere.

### T7.3. The repo state accompanying the thesis is identified.

Sufficient children:

- T7.3.a [observable] The exact repo commit, tag, archive, or equivalent state
  accompanying the thesis is identified.
- T7.3.b [observable] LFS-tracked outputs and generated artifacts in that
  final state are intentional.

### T7.4. The deadline condition is satisfied.

Sufficient children:

- T7.4.a [observable] The thesis handin happened before the applicable
  deadline, or the extension terms that make it on-time are recorded.

### T7.5. All mechanical handin steps are complete.

Sufficient children:

- T7.5.a [observable] Upload, print, portal, and handin steps that apply are
  complete.
- T7.5.b [observable] [Jorn] Jorn says that no further mechanical handin step
  remains.

## TODO(gap) Index

- `T0.gap.a`: decide whether any top-layer aspect of "the thesis project is
  done" is still missing from T1-T7.
- `T3.1.gap.a`: if Kai is not the intended checker for T3.1, name the
  replacement checker explicitly.
- `T3.2.gap.a`: if Elizabeth is not the intended checker for T3.2, name the
  replacement checker explicitly.
- `T7.2.gap.a`: name the exact university-side components required for T7.2 if
  they are not written down elsewhere.
