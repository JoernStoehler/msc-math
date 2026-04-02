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
