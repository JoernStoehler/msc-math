# Post-mortem: Bundle F — Database cleanup (2026-04-07)

## Findings

### 1. Agents should proactively document "acceptable for now" decisions

**What happened:** After the pre-merge review flagged two items (callers silently dropping new KktOutcome variants, Source::LagrangianProduct with 0.0 placeholders), the orchestration agent said "discussed with Jörn, acceptable for now" in the report but did NOT add them to TASKS.md. Jörn had to prompt: "TODOs documented?"

**Rule:** When an agent decides something is "fine for now, revisit later," it should immediately persist that decision in TASKS.md with context on when to revisit. Don't wait for Jörn to notice the gap.

**Scope:** Applies to orchestration agents and pre-merge workflow. Could be added to pre-merge Phase 6 or as a general convention.

### 2. Data regeneration agents should verify output paths against tracked files

**What happened:** The data regeneration agent ran 4 experiments. The experiments wrote JSONL to new directories (`combinatorial-profiling/`, `combinatorial-anatomy/`, etc.) because the code had stale output paths. The tracked JSONL files were in different directories (`cell-widths/`, `boundary-characterization/`). The agent didn't notice the mismatch — it just created the new directories and reported success. The orchestration agent discovered the issue during verification and had to copy data + eventually fix the paths.

**Rule:** Before running experiment binaries that produce data, agents should run `git ls-files -- '*.jsonl'` in the experiment directory to find where tracked data lives, and verify the experiment output path matches. If they don't match, flag before running.

**Scope:** Applies to any agent that regenerates experiment data. Could be added to experiment conventions or as a pre-flight check in data regeneration prompts.
