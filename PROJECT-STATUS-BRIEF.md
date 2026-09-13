# Project status and content tree

Updated: 2026-09-13

## Thirty-second answer

The project has a real content structure. It is a nearly complete 110-page
master's-thesis candidate plus its proofs, code, experiments, and retained
evidence. The best human presentation is **a portfolio of related results with
one common mathematical spine**, not one artificially unified theorem.

The spine is:

> Viterbo's conjecture is false because of the HKO pentagon-product
> counterexample. This project asks how rigid that example is, how its
> symplectic capacity can be computed finitely, and whether broader computation
> finds anything better.

The strongest answer is an exact local-rigidity theorem. The project also proves
an exact structured-family result and a conditional finite algorithm, and
reports a bounded negative computational search.

## Human-facing content tree

Status key: `[THEOREM]`, `[EMPIRICAL]`, `[METHOD]`, `[CONTEXT]`, `[OPEN]`.

```text
Probing Viterbo's Conjecture
|
|-- 1. Why this project exists [CONTEXT]
|   |-- Viterbo predicted normalized capacity <= 1.
|   |-- Haim--Kislev--Ostrover found a pentagon-product counterexample.
|   `-- New question: how special, rigid, or improvable is it?
|
|-- 2. Make the geometry finitely computable [METHOD]
|   |-- Generalized Reeb orbits on polytopes
|   |-- Haim--Kislev finite quadratic program
|   |-- Simple minimizing orbit / finite word reduction
|   `-- First-order variation in facet coordinates
|
|-- 3. Main achievement: local rigidity of HKO [THEOREM]
|   |-- HKO is locally maximal among nearby 10-facet polytopes.
|   |-- Equality is exactly translation, scaling, and linear-symplectic motion.
|   `-- Exact certificate: 26 upper functions control 25 transverse directions.
|
|-- 4. Additional exact results [THEOREM]
|   |-- Exact capacity profile for two relatively rotated regular pentagons
|   |-- HKO is globally maximal inside that one-parameter family
|   `-- Exact flow-graph capacity search under explicit regularity hypotheses
|
|-- 5. Broader computational search [EMPIRICAL]
|   |-- 14,336 retained random-polytope/product rows: none above 1
|   |-- 100,000 candidates screened; 1,675 selected/baseline rows evaluated:
|   |   none above 1
|   `-- Honest limit: a bounded negative result, not nonexistence
|
|-- 6. How to trust and interpret the work [METHOD]
|   |-- Binary64 exploration versus exact SageMath certificates
|   |-- Rust libraries, tests, retained data, and reproducibility boundaries
|   |-- 3-D visualization as intuition, not proof
|   `-- Factual AI-use disclosure and research-process case study
|
`-- 7. What remains mathematically open [OPEN]
    |-- Local maximality when facets may appear or disappear
    |-- Complete first-order theory at singular optimizer data
    `-- Other structured polygon-product families and better counterexamples
```

## What the project has achieved

1. **Central result.** A theorem-strength candidate proves that the HKO body is
   Hausdorff-locally maximal within the stratum of polytopes with exactly ten
   facets, modulo the natural symmetries. This is the headline contribution.

2. **Exact side result.** The thesis determines the full systolic-ratio profile
   of the relatively rotated regular-pentagon product. The HKO position is its
   global maximum.

3. **Finite mathematical machinery.** The thesis develops the polytope/Reeb
   reduction, the Haim--Kislev quadratic-program route, first-order upper
   functions, and an exact flow-graph theorem with explicit hypotheses.

4. **Bounded search result.** The retained searches found no additional body
   above the former bound. They still provide a reproducible negative benchmark
   and some sub-threshold structure, but not a general impossibility result.

5. **Evidence separation.** Discovery code, exact certificates, mathematical
   implications, empirical tables, and exploratory images are presented as
   different evidence types instead of being blurred together.

## Current status

- A complete 110-page PDF exists at `thesis/build/main.pdf`.
- The active manuscript contains an abstract, introduction, 13 substantive
  sections, conclusion, appendix, and bibliography.
- The PDF was rebuilt on 2026-09-11. The repository records that the build,
  references, and selected layout checks passed.
- The HKO local theorem and rotated-pentagon theorem are treated as established
  theorem candidates in the active thesis, with exact certificates. The
  flow-graph result is conditional on stated regularity assumptions.
- Data science is the least settled scientific strand. The active prose now
  makes a defensible bounded claim, but Jörn has not yet decided whether this is
  enough or whether any remaining incompleteness should be finished.
- Full-manuscript scientific and visual review by Jörn remains unrecorded.
  Kai's final feedback stage also remains ahead under the recorded review plan.
- Current submission/deadline status is unknown. The recorded 2026-08-30
  deadline was retracted; the repository must not guess whether submission
  happened or whether a new deadline exists.
- A frozen Zenodo release and final archive/rights audit are planned but not
  recorded as completed. They are delivery work, not prerequisites for judging
  the mathematics.

So the shortest honest status is:

> **Research-rich complete draft; central mathematics substantially closed;
> data-science scope and human acceptance still open; external submission
> status unknown.**

## Brief question sequence to establish the real status

Answer these in order, with one line each. Stop once an answer exposes the next
concrete blocker.

1. **Candidate:** Is `thesis/build/main.pdf` the current thesis candidate, or is
   there a newer version elsewhere?

2. **Headline claims:** Should the final thesis keep these three theorem claims
   as written: HKO ten-facet local maximality, exact rotated-pentagon profile,
   and conditional flow-graph correctness?

3. **Data-science stopping rule:** Is the bounded negative result already enough
   for submission, or must one specific incomplete analysis be finished?

4. **Your PASS judgment:** After a full read, what is the first concrete issue
   that prevents you from saying, "this should pass"? If there is none, record
   the candidate as ready for Kai.

5. **Kai stage:** Has Kai already seen this candidate? If not, is the next step
   to send it after Jörn's repairs, as currently documented?

6. **Official state:** Was a thesis submitted, and what deadline or university
   action is currently real? This must come from Jörn or a current official
   source; the repository does not know.

7. **Closure:** After the thesis itself is accepted, which archival promise is
   required now: only a clean repository, or also the planned frozen Zenodo
   bundle?

## Recommended spoken presentation

For a supervisor, examiner, or mathematically literate listener:

> The thesis starts from the HKO counterexample to Viterbo's conjecture. It
> turns the relevant polytope dynamics into finite optimization problems and
> uses exact certificates to prove that the HKO ten-facet polytope cannot be
> improved locally except by symmetries. It also computes the exact profile of
> the rotated-pentagon family and proves a conditional exact flow-graph method.
> Broader random and data-driven searches found no new counterexample, which is
> reported as a finite negative result rather than a theorem. The result is a
> map of where the known counterexample is rigid, which computational tools are
> justified, and where the global search remains open.

## Where the layers live

- Reader-facing thesis: `thesis/`
- Proof development and audits: `formal/`
- Empirical producers and retained evidence: `experiments/`
- Reusable Rust implementations and tests: `crates/`
- Source papers and notes: `papers/`
- Reproduction and project-wide facts: `docs/`
- Submission and archive material: `submit/`
- Current planning context: `memories/`

These directories are evidence layers, not the order in which the research
should be explained to a human. Use the content tree above for the story.
