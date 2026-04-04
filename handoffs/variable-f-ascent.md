# Task: Finalize variable-F gradient ascent experiment

## Context

Experiment exploring whether allowing facet count to grow (F→F+1) during gradient ascent unlocks higher sys values. Implemented, run, and analyzed in one session. Core findings are solid but the logbook framing and interpretation need revision — the session ended with degraded reasoning at ~250k tokens.

## Scope

1. **Reframe RQ2 logbook and conclusions.** The current logbook overstates "D wins 10/10 vs A" as a finding. D ≥ A is trivially guaranteed (D starts from A's endpoint, ascent can only improve). The actual findings are:
   - RQ1 (86% of F=10 local maxima improve in F=11 space) is the clean result
   - RQ2 Path B < A shows expanding before optimizing hurts
   - RQ2 Path C ≈ A shows more facets don't inherently help without prior optimization
   - RQ2 Path D is a sanity check confirming D ≥ A, with the magnitude of improvement being the interesting part (mean +0.033)

2. **Update logbook conclusion** to reflect the above. The "optimize first then expand" strategy is confirmed but the framing should be precise about what's trivial vs informative.

3. **Run `/pre-merge`** and present to Jörn for merge.

## Out of scope

- Don't iterate F→F+1→F+2 (follow-up experiment)
- Don't implement gradient-informed placement (follow-up)
- Don't run cut-and-ascent on HKO2024 (scaffolded in exp-hko-local-maximum/cut-and-ascent/, separate task)
- Don't add database caching to other gradient ascent experiments

## Key files

- `/workspaces/msc-math/.claude/worktrees/variable-f-ascent/crates/exp-sys-landscape/variable-f-ascent/` — the experiment
  - `run.rs` — binary with RQ1 (50 trials) + RQ2 (40 trials, 4 paths), database caching via local `cache.jsonl`
  - `analyze.py` — RQ1 scatter plot + RQ2 four-way box plot
  - `logbook.md` — needs reframing per scope item 1
  - `variable-f-ascent.jsonl` — 90 rows of results
  - `cache.jsonl` — 205MB local capacity cache (gitignored), 12K polytopes, makes reruns 18x faster
- `/workspaces/msc-math/.claude/worktrees/variable-f-ascent/crates/exp-hko-local-maximum/cut-and-ascent/` — scaffolded HKO2024 experiment (run.rs compiles, logbook has preliminary findings and ideas, not yet run)

## Prior findings

- D ≥ A is guaranteed by construction (ascent from A's endpoint). Don't present it as an empirical discovery.
- B's poor performance is from thin-sliver facet geometry, not from having more facets (C with natural F=11 geometry does fine).
- The cache.jsonl is 205MB for 12K polytopes. Gitignored, regenerated on first `--fresh` run.
- RNG state: seed 43 for RQ2, seed 42's local maxima loaded from gradient-ascent-general.jsonl for RQ1. Path D's facet addition and F=11 ascent consume RNG draws, so Paths B and C get different random state than in a three-way run — this is fine, the comparison is within-seed.

## Branch state

Worktree at `/workspaces/msc-math/.claude/worktrees/variable-f-ascent/`, branch `variable-f-ascent`. 4 commits ahead of main:
- `075d63ca` Initial experiment
- `889c4748` Review fixes
- `0fdb18c6` Move HKO2024 to exp-hko-local-maximum
- `8846bce6` Add Path D + database caching

## Success criteria

- Logbook reframed: D vs A presented as sanity check, not headline finding
- `/pre-merge` passes
- Jörn approves merge
