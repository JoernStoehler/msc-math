<!--
Purpose: temporary implementation plan for rewriting the orbit-recovery
experiment around a reusable target-pool + cache-lookup architecture.
Context: keep this separate from the owning experiment implementation while the
rewrite is still speculative. Track stable outcomes here and in the experiment
outputs under `experiments/verification/orbit-recovery/`.
-->

# Orbit Recovery Rewrite Plan

## Goal

Rewrite `experiments/verification/orbit-recovery/` so the experiment validates
minimum-action orbit recovery on an experiment-owned target set while
opportunistically reusing existing cached polytope/capacity/sigma data from
elsewhere in the repo.

The intended experiment surface is:

1. define a target pool of polytopes to validate,
2. search repo caches for matching records,
3. reuse capacity + minimum sigma data when present,
4. recompute beta for the chosen sigma,
5. recover the orbit,
6. verify finite numerical propositions,
7. emit a validation dataset/report for interpretation.

This is a local-first verification/polish task, not a LICCA-bound experiment.

## Why Rewrite

The current binary mixes too many concerns in one file:

- hard-coded dataset generation,
- owned cache path policy,
- cache-hit fast path,
- full `ehz_capacity` fallback,
- orbit recovery,
- proposition checks,
- JSONL reporting.

That makes it awkward to answer the real experiment question:

- which polytopes do we want to validate on?
- which minimum sigma data do we trust and reuse?
- which rows should be generated versus discovered?

It also hard-codes the historical 112-row run and does not match the current
repo state, where the strongest existing cache surface is the byte-identical
170-row shared catalog mirrored across multiple experiments.

## Target Architecture

### 1. Target pool layer

The experiment owns a target pool specification instead of a single built-in
"known + generated randoms" path.

V1 target pool:

- literature/known low-cost validation polytopes,
- a stratified sample from the shared 170-row cache,
- optionally a small curated list of extra rows chosen for multiplicity or
  degeneracy coverage.

V1 exclusions:

- exclude special high-cost known cases such as crosspolytope and other
  dedicated high-facet computations from the default validation pool,
- add those only later as explicit targeted checks if they are needed for a
  concrete validation claim.

The target pool should be described by stable metadata, not only by generated
control flow:

- known polytope names,
- explicit facet-count/sample-count strata for cached random rows,
- optional inclusion tags such as `lagrangian_product`, `random`,
  `degenerate`, `known`.

### 2. Cache discovery layer

The binary loads candidate cache files with `database::load_many(...)`.

V1 source list:

- `experiments/verification/orbit-recovery/polytopes.jsonl`,
- `experiments/combinatorial-cells/polytopes.jsonl`,
- `experiments/sys-landscape/cache.jsonl`.

Lookup policy:

- exact lookup by `DualVerticesKey` when the target already has a concrete
  polytope,
- provenance lookup by `Source` when the target is expressed as a planned random
  row and `Source` is stable enough to identify it,
- reject conflicting rows loudly via `load_many()` conflict handling,
- do not silently choose between inconsistent cached values.

The cache layer is a search surface, not a policy claim that one of these files
is canonical.

Read/write policy for this rewrite:

- treat the three current 170-row mirror candidates as read-only inputs,
- do not write new rows back into those shared mirror files during this rewrite,
- if the experiment needs produced rows beyond the shared mirrors, write them to
  a separate orbit-recovery-owned extension/output file.

### 3. Miss-handling layer

If the target pool asks for a polytope and no cached row supplies it:

- build or generate the polytope locally,
- compute full `ehz_capacity` only when the row lacks reusable minimum-action
  data,
- write any newly produced polytope/capacity/sigma information into the
  experiment-owned extension/output surface chosen by the rewrite.

V1 default:

- prefer reusing the shared 170-row cache,
- produce missing rows only for explicit curated targets,
- do not expand into broad new random generation just because the cache is
  missing a row.

Cache-hit trust policy:

- a reused row must have `capacity`,
- it must have a non-empty `sigmas` list,
- `sigmas[0].perm` must exist,
- `sigmas[0].action` must agree with `capacity` within `1e-10`.

If any of those checks fail, treat the row as unusable for fast-path reuse and
fall back to local minimum-action computation for that polytope.

## Minimum-Orbit Data Model

### V1 scope

Validate one best minimum-action orbit per polytope.

Per target row:

- `polytope`,
- `capacity`,
- `best_permutation`,
- recomputed `beta` from `solve_kkt_for_dual_vertices(dual_vertices, best_permutation)`.

Authoritative reused fields on a cache hit:

- `capacity`,
- `sigmas[0].perm`,
- `sigmas[0].action` as a consistency check against `capacity`,
- fresh `beta` from the current KKT solve.

This matches the current library surface and the current formal validation note.

### Out of scope for V1

- all tied minimum sigmas,
- near-minimum sigmas,
- arbitrary non-minimizing critical points of the dual problem,
- a general "recover every critical point into the primal problem" framework.

