# CLAUDE.md

## Knowledge Placement

**When you produce new knowledge** (findings, conventions, docs, comments):
- Tied to a specific file or function? → code comment, doc comment, or file header. This is the natural location agents look at when working with that code.
- Applies to most agents? → CLAUDE.md.
- Applies to a minority of agents? → `.claude/skills/*/SKILL.md` (progressive disclosure: name + description always loaded, body on demand).
- Project management (tasks, ideas, deferred work, constraints)? → `TASKS.md` (root). Grows stale; that's fine.
- Session learning or cross-session state? → `MEMORY.md`. Migrate stable entries to CLAUDE.md or standard locations.
- Don't dump unrelated knowledge into README.md files. Each README covers its own directory's purpose.

**When you need knowledge you don't have:**
- Check code comments, file headers, and README.md in the relevant directory first.
- Check CLAUDE.md (you already have it in context — search for keywords).
- Check skill names and descriptions — load the skill if it matches your need.
- Check `TASKS.md` for project-level context (what's planned, what's deferred, why).
- Check `papers/` for referenced paper sources when verifying math or citations.
- Check `.devcontainer/` for environment details (what's installed, how sessions run).

**When editing CLAUDE.md, SKILL.md, or agent prompt files:**
- Load the `writing-conventions` skill first. It contains the rationale, style rules, and cross-reference tag system.
- Editing CLAUDE.md or agent prompts without loading the skill risks breaking conventions that are expensive to detect later.

**Agent prompt architecture:** Subagent definitions in `.claude/agents/*.md` 1:1 copy relevant CLAUDE.md sections into their prompt body. This duplication is intentional — agents reliably follow inline instructions but unreliably follow "go read file X." Cross-reference tags (`<copied-to>` in CLAUDE.md, `<copied-from>` in agent files) track which copies need updating. Details in the `writing-conventions` skill.

## Communication with Jörn

**Before requesting Jörn's attention:** Investigate first. Autonomous investigative work is basically costless. An investigation is worth doing if it either resolves the problem without Jörn, or speeds up Jörn's investigation via a report with preliminary findings.

**When requesting Jörn's attention:**
- Describe the narrowly scoped cognitive task Jörn should do
- Say why Jörn should do it instead of you
- Provide the context it exists within — Jörn usually drops in without working memory of your session
- After pauses in discussion, re-provide session context. Jörn switches between multiple agent sessions and does not monitor what agents do.

**Formatting for efficient exchange:**
- Number items so Jörn can respond "3 yes, 5 no" instead of quoting paragraphs
- Omit filler phrases — aim for efficient information exchange, not politeness
- When presenting decisions with tradeoffs: use tables, quantify costs/benefits, state recommendation upfront
- When you make repo changes Jörn should know about, mention and explain them — Jörn reviews diffs in VS Code but may not check them unprompted

**Interaction dynamics:**
- Push back on contradictions, gaps, unclear statements, and oversights. Jörn is not infallible — he sometimes makes ambiguous typos or has brainfarts — and he welcomes pushback.
- Never take silence as confirmation. Especially during fast-paced back-and-forth where Jörn may respond to only parts of messages, or respond with delay.
- **Word-choice sensitivity:** Jörn communicates distinctions via subtle word choices that agents tend to gloss over. When Jörn says "not quite" and corrects a nuance, the specific words he chose carry meaning. Don't paraphrase corrections back into your original framing — adopt his exact phrasing and check whether you lost a distinction.

## Staying Focused Across Long Sessions

**Plan file as persistent memory:** Update the plan file as you work — it survives context compaction, your working memory does not.
- After completing an item: mark it done, note any surprises or context future items need.
- Before starting a new item: record what you're about to do and why.
- When discovering context relevant to upcoming items: write it into the plan now, not "later."
- When you need something to survive a session boundary or compaction: put it in the plan file.

**What gets lost at compaction** (danger ranking, most to least dangerous):
1. **Scheduled items you haven't started** — you forget they exist and they never get done
2. **Context and considerations for upcoming items** — you redo them from scratch or miss nuances
3. **Completed items** — low cost, already done, only needed for final reporting

**Session recovery after compaction or handoff:**
- If you suspect you lost context: check the plan file first, then MEMORY.md.
- If you need details from the pre-compaction conversation: delegate JSONL transcript reading to a subagent. Never read the transcript yourself — it's too large and wastes your context window.
- Never guess about what happened pre-compaction — verify or say "I don't know."
