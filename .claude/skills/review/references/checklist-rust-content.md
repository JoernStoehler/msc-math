# Review Checklist: Rust Math-Code Correctness and Tests (Phase 2)

Detection rules for math-code correspondence and test quality. Run on clean files (after phase 1 fixes).

## Math-Code Correspondence

**Important limitation:** Agents may miss subtle mathematical errors. Be explicit about confidence levels and flag areas of uncertainty for Jörn.

### 1. Doc Comment Formulas Match Code

For each function with mathematical doc comments:
- Does the doc comment formula match the code's actual computation?
- Not aspirational ("the code should compute X") — what does it actually compute?
- Not approximate ("roughly equivalent to X") — exact correspondence required.
- "1:1" means literal structural correspondence, not just "inspired by."

### 2. Invariant Enforcement

For each type with doc comment invariants:
- Is the invariant enforced by the constructor (`new()`, `from()`, etc.)?
- Are fields private (preventing bypass)?
- Are there `assert!` or `debug_assert!` checks at relevant points?
- Can the invariant be violated through any public API?

### 3. Properties Have Tests

For each property stated in doc comments:
- Is there a corresponding test?
- Does the test actually check the stated property (not a weaker version)?

### 4. math.tex Cross-Reference Content

For each `[lem:label]`, `[thm:label]`, etc. in doc comments:
- Does the corresponding `\label{}` exist in the module's `math.tex`?
- Does the one-line English description match what the math.tex result actually says?
- Labels must never reference `thesis/` — code references its own module's `math.tex`.

Verification:
```bash
grep -r '\[lem:' crates/src/                  # find all cross-refs in code
grep -r 'label{lem:xyz}' crates/src/**/math.tex  # find the math.tex source
```

### 5. Math-Code Structural Correspondence

For types implementing mathematical definitions:
- Does the Rust type structure mirror the mathematical definition?
- Are field names meaningful in the mathematical context?
- Are operations (methods) correct translations of mathematical operations?

---

## Test Quality

**Note:** Do NOT decide whether the test suite is exhaustive — that's Jörn's domain. Check that existing tests follow conventions and flag obvious gaps.

### 6. Test Documentation

Every test MUST have at least a `///` doc comment stating the mathematical property it asserts.
- Detection: grep for `#[test]` and check the preceding lines for `///` doc comments. Missing doc comments are a violation.

### 7. Test Pattern Classification

Each test should fit one of five patterns:

| Pattern | Suite | Speed | Use for |
|---------|-------|-------|---------|
| Fixture-based property | Default (debug) | <1s | Math properties vs pre-computed fixture |
| Internal behavior smoke | Default (debug) | <5s | Small inputs (F <= 6) with debug checks |
| Expensive input-output | `#[ignore]`, release | ~1s release | Complex cases (F > 8) |
| Fixture generator | `#[ignore]`, release | minutes | Regenerate fixture after code changes |
| Staleness detector | Default (debug) | <1s | Warn if fixture out of sync |

Flag tests that don't clearly fit a pattern or use the wrong pattern for their purpose.

### 8. Math Proposition Tests

- Do proptest generators approximate the right mathematical quantifiers?
- Are the properties under test actual mathematical propositions?
- Are proptest parameters reasonable (not too few cases, not too many for the time budget)?

### 9. Category A vs B

For expensive functions:
- **Category A (input-output)**: uses fixtures or `#[ignore]` + release mode. Tests mathematical properties.
- **Category B (internal behavior)**: runs in debug mode with small inputs (F <= 6). Tests safe execution.

Flag tests that mix categories (e.g. testing output correctness in debug mode on large inputs).

### 10. Fixture Management

- Is there a staleness detector for each fixture?
- Is there a regenerator for each fixture?
- Are fixture files committed to git?

### 11. Test Input Diversity

- Are inputs diverse enough? (Not just simplices and cubes.)
- Are edge cases covered? (Degenerate polytopes, minimum facet count, maximum facet count in budget.)
- For proptest: are generators producing a useful distribution?

### 12. Test Suite Timing

- Default suite must stay < 3 min single-threaded.
- Individual default tests should be < 5s each.
- Flag tests in the default suite that might exceed these limits.
