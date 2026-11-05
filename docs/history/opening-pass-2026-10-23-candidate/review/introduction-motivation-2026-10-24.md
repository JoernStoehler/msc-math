# Introduction motivation repair — 24 October 2026

Candidate-only repair in `opening-pass-2026-10-23`; receiving PM is
`01a16da4-32c1-79c2-b7a2-cec58615a4c1`.

**Historical disposition:** Jörn rejected this candidate at 12:06:51 UTC in
answered Desk `7b13611b-bd83-42df-91de-674bd2bf1d02`, despite the review below.
The source remains recoverable at `a39e93ff`; the PDF link is mutable and now
shows later work. [The next repair and reader evidence](introduction-function-2026-10-24/README.md)
preserve the mismatch. The remainder records the checks before that rejection.

## Rejection received

Desk `d050457a-6ed3-4bad-8136-289c72ee022c` was answered at
2026-10-24T11:21:53.498Z:

> i don't evne recognize the introduction as such? it starts with previous work, instead of motivating first that this thesis aims to give contributions to a field, especially around bringing in empirical methods and using AI?

The full question and answer were read. This rejects candidate `74adc710`;
it is not an outstanding grade request. The earlier Luna/medium writer and
reviewer shared inherited context; their pass did not establish acceptance.
The earlier source-owned record is [the opening-pass account](opening-pass-2026-10-23.md).
No new question was sent to Jörn by this worker.

## What changed

The old introduction began with the conjecture, capacity comparisons and the
HKO counterexample before stating what this thesis sought to contribute.
The new first three paragraphs lead with the thesis's purpose in computational
symplectic geometry, explain why empirical comparisons and search complement
explicit families, and introduce AI-assisted mathematical research as a subject
of reflection. The conjecture, specific contributions and literature follow.
AI research-process reflection remains distinct from the unchanged disclosure.

Only `thesis/chapters/01-introduction.tex` changed in the manuscript. The
rotated-pentagon formula is summarized with its theorem reference instead of
repeated in full. Capacity/Mahler context moved to the literature subsection.
The empirical summary now includes weakening under selection; numerical
association and unsuccessful search are explicitly distinguished from proof.
No new mathematical result or wider validation claim was introduced.

## Evidence map

“Met” below means supported at the level needed for this introduction rewrite.
It does not certify the underlying mathematics or human writing acceptance.

| Requirement | Status | Inspected support / remaining limit |
| --- | --- | --- |
| Purpose and contributions before prior work | Met | Introduction paragraphs 1–3 now precede Viterbo/HKO; specific statements remain in Contributions. |
| Empirical rationale and limits | Met | `08-data-science.tex`: sampling, selection, opposite exact pentagon/triangle laws, search comparisons and endpoint probes. No general causal law or certified local search optimum claimed. |
| AI as part of research, distinct from disclosure | Met | `15-ai-reflection.tex`, `ai-use-disclosure.tex`, project-facts items 8 and 45. The reflection gives the local-proof example and task/exposition difficulties; no general experiment on AI effectiveness is claimed. |
| Mathematical contribution summaries | Met for alignment | Theorem statements in `07-hko.tex`, `09-rotated-regular-polygons.tex`, `10-affine-pentagons.tex`, `10b-symmetric-partners.tex`, `11-product-position.tex`. Ten-facet restrictions, non-strict second maximum and family bounds preserved. Proofs not reverified. |
| Algorithmic/expository contribution | Met for alignment | HK finite formula/product reduction in §§2–4; flow theorem in §5 retains regularity and exact-real-arithmetic conditions; first variation in §6. No unconditional implementation correctness claim. |
| Literature attribution | Partial | Existing citations and in-manuscript context preserved; no external-paper or novelty audit performed. |
| Build and focused page fit | Met | `sh thesis/build.sh` exits 0; 104-page PDF; introduction on physical/printed pp. 5–6. Both writer and reviewer inspected rendered pp. 5–6; writer also inspected preceding contents page. No opening overflow or unresolved references. |
| Whole-document checks | Partial | Build log retains a 13.45471pt overflow in unchanged `14-code-data.tex` and underfull boxes outside the opening. No whole-PDF visual review. |
| Fresh-context review | Met, bounded | Astra/high reviewer approved `opening-v2`; manuscript alignment and two pages only. Same model family as writer; no cross-model or human validation. |
| Human acceptance | Missing | Jörn has not assessed this new candidate. PM owns any next consultation. Whole-thesis PASS remains open. |

## Runtime and review identity

Writer: `01a16da7-54ea-7ea3-8fe6-fd8a0a4e851c`, verified current
`turn_context`: `gpt-6-astra`, effort `high`.
Reviewer: `01a16daa-70c3-7fa2-bd00-298c9f2ce022`, native path
`/root/opening_review`, also verified from its actual current `turn_context`
as `gpt-6-astra` / `high`. Spawned with `fork_turns="none"`; received the
actual rejection and bounded review assignment, plus repository access,
without parent conversation or the parent's proposed verdict.
[Retained reviewer return](introduction-motivation-2026-10-24-review.txt).

Reviewed introduction SHA-256:
`1b2731fd373233116edd486bfd9e77972065da5686e3957e2f181415814b0069`.
Reviewed PDF SHA-256:
`492aa6522d6a201b0b01030b3839c3476efe37af82f2d18f549d9b294d9db9c8`.
Scratch build/render/review evidence is under
`/tmp/msc-math/introduction-motivation/20261024/`.

[Open the candidate at page 5](https://joern-pc.tailc5e761.ts.net/files?project=msc-math-worktrees&path=opening-pass-2026-10-23%2Fthesis%2Fbuild%2Fmain.pdf&page=5).
This exact Desk link was opened in Chrome: it showed page 5 of 104 and the
new first paragraph. The older absolute-path URL convention served the Desk
HTML shell rather than the PDF, so it is not used here.

## State and handoff limits

Candidate and main `STATUS.md` / `current.json` reconcile the answered
rejection and route the new candidate to PM. Main manuscript and unrelated
viewer edits are preserved. Candidate thesis synthesis explicitly labels
historical page references/counts stale; its global source manifest was not
refreshed without whole-source/PDF/PASS reconciliation.

The graph projection is regenerated. The existing dashboard checker fails in
both checkouts because `docs/history/file-viewer-review-20261023.md` has no
explicit serving entry; this unrelated source/allowlist mismatch is preserved
and handed off, not treated as a manuscript failure.
