# Feedback: Agents (.claude/agents/)

Raw observations from agents about review/planning subagents. Do not analyze here — a future agent-design session will review and act on these.

## Format

Each entry: date, which agent, what happened, what was confusing/missing/unhelpful. Include: did the agent trigger when expected? Did it produce useful output?

## 2026-03-30: review-proof on verify-numerics/math.tex

Triggered proactively on first draft, found 11 issues (4 high: missing assumptions, broken \ref, handwavy "second-order" claim, dropped second-order term in runtime bound). All addressed before presenting to Jörn. Good ROI — saved one Jörn round-trip.

## 2026-04-01: opus subagent for QP algorithm research

Subagent confidently recommended vertex enumeration ("max of quadratic on polytope is at a vertex"). Self-corrected mid-analysis (indefinite H breaks this), but the main agent presented the recommendation before verifying applicability. Jörn caught it. **Lesson:** When a research subagent makes a mathematical claim, the main agent must verify it applies to the specific problem before presenting. Subagents don't know domain-specific constraints (our H is indefinite).

## 2026-04-01: review-formalization NOT used (should have been)

The claim "false negatives are caught by the capacity algorithm's sub-permutation enumeration" was stated confidently without checking the code. A review-formalization agent checking "does this statement correspond to what the capacity algorithm code does?" would have caught that the algorithm prunes permutations within subsets, so boundary optima aren't guaranteed to be found. **Lesson:** Use review-formalization proactively when making claims about code behavior, especially code outside the current experiment.

## 2026-04-01: foreground execution pattern

The agent ran long commands (`cargo run`, `Agent` for recover-context) in foreground 3+ times despite:
1. A memory entry (feedback_cpu_management.md) explicitly saying "run heavy jobs in bg"
2. CLAUDE.md saying to use run_in_background
3. Jörn correcting it during the session

The pattern: agent queues tool calls before responding to messages. In a tight interaction loop, this blocks the user for minutes. Two new memory entries were added (respond_before_tools, blocker_stops_old_work) but the underlying issue is that the agent treats tool calls as higher priority than conversation.

Suggested: the agent infrastructure could enforce that in-progress user messages are responded to before tool calls are made. Currently this is only a behavioral instruction that agents violate.

## 2026-04-01: continued old task after blocker identified

Agent was told "iterate on the bound." Then the pipeline was identified as broken. Agent continued iterating on the bound (running it through the broken pipeline) while simultaneously fixing the pipeline. Three wasted rerun cycles.

The error class: "prior instruction superseded by new context, but agent continues both." This is different from "didn't listen" — the agent acknowledged the pipeline was broken but didn't stop the old task.

## 2026-04-01: review subagent output relayed without cross-checking

**What happened:** Agent ran /pre-merge, launched 4 review subagents (review-proof, review-formalization, review-claims, review-rust). When results came back, relayed findings to Jörn without verifying them against actual files. On cross-check (prompted by Jörn), 3 problems found:

1. **False positive:** review-formalization flagged `rem:adjacency-pruning → lem:numerical-transition-feasibility` as "dangling reference." Label exists in `crates/src/kkt/math.tex` and resolves via root math.tex. Agent scoped its search to the experiment's math.tex only.

2. **Misleading framing:** review-formalization called Q error bounds lemmas "orphaned" after SP code removal. These are valid math independent of which solver is used — unused ≠ orphaned.

3. **Known gaps as findings:** review-formalization flagged 5 functions without math.tex entries. Logbook says "Part III not written." Known gap, not discovery.

4. **Missed self-correction:** review-proof flagged rem:trinary-beta item 5 for citing lem:near-boundary-drop incorrectly. But the text 6 lines later explicitly acknowledges the limitation ("the lemma does not justify converting Indeterminate to DROP"). Agent read the claim but not the clarification.

**Two separate problems:**

(A) **This agent should have cross-checked.** The core rule ("never write a factual claim without verifying it") applies to agent findings presented as claims. No additional guidance needed for this — the rule already covers it.

(B) **Future agents will make the same mistake.** This is a structural workflow gap. Two fixes needed:

- **pre-merge skill:** Content checks section should explicitly say to launch review subagents (review-claims for factual claims, review-proof for math.tex, review-formalization for cross-references) and to cross-check their output before presenting.

