---
name: review-branch
description: Independent critical review of code/docs in worktree branches. Present calibrated findings and recommend action. Covers Rust, Python, LaTeX, data pipelines.
---

# Branch Code Review

## Core Principle

Your job: Find all problems, suggest solutions, make a calibrated recommendation.

Jörn reads your report and makes the final decision (he deviates ~50% of the time based on project context you lack).

**Time costs:**
- Jörn's time is the bottleneck (scarce, expensive)
- Your time is cheap (practically unbounded)
- **Goal: Save Jörn time**, even if review takes you much longer
- High-latency thorough review > low-latency barely-usable report

**Workflow:**
1. **You:** Investigate thoroughly, find all issues
2. **You:** For each issue: suggest solution(s) or note if solution unclear
3. **You:** Make honest, calibrated recommendation (merge / fix X and Y / needs rework / etc.)
4. **Jörn:** Reads executive summary, makes actual decision based on his context

**Why Jörn often deviates:**
- Some suggested "improvements" actually worsen code
- Some fixes need implementer context vs reviewer doing them
- Project management considerations (deadlines, dependencies, priorities)
- Domain knowledge about what matters for the thesis

**Your value:** Present all findings in calibrated, honest format so Jörn can quickly absorb and apply his judgment where it's needed most.

## Plan Phase (for multi-aspect reviews)

For complex reviews, write a plan file before deep analysis.

**When to write a plan:**
- Branch diff exceeds 300 LOC, OR
- Branch touches >3 files or >2 languages, OR
- Branch includes deletions requiring verification, OR
- Branch modifies mathematical correctness code, OR
- Branch changes data pipelines

**Plan file structure:**

1. **Detailed Plan (for agent execution, at top):**
   - Full checklist of items to verify
   - Methodology for each phase
   - Expected evidence to collect
   - Known unknowns to investigate
   - Planned subagents/teams (what work to delegate, which phases to parallelize)

2. **Executive Summary (for Jörn, at bottom):**
   - What changed (3-5 bullet points)
   - Key concerns to investigate
   - Preliminary scope (which files/aspects to review in depth)
   - Delegation strategy (how much work delegated to subagents/teams, estimated timeline)

**Plan file location:** `~/.claude/plans/<branch-name>-review.md`

Write plan, discuss with Jörn if needed (describe plan in chat and ask for feedback), then execute.

## Methodology

**Review scope:**
- Review the **final state** (git diff main...HEAD) for correctness
- Review **individual commits** only for commit quality (messages, atomicity)
- For branches >1000 LOC changed, prioritize: (1) mathematical correctness, (2) public API changes, (3) test changes, (4) documentation. Note in report which files were reviewed in depth vs. skimmed.

### Phase 0: Check for Existing Analysis (before exploration)

Look for `docs/reports/*`, `*_INVESTIGATION.md`, `*_REVIEW*.md` in worktree.
If found: read first, verify claims, explore gaps. Saves 20min vs redundant exploration.

### Phase 1: Fast Checks (5-10 min)

**Agent infrastructure changes** (if branch touches `.claude/`, `CLAUDE.md`, skills/):
- Verify no contradictions with existing CLAUDE.md content
- Instructions are clear and actionable for agents
- No stale references to removed files or tools
- Test the skill/change yourself if feasible (e.g., invoke a modified skill)

**Build verification:**
- Rust: `cd crates && cargo test && cargo clippy`
- Python: `ruff check experiments/ && pytest experiments/` (if applicable)
- LaTeX: `cd thesis && latexmk` (if applicable)

Run tests per CLAUDE.md "Commands" section for the repository.

**If tests fail:** Note the failures, investigate root cause briefly (5-10 min), then include test failures as the top finding in your report. Recommend "fix tests before re-review" unless the failures are clearly pre-existing on `main` (check by running tests on `main`). If pre-existing, note this and continue the review.

**Git comparison:**
- ALWAYS compare against local `main`, never `origin/main`
- Use three-dot diff: `git diff main...HEAD`
- State base explicitly in report: "Compared against local `main` at `abc1234`"

