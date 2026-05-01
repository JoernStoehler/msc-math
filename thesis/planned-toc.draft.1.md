# Planned Thesis TOC Draft 1

<!--
Disposable planning draft.

Purpose:
- Give Jorn a concrete best-guess finished-thesis shape before interactive TOC
  drafting starts.
- Separate main-body thesis content from appendices, evidence packets, and
  future/cut material.
- Make hidden dependencies visible early: theorem-strength gaps, unsupported
  prose, stale algorithm text, figure/table needs, and Jörn-only decisions.

Status:
- Agent best guess from repo maps as of 2026-04-25.
- Not a task tracker and not a source of truth.
- Expected to be deleted or replaced by `thesis/planned-toc.md`.

Main terminology:
- "Retained claim cluster" means a thesis-facing group of claims that should be
  readable as one reader story: the question, what was done, what was found,
  what strength the evidence has, and what remains outside scope.
- I avoid "story block" below except in comments because the phrase is too vague
  for the artifact we want.
-->

## Executive Guess

The finished thesis should probably be organized around two main retained claim
clusters:

1. HKO2024 is locally maximal in the relevant `M_10` model, modulo natural
   symmetries, with exact first-order proof if Packet 3 closes.
2. The search landscape is hostile: bounded random, local optimization,
   continuation, and standard data-science attempts did not find a second
   transferable `sys > 1` regime beyond the known pentagon-pentagon geometry.

The algorithm/proof/numerics material should support those clusters rather than
remain the top-level organizing principle. The current `thesis/main.tex` order
is algorithm-first and likely stale:

```text
Introduction
Algorithms
Tube Algorithm
Proofs
Computational experiments
Appendix: Numerical Implementation
Appendix: Notation Glossary
```

<!--
Why I think this should change:
- Kai accepted HKO local maximality and hostile landscape as sufficient thesis
  story blocks, according to the writing roadmap.
- The current experiments chapter is only a placeholder.
- Tube algorithm is blocked/future by default and should not be a full main
  chapter unless Jorn promotes it.
- Algorithms/proofs are important, but they are methods and foundations for the
  retained claims; they are not the final thesis question by themselves.
-->

## Proposed Main TOC

### 1. Introduction

One-sentence obligation: state Viterbo's conjecture, the HKO2024 counterexample,
the thesis question, and the two contributions: a local-maximality analysis of
HKO2024 and a bounded negative search-landscape study.

Likely contents:

- Problem statement: symplectic capacity, volume, systolic ratio `sys`.
- Viterbo's conjecture and the role of counterexamples.
- HKO2024 as the known `sys > 1` geometry.
- Thesis questions:
  - Is HKO2024 isolated/local-maximal in a meaningful finite model?
  - Do standard computational search methods find another transferable high-`sys`
    regime?
- Contributions:
  - Exact/symbolic first-order certificate route for HKO local maximality.
  - Empirical and data-science evidence for hostile search landscape.
  - Supporting implementation and numerical validation layer.
- Reading guide.

<!--
Confidence: high.

The intro should not be written first except as a scaffold, because exact HKO
claim strength and hostile-landscape wording still determine the precise
contribution sentences.
-->

### 2. Mathematical and Computational Preliminaries

One-sentence obligation: give the reader the definitions and algorithms needed
to understand `sys`, EHZ capacity on polytopes, and what the experiments compute.

Likely contents:

#### 2.1 Convex bodies, symplectic form, and systolic ratio

- Convex body / polytope conventions in `R^4`.
- Standard symplectic form and linear symplectic maps.
- EHZ capacity and `sys(K) = c_EHZ(K)^2 / (2 vol(K))` or the final chosen
  normalization.
- Lagrangian products and regular polygon products.

Source candidates:
- `thesis/basic-definitions.tex`
- `thesis/appendix-notation.tex`
- HKO paper references through current bibliography

#### 2.2 Reeb orbits and the finite polytope computation

- Explain the finite orbit model used by the algorithms.
- State what a candidate orbit/sigma word is.
- Explain action, capacity, and the role of minimizing actions.

