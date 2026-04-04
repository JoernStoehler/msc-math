# Cut-and-Ascent: Logbook

## Motivation

The facet-splitting experiment tested 536 F=10→F=11 cuts on HKO2024 and found all decrease sys. But it did not run gradient ascent on the F=11 polytopes afterward — the optimizer might recover and push sys higher than HKO2024's 1.0472 in the richer F=11 space.

This experiment extends facet-splitting by adding gradient ascent after each cut: cut HKO2024 → create F=11 polytope → run gradient ascent → check if final sys exceeds 1.0472.

If any trial improves over HKO2024, it would weaken the evidence for HKO2024 being a local maximum (in the space of polytopes with ≤11 facets).

## Status

**Scaffolded.** Binary and logbook created. Preliminary data from 5 trials (migrated from variable-f-ascent experiment): 0/5 improved.

## How to run

```bash
cargo run -p exp-hko-local-maximum --release --bin hko-cut-and-ascent           # resume
cargo run -p exp-hko-local-maximum --release --bin hko-cut-and-ascent -- --fresh # rerun
```

## Files

| File | Role |
|------|------|
| run.rs | Binary: cut HKO2024 + gradient ascent on F=11 polytope |
| logbook.md | This file |

## Preliminary findings (from variable-f-ascent, 2026-04-04)

5 random facet placements on HKO2024 with ε=1e-3, followed by gradient ascent (overshoot + wiggle escape, same algorithm as gradient-ascent-general). All 5 trials converged back to sys≈1.0472 with Δ ≈ -0.0000. All added facets remained non-redundant (active).

This is consistent with facet-splitting's finding (0/536 cuts improved sys without ascent) and suggests that gradient ascent does not change the picture. However, 5 trials with random placement is a small sample.

## Ideas to flesh this out

1. **Scale up:** 50-100 random placements for statistical confidence. At ~10s per trial, 100 trials ≈ 17 min.
2. **Gradient-informed placement:** Use the subdifferential analysis from `second-order/` to place the new facet near directions where the gradient is flat (subdifferential directions). These are the directions where adding a new degree of freedom could break the flat barrier.
3. **Larger ε:** Try ε = 1e-2 and 1e-1 for deeper cuts. The thin-sliver problem observed in variable-f-ascent RQ2 suggests shallow cuts may not provide enough room for the optimizer.
4. **Multiple cuts:** Add 2-3 facets simultaneously (F=10→F=12 or F=13) for more degrees of freedom.
5. **Combine with second-order analysis:** The second-order experiment found 15 flat subdifferential directions with negative curvature. Do these directions correspond to facet placements that don't help? Or are there untested directions that might?

## Related experiments

- `facet-splitting/` — F=10→F=11 cuts without ascent; 0/536 improved
- `gradient-analysis/` — gradient at HKO2024 is zero; 1-step convergence
- `second-order/` — all 15 flat directions have negative curvature
- `variable-f-ascent/` (exp-sys-landscape) — the source of the gradient ascent algorithm and preliminary HKO2024 data
