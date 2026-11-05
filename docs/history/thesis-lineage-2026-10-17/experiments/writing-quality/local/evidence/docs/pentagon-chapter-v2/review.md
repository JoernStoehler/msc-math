# Completed review and limitations

## Evidence recovered before writing

The positive source was the actual seven-page HKO wrapper and its opening input
under `.git/codex/hko-writing/`. The human judgment and annotations were read
in `.git/codex/review-calibration/hko-reading-feedback.md`, together with
`memories/authoring-workflow.md`. The useful positive pattern is to identify
the geometric question, introduce objects for a visible purpose, and explain
how their properties establish the conclusion. This is not a phrase blacklist.
The HKO PASS itself included annotations and is not evidence that every sentence
in it should be copied.

## Frozen blind diagnostic

An independent reviewer with no conversation history received five passages,
without their source labels or human judgments. Before labels were disclosed,
it returned these predictions:

| Passage | Frozen prediction | Human evidence |
| --- | --- | --- |
| “The HKO body has systolic ratio greater than one. We now ask what happens when its ten supporting hyperplanes move…” | Acceptable: question followed by concrete scope of perturbations | From accepted HKO draft |
| “Floating-point evaluations of the systolic ratio suggest a simple rotation profile.” | Acceptable: numerical observation introducing a profile | Rejected pentagon feature: leads with implementation rather than mathematical activity |
| “Thus each function touches the ratio from above… Different bounds may handle different directions of motion.” | Acceptable: explains how the upper bounds serve the proof | From accepted HKO draft |
| “Every angle is equivalent to d(theta) in that interval.” | Revise: does not state what equivalence preserves | Rejected pentagon feature |
| “The endpoint values come from the Haim–Kislev–Ostrover example.” | Revise: provenance without the actual values or identification | Rejected pentagon feature |

The reviewer missed the implementation-first failure. After revealing labels,
its assignment was corrected to assess whether the mathematical inquiry leads
and whether computational provenance serves an already intelligible inquiry.
It acknowledged the miss. No further calibration rounds were used, and this
small retrospective diagnostic is not a validated reliability benchmark.

## New-chapter reading review

The same reviewer then read the complete new chapter independently of the
author's drafting. It found no necessary prose revision, with concrete grounds:

- The opening asks how rotation changes capacity, explains why volume matters,
  and gives numerical observations a supporting role.
- The roadmap identifies the common feasible weights and why known endpoint
  capacities can control the interval.
- Symmetry statements name the preserved quantity, and the endpoint paragraph
  displays the capacity immediately after identifying the HKO position.
- The candidate's purpose precedes its word and weights. Closure and the
  grouped-pairing calculation explain its value. The transition to bounding
  all competitors explicitly states the remaining task.
- The displayed angular form is immediately used in interpolation without
  introducing the unnecessary term “first harmonic”. Nonnegative coefficients
  justify the inequality rather than leaving its direction implicit.
- Volume normalization and equality cases answer the opening question.

The author also assessed these functions and the order in which the objects
are needed. Reviewer acceptance was evidence for that judgment, not delegated
authority to declare a human PASS. The rewritten weight vector uses the already
defined geometric constant h, avoiding unrelated radical constants while
making normalization and closure easier to inspect.

## Separate meaning-regression check

After Jörn accepted the underlying argument, the reviewer was given a distinct,
narrow assignment to compare mathematical changes with audit section 2 and
the old attaining word. It found no regression:

- `2h/[4(1+h)] = sqrt(5)/10` and `1/[4(1+h)] = (5-sqrt(5))/20`.
- Closure still follows from `n0+n1=-2h n3`. The normalized weighted groups are
  `(u,0),(0,v),(-u,0),(0,-v)` and their ordered pairings total `2u dot v`.
- Interpolation has the same endpoint assignments, coefficient signs and sum;
  the reciprocal capacity inequality has the correct direction.
- Evenness, period, the distance formula, systolic coefficient and maximum
  are unchanged. The explicit symplectic-convention conversion chooses the
  opposite HKO endpoint, which reflection identifies with the original one.

This was a separate task but reused the prose reviewer's context because of
the agent-thread limit. It is not an independently sourced broad mathematical
audit, a proof-assistant verification, or a reproof of the imported theorems.

## Build and rendered inspection

`latexmk -pdf -interaction=nonstopmode -halt-on-error standalone.tex` completed
successfully with four pages. The final log has no LaTeX warnings, undefined
references/citations, or overfull/underfull boxes. All four pages were rendered
and visually inspected. An initial float position placed the plot ahead of
the section opening; the float option was corrected and every final page was
inspected. The chapter now opens with the mathematical question, and the plot
follows its discussion. The equations and bibliography are legible and unclipped.

Remaining limitation: no human judgment on this new exposition. Full-thesis
integration, removal of stale computational-proof overview claims and
whole-document layout checks are explicitly deferred; the exact handoff is
in README.md. Neither earlier artifacts nor the main thesis were changed.
