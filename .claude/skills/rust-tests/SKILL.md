---
name: rust-tests
description: Testing conventions for Rust code in crates/. Load when writing, editing, or reviewing *_test.rs files or test fixtures. Covers testing philosophy, expensive function testing patterns, test organization, fixtures, and documentation requirements.
---

# Rust Test Conventions

## Testing Philosophy

Two classes of tests, both applied excessively:

1. **Math proposition tests** (due diligence falsification): proptest generators approximate mathematical quantifiers ("∀ polytopes K", "∀ A ∈ Sp(4)", etc.). Properties under test are mathematical propositions (e.g. J^2 = -I, symplectomorphisms preserve capacity).
2. **Standard correctness tests**: Rust best practices for correctness-critical code — edge cases, invariant checking, regression tests.

**Test exhaustiveness is Jörn's domain.** Jörn must design which mathematical propositions the test suites need to cover. Agents CAN brainstorm, implement, and debug mathematical proposition tests. Agents CANNOT provide the exhaustiveness signal.

## Testing Expensive Functions

For expensive functions (e.g., `ehz_capacity()` with exponential cost), split tests into two categories:

### Category A: Input-Output Behavior
**What it tests:** Does `f(input)` return the correct output value? Mathematical properties (conformality, monotonicity, etc.).

**Test strategy:**
- **Preferred:** Use fixtures (pre-computed in release mode), run tests in debug suite (fast, <1s)
- **Alternative:** Mark `#[ignore]`, run in release mode (slow but thorough)

**Why:** We only care about the result, not how the code executes. Release mode gives 50-80x speedup.

### Category B: Internal Behavior
**What it tests:** Does the code execute safely without crashes, bounds errors, overflow, or assertion failures?

**Test strategy:**
- Run in debug mode (enables debug_assert!, overflow checks, bounds checks)
- Use small inputs (F ≤ 6 for capacity) to stay fast (<5s per test)

**Why:** Testing that code *runs correctly*, not that it *produces correct output*.

## Test Organization

**Inline test modules:** Tests live in `#[cfg(test)] mod tests { ... }` at the bottom of each source file. Each module covers the concerns for that source file. Modules that test multiple concerns (e.g., `hk2017/mod.rs`) use descriptive names: `tests_literature`, `tests_conformality`, etc.

| Pattern | Suite | Speed | Use for | Example |
|---------|-------|-------|---------|---------|
| **Fixture-based property** | Default (debug) | <1s | Math properties vs pre-computed fixture | conformality tests, symplectic invariance tests |
| **Internal behavior smoke** | Default (debug) | <5s | Small inputs (F ≤ 6) with debug checks | KKT edge case tests |
| **Expensive input-output** | `#[ignore]`, release | ~1s release | Complex cases (F > 8), fixture unsuitable | `pentagon_capacity()` |
| **Fixture generator** | `#[ignore]`, release | minutes | Regenerate fixture after code changes | `generate_capacity_fixtures.rs` |
| **Staleness detector** | Default (debug) | <1s | Warn if fixture out of sync | `fixture_staleness_check()` |

## Test Documentation

**Test module header (top of each `#[cfg(test)] mod tests` block):**
```rust
// Tests for {module}: {proposition or concern}.
//
// Proposition: {mathematical statement being tested}
// Reference: [lem:label] or [thm:label]
//
// Strategy: {fixture-based | proptest N cases | exhaustive for F≤6}
```

Every individual test MUST have at least a doc comment stating the mathematical property it asserts. Tests for expensive or complex functions should additionally explain why they use their execution mode (debug/release/fixture), why they use their specific input, and relationship to other tests (if any).

**Panic tests:** Use `#[should_panic(expected = "...")]` to test that invalid inputs are rejected. Always include the `expected` string to avoid false passes from unrelated panics.

## Fixtures

**Fixture location:** `tests/fixtures/capacity_dataset.json` (committed, 33 polytopes with precomputed capacities, scaled variants for conformality tests).

**Fixture regeneration:** `generate_capacity_fixtures.rs` (in `algorithms/hk2017/`) contains the `#[ignore]` test that regenerates the fixture dataset in release mode.

## Test Suites

| Suite | Command | When to run |
|-------|---------|-------------|
| **Default** | `cargo test --lib` | Every iteration |
| Regenerate capacity fixture | `cargo test --release generate_capacity_fixtures -- --ignored` | After changes to `ehz_capacity()` |
| Expensive capacity tests | `cargo test --release -- --ignored` | After capacity algorithm changes |
| All ignored tests | `cargo test -- --ignored` | Full validation |

Target: default suite <3 min single-threaded. Individual default tests must each complete in <5s.

## Proptest Parameters

Proptest parameters must be calibrated for the time budget:
- Too few cases: the property is under-tested and bugs can slip through.
- Too many cases: the test exceeds the 5s-per-test budget and slows the default suite.
- When setting `cases`, verify the test runs in <5s in debug mode on the target machine before committing.

## Math-Code Correspondence (for test design)

Rust types, function signatures, and function bodies 1:1 correspond to mathematical definitions. "1:1" means literal structural correspondence. Tests should verify this correspondence:
- Doc comment formulas must match code's actual computation
- Invariants stated in doc comments must be enforced by types/constructors/assert!/debug_assert!
- Properties stated in doc comments must have corresponding tests
