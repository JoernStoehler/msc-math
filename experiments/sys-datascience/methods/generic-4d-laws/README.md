# Non-product four-dimensional generator wishlist

This is a target-free breadth-first packet for the generator-transfer line. It
asks whether inexpensive, mathematically legible 4D laws broaden the
combinatorial/geometric support beyond IID normalized halfspaces and planar
products. It stops at exact Euclidean reconstruction: no capacity, `sys`,
selection rank, or target table is imported.

## Reproduction

The producer is a self-contained Rust workspace. From the repository root:

```bash
cargo test --release --manifest-path experiments/sys-datascience/methods/generic-4d-laws/Cargo.toml
cargo run --release --manifest-path experiments/sys-datascience/methods/generic-4d-laws/Cargo.toml -- smoke experiments/sys-datascience/methods/generic-4d-laws/artifacts/smoke
cargo run --release --manifest-path experiments/sys-datascience/methods/generic-4d-laws/Cargo.toml -- validate experiments/sys-datascience/methods/generic-4d-laws/artifacts/smoke
```

The smoke uses deterministic seeds `11, 29, 47` and the predeclared grid in
`manifest.json`. Generated rows are deterministic for a fixed source commit;
the summary's wall time is intentionally non-deterministic and is retained
only for the smoke-cost disposition. The producer rounds f64 coordinates to a
fixed rational lattice of denominator `10^6` before exact validation. Thus the
exact contract is about the explicitly represented rational polytope, not an
unrecorded tolerance decision.

## Construction and checks

- **Central H-law:** sample `r` Haar directions on `S^3`, set
  `a_i=2u_i/w_i` and `-a_i`, with equal widths (`sigma=0`) and modest centered
  log-width (`sigma=0.15`). Exact polar reconstruction is accepted only when
  all requested `2r` columns are active.
- **Central V-law:** sample `n` sphere or 4-ball points and propose all `±x_i`.
  The row records the proposal count and the *actual* hull facet count; no
  facet stratum is silently forced. Exact active-point filtering and
  extremality checks give the retained vertex count. Singular 4-tuples and the
  smallest positive determinant are recorded as conditioning diagnostics.
- **Zonotope:** construct all `2^m` Minkowski sums
  `sum_j epsilon_j ell_j v_j`. The retained hull is exact, central symmetry is
  checked, and the proposal count is preserved. `m=4` equal lengths is the
  cheap accepted grid.
- **IID 4-ball hull:** propose `n=12` non-symmetric points, retrying at most
  eight times until the origin/full-dimensional exact gate passes. The row
  keeps proposal and retained vertex counts separately.
- **Affine controls:** apply a diagonal lognormal map with determinant one to
  the simplex, cube, and cross-polytope. These are fixed-orbit combinatorial
  controls, not independent combinatorial breadth. Unit tests compare the
  complete incidence fingerprint before/after the map and rows expose the
  coordinate-dependent diagonal.

Every accepted row includes the f-vector, positive exact volume and
unit-4-ball-normalized volume, central-symmetry witness, product cross-block
fraction of dual normals, mean absolute `omega_0` pair value, determinant
conditioning, exact candidate-4-set count, and an upper bound on exact
feasibility checks. `rows.jsonl` contains one record per requested attempt,
including rejected rows when a future parameter fails. `manifest.json` pins the
source commit, seeds, grid, target-field prohibition, and omitted variable-
facet distance contract.

## Breadth-first disposition

All 27 retained smoke rows passed exact bounded/full-dimensional/irredundant
reconstruction. The central H and V laws are genuinely non-product under the
dual-normal witness (`cross_block_fraction=1` in this small smoke); the IID
ball hull is non-symmetric and usually retains 11 of 12 proposals. The H law
has exactly the requested facet count, while V rows expose much larger actual
facet counts (26--46 for the declared grid). The m=4 zonotope is a
centrally-symmetric parallelepiped stratum (`f=(16, ..., 32--52)` here), so it
adds an affine-orientation control but not independent combinatorial breadth.
The affine controls preserve their base fingerprints exactly (simplex
`(5,10,10,5)`, cube `(16,32,24,8)`, cross-polytope `(8,24,32,16)`).

The `m=5` zonotope (modest lognormal lengths) and `n=20` IID-hull rows were
attempted in the initial smoke grid and abandoned after measured exact-polar
cost exceeded 90 seconds before the first artifact; no looser numeric law was
substituted. They remain explicit deferrals, not missing rows. The accepted
27-row smoke took about 45 seconds in the development container (compile
excluded from the retained timing note).

## Interpretation boundary

These are feasibility and support observations only. They do not rank laws,
make a population-support claim, establish a capacity result, evaluate `sys`,
or authorize target exposure. The current artifact can support a later choice
of one or two laws for an independently frozen target-free feature comparison;
that choice requires a fresh portfolio decision and review. An approximate
variable-facet body distance is omitted because no cheap, stable copy-local
contract exists in this packet.
