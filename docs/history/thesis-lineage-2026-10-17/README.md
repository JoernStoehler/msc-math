# Thesis lineage of 16–17 October 2026 (preserved snapshot, not integrated)

Source branches (since deleted; full history in
`/workspaces/archived/consolidation-2026-11-05/msc-math.bundle`):
`claude/next-2026-10-16` (tip `ec0af105`, 106 commits), `codex/patterns-2026-10-17` (tip `4442275c`)
(32 commits), both branched from main at `cf3cb699` (14 Oct) and never merged; main
gained 62 commits since. Nothing here is built into `thesis/`.

## What is here, and why it might matter

| Path | What it is | Not in main because |
|---|---|---|
| `thesis/chapters/08-data-science.tex`, `thesis/appendices/data-science-contact.tex`, `thesis/figures/contact-cusp.png` | A Chapter 8 rewrite that adds §8.9 "contact loss predicts a constructed upper tail near HKO" (a local piecewise-linear law with a cusp, plus appendix A.12 exact-witness package). Branch chapter is 684 lines vs main's 584. | Main's Chapter 8 has no contact-loss section. Jörn's 17 Oct notes accept "somewhat worse prose" for more Chapter 8 insight. |
| `OWNER-NOTES.md` | Jörn's recorded answers/decisions of 17 Oct (registration, Kai, English consent, pattern-method ratings, ridge lead, etc.). | Not carried into main's docs. |
| `thesis/review/` | Review packet: frozen 105-page Kai candidate PDF, meeting agenda/checklist, registration sources, submission-form drafts, verification JSONs. | Dated to the 19 Oct Kai meeting; probably stale, kept as evidence. |
| `docs/history/thesis-audit-*`, `thesis-followup-*`, `thesis-integration-*`, `docs/*.md` | Audit/verification records and evidence map for that candidate. | Describe a candidate that main did not adopt. |
| `experiments/writing-quality/` | Reader/writing-quality runs. | Evidence only. |
| `thesis/chapters/10-symmetric-partner.tex` | Earlier symmetric-partner chapter. | Superseded by main's `10b-symmetric-partners.tex` (selected 22 Oct). |
| `patterns-branch-variants/` | Variants of chapters/docs from the patterns branch that differ from next and main. | Evidence. |

Related and already in main: `patterns/` (research results), `topics/` research records.
Unaccepted introduction v5 and its chapter edits: `../opening-pass-2026-10-23-candidate/`.

## Decision still open (Jörn)

Whether to adopt the Chapter 8 contact-loss section into the thesis. See main `STATUS.md`
("Thesis scope"). To try it: diff `08-data-science.tex` here against `thesis/chapters/`,
copy the §8.9 block, add the appendix and figure, and run `thesis/build.sh`.

## Small prose edits

`next-thesis-edits.patch` and `patterns-thesis-edits.patch` hold the branches' remaining edits to
the thesis chapters/preamble/bibliography (mostly 2–13-line wording changes: removing project
chronology, softening claims, a title page, an AI-contribution paragraph for the HKO proof, a
`\input` for the symmetric-partner chapter and contact appendix). They are diffs against the
14 Oct base (`cf3cb699`), so use `git apply --3way` and expect conflicts. The Ch. 8 rewrite
is kept as a whole file instead (above); `other-docs-from-next/` has the submission-status docs
(`DISTANCE-TO-SUBMISSION.md`, registration material) as of 17 Oct.
