---
name: review
description: How to spawn and run review subagents. Each review subagent checks ONE convention skill against a file list — spawn multiple for multiple concerns. Load when you need to review deliverables before presenting to Jörn. Does NOT cover math correctness — use the math-review agent for that.
---

# Review Workflow

## When to review

Mandatory before presenting `.tex` deliverables to Jörn. Recommended for all deliverables.

## Two verification workflows

**Convention review** — check whether target state properties are met. Uses the `review` agent loading ONE convention skill. The conventions in the skill ARE the review specification.

**Math proofreading** — scan for known error patterns (unargued claims, missing conditions, logical gaps). Uses the `math-review` agent, which has detection patterns inline. Different workflow, different agent.

## Principle: fix syntax before semantics

Phase 1 (formatting/style) issues distract from phase 2 (content/correctness). Fix formatting first, then review semantics on clean files.

## How to run reviews

Strictly sequential across phases: all Phase 1 subagents run and findings are fixed before any Phase 2 subagent is spawned. Within each phase, subagents run in parallel.

1. Identify changed files: `git diff main...HEAD --name-only`
2. Spawn Phase 1 subagents (one per convention skill per file group)
3. Fix Phase 1 findings
4. Spawn Phase 2 subagents on cleaned files
5. Present merged report to Jörn

## Which reviews to spawn

### Phase 1 — Formatting and style (fix before phase 2)

| Files | Convention skill | Notes |
|-------|-----------------|-------|
| `.tex` in thesis/ | `tex-format` | Also run `latexmk && ./check-build.sh` first |
| `.rs` files | `rust-conventions` | |
| `.py` files | `python-conventions` | |
| Figure PNGs | Use `figure-review` agent | Specialized for visual inspection |

### Phase 2 — Content and correctness (on clean files)

| Files | Agent | Convention skill | Notes |
|-------|-------|-----------------|-------|
| `.tex` with math | `math-review` | (patterns inline) | Opus model, ONE file per spawn |
| `.tex` in thesis/ | `review` | `tex-content` | Correctness, citations, pedagogy |
| `.rs` files | `review` | `rust-tests` | Test quality, math-code correspondence |
| Experiment writeups | `review` | `experiment-conventions` | Facts vs data, interpretation quality |

Spawn one subagent per row. Multiple rows can apply to the same file — that's intentional (separate concerns, separate agents).

## Spawning pattern

```
Agent(
  subagent_type="review",
  description="Review tex format",
  run_in_background=true,
  prompt="""
    Load the tex-format skill.
    Review these files: experiments/foo/math.tex
    Report to: /tmp/review-tex-format.md
    Phase 1: fix obvious violations directly, report what you fixed.
  """
)
```

For math proofreading:
```
Agent(
  subagent_type="math-review",
  description="Proofread math",
  run_in_background=true,
  prompt="""
    Proofread this file: experiments/foo/math.tex
    Report to: /tmp/math-review.md
  """
)
```

## Scope of agent review

- Agents catch surface issues and convention violations. Jörn verifies mathematical correctness.
- Agents check test quality. Agents cannot decide which propositions need testing — that's Jörn's domain.
- `figure-review` and `figure-fix` agents handle visual inspection and iteration.
