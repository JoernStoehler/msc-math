# How to spawn review subagents

This reference documents the mechanics for the main agent.

## The review agent

A single generic review agent is defined at `.claude/agents/review.md`. It preloads ALL convention skills so it can review any domain. The main agent specifies the concern and files; the subagent self-serves by reading the appropriate checklist reference doc.

## Spawning pattern

Use the Agent tool with `subagent_type: "review"`. Spawn multiple instances in parallel with `run_in_background: true`.

The prompt needs:
1. **Concern** — which review concern (matching the review skill's concern list)
2. **Files** — which files to review
3. **Report path** — where to write the report (e.g. `/tmp/review-tex-style.md`)
4. **Phase behavior** — phase 1: fix and report; phase 2: report only

The subagent reads the checklist reference doc itself based on the concern. No need to specify which checklist to load.

### Example: review a .tex deliverable

```
Agent(
  subagent_type="review",
  description="Review tex style",
  run_in_background=true,
  prompt="""
    Review these files for LaTeX style (phase 1):
    - thesis/sections/algorithm.tex
    - thesis/sections/algorithm-proof.tex

    Report to: /tmp/review-tex-style.md
    Phase 1: fix obvious violations directly, report what you fixed.
  """
)

Agent(
  subagent_type="review",
  description="Review tex math correctness",
  run_in_background=true,
  prompt="""
    Review these files for mathematical correctness (phase 2):
    - thesis/sections/algorithm.tex
    - thesis/sections/algorithm-proof.tex

    Report to: /tmp/review-tex-math.md
    Phase 2: report only, do not edit. Flag items for Jörn's verification.
  """
)
```

### Example: review Rust code

```
Agent(
  subagent_type="review",
  description="Review rust style",
  run_in_background=true,
  prompt="""
    Review these files for Rust coding style (phase 1):
    - crates/src/algorithms/hk2017.rs
    - crates/src/kkt.rs

    Report to: /tmp/review-rust-style.md
  """
)
```

### Example: review experiment writeup against data

```
Agent(
  subagent_type="review",
  description="Review experiment facts",
  run_in_background=true,
  prompt="""
    Review this experiment writeup for factual accuracy (phase 2):
    - experiments/sys-optimization/sys-optimization.tex

    Data sources to verify against:
    - experiments/sys-optimization/sys-optimization.jsonl
    - experiments/sys-optimization/sys-optimization_output.txt

    Report to: /tmp/review-experiment-facts.md
  """
)
```

## After reviews complete

1. Read the report files from /tmp/
2. Fix phase 1 issues (or verify subagent already fixed them)
3. Spawn phase 2 reviews on the cleaned files
4. Merge reports into a summary for Jörn

## Agent teams alternative

For larger reviews, agent teams (experimental) can be used instead of subagents.
Teams allow teammates to communicate with each other and self-coordinate via a shared
task list. Enable with `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1` in settings.json
(already enabled in this project).

Teams are better when:
- Review findings in one area affect what to look for in another
- The review is large enough that a lead agent coordinating is valuable
- You want teammates to challenge each other's findings
