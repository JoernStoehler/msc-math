# HKO-Neighborhood: Local Maximality of HKO2024

Is HKO2024 (the only known Viterbo counterexample, sys = 1.047) a local maximum of the systolic ratio? In its 10-facet parameter space? Even when allowing facet-splitting into 11-facet polytopes?

## Status
Complete

## Design

- **Phase A (F=10 space):** Compute analytical gradients d(sys)/dh and d(sys)/dn at HKO2024, then run gradient ascent
- **Phase B (F=11 facet-splitting):** Cut HKO2024 facets with random hyperplanes at small epsilon, check if sys increases
  - 536 successful cuts: 2 representative facets x 100 directions + 48 mixed + 20 control, each at epsilon in {1e-3, 1e-4}

## Key findings

**Phase A (F=10 space):** HKO2024 is a **local maximum** in h-space (normals fixed).
- All 10 height derivatives ∂sys/∂h_k are negative (range: -0.52 to -1.68)
- Normal gradient |∇sys_n| ≈ 1.53 is nonzero (not a critical point in full (n,h) space)
- Gradient ascent converges in one step with Δsys ≈ 5e-9 (machine precision)
- 44 near-optimal orbits, all at essentially the same action (gaps < 5e-14)
- 364 total valid orbits; the orbit structure is highly degenerate
- **Subgradient structure:** Gradients are orbit-dependent (computed from best orbit
  S={0,2,4,6,8,9}). Facets {1,3,5,7} not in S have zero normal gradient (envelope
  theorem gives zero for unvisited facets). The broken C5 symmetry in gradients is
  entirely due to orbit choice, not polytope geometry.

**Phase B (F=11 facet-splitting):** All tested sub-polytope cuts decrease sys.
- 536 successful cuts: 2 representative facets (Q: 0, P: 5) × 100 directions + 48 mixed + 20 control, each at ε ∈ {1e-3, 1e-4}
- All 536 cuts decrease sys (best Δsys = -4.43e-9, worst = -3.18e-4)
- Larger epsilon (deeper cuts) cause larger sys decrease
- **Caveat:** Phase B only tests sub-polytopes K' ⊊ K. Joint perturbations
  (relax existing halfspace + add cut) are not tested — Phase A's negative
  h-derivatives suggest these also cannot help, but this is not proven.

## Run

```bash
cd experiments/
cargo run --bin hko_neighborhood --release
python3 hko-neighborhood/hko_neighborhood.py
```

## Files

| File | Purpose |
|------|---------|
| `hko_neighborhood.rs` | Rust binary: sensitivity, ascent, splitting |
| `hko_neighborhood.py` | Python: figures + analysis |
| `hko-neighborhood-sensitivity.jsonl` | Gradients at HKO2024 |
| `hko-neighborhood-ascent.jsonl` | Gradient ascent trajectory |
| `hko-neighborhood-splitting.jsonl` | Facet-splitting data |
| `hko-neighborhood-gradient.png` | Figure: gradient visualization |
| `hko-neighborhood-orbits.png` | Figure: orbit structure |
| `hko-neighborhood-splitting.png` | Figure: facet-splitting results |

## Key observation

All d(sys)/dh_k < 0: increasing any height decreases sys (normals fixed).
Decreasing heights shrinks K toward degeneracy. The normal gradient is nonzero
(|nabla sys_n| = 1.53), so HKO2024 is not a critical point on the (n,h) parameter
space. However, gradient ascent in the joint (h,n) direction converges
immediately (delta sys = 5e-9), suggesting HKO2024 sits near a boundary of the
feasible region where no smooth deformation improves sys.

## Known limitations

- Phase B only tests sub-polytopes K' subset K; joint perturbations (relax existing halfspace + add cut) are not tested
- Normal gradient is orbit-dependent (computed from best orbit S={0,2,4,6,8,9})
- Facets not in the optimal orbit have zero normal gradient (envelope theorem)
