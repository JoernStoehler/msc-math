# Numerics Experiment Package

Orientation doc for `dev-numerical-analysis` (`experiments/numerics`).

This package contains locally maintained numerical-confidence experiments for the KKT/QP stack used by capacity computations.

Subdirectories and current role:
- `error-bounds/` — end-to-end QP error-bounds pipeline (abstract problem collection, solver checks, analysis, tests).
- `q-error/` — formal-error verification across `(S, sigma)` nodes for known polytopes.
- `kkt-inertia/` — inertia formula validation for the KKT matrix.
- `unknown-predicates/` — audits of UNKNOWN admissibility decisions and their impact.
- `algebraic-exactness/` — exact HKO-style algebraic geometry + selected KKT reference runs.
- `sage-feasibility/` — SageMath end-to-end feasibility and timing experiment.
- `gradient/` — gradient-validation split packets (`numerics`, `numerics-edge-cases`, `numerics-subdifferential`).
- `testdata/` — committed JSONL fixtures for bounded regression.

Use this folder for short local planning, role tracking, and artifact refresh rules; avoid writing long-running experimental logs here.