Source candidates:
- `thesis/clarkedual-action-principle.tex`
- `thesis/simple-minimizer-existence.tex`
- `thesis/general-case-algorithm.tex`
- `thesis/lagrangian-product-algorithm.tex`

#### 2.3 Algorithms used in the thesis

- Present only the algorithms the reader needs for the HKO and landscape
  experiments.
- Include general polytope algorithm and Lagrangian-product specialization.
- Keep pruned variants if they are used in retained experiments or needed for
  reproducibility.
- Defer numerical tolerances and certification details to Appendix A.

Source candidates:
- `thesis/algorithms.tex`
- `thesis/general-case-algorithm.tex`
- `thesis/lagrangian-product-algorithm.tex`
- `thesis/pruned-general-case-algorithm.tex`

Open risks before writing:

- `thesis/migration-findings.md` rows 3-11 affect this chapter: KKT multiplier
  names, KKT matrix signs, Q factor, beta/eigen thresholds, accumulator
  references, `|S| >= 2`, and billiard adjacency pruning.
- Existing algorithm boxes may describe a stronger or different accumulator
  story than public wrappers expose by default.

<!--
Confidence: medium-high.

This chapter may be split into "Preliminaries" and "Algorithms" if it grows.
I would not put tube here unless it is promoted; tube currently looks like a
future algorithm note, not a thesis-spine method.
-->

### 3. Local Maximality of HKO2024

One-sentence obligation: prove or honestly state the strongest supported
local-maximality result for HKO2024 in `M_10`, modulo translations, scaling, and
linear symplectic maps.

Likely contents:

#### 3.1 The HKO2024 configuration

- Define the HKO pentagon-pentagon Lagrangian product in the thesis notation.
- State known `sys > 1` value and its role as the baseline counterexample.
- Explain the finite `M_10` setting.
- Fix notation for dual coordinates `a_i`; avoid old `(n,h)` gauge framing.

Source candidates:
- `research/hko-local-maximum.md`
- `research/hko-local-maximum-status.md`
- `thesis/lagrangian-product-algorithm-proof.tex`
- `papers/hko2024/counterexample.tex`

#### 3.2 Symmetries and the correct local-maximality statement

- State the natural symmetry directions: translations, scaling, and linear
  symplectic maps.
- Explain why the thesis must not claim strict local maximality in raw `R^40`.
- State the target theorem shape:
  "Clarke-flat first-order directions equal the 15-dimensional symmetry tangent
  space."

Dependency:
- Packet 3 exact certificate closure, or a weaker formulation.

#### 3.3 Exact first-order certificate

- Explain exact field: quartic `Q(tan(pi/5))`, not `Q(sqrt(5))`.
- Present Packet 1: exact geometry and symmetry tangent space.
- Present Packet 2: endpoint/midpoint prototypes and equality-case
  combinatorics.
- Present Packet 3: active-gradient matrix, rank `25`, kernel dimension `15`,
  kernel equals symmetry tangent space.

Current state:
- Packet 1 essentially closed.
- Packet 2 partially closed.
- Packet 3 not closed yet: two asymmetric seven-facet representative classes
  remain unresolved; current widened witness is real but partial.

Must not use stale text blindly:
- `formal/hko-local-maximum/gradient-analysis.tex` still says `44` orbits and
  `10` gradients.
- Current bookkeeping is `150` exact action orbits, `20` visited subsets, and
  `28` distinct height gradients.

#### 3.4 Supporting evidence around the exact result

- Second-order evidence: fixed-`F=10` curvature support.
- Perturbation/facet-splitting/cut-and-ascent/neighborhood evidence.
- Lagrangian-boundary local region if useful for context.
- Clearly separate theorem support from proof replacement.

Source candidates:
- `research/hko-local-maximum.md`
- `formal/hko-local-maximum/second-order.tex`
- `experiments/hko-local-maximum/`

Jörn decisions:

- If Packet 3 closes: exact theorem wording.
- Whether second-order material is supporting evidence, repaired proof route, or
  future/cut.
