# File Location Decision Template

Use this template when deciding where files should live in the repo.

## Standard Locations

| File Type | Location | Reason |
|-----------|----------|--------|
| Investigation code | `crates/src/**/*_test.rs` with `#[ignore]` | Optional reading, high debug value |
| Session reports | `docs/reports/<timestamp>-<topic>.md` | Archived context, not in main tree |
| Decision rationale | Code doc comments (module-level `//!`) | Permanent context at usage site |
| Deprecated code | `#[cfg(test)] mod deprecated` | Test-only usage, clear signal |

## Decision Framework

| Factor | Weight Guidance | Questions to Ask |
|--------|----------------|------------------|
| Discoverability | HIGH for debugging/future work | Will future agents need this? When? |
| Cognitive load | HIGH for "always read" paths | Is this in mandatory reading (src/*.rs)? |
| Permanent value | HIGH for ongoing reference | Session artifact vs ongoing documentation? |
| Space cost | Context-dependent | LOC count, complexity added |

**Thesis context weighting:**
- Debugging speed > code cleanliness (1-month deadline)
- Future agent efficiency > pristine structure
- Correctness verification > performance optimization

## Examples from Complexity Review Session (2026-02-11)

1. **Investigation test file (qhull_boundedness_test.rs)**
   - Decision: Keep in `crates/src/geom/` as `*_test.rs`
   - Rationale: Saves 1-2hr debugging (HIGH) > 508 LOC cost (LOW for test module)

2. **Session reports (BOUNDEDNESS_INVESTIGATION.md, COMPLEXITY_REVIEW_REPORT.md)**
   - Decision: Move to `docs/reports/2026-02-11-complexity-review.md`
   - Rationale: Session artifact, not ongoing reference

3. **Deprecated functions (volume_divergence)**
   - Decision: Move to `#[cfg(test)] mod deprecated`
   - Rationale: Test-only usage, removes from production reading path

## Analysis Pattern

For each file/code location decision:

1. **Identify file type** (investigation, report, deprecated, etc.)
2. **Check standard location** (table above)
3. **Quantify tradeoffs**:
   - Discoverability impact (time saved/wasted)
   - Cognitive load (LOC in mandatory vs optional reading)
   - Permanent vs temporary value
4. **State weights** (HIGH/MEDIUM/LOW with justification)
5. **Recommend** with thesis context priority

Example quantification:
```
| Factor | Value | Weight |
|--------|-------|--------|
| Debugging time saved | 1-2 hours | HIGH |
| LOC cost | 508 lines | LOW (test module, optional reading) |
| Future agent efficiency | Immediate access vs git archaeology | HIGH |
| Maintenance burden | None (test-only, can break if unused) | LOW |
```

Decision: KEEP (HIGH benefits > LOW costs for thesis timeline)
