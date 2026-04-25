---
name: cached-map-maintenance
description: Use when creating, refreshing, pruning, renaming, splitting, deleting, or reviewing cached map files such as AGENTS.md map sections, ROADMAP.md, crates/MAP.md, experiments/MAP.md, tasks/README.md, research/README.md, or future subtree MAP.md files.
---

# Cached Map Maintenance

This skill maintains agent-facing cache maps. A cached map helps an agent decide
where to look next without rereading the whole repo. It is not the source of
truth unless the file explicitly owns a convention or steering decision.

## Map Types

Classify the file by the question it answers for a using agent:

| Type | Files | Agent question | Authority |
| --- | --- | --- | --- |
| always-loaded table of contents | `AGENTS.md` map sections | What repo is this, what invariants apply, and where do I look next? | root instructions plus discoverable skills |
| global routing map | `ROADMAP.md` | Which closeout stream am I in, and which task bundle starts the work? | `tasks/*.md`, Jörn/Kai/external decisions |
| subtree navigation cache | `crates/MAP.md`, `experiments/MAP.md`, future subtree `MAP.md` files | Which local subsystem, package, entity, or artifact surface should I inspect first? | code, package manifests, local headers, research/task notes |
| convention/index map | `tasks/README.md`, `research/README.md` | How do I read or edit this directory, and what does each indexed file own? | accepted conventions plus actual file usage |
| generated diagnostic | none currently | What did a generator observe? | generator output; delete when grep/local inspection is cheaper |

## Workflow

Use this loop for both first drafts and updates. For updates, start from the
existing map and preserve only the parts that still answer the agent question.

1. Name the map type and the agent question.
2. Name the authority surfaces that overrule the map.
3. Pick a small read set: headers, manifests, nearby `README.md` / `MAP.md`,
   relevant task or research notes, and targeted `rg` queries.
4. Patch the map as a cache:
   - keep entrypoints, ownership boundaries, navigation shortcuts, source
     surfaces, refresh triggers, and known open edges;
   - link to source files instead of copying local implementation detail;
   - mark uncertain or historical facts in precise prose;
   - delete derivable prose and stale plans.
5. Route displaced content:
   - active work -> `ROADMAP.md` or `tasks/*.md`;
   - research interpretation or proof-route state -> `research/*.md`;
   - repeatable check procedure -> a skill or verification packet;
   - local implementation detail -> file headers or local docs;
   - obsolete generated/global cache -> delete after git captures rollback.
6. Run checks:
   - `git diff --check`
   - `bash scripts/toc.sh <changed markdown files>`
   - targeted stale-reference `rg`
   - skill validation if a skill changed.

## Structure Defaults

For subtree `MAP.md` files, prefer:

```markdown
# <Subtree> Map

## Status
- State:
- Last updated:
- Source surfaces:
- Refresh when:

## Map Type And Authority
- Type:
- Agent question:
- Authority:
- Non-authority:

## Role
## <Main Navigation Table>
## <Key Concepts Or Boundaries>
## Open Edges
```

Use different sections when the map type needs them, but keep the file short
enough that an agent can skim it before opening local sources.

## Reference Style

- Cite stable paths, symbols, labels, commands, and section names.
- Avoid line references unless the line is the point of the claim.
- Use wildcard notation only for source-surface descriptions, not as a hidden
  proof that the map was exhaustively verified.
- If a fact came from a dated migration or snapshot, include the date and say
  what would refresh it.
- Do not cite scratch files as current authority unless the task explicitly
  promotes them.

## Coordination

- Use `$roadmap-maintenance` as well when editing `ROADMAP.md`, `tasks/*.md`, or
  task-bundle conventions.
- Use `$research-direction` as well when a map change depends on research
  interpretation, proof-route status, or thesis story framing.
- Use `$harness-engineering` as well when changing `AGENTS.md`, skill routing,
  or other agent harness behavior.

## Stop

Stop and ask Jörn when:

- a map would encode a new thesis-scope, mathematical, advisor-facing, or
  deadline decision;
- a descriptive cache would become a policy decision about API stability,
  canonical data ownership, or what agents should prioritize;
- a rename or deletion would remove the only current pointer to active work.
