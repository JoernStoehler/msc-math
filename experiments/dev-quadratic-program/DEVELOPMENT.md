# Dev Quadratic Program Development

This is the physical home for active QP capacity-route development while route
semantics, instrumentation, numerics, performance, and verification are still
coupled. Keep route-changing code local here before promoting stable contracts
into `crates/symplectic`.

## Code Map

- `src/`: library-like QP development code and local route variants.
- `src/f64_route/`: current local f64-only capacity route: sigma enumeration,
  f64 one-sigma orbit payload, q/action interval conversion, admissibility
  mapping, candidate aggregation, and report shape.
- `src/fallback_route/`: local exact fallback and certified aggregation route
  over retained f64 candidates. It intentionally certifies only the retained
  candidate set; candidate-filter safety remains a separate audit question.
- `src/exact_route/`: local exact-all-visited-sigma reference route. It
  enumerates the transition-pruned sigma stream and exact-solves each visited
  sigma, so it does not depend on f64 candidate retention. Under a
  source-backed complete HK candidate-family contract, its least action can
  certify the scalar capacity without a per-word second-order test; its
  one-sigma outputs remain KKT witnesses, not automatically physical orbits.
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
- `examples/owned_consumers.rs`: caller-shaped examples kept in this dev
  packet. They are not downstream evidence; they are cheap API and architecture
  probes for route-result ergonomics, copy-editability, instrumentation
  friction, and status/result clarity while route contracts are still changing.
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

Use `docs/capacity-architecture.md` for the outer-to-inner interface map:
systolic ratio, scalar capacity, minimizers, action windows, context-free
one-word analysis, internal algorithms, and retained evidence.

The selected scalar routes live in `src/selected_route/`; production copies
live in `crates/symplectic::algorithms::capacity_4d`. Change experiment copies
freely, but update production semantics deliberately and rerun
`tests/selected_route_correspondence.rs` plus the relevant producer. Older
f64, fallback, and exact routes below remain comparison controls rather than
the selected API.

Exact one-sigma KKT solving and older fallback aggregation exist in
`crates/symplectic`:

- `aggregate_orbits_with_dual_vertices_exact`;
- `aggregate_certified_orbits_with_dual_vertices_exact`;
- `OrbitGuaranteeMode`;
- `CertifiedOrbitSetMode`;
- `exact::solve_orbit_sigma_exact`.

`src/fallback_route/` is the physical home of a local fallback aggregation
copy used for route-comparison work. Keep route-semantic changes local here
until they are stable enough to promote back to `crates/symplectic`.

The remaining route work is split as follows:

| Route | Current implementation state | Evidence state | Next implementation/evidence step |
| --- | --- | --- | --- |
| selected general scalar capacity | instrumentable copy in `src/selected_route/general.rs`; production in `symplectic::algorithms::capacity_4d` | exact predicate/fallback, outward-bound, ablation, and correspondence producer in `tools/general_algorithm_ablation/` | migrate scalar consumers after concurrent code movement is reconciled |
| selected structural-product scalar capacity | KKT-free copy in `src/selected_route/product.rs`; production in `symplectic::algorithms::capacity_4d` | complete exact intermediate audit, old-route comparison, retained product check, and correspondence producer in `tools/product_closure_route/` | migrate scalar product consumers; retain the older route for branch-sensitive callers |
| heuristic f64 capacity | local route in `src/f64_route/` | scan/analyze/performance packets exist | keep labels explicitly heuristic; do not promote as certified |
| retained-candidate f64 predicate/fallback | local fallback aggregation in `src/fallback_route/`; retained-candidate audit in `tools/kkt_error_audit/` | verified-inverse predicate has useful survivor-level evidence but not a production proof | compare exact-resolving more retained capacity-window candidates against proving stronger f64 predicates |
| candidate-filter safety | no production route; diagnostic audit in `tools/candidate_filter_audit/` | complete small/edge cases measured; HKO only first-500 diagnostic | build targeted/exhaustive/parallel audits before claiming retained-candidate exact certification is complete |
| exact-all-visited-sigma rational capacity | local transition-pruned exact route in `src/exact_route/`; scalar capacity depends on complete-stream and input-provenance assumptions | small audit measurements exist; cost is case-dependent | preserve the complete-stream contract; use exact witness/action language for generic caller-supplied streams and do not treat returned witness lists as complete physical-orbit sets |
| algebraic/Sage route | not local Rust code | Sage/theorem packets are the evidence home when needed | keep separate unless a Rust/thesis surface confuses binary64 rational exactness with algebraic-object exactness |

Keep crate imports for outside-domain utilities such as retained input loading,
random generation, generic geometry helpers, and stable data types.

Do not silently alter the selected production contract while changing a local
comparison route. The correspondence suite checks the promoted scalar routes;
it does not imply that older branch-sensitive outputs are equivalent.
