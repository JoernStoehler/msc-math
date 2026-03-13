# How to spawn review subagents

This reference documents the mechanics for the main agent.

## The review agent

A single generic review agent is defined at `.claude/agents/review.md`. It preloads ALL convention skills so it can review any domain. The main agent specifies the concern and files; the subagent self-serves by reading the appropriate checklist reference doc.

## Spawning pattern

Use the Agent tool with `subagent_type: "review"` (or `"figure-review"` for PNG visual inspection). Spawn multiple instances in parallel with `run_in_background: true`.

The prompt needs:
1. **Concern** — which review concern (matching the review skill's concern list)
2. **Files** — which files to review
3. **Report path** — where to write the report (e.g. `/tmp/review-tex-style.md`)
4. **Phase behavior** — phase 1: fix and report; phase 2: report only

The subagent reads the checklist reference doc itself based on the concern. No need to specify which checklist to load.

**Strictly sequential across phases:** Spawn all Phase 1 subagents, wait for completion, fix findings, THEN spawn Phase 2 subagents. Never run Phase 1 and Phase 2 simultaneously.

### Example: review a .tex experiment writeup (full sequence)

**Step 1 — Phase 1 subagents (all in parallel):**

```
Agent(
  subagent_type="review",
  description="Review tex style",
  run_in_background=true,
  prompt="""
    Review these files for LaTeX style (phase 1):
    - experiments/foo/foo.tex

    Report to: /tmp/review-tex-style.md
    Phase 1: fix obvious violations directly, report what you fixed.
  """
)

Agent(
  subagent_type="figure-review",
  description="Review figure PNGs",
  run_in_background=true,
  prompt="""
    Review these figures for visual quality:
    - experiments/foo/foo_errors.png
  """
)

Agent(
  subagent_type="review",
  description="Review python style",
  run_in_background=true,
  prompt="""
    Review these files for Python style (phase 1):
    - experiments/foo/plot_foo.py

    Report to: /tmp/review-python-style.md
  """
)
```

**Step 2 — Fix Phase 1 findings.** Then:

**Step 3 — Phase 2 subagents (all in parallel, on cleaned files):**

```
Agent(
  subagent_type="review",
  description="Review experiment facts",
  run_in_background=true,
  prompt="""
    Review this experiment writeup for factual accuracy (phase 2):
    - experiments/foo/foo.tex

    Data sources to verify against:
    - experiments/foo/foo.jsonl

    Read PNGs to verify figure descriptions match the actual figures.

    Report to: /tmp/review-experiment-facts.md
    Phase 2: report only, do not edit. Flag items for Jörn's verification.
  """
)

Agent(
  subagent_type="review",
  description="Review tex math correctness",
  run_in_background=true,
  prompt="""
    Review these files for mathematical correctness (phase 2):
    - experiments/foo/foo.tex

    Report to: /tmp/review-tex-math.md
    Phase 2: report only, do not edit. Flag items for Jörn's verification.
  """
)
```

### Example: review Rust code (Phase 1 only)

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

### Example: fix figure visual issues autonomously

When Phase 1 figure review finds issues, use `figure-fix` to iterate:

```
Agent(
  subagent_type="figure-fix",
  description="Fix omega-obstacle figures",
  run_in_background=true,
  prompt="""
    Review and fix all visual issues in:
    experiments/omega-obstacle/omega_obstacle.py

    Report to: /tmp/figure-fix-omega.md
  """
)
```

## After reviews complete

1. Read report files from /tmp/
2. Merge reports into a summary for Jörn

## Agent teams alternative

For larger reviews, agent teams (experimental) can be used instead of subagents.
Teams allow teammates to communicate with each other and self-coordinate via a shared
task list. Enable with `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1` in settings.json
(already enabled in this project).

Teams are better when:
- Review findings in one area affect what to look for in another
- The review is large enough that a lead agent coordinating is valuable
- You want teammates to challenge each other's findings
