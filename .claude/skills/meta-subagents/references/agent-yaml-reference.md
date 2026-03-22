# Agent Definition YAML Reference

Official docs: https://code.claude.com/docs/en/sub-agents.md

## File location

`.claude/agents/<agent-name>.md`

## YAML fields

```yaml
---
name: agent-name                    # Required: lowercase-with-dashes
description: What the agent does and when to delegate to it. Be specific about trigger conditions.
# Optional fields:
tools: Read, Grep, Glob, Bash       # Comma-separated; inherits all if omitted
disallowedTools: Edit, Write        # Explicitly deny tools
model: sonnet                       # sonnet, opus, haiku, or inherit (default)
permissionMode: default             # default, acceptEdits, dontAsk, bypassPermissions, plan
skills: skill-a, skill-b            # Skills to preload into agent context
---
```

## Built-in subagent types

Before creating a custom agent, consider if a built-in type suffices:

| Type | Purpose | Tools |
|------|---------|-------|
| `Explore` | Fast codebase search, file discovery | Read-only |
| `Plan` | Architecture, implementation planning | Read-only |
| `Bash` | Command execution | Bash only |
| `general-purpose` | Complex multi-step tasks | All tools |

## Checklist

Before committing a new agent:

1. [ ] Name is lowercase-with-dashes
2. [ ] Description explains what AND when to delegate (and what it does NOT guarantee)
3. [ ] Tools are minimal for the task
4. [ ] Skills preloaded via `skills:` field where applicable
5. [ ] Body is minimal — methodology belongs in skills, not inline
6. [ ] Tested via delegation
