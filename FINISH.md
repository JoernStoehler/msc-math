<!--
Purpose: agent-facing finish-mode map for the master-thesis closeout.
Context: this is a cached project-knowledge layer, not the literal done
truth-spec and not the task tracker. It tells agents where the done-state and
current-state surfaces live, what must be regenerated from repo evidence, and
where Jorn-only knowledge must be captured before planning content labor.
It answers these recurring questions:
- what counts as "done" for the thesis project
- where an agent should look before deciding what work is thesis-live
- what current-state knowledge is cached here to save agent/Jorn time
- what facts must not be guessed from open tasks alone
- how to refresh this file when repo state or Jorn's view changes
Writing/update rules:
- prefer current-state description over aspirational roadmap prose
- cite source surfaces and commands that let a future agent refresh the cache
- keep derived summaries short; link outward for detailed evidence
- mark missing human/external facts with `TODO(Jorn)` or `TODO(Jorn/Kai)`
- do not duplicate long tracker rows, result prose, or experiment logs
Freshness rules:
- if `FINAL-VERIFICATION.md`, `TASKS.md`, or `RESULTS.md` contradict this file,
  update this file or mark the affected section `stale`
- if a value decision changes, update both this file and the tracker row that
  makes the decision actionable
- if this file becomes a plan instead of a map, move the plan back to `TASKS.md`
  or a scoped planning note
-->

# FINISH.md

## Status

- State: finish-mode map scaffold.
- Last updated: 2026-04-24.
- Source note: created after Jorn accepted the phase-1 done-state basis on
  2026-04-24.
- Known limits:
  - The repo-state map below is not yet filled from a fresh evidence pass.
  - The Jorn-knowledge map below is a capture surface; it does not yet contain
    Jorn's answers.
  - Content and presentation labor must not be selected from this file until
    both maps are filled enough to support value assessments.

## Source Surfaces

| Surface | Role | Refresh signal |
| --- | --- | --- |
| `FINAL-VERIFICATION.md` | literal thesis-done truth-spec | done-state semantics or external closure requirements change |
| `TASKS.md` | sequencing, ownership, active/future classification | work changes owner, status, dependency, or next action |
| `RESULTS.md` | thesis claim surface and epistemic strength map | retained claims or claim strength changes |
| `thesis/submission/README.md` | submission/admin forms, source links, and external-clock TODOs | Prüfungsamt, Kai, Elizabeth, Zenodo, or upload facts change |
| `FINISH.md` | cached finish-mode map for agents | any source surface above changes the global closeout picture |

Run `bash scripts/tasks-toc.sh` after changing `TASKS.md`.

## Done-State Surface

Current status:

- Phase 1 done-state basis is accepted by Jorn on 2026-04-24, modulo inline
  external TODOs.
- The literal definition of thesis completion is in `FINAL-VERIFICATION.md`.
- `FINAL-VERIFICATION.md:T9` defines the final repo-facing closure condition:
  no direct repo-related master-thesis work remains after the final archive
  action.
- `thesis/submission/README.md` stores the current admin/source facts:
  downloaded MNTF forms, registration-form state, final handin unknowns, and
  preservation/dissemination candidates.

Current open external facts:

- TODO(Jorn): hand in the already-filled Bachelor-/Masterarbeit registration
  form after Elizabeth agrees/signs; earliest expected date from current state
  is Monday 2026-04-27.
- TODO(Jorn): verify exact Prüfungsamt copy count, form names, USB/CD contents,
  and upload mechanics from the current Ausgabebescheid / checklist.
- TODO(Jorn/Kai): choose non-GitHub preservation destination(s) before the
  final archive step. Current named candidate: Zenodo.
- TODO(Jorn/Kai): decide after Kai's review whether arXiv upload and outreach
  mails to Haim-Kislev, Ostrover, and similar researchers stay follow-up work or
  become thesis-closure work.

Do not keep elaborating the done state by guessing which content or
presentation labor is worth doing. Those decisions require the current-state
and Jorn-knowledge maps below.

## Current-State Surface

Purpose: produce a cached map that lets agents answer "what is thesis-live,
what is stale, and what should be deferred?" without rereading the whole repo or
asking Jorn broad questions.

