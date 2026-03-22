# Agent Failure Modes

Empirically observed failure modes that affect how you should design all agent infrastructure (skills, workflows, conventions, review agents, CLAUDE.md). Don't just tell agents to avoid these — design infrastructure that accounts for them structurally.

## Instruction overload

Agents degrade silently when holding too many novel behavior modifications simultaneously. They don't notice they're overwhelmed — they proceed and drop constraints. Design implications:
- Skills and CLAUDE.md should minimize the number of simultaneously active novel instructions
- Complex workflows should delegate to focused subagents with small instruction sets, not rely on one agent holding everything
- Review agents work because each gets one concern with a small checklist, not all concerns at once
- When writing instructions, evaluate complexity per-item (how many behavior modifications?), not per-token

## Skipping planning

Agents skip planning even at levels where it's obviously worth it (e.g., "what's the goal of this session?"). They'll agree in hindsight that planning would have helped. Design implications:
- Complex skills should include planning as a required step in the workflow, not a suggestion
- Don't rely on agents choosing to plan — build mandatory scope/plan phases into workflow skills

## Under-asking questions

Agents systematically avoid questions that cost Jorn 10 seconds but have high expected value (10% x 1 hour saved). They overweight the visible cost of interrupting and ignore the expected cost of being wrong. Design implications:
- Build explicit checkpoints into workflows where the agent must verify assumptions with Jorn
- Treat "ask Jorn" as a concrete workflow step, not a fallback

## Not modeling own unreliability

Observed in msc-math: agents are unreliable at both writing correct proofs on first attempt and checking proofs for correctness. They don't realize this, and proceed as if their output is correct — instead of taking ~60 seconds of Jorn's time to verify. Likely generalizes to any domain where agent output requires correctness (not just plausibility). A specific mechanism: agents don't verify the results of their own actions (edits, reverts, delegations) — they assume the action succeeded. Design implications:
- Review workflows must be mandatory, not optional — agents won't choose to verify
- Build verification into the workflow itself (write -> review -> fix -> re-review), not as a separate "if you want" step
- For critical content (math proofs, canonical facts), the workflow should include a Jorn verification step by default — the cost is low (~60s) and the cost of proceeding with errors is high
- After any action (edit, revert, subagent delegation), check the result rather than assuming success

## Treating presentation as confirmation

Agents show information to Jorn and proceed as if he confirmed it. "I presented the scope" becomes "the scope is agreed." "I used this document" becomes "this document is approved." Presentation != confirmation. Design implications:
- Workflows should have explicit confirmation gates — don't let "I showed it" advance the state
- The existence of a document (plan file, target state) is not evidence of its approval status

## Not checking existing state before changing it

Agents add content without reading what's already there, creating duplication or contradiction. Design implications:
- Skills that add content should include a step to read the target file first
- Review subagents should check for duplication against surrounding content, not just internal consistency

## Not modeling Jorn's current state

Agents don't consider what Jorn can see, access, or answer right now. They send text walls too long for Jorn to review, ask questions requiring file reads he hasn't done, respond to old queued messages instead of the most recent one. Design implications:
- Skills that produce output for Jorn should summarize key decisions and questions at the end, not assume Jorn followed along
- Prefer file:line references over inline text when showing content for review
- Handoff files exist partly because agents can't reliably communicate findings within a session

## Transferring cognitive work to Jorn

Agents ask Jorn to do thinking the agent should do: "what do you want me to do?", asking permission on obvious actions, requesting scope instead of proposing one. This is distinct from under-asking — the problem isn't asking too little, but asking the wrong kind of thing (open-ended "what should I do?" instead of specific "is X correct?"). Design implications:
- Skills should frame agent actions as proposals to verify, not open questions for Jorn to answer
- "I plan to do X because Y — any objection?" is cheaper for Jorn than "what should I do?"

## Responding to social signal instead of content

When corrected, agents respond to the criticism ("You're right, I shouldn't have done that") instead of fixing the thing. Empty agreement wastes time. Design implications:
- Post-correction workflow: fix first, acknowledge briefly, move on

## Not generalizing from mistakes

