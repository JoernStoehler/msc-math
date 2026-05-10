<!--
Purpose: conventions for task mini-roadmaps under tasks/.
Context: task bundles are cached project knowledge. They preserve expensive
decisions and useful agent shortcuts, but they are not ground truth.
-->

# Task Bundle Conventions

## Role

Task bundles organize current work by topic, not lifecycle. They should let a
future agent find the right source files, value judgments, and next action
without rereading the whole repo or reconstructing chat history.

Ground truth stays elsewhere:

- `tasks/verify-thesis-done.md`: once-run final thesis-done gates.
- source files and data files: code/data evidence.
- `research/*.md`: interpretation, proof-route state, decision history, and
  negative results.
- `tasks/submit-thesis.md`: submission/admin source links.

Supporting reference reports may live in `tasks/references/*.md` when they are
useful to cite from a task bundle but are not themselves a live topic roadmap.
Examples include completed merge reviews, one-off audits, and consolidated
branch records. Keep active work state in `tasks/<topic>.md`.

## Keep Or Delete

Keep a fact only if it changes a future decision, prevents a likely agent
mistake, records Jorn/Kai/external assessment, or gives a concrete resume/check
condition.

Delete stale schedule chatter, obsolete ownership, old packet queues, and
derivable state. Git history is the archive for old tracker completeness.

## First-Class Task Outcomes

Tasks may produce epistemic artifacts, not only code or prose. Common outcomes:

- write or repair a proof in `formal/` or `thesis/`
- write an experiment interpretation in `research/`
- record or revise thesis-facing obligations in a topic bundle
- run a verification packet and route each failure to a topic bundle
- get Jörn approval for theorem-strength proof or interpretation claims

Task rows should link to those artifacts. Do not bury full proofs, experiment
interpretations, or claim arguments in `tasks/*.md`.

## Required Sections

Each `tasks/<topic>.md` should use:

```markdown
# <Topic> Roadmap

## Status
- State: <active | map-input | blocked | future | stale>
- Last updated: YYYY-MM-DD
- Source surfaces: <paths>
- Refresh when: <observable trigger>

## Steering Cache
Jorn-expensive or external facts. Preserve aggressively.

## Work Map
Current mini-roadmap.

## Agent Cache
Agent-expensive shortcuts. Useful but easier to invalidate.

## Pruned / Stale
Only entries that prevent likely rediscovery.
```

## Labels

State tags:

- `[active]`: currently on the thesis path.
- `[blocked]`: cannot proceed until named blocker clears.
- `[Jorn]`: needs Jorn's mathematical, scope, taste, or advisor-context call.
- `[external]`: depends on external-world action.
- `[map-input]`: evidence/context needed before value decisions.
- `[future]`: useful after thesis closeout by default.
- `[cut]`: intentionally removed from thesis path.
- `[done]`: acceptance condition met.
- `[stale]`: retained only to prevent rediscovery.
- `[moved]`: content lives in another surface.

Value classes:

- `mainline thesis`
- `contingent during writing`
- `external clock`
- `map input`
- `future/follow-up`
- `cut/weaken`

## Cache Types

`Steering Cache` is Jorn-expensive or external knowledge: Jorn/Kai decisions,
advisor context, scope/value judgments, deadlines, university requirements, or
hard-to-reproduce steering rationale.

Steering entries should state their epistemic status in precise prose when it
matters. Examples: "Kai suggestion from email, useful external idea, not a
requirement"; "Jörn thesis-scope judgment, expected to stand unless explicitly
reopened"; "agent synthesis from repo evidence, cheap to refresh by rerunning
the linked scan." Avoid terse hard/soft/suggestion labels when those labels
hide the reason for trusting or reopening the entry.

`Agent Cache` is agent-expensive knowledge: file pointers, known commands,
failed routes, promising approaches, intermediate calculations, and grep/read
shortcuts. It can be deleted if stale, but saves agent time when fresh.

Every `Steering Cache` entry should say why it matters. Every `Agent Cache`
entry should say how to refresh it or what invalidates it.

Do not use `/tmp` files as durable source surfaces. If a scratch report changes
future task decisions, copy or summarize the relevant result into a tracked
task, reference, research, or experiment file and cite that tracked file.
