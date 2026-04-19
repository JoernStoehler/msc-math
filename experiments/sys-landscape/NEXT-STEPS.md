# Sys-Landscape Next Steps

## Active Objective

Implement the witness-search extension as the next high-leverage package-level thread, with minimal surface area outside current packages.

## Current Thread (single, contiguous)

1. Implement witness-oracle instrumentation in fixed-F ascent code paths.
   - Add configurable outputs for top-`m` and within-gap witness sets plus optional incumbent set input.
   - Persist for every exact evaluation: exact `sys`, minimizing witness, near-active witness set, and runtime diagnostic fields.
2. Reuse benchmark pipeline before opening new continuation methods.
   - Use `variable-f-ascent` endpoints and existing exact-evaluated rows as the first benchmark bank.
   - Compare minimizer-only, top-`m`, within-gap, parent cache, and hybrid witness sets.
   - Track exact-call reduction, hit rates, and safe-rejection rate from upper bound checks (`U_A(K)<1`).
3. Add reduced-model descent loop as the first warm replacement candidate.
   - Start with soft-min local model and serious/null-step logic on local witness sets.
   - Require exact-check fallback on candidate acceptance; keep reduced model as triage/guide only.
4. Add witness-guided F continuation as the structured successor to random continuation.
   - Compare against existing `variable-f-ascent` and HKO-style `cut-and-ascent` baselines.
   - Use lifted parent witnesses in child `F+1` runs; do not restart witness state from scratch.

## Blocking Conditions

- The oracle instrumentation must be explicit in experiment outputs before any benchmark comparison is considered complete.
- Any continuation change is blocked until instrumentation and prefilter benchmarks are running on a fixed seed bank; otherwise comparisons are not apples-to-apples.

## Stop Conditions

- If reused local witness sets do not reduce exact calls or do not provide safe, measurable pruning in the same benchmark bank, stop witness-guided continuation and shift to structural-family experiments using the existing package boundaries.
- If exact checks fail to confirm improvements above historical maxima, retain results as negative evidence and document explicitly in task notes.

## Concrete Files/Commands to Touch First

- `experiments/sys-landscape/gradient-ascent-general/main.rs`
- `experiments/sys-landscape/gradient-ascent-products/main.rs`
- `experiments/sys-landscape/variable-f-ascent/main.rs`
- `experiments/sys-landscape/feature-pattern-search/analyze.py` (only if normalized schema changes are needed later)
- `experiments/sys-landscape/normalized-dataset` scripts/binaries for new source packet ingestion
- `tasks`: `TASKS.md` entries under the sys-landscape witness-search bundle should mirror this order and evidence output expectations.