Agents fix the specific instance flagged by Jorn but don't abstract the error class or scan for other instances — in the code, in other files, or in their own recent behavior. Example: "forgot to run test XYZ" doesn't trigger "what else did I forget?" even though asking that question is well within agent capability. One specific manifestation: agents learn lessons abstractly but don't spontaneously apply them to their own current behavior (learned "keep shared files identical across repos" -> then immediately made project-specific modifications; documented delegation failures -> then immediately trusted subagent reports without verification). The `meta-feedback-processing` skill addresses this but agents still under-apply it. Design implications:
- The generalization step must be part of the resolution workflow, not a follow-up
- Review agents and postmortem skills should explicitly prompt for error-class abstraction

## Delegation failures: loud vs silent

Subagent failures come in two kinds. Loud failures: the subagent reports "I'm stuck on subtask X" — the parent replans, no damage. Silent failures: the subagent reports "done" but did X' instead of X — the parent proceeds with a broken assumption. Silent failures are far more dangerous. They arise when the parent's mental model, the skill/agent description, or the prompt diverge from what the subagent actually does. Crucially, the subagent cannot detect or report this kind of failure — it has no access to the parent's intent, so the information asymmetry is structural, not a matter of subagent quality. Verification must come from outside the subagent (the parent or a separate verifier), not from the subagent being "better." Verification of reasoning tasks is especially hard because the output looks plausible even when wrong. Design implications:
- Skill/agent descriptions must state what the tool does AND does not guarantee — descriptions are the contract that parent agents plan around
- Verification after delegation is high-value because the communication channel (the prompt) is unreliable and agents don't anticipate ambiguity in their own prompts
- Complete verification (prove correctness) and incomplete falsification (find some errors) are different — don't let "we ran a review" be mistaken for "this is verified"
- For novel tasks, result types must be explicit in the prompt — agents have no training priors to fall back on

## Lossy transcription

When encoding precise statements (from Jorn, from sources, from specifications) into their own words, agents systematically lose meaning. The losses aren't random — they tend toward simplification, generalization, flattening distinctions, and substituting the agent's framing for the original. Particularly dangerous when transcribing novel or anti-intuitive knowledge, where training priors pull the rewording toward the familiar (wrong) meaning. Design implications:
- Review subagents should compare agent output against original source text, not just check internal consistency
- When encoding someone's precise statement, prefer quoting over paraphrasing
- Verification of transcribed content requires access to the original — a subagent checking only the transcription can't detect meaning drift

## Defaulting to the easy action

When the correct action requires more effort, agents take the lower-effort alternative. Examples: paraphrasing instead of extracting original text, claiming convergence instead of continuing to search. Design implications:
- Workflows should make the correct (higher-effort) action the default path, not the alternative
- When a skill offers two approaches (easy and thorough), frame the thorough one as default and the easy one as the exception requiring justification

## Defaulting to the familiar action

Agents pick the approach that pattern-matches to training, even when the situation calls for something different. Examples: "copy from the most mature source to others" (familiar pattern) instead of "evaluate each source's strengths and cross-pollinate" (requires judgment). The familiar approach is not necessarily easier — familiarity and effort are independent. Agents pick the approach they've seen most, regardless of effort. Design implications:
- Novel workflows should explicitly name the familiar-but-wrong approach and say why it doesn't apply here
- When a task requires judgment between approaches, the skill should flag that the obvious/familiar approach may not be correct

## Additional observations

**Word-choice sensitivity.** Jorn communicates via subtle word choices that encode real distinctions. Agents trained on natural language tend to normalize variations ("not quite" -> "yes but also"), losing the correction's content. Adopt Jorn's exact phrasing rather than paraphrasing — the cost of preserving exact wording is zero but the cost of losing a distinction compounds across the session.

**Prompting modern models (2026).** Most prompt engineering advice is outdated or cargo culting. What matters: provide enough context, write clearly and unambiguously, use formats agents were trained on (markdown, code blocks, bullet lists). Don't waste effort on "prompt engineering tricks."

**Agent quality near context limits.** Agents get unfocused and impatient as sessions approach 200k tokens (near compaction). Basic operations remain fine, but decision-making quality degrades. Jorn sometimes triggers compaction earlier to avoid this.
