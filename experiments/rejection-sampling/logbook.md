# Rejection Sampling: Logbook

## Motivation

To probe Viterbo's conjecture computationally, we need large datasets of random convex polytopes in R^4. We generate these by rejection sampling: sample a candidate halfspace representation, then validate that it defines a bounded, irredundant polytope. This experiment measures acceptance rates across facet counts and height ranges, to calibrate the sampling parameters used by downstream experiments (random-sweep, random-product-sweep).

## Status

**Complete.** Acceptance rates characterized; results used to set parameters in other experiments.

## How to run

```bash
cd experiments/ && cargo run --bin acceptance_sweep --release
```

No Python script or figure. The .tex writeup uses a table directly from the JSONL data.

### Files

| File | Role |
|------|------|
| `run.rs` | Rust binary: generates dataset |
| `acceptance_sweep_test.rs` | Unit tests for the sweep function |
| `math.tex` | Thesis writeup (sampling procedure, boundedness proposition + proof, acceptance rate table) |
| `acceptance.jsonl` | Dataset (18 rows: 6 facet counts x 3 height ranges) |

## Design

- **Attempts:** 1000 per (F, height_range) configuration.
- **Height ranges:** narrow [0.8, 1.2], medium [0.5, 2.0], wide [0.1, 5.0].
- **Facet counts:** F = 5, 6, 7, 8, 9, 10.
- **Total configurations:** 18 (6 x 3).
- **Seed:** 42 (deterministic).

## Findings

1. **Narrow height range gives highest acceptance:** >72% at F=10, vs 50% (medium) and 28% (wide).
2. **Acceptance increases with F for narrow and medium ranges:** From 5.7% at F=5 to 72.9% (narrow) / 50.4% (medium) at F=10.
3. **Acceptance is insensitive to height range at low F:** At F=5, all three height ranges give exactly 5.7%. This is because rejection is dominated by the boundedness check, which depends only on normal directions, not heights.
4. **Wide height range hurts at high F:** The irredundancy check becomes the bottleneck when heights vary over a 50x range, causing facets to lose enough incident vertices.
5. **Practical consequence:** For F >= 7 and moderate height ranges, rejection sampling is efficient (acceptance >= 30%). Even the worst case (F=5, 5.7%) requires only ~18 attempts per accepted polytope.

## Known limitations

- 1000 attempts per configuration; may not capture rare events.
- Only F=5..10 tested; higher facet counts not covered.
- Fixed seed (42) for reproducibility.

## Related experiments

- `random-sweep`: Uses the calibrated parameters from this experiment to generate random polytopes.
- `random-product-sweep`: Uses similar height range parameters.