- Which empirical evidence deserves thesis space.

<!--
Confidence: high that HKO deserves a main chapter.
Confidence: medium on subchapter split until Packet 3 finishes.

If Packet 3 closes cleanly, this is probably the main theorem chapter. If it
does not, the chapter still exists but becomes "evidence and exact route toward
local maximality"; however Jorn currently expects Packet 3 to close before
writing this part.
-->

### 4. Search Methods and the Hostile Landscape

One-sentence obligation: show that a broad but bounded set of computational
search methods did not find a second transferable `sys > 1` regime, and state
exactly what that does and does not license.

Likely contents:

#### 4.1 Search question and claim boundary

- State the question: can standard computational methods find another high-`sys`
  regime beyond HKO/pentagon-pentagon?
- State the boundary:
  - bounded empirical search found no new `sys > 1` example beyond known
    pentagon-pentagon family;
  - local methods improve states but did not yield a transferable global-search
    heuristic;
  - seed counts are too small for a density or brute-force-impossibility claim.

Source candidates:
- `research/sys-landscape.md`
- `research/sys-landscape-toolbox-audit.md`
- `tasks/landscape.md`

#### 4.2 Baseline random and structured searches

- Random generic sampling: 70 rows, max `sys=0.739`, no `sys > 1`.
- Random Lagrangian-product sampling: 100 rows, max `sys=0.794`, no `sys > 1`.
- Rotated regular-product sweeps: pentagon-pentagon at `theta=18 deg`; no
  further tested regular-family violation known.
- Rejection/acceptance calibration only as support if needed.

Source candidates:
- `experiments/sys-landscape/random-sample/`
- `experiments/sys-landscape/random-product-sample/`
- `experiments/sys-landscape/rotated-regular-products/`
- `research/sys-landscape-toolbox-audit.md`

#### 4.3 Local optimization and continuation

- Fixed-`F` gradient ascent:
  - general ascent: 10 seeds, max `sys=0.9030`, no `sys > 1`;
  - product ascent: 12 seeds, max `sys=0.8727`, no `sys > 1`.
- Variable-`F` continuation:
  - 90 trials;
  - gains from `F=10` to `F=11` common but still below `1`.
- Interpret as local improvement without transferable global-search heuristic.

Source candidates:
- `experiments/sys-landscape/gradient-ascent-general/`
- `experiments/sys-landscape/gradient-ascent-products/`
- `experiments/sys-landscape/variable-f-ascent/`

#### 4.4 Data-science search attempts

- Feature-block regression: ridge and random forest over random and endpoint
  regimes.
- Transfer guard: grouped CV / lineage-grouped splits; random-to-endpoint
  transfer is the load-bearing test.
- Current interpretation: endpoint-side features useful within random data but
  weak for transfer to endpoints.
- Regime classification (`M012`) and residualized endpoint regression (`M013`)
  are present but thesis use undecided.

Source candidates:
- `research/sys-landscape-datascience/method-ledger.md`
- `research/sys-landscape-toolbox-audit.md`
- `experiments/sys-landscape/datascience/methods/feature-pattern-search/`

Jörn decisions:

- Whether `M012` regime classification is claim-bearing, caveat-only, or omitted.
- Whether `M013` residualized endpoint regression is claim-bearing,
  supporting-only, spike-only, or omitted.

#### 4.5 Skipped/deferred standard-toolbox families

- PCA/global dimensionality reduction.
- Clustering/manifold learning.
- SVM/boosting/nearest-neighbor methods.
- Neural-network methods.
- Bayesian optimization.

Purpose:
- Name these as omitted/deferred method families, not failed methods.
- Keep the thesis honest about the standard toolbox without opening a new
  methods program.

#### 4.6 Visual exploration and negative pattern discovery

- Optional subsection.
- Treat "looking at pictures" as mathematical exploration if included.
- It should either be standalone negative exploration or supporting material,
  not infrastructure.

Jörn decision:
- Include as standalone thesis material, supporting figure/commentary, or
  future/cut.

