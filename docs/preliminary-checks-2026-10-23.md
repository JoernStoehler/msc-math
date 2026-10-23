# Preliminary checks of the thesis, as of 23 Oct 2026

Jörn, desk question 6ded353d (23 Oct): the thesis is not submitted to the
university "bc it so far only received failing grades in preliminary checks".
This file lists the checks I could find and the reasons Jörn gave. Written by
Claude on 23 Oct from repo records, the desk and `~/.codex/history.jsonl`
(Jörn's typed prompts; cited as date + session-id prefix). Mining of
`~/.codex/sessions` transcripts was skipped for time, so verdicts given only
inside transcripts may be missing. `feedback list --project msc-math` is empty.

Grading standard (Jörn, 18 Sep, `docs/history/ds-result-trial/human-feedback.md`
"Whole-PDF acceptance criterion"): PASS = ready for submission, Kai finds only
minor problems Jörn can fix by hand in about two hours; borderline is FAIL.

## Checks by Jörn (his own judgment)

| Date | Object | Verdict | Stated reasons | Source |
|---|---|---|---|---|
| before 4 Sep | earlier Sol candidates | FAIL | "zero writing of passing quality" | history.jsonl 2026-09-04 01a06b59; "improving from FAIL to FAIL" 2026-09-14 01a09bff |
| 14 Sep | thesis PDF then current | FAIL ("fails by ~most standards") | expects AI writing style to fail "hard": empty phrases, indirectness, forward references; "headache after two pages"; HKO formulas left to repo files ("sth not in the pdf is not in the pdf"); AI-reflection chapter trivially narrow | history.jsonl 2026-09-14 01a09f87, 01a09f8d |
| 14 Sep | DS two-page sample | prose passing, narrative FAIL | "lacks structure ... too chaotic to follow"; later "typical AI bad-writing pattern" (lists) | `docs/history/review-evidence/human-feedback/ds-reading-2026-09-14.md` |
| 14 Sep | HKO chapter draft | PASS | "would already be a PASS grade"; wants named lemmas, figures, annotated CAS appendix | `docs/history/review-evidence/calibration/hko-reading-feedback.md` |
| 14 Sep | Chapter 4 (QP) | PASS, writing "borderline" | stays on the math side | `.../calibration/whole-thesis-reading-1700.md` end |
| 14 Sep | abstract, intro, preliminaries (17:00/17:30 PDF) | many objections, preliminaries opener "sloppy" | wrong ratio name, contribution unclear, too much detail, retrofitted motivation, missing literature review | same file |
| 15 Sep | frozen 18:00 whole thesis (86 pp.) | FAIL, "substantially closer than prior Sol candidates" | reasons not recorded | `docs/history/memories/archive/sprint-one-todos-2026-09-16.md` l.15 (agent relay) |
| 18 Sep | DS passages and DS opening (snapshot v2) | FAIL | "too much narration of what was done instead of results + proof"; "'above' is horrible ... 'the algorithms' is vague ... FAILing grade quality in the first two sentences" | `docs/history/ds-result-trial/human-feedback.md`; history.jsonl 2026-09-18 01a0b4e9 16:26 |
| 18 Sep | whole run (93-pp. v3 diagnostic) | predicted FAIL, run stopped, never graded | "DS and AI are FAIL, but also ... lots of other stuff is FAIL" | history.jsonl 2026-09-18 01a0b4e9 16:53; `docs/history/coordination/stopped-run-handoff.md` |

No human verdict on any October build was found, including
`thesis/build/main.pdf` of 22 Oct (105 pp.).

## Checks by agents (may be wrong)

- 14 Sep, Astra assessment: "FAIL / return for major revision"
  (`docs/history/current-thesis-assessment-2026-09-14.md` l.12).
- 15 Sep, ChatGPT Pro examiner report and two self-reviews
  (`docs/history/review-evidence/pro-review-2026-09-15/`, `.../calibration/`).
  The self-review recommends against approval; DS chapter weakest.
- 22 Oct, agent review set (`docs/history/thesis-agent-review-20261022/`):
  found few surviving defects; code-contract review unfinished. This is the
  most recent check and it is not Jörn's.

## Failure causes in Jörn's words, ranked by how often he cites them

1. **AI writing style / prose quality.** Cited 4 Sep, 14 Sep (several),
   18 Sep (three times): empty or vague phrases ("above", "the algorithms",
   "was useful for"), enumerations, sloppy openers. Current tex: the DS and
   preliminaries openers he rejected were rewritten (`chapters/08-data-science.tex`
   l.6-11 now uses his own suggested sentence; `chapters/02-preliminaries.tex`).
   **Not shown addressed**: no human has read the rewritten prose; the only
   evidence is agent review.
2. **Narration instead of results; chaotic chapter narrative (DS).** 14 Sep and
   18 Sep: "narration of what was done instead of results + proof", lead with
   conclusions. Current tex: DS chapter now opens result-first (l.13-22).
   **Partly addressed in tex, unverified by Jörn.**
3. **Incompleteness: content outside the PDF or too thin.** 14 Sep: HKO
   coordinates left to repo files; AI chapter only about a sign bug. Current tex:
   `appendices/hko-certificate.tex` contains the annotated CAS verification;
   `chapters/15-ai-reflection.tex` (1378 words) now discusses delegation and
   success criteria as he proposed. **Addressed in tex, unverified by Jörn.**
4. Abstract/introduction framing (14 Sep, one session): changes applied the same
   day; revised versions not re-read by him.

## Next block (proposal)

Get one human verdict on the 22 Oct PDF before more writing. Cheapest form,
by his 18 Sep request: Jörn samples 5-8 minutes across DS, preliminaries and
the introduction of `thesis/build/main.pdf` and marks every problem he notices.
Agents then fix all marks in parallel and run a style pass for the same defect
class over the whole PDF. Cost: about 10 minutes Jörn plus a few agent-hours.
Basis: all October work has been graded by agents only, and on 14-18 Sep his
verdicts and agent verdicts disagreed.