Those belong to a broader "all-minimum simple-orbit validation" or numerics
task, not the first rewrite of this experiment.

## Verification Surface

For each recovered minimum-action orbit, record:

- closure error,
- on-facet error,
- inside-`K` error / max violation,
- action error versus the chosen minimum-action value,
- active facet count,
- total segment count,
- solution dimension of the base-point linear system,
- capacity-computation time and recovery time.

Interpretation rule:

- Reeb-velocity / simplicity is treated as "by construction from the chosen
  `(sigma, beta)` surface plus the formal recovery theorem", not as a separate
  empirical proof target unless a later version adds explicit checks.

Default tolerances:

- closure/on-facet/inside: `1e-6`,
- action: `1e-5`.

Tolerance policy:

- treat the binary/note/plot thresholds as authoritative for the rewrite,
- treat the tighter analyzer thresholds as drift to be removed,
- define the tolerances in one authoritative Rust location and keep the
  analyzer and plot aligned with that source.

## Output Artifacts

The rewritten experiment should produce:

- a committed or intentionally uncommitted validation JSONL, depending on size
  and stability,
- analyzer output summarizing failures, worst margins, and dimension counts,
- optional figure only if the figure still serves a real thesis-facing purpose.

The output schema should stay focused on validation evidence, not on duplicating
the full polytope cache schema.

Required family tags in the validation output:

- `known`,
- `random`,
- `lagrangian_product`.

## Suggested Refactor Shape

### Phase 1. Separate selection from execution

Extract helper functions or structs for:

- target-pool construction,
- cache loading/search,
- capacity/sigma resolution,
- per-row validation execution.

Acceptance check:

- the main binary reads as a pipeline over a resolved target list rather than as
  intertwined dataset generation and recovery logic.

### Phase 2. Replace hard-coded 112-row generation plan

Remove the built-in `RANDOM_PLAN` as the defining experiment identity.

Replace it with:

- known rows,
- cached shared-catalog sampling,
- optional explicit curated extras.

Acceptance check:

- the experiment can run on a local-first curated dataset without implying that
  the historical 112-row run is still the default truth.

### Phase 3. Compute real `solution_dim`

Stop hardcoding `solution_dim: 0` in the experiment output.

Likely implementation:

- expose `solution_dim` from `recover_and_verify`, or
- factor base-point recovery to return it directly alongside the orbit.

Acceptance check:

- symmetric/underdetermined known cases no longer report dimension zero by
  default.

### Phase 4. Refresh note + analyzer

After the rewrite produces stable results:

- regenerate the orbit-recovery output,
- update this file and the corresponding experiment outputs,
- remove historical/current ambiguity,
- delete or fold this plan file into the stable note.

## Implementation Decisions

### Experiment-owned versus shared policy

Keep the experiment responsible for:

- target pool definition,
- which cache files it searches,
- when to generate missing rows,
- how the validation output is written.

Do not push "search everywhere in the repo" into the library as a global policy
yet. The library/storage layer should remain path-policy-free.

### Identity and reuse

Preferred identity order:

1. exact `DualVerticesKey`,
2. `Source` when intentionally using seeded/generated families,
3. no fuzzy geometry matching in V1.

### Output ownership

The rewritten experiment may still keep its own cache/output files even if it
reuses rows from shared mirrors. Reuse does not imply shared write ownership.

## Acceptance Checks

The rewrite is complete when:

1. the experiment no longer hardcodes the historical 112-row run as its default
   dataset identity,
2. it can consume a curated target pool while reusing cache rows from the shared
   catalog,
3. cache hits reuse `capacity + minimum sigma` and recompute `beta` fresh,
4. `solution_dim` is real, not hardcoded,
5. the regenerated note/report clearly states what dataset was validated,
6. local checks pass:
   - build the binary,
   - run it on a small curated pool,
   - run the analyzer,
   - inspect emitted failures/worst margins.

Verification matrix requirements:

- at least one exact-key cache hit,
- at least one provenance/`Source` hit,
- at least one forced miss path using local generation or local minimum-action
  computation,
- at least one known symmetric/underdetermined case where `solution_dim > 0` is
  expected,
- analyzer output matches the emitted dataset under the authoritative
  tolerances,
- if the figure remains in scope, note/figure/data summaries agree.

## Stop Conditions

Stop and re-evaluate before continuing if:

- the task grows into "all minimum orbits" instead of one best minimum orbit,
- the rewrite needs a new stable library API for rich orbit reports,
- cached files disagree on trusted fields such as `capacity` or
  `sigmas.first().perm`,
- a reused row fails the `capacity` vs `sigmas[0].action` consistency check,
- the target dataset choice becomes a thesis-facing research decision rather than
  an implementation decision.

## Cleanup Rule

Delete this file, or fold its stable conclusions into this plan and the owning
experiment outputs, once the rewrite lands and the new experiment structure is
no longer speculative.
