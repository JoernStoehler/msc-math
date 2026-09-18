# Candidate specialist flag inventory

Six fresh-context passes applied the two unchanged candidate prompts to complete selected introduction, numerics and variation chapters. **13 flags, no human adjudications, no manuscript edits.** The raw responses contain every flag, retained-detail recommendation, surrounding justification, uncertainty and proposed minimal action. This is a finite concern list for integration, not an acceptance verdict or evidence that unflagged text is clean.

The frozen selection follows candidate assembly's `main.tex` at 2026-09-18 13:05:36 UTC. The three chapters contain 4,446 whitespace-separated source/formula tokens (not model-token counts); each was reviewed twice. Main and the resolved legacy preamble supplied structure/notation context. Other chapters, labels and previous reviews were not supplied to these fresh reviewers. The coordinating agent read the corrected [qualification utility account](/workspaces/msc-math/.worktrees/reviewer-trial/docs/reviewer-trial/evaluation/qualification-utility.md); raw reviewers did not receive those labels. Earlier human judgments do not validate these new flags.

`manifest.json` records absolute original paths, frozen bytes' SHA-256 hashes, exact unchanged prompt hashes, raw output hashes and counts. The source directory was being edited concurrently; these are snapshot claims, not claims about its later head. Each raw file has agent-reported review timestamps. Freeze to final raw response was under two minutes (last reported completion 13:07:10 UTC); preparation and synthesis added several minutes. No API cost telemetry was available to this task. Per-agent timestamps are self-reported, not instrumented API duration.

## Complete finite inventory

The identifiers below index concerns in the linked raw files. Nothing is omitted or silently reclassified as a validated defect. Coordinator triage is an additional recommendation, not a second independent evaluation.

| ID | Source / exact anchor excerpt | Reviewer proposal | Coordinator triage |
| --- | --- | --- | --- |
| IR1 | Introduction: “The contributions grew out of a broad computational investigation.” (two following discovery-history sentences together) | Move generic discovery history out of introduction | Defer to intended thesis framing. The whole passage explains why separate results belong together; removing it is not an automatic improvement. |
| IR2 | Introduction: “Its performance in our tests made it less useful for large searches,” | Move performance assessment to method chapter | Plausible small edit if performance is already explained there; retain cross-checking role. |
| IQ1 | Introduction: “without classifying every minimizing orbit.” | Delete appended distinction | Plausible simplification in this overview. Preserve existential six-facet theorem and full minimizer distinction where algorithm outputs are specified. |
| IQ2 | Introduction: “but does not include arbitrary pentagons.” | Delete as already implied family restriction; reviewer uncertain | Low priority; family scope is relevant and short. No need to remove merely to satisfy this flag. |
| NR1 | Numerics: “It first uses a cheap induced norm bound and retries an indeterminate system with a tighter entrywise bound.” | Move retry ordering to methods note | Optional compression; numerics is itself an implementation-explanation chapter. Can retain if it explains fallback behavior. |
| NR2 | Numerics: “A compiler, dependency, target, or arithmetic-contract change therefore requires the floating-point certificate tests to be repeated.” | Relocate maintenance instruction | Plausible methods/reproduction-note placement; preserve actual arithmetic assumptions. |
| NR3 | Numerics: “The test source is” through the two correspondence-test paths | Move paths to footnote | Low-cost presentation improvement if layout benefits; preserve both reader-resolvable links and fixture scope. |
| NQ1 | Numerics final paragraph: “The guarantees above concern the exact dyadic polytope represented by the input.” through “candidate-count limits.” | Delete repeated scope recap; reviewer uncertain | Low priority. Repetition may help after dense certification material; exact-input interpretation must remain explicit elsewhere. |
| NQ2 | Numerics: “not a theorem about all polytopes.” | Delete appended denial | Plausible small deletion; positive implementation-domain statement and justification remain. |
| NQ3 | Numerics: “Exact arithmetic is therefore a selective correctness fallback, not the default arithmetic for every word.” | Delete repeated arithmetic-policy summary | Plausible compression; can also retain as useful summary if nearby detail is shortened. |
| VR1 | Variation: “A later comparison tests seven fixed policies on sixty-four matched starts,” through optimizer-comparison path | Move study inventory to experiment note | Highest-priority content-placement question: paragraph names a study but gives no result. Supply its relevant finding or relocate intact, preserving historical-objective and separate-panel qualifications. |
| VQ1 | Variation: “The hypothesis preceding” envelope equation “contains real mathematical work.” through displayed-identity warning | Move numerical-window warning to search subsection | Relocation better than unqualified deletion: branch windows arise immediately afterwards, so warning has a nearby use. Preserve coverage requirement and gap hypotheses. |
| VQ2 | Variation: “They do not require finite differences of the full capacity computation.” | Delete alternative-computation comparison; reviewer uncertain | Low priority; reader implementing local search may find this direct comparison useful. |

## Raw runs and exact context

| Full frozen chapter | Reproduction-detail review | Qualification review | Flags |
| --- | --- | --- | ---: |
| [Introduction](inputs/introduction.tex) | [raw IR](raw/introduction-reproduction.md) | [raw IQ](raw/introduction-qualifications.md) | 2 + 2 |
| [Numerics](inputs/numerics.tex) | [raw NR](raw/numerics-reproduction.md) | [raw NQ](raw/numerics-qualifications.md) | 3 + 3 |
| [Variation](inputs/variation.tex) | [raw VR](raw/variation-reproduction.md) | [raw VQ](raw/variation-qualifications.md) | 1 + 2 |

No duplicate concern was removed. The two prompts were copied byte-for-byte to `prompts/`. Both narrow reviewers remain plausible human-triaged flaggers with unknown utility; these runs neither establish useful yield nor justify automatic deletion. Potentially useful next integration actions are VR1, source-path footnotes and a few short redundant clauses, chosen after checking their role in the assembled text. No additional human review is requested by this packet.