<!--
Confidence: high that landscape deserves a main chapter.
Confidence: medium on whether visualization belongs here, appendix, or omitted.

This chapter should avoid sounding like "we proved no other examples exist".
The correct thesis shape is bounded, method-by-method negative evidence.
-->

### 5. Validation, Reproducibility, and Evidence Boundaries

One-sentence obligation: explain why the computational results are credible
enough for the thesis, without pretending empirical validation is a theorem.

Likely contents:

#### 5.1 Implementation boundary

- State what code was used for capacity computation and experiment production.
- Explain that durable algorithms live in `crates/` and exploratory evidence in
  `experiments/`.
- State enough reproducibility commands or archive promises for thesis readers.

Source candidates:
- `crates/MAP.md`
- `experiments/MAP.md`
- `tasks/reproducibility.md`
- verification packet `repo-promises-are-truthful.md`

#### 5.2 Verification experiments

- Correctness package checks: conformality, symplectic invariance, monotonicity,
  continuity, literature agreement, pruned/unpruned/billiard agreement.
- All-minimum and orbit-recovery evidence:
  - 28 selected polytopes;
  - 469 trusted minima;
  - full reconstruction success for all 469.
- Treat this as validation evidence, not theorem-strength proof.

Source candidates:
- `research/verification.md`
- `experiments/verification/`

#### 5.3 Numerical caveats that affect retained claims

- Public `ehz_capacity*` wrappers call f64-only aggregation by default.
- Stronger exact/guaranteed aggregation exists as a non-default path.
- The thesis must be explicit whether it describes public wrappers, verification
  layer, or both.
- Formal numerics has exact per-sigma solver and trinary beta material, but also
  named gaps around near-threshold beta, empirical constants, and Taylor
  cancellation.

Source candidates:
- `tasks/numerics.md`
- `formal/numerics/error-bounds.tex`
- `experiments/numerics/q-error/q_error_output.txt`
- `thesis/appendix-numerical.tex`
- `crates/symplectic/src/lib.rs`
- `crates/symplectic/src/algorithms/orbit_search.rs`

<!--
Confidence: medium.

This could be a main-body chapter if the thesis needs to convince the reader
that the empirical story is reliable. It could also be split: short main-body
"Reliability of computations" section plus detailed numerical appendix. I lean
main-body summary + appendix details.
-->

### 6. Conclusion and Outlook

One-sentence obligation: restate what the thesis established, what it failed to
find, and what remains as publication/future work.

Likely contents:

- HKO local-maximality result at the final certified strength.
- Hostile-landscape bounded negative result.
- What this says about searching for Viterbo counterexamples.
- Future work:
  - publication-grade LICCA or higher-`F` HKO checks if not already integrated;
  - exact theorem polish if any part remains conditional;
  - witness-guided continuation / reduced-model search;
  - skipped data-science families if worth pursuing later;
  - tube algorithm if it remains outside thesis.

<!--
Confidence: high that this shape is needed, low on wording until claim strength
is frozen.
-->

## Proposed Appendices

### Appendix A. Numerical Implementation and Error Handling

One-sentence obligation: give the technical numerical details needed to interpret
the computation, with caveats visible.

Likely contents:

- Input representation.
- Transition feasibility and pruning.
- KKT system and numerical error.
- Accumulator and final answer.
- Explicit boundary between:
  - ordinary f64 wrappers;
  - stronger exact/guaranteed verification path;
  - unimplemented or future second-pass escape hatches.

Current risk:
- Existing `thesis/appendix-numerical.tex` is stale relative to current public
  wrappers.
- `thesis/appendix-rewrite-notes.md` says Jorn wanted a top-down rewrite with a
  new combinatorial-realization theorem; that theorem is not currently in the
  repo.

Draft recommendation:
- Keep a shorter, honest appendix unless Jorn supplies the new theorem.
- Do not make exact-combinatorial-realization claims by implication.

### Appendix B. Algorithm Proof Details

