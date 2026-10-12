# Resource workflow review candidate, 6 October 2026

Codex continued the parent skill review after Jörn challenged stopping at the investigator acknowledgment. The [complete resource candidate](codex-resources/SKILL.md) is now concrete and reviewable outside runtime discovery. It covers existing read-only routes, allocation, explicitly authorized action, scheduling and recovery while stating the missing executor/controller capability. No activation or live account operation occurred.

## Recommendation and alternatives

Start with `codex-resources`, one sectioned related workflow. Keep Claude-cloud CLI setup/execution separately owned and rename its current broad identity to `codex-cli-from-claude` when the selected migration is integrated. The latter describes the existing consumer and capability without suggesting account quota management.

| Naming alternative for the current cloud skill | Assessment |
| --- | --- |
| codex-usage | Broad and confusable with resource tracking; preserves the reported naming defect |
| codex-cli-from-claude | Names the existing consumer/workflow and distinguishes resource management; preferred candidate |
| claude-codex | Shorter but ambiguous about direction and transport |
| codex-cloud-bootstrap | Identifies setup but obscures its task execution wrapper; cloud is also underspecified |

| Resource organization | Assessment |
| --- | --- |
| One complete sectioned codex-resources | Supplies normally connected observation/action/recovery knowledge and preserves quality-first staging; read-only requests load some irrelevant action detail |
| Tracking skill plus resets skill | More discriminating actions and potentially cheaper reads; adds cross-skill discovery and authority/state/recovery handoff obligations; retain for later behavior/cost comparison |
| Expand the current cloud CLI skill | Unrelated consumer/lifecycle, reinforces the bad name; reject as the default |

This is an expected-benefit judgment grounded in the current use cases and accepted design heuristic, not a behavioral experiment or proof of near-optimality. No question about thread organization needs Jörn's input.

## Content review against representative requests

These are explicit checks of the candidate's directions, not fresh-agent forward tests.

| Request / evidence | Candidate directs | Assessment |
| --- | --- | --- |
| How much weekly quota remains? | Read the quota helper; report timestamp/server scope; no redemption/model turn | Gives the existing direct route; no needless local transcript accounting |
| What did this assignment consume? | Scoped local usage reader and its coverage; no percent-to-token conversion | Avoids equating account-wide change with the assignment |
| A returned credit expires soon; a matching schedule is already authorized | Route the action under existing authority, preserving selected credit/account and timing | Avoids another approval gate merely because a check discovered urgency |
| Check inventory, with no action selected | Read-only result; present a consequential action choice only if needed | No silent consume or invented scheduling |
| Explicit redeem-now request for a selected credit | Real CLI/demonstrated equivalent, private selection and a recovery owner | Does not let omitted creditId silently select an alternate |
| Request succeeded but transport lost the reply | Reconcile/retry the same logical attempt identity | Distinguishes transport ambiguity from a clear refusal |
| Clear refusal, with recovery selected | Diagnose/current-state reconciliation or available selected fallback; preserve ownership | Neither refusal label nor early timestamp establishes completed recovery |
| Timed fallback can only rerun the pinned failed script | Report executor/diagnostic gap and prepare a capable selected route before promising recovery | Exposes the concrete October failure instead of hiding it behind a timer |
| Timer fires / child sends an acknowledgment | Retain parent action/recovery obligation | Component/receipt completion cannot close the due redemption objective |
| Selected credit expires without confirmation | Supported failure/expiry report and explicit ownership release | No dummy work, purchase or alternate credit |
| Ordinary Codex setup or incidental token mention | Candidate trigger excludes it; existing cloud CLI owner remains relevant where appropriate | No universal resource ritual |

## Implementation and migration limits

The resource candidate is a guidance baseline, not an installed capability. Current scripts implement inspection/accounting; no reusable production redemption/recovery controller was added. A future selected controller needs mock-tested refusal, ambiguous success, new-attempt and expiry branches plus a real receiving/notification route. Official protocol response definitions do not establish production eligibility or refusal cause.

A cloud-skill rename has a concrete migration surface: directory/name, interface invocation, Claude discovery link, callers in communication/environment/ownership docs, and maintenance links. Preserve scripts and per-grant authority; do not repurpose the workflow. Repository candidates and historical evidence remain historical names. No global cache or host configuration change is included here. The candidate draft does not create a required user approval question or activate the separate production reset proposal.

No current human input is required to read these owners, compare names/layouts, or assess these cases. Codex can prepare bounded independent forward tests when needed. Jörn's help is warranted when a specific test/result exposes an unresolved intended outcome or criterion, rather than for ranking abstract architectures in advance.
