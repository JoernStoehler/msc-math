<!--
Purpose: phase-2 finish-mode state capture before the final three-week plan.
Context: created after the done-state basis in FINAL-VERIFICATION.md was
accepted on 2026-04-24. This file is the working surface for repo-state and
Jorn-knowledge migration before deciding which content/presentation labor is
worth doing.
-->

# Finish Current State And Jorn Knowledge Map

Status as of 2026-04-24:

- Phase 1 done-state basis is accepted modulo inline external TODOs:
  `FINAL-VERIFICATION.md`, the Finish Mode block in `TASKS.md`, and
  `thesis/submission/README.md`.
- Do not further flesh out the done state by guessing what presentation or
  content labor is worth doing.
- Before value decisions, produce two maps:
  1. repo-state map: what current files and artifacts say, what is stale, what
     is thesis-live, what is future/follow-up, and what has no current owner;
  2. Jorn-knowledge map: what Jorn knows or believes that is not yet accessible
     from repo files, including trust levels, advisor context, page-budget
     priorities, and known tempting-but-not-worth-it work.

## Operating Rule

Content and presentation labor is not selected directly from the current open
task list. It is selected only after the repo-state map and the Jorn-knowledge
map are good enough that a future agent can trace retained thesis content back
to value assessments.

If a candidate task appears before those maps exist, classify it as one of:

- `map input`: evidence or context needed to build the maps;
- `external clock`: action with a calendar dependency, such as the registration
  handin after Elizabeth agrees;
- `do not start yet`: content/presentation labor whose value depends on the
  maps;
- `future/follow-up`: interesting work outside thesis closeout by default.

## Repo-State Map Skeleton

Fill this from repo evidence before asking Jorn to do interpretation work.

| surface | current repo signal | likely thesis role | stale/fresh risk | next agent action |
| --- | --- | --- | --- | --- |
| `RESULTS.md` | TODO(agent): summarize claims and epistemic labels. | TODO(agent) | TODO(agent) | TODO(agent) |
| `TASKS.md` open/Jorn/blocked rows | TODO(agent): classify by mainline thesis, contingent during writing, external clock, or future/follow-up. | TODO(agent) | TODO(agent) | TODO(agent) |
| `thesis/` | TODO(agent): summarize actual chapter/source state and obvious build blockers without rewriting. | TODO(agent) | TODO(agent) | TODO(agent) |
| HKO research notes and artifacts | TODO(agent): summarize current theorem/evidence/blocker split. | TODO(agent) | TODO(agent) | TODO(agent) |
| hostile-landscape notes and artifacts | TODO(agent): summarize current retained claim candidates. | TODO(agent) | TODO(agent) | TODO(agent) |
| numerical appendix / solver notes | TODO(agent): summarize current proof-vs-validation-vs-caveat state. | TODO(agent) | TODO(agent) | TODO(agent) |
| standalone Kai-discussed results | TODO(agent): crosspolytope, visualization, pentagon-rotation status. | TODO(agent) | TODO(agent) | TODO(agent) |
| final assembly/admin | TODO(agent): summarize open external facts and mechanical checks. | TODO(agent) | TODO(agent) | TODO(agent) |

## Jorn-Knowledge Map Skeleton

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

## Completion Check For Phase 2

Phase 2 is complete when:

- each row in the repo-state map has an evidence-backed current signal and next
  action;
- each Jorn-knowledge section has enough answers that agents can avoid asking
  broad "what matters?" questions during writing;
- `TASKS.md` reflects the resulting mainline thesis / contingent during writing
  / external clock / future-follow-up split;
- the next step can be a three-week backchain plan rather than another
  discovery pass.
