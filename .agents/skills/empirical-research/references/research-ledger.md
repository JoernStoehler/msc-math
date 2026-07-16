# Research Ledger Convention

This is a binding shared-state convention.

## What Must Be Tracked

Give every item admitted to shared/current research state a stable ID: question,
observation, hypothesis, experiment idea/proposal, assessment, result, or
decision. Ephemeral thoughts and disposable checks need no ID unless their
result changes shared state.

Do not collapse different kinds into one ontology. Retain only what another
agent needs to understand, challenge, or use the item:

- ID, kind, topic, provenance, disposition, and related IDs;
- sources and reasoning;
- for assessed proposals: question served, outcome-conditioned updates, and
  cost;
- for results: direct observation, technical/review status, and proposal link.

A materially changed item gets a new ID linked by `supersedes` or
`derived_from`; it does not inherit the old item's premises, value, review, or
authority. Clarification or disposition changes may edit the same item.

## Current Context And History

Keep current records under Git with the scientific owner. Prune items that no
longer help current agents; Git retains history. Do not load an append-only
archive into every session.

Keep a compact frontier view with:

- priority questions, active alternatives, and topic gaps;
- value/cost assessments and observed prediction errors;
- unresolved gates;
- stopped/rejected work and reasons.

Views are navigation, not evidence. Evidence remains with its producer/owner.

## Updating After Work

The orchestrator records assignments, actual cost, pointers, and technical
disposition. The lead records interpretation, belief/value changes, new ideas,
and successor proposals. Separate useful and defective parts.

Before handoff/closure, ensure loss of active sessions or scratch would not
erase material state. Session UUIDs can recover early work but do not replace
durable promoted evidence.
