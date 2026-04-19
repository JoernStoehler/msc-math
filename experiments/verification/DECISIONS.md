# Verification Decisions

1. Keep the cross-implementation split explicit.
   - `all-minimum` owns minimum-set generation from HK2017 sigma candidates and writes trusted rows.
   - `orbit-recovery` owns geometric recovery validation only.
   - `correctness` remains the package-level property gate.
   - `algorithm-comparison` remains separate and owns performance/variant comparison evidence.

2. Treat local-first diversity as a design constraint, not an exhaustive proof surface.
   - Shared-cache diversity and representative sampling were chosen for coverage value and reproducibility, not global completeness over all local artifacts.
   - Smoke/full modes and canonical outputs encode this split.

3. Preserve the trust boundary on minima.
   - `all-minimum` does not treat cached minimum sigmas as truth; it computes minima from shared-cache sources and validates by action.
   - `orbit-recovery` ignores non-essential payload fields from trusted rows and assumes the schema/version alignment with `all-minimum`.

4. Keep exact arithmetic and float-tolerance assumptions visible where they matter.
   - `OrbitGuaranteeMode::MinimaSafe`, `solve_orbit_sigma`, and `ehz_capacity` agreement checks are the minimum reproducible cross-check seam.
   - Tolerances are fixed in implementations and should be interpreted as empirical runtime-stability tolerances, not absolute proof margins.

5. Do not recreate old `research/**` directories in experiments.
   - Mapping is maintained through notes only, with local notes taking current precedence.
