# Orchestration Cheat Sheet (Jörn)

Quick-reference for running orchestration-pattern sessions.

## Agent() Dry Run Results (2026-04-06)

Tested on Opus 4.6, 1M context. Commit `f8044b35`.

| Feature | Status | Key finding |
|---------|--------|-------------|
| Foreground Agent() | **Works** | Returns result, blocks until done. 17K tokens, 196s. |
| Background Agent() | **Works** | True parallelism confirmed. Same tools, same filesystem. No awareness of being in background. 14K tokens, 122s. |
| Worktree isolation | **Works** | Proper git branch from local main, file isolation confirmed. Auto-cleanup broken (worktree persists even after agent deletes all created files). |
| SendMessage to completed agent | **Silently fails** | Message accepted (`success: true`), but no response. Agent process already terminated. |
| SendMessage to running bg agent | **Not tested** | Should work in theory — agent process still alive. |
| Sub-sub-agents | **Not possible** | Agent() tool not available to subagents. Enforced by tool set, not convention. |
| Subagent context | **Auto-loaded** | CLAUDE.md + MEMORY.md + rules + skills all available. Same system prompt as orchestration agent. |
| Working directory | **Inherited** | Subagent gets whatever `cwd` the orchestration agent had at spawn time. Use absolute paths. |
| Post-agent hook | **Works** | Reliability notice injected into orchestration agent context after each Agent() call. |
| `model` parameter | **Available** | `"sonnet"` or `"haiku"` for trivial work. Default inherits parent model. |

### Implications for orchestration

- **No back-and-forth with finished subagents.** Coordination pattern is: spawn → get result → spawn next. Put everything the subagent needs in the initial prompt.
- **Background agents run truly in parallel.** Good for independent tasks. Orchestration agent gets notified on completion.
- **Worktree isolation for parallel writers.** Use `isolation: "worktree"` when multiple subagents edit files. Manual cleanup needed (auto-cleanup broken).
- **Use absolute paths in subagent prompts.** Don't assume subagent knows its cwd.
- **Use cheap models for trivial work.** `model: "sonnet"` for exploration, file checks, simple edits. `model: "opus"` only for tasks needing deep reasoning (proofs, complex code, architectural decisions).
