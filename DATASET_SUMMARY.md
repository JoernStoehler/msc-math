# Dataset Generation Summary

Generated: 2026-02-11

## Dataset Size

- **Total polytopes**: 206
  - 6 known polytopes (simplex, cube, hk_counterexample, hko_pentagon, billiard_triangle, crosspolytope)
  - 200 random polytopes (50 each for F=5,6,7,8)
- **Valid capacity computations**: 205
  - Crosspolytope (16 facets) skipped due to computational cost

## Systolic Ratio Statistics

The systolic ratio is defined as sys(K) = c_EHZ(K)² / (2·vol(K)).

Viterbo's conjecture predicts sys(K) ≤ 1 for all convex bodies K.

**Statistics across all 205 valid polytopes:**
- Minimum: 0.001011
- Median: 0.154137
- Maximum: 1.047214

## Counterexamples to Viterbo's Conjecture

**Found 1 counterexample with sys > 1:**
- `hko_pentagon`: sys = 1.047214

**No new counterexamples among 200 random polytopes.**
All random polytopes satisfy sys < 1.

This confirms that counterexamples are rare and require specific constructions.

## Computational Performance

Median capacity computation time by facet count:

| Facet Count | Median Time (ms) | Sample Size |
|-------------|------------------|-------------|
| F=5         | 0.89            | 50          |
| F=6         | 3.65            | 50          |
| F=7         | 13.76           | 50          |
| F=8         | 86.35           | 50          |

Time complexity appears exponential in facet count, as expected from the Haim-Kislev 2017 algorithm.

## Acceptance Rates (Rejection Sampling)

Acceptance rates for random polytope generation across different height ranges:

| F | h ∈ [0.5, 2.0] | h ∈ [0.1, 5.0] | h ∈ [0.8, 1.2] |
|---|----------------|----------------|----------------|
| 5 | 5.70%          | 5.70%          | 5.70%          |
| 6 | 16.80%         | 16.20%         | 17.00%         |
| 7 | 33.40%         | 30.50%         | 34.40%         |
| 8 | 45.00%         | 37.10%         | 50.60%         |

Observations:
- Acceptance rate increases with facet count
- Height range has minimal impact on acceptance rate
- Low facet counts (F=5) have poor acceptance (~6%), making generation expensive

## Key Findings

1. **Viterbo conjecture holds for random polytopes**: None of the 200 random polytopes (F=5-8) violate the conjecture. The only counterexample is the known `hko_pentagon` construction.

2. **Systolic ratios are typically small**: The median sys ≈ 0.15, well below the Viterbo bound of 1.0.

3. **Computational cost grows exponentially**: Capacity computation time grows by roughly 4-7× per additional facet.

4. **Generation efficiency**: Rejection sampling works well for F≥6 (acceptance >16%), but struggles for F=5 (acceptance ~6%).

## Output Files

- `experiments/data/polytopes.jsonl`: 206 rows (1 per polytope)
- `experiments/data/acceptance.jsonl`: 18 rows (acceptance rates by config)
- `experiments/figures/sys_histogram.png`: Distribution of systolic ratios
- `experiments/figures/facet_vs_capacity.png`: Capacity vs facet count
- `experiments/figures/acceptance_rates.png`: Acceptance rates by facet count
