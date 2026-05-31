# Codex Capability Skill Pruning Decisions

Purpose: record Jörn's reactions to candidate `SKILL.md` content before any
active skill is installed.

Status: drafting reference. This file records review decisions and unresolved
questions. It is not itself a skill and should not be treated as final
instruction material.

Source files:

- `/tmp/codex-capability-knowledge-transfer-items.md`: overcomplete sentence
  pool.
- `/tmp/codex-capability-pruning-review.md`: scratch review sheet.

## 2026-05-19 Round 1

Source type: Jörn chat reaction to pruning review items A1-A6 and B1.

### A. Bulk Decisions

A1. Purpose-of-skill material can be compact in the final skill.

- Jörn verdict: yes if this means purpose of the skill.
- Jörn note: the complexity is probably capturable in one paragraph.
- Jörn caveat: do not commit to the count `2-4`; decide after seeing the
  actual draft.

A2. Low/high task-class ratings belong in references, not the main skill body.

- Jörn verdict: yes.
- Jörn note: use one reference file containing a big table plus an explainer.
- The explainer should state that the numbers are Jörn ass-numbers and record
  the definitions/process that produced them.

A3. `K*` candidate skill design items are build notes, not final skill content.

- Jörn verdict: mostly yes.
- Jörn note: `SKILL.md` should not talk about how it was built.
- Build notes may live in references, be lost, or be treated as part of
  `$harness-engineering`-type process knowledge.

A4. Deep-research report should not be a main skill premise.

- Jörn verdict: yes.
- Jörn note: it is borderline not worth mentioning.

A5. Preserve Jörn raw examples in references.

- Jörn verdict: yes.
- Jörn note: better if Jörn rewrites raw examples directly, so nuance is not
  accidentally lost.

A6. "Routing skill, not capability encyclopedia" is unresolved as written.

- Jörn verdict: too vague.
- Jörn note: saving about 10 seconds of reasoning for future agents is less
  important than preventing wrong reasoning by agents who actually need
  capability knowledge.
- Jörn note: the table with ass-numbers is load-bearing information to
  transmit, in addition to safe-enough heuristics.
- Current implication: do not reduce the final skill to mode names alone.
  Ensure agents can find and use the ratings table when capability knowledge
  matters.

### B. Work Modes

B1. Include a mode list in final `SKILL.md`.

- Jörn verdict: unclear.
- Jörn question: what does "modes" mean here, and what is the purpose of
  including them?
- Current implication: do not use the term "modes" in final material unless
  it is replaced by a plainer phrase or explicitly justified.

## Open Decisions

- Whether the final skill needs a compact embedded table, a reference-table
  pointer, or both.
- How much of the ratings-table explainer belongs in `SKILL.md` versus the
  ratings reference.
- How to phrase the routing menu so it is not misleadingly presented as a
  list of actions.
- Whether any state-label abstraction should exist at all.
- How to avoid a "rules" frame that suppresses useful implicit GPT-5.5
  judgment.

## 2026-05-19 Round 2

Source type: Jörn chat reaction to B1'-B5.

### B. Next Actions / Work Shapes

B1'. Include a short next-action menu in final `SKILL.md`.

- Jörn verdict: yes.
- Jörn note: this is useful for keeping `SKILL.md` focused on being useful.
- Jörn caveat: the candidate items/actions are not written in a form that he
  can comment on yet.
- Current implication: rewrite the next-action list in plainer and more
  concrete terms before asking for item-level approval.

B2. Include knowledge-extraction work.

- Initial Jörn reaction: tentatively no if this means harness engineering for
  writing up GPT-5.5 capability knowledge, or a specialized workflow for some
  other target.
- Revised Jörn reaction: yes, the concept needs to be communicated somewhere.
- Jörn clarified target:
  - extracting experience that Jörn has
  - extracting task understanding that Jörn has
  - extracting external context that Jörn has access to but agents do not yet
  - making extracted knowledge accessible inside the repo to future agents
- Current implication: include the concept, but probably not as a heavyweight
  named mode. It may be a next-action option or a reference-only workflow.

B3. Include context-durability work.

