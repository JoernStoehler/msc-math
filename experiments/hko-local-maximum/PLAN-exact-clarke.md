# Exact Clarke Plan (HKO Local Maximality)

Scope:
- prove the `M_10` first-order cone statement with exact arithmetic and machine-readable certificates only.

Current checkpoint:
- `exact-clarke/` is the active route for this target.
- `hko-geometry.json` and `hko-symmetry-tangent.json` are complete.
- `build_widened_seed_witness.py` and `verify_widened_seed_witness.sage` are in place; current checks stop short of full rank equality against the final active set.

Near-term tasks:
1. Keep witness contract stable
   - `exact-clarke/widened-seed-witness.json`
   - `exact-clarke/build_widened_seed_witness.py`
   - `exact-clarke/verify_widened_seed_witness.sage`
   - Ensure every future witness update preserves:
     - `field`
     - exact active geometry in `R^40`
     - exact representative-row ranks and kernel dimensions
     - explicit symmetry inclusion checks

2. Finish representative coverage on the seven-facet side
   - files to inspect and refresh: `numerical-permutation-orbits.json`, `endpoint-seed-rows.json`, `midpoint-seed-rows.json`
   - objective: extend exact seed coverage so the active candidate family spans rank `25` in the final union matrix.

3. Assemble the final matrix certificate
   - expected outputs in `numerical-minima-summary.json`, `numerical-family-reconciliation.json`, `reduced-sys-prototypes.json`
   - prove:
     - exact active-gradient matrix `G` in dual-vertex coordinates
     - exact `rank(G)` and right-kernel basis
     - exact symmetry tangent matrix rank and the cone/cokernel equality chain

4. Integrate once `G` is final
   - connect to `research/hko-local-maximum/design/exact-clarke-closure-plan.md`
   - keep formal handoff path explicit in `formal/` and `RESULTS.md` updates.

Delivery checks before closing:
- verifier on `exact-clarke/widened-seed-witness.json` exits with the strengthened active-set coverage.
- final `G` artifact is reproducible from script outputs in `exact-clarke/` and is cross-checkable without reusing floating-point active-set discovery.
- unresolved items move to the route decision in `research/hko-local-maximum/design/exact-clarke-subgradient.md` only if a trusted route cannot be completed.
