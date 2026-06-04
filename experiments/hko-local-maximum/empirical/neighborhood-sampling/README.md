# HKO Neighborhood Sampling

This experiment groups nearby-polytope random samplers under one Rust binary.
Each sampler writes artifacts into its own subfolder.

## Samplers

| Sampler | Command | Artifact folder | Meaning |
| --- | --- | --- | --- |
| `m10` | `hko-neighborhood-sampling m10` | `m10/` | General fixed-`F=10` dual-vertex perturbations near HKO. |
| `m11` | `hko-neighborhood-sampling m11` | `m11/` | Add one cutting facet to move from `F=10` to `F=11`. |
| `m10-lagrangian-product` | `hko-neighborhood-sampling m10-lagrangian-product` | `m10-lagrangian-product/` | Fixed-`F=10` perturbations that preserve the 5 `q` + 5 `p` Lagrangian-product structure. |
| `m10-lagrangian-product-probe` | `hko-neighborhood-sampling m10-lagrangian-product-probe` | `m10-lagrangian-product/` | Radial boundary probe in the same Lagrangian-product `M_10` family. |

There is no current pure `m11-lagrangian-product` sampler. A true such sampler
would add a product-preserving facet, giving `6 q + 5 p` or `5 q + 6 p`.

## Files

- `main.rs`: dispatches to one sampler.
- `samplers/`: sampler implementations.
- `m10/`, `m11/`, `m10-lagrangian-product/`: tracked data, figures, analysis
  scripts, and job scripts owned by each sampler.
