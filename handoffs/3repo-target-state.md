# Target State: Agent Workflow Improvements Across 3 Projects

**Date:** 2026-03-21
**Projects:** msc-math (thesis), dnd-claude-code (TTRPG), xrisk-pause-game (card game)
**Sync strategy:** Copy files between repos. Sync rarely. No shared repo — not worth the indirection for 3 projects.

---

## 1. General agent behavior norms

**What:** A section in each project's CLAUDE.md (or a shared convention skill) that codifies behaviors every agent should exhibit regardless of domain. Currently scattered or missing across all three repos.

**Known behaviors:**

**Anti-sycophancy.** Agents should push back on bad ideas, contradictions, and oversights. msc-math has one line about this. dnd and pause-game have nothing. None of the three treats this as a first-class behavioral norm with guidance on *how* to push back effectively (not just "you should push back" but when it's appropriate, how to frame it, when to defer to the human anyway).

**Defer without forgetting.** When an agent notices an issue that isn't the current subtask, it should not drop the current work to chase it, but also not silently forget. Three escalation levels by weight:
- **TODO comment** in the relevant file — lightweight, caught on `grep TODO`, resolved by whoever picks it up next
- **todo() tool** — places it in the current session's task list, picked up after current subtask
- **TASKS.md entry** — for issues needing more context than a comment, or spanning multiple files/sessions

**Generalize from issues.** When an issue is resolved, the agent should abstract the error class and hunt for other instances. This must be part of the resolution workflow, not a deferred step — once an issue is marked done, the generalization step may never happen. msc-math's feedback-processing skill captures this; the other repos need it too. The skill itself can be synced, but the behavioral norm ("this is what resolving an issue means for us") belongs in CLAUDE.md.

**Applies to:** All 3 projects. The norms are identical; examples in CLAUDE.md will be project-specific.

---

## 2. Hook lineage tracking

**What:** Add a comment header to each hook script noting shared lineage and last-sync date. Not a shared repo — just transparency about where the scripts came from so future agents know to check the other repos when making improvements.

**Applies to:** All 3 projects. Trivial effort — one header comment per file.

---

## 3. Postmortem skill (not agent)

**What:** A user-invocable skill for blameless end-of-session reflection. Must be a skill, not an agent, because agents can't access the parent session's conversation history — a postmortem needs to reflect on what happened *in this session*.

**Shared core:** "What slowed you down? What was unclear? What worked well?" → write findings to file → feed into feedback-processing loop (target #1).

**Project-specific extras:**
- **msc-math:** Update plan file, check TASKS.md reflects current state, flag any unverified claims introduced this session.
- **dnd:** Update PROGRESS.md, check lore consistency of new content, note any canon decisions made in conversation but not yet written to files.
- **pause-game:** Already has a post-mortem skill (15 lines, 5 checklist items). Currently it checks: agent splitting needed, fabrications slipped through, iterated in front of user, false attribution, memory update needed. This is a good starting point but needs the shared-core questions added.

**Present state:** dnd has a postmortem *agent* (wrong: can't see session). msc-math has nothing. pause-game has a minimal skill.

**Applies to:** All 3 projects.

---

## 4. Session-handoff skill with subagent review

**What:** A user-invocable skill that (1) runs in main context to capture session state into persistent artifacts, then (2) spawns a review subagent to verify the handoff is complete and accurate.

**Why subagent review:** The main agent wrote the handoff, so it has the same blind spots. A fresh subagent reading only the written artifacts simulates what the next session will actually experience — if it can't reconstruct the state, the handoff has bugs.

**Shared core:** Checklist ("Would the next agent understand from files alone?") → write state to artifacts → spawn review subagent → fix gaps the reviewer finds.

**Project-specific extras:**
- **msc-math:** Primary artifact is the plan file. Also update TASKS.md. Check that math.tex and code are consistent with any changes discussed. msc-math already has plan persistence documented but no checklist or verification step.
- **dnd:** Primary artifact is PROGRESS.md (or TASKS.md — see target #6). Check that NPC/encounter/lore decisions made in conversation are written to campaign files. dnd already has a session-handoff skill but it has no subagent review step.
- **pause-game:** Primary artifact is TASKS.md. Check that design decisions and card concepts discussed in conversation are written to design/ files. pause-game has no handoff workflow.

**Opt-in review:** The subagent step adds latency. Make it opt-in for short sessions (e.g., `/meta-session-handoff --review`).

**Applies to:** All 3 projects.

---

## 5. Handoff files directory

**What:** Each project gets a `handoffs/` directory for scoped future-session specs. A handoff file pre-packages a task with all the context needed so a fresh session can start without re-discovery.

**Distinction from TASKS.md:** TASKS.md tracks *what needs doing*. Handoff files package *how to start doing a specific thing* — which files to read, what decisions were already made, what order to work in, what to watch out for.

**Present state:** msc-math has 15 handoff files (primary mechanism for multi-session continuity). dnd and pause-game have none.

**The session-handoff skill (target #4) should produce these** as part of its workflow when the session involved substantial work that a future session will continue.

**Applies to:** dnd and pause-game (msc-math already has this).

---

## 6. TASKS.md format with escalation markers for dnd

**What:** Replace dnd's per-campaign PROGRESS.md with a root-level TASKS.md following the format proven in msc-math and pause-game. Include escalation markers.

**Why:** File-based task management (TASKS.md) has proven better than alternatives (Linear, GitHub issues, Jörn-tracks-everything). Agents can read and update it directly, it stays in sync because it's part of the same commit flow as the work.

**Escalation markers** from msc-math: 🔴 = Jörn decision required, 🟡 = agent attempts / Jörn reviews, 🟢 = agent autonomous. Eliminate ambiguity about what agents can do on their own.

**Maturity map** from pause-game: A summary table (Settled / Draft / Placeholder / Not started) at the top of TASKS.md gives a quick readiness snapshot.

**Play-readiness info** (currently in PROGRESS.md: "Ready to Run Now", party-level recommendations) moves to campaign CLAUDE.md or a separate file — it's about play, not agent work.

**Applies to:** dnd-claude-code specifically. msc-math already has TASKS.md with escalation markers. pause-game already has TASKS.md with maturity map. Both could adopt from each other (msc-math could add a maturity map; pause-game could add escalation markers).

---

## 7. Meta-knowledge skills — the taxonomy and sync

**What:** Copy msc-math's project-agnostic meta-knowledge skills to the other repos. These are skills about *how to use agents effectively*, not about any specific domain.

The agent knowledge in a project decomposes into distinct artifact types:

| Artifact type | Purpose | Creates what |
|---|---|---|
| **Convention skill** | Development-time guidance: "how to write X correctly" | Standards, not checklists |
| **Review subagent** | Final-gate checker: loads related conventions + checklist | Quality verification |
| **Workflow skill** | Execution protocol cutting across multiple conventions | Step-by-step procedure |
| **Progressively disclosed knowledge** | Reference material loaded only when needed | Deep domain knowledge |
| **General subagent** | Specialized worker delegated a scoped task | Task-specific output |

**Key structural insight:** Conventions map 1:1 to review subagents — each convention package gets a corresponding reviewer. But development workflows cut *across* convention packages: e.g., writing a Rust library file activates code + comment + test conventions simultaneously. Review agents bundle per-concern; workflows bundle per-activity.

**Each artifact type needs a "how to create" meta-skill:**
- **create-convention** — how to write a convention skill
- **create-review-agent** — how to create a review subagent from a convention (dedicated specialization of create-agent, since the convention→reviewer mapping is standard)
- **create-workflow** — how to write a workflow skill
- **create-skill** — how to write progressively disclosed knowledge (exists in dnd; partially in msc-math's meta-documentation)
- **create-agent** — how to write a general subagent (exists in dnd; partially in msc-math's meta-documentation)

msc-math's meta-documentation skill currently covers all of these in one 246-line file. The taxonomy above suggests splitting it — but that's a refinement, not a blocker. The current monolithic skill works.

**What to sync now:** meta-documentation (or its split successors), collaboration, feedback-processing. dnd's create-agent and create-skill should be synced with msc-math's conventions for consistency.

**Applies to:** dnd and pause-game get copies of msc-math's meta-skills. msc-math may refine/split its meta-documentation skill.

---

## 8. Review agents with checklists — the pattern

**What:** Each project creates specialized review agents that follow a shared structure: two-phase (style→content), checklist-driven, autoloading related convention and workflow skills. The *template* is cross-cutting; the *checklists and conventions* are project-specific.

**The architecture:**
- Convention skills explain *how to do things* (development-time guidance, focused on teaching)
- Review agents are the *final gate* (load convention skills + checklists, verify output)
- Workflow skills are execution protocols that cut across conventions (one activity loads multiple convention packages)

**Present state:**
- **msc-math:** Has the most mature version — review skill + 11 reference checklists + how-to-spawn guide. But it's one monolithic review agent, not specialized per-concern.
- **pause-game:** Has review.md (code/balance/visual) + event-review.md (content/fabrication) + check-claims.md (doc verification). Three agents, but no convention skills — review criteria are inline.
- **dnd:** Has lore-checker.md (consistency only). No review agent, no convention skills, no checklists.

**Target per project:**
- **dnd:** Create convention skills (npc-conventions, encounter-conventions, location-conventions) → create corresponding review agents → add checklists. Start with one review agent loading all conventions; split when needed.
- **pause-game:** Extract review criteria from agent definitions into convention skills (card-conventions, event-conventions, code-conventions). Agents then autoload these. Add reference checklists as separate files.
- **msc-math:** Consider splitting the monolithic review agent into per-concern reviewers that can run in parallel (tex-review, rust-review, experiment-review). The convention skills and checklists already exist.

**Applies to:** All 3 projects, with different starting points and different amounts of work needed.

---

## 9. Model selection guidance for ad-hoc subagents

**What:** Guidance on when to override the model for general-purpose (non-custom) subagent spawns.

Custom agents already declare their model in YAML frontmatter — that's solved. The gap is ad-hoc `Agent()` calls where the parent picks a model on the fly.

msc-math's collaboration skill has a tested decision tree (Opus for reasoning/math, Sonnet for style/search, Haiku for bulk checks). If the collaboration skill is synced (target #7), this guidance comes for free.

**Applies to:** All 3 projects, but mostly solved by syncing the collaboration skill.

---

## 10. dnd-specific: review agents and convention skills

**What:** dnd-claude-code currently has no review pipeline and no convention skills. Campaign content (NPCs, encounters, locations, session prep) is guided only by campaign CLAUDE.md tone/philosophy sections.

**Needed convention skills:** npc-conventions (personality/motivation/voice structure, stat block format), encounter-conventions (balance, difficulty signaling, multiple solution paths), location-conventions (format, skimmability, quick-ref boxes).

**Needed review agents:** Content reviewer(s) loading the above conventions. lore-checker already exists but only checks cross-references — it could be extended or supplemented.

**Con:** dnd has only one active campaign and creative content resists rigid checklists. Start light — one review agent, split later if needed. Checklists should be guidance-weight, not compliance-weight.

---

## 11. pause-game-specific: extract conventions from monolithic skill

**What:** pause-game's write-cards skill (358 lines) is a monolithic domain knowledge dump. It combines card writing conventions, card syntax reference, speaker roster, scenario sourcing, anti-patterns, and common mistakes. The event-review agent has its own inline checklists.

**Target:** Split write-cards into:
- **card-conventions skill** — how to write cards correctly (development-time guidance)
- **card-reference skill** — speaker roster, syntax examples, weight guidelines (progressively disclosed knowledge, loaded when writing)
- A corresponding **card-review agent** that autoloads card-conventions and checks against a checklist

Similarly, extract event-review's inline criteria into an **event-conventions skill**.

**Why:** Follows the convention→reviewer pattern (target #8). Makes the review agent maintainable separately from the development guidance.

**Con:** write-cards works as-is. Splitting adds files. Only worth it if the project grows more content types or if review quality needs improvement.

---

## 12. pause-game-specific: CI/CD and check-claims as patterns for msc-math and dnd

**What:** pause-game has two things the other repos lack:
- **CI/CD:** GitHub Actions → Cloudflare Pages on push to main. `npm run check` gates deployment.
- **check-claims agent:** Verifies CLAUDE.md/TASKS.md claims against the actual codebase.

**For msc-math:** Add a GitHub Actions workflow for `cargo test` + `cargo clippy` on push. Currently gating depends entirely on agent discipline. Even minimal CI catches regressions that agents miss.

**For dnd:** Less urgent — dnd is mostly markdown content, not code. But the check-claims pattern (verify CLAUDE.md claims against repo state) would catch stale documentation, which is a real risk as the repo grows.

**check-claims for msc-math:** With 295 lines of CLAUDE.md and 322 lines of TASKS.md, stale claims are inevitable. A check-claims agent verifying file counts, module structure, and convention claims against the codebase would be high-value.

---

## 13. pause-game-specific: fabrication catalog as pattern for msc-math

**What:** pause-game's `iabied-vocabulary.md` explicitly tracks verified terms AND known fabrications (terms agents invented and wrongly attributed to sources). The event-review agent checks against this list.

**For msc-math:** A fabrication catalog for mathematical terms, theorem names, and citation claims would be high-value. msc-math's "never claim without evidence" rule and citation verification are strong, but there's no persistent record of known fabrications to check against. In a thesis context, a fabricated theorem name or wrong attribution is especially dangerous.

**For dnd:** Less critical — fabricated D&D lore is lower-stakes and the source-of-truth chain (session logs > sheets > lore) provides a different defense.

---

## 14. Maturity map for msc-math's TASKS.md

**What:** pause-game's maturity map table (Settled / Draft / Placeholder / Not started) answers "what's done" faster than scanning 322 lines of priority-ordered tasks. msc-math should add a summary table atop TASKS.md.

**Applies to:** msc-math specifically. pause-game already has this.

---

## Summary: What to sync first

Priority order for a sync session:

1. **Hook lineage headers** — trivial, do first
2. **General agent behavior norms** — write the CLAUDE.md section, copy to all 3
3. **Postmortem skill** — adapt pause-game's existing skill, extend with shared core, copy to all 3
4. **Session-handoff skill** — adapt dnd's existing skill, add subagent review step, copy to all 3
5. **Meta-knowledge skills** — copy msc-math's meta-documentation, collaboration, feedback-processing to dnd and pause-game
6. **TASKS.md for dnd** — migrate from PROGRESS.md, add escalation markers + maturity map
7. **Handoff files** — create handoffs/ in dnd and pause-game
8. **Convention skills + review agents** — project-specific, do per-project after sync
