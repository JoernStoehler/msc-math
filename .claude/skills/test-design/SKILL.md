---
name: test-design
description: Workflow for designing correctness test suites for Rust code. Load when adding tests for new or changed mathematical code, not for simple bug fixes or plumbing tests.
---

# Test Suite Design Workflow

## 1. Identify what needs testing

Read the function's doc comments and its math.tex entry (`[lem:label]` cross-reference). Identify:
- What mathematical properties does this function claim to satisfy?
- What invariants does the type system enforce vs what's checked at runtime?
- What are the boundary cases for the input domain?

## 2. Design mathematical proposition tests

For each mathematical property, design a proptest that approximates the corresponding quantifier:
- "∀ polytopes K" → `proptest` generator over random polytopes
- "∀ A ∈ Sp(4)" → generator over random symplectomorphisms
- "∀ β > 0" → generator over positive weight vectors

Properties to consider:
- Invariance (symplectomorphism invariance, conformality)
- Consistency (algorithms agree where domains overlap)
- Idempotence / fixed points
- Monotonicity / ordering
- Edge cases of the mathematical definition

## 3. Design standard correctness tests

- Known-value tests from literature or hand computation
- Edge cases: minimal inputs (F=5), degenerate inputs, boundary values
- Regression tests for previously found bugs
- Panic tests (`#[should_panic(expected = "...")]`) for invalid inputs

## 4. Categorize by execution cost

| Category | What | Strategy | Suite |
|---|---|---|---|
| A: Input-output | Does f(x) = correct y? | Use fixtures, run in debug | Default (<5s) |
| B: Internal behavior | Does code run safely? | Small inputs (F≤6), debug mode | Default (<5s) |
| Expensive | Complex cases, F>8 | `#[ignore]`, release mode | `cargo test --release -- --ignored` |

## 5. Calibrate proptest parameters

Run each proptest in release mode. Target: <1s per test. Adjust `cases` if too slow or too fast.

## 6. Document

Test module header:
```rust
// Tests for {module}: {proposition or concern}.
// Proposition: {mathematical statement being tested}
// Reference: [lem:label] or [thm:label]
// Strategy: {fixture-based | proptest N cases | exhaustive for F≤6}
```

## Test exhaustiveness is Jörn's domain

Agents can brainstorm, implement, and debug tests. Agents cannot provide the exhaustiveness signal — which mathematical propositions the test suite needs to cover is Jörn's decision.