# Brief for the Planning Agent

**Your job:** Design the target state of `crates/src/` (the Rust library). Produce a specification that an implementation agent follows. Jörn approves before implementation begins.

**Do not write code.** Explore the codebase (use subagents for summarization — you don't need to read function bodies, just what each file provides). Then design.

## What the crate does

Computes the EHZ symplectic capacity c_EHZ(K) for convex polytopes K in R^4. This involves:
1. Representing 4D polytopes (halfspace/vertex data, combinatorial skeleton, exact rational + fast f64)
2. Enumerating candidate Reeb orbits (subset S of facets + cyclic permutation σ)
3. Solving a constrained QP per candidate: max (1/2)βᵀHβ s.t. Cβ=d, β>0
4. Returning the minimum-action orbit (= maximum Q)

Downstream consumers: ~18 experiment binaries, thesis LaTeX.

## What exists

**Two KKT solver variants** (intentionally different algorithms, BOTH kept):
- `kkt/augmented.rs` — (m+5)×(m+5) saddle-point eigendecomposition. Returns KktResult with error bounds. Currently wired into the capacity pipeline.
- `kkt/projection_solver.rs` + `constraint_solver.rs` + `margin_search.rs` — projects to constraint null space, LP-based margin search. Returns Solution with trinary Verdict. New code, NOT yet wired into the capacity pipeline.

**Capacity pipeline** (calls augmented solver only — `kkt::solve()` is NOT yet wired in):
- `algorithms/hk2017/` — general EHZ capacity via exhaustive (S,σ) enumeration. Calls `augmented::solve_kkt(normals, heights, perm)` directly.
- `algorithms/billiard/` — Lagrangian product capacity via structured enumeration. Same calling pattern.
- `kkt_rational.rs` — exact rational solver for resolving ambiguous cases

**Parameterization status:** `Polytope4D` stores dual vertices aᵢ natively. But `augmented::solve_kkt` still takes `(normals, heights)`, and hk2017/billiard extract these via `.normals_f64()` / `.heights_f64()`. The dual vertex migration is done at the data model level, not at the solver interface level.

**Geometry** (`geom/`): polytope representation, skeleton, symplectic form, volume, known polytopes, reeb trajectories, vertex enumeration, 2D polygons.

**Regression data**: `crates/tests/fixtures/capacity_dataset.json` — 33 polytopes (literature + random F=5..8) with committed capacity values.

## What's wrong (problems the new design should solve)

1. **Two solver variants with incompatible interfaces.** augmented returns `KktResult` (β, q_corrected, error_bound, inertia). projection returns `Solution` (verdict, q, β, margin). No common interface. The capacity pipeline only uses augmented.

2. **Duplicated accumulator logic.** hk2017 and billiard both manually track best_certified / best_uncertain with identical loop structures. This should be a shared pattern.

3. **No trinary pipeline.** The plan calls for TRUE/FALSE/INDETERMINATE verdicts flowing through the capacity pipeline, with rational fallback for INDETERMINATE nodes. Currently: manual threshold checks, no rational integration.

4. **File naming and test organization.** Files in `geom/` have names that don't always indicate content. Test files are long and mix multiple concerns. Convention should be single-concern-per-test-file.

5. **15 broken experiment binaries.** Phase 1 changed the Polytope4D API; experiments weren't updated. 144 call sites across 15 files.

6. **Test suite is slow (~150s).** Proptest cases, no `#[ignore]` for slow tests, no shared fixtures.

7. **Mixed parameterizations.** Some code uses (normals, heights), some uses dual vertices aᵢ. The crate should use dual vertices everywhere (decision from Jörn).

## Constraints and decisions (from Jörn)

- Both KKT solver variants are kept (different algorithms with different numerical properties, for comparison)
- Dual vertices aᵢ everywhere — no unit-length normals
- Single crate, module separation (not multi-crate)
- Fully general p×m KKT solver (not hardcoded p=5)
- Separate f64 and rational implementations (no generics)
- YAGNI/KISS: don't abstract unless needed
- Single-concern-per-file for test files
- Folder depth should follow logical hierarchy

## Key documents

- **This brief** — read first
- **Plan file** (`.claude/plans/synchronous-jumping-robin.md`) — algorithm design (Parts A-D) is the authoritative spec for what the KKT solver and capacity pipeline should do. The implementation phases are outdated.
- **`kkt-rework-spec.md`** (repo root) — older design notes, partially superseded by the plan file. Not reviewed by Jörn.
- **`handoffs/kkt-module-spec.md`** — subagent implementation spec from Phase 2. Agent working notes, not authoritative.
- **`CLAUDE.md`** — project conventions and workflow rules.

## What NOT to do

- Don't incrementally patch the old code. Design the target state, then implement.
- Don't delete the augmented solver. It's a different algorithm, not legacy code.
- Don't confuse "solves the same mathematical problem" with "redundant code."
- Don't write code. Plan first.