Operating rule:

- Content and presentation labor is not selected directly from the current open
  task list.
- Before value decisions, produce two maps:
  1. repo-state map: what current files and artifacts say, what is stale, what
     is thesis-live, what is future/follow-up, and what has no current owner;
  2. Jorn-knowledge map: what Jorn knows or believes that is not yet accessible
     from repo files, including trust levels, advisor context, page-budget
     priorities, and known tempting-but-not-worth-it work.

If a candidate task appears before those maps exist, classify it as one of:

- `map input`: evidence or context needed to build the maps;
- `external clock`: action with a calendar dependency, such as the registration
  handin after Elizabeth agrees;
- `do not start yet`: content/presentation labor whose value depends on the
  maps;
- `future/follow-up`: interesting work outside thesis closeout by default.

## Repo-State Map

Fill this from repo evidence before asking Jorn to do interpretation work.

| Surface | Current repo signal | Likely thesis role | Stale/fresh risk | Next agent action |
| --- | --- | --- | --- | --- |
| `RESULTS.md` | TODO(agent): summarize claims and epistemic labels. | TODO(agent) | TODO(agent) | TODO(agent) |
| `TASKS.md` open/Jorn/blocked rows | TODO(agent): classify by mainline thesis, contingent during writing, external clock, or future/follow-up. | TODO(agent) | TODO(agent) | TODO(agent) |
| `thesis/` | TODO(agent): summarize actual chapter/source state and obvious build blockers without rewriting. | TODO(agent) | TODO(agent) | TODO(agent) |
| HKO research notes and artifacts | TODO(agent): summarize current theorem/evidence/blocker split. | TODO(agent) | TODO(agent) | TODO(agent) |
| hostile-landscape notes and artifacts | TODO(agent): summarize current retained claim candidates. | TODO(agent) | TODO(agent) | TODO(agent) |
| numerical appendix / solver notes | TODO(agent): summarize current proof-vs-validation-vs-caveat state. | TODO(agent) | TODO(agent) | TODO(agent) |
| standalone Kai-discussed results | TODO(agent): crosspolytope, visualization, pentagon-rotation status. | TODO(agent) | TODO(agent) | TODO(agent) |
| final assembly/admin | TODO(agent): summarize open external facts and mechanical checks. | TODO(agent) | TODO(agent) | TODO(agent) |

## Jorn-Knowledge Map

Fill this by asking Jorn focused prompts after the repo-state map gives enough
context. Do not ask Jorn to inventory files.

### Trust And Risk

- TODO(Jorn): Which claims do you personally trust enough to write strongly?
- TODO(Jorn): Which claims feel mathematically or numerically shaky even if the
  repo currently sounds confident?
- TODO(Jorn): Which repo artifacts do you distrust or remember as historical,
  exploratory, or superseded?

### Advisor Context

- TODO(Jorn): What did Kai seem to care about most after the 2026-04-14
  meeting?
- TODO(Jorn): What would Kai likely see as a meaningful thesis improvement
  versus optional research polish?
- TODO(Jorn): What feedback or expectations from Elizabeth are not written in
  the repo?

### Page Budget And Presentation Value

- TODO(Jorn): Which results deserve thesis spine space?
- TODO(Jorn): Which results should appear only as short current-state
  inclusions?
- TODO(Jorn): Which results are tempting but should be cut or future-labeled
  unless they become free during writing?

### Agent-Orchestration Preferences

- TODO(Jorn): Which next decisions require a single contiguous Jorn block?
- TODO(Jorn): Which decisions can be converted into review packets where agents
  prepare a concrete diff or summary first?
- TODO(Jorn): Where should agents deliberately stop and ask because the value
  judgment is not inferable from repo evidence?

## Phase-2 Completion Check

Phase 2 is complete when:

- each row in the repo-state map has an evidence-backed current signal and next
  action;
- each Jorn-knowledge section has enough answers that agents can avoid asking
  broad "what matters?" questions during writing;
- `TASKS.md` reflects the resulting mainline thesis / contingent during writing
  / external clock / future-follow-up split;
- the next step can be a three-week backchain plan rather than another
  discovery pass.
