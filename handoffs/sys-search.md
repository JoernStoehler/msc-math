# Handoff: sys-search experiment

**Date:** 2026-03-26
**Branch:** `sys-search` (1 commit, ready to merge)
**Status:** Dev run complete. Pipeline works. Ownership returning to Jörn.

## What exists

Working end-to-end pipeline: `run.rs` (680 lines) generates polytopes, runs gradient ascent in a-space with overshoot + wiggle boundary-crossing, writes per-seed summaries and per-iteration trace to JSONL. `analyze.py` produces 5 figures and a summary table. `logbook.md` documents design, findings, and open questions.

Dev run: 42 seeds (10 general + 12 Lagrangian + 20 warm starts). Best sys = 0.933. No sys > 1. Zero panics.

## What the dev run revealed

The experiment started as "run gradient ascent with boundary-crossing, see if sys > 1." The dev run answered that (no), but surfaced a more interesting question: **what does the sys landscape look like, and how should we search it?**

Key observations that reframe the experiment:
- Local optima are created by combinatorial structure (which orbit is optimal), not smooth geometry. Gradient ascent converges fast within each cell but stops at boundaries.
- The gradient direction is uninformative past a combinatorial boundary (overshoot never wins).
- Random perturbation (wiggle) works, but the 5% scale is arbitrary and we don't understand why it works — is it basin hopping between distinct optima, or expensive saddle escape within one basin?
- We don't know how many local optima exist, how large their basins are, or whether there's funnel structure.

## What's not done

The logbook's research questions are partially answered:
- RQ1 (single-step characterization): answerable from trace data but not yet analyzed in depth
- RQ2 (does any trajectory reach sys > 1): no, at dev scale
- RQ3 (strategy comparison): overshoot vs wiggle done, but the strategy space is much larger than these two
- RQ4 (general vs Lagrangian): dev-scale data suggests similar behavior, not statistically significant

The experiment needs a second phase focused on landscape characterization and strategy design. The logbook's "Open questions" and "Landscape observations" sections document what we know and don't know.

## Technical notes

- Everything is in dual-vertex (a) space. Gradient d(sys)/d(a_k) via library's `capacity_derivatives_a` and `volume_derivatives_a`. Steps: a_k(t) = a_k + t * d_k. Step bound linearized via A_D^{-1}.
- For Lagrangian products, gradient direction is projected to preserve subspace structure. Billiard backend used (fast).
- Wiggle perturbs dual vertices directly. `Polytope4D::from_f64` validates results — no catch_unwind, no panic suppression.
- The trace doesn't currently record which orbit (permutation) is optimal at each step. Adding this would distinguish "basin hopping" from "same-basin re-entry" after wiggle. Easy code change (~5 lines in `gradient_ascent`).
- Constants `N_GENERAL`, `N_LAGRANGIAN_PER_BUCKET`, `N_WARM_STARTS` are set to dev scale (10, 4, 20). Increase for production.

## Sibling experiments

- **gradient-correctness** (not started): validates the gradient formula this experiment relies on.
- **combinatorial-boundaries** (not started): characterizes the boundaries this experiment crosses. Its findings about boundary density and sys continuity across boundaries would directly inform search strategy design.
