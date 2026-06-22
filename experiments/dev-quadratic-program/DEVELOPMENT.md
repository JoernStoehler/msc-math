# Dev Quadratic Program Development

This packet owns active QP capacity-route development while route semantics,
instrumentation, numerics, performance, and verification are still coupled.
Keep route-changing code local here before promoting stable contracts into
`crates/symplectic`.

## Code Map

- `src/`: library-like QP development code and local route variants.
- `src/f64_route/`: current local f64-only capacity route: sigma enumeration,
  f64 one-sigma orbit payload, q/action interval conversion, admissibility
  mapping, candidate aggregation, and report shape.
- `src/fallback_route/`: bridge to the existing `crates/symplectic` exact
  fallback and certified aggregation route over retained f64 candidates. This
  is not yet an instrumented local copy and does not by itself answer whether
  f64 candidate solving discarded an exact capacity candidate.
- `src/exact_route/`: local exact-all-visited-sigma reference route. It
  enumerates the transition-pruned sigma stream and exact-solves each visited
  sigma, so it does not depend on f64 candidate retention.
- `src/validation.rs` and `src/geometry.rs`: f64 input validation and
  combinatorics diagnostics used by the f64 route.
- `src/product/`: product detection, rounding, and near-redundant product
  preprocessing.
- `tools/scan/`: command-line scanner, input-source selection, and JSONL
  output.
- `tools/analyze/`: scan-row summary command.
- `tools/kkt_error_audit/`: retained-candidate exact/f64 audit for predicate
  and Q-bound search.
- `tools/candidate_filter_audit/`: exact-all-visited-sigma audit that checks
  whether the f64 single-sigma solve discards exact-admissible positive-`Q`
  sigmas before fallback/certification can see them.
- `verification/`: route-specific expectation manifest and comparison tools.
- `numerics/`: route-specific numerics event conversion and near-singular
  diagnostics.
- `performance/`: route-specific timing binaries, summarizers, and manifest
  performance wrapper.
- `numerics-audit/`: separate generic QP/KKT numerical-error audit crate.
- `docs/f64-route.md`: detailed f64 route experiment notes and commands.

## Route Status

Use `docs/route-consumer-matrix.md` as the current route-design anchor. Route
design is no longer the blocker for this packet; implementation and evidence
must now be organized by route contract.

The local f64 route currently owns f64 route semantics and instrumentation
points. It still calls the lower-level f64 KKT eigensolver in
`crates/symplectic::kkt::saddle_point_solver`. If error-bound work needs to
alter the KKT solve itself, copy/edit the necessary KKT kernel locally before
changing semantics.

Exact fallback is already developed in `crates/symplectic`, not future work:

- `aggregate_orbits_with_dual_vertices_exact`;
- `aggregate_certified_orbits_with_dual_vertices_exact`;
- `OrbitGuaranteeMode`;
- `CertifiedOrbitSetMode`;
- `exact::solve_orbit_sigma_exact`.

What is still missing here is an instrumented local fallback-route copy that
fits this packet's route-comparison needs. Do not describe fallback as
undeveloped; describe it as bridged from `crates/symplectic` until local
instrumentation or semantic changes are needed.

The remaining route work is split as follows:

| Route | Current implementation state | Evidence state | Next implementation/evidence step |
| --- | --- | --- | --- |
| heuristic f64 capacity | local route in `src/f64_route/` | scan/analyze/performance packets exist | keep labels explicitly heuristic; do not promote as certified |
| retained-candidate f64 predicate/fallback | fallback bridge in `src/fallback_route/`; retained-candidate audit in `tools/kkt_error_audit/` | verified-inverse predicate has useful survivor-level evidence but not a production proof | compare exact-resolving more retained capacity-window candidates against proving stronger f64 predicates |
| candidate-filter safety | no production route; diagnostic audit in `tools/candidate_filter_audit/` | complete small/edge cases measured; HKO only first-500 diagnostic | build targeted/exhaustive/parallel audits before claiming retained-candidate exact certification is complete |
| exact-all-visited-sigma rational capacity | local transition-pruned reference route in `src/exact_route/` | small audit measurements exist; cost is case-dependent | use as comparison/reference on small or targeted cases before promoting any retained-candidate route as globally certified |
| algebraic/Sage route | not local Rust code | Sage/theorem packets own this when needed | keep separate unless a Rust/thesis surface confuses binary64 rational exactness with algebraic-object exactness |

Keep crate imports for outside-domain utilities such as retained input loading,
random generation, generic geometry helpers, and stable data types.

Do not promote local route changes back into `crates/symplectic` until the
contract is stable enough that multiple consumers should call it as a library.
