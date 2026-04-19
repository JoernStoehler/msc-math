# Numerics Decisions

Non-obvious choices retained for continuity:

- The experiment-first boundary is preserved for geometry and exact-KKT prototyping.
`crates/` behavior is not being retrofitted first (`Polytope4D`, cached rational
JSONL, and database layout stay unchanged in this scope).
- Algebraic arithmetic is designed as an ordered-field boundary, not a symbolic CAS.
The chosen first-mile pattern is static named fields with compile-time specs and
`Algebraic<S>` values.
- `BigRational` is used directly (or via an alias) in v1; wrappers were avoided to
keep trait and serialization plumbing explicit and predictable.
- Rejected runtime-generic number fields in v1: no dynamic runtime field API until a second
named field proves reuse pressure.
- Sign and ordering are semantic, not epsilon-based. `Sign`/`cmp_real` logic remains
the branch criterion for admissibility-critical branches; `to_f64()` is reporting only.
- Serialization policy is canonical coefficients only, so persisted artifacts are
deterministic across runs.
- Error-propagation architecture in KKT remains trinary (`TRUE` / `FALSE` /
`INDETERMINATE`) with lazy exact fallback. This directly constrains what is considered
cheap and what is deferred.
- For `num-projection` and interior `β>0` behavior, `E = ||H||·||β̃||·||r||/σ_min(C)`
is now the practical bound and comparison baseline, with exact arithmetic used as a
diagnostic/ground-truth channel rather than default path.
- In UNKNOWN admissibility experiments, phase-2 high-precision reruns are deferred
until UNKNOWN begins to affect winning nodes on tested scope.

Decisions that constrain future edits:

- Keep `algebraic-exactness` experiment-owned until `algebraic-numbers` has a stable
minimal surface and formalized tests; only then migrate the reusable crate.
- Keep `error-bounds`/`q-error`/`kkt-inertia` anchored to known-polytopes evidence while
adding formal proof gaps, then widen data coverage only with explicit commands.
- Keep `sage-feasibility` explicitly temporary and method-focused; do not let it
dictate production architecture unless timing + completion evidence justifies it.
- `README.md`, `RESEARCH.md`, and `PLAN-error-bounds.md` are superseded by this note
layer and removed to avoid duplicate planning text.
