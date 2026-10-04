# Project-local workflow proposal

**Recommendation for discussion, 30 September 2026:** keep capabilities in this
repository, with one maintained source and thin client adapters. No migration,
new service, model pilot or thesis execution is selected by this proposal.

Use `.agents/skills/<name>/` as canonical source. Retain **chatgpt-pro,
human-prose-review, live-task-graph and review-files**. Port **harness-engineering-v2,
incident-log, problem-structuring, lesswrong-fetch and codex-usage** from the
inspected upstream commit. Add nine relative links under `.claude/skills/`,
each targeting `../../.agents/skills/<name>`, and root `CLAUDE.md` importing
`@AGENTS.md`. Record source pin, files, intentional changes and maintenance in
`.agents/skills/README.md`. Keep upstream for reviewed comparison, without a
runtime checkout, project plugin, automatic sync or changes to installed plugins.
The [migration plan](skill-migration-plan.md) supplies exact dispositions and
official discovery references; actual client discovery remains a technical check.

Preserve project contracts while adapting assumptions:

- Incident intake goes to `docs/history/incidents/intake/`; a useful structure
  map goes to `docs/coordination/structure.md`. Neither replaces `current.json`.
- Harness guidance must distinguish Codex mechanics from Claude. LessWrong
  retrieval uses GraphQL through an available HTTP client, with a labeled HTML
  fallback on failure and bounded comments.
- `codex-usage` keeps checkout-relative scripts. Default bootstrap inspects
  readiness; explicit install/session-link actions preserve existing state.
- Retain the Pro dossier and human transfer workflow, review expiry and
  credential boundaries. Replace dotfiles/other-checkout dependencies with
  local instructions in `INSTALL.md` and `docs/development-environments.md`.
  Missing host access is reported, rather than inferred from skill discovery.

The [environment contract](../development-environments.md) distinguishes one
Ubuntu workstation reached locally or through SSH, a separate Claude cloud
clone, and human-mediated Astra Pro. Shared repository files make instructions
portable; Tailscale transfer, review serving, credentials and persistent services
remain host capabilities. Cloud work returns inspectable artifacts. Pro receives
a focused ZIP and returns evidence for independent review. No cloud telemetry
export or credential forwarding is needed for this migration.

Delivery is part of acceptance. Native Codex helpers and TUI transport have
demonstrated returns; historical failure causes remain unknown. The lightweight
observer lacked native send tools. Async submissions reported accepted never
appeared for Jörn, so that question route is disabled. Use visible chat for
decisions and verify the receiving route for workers. Workstation OTel is live,
but supplies neither a semantic journal nor established billing/queue measures.
The semantic dashboard is available and technically checked; misleading workload
and completion entries have been corrected. Jörn's acceptance is not established.
These observations do not close project or workflow completion.
See the [coordination contract](README.md).

Local `codex features list`, CLI **0.159.2**, checked 30 September:

For the later release-matched feature/configuration assessment and source
navigation, use [the Codex reference](../codex.md). The table here remains the
dated observation underlying this proposal.

| Features | Local stage/state | Consequence |
| --- | --- | --- |
| `goals`, `multi_agent`, `hooks`, `apps`, `plugins`, `worktrees` | Stable, enabled | Available mechanisms; coverage still depends on role/runtime. |
| `rollout_budget`, `token_budget`, `runtime_metrics`, `agent_message_board` | Under development, disabled | No established project-wide budget or messaging enforcement here. |
| `analytics_plan_history` | Experimental, enabled | Purpose/data effects unverified; part of separately owned host drift. |

Goal token/time observations can support a scoped packet; they do not establish
whole-project billing or a universal spending stop. `codex_apps` are connector
tools; `codex_tui` controls local sessions. TUI creation/send/return worked in the
active window; endpoint lifetime after reconnect remains unverified. Retain
explicit scope, timestamps and unknown external usage.

After Jörn selects migration, one owner can prepare the nine packages/adapters
and narrow documentation changes, review source-to-candidate differences, then
integrate. Cheap acceptance comprises frontmatter/link checks, shell/Python
syntax and mocked Codex-wrapper/bootstrap checks. Fresh Codex and Claude
sessions must confirm selected local paths, explicit invocation and a useful
returned artifact; filesystem presence alone is insufficient. These checks
support bounded acceptance, not a reliability guarantee. Small migration work
does not inherently require a large spending proposal or paid behavioral suite.

The earlier source plan estimates **80–160 agent-minutes plus startup**, and
10–15 minutes of human review; these are unmeasured planning estimates, with
tokens/quota/spend unknown. Omitting `lesswrong-fetch` and `codex-usage` reduces
local maintenance but leaves those capabilities dependent on upstream setup.
No paid pilot or new service is needed for the recommended initial migration.

**Discussion cruxes:** whether the smaller package's maintenance savings justify
its remaining dependencies; whether artifact returns suffice for cloud work;
what semantic events agents should publish. Recommend nine skills, artifact
returns initially, the [two knowledge-index edits](knowledge-evaluation-plan.md)
and deferring their optional pilot. If semantic event recording is selected,
start with returned `events.jsonl` records containing UTC, task, owner, event,
artifact and observed delivery status; keep usage scope explicit and unknown
spend unknown. This is optional, not another service prerequisite. Technical
discovery, adapter compatibility, duplicate sources and host availability are
checks owned by implementers, not questions delegated to Jörn.