- Jörn reaction: this became clearer after resolving B2.
- Current implication: treat context-durability as part of the knowledge
  extraction path: if needed knowledge currently lives only in Jörn/chat/temp
  context, make it accessible in repo files for future agents.

B4. Include prompt-writing work.

- Jörn verdict: unresolved.
- Jörn distinction:
  - knowing what to put in the prompt
  - style help for succeeding at knowledge transfer
- Jörn note: the second item might be only about four plain sentences, but
  this is not yet known.
- Current implication: before deciding final skill content, check how much
  extra complexity prompt-writing guidance adds beyond:
  - knowing what can be delegated
  - knowing what makes a task delegatable
  - turning non-delegatable task knowledge into a delegatable task description

B5. Do not define all next-action items in equal detail.

- Jörn verdict: yes.
- Jörn note: never balance lists or optimize for visuals or homogeneous format.
- Jörn priority order: correct, specific, clear, unambiguous,
  low-cognitive-load, useful/relevant, maintainable.
- Current implication: final skill may use uneven item lengths and omit
  symmetry if that improves information transfer.

## 2026-05-19 Round 3

Source type: Jörn chat reaction to batch 2 next-action menu.

### Routing Menu Shape

The candidate "next-action menu" is directionally useful, but "Use when" is an
awkward frame because not every entry is an action.

Jörn notes:

- Some entries are really "scoping is already done; just work on the task".
- "Focus on the task" is odd to phrase as an action, because it means there
  is no additional scoping move to take.
- A better first entry would say roughly: scoping has already been done; work
  on the task by exploring, comparing approaches, planning, implementing,
  reviewing, iterating, and handing back the finished deliverable.
- The ask-Jörn case should say roughly: information is missing that is not
  discoverable in the repo, or not cheaply discoverable relative to asking
  Jörn, and it is worth asking up front instead of waiting for final review.

Current implication:

- Replace "next-action menu" with a plainer concept such as "routing outcomes
  after initial scoping".
- Do not force every entry into "do X now" grammar.
- Phrase the direct-work case as the default state where no extra scoping move
  is needed.
- Favor state/outcome conditions over process descriptions. Jörn notes that
  GPT-5.5 often does not profit much from process descriptions; it is more
  useful to make the state clear, e.g. "scoping is done", so the agent can
  notice that no extra scoping work is needed.

### No Formal State-Label Layer

Source type: Jörn chat reaction.

Jörn does not currently see enough advantage in formal labels. Trying to
formalize the situations risks adding complexity from abstraction and
maintenance burden from content/invariants.

Current implication:

- Do not introduce a formal "state labels" vocabulary.
- Prefer plain situational headings if needed, e.g. "if verification is
  unclear", rather than new terms with invariants.
- Any grouping in the skill must earn its keep by making the final text
  shorter, clearer, or harder to misuse.

## 2026-05-19 Round 4

Source type: Jörn chat reaction to `/tmp/codex-capability-core-rules.md`.

### Avoid "Rules" Framing

Jörn disliked the frame of "rules". Agents should continue to use their
non-explicit judgment. The issue is not that GPT-5.5 has no useful internal
judgment; the issue is that expressed self-assessment about future capability
is overconfident under bad RLHF/RLVR habits, especially when the task is
ambitious or unusual and lacks a strict feedback mechanism.

Candidate skill sentence accepted directionally by Jörn:

- "Self-assessment by GPT-5.5 about its own capabilities is badly calibrated
  and directionally overconfident for ambitious or unusual tasks that lack a
  strict feedback mechanism."

Current implication:

- Reframe `/tmp/codex-capability-core-rules.md` as knowledge items, not rules.
- Do not tell agents to ignore their own judgment globally.
- Tell agents not to over-weight expressed self-assessment as evidence for
  capability when task feedback is weak.

## 2026-05-23 YAML And Purpose Approval

Source type: Jörn chat approval.

Jörn approved the YAML frontmatter and `Purpose` section in
`/tmp/codex-capability-SKILL-draft.md` 1:1.

Current implication:

- Do not keep reworking the approved YAML/purpose unless a later body decision
  creates a concrete mismatch.
- Discuss and draft the body next.
- Long tables or material not needed by every reader can move to
  `references/*.md`.