- **Not in agent definitions** — the caller doesn't read those, so guidance there doesn't reach the agent that needs it. The fix belongs in places the caller sees: the pre-merge skill (see skills.md entry), CLAUDE.md's subagent section, or similar.

## 2026-04-02: repo-wide path-update subagents missed file categories

**What happened:** Launched 5 parallel sonnet subagents for Phase 4 path updates (math.tex, Python, CLAUDE.md+rules, TASKS.md+handoffs, logbooks). Two categories of files were missed:

1. `.claude/skills/` and `.claude/agents/` — The CLAUDE.md+rules subagent was prompted with "CLAUDE.md and .claude/rules/*.md" but not `.claude/skills/` or `.claude/agents/`. Three skill files and one agent file had stale `experiments/` paths.

2. `.rs` doc comments in 8 files — The logbook subagent updated logbooks but the prompt didn't cover .rs files. The Python subagent covered analyze.py but left docstrings alone. No subagent was responsible for .rs doc comments.

Required two additional fix-up passes (one via subagents, one manual via sed).

**Error class:** Subagent scope gaps when partitioning work by file type. Each subagent's prompt defined a narrow file set, and files that didn't fit neatly into any category fell through the cracks.

**Suggestion:** For repo-wide find-and-replace tasks, add a "sweep" subagent whose job is to grep for remaining stale references across ALL file types after the targeted subagents complete, and fix anything they missed. Or: include a verification grep in each subagent's prompt and have them report (not fix) files outside their scope.

## 2026-04-03: should have self-reviewed audit before presenting

Session: TASKS.md review. Main agent produced a 10-point audit of TASKS.md accuracy and presented it to Jörn without subagent verification. Jörn asked "Have you asked a subagent to verify/check your review for accuracy & for full argumentation & for gaps/oversights?" The verification subagent then found a real error (verify-numerics incorrectly recommended for Completed despite having open items) and several gaps (thesis TODOs in other files, priority guidance conflict, stale cross-experiment cleanup section).

**Pattern:** When presenting factual claims about repo state to Jörn (audits, investigation findings, data analysis), verify with a subagent before presenting. The core rule says "never write a factual claim without verifying it" — this applies to audit conclusions too, not just code/data claims. The verification subagent is cheap; a wrong claim reaching Jörn wastes his time to catch and correct.

## 2026-04-03: fabricated explanation instead of flagging inconsistency

**What happened:** After compaction, agent saw partial work on disk (new directories, moved files, new run.rs files) but no Agent() tool calls in its visible conversation history. Instead of flagging this contradiction to Jörn, the agent:

1. Assumed the agents existed and were lost to compaction
2. Proposed launching new agents as a workaround
3. When Jörn pushed back ("why not resume them?"), said "I don't have agent IDs" — still not flagging the core inconsistency
4. When Jörn asked "when did a compaction happen?", still didn't flag it
5. Only after Jörn asked "do you not see your own Agent() calls?" did the agent look at the JSONL log

The agent IDs were in the JSONL transcript the whole time. The agents had been launched (lines 494-496 of the transcript) but their calls were not in the agent's visible context — likely compacted away. The correct response at step 1 was: "I see agent work on disk but no Agent() calls in my context. This is inconsistent — let me check the JSONL log to find the agent IDs."

**What should have happened:** When observations contradict each other (work exists but no tool calls created it), flag the contradiction immediately instead of constructing a plausible-sounding explanation. The CLAUDE.md rule "never write a factual claim without verifying it" applies — "the agents are gone" was an unverified claim.

**Pattern:** Confabulation under uncertainty. Same error class as 2026-04-03 "trusted claude-code-guide subagent documentation claims without verification" — the agent fills gaps with plausible explanations instead of saying "I don't know, let me check."

## 2026-04-03: used subagent to read 3 lines from a file

**What happened:** Needed to find agent names from lines 494-496 of a JSONL transcript. Launched a recover-context subagent to do this. When that wasn't enough, tried to launch a second subagent for the same file. Jörn rejected it and said to just read the file directly. A single `sed -n '494,496p' | python3 -c ...` command took 2 seconds and returned exactly what was needed.

