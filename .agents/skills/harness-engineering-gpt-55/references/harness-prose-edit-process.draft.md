# Harness Prose Edit Process Draft

<!--
Working draft; not active policy. This records the edit loop to use during the
current harness migration so prose does not drift into chat-only terminology or
generic instruction-doc polish.
-->

## Objective

Harness prose should change future agent behavior in this repo. It should not
make agents decode local shorthand, migration history, or invented terminology
before they can act.

## Before Editing

For each proposed sentence or bullet, answer these questions:

- What should a future agent do differently because this text exists?
- What concrete misread, bad edit, or wasted action does it prevent?
- Can the text use ordinary words and direct verbs instead of a coined label?
- Is this in the file or section Jörn asked to edit?

If the answer is unclear, do not write polished filler. Leave a `TODO` or ask.

## Wording Rules

- Prefer sentences of the form "agents can..." or "put X in Y when..." over
  named abstractions.
- Do not introduce a compact term unless it is standard, already used in this
  repo with the same meaning, or explicitly defined because the file needs it.
- If a phrase explains what a coined term means, delete the term and write the
  meaning directly.
- Put the instruction itself in visible prose.
- Put rationale in `<!-- comments -->` only when the convention would otherwise
  invite a wrong explanation.
- Move long or history-heavy rationale to an adjacent draft/reference file
  instead of inline comments.

## After Editing

- Scan the touched section for sibling defects, not only the exact word Jörn
  pointed out.
- Search for likely abstraction drift: `surface`, `layer`, `routing`,
  `authority`, `scope`, `material`, `artifact`, `target`, `first`, `shape`,
  `ownership`, and similar vague nouns.
- Read the touched section as a fresh agent that only has `AGENTS.md`, the
  skill description, and this file.
- Keep the edit scoped unless Jörn explicitly asks for a wider cleanup.

## Failure Mode To Avoid

Do not optimize for a document that sounds systematic. Optimize for future
agents doing the right repo work with fewer wrong guesses and less reasoning
about the harness itself.
