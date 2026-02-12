# timing_model.py — EHZ capacity timing model

Fits an exponential model T(F) = a · b^F to measured EHZ capacity computation times.

## Model

T(F) = 7.73 × 10⁻⁸ · 5.74^F seconds (R² = 0.9999)

Fitted via log-linear regression on timing data from `experiments/profiling/timing_data.csv`.

**Note:** The timing data was collected before the Lagrangian/symplectic product split.
The `symplectic_tri_sq` row (F=7, capacity=1.5) is actually the Lagrangian product.
The current `symplectic_triangle_square()` returns a different polytope (capacity=1.0).
Timing would be similar (same facet count), but capacity values would differ if regenerated.

## Measured vs predicted

| F  | Measured (ms) | Predicted (ms) |
|----|---------------|----------------|
| 5  | 0.84          | 0.52           |
| 6  | 5.41          | 3.00           |
| 7  | 20.8          | 17.2           |
| 8  | 89.7          | 98.8           |
| 10 | 3003          | 3257           |

## Practical limits

- F ≤ 8: sub-second, suitable for large datasets (1000+)
- F = 9: ~0.4s/polytope, feasible for 50–100 polytopes
- F = 10: ~2s/polytope, feasible for 20–50 polytopes
- F = 12: ~1 min/polytope (extrapolated), borderline for small studies
- F ≥ 14: hours per polytope, impractical without algorithmic improvements
