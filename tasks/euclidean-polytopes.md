<!--
Purpose: migration roadmap for a durable Euclidean convex-polytope crate.
Context: ordinary convex geometry currently lives inside symplectic-facing
modules, which makes non-symplectic helpers harder to reuse and review.
-->

# Euclidean Polytopes Roadmap

## Status

- State: active.
- Last updated: 2026-05-10.
- Source surfaces: `crates/euclidean-polytopes/`,
  `crates/symplectic/src/geom/`, `crates/MAP.md`, `tasks/rust-tech-debt.md`.
- Refresh when: a geometry helper moves between crates, a public API decision
  is accepted/rejected, or a retained experiment starts using the new crate.

## Steering Cache

- [Jorn request 2026-05-10] Create a nice crate for Euclidean, not symplectic,
  polytopes. Start with the crate scaffold, `README.md`, `DEVELOPMENT.md`, and
  a migration task with done criteria. Prefer flat function style over aliases:
  callers should pass ordinary `Vec<Vector4<T>>` / `&[Vector4<T>]` values and
  own context-specific contracts such as `0 in int conv(a)`.
  Why it matters: this keeps the API close to the math and avoids another
  wrapper-heavy crate while refactoring existing geometry users.
- [agent synthesis 2026-05-10] The reusable Euclidean operations are:
  origin-interior test for point sets, V-representation non-redundancy, polar
  vertex enumeration, full-dimensional volume from dual/primal vertices, and
  affine-subspace polygon/volume helpers in ambient `R^4`.
  Why it matters: these are the operations that repeatedly appear underneath
  symplectic algorithms, exact validation, and experiment geometry setup.
- [Jorn correction 2026-05-10] The crate needs both `f64 -> f64 or
  indeterminate` APIs and `exact -> exact` APIs. Exact implementations may use
  the `f64` pathway as a cheap filter, but every indeterminate case must be
  decided by a slow exact calculation before returning.
  Why it matters: vertex enumeration and incidence tests have cheap common
  floating paths, but near-singular 4-tuples, duplicate candidates, and
  halfspace-boundary cases must not be guessed by tolerance.

## Work Map

| item | state | value class | owner/gate | next action | source |
| --- | --- | --- | --- | --- | --- |
| Crate scaffold and API target | `[active]` | mainline thesis | agents, Jorn for API taste close calls | Review `README.md` and `DEVELOPMENT.md`; decide whether the flat-input/no-public-wrapper direction is accepted for the first migration. | `crates/euclidean-polytopes/README.md`, `crates/euclidean-polytopes/DEVELOPMENT.md` |
| Robust floating/exact architecture | `[active]` | mainline thesis | agents | Define the shared result vocabulary for approximate routines that return determined `f64` results or indeterminate, and exact routines that resolve all indeterminate cases. | `crates/euclidean-polytopes/README.md`, `crates/euclidean-polytopes/DEVELOPMENT.md` |
| Exact point-set predicates | `[active]` | mainline thesis | agents | Implement exact `origin_in_interior_of_conv` and `all_points_are_extreme` over `Vector4<T>` with `T: ExactScalar`; add `f64` variants only when they can expose indeterminate instead of guessing. | `crates/symplectic/src/geom/vertex_enumeration/`, `crates/algebraic-numbers/` |
| Polar vertex enumeration | `[active]` | mainline thesis | agents | Move/adapt 4-subset enumeration so `polar_vertices_exact(vertices)` computes the polar vertices under the `0 in int conv(vertices)` contract, using `f64` filtering plus exact fallback for indeterminate tuples; cross-check with existing `Polytope4D` fixtures. | `crates/symplectic/src/geom/vertex_enumeration/enumerate.rs` |
| Full-dimensional volume | `[active]` | mainline thesis | agents | Factor ordinary `R^4` volume away from `Polytope4D`; expose `f64` volume as determined-or-indeterminate when incidence is tolerance-sensitive, and exact volume as exact incidence plus exact determinant summation. | `crates/symplectic/src/geom/volume.rs` |
| Affine-subspace polygons and volume | `[map-input]` | contingent during writing | Jorn if generic-vs-specific API affects thesis callers | Identify first concrete caller for polygons in affine 2-planes of `R^4`; choose either a targeted polygon area helper or a broader affine-volume function only after that caller is known. | `crates/symplectic/src/geom/polygon.rs`, future experiment callers |
| Symplectic integration cleanup | `[active]` | mainline thesis | agents | After each migrated slice, keep `symplectic` as the owner of symplectic form, capacity, KKT, omega signs, Reeb-direction adjacency, and experiment-facing wrappers only. | `crates/symplectic/src/geom/`, `crates/symplectic/src/algorithms/` |

## Done Criteria

The migration task is done when:

- `crates/euclidean-polytopes/` has consumer and maintainer docs matching the
  implemented public API;
- workspace commands pass:
  `cargo test -p euclidean-polytopes`,
  `cargo clippy -p euclidean-polytopes --all-targets -- -D warnings`, and
  `cargo check --workspace`;
- `symplectic` no longer owns reusable ordinary convex-geometry algorithms
  except thin wrappers needed for compatibility;
- existing symplectic behavior is preserved by tests or explicit migration
  notes;
- `f64` combinatorial APIs expose indeterminate outcomes, and exact APIs resolve
  those outcomes before returning exact data;
- API review has confirmed that no alias/wrapper/trait was added without a
  current caller or a clear simplification;
- `crates/MAP.md`, this task, and affected crate docs are updated.

## Agent Cache

- [fresh 2026-05-10] `Polytope4D` currently mixes ordinary geometry with
  symplectic data: dual vertices, primal vertices, incidence, vertex adjacency,
  omega signs, and f64 copies. The Euclidean crate should take the ordinary
  pieces first; omega signs and capacity-facing adjacency remain symplectic.
- [fresh 2026-05-10] Existing full-dimensional volume triangulates facets from
  their centroid and cones to the origin. That relies on normalized
  H-representation `a_i . x <= 1`, hence `0` is inside every valid full
  polytope.
- [fresh 2026-05-10] The lower-dimensional volume requirement is underspecified
  enough that a generic `affine_volume` API would be premature. Start with the
  first concrete polygon/face caller.
- [fresh 2026-05-10] Existing `symplectic` vertex enumeration already uses an
  `f64` prefilter before exact rational checks. The new architecture should make
  that pattern explicit: approximate callers see indeterminate, exact callers
  get a final exact answer after fallback.

## Pruned / Stale

- None yet.