One-sentence obligation: hold long proofs of algorithm correctness and action
principle material if they distract from the main HKO/landscape flow.

Possible moved material:

- Clarke dual action principle.
- Simple minimizer existence.
- General-case algorithm proof.
- Pruned algorithm proof.
- Lagrangian product proof.

Tradeoff:
- If the thesis is mathematically proof-driven, some of this may remain in the
  main body.
- If the main body is organized around HKO and landscape, long foundational
  proofs should move here or be shortened.

### Appendix C. Additional Experiment Tables and Provenance

One-sentence obligation: provide reproducibility details for datasets, tables,
and figures cited in the main text.

Possible contents:

- Dataset/artifact provenance for HKO evidence.
- Dataset/artifact provenance for landscape searches.
- Verification command summaries.
- Additional tables that would interrupt the main story.

Important:
- Only include artifacts cited or promised by thesis text.
- Do not chase every historical JSONL file.

### Appendix D. Notation Glossary

One-sentence obligation: preserve a compact notation reference.

Source:
- `thesis/appendix-notation.tex`

### Appendix E. Optional: Tube Algorithm Notes

One-sentence obligation: include only if Jorn promotes the tube algorithm or
needs it as future-work context.

Current default:
- Future/cut from main thesis.

Why:
- Current tube section has Jörn-review TODOs.
- Rotation increment formula and correctness math are not thesis-ready.
- `thesis/migration-findings.md` rows 1 and 12-14 show code/thesis divergence.

Alternative:
- Move to conclusion/outlook as "not used in the thesis closeout".

## What Probably Belongs in the Main Body

High-confidence main body:

- Introduction with Viterbo/HKO/thesis questions.
- Definitions and enough algorithm setup to read computations.
- HKO local-maximality chapter.
- Hostile-landscape chapter.
- Short validation/evidence-boundary chapter or section.
- Conclusion.

Medium-confidence main body:

- Feature-pattern data-science details, if compressed enough.
- Verification/orbit-recovery summary, if empirical claims are prominent.
- Visualization negative exploration, if Jorn wants it as a mathematical result.

Probably not main body by default:

- Full numerical appendix proof machinery.
- Tube algorithm.
- Large data freshness/provenance matrices.
- Exhaustive data-science taxonomy.
- LICCA/higher-`F` future checks unless already returned and cheap to integrate.
- Repo/software/process story, unless there is an explicit reader-facing reason.

## Current Dependency Map

### Must resolve before writing final HKO chapter

- Exact Packet 3 closure or final weaker claim route.
- Stale `44/10` HKO formal prose reconciliation.
- Final theorem wording modulo symmetries.
- Whether second-order material is proof route, support, or future.

### Must resolve before writing final landscape chapter

- Claim wording must stay bounded: no density or impossibility overclaim.
- `M012` and `M013` thesis use.
- Visualization inclusion status.
- Which skipped/deferred standard-toolbox rows are worth mentioning.

### Must resolve before writing algorithms/numerics as trusted exposition

- `thesis/migration-findings.md` rows 3-11.
- Public f64 wrappers vs stronger guarantee-mode boundary.
- Whether numerical appendix describes current implementation, desired
  verification layer, or both.
- Whether the new combinatorial-realization theorem is required or omitted.

### Must resolve before final reproducibility claims

- Which artifacts are cited in thesis text.
- Which figures/tables are included.
- Whether repository promises mention fresh clone, commands, archive, or only
  source availability.

## Concrete Draft TOC Variant

This is the compressed version I would start editing interactively:

