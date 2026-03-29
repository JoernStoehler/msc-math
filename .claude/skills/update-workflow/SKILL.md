---
name: update-workflow
description: Iterate on existing agent infrastructure (skills, hooks, rules, CLAUDE.md sections) based on feedback and observed failures. Use when Jörn asks to fix, improve, or refine how agents handle a known situation — not for building something new from scratch.
---

# Update Existing Agent Infrastructure

For targeted improvements to infrastructure that already exists. If the infrastructure doesn't exist yet, use `/create-workflow` instead.

## 1. Understand what needs to change

Read the relevant materials:
- The infrastructure file(s) being updated
- Feedback: `feedback/` entries mentioning this infrastructure
- Test results: any existing test tasks in `test-workflow/references/test-tasks/`
- Session logs if Jörn points to specific incidents

Summarize to Jörn: what the current infrastructure says, what the observed problem is, what the gap is between them.

## 2. Diagnose

Identify why the current infrastructure produces the wrong behavior:
- **Vague phrasing?** Agent filled the gap with a training-data default that doesn't match intent.
- **Missing trigger?** Agent doesn't load the skill/rule when it should.
- **Conflicting instructions?** Two sources say different things — agent picks one unpredictably.
- **Attention overload?** Too many instructions, agent drops some.
- **Wrong abstraction level?** Instruction is too abstract to act on, or too specific to generalize.

Present diagnosis to Jörn. He confirms or redirects.

## 3. Draft the fix

Edit the file(s). Follow CLAUDE.md "Text that agents read" conventions. Run the vague-word scan on changed text.

For the fix, prefer:
- Making the existing text more specific over adding new text
- Removing text that doesn't earn its attention cost over keeping it "just in case"
- Concrete examples over abstract rules
- Scripts/hooks that enforce behavior over instructions that request behavior

## 4. Test

Run relevant test tasks from `/test-workflow` against the updated infrastructure. If no test task exists for this failure mode, write one first.

## 5. Jörn reviews

Present: what changed, why, test results. Get explicit approval.

## Reference sources

**Claude Code specs:** `curl -sL https://code.claude.com/docs/llms.txt`
**System prompt:** `references/system-prompt/`
**Expert model background:** `references/agent-expert-model.md`
