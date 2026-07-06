---
name: gpt-55-prompting
description: Use when editing, reviewing, diagnosing, or substantially revising `SKILL.md` files/frontmatter, `AGENTS.md`, `.codex/agents` configs, subagent or reviewer prompts, fresh-agent review/evaluation prompts, or other GPT-5.5/Codex agent-facing instruction surfaces. Also use when drafting or evaluating prompts for GPT-5.5/Codex agents. Do not use for ordinary coding, research, or prose work unless the prompt or instruction surface is being changed or evaluated.
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
- For difficult agent-facing text, use fresh-agent review when available and
  worth the cost. Ask what was clear to that subagent, what it had to infer, what
  it misread, what seemed obvious, irrelevant, unmotivated, or too strong, and
  which constraints lack an argument that every best solution should satisfy
  them. Do not ask the reviewer to theorize about what would be clear to future
  agents; the reviewer's own read is the evidence.

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

## Subagent Review Prompts

When prompting a subagent to review something, make the review labor target the
right object under the right lens. This applies to review prompts regardless
of the artifact type; do not treat the examples in the user request, repo, or
skill metadata as a closed list of review targets.

- Name the review target and source material.
- Give the context needed to infer relevant qualities: downstream use, intended
  reader/user, ownership boundary, prior failure, or concrete decision the
  review should inform.
- State the review lens when one is known: the qualities, questions, risks, or
  failure modes the reviewer should check. Avoid bare "review this" prompts
  when a narrower or sharper lens is available.
- Do not whitelist the review so narrowly that the reviewer withholds
  unexpected important issues. Say when the named lens is a priority rather
  than the whole allowed finding space.
- Choose fresh, non-forked review only when the review question depends on the
  agent seeing the material without hidden parent-session context. Other
  reviewer context choices should follow from the review lens.
- When reviewing a prompt or instruction surface, ask what the reviewer actually
  understood, inferred, misread, missed, found irrelevant, found unmotivated,
  or treated as too strong while doing the review. Do not ask reviewers to
  speculate abstractly about future agents; the reviewer's own confusion,
  inference burden, and mistakes are the evidence.
- When the review output will be used to improve a prompt, workflow, or
  instruction surface, preserve the exact review prompt, raw review output,
  review verdict, and parent/designer interpretation as separate layers.
- For ordinary small code reviews, do not add this extra evidence structure
  when a normal findings-first review is enough.
