# Generator-zoo smoke

This owner is a bounded breadth-first geometry smoke for explicit planar
factor laws. It keeps the law, version, parameter, seed, row, attempt, and
bucket in every identity. Each accepted factor is translated/as-generated and
area-normalized to one; no recentering is applied. The product boundary is the
existing exact `SysLandscapePolytopeCache::from_lagrangian_product` constructor.
The local H/vertex construction plus that exact boundary enforce boundedness
and all prescribed product facets active; rejected attempts remain visible in
the report. No target value, bounce label, or retained row is used for
selection.

## Implemented laws

- **Current baseline control**: sample IID outward-normal angles uniformly on
  the circle and IID supports in `[0.8,1.2)`, then condition on all `n`
  prescribed facets being active. This is the factor law used by the current
  random-product producer and gives the atlas an explicit comparison arm.
- **Zonogon**: sample `r=n/2` distinct unoriented angles in `[0,pi)`, positive
  lengths `ell_j ~ U(0.5,1.5)`, and walk the boundary of
  `sum_j [-ell_j v_j, ell_j v_j]`. It has `2r` sides and is centrally
  symmetric, so this arm is run only for even side counts.
- **Primal point hull**: sample `n+4` IID points uniformly in the unit
  disk, take their monotone-chain convex hull, and accept exactly `n` strict
  hull vertices with the origin in the interior. This is a natural population
  conditioned on the named side count, not a fixed-vertex sampler.
- **Repulsive gap**: sample normalized IID Gamma gaps, i.e. a symmetric
  Dirichlet gap law with parameter `alpha`, cumulative angles, and equal
  supports. `alpha=1` is an IID angular control; `alpha=4,16` are repulsive
  gaps; `regular` is the exact regular limiting control. This is explicitly a
  Dirichlet repulsive-gap approximation, not a circular-beta/CUE sample.
- **Regular mutation**: start from a randomly rotated regular fan, apply four
  bounded angular/support mutations, and reject if cyclic gaps or active facets
  fail. The parameter records the chain length and scale.

The attempted surface-area/edge-measure closure law was abandoned: faithfully
sampling positive edge measures subject to `sum ell_i u_i=0`, while retaining a
named conditioning law, needs a new local geometry path and would otherwise
silently approximate the closure.

## Artifacts and command

```text
cargo run -p exp-sys-landscape --release --bin sys-datascience-generator-zoo-smoke -- \
  --out-dir experiments/sys-datascience/methods/generator-zoo-smoke/artifacts \
  --seed 20260714 --attempts 64 --rows-per-law 1
```

`factor-shapes.jsonl` is copy-local analyzer input. Every accepted factor row
has schema `factor-shape-row-v1`, `side_count`, `area_normalized`, and a CCW
`vertices_ccw` list. Its `population` field combines the law and knob setting,
so the atlas never pools distinct distributions. `product-smoke.jsonl` records exact product acceptance,
area, the exact incidence-volume witness in this compact smoke, attempts, and
timings. `batch-report.json` contains per-law acceptance and timing totals,
dispositions, and the producing source revision/dirty flag.

This is construction/provenance evidence only. The tiny rows do not estimate
the natural populations, validate conditioning probabilities at production
scale, or establish transfer/persistence of any `sys` relation.
