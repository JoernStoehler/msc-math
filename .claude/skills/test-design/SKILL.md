---
name: test-design
description: Workflow for designing correctness test suites for Rust code. Load when adding tests for new or changed mathematical code, not for simple bug fixes or plumbing tests.
---

# Test Suite Design

## Key decisions

- **Proptest for quantified properties.** "∀ polytopes K" → proptest generator over random polytopes. "∀ A ∈ Sp(4)" → generator over symplectomorphisms. "∀ β > 0" → generator over positive weights.
- **Property families:** invariance, consistency (algorithms agree), idempotence, monotonicity, edge cases of the mathematical definition.
- **Known-value tests** from literature or hand computation. Edge cases: minimal inputs (F=5), degenerate inputs.
- **Test exhaustiveness is Jörn's domain.** Agents implement and debug tests. Which mathematical propositions need coverage is Jörn's decision.

## Cost categorization

| Category | Strategy | Suite |
|---|---|---|
| A: Input-output (f(x) = y?) | Fixtures, debug mode | Default (<5s) |
| B: Internal behavior (safe execution?) | Small inputs (F≤6) | Default (<5s) |
| Expensive (F>8, complex) | `#[ignore]`, release mode | `cargo test --release -- --ignored` |

Calibrate proptest: target <1s per test in release mode.
