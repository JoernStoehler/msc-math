# Verification Experiment Package

Orientation doc for `dev-capacity-validation` (`experiments/verification`).

This package owns slow, artifact-backed correctness checks for the capacity pipeline.

Subdirectories and current role:
- `correctness/` — core axioms/property checks (conformality, invariance, monotonicity, continuity, literature values).
- `algorithm-comparison/` — runtime/timing and variant-agreement studies (`benchmark/`, `ablation/`, `profiling/`).
- `all-minimum/` — trusted minimum-set computation for selected polytopes.
- `orbit-recovery/` — geometric recovery/closure checks on trusted minimum rows.
- `src/` — shared helpers for verification binaries.

Keep this package as the boundary between local experiment evidence and production-facing claims.
