# Numerics Reasoning

`experiments/numerics` is the exploratory/validation surface for the KKT capacity
numerics used elsewhere in the thesis. The durable math claims should be moved
through formal files and crates only after they are stabilized; this directory keeps
the "current evidence" layer.

Subdirectory map and what the artifacts imply:

`algebraic-exactness` now owns the spike work for exact algebraic geometry
and selected exact KKT checks on HKO-style inputs. The code is currently
experiment-owned in `src/algebraic/*` with canonical artifacts:
`exact-polytopes.jsonl` and `exact-kkt-comparison.jsonl`. `smoke-*` files are the
default local run products and should be treated as non-canonical.

`error-bounds` is the main abstract-numerics proof/validation packet. The current
3-stage structure (collect → run → analyze) and `tests.rs` indicate the packet is
no longer a logbook: it is intended as a reproducible validation harness for
f64-vs-exact behavior and bound behavior in the KKT flow. The key
proven/computed item carried forward is the empirical support for
`|Q−Q*| ≤ ||H||·||β̃||·||r||/σ_min(C)` on relevant datasets plus the documented
structure-based failure mode on rank-deficient cases.

`q-error` and `kkt-inertia` are now confirmation packets for known-polytopes coverage.
`q-error` shows bound correctness and exact comparison on winning nodes for all
non-singular F≤10 winners, while `kkt-inertia` confirms the inertia decomposition
for tested known polytopes and classifies the few mismatches as eigenvalue-threshold artifacts.

`unknown-predicates` now serves as a direct check that UNKNOWN admissibility cases
do not currently alter selected outputs on current datasets, so UNKNOWN is treated
as a numeric-noise concern in this experiment scope, not a fundamental correctness
gap.

`sage-feasibility` is explicitly exploratory and still the split test between
Rust orchestration and Sage as an independent baseline. Its current value is in
quantifying end-to-end feasibility and timing on controlled benchmark families
without adding extra API complexity to the main Rust crates.

For future work, the migration rules have effectively narrowed to:
- avoid expanding this tree into a chronological log;
- keep each packet focused on a narrow proposition with explicit artifact contracts;
- leave durable API ownership for `crates/` unless a packet reaches stability.
