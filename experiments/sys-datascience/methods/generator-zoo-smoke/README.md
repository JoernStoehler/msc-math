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
- **Regular mutation**: start from a randomly rotated regular fan and apply
  exactly four steps. At each step, angle increments are independent
  `N(0,scale^2)` values clipped to `+/-0.2*(2*pi/n)`, while supports are
  multiplied by `exp(0.5*Z)` with independent `Z~N(0,scale^2)`; the log-support
  increment is therefore unbounded. After each step, sort angles and reject
  cyclic gaps below `0.2*(2*pi/n)` or at least `pi`; the final vertex/H
  conversion conditions on every facet being active. The parameter records the
  chain length and scale (`steps=4,scale=0.03` in this smoke).

The attempted surface-area/edge-measure closure law was abandoned: faithfully
sampling positive edge measures subject to `sum ell_i u_i=0`, while retaining a
named conditioning law, needs a new local geometry path and would otherwise
silently approximate the closure.

## Artifacts and command

```text
cargo run -p exp-sys-landscape --release --bin sys-datascience-generator-zoo-smoke -- \
  --out-dir experiments/sys-datascience/methods/generator-zoo-smoke/artifacts \
  --seed 20260714 --attempts 64 --rows-per-law 5
```

`factor-shapes.jsonl` is copy-local analyzer input. Every accepted factor row
has schema `factor-shape-row-v1`, `side_count`, `area_normalized`, and a CCW
`vertices_ccw` list. Its `population` field combines the law and knob setting,
so the atlas never pools distinct distributions. The retained reviewed smoke
has 115 product rows: 110 accepted and 5 exhausted; all five exhaustions are
the primal-hull `3x3` bucket. The accepted products emit 220 factor rows.
`product-smoke.jsonl` records exact product acceptance, area, the exact
incidence-volume witness in this compact smoke, attempts, and timings.
`batch-report.json` contains per-law acceptance and timing totals,
dispositions, and the producing source revision/dirty flag.

These counts are descriptive evidence for this bounded smoke only. They do not
estimate the natural populations, validate conditioning probabilities at
production scale, or establish transfer/persistence of any `sys` relation.

## Factor-only mode

Use `--factor-only` for shape coverage without exact 4D product validation. It
uses the same law formulas, local all-active-facet conditioning, and area
normalization as product mode, but samples one factor directly and writes to a
separate `--factor-out-dir` (defaulting below `artifacts/factor-only`) so the
product-mode artifacts are not overwritten. Population selection is explicit
and unambiguous: repeat `--factor-population 'LAW|PARAMETER'`; side counts are
an explicit comma-separated list. For example, the disposable planned panel
uses 20 accepted shapes per population and side-count stratum:

```text
cargo run -p exp-sys-landscape --release --bin sys-datascience-generator-zoo-smoke -- \
  --factor-only --factor-out-dir /tmp/generator-zoo-factor-only \
  --seed 20260714 --attempts 64 --factor-rows-per-population 20 \
  --factor-side-counts 4,6 \
  --factor-population 'current-baseline|delta=0.2' \
  --factor-population 'repulsive-gap|alpha=1' \
  --factor-population 'repulsive-gap|alpha=4' \
  --factor-population 'zonogon|lengths=uniform(0.5,1.5)'
```

The factor-only output has the same `factor-shape-row-v1` schema, with
non-colliding `generator-zoo-v1/factor-only/...` IDs and `factor_role=single`.
`factor-only-report.json` records the selected populations and side counts,
bounded attempts/exhaustions, generation cost, source revision/dirty state,
and the fact that exact 4D validation was intentionally not requested. Its
`source_dirty` predicate is exactly `git status --porcelain --` restricted to
`generator-zoo-smoke/main.rs` and `experiments/sys-landscape/Cargo.toml`; output
artifacts and timing files are excluded. The report is the authoritative
count/cost artifact for this mode; these rows are shape evidence only and do
not establish product validity, `sys` transfer, or persistence.
