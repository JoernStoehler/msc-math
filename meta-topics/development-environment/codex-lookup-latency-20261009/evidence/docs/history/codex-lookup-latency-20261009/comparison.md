# Decision comparison, 9 October 2026

Status: draft for owner review; no instruction activation. The earlier two-skill proposal is withdrawn. Root owns integration and any approved local installation.

## Decision and evidence

Recommend removing one source-order contradiction in the installed OpenAI Docs skill. Do not add general scope, batching or review rules: those behaviors already have adequate instructions and the root failed to follow them. This is a partial repair, not a finding that the recurring waste or 20-second acceptance criterion is solved.

The installed skill requires official search and actual page retrieval before local inspection even for a specific CLI command. The higher-priority developer instruction instead says to check local code for OpenAI usage questions, browsing as fallback. The candidate makes the lower-priority conditional skill consistent with that requirement. Current public product/API/model claims and explicit documentation requests retain their documentation route. Source findings remain qualified by revision and are not live testing.

Existing message-review already exempts direct synchronous discussion. The withdrawn formatting exception would restate a boundary rather than repair a demonstrated contradiction. Independent-read batching, task completion and user consultation also already have instructions. Their failure here is not evidence that another sentence would fix them.

## Two additional unchanged-guidance checks

These checks happened before this comparison. Both miss 20 seconds and are retained, not replaced by a passing sample. They were gpt-6.1-sol, medium, and are not a controlled comparison with every earlier trial or the root. They establish observed decisions, not isolated prompt effects.

| Worker | Final completion | Lookup rounds | Result |
| --- | ---: | ---: | --- |
| /root/unchanged_explorer_check | 34.449 seconds | 5 | Found the source test for fenced Bash copy targets after repository-navigation detours. |
| /root/unchanged_default_check | 30.706 seconds | 5 | Followed official documentation, then recommended making the entire final response Bash because the docs omitted code-block extraction. Whole-response copying can work; the answer missed the separate fenced-block option supported by the cached implementation. |

Retained native traces:

- `/home/joern/.codex/sessions/2026/10/09/rollout-2026-10-09T12-32-29-01a12038-b3f6-7fd2-962f-9234db1c8357.jsonl`
- `/home/joern/.codex/sessions/2026/10/09/rollout-2026-10-09T12-32-57-01a12039-2304-7f93-9360-5bfcd6b8ecb6.jsonl`

These two checks add another failure, not proof that changing source routing is sufficient. They should not have become another trial series; no further timing trial is selected.

## Alternatives and expected net value

| Alternative | Quality/reliability | Cost, Jörn effort and maintenance | Decision |
| --- | --- | --- | --- |
| Leave all guidance unchanged | Retains a prescribed source-order conflict; avoids speculative new behavioral rules. | No rollout cost or new maintenance. | Prefer this for review/scope/batching guidance, but not the concrete source-order conflict. |
| Revise only the conditional source contract | Permits evidence capable of answering local operation questions; preserves public-documentation boundaries. No measured latency guarantee. | Two coupled file edits; local installed override can be replaced by refresh. Needs owner review once, not routine babysitting of every lookup. | Recommended as a partial repair. |
| Add generic stop, batching and review instructions | Root already had these constraints; benefit from repetition unestablished. | More context and another competing owner. | Reject. |
| Put CLI details in thesis AGENTS.md or add an answer map | Some earlier source-discovery successes, inconsistent timing; conflicts with Jörn’s placement objection. | Irrelevant project-wide loading and maintenance of volatile details. | Reject. |
| Add a dedicated script/service or change model/effort | No evidence isolates missing mechanics or model controls as the cause. | Added mechanism, maintenance or quality tradeoff; broader intervention. | Reject. |
| Jörn supervises each lookup | Can catch bad decisions immediately; autonomy improvement unmeasured. | Repeated human interruption, contrary to the requested reduction in babysitting. | Available fallback, not claimed necessary. |

This is an expected-value judgment among these alternatives, not proof of global optimality. The independent decision reviewer /root/diagnosis_check agrees that conflict removal is justified and that latency improvement remains unproved.

## Reviewable change and rollout boundary

The exact [candidate diff](source-route-candidate/candidate.patch) revises the installed skill’s first-action order, its sole-exception wording, its source-domain restriction and the coupled self-knowledge reference. Other routes, model preservation, API-key boundaries, metadata and substantive-message review remain intact. No `/copy` facts, source-cache path, new tool or general workflow checklist are added.

Owning live paths are `/home/joern/.codex/skills/.system/openai-docs/SKILL.md` and its `references/codex-self-knowledge.md`. This is local personal skill scope across repositories, not thesis guidance. No plugin package, sibling checkout, config or model is selected for change. Installation refresh can overwrite it; retain this diff and baseline hashes for deliberate review, not automatic reapplication. Approval here would authorize only these local edits, not distribution.

Before activation, compare live files with retained baseline hashes; any intervening semantic change requires reconciliation and renewed owner review. Check patch applicability and candidate syntax/links against the installed package. After activation, inspect the exact resulting diff. Observe source selection on the next actual applicable task; do not launch more synthetic timing trials. Report recurrence when observed. These checks do not validate general autonomy or establish 20-second reliability.

Jörn explicitly objected to activation without consultation. The harness-engineering skill also separates semantic revision from owner-approved activation. Root therefore presents this concrete candidate for approval rather than silently loading it.

## Relevant standard

The loaded harness-engineering-v2 skill supplies outcome, evidence, audience/scope, owner-surface, alternatives, preservation, approval and evaluation criteria. This comparison applies them rather than using structural validation as evidence of success. OpenAI’s [evaluation guidance](https://developers.openai.com/api/docs/guides/evaluation-best-practices) separates objectives, datasets, metrics and comparison, and identifies instruction following and tool selection as separate evaluation targets. It supports evaluating actual choices; it does not establish this candidate’s performance. A current skill-building page retrieval failed; it is not cited as inspected evidence.

Static verification completed before owner request: `git apply --check` on the live baseline passed; the skill validator passed on a temporary complete package with the candidate files substituted. The live files were unchanged. Neither check establishes behavioral improvement.

## Superseded by user-directed replacement

Jörn requested removing the old skill and creating a concise personal source-routing skill with Luna triage. The pending patch was not activated. The [replacement record](replacement/README.md) owns the installed outcome and its checks.
