---
name: gpt-55-prompting
description: Use when editing, reviewing, diagnosing, or substantially revising `SKILL.md` files/frontmatter, `AGENTS.md`, `.codex/agents` configs, subagent or reviewer prompts, or other GPT-5.5/Codex agent-facing instruction surfaces. Also use when drafting or evaluating prompts for GPT-5.5/Codex agents. Do not use for ordinary coding, research, or prose work unless the prompt or instruction surface is being changed or evaluated.
---

# GPT-5.5 Prompting

For nontrivial prompt or harness work, read the target prompt surface and the
smallest relevant parts of:

- `references/gpt-55-prompt-guidance.md`
- `references/harness-engineering.md`

Use these references as GPT-5.5 behavior guidance. Reconcile them with
`AGENTS.md`, `$skill-writing`, the target surface, and Jörn's explicit
requirements; do not let generic guidance erase repo-local constraints.

- Write prompts around the intended outcome, relevant constraints, available
  evidence, success criteria, validation or review checks, and stopping
  conditions.
- Verifiers can be soft criteria when they are the right way to judge the
  outcome. Phrase them to preserve the intended difficulty; avoid criteria that
  GPT-5.5 is likely to operationalize as an easier incomplete substitute.
- Avoid step-by-step process instructions unless each step is necessary for
  correctness, safety, required tooling, or preserving the requested interaction
  contract.

- Use real observed failure scenarios when available; Jörn's memory of repeated
  failures is evidence even when no log excerpt is available.
- Review prompt and harness changes against concrete failure scenarios. Compare
  expected behavior with and without the change.
- Prefer rules that prevent the failure by changing the agent's usable context,
  success criteria, evidence, or checks, not by adding narrow process steps that
  only fit the example.

- Use `/tmp` scratch for nontrivial prompt drafting, review, or diagnosis before
  chat.
- Preserve required artifact format, frontmatter, metadata, output schema, and
  user-specified structure.
- Compare materially different phrasings when that helps.
- Check against Jörn's explicit requirements.
- Match the requested deliverable. Return a polished prompt artifact when Jörn
  asked for an artifact; return review findings, failure cases, or proposal
  options when that is the requested output.
- Show alternatives or process only when they help Jörn evaluate the prompt
  decision or when Jörn asks for them.
