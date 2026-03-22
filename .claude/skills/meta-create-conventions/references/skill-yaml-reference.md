# SKILL.md YAML Frontmatter Reference

Official docs: https://code.claude.com/docs/en/skills.md

## Required fields

```yaml
name: skill-name                    # lowercase-with-dashes, max 64 chars
description: What it does and when to use it. Include trigger phrases users would naturally say.
```

## Optional fields

```yaml
disable-model-invocation: false     # true = manual /skill-name only
user-invocable: true                # false = Claude-only, hidden from menu
allowed-tools: Read, Grep, Glob     # restrict tool access
context: fork                       # run in isolated subagent
agent: Explore                      # subagent type if context: fork
```

## File structure

```
<skill-name>/
├── SKILL.md           # Required: main instructions with YAML frontmatter
├── references/        # Optional: detailed reference material
│   └── *.md
├── examples/          # Optional: example outputs
│   └── *.md
└── scripts/           # Optional: executable scripts
    └── *.sh
```

## Checklist

Before committing a new skill:

1. [ ] Name is lowercase-with-dashes
2. [ ] Description explains what AND when (and what it does NOT cover)
3. [ ] Body is under 500 lines
4. [ ] Instructions are clear and actionable
5. [ ] Large reference material is in separate files
6. [ ] Tested with `/skill-name` invocation
