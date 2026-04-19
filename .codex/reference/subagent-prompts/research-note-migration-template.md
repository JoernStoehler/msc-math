# Spark-High Research-Note Migration Packet

Required cwd/worktree: `<WORKTREE_CWD>`

Use this worktree for all commands and edits. Do not edit `main` or the root checkout.

Read first:
- `.codex/reference/subagent-prompts/research-note-migration-context.md`

Assigned topic:
- `<TOPIC_DIR>`

Source notes to migrate:
- `<OLD_RESEARCH_FILES>`

Target local note files:
- `<TOPIC_DIR>/REASONING.md`
- `<TOPIC_DIR>/DECISIONS.md`
- `<TOPIC_DIR>/NEXT-STEPS.md`

Objective:
- Migrate the assigned old `research/**/design/*.md` notes into the new local-note structure for `<TOPIC_DIR>`.
- Preserve high-value information that is expensive to rediscover.
- Delete or ignore low-value chronology, repetition, and stale planning noise.

Required output shape:
- `REASONING.md`
  - Current reasoning about the visible code, data, results, or experiment layout in this topic.
  - Explain what the current artifacts imply.
  - Keep it readable without opening deleted historical notes.
- `DECISIONS.md`
  - Curated retained decisions, rejected routes, constraints, and advisor instructions that still matter.
  - Do not turn this into a diary or a generic changelog.
- `NEXT-STEPS.md`
  - One active forward-looking thread for this topic.
  - Include blockers, stop condition, and exact commands or files to touch when they are already known.

Scope and ownership:
- You own only the assigned topic directory and its migrated local note files.
- You may read outside the topic when needed for disambiguation, but do not edit outside your topic.
- Do not edit `.agents/skills/**`, `AGENTS.md`, or unrelated experiment packages.

Migration rules:
- Prefer concise synthesis over copy-moving prose.
- Keep only information that is hard to recover from current code, current data, git history, or parent notes.
- If an old note is fully obsolete and cheap to rederive, omit it instead of preserving it.
- If a source note contains several unrelated ideas, split them across the three target files by purpose.
- In `REASONING.md`, do not rely on `DECISIONS.md` for basic comprehension.
- In `DECISIONS.md`, record only non-obvious choices worth not forgetting.
- In `NEXT-STEPS.md`, do not accumulate background prose that belongs in `REASONING.md`.
- Keep explicit references when useful, e.g. `exact-clarke/verify_widened_seed_witness.sage` or `formal/<file>.tex:\ref{label}`.
- Prefer stable semantic statements over date-stamped status blurbs unless a date materially matters.

Success check:
- The assigned topic has the three local note files.
- The files are shorter and more useful than the sum of the old design notes.
- A future agent can work on the topic without opening the old `research/.../design/*.md` files first.

Stop condition:
- Stop and report if the topic’s old notes are too entangled with another topic to split safely.
- Stop and report if you would need to rename or move experiment directories to complete the migration.

Output format:
- Summary of files edited.
- Short list of what was preserved versus intentionally dropped.
- Commands run, if any.