```text
Abstract

1. Introduction
   1.1 Viterbo's conjecture and systolic ratio
   1.2 HKO2024 and the search for further counterexamples
   1.3 Contributions and thesis structure

2. Preliminaries and Computation of EHZ Capacity
   2.1 Convex polytopes in R^4 and symplectic notation
   2.2 Reeb orbits, action, and EHZ capacity
   2.3 Finite candidate orbits for polytopes
   2.4 Algorithms used in the experiments

3. The HKO2024 Configuration as a Local Maximum Candidate
   3.1 The pentagon-pentagon product and its symmetries
   3.2 The correct local-maximality statement
   3.3 Exact first-order certificate
   3.4 Supporting second-order and perturbation evidence
   3.5 Consequences and limitations

4. Negative Evidence for the Search Landscape
   4.1 Search question and claim boundary
   4.2 Random and structured baseline searches
   4.3 Local ascent and continuation
   4.4 Feature-based regression and classification attempts
   4.5 Skipped standard-toolbox methods and validity caveats
   4.6 Optional: visual exploration as negative evidence

5. Validation and Reproducibility of the Computations
   5.1 Implementation and artifact boundary
   5.2 Verification experiments and orbit recovery
   5.3 Numerical reliability and caveats

6. Conclusion
   6.1 What the thesis establishes
   6.2 What remains open
   6.3 Future computational directions

Appendix A. Numerical Implementation Details
Appendix B. Proof Details for Capacity Algorithms
Appendix C. Experiment Provenance and Supplementary Tables
Appendix D. Notation Glossary
Appendix E. Optional/Future: Tube Algorithm Notes
```

## Alternative TOC Variant If Algorithms Must Stay Prominent

```text
1. Introduction
2. Foundations: EHZ Capacity and Polytope Orbits
3. Algorithms and Numerical Reliability
4. HKO2024 Local Maximality
5. Computational Search Landscape
6. Conclusion
Appendices...
```

<!--
I like this less because it keeps the reader waiting too long for the thesis
results. It may be better if the advisor/examiner expects the thesis to read as
"we built and justified a computational method, then applied it". The current
research/task maps suggest the final value is instead in the HKO and hostile
landscape results.
-->

## Alternative TOC Variant If HKO Exact Proof Dominates

```text
1. Introduction
2. Mathematical Setup
3. Exact HKO Local-Maximality Theorem
4. Computational and Numerical Support for HKO
5. Search Landscape Beyond HKO
6. Conclusion
Appendices...
```

<!--
This becomes attractive if Packet 3 closes very cleanly and the exact theorem is
the obvious center of the thesis. It risks making hostile-landscape evidence
feel like an add-on, though Kai apparently accepted it as the second thesis
spine component.
-->

## My Strongest Recommendations

1. Do not keep the current `main.tex` order as the final conceptual order.
2. Do not put tube in the main body by default.
3. Do not let the numerical appendix become the thesis center unless retained
   claims force it.
4. Put HKO before hostile landscape unless Jorn wants the narrative "we search,
   fail, then understand why HKO is special"; HKO-first gives the reader the
   known counterexample before negative search evidence.
5. Keep validation in the main body only at summary level; move technical
   numerical details to appendices.
6. Make all empirical claims visibly bounded by artifact scope.
7. Use the TOC to decide figure needs after the section structure stabilizes.

## Likely Figures and Tables

Possible main-body figures:

- HKO pentagon-pentagon geometry schematic, if a clean thesis-owned asset exists
  or can be generated.
- Landscape search summary plot/table: method family, search surface, max `sys`,
  result.
- Feature transfer summary, only if `M011` remains main evidence.
- Visualization negative-exploration figure only if that result is retained.

Possible appendix tables:

- Full method-ledger/audit table from `research/sys-landscape-toolbox-audit.md`.
- Verification package summary: correctness checks, all-minimum, orbit-recovery.
- HKO representative/witness bookkeeping summary after Packet 3 closes.

Avoid:

- Figures whose provenance cannot be traced.
- Decorative or exploratory plots that do not support a retained claim.

## Questions I Would Ask During Interactive TOC Drafting

1. Should HKO local maximality be the first result chapter, or should the search
   landscape come first as motivation?
2. Is the numerical method story a main-body chapter or an appendix plus a
   validation summary?
3. Is visualization a standalone mathematical negative result in the thesis?
4. Is tube completely future/cut for submission?
5. What exact reader promise should the thesis make about code/reproducibility?
6. How much of the algorithm proof material should be main text versus appendix?

