# Orchestration Cheat Sheet (Jörn)

Quick-reference for running orchestration-pattern sessions.
Baseline commit (pre-orchestration): `f8044b35`. Current infra committed on `2270ed64`.

## Starting a Session

1. Open a new Claude Code session
2. Type `/orchestrate` to load the orchestration agent role
3. Describe the task — the agent will enter Plan mode and start decomposing

## `/compact` text

When context gets large and you want to compact at a natural boundary:
```
/compact "Preserve: plan file path, task graph status, pending agent results, Jörn decisions made this session, current worktree/branch"
```

## Agent() Dry Run Results (2026-04-06)

| Feature | Status | Finding |
|---------|--------|---------|
| Foreground Agent() | **Works** | Blocks until done. ~17K tokens, ~3 min for a simple task. |
| Background Agent() | **Works** | True parallelism. Same tools/filesystem. No awareness of being in background. |
| Worktree isolation | **Works** | Proper git branch from local main. Auto-cleanup broken (worktree persists even if agent deletes all files it created). |
| SendMessage to completed agent | **Silently fails** | Message accepted, no response. Process already terminated. |
| Sub-sub-agents | **Not possible** | Agent() tool not available to agents. Enforced, not convention. |
| Agent context | **Auto-loaded** | CLAUDE.md, MEMORY.md, rules, skills all available. |
| Working directory | **Inherited** | Agent gets the orchestration agent's cwd at spawn time. |
| `model` parameter | **Available** | `"sonnet"`, `"haiku"`, or `"opus"`. Default inherits parent. |

## Key Constraints

- **No follow-up to finished agents.** Put everything in the initial prompt. Pattern: spawn → notification → spawn next.
- **Background for parallelism.** `run_in_background: true` keeps the session responsive.
- **Worktree cleanup is manual.** After merging, remove worktrees with `git worktree remove`.
- **Absolute paths in prompts.** Agents inherit cwd, which may not be the repo root.

## Decisions Made

- **Agent() over Teams**: Teams add protocol overhead (idle management, shutdown) that burns agent attention. Agent() is simpler and sufficient for orchestration.
- **"Orchestration agent" / "agent" terminology**: Matches Anthropic's naming with added specificity. "Subagent" avoided in favor of just "agent" (Jörn's preference).
- **Delegation guide as reference file**: Agents Read() it on demand rather than loading into SKILL.md (keeps SKILL.md short, delegation guide can grow with examples).
- **Skills deleted**: download-paper, experiment-design, project-management-partner, test-design, thesis-writing, handoff — content was either obvious to agents or captured in CLAUDE.md/rules.
- **math.tex → main.tex**: Root aggregator renamed, build switched to latexmk.
- **Cargo.toml moved to crates/**: All cargo commands run from `crates/`.
