# Submission review: what this skill demonstrably supplies

## Judgment and intended use

This is one technical-reference skill for resolving confusion about OpenAI/Anthropic products, tools, context and model-behavior evidence. Its corrective use is: a task or another skill notices a failure; this skill recovers the relevant mechanics and helps choose a maintained fix. It is not a general research planner or a promise to prevent every failure before it happens.

The revised submission demonstrates that workflow on the disclosed handoff and question/goal cases, with concrete diagnoses and maintenance artifacts. It also supplies the research/telemetry capability map needed for the fourth disclosed case. These results support usability on those cases. They do not establish performance on the user's withheld cases, a causal speedup from the skill, or live client notification repair.

## Demonstrations at downstream consultation moments

| Downstream task / consultation point | What consultation settled | Changed action / delivered artifact | What was actually verified |
| --- | --- | --- | --- |
| Prepare repo to continue on Claude chat/Cowork, after inspecting startup commands | Workstation absolute paths and personal skill resources won't travel with a checkout; Cowork is not Claude Code auto-loading; localhost isn't a guaranteed remote demo | Changed build script and repo instructions together; moved cache policy into repo; explicit reading prompt; verified pushed revision; repository ZIP/Git bundle and standalone report | Remote commit equals local HEAD, transferred contract present, report generation succeeds; fresh relocated wrapper selects writable runtime cache. Cargo stub validates invocation/path only; real Rust build unavailable |
| Diagnose required question lost after a steer, before guessing a deployment target | Async acceptance is not an answer/pause; accepted follow-ups clear TUI pending question state; reporting-style steer doesn't answer required target | Repo-local conditional recovery guide + short AGENTS pointer; required-input ledger remains unresolved; actual follow-up question prepared | Case/input IDs and cited paths checked; no target guessed. This is a recorded interaction fixture, not live UI delivery |
| Diagnose an active `/goal` loop that fails to obtain input | Slash parsing, model goal activation, active continuation and completion notification are distinct. Separate question alerts can operate during goals; Plan-mode blocking questions depend on flags. Pause/blocked/catalog restrictions matter | Conditional guidance and revised diagnosis; inspect available question route/mode/notification preferences before demanding goal exit; no fabricated catalog goal override | Tagged source paths and hashes inspected. User controls and supported catalog key identified. Live workstation config, activation trace and notification replay remain unknown |
| Diagnose slow BSc-scale research once serial walltime/context/review problems are noticed | Separate documented delegation, models/effort, contexts/compaction, worktrees, shared quota and telemetry capabilities by surface | Focused research reference + trace attribution guide and helper; no arbitrary swarm-size cap or new research-program mandate | Capability claims linked to primary docs; synthetic trace correctly keeps parallel overlap and unobserved time. No mathematics-performance experiment was run |

Artifacts are under `examples/repository-handoff/` and `examples/input-and-goal-recovery/`. The user's example cases are disclosed development cases, not held-out evaluations.

## Low-cost access and measured consultation

A known incident can go directly to `scripts/consult.py handoff`, `goal`, `questions`, `research` or `traces`; no web search or corpus-wide reading is required. The handoff page is about 650 words. The goal page is under 1,000 words and includes the additional attention-path finding. Primary docs/tagged source are retained when a contested detail needs audit.

In the first assisted handoff, the agent read 17,918 bytes of five skill/reference pages plus two fixture documents. In the initial corrective diagnosis it read 20,330 bytes; the subsequent attention-path correction consulted 10,374 further bytes. These are reported file-read volumes, not total model context/token measurements. The routing was simplified afterward to avoid reading README, index and the whole correction register for a known case. No measured end-to-end speedup is claimed.

## Negative evidence and revisions

- The no-skill handoff baseline also repaired the cache path, pushed its fixture remote and exported a report. Both baseline and assisted wrappers passed the same relocated-cache invocation check. The comparison therefore does not establish that this fixture needs the skill or that the skill is faster. The assisted output explicitly names AGENTS.md and repo-contained cache policy in the receiving prompt, improving the auditable handoff contract.
- In the earlier A/B/C layout comparison, all three agents omitted the Work restriction when recommending project-only memory. All three misstated the Skills API's 25 MB per-file limit as a total uncompressed-content limit. Some alleged quotes were paraphrases or mis-cited paths. Raw answers are retained in `evidence/decision-evaluation/`; these are not presented as uniformly successful answers.
- The first goal diagnosis was too narrow: ordinary completion alerts are suppressed during active goals, but there are separate question attention paths. The source audit discovered that distinction, and the guidance/fixture diagnosis was revised. It does not assert that asking requires ending every goal.
- A diagnostic citation used `references/model-catalog-review.md` although the file is under `evidence/`. It was corrected and cited paths were checked. This is recorded as an evidence-quality defect, not hidden behind a test count.
- No comparative task-success gain was demonstrated for putting every article into correction-enriched views. Canonical ownership stays corpus plus curation and immutable raw responses; generated views are a convenience, not an empirically superior retrieval architecture.

## Maintenance choices exposed by the goal case

A short AGENTS pointer changes discovery; a conditional recovery guide holds rare feature detail without bloating every task. A catalog description override is usable only for a supported key: the inspected `models[].model_messages.tools.send_user_message_async.description` can clarify async attention behavior, but there is no catalog tool-description field for `create_goal`/`update_goal` in this tag. Full catalog loading is startup-scoped; it cannot change runtime continuation or notification gates. A confirmed runtime defect belongs to a harness/product change. Those distinctions prevent expensive, ineffective fixes.

The example recovery guide is a proposed intervention artifact in a fixture, not an extra skill installed for the user. No global AGENTS, user catalog, live account setting or recurring job was changed.

## What remains for independent grading

Run the user's withheld tasks with the installed skill on the intended client, retaining exact consultation/activation traces, user interventions and artifacts. Live Cowork handoff needs actual input access, tool prerequisites and UI presentation; live goal/question recovery needs the user's mode, feature flags, notification settings and observed behavior. The skill records these missing facts instead of converting source plausibility into runtime proof.

Model-behavior material is deliberately incremental: documented/vendor claims, user reports, executed observations, comparisons and hypotheses stay separate. `model-evidence.md` defines how later work can add our own findings without promoting an attractive story into established fact.
