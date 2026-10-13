# Research capability reference review

Reviewed `references/research-execution.md` against the existing locally acquired source corpus and the runtime collaboration schema. Scope is the reported failure mode: a BSc-scale theoretical mathematics research task ran too sequentially in one overloaded context and lost sanity checks. The reference provides a decision aid, not a universal multi-agent prescription.

## Claim checks

| Area | Evidence checked | Result / boundary retained |
| --- | --- | --- |
| Codex subagent availability and product surfaces | OpenAI acquired Subagents page, `corpus/articles/openai/chatgpt-work/chatgpt-work-docs-agent-configuration-subagents.md`; current collaboration schema in this session | Kept Work vs local app/CLI/IDE distinctions; current tool has per-agent model and reasoning-effort overrides. Documentation names configuration defaults, including the concurrent-thread setting. No numeric cap is asserted for this runtime. |
| Codex costs/context separation | Same Subagents page | Source says subagent workflows consume more tokens, noisy output can pollute main thread, and summaries return to main. Framed as possible context-management benefit with coordination/consistency risks, not a correctness guarantee. |
| Codex worktrees | OpenAI acquired Worktrees page, `corpus/articles/openai/chatgpt-work/chatgpt-work-docs-environments-git-worktrees.md` | Defined worktrees as Git checkout isolation for parallel chats/work; explicitly says this is not a context-window fix. Client-specific availability retained. |
| Codex observability | `corpus/articles/openai/codex/codex-skill-evals.md`; `corpus/articles/openai/codex/codex-agents-md.md` | Verified usage keys `usage.input_tokens` / `usage.output_tokens`, `turn.completed`, optional CLI logs, and session JSONL only when enabled. Labeled this local CLI evidence rather than a universal chat live meter. |
| Codex compaction vs fresh context | Source review across corpus; no surface-specific Codex behavior established for this claim | Used careful negative boundary: did not specify Codex trigger/preservation mechanics; distinguished conceptually and stated Claude mechanics cannot be transferred. |
| Claude Code subagents/model/effort/worktree | Anthropic acquired `claude-code-subagents.md` | Verified independent context, model/effort frontmatter, `isolation: worktree`, and plan usage consumption. Stated selected model/client/version affects available controls. |
| Claude Code teams and messaging | Anthropic acquired `claude-code-docs-en-agent-teams.md` | Verified direct teammate coordination and extra token cost; recommended only where independent teammates need to coordinate/challenge. |
| Claude context/session/telemetry | Anthropic acquired context, sessions, and monitoring-usage docs | Verified `/context`, `/compact`, JSONL transcripts, structured usage/cost outputs, OTel attributes. Kept Code surface boundary explicit. |
| Claude chat separation | `references/claude-chat.md`, based on locally acquired Help Center sources | Did not claim Claude chat has Code agent tools, filesystem worktrees, commands, or logs. Account plans and rollout can vary. |

## Quote audit

The reference uses short source quotations. Exact source text checked:

- Codex Subagents: “consume more tokens than comparable single-agent runs.”
- Claude Code Agent teams: “use significantly more tokens than a single session.”
- Claude Code context page: “Only the subagent's final text response comes back to your context” (the guide's illustrative walkthrough); the reference now uses this exact wording.

## Retrieval fit

The page is a single focused consult for diagnosis and intervention selection. It points consumers to only the small set of source records needed for audit and keeps implementation detail out of the default task workflow. The recommended math pattern is explicitly labeled as an inference from the failure report and available controls. It does not require a swarm, prescribe an agent count as a universal optimum, or equate parallelism with mathematical validity.
