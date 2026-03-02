# q_error: Numerical vs exact accuracy for the KKT solver

## Original idea (Jörn)

We have an algorithm that is proven in theory to work for real numbers (and
rationals). But we use f64. So the idea is to have an experiment that:

1. At select nodes: compare numerical to the exact values.
2. At other nodes: look at the numerical error bounds we computed and that we
   RELY ON TO BE SMALL.

## Current state: broken

The experiment does neither of these things. It:
- Only looks at the winning (S, σ, β) per polytope (1 node per polytope)
- Prints diagnostic tables but asserts nothing
- Does not compare against exact values
- Does not verify error bounds across all nodes

### Problems identified (2026-03-02, Jörn)

The experiment's purpose is checking INTERNAL NUMERICAL ACCURACY. Jörn relied
on it asserting ALL we know about internal numerical accuracy as a way to be
confident in the lemmas. If it never asserted anything then the math is LESS
TRUSTWORTHY THAN WE THOUGHT SO FAR.

Specific problems:

1. **Only examines the winner** — calls `ehz_capacity()` and takes the single
   best (S, σ, β). The numerically interesting cases (rank-deficient, near-zero
   β, ill-conditioned) are typically NOT the winner.

2. **Does not assert anything.** Prints tables only. The `debug_assert!`s in
   `kkt.rs` check that E is small in absolute terms, but no test verifies that
   E is a valid upper bound on the actual error.

3. **Polytope selection not designed for diversity.** The selection criterion
   should be (runtime feasibility AND diversity of numerical conditions).

4. A lot of error bound assertions can be done in `assert!` in the library as
   well — doesn't all have to live in the experiment.

### What would fix this

- Loop over ALL (S, σ) pairs for small polytopes (F ≤ 8)
- For each KKT solve that returns Some, assert the error bounds are small
- At select nodes, compare against exact values
- Make it a `#[test]` or at minimum assert in the binary so violations fail
  loudly
- Move applicable assertions into `assert!` in the library code itself

### Local copy of build_kkt_system

Lines 26-57 contain a copy of `kkt::build_kkt_system` (which is `pub(crate)`).
Last synced: 2026-03-01. Maintenance risk.

## Input

Known polytopes filtered to F ≤ 10 (ehz_capacity is exponential). Currently 7.

## Output

Tables to stdout (no assertions).

## Run

```bash
cd experiments/ && cargo run --release --bin q_error
```