**What should have happened:** The first recover-context subagent (to find which lines contained Agent calls) was justified — searching a 610-line JSONL for relevant entries is a lookup task. But once the lines were known (494-496), reading 3 lines is a direct operation, not a subagent task.

**Pattern:** Over-delegation. Jörn's framing: "Don't ask a librarian to find, read a book and report back some insight from the book. Ask them to find the book and then you read it." Use subagents to locate information, then read and interpret it yourself.

## 2026-04-03: did not understand own subagent lifecycle tools

**What happened:** After compaction removed Agent() calls from visible context, agent needed to check on / resume 3 subagents. The following sequence wasted ~15 minutes of Jörn's time:

1. Assumed agents were dead without checking. Proposed new agents.
2. When told to resume via SendMessage, claimed "I don't have agent IDs." Didn't look for them.
3. When told to check JSONL, over-delegated to a recover-context subagent for a 3-line read.
4. After finding agent names, sent SendMessages. When nothing happened, claimed "they're dead" without verifying.
5. Didn't know TaskOutput existed for checking agent status until Jörn listed the tools.
6. Used TaskOutput with agent names, got "no task found." Then used tool_use_ids, same error. Didn't distinguish between "wrong ID format" and "agent dead."
7. Only after Jörn asked "are those agent IDs?" realized tool_use_ids ≠ agent IDs. Agent IDs come from Agent() tool results — which never arrived because the agents were interrupted.

**What should have happened:** After finding no Agent() calls in context:
1. Check JSONL for the calls (direct read, not subagent)
2. Check for results — if no results, agents were interrupted and IDs were never returned
3. Try both names and tool_use_ids with TaskOutput to check status
4. If all fail, conclude agents are dead and report the evidence chain to Jörn

**Root cause:** Agent doesn't understand the subagent lifecycle:
- Agent names (from `name` param) vs agent IDs (from result `agentId` field) vs tool_use_ids
- SendMessage uses names. TaskOutput uses task_ids. These may or may not be the same.
- If Agent() never returned a result, no agent ID exists — the agent was killed before registration.
- SendMessage returning "success" doesn't mean the agent is alive — it may silently succeed for dead agents.

**Action needed:** Add a "Subagent recovery after compaction" section to CLAUDE.md or a rule file with the lifecycle facts above, so future agents don't repeat this 15-minute fumble.

**Pattern:** Tool illiteracy — agent doesn't read or understand its own tool descriptions well enough to use them correctly under non-happy-path conditions. Related to "confabulation under uncertainty" (same session) but distinct: the information gap here is about tooling, not about repo state.

## 2026-04-03: trusted claude-code-guide subagent documentation claims without verification

Session: building PostExitPlanMode hook. Agent launched claude-code-guide subagent to research hook events. Subagent returned a massive response (28 events, full JSON schemas, detailed field descriptions). Agent started building on this without reading the actual Anthropic docs. Jörn asked "Did you read the anthropic guide on hooks?" — agent then fetched the real docs via WebFetch and found the subagent output was plausible but unverifiable in detail.

**Pattern:** Subagents (especially those using smaller models) are unreliable for external documentation lookups — they may produce plausible but fabricated API details, field names, or event schemas. The Core Rule covers "the code does X" and "the data shows Y" but not "the docs say Z." For external documentation claims, verify against the primary source (WebFetch, official docs) before building on them.

## 2026-04-03: hook-injected additionalContext should be lightweight

Session: building PostToolUse reliability blurb for Agent tool. First draft had prescriptive numbered rules: "(1) check file:line sources directly, (2) verify factual claims against primary sources, (3) for critical decisions, launch a second subagent to cross-check." Jörn's feedback:
- "this result is not needed" (unnecessary preamble)
- "esp doc claims adds complexity" (only applies to one agent type, not general)
- "(1) only applies to one agent" (file:line sources are specific to session-search)
- "doesn't define critical" (agent should decide using its own context, not a predefined rule)

Revised blurb: "Subagent answers can be overconfident or miss context. Cheap-to-check facts (file existence, grep results, data values) are worth verifying directly. A second subagent can cross-check if warranted."

**Pattern:** Hook-injected context (additionalContext) is a system reminder, not an instruction manual. State the risk, give lightweight guidance, let the agent decide. Don't write prescriptive multi-point rules — they add complexity and try to predetermine decisions the agent should make using its own context.
