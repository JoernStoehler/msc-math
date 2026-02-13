---
name: review-branch
description: Independent critical review of code/docs in worktree branches. Present calibrated findings and recommend action. Covers Rust, Python, LaTeX, data pipelines.
---

# Branch Code Review

## Core Principle

Your job: Find all problems, suggest solutions, make a calibrated recommendation.

Jörn reads your report and makes the final decision (he deviates ~50% of the time based on project context you lack).

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

For complex reviews (>1hr estimated), write a plan file before deep analysis.

**Plan file structure:**

1. **Executive Summary (for Jörn, at top):**
   - What changed (3-5 bullet points)
   - Key concerns to investigate
   - Estimated review time
   - Preliminary risk assessment

2. **Detailed Plan (for agent execution):**
   - Full checklist of items to verify
   - Methodology for each phase
   - Expected evidence to collect
   - Known unknowns to investigate

**When to write a plan:**
- Branch touches >3 files or >2 languages
- Includes deletions requiring verification
- Changes affect data pipelines or mathematical correctness
- Estimated review time >1 hour

**Plan file location:** `~/.claude/plans/<session-id>.md`

Write plan, discuss with Jörn if needed (via AskUserQuestion), then execute.

## Methodology

### Phase 0: Check for Existing Analysis (before exploration)

Look for `docs/reports/*`, `*_INVESTIGATION.md`, `*_REVIEW*.md` in worktree.
If found: read first, verify claims, explore gaps. Saves 20min vs redundant exploration.

### Phase 1: Fast Checks (5-10 min)

**Build verification:**
- Rust: `cd crates && cargo test && cargo clippy`
- Python: `ruff check experiments/ && pytest experiments/` (if applicable)
- LaTeX: `cd thesis && latexmk` (if applicable)

Run tests per CLAUDE.md "Commands" section for the repository.

**If tests fail:** Stop and request fixes before continuing review.

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
- Check per: `crates/CLAUDE.md`
- Key rules: iterators over for loops, minimal mutability, types encode invariants, colocated tests
- Property tests for ∀ statements (proptest)

**Python conventions:**
- Check per: `experiments/CLAUDE.md`
- Key rules: header docstrings (Goal/Input/Output), path conventions (REPO_ROOT), self-contained scripts

**LaTeX conventions:**
- Check per: `thesis/CLAUDE.md`
- Key rules: file headers (identity/sources/structure), no false `% Jörn:` markers, proof structure

**Performance claims:**
- Require measurement: "~1ms" is claim, "Benchmark shows 1.5ms for F=5-12" is measured
- Add benchmark if claim exists without measurement

**Test coverage:**
- Critical paths tested (error paths, math properties, degenerate cases)
- Core cases covered (happy path, known-good inputs, basic errors)
- Edge cases tested (property-based tests, boundaries, robustness)

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
- Read thesis .tex to acquire domain knowledge
- Trust levels: "Jörn-verified" > "speculative" > "in-progress" (marked in LaTeX comments)
- We form conjectures based on mathematical intuition + empirical evidence

**Verification approach:**
- Check formulas match literature OR
- Check tests validate mathematical properties (scaling laws, symmetries)
- For statistical code: verify regression formulas (e.g., log-linear for exponential), R² calculation (1 - ss_res/ss_tot), median computation (even/odd handling)

**DO NOT ask if R²=0.997 is "good enough"** — it obviously is. Trust domain knowledge.

**Bayesian mindset:**
- R² is a tool, not a Frequentist religion
- Interpolation vs extrapolation: claims within fitted domain are ~99% trustworthy
- Linear scaling (dataset cost = polytope cost × count): 99% prior

#### Naming and Documentation Accuracy

Check public symbols accurately describe behavior:
- Function names match what code does (not aspirational)
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

**Executive Summary (for Jörn, at top):**

```markdown
# Review: [one-line summary of branch]

**Branch:** <path>
**Base:** local `main` at `<commit>`
**Date:** YYYY-MM-DD

**Summary of findings:**
1. [Most significant finding with suggested solution]
2. [Second most significant]
3. [Third most significant]

**Recommendation:** [Your recommended action based on findings]

**Time investment:** Xmin review
```

**Detailed Content (50-100 lines):**

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

6. **Strengths:** What the branch does well

7. **Issues:** All problems found, with suggested solutions for each

8. **Recommendation:** Final recommendation with rationale

**Report file location:** `docs/reports/YYYYMMDD-<topic>-review.md`

**Commit report:** Add and commit the report file to the branch.

## Common Pitfalls

Avoid these failure modes from past reviews:

- ✗ **Treating recommendation as final decision** — Jörn makes the actual call (deviates ~50%)
- ✗ **Hiding findings to make recommendation look better** — Present ALL findings, Jörn needs full picture
- ✗ **Asking if obvious things are "good enough"** — R²=0.997 is obviously excellent, trust domain knowledge
- ✗ **Manual LaTeX cross-reference checking** — Compilation catches broken refs
- ✗ **Comparing to failed attempts** — Compare to main, not to abandoned branches
- ✗ **Overly generous when critical paths untested** — Untested error paths = flag for Jörn
- ✗ **Performance claims without measurements** — Require benchmarks
- ✗ **Academic tangents** — Thesis constraints: correctness > theoretical analysis
- ✗ **Using `origin/main` as comparison base** — Always local `main`
- ✗ **Not stating git comparison base** — Always state: "Compared against local `main` at `abc1234`"
- ✗ **Forgetting to commit report** — Report is part of branch history

## Thesis Constraints

**Context:** Master thesis on Viterbo's conjecture, March 2026 deadline.

**Typical data:** Polytope4D with 5-16 facets. Research code, not production.

**Priority:** Correctness > performance.

**Don't suggest:**
- Theoretical numerical analysis (time cost > value)
- O(n²) documentation when n ≤ 16 (overkill)
- Production features (auth, logging, deployment — not relevant)

**Do suggest:**
- Critical path tests (error paths, math properties, edge cases)
- Benchmarks for performance claims
- Robustness fixes (timeouts, limits, graceful degradation)

**Property-based testing:**
- Use proptest for ∀ statements: "∀ λ > 0: vol(λK) = λ⁴·vol(K)" → proptest
- Not for single examples

## File Location Decisions

**Investigation code:** `*_test.rs` with `#[ignore]` tag

**Session reports:** `docs/reports/<timestamp>-<topic>.md`

**Deprecated code:** `#[cfg(test)] mod deprecated`

**Review reports:** `docs/reports/YYYYMMDD-<topic>-review.md` (committed to branch)
