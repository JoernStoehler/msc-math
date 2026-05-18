---
name: clarify-in-chat
description: Use when a session or user request does not already have a concrete, verifiable target, especially when the agent would otherwise start work without knowing what observable outcome, acceptance check, or review criterion defines done. Not needed when the task already has a clear deliverable and verification path.
---

# Clarify In Chat

This is a rough scaffold and should be rewritten before it becomes durable
guidance.

## Content to Cover in the Rewrite

- Do not patch while intent is under dispute. Error/frustration messages are
  control-flow signals, not implementation requests.
- Do not ask "is this correct?" over a bundle of assumptions. A `no` becomes
  nearly useless.
- If many bits are missing, expose the uncertainty structure: target file,
  placement, content shape, command, wording. Ask the highest-information
  routing question first.
- Prefer preserving Jörn's proposed shape. When Jörn gives a sketch, transform
  it minimally instead of generating alternatives.
- Approval is not something Jörn owes. The agent owns producing an approvable
  understanding.
- Cover the `W` failure mode: if Jörn says he is waiting, the previous assistant
  message likely gave no actionable instruction or asked about unseen context.
- A surprised "why is X here?" should trigger conceptual re-evaluation, not
  just a local/provenance explanation.
- Do not minimize immediate task length at the cost of joint cost. Resetting a
  bad draft is often cheaper than defending it.
- Read-only exploration is allowed under "do not edit" constraints. Use it to
  reduce uncertainty without asking Jörn.
- Use exact candidate text only once the high-level axes are settled. Before
  that, candidate patches create noise.
- Fastest successful pattern from the Python-command incident: take Jörn's
  sketch, make only factual and clarity corrections, then show the result.

## Example to Reuse or Rewrite

Python-command incident output that converged:

```bash
# python
# `python` is absent on Ubuntu 24.04; `python3` lacks undeclared packages.
uv run --with pyyaml --script /home/vscode/.codex/skills/.system/skill-creator/scripts/quick_validate.py .agents/skills/clarify-in-chat
uv run --script PYTHON_SCRIPT.py # PEP 723 inline dependencies
```
