# Deletion Verification Examples

Examples from past reviews to calibrate judgment.

## Example 1: Replacement with Broader Scope

**Deleted:** `time_capacity.rs` (34 LOC)
- Purpose: Times 6 known polytopes
- Output: CSV with timing data

**Replacement:** `benchmark.rs` (78 LOC)
- Purpose: Times 76 random polytopes (F=5-12)
- Output: CSV with timing data

**Verdict:** ✓ Appropriate
- Different purpose (known → random)
- Broader scope (6 → 76 polytopes)
- Same output format (CSV)

## Example 2: Intentional Functionality Removal

**Deleted:** `project_dataset_size()` function in `timing_model.py`
- Purpose: Estimates how many polytopes can be generated in X hours

**Replacement:** None

**Evidence of intentional removal:**
- "projections" field removed from `timing_model.json`
- Commit message: "Remove dataset projection (superseded by practical limits table)"

**Verdict:** ✓ Intentional cleanup
- Feature removed, documented in commit
- Practical limits table in LaTeX provides similar info

## Example 3: Dead Code Deletion

**Deleted:** `profile_capacity.rs` (16 LOC)
- Purpose: Ad-hoc profiling stub (hypercube + crosspolytope, 10 iterations, no output)

**Replacement:** None

**Verdict:** ✓ No replacement needed
- Debugging code
- No integration with rest of codebase
- Not referenced anywhere

## Example 4: Content Migration

**Deleted:** `timing_model.md`, `crosspolytope_experiment.md` (experiment writeups)

**Replacement:** `thesis/experiments/benchmark.tex` (LaTeX section)

**Verdict:** ✓ Content migrated
- Writeups moved from .md to .tex
- Content preserved (crosspolytope note in benchmark.tex final paragraph)
- `.tex` is source of truth for thesis

## Anti-pattern: Lost Functionality

**Deleted:** `validate_input()` function

**Replacement:** None

**Evidence:**
- No commit message explaining deletion
- Other code still calls `validate_input()` (would break)
- No tests added for new validation approach

**Verdict:** ✗ NEEDS WORK
- Functionality lost unintentionally
- Breaking change
