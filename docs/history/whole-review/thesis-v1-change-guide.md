# Whole thesis v1: review guide

**Complete intermediate PDF:** `whole-thesis-snapshot-v1.pdf` in the review index
(repository file `thesis-v1-5458356d.pdf`), 93 pages. Printed page numbers
match PDF page numbers. Source revision: `5458356d8e60a7594a0ae029f735fe77f875090f`.
This is an intermediate frozen snapshot. DS v2 has been desk-rejected on
exposition and is awaiting revision under the active prose gate; this snapshot
is not the requested human whole-thesis reading candidate. Human whole-thesis
acceptance and release selection remain pending.

## Latest changes and reading map

| Pages | What to read |
|---|---|
| 1, 5-8, 77-78 | The abstract, introduction and conclusion now include the factor-balancing experiment, opposite exact triangle/pentagon laws and the bounded branch-history search result. |
| 49-59 | The rewritten DS chapter leads with geometric claims, conditional attenuation and controlled evidence. It adds the exact triangle profile, states the limits of adaptive sampling comparisons, and connects selection descriptors with direct search. On p. 49, begin at Section 8 below the HKO table; stop before Section 9 on p. 59. |
| 78-89 | DS methods, additional results and evidence index. The new exact triangle proof is on pp. 78-79; the factor-balancing protocol and finite-budget optimizer comparison begin on p. 83. Appendix A starts partway down p. 78, after the conclusion. |
| 74-76 | Revised AI reflection distinguishes understanding a goal from pursuing it, and explains whole-task evaluation and persistent writing difficulties. |
| 41-49, 59-68 | Main HKO theorem, rotated-pentagon profile, independently deformed pentagons and factor-plane rotation. These form the mathematical context for the revised summaries and DS chapter. |
| 70-73, 90-93 | Numerical contracts, code/data availability, exact HKO program and references. The availability section retains its boundary between published material and newer local work. |

The separately supplied DS v2 reading on standalone pp. 2-11 corresponds to
Section 8 here. The three-page AI v2 reading corresponds to pp. 74-76 here.
This whole PDF includes both revisions and the updated summaries.

## Scientific and prose review status

The exact triangle law and its transcription received independent mathematical
checks. The balancing experiment received separate statistical and geometric
checks; its 320 matched pairs and 640 successful evaluations are reflected in the
chapter. These results support a material contribution from factor-shape changes,
without identifying covariance imbalance as the sole cause or promising universal
improvement.

**Final bounded cross-chapter consistency check: agent PASS on this exact PDF.**
The independent reviewer confirmed the artifact hash and checked all five final
deltas. The three prior synthesis findings are resolved: updated summaries,
bounded local-search success and the quarter-turn notation collision. No new
substantive consistency issue likely to require more than two hours of repair
was found. This is not an independent proof audit or a whole-human-PASS. See the
frozen `thesis-v1-synthesis-review.md`.

**Prose status: DS v2 is desk-rejected; revision is required before whole reading.**
The earlier DS editor independently rechecked three substantial repairs, but that
bounded follow-up did not establish chapter acceptance. A compact replacement for
the broad scatter grids
(pp. 52-53) remains optional. The AI reviewer reports one medium mark (the recap
in Section 15.3 on p. 75) and two low/optional marks (the meaning of twenty-five
nonsingular cases on p. 74 and repeated diagnosis in the final paragraph on
p. 76). These are agent findings, not human acceptance. See the accompanying
`ds-v2-agent-notes.md` and `ai-reflection-v2-agent-notes.md`.

## Integration checks and minor layout observations

The final build has no LaTeX/Biber warnings, overfull/underfull boxes or unresolved
references. All 344 PDF links and 462 named destinations were inspected for
internal-target validity. Title and author metadata are correct. All 93 pages
were rendered and inspected in contact sheets; 22 changed or adjacent pages were
also inspected at reading resolution. No clipping, overlap, unreadable text,
broken table/figure layout or blank pages was found.

Sparse page endings remain on pp. 40 and 48; p. 76 contains only the final AI
paragraph before the conclusion's page break. These are visible minor layout
observations, not scientific defects. The build and layout checks do not establish
human prose acceptance or independently validate every proof and experiment.

The frozen `thesis-v1-manifest.json` records hashes for all 51 actual
project inputs, generated inputs, TeX distribution inputs and build evidence.
Every project input matches the source revision and remained unchanged during the
build. The older historical build manifest has been preserved.
