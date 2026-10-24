# Status — msc-math

v1.6 · 2026-10-24 · Codex thread 01a16d53-7d7a-7fb0-a64b-bf5c14b53256

## Goal
The intended outcome is submission and a passing result for Jörn's MSc thesis "Probing Viterbo's conjecture". It has not been submitted to the university; so far it has only received failing grades in preliminary checks (source: Jörn, desk question 6ded353d, 23 Oct). Checks found and their causes: [preliminary checks](docs/preliminary-checks-2026-10-23.md).

## Open
- **Applicable colloquium requirement/status established.** Current listed PO includes it; personal applicability/completion is unverified ([current-source check](docs/history/official-requirements-check-20261023.md)). Next: check governing student record before deciding whether an inquiry is needed. Owner: unassigned.
- **AI-use disclosure/declaration reconciled.** Current form matches the retained copy; university AI guidance does not settle this thesis-specific interpretation ([source check](docs/history/official-requirements-check-20261023.md)). Next: retrieve chair/examiner-specific instructions before proposing signable wording. Owner: unassigned.
- **Whole-PDF PASS by a human (Jörn or Kai).** Jörn reports only failing preliminary checks; bounded agent reviews do not establish readiness. One exact-product code route is now [source-checked](docs/history/code-contract-review-20261023.md); other code/PDF obligations remain. Next: identify surviving whole-manuscript acceptance failures. Owner: unassigned.
- **22 Oct sprint results assessed for the thesis.** Commit `d91d01b0` ("Prove affine polygon capacity extensions…"). Next: decide whether they enter the thesis. Owner: unassigned.
- **File-viewer edits landed or dropped.** Existing three-file diff inspected; 14 fixture tests pass ([review](docs/history/file-viewer-review-20261023.md)). Next: determine whether its two fixed Workbench-build targets still need the optimization; source remains uncommitted and preserved. Owner: unassigned.
- **Revised opening readability grade pending.** Jörn's original grade was FAIL (Desk `77acc843`; painful AI-style phrasing). Candidate commit `74adc710` on isolated branch `claude/opening-pass-2026-10-23` revises the full two-page opening, which still appears on PDF pp. 5–6. Root sent Desk question `d050457a-6ed3-4bad-8136-289c72ee022c` for this excerpt only. Separate Luna/medium reviewer `01a16d56-b14f-7352-958b-085fc1de7a6a` approved prose and layout after comparing result summaries with manuscript claims. The reviewer inherited parent history, and writer and reviewer both executed Luna/medium; this was neither blinded/cold nor cross-model validation. Proofs and external papers were not verified. Next: Jörn supplies a readability grade; root handles any narrow follow-up. Whole-thesis acceptance remains separate. Owner: Jörn.


## Waiting on Jörn
- Revised introduction pp. 5–6 grade (PASS/FAIL or 0–10) — Desk `d050457a-6ed3-4bad-8136-289c72ee022c`; [opens at page 5](https://joern-pc.tailc5e761.ts.net/files?project=msc-math-worktrees&path=opening-pass-2026-10-23%2Fthesis%2Fbuild%2Fmain.pdf&page=5). This is a fresh readability grade for the revised excerpt; the original FAIL on Desk `77acc843-87d1-4bdd-bedb-cb0027e6a074` remains historical state.
