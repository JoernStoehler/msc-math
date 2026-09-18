# Whole-thesis reading of the fixed 17:00 snapshot

PDF: `.git/codex/thesis-review-1700/build/main.pdf`, 81 pages, SHA-256
`8b1b8d2d54203af83f2e01f2b684c040ba52e845d0f23256ca9117c2f511e5d6`.
This snapshot contains the HKO point edits but predates the lemma/figure/CAS
appendix restructuring. Its PDF stays unchanged during reading.

First comments concern the abstract, newly written by the coordinator:

- “capacity-to-volume ratio” is factually wrong for the systolic ratio.
  Coordinator acknowledged authorship and changed it to “systolic ratio”.
- “The main theorem” should make the thesis contribution explicit.
  Changed to “Our main theorem”.
- The upper-bound/ordered-number-field/transverse-derivative proof summary
  is probably too detailed for the abstract. Removed those two sentences;
  mechanism remains in the introduction and proof.
- “the finite capacity calculations used in this argument” has a sloppy
  reference. Replaced with a direct statement about finite EHZ capacity
  algorithms for polytopes and first-order formulas under regularity hypotheses.
- Jörn then specifies the contribution statement: implementation, optimization
  and extension of two algorithms based on HK2017's QP and CH2021's flow graph,
  plus first-order variation. He judges the regularity qualification unnecessary
  in the abstract. The candidate follows this direction.
- The pentagon result should be called a theorem/proof of an explicit formula
  for arbitrary relative rotation, including HKO's global maximum within that
  family. The candidate now states both the proved formula and that maximum.
- Jörn requests “An empirical investigation” rather than the definite article;
  this wording is applied.
- Jörn finds selection rules and adaptive sampling too detailed for a mathematics
  abstract. He tentatively proposes the contrast between sub-threshold patterns
  and unsuccessful naive extrapolation, with methodological/geometric next steps,
  but explicitly is not satisfied with his proposed wording. The revision uses
  this higher-level contrast and leaves the next route open; it does not claim
  that basic data science has been exhausted or that either route is necessary.

Changes are in the evolving candidate, not the fixed reading PDF. No
whole-thesis approval has arrived. These comments are not a controlled
model comparison and no prospective abstract review was frozen.

Introduction: Jörn calls the question about why a particular shape has a large
value too specific. He provisionally finds the introduction somewhat sloppy
but fast-paced/relevant and reserves judgment until finishing the chapter.
The question is broadened to geometric relationships involving capacity;
no overall introduction judgment is inferred.
Jörn also requests making the title more explicit at the end of the motivation.
The opening now explains probing the conjecture as investigating capacity and
volume beyond the disproved bound.
Jörn objects that the motivating question was tailored retrospectively to the
theorem. He requests the broader question of local maximality among convex
bodies and the wording “partially answers the first question”. Both are applied;
the ten-facet restriction remains explicit in the result.
Jörn requests identifying the expository contribution: expanding the original
paper's proof into a self-contained presentation for master's students. The
finite-calculation motivation now says this explicitly.
Jörn finds the feasible/global/stationary/boundary-orbit distinctions too
detailed and irrelevant at introduction level. Those sentences are removed
there; the mathematical distinctions remain in the relevant chapters.
Applying that same feedback, the coordinator also removed the introductory
stationary-optimizer-continuation caveat from the HKO proof overview.
Jörn says the flow-graph paragraph lacks relevance: it was pursued as an
alternative, performed poorly, and now serves as a reference implementation
for comparisons. He also requests explaining before the separate results
that they grew from a broad computational exploration. Both points are added.
Cross-method agreement is described as evidence, not a proof of implementation
correctness. Detailed regularity/rational-arithmetic qualifications remain in
the method chapter, not in this introductory contribution statement.
Jörn finds the DS overview too detailed and the organization paragraph
insufficiently motivated. The former is reduced to purpose and high-level
outcomes; the latter explains dependency order, a theorem-first route, and
which chapters are independent of the HKO proof. He identifies the missing
literature review and requests indirect significance of Viterbo's conjecture.
A source-checked literature/motivation addition is underway; his recalled
global EHZ/cylindrical equality is being checked, not assumed.
A primary-source search locates HKO Remark 1.4(v), which explicitly leaves
EHZ/cylindrical equality on convex R4 domains open, and Edtmair's 2024 local
near-ball equality and global-surface-of-section characterization. The new
literature subsection distinguishes these and adds the AKO2014 restricted
Mahler equivalence and AB2023 Zoll local-maximality context. Sources are cited
in the candidate bibliography. The independent scope check prompted adding
Edtmair's smooth star-shaped hypothesis and the forward/backward return
condition in the surface-of-section description; both are incorporated.

## Review after switching to the fixed 17:30 PDF

Jörn immediately rejects the preliminaries opener (“The later computations
replace a boundary-dynamics problem by finite data from the facets of a
polytope”) as sloppy and asks whether Codex should have flagged this wider
class of writing-quality issues. Coordinator withdraws the implied prose
readiness claim: the preflight had been restricted to material mathematical
prerequisites/inference defects and explicitly excluded a style inventory.
That narrower check was not evidence for prose readiness. A new sequential
prose review is underway, with the restriction withdrawn and human examples
provided without making them an exhaustive taxonomy. The known opener is not
counted as a new detection. Jörn is asked to pause this section's human review.

Jörn next objects to “We will use ... without further comment.” The sentence
is deleted in a scratch-owned notation copy; original tracked sources and
the fixed PDF remain unchanged. The section opener now identifies the
nonsmooth-boundary problem and the purpose of the variational formulation.

Section 13.1: Jörn says the specified-certificate example misrepresents the
real history. He supplied the local-maximality conjecture and only a broad
upper-bound/neighborhood-modulo-symmetries strategy. A GPT-5-series model
(exact version uncertain) developed essentially the full argument: 26 upper
bounds, feasible beta(a) sections from invertible 5x5 closure minors after an
initial approach gave only 25 nonsingular cases, remaining volume proof steps,
and SageMath code. The candidate now credits that contribution and does not
portray the finished derivative certificate as user input. This is sourced
to Jörn's direct account; original-session corroboration is pending. Jörn
offers a host-side Luna search of the session logs; no model version is guessed.
Jörn later raises, then withdraws, a Chapter 4 reconstruction/motivation
annotation after recalling the earlier dual-reconstruction theorem. No edit
or new confirmed defect is attributed to that withdrawn annotation. It does
not withdraw his separate Section 13.1 historical-attribution correction.
Jörn subsequently judges Chapter 4 safe from the math/CS audience mismatch:
“it stays nicely on the math side! readable for a mathematician like Kai!”
This is positive readability/audience evidence for Chapter 4 in the fixed
17:30 PDF, not a whole-thesis PASS judgment. The independent predicted QP
issues remain agent suggestions, not human-confirmed defects. The working-copy
clarifications preserve the mathematical structure; a follow-up check qualified
the new dwell-time interpretation to pure-facet-word orbits.
Jörn then confirms he finished Chapter 4: it is appropriate and “would be a
PASS”, but writing quality is “borderline”. This upgrades the evidence to a
completed chapter-level PASS judgment for the fixed 17:30 version, while
preserving the explicit quality reservation. Whole-thesis approval is open.
