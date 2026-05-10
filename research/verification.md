# Verification Research Note

## Scope

This note centralizes the experiment-only verification package boundary for the thesis pipeline.
`experiments/verification` remains the validation layer that separates fast crate checks from slower, artifact-backed evidence.

## Current State

The verification package is organized into four roles:

- `correctness/`: validates core capacity implementation properties and current evidence for
  conformality, symplectic invariance, monotonicity, continuity, literature agreement, and
  unpruned/pruned/billiard agreement.
- `all-minimum/`: computes trusted minimum-orbit rows from a shared local-first polytope pool and
  cross-checks minimum-action values against `ehz_capacity`.
- `orbit-recovery/`: validates those trusted rows with KKT rebuild and `recover_and_verify`, checking
  closure, facet adherence, inside-`K` compliance, and action error.
- `algorithm-comparison/`: provides performance and variant-consistency evidence for implementation choices.

Canonical evidence packets currently tracked in checked-in sources are:

- `experiments/verification/correctness/main.rs` plus `correctness.jsonl` for the six proposition checks.
- `experiments/verification/all-minimum/main.rs` plus `all-minimum.jsonl` and
  `all-minimum-orbits.jsonl` for canonical minimum rows.
- `experiments/verification/orbit-recovery/main.rs` plus `orbit-recovery*.jsonl` for reconstruction checks.

Canonical all-minimum and orbit-recovery runs reported:

- 28 selected polytopes in the local-first pool.
- 469 trusted minima.
- Full reconstruction success for all 469 minima in `orbit-recovery`.
- Tight checks on strict threshold sets using `1e-6` (closure/facet/inside) and `1e-5` (action), with observed error scales `e-11` and `e-14`.

## Evidence And Interpretation

- `all-minimum` is the generator of trusted minima, not a full geometric ground-truth verifier.
- `orbit-recovery` is the geometric validator for those trusted minima.
- The trust boundary is explicit: algorithm changes to shared solver code should refresh both
  `all-minimum` and `orbit-recovery` outputs before cached claims are reused.
- After data refreshes, `correctness` remains the package-level property gate for
  global claims.

## Decisions

1. Keep the cross-implementation split explicit:
   - `all-minimum` owns minimum-set generation and writes trusted rows.
   - `orbit-recovery` owns geometric recovery validation.
   - `correctness` remains the package-level property gate.
   - `algorithm-comparison` stays separate for performance and variant comparison.
2. Treat local-first diversity as a coverage/reproducibility choice, not an exhaustive proof surface.
3. Preserve trust boundaries across minima:
   - `all-minimum` computes minima from shared-cache sources and validates by action.
   - `orbit-recovery` ignores non-essential trusted-row payload fields and depends on schema/version alignment.
4. Keep exact arithmetic and tolerance assumptions explicit:
   - `OrbitGuaranteeMode::MinimaSafe`,
     `solve_orbit_sigma_with_dual_vertices`, and `ehz_capacity` agreement
     checks are the minimum reproducible cross-implementation check surface.
   - Runtime tolerances are empirical stability tolerances, not absolute proof margins.
5. Do not recreate legacy `research/**` directories inside `experiments/`; tracking should be through notes only, with local notes currently taking precedence.

## History

- The previous structure used separate topic notes for reasoning, decisions, and next
  steps, now merged into this note.
- The package was split into three packets (`all-minimum`, `orbit-recovery`, and `correctness`)
  to replace a single monolithic minimum/verification flow.
- The local-first selection currently tracks 28 polytopes and 469 minima in the canonical packet,
  while keeping tolerance-based behavior and local-first constraints explicit.

## Next Steps

- Active objective: keep `correctness`, `all-minimum`, and `orbit-recovery` aligned after shared
  solver changes.
- Standard sequence:
  - `cargo run -p dev-capacity-validation --release --bin axioms-correctness`
  - `cargo run -p dev-capacity-validation --release --bin axioms-all-minimum --full`
  - `cargo run -p dev-capacity-validation --release --bin axioms-orbit-recovery --full`
  - `cd experiments/verification/<packet> && uv run analyze.py` for `all-minimum` and
    `orbit-recovery`; use `uv run plot_orbit_recovery.py` for `orbit-recovery` visuals.
- Boundary-sensitive checks first:
  - schema changes between `all-minimum-orbits.jsonl` and `orbit-recovery`,
  - shared polytope selection changes that affect reproducibility,
  - tolerance regressions in closure/on-facet/inside/action checks.
- Stop condition:
  - all three packets complete in full mode (or explicit smoke-only justification),
  - coherent and parseable `all-minimum` and `orbit-recovery` datasets,
  - `correctness` passes all six propositions and literature comparisons in
    `correctness/correctness.jsonl`.
- If failures appear:
  - rerun the smallest failing packet in smoke mode and compare against tracked smoke outputs,
  - confirm command/path stability in `experiments/verification/{all-minimum,orbit-recovery,correctness}`,
  - then adjust source only if required.