**Commit quality check (5min, HIGH value):**
- Messages describe "why" not just "what"
- Atomic commits (one logical change each)
- Co-authored-by present
- No "oops" or fixup commits

**Working tree check:**
- `git status` shows clean working tree
- No uncommitted changes
- No generated data files (experiments/data/, experiments/figures/) committed
- Committed fixture files (e.g., tests/fixtures/) are intentional

**Archaeology check:**
- If any code appears sourced from `archaeology/`, flag prominently
- CLAUDE.md marks all archaeology content as untrusted - needs extra scrutiny

### Phase 2: Deletion Verification (10-30 min, if applicable)

**CRITICAL: Read deleted code FIRST before evaluating replacements.**

For each deleted file:

1. Use `git show main:<path>` to view deleted file contents
2. Understand its purpose (what did it do?)
3. Identify the replacement (or confirm it's dead code)
4. Verify replacement does what old code did, OR intentionally doesn't

**Document in deletion verification table:**

| Deleted File | LOC | Purpose | Replacement | Verdict |
|--------------|-----|---------|-------------|---------|
| timing_model.py | 170 | Fit model, project datasets | benchmark.py (fit + orchestrate) | ✓ Core replaced, projection removed intentionally |
| profile_capacity.rs | 16 | Ad-hoc profiling stub | None (debugging code) | ✓ No replacement needed |

**Evidence for intentional removal:**
- Check if related config/data files also changed (e.g., "projections" field removed from JSON)
- Read commit messages for explanation
- Verify no dangling references to deleted functionality

**Common deletion patterns:**
- Replacement with broader scope (old: 6 examples → new: 76 examples)
- Intentional cleanup (feature removed, documented in commit)
- Migration (`.md` writeup → `.tex` section)
- Dead code (debugging stubs, one-off experiments)

See `deletion-examples.md` for concrete examples.

**If uncertain about deletion:** Flag for Jörn's review, don't approve merge.

### Phase 3: Code + Data Review (20-60 min)

#### Language-Specific Conventions

**Agent note:** Detect languages from git diff. Omit inapplicable checks.

**Rust conventions:**
- Read `crates/CLAUDE.md` for full conventions
- **Mathematical doc comments**: Verify that doc comments defining mathematical objects match implementations:
  - Doc comment formula matches code's computation
  - Stated invariants enforced by type/constructor
  - Stated properties are tested
- **Cross-crate semantic changes**: If branch modifies public API or function semantics, check downstream crates for usage patterns that depend on old semantics (see crate dependency graph in CLAUDE.md)

**Python conventions:**
- Read `experiments/CLAUDE.md` for full conventions

**LaTeX conventions:**
- Read `thesis/CLAUDE.md` for full conventions

**Performance claims:**
- Require measurement: "~1ms" is claim, "Benchmark shows 1.5ms for F=5-12" is measured
- Add benchmark if claim exists without measurement

**Test coverage:**
- Critical paths tested (error paths, math properties, degenerate cases)
- Core cases covered (happy path, known-good inputs, basic errors)
- Edge cases tested (property-based tests, boundaries, robustness)
- **Test runtime strategy**: For expensive functions (e.g., capacity), tests follow two-category pattern (see `crates/CLAUDE.md` "Testing expensive functions"):
  - **Category A (Input-Output):** Test correctness of results. Use fixtures (preferred, fast) or #[ignore] + release mode. Examples: capacity values, mathematical properties (conformality, monotonicity).
  - **Category B (Internal Behavior):** Test safe execution in debug mode with small inputs (F ≤ 6). Examples: smoke tests exercising enumeration/pruning logic, error path handling.
  - Verify tests have doc comments explaining what/why/mode (debug/release/fixture).
  - See monitoring Check 7 for detailed criteria.
- **Assertion usage**: Expensive mathematical validation should use `debug_assert!` (not `assert!`). Critical safety invariants should use `assert!` (not `debug_assert!`).

#### Data Pipeline Tracing (if branch touches experiments/)

For branches modifying data pipelines, trace end-to-end flow:

**Example: Rust → CSV → Python → JSON → LaTeX**

1. Identify source (Rust binary output, Python script, manual data)
2. Follow transformations (parse, compute, write)
3. Verify consistency:
   - CSV columns match Python parser expectations
   - JSON parameters match LaTeX writeup values
   - Stats table values match computed values from source

**Verification technique:** Write Python one-liner to recompute stats from CSV, compare to LaTeX table. Exact match → high confidence.

**Common pipeline issues:**
- Mismatched column names (silent breakage)
- Parameter values drift (JSON vs LaTeX)
- Units inconsistent (ms vs s, different between code and docs)

#### Statistical/Mathematical Correctness

**You have domain knowledge:**
- Statistics (log-linear regression, R² calculation, model fitting)
- Symplectic geometry (read thesis .tex for specifics)
- Bayesian reasoning (assign probabilities to mathematical statements)

**For thesis-specific math:**
- Read relevant thesis .tex sections if they exist (thesis is growing, not complete)
- Trust levels: Look for `% Jörn: [level] approved (hash)` markers in .tex files (see thesis/CLAUDE.md for format)
  - Levels: text > math > structure
  - Content without markers is agent-written and unreviewed
- We form conjectures based on mathematical intuition + empirical evidence

**Verification approach:**
- Check formulas match literature OR
- Check tests validate mathematical properties (scaling laws, symmetries)
- For statistical code: verify regression formulas (e.g., log-linear for exponential), R² calculation (1 - ss_res/ss_tot), median computation (even/odd handling)

**DO NOT ask if R²=0.997 is "good enough"** — it obviously depends on what we use the result for in the project. Trust domain knowledge and common sense.

**Bayesian mindset:**
- R² is a tool, not a Frequentist religion
- Interpolation vs extrapolation: claims within fitted domain are often ~99% trustworthy, with bugs overtaking spurious correlation as failure mode
- Linear scaling (dataset cost = polytope cost × count): 99% prior

#### Naming and Documentation Accuracy

Check public symbols accurately describe behavior:
- Function names match specifically what code does (not aspirational, not ambiguous)
- Doc comments describe actual behavior (not outdated)
- Test names reflect what's tested (fallback paths can change semantics)

### Phase 4: Findings + Recommendation (10-20 min)

#### Making Your Recommendation

After investigation, synthesize findings into a recommendation. There is no fixed set of recommendation types — use whatever fits the situation.

**Common recommendation patterns:**
- "Merge immediately" (all checks pass, no issues found)
- "Merge after fixing X and Y" (minor issues, can be fixed quickly)
- "Hand back to implementer with report" (needs deeper rework or implementer context)
- "Needs architectural rethinking" (fundamental design issues)
- "Blocked on external dependency" (waiting on something outside branch scope)

**Calibration matters:**
- Present ALL findings, even minor ones
- For each finding: suggest solution(s) or note if solution unclear
- Be honest about uncertainty ("unclear if this is intentional" vs "definitely wrong")
- Quantify impact where possible ("breaks 3 tests" vs "might affect performance")

**Jörn will deviate ~50% of the time** based on:
- Whether "improvements" actually help or hurt
- Whether reviewer vs implementer should fix
- Project priorities (deadline, dependencies, scope)

Your job: give Jörn the information he needs to make that call quickly.

#### Report Structure

**Header (at top):**

```markdown
# Review: [one-line summary of branch]

**Branch:** <path>
**Base:** local `main` at `<commit>`
**Date:** YYYY-MM-DD
```

**Detailed Content (50-100 lines, below header):**

Present findings organized by topic:

1. **Build Verification:**
   - Tests: status + evidence
   - Clippy: status + warnings (if any)
   - LaTeX compilation: status (if applicable)

2. **Deletion Verification** (if applicable):
   - Table showing each deletion with verdict
   - Evidence for intentional removals

3. **Code Quality:**
   - Convention adherence (Rust/Python/LaTeX)
   - Naming/documentation accuracy
   - Error handling
   - Performance claims (measurement required)

4. **Data Pipeline** (if applicable):
   - End-to-end trace (CSV → Python → JSON → LaTeX)
   - Consistency verification

5. **Mathematical/Statistical Correctness** (if applicable):
   - Formula verification
   - Test coverage of mathematical properties

6. **Strengths:** What the branch does well (be specific - what can Jörn trust without deep review?)

7. **Issues:** All problems found, with suggested solutions for each

8. **Pre-existing Issues** (if any): Problems that exist on `main`, not introduced by branch (separate section, clearly marked)

**Executive Summary (for Jörn, at bottom):**

```markdown
## Executive Summary

**Summary of findings:**
1. [Most significant finding with suggested solution]
2. [Second most significant]
3. [Third most significant]

**Recommendation:** [Your recommended action based on findings]

**Time investment:** Xmin review
```

**Report file location:** `docs/reports/YYYYMMDD-<topic>-review.md`

**Commit report:** Add and commit the report file to the branch (not to `main`). This preserves the review in the branch history. Only Jörn merges to `main`, so your commit stays on the branch until he decides to merge.

## Common Pitfalls

Avoid these failure modes from past reviews:

- ✗ **Treating recommendation as final decision** — Jörn makes the actual call (deviates ~50%)
- ✗ **Hiding findings to make recommendation look better** — Present ALL findings, Jörn needs full picture
- ✗ **Asking if obvious things are "good enough"** — R²=0.997 is obviously enough to declare a test result is a pass, and yet provides little evidence on its own that the test is bug-free. trust domain knowledge and common sense.
- ✗ **Manual LaTeX cross-reference checking** — Compilation catches broken refs
- ✗ **Comparing to failed attempts** — Compare to main, not to abandoned branches
- ✗ **Overly generous when critical paths untested** — Untested error paths = flag for Jörn
- ✗ **Performance claims without measurements** — Require benchmarks
- ✗ **Academic tangents** — Thesis constraints: correctness > readability > exhaustive theoretical analysis
- ✗ **Using `origin/main` as comparison base** — Always local `main`
- ✗ **Not stating git comparison base** — Always state: "Compared against local `main` at `abc1234`"
- ✗ **Forgetting to commit report** — Report is part of branch history

## Thesis Constraints

**Context:** Master thesis on Viterbo's conjecture, March 2026 deadline.

**Typical data:** Polytope4D with 5-16 facets. Research code, not production.

**Priority:** Correctness > performance.

**Don't suggest:**
- Theoretical numerical analysis (time cost > value) when empirical analysis gives even higher confidence anyway
- O(n²) documentation when n ≤ 16 (asymptote has no decision relevance there)
- Production features (auth, logging, deployment — not relevant)

**Do suggest:**
- Critical path tests (error paths, math properties, edge cases)
- Benchmarks for performance claims
- Robustness fixes (timeouts, limits, graceful degradation)

**Property-based testing:**
- Use proptest for ∀ statements: "∀ λ > 0: vol(λK) = λ⁴·vol(K)" → proptest
- Not for single examples

**Algorithm agreement (critical invariant):**
- If branch modifies any capacity algorithm (hk2017, billiard, tube), verify cross-algorithm agreement
- CLAUDE.md states: "Where domains overlap, algorithms must agree on computed capacity"
- Run `cargo test` across all capacity crates, not just the modified one
- Check for agreement tests in test suites

## File Location Decisions

**Investigation code:** `*_test.rs` with `#[ignore]` tag

**Session reports:** `docs/reports/<timestamp>-<topic>.md`

**Deprecated code:** `#[cfg(test)] mod deprecated`

**Review reports:** `docs/reports/YYYYMMDD-<topic>-review.md` (committed to branch)

See `file-location-decisions.md` (colocated) for full decision framework and examples.
