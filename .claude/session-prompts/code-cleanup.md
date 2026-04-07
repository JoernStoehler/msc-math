# Session: Code Cleanup

Autonomous code quality session. No thesis files — crates/ only.

## Tasks

### 1. Step-bound code deduplication
`compute_step_bound` exists in two places:
- `crates/exp-combinatorial-cells/cell-widths/` (enriched version with omega_0 detection, catches 43% of boundaries)
- `crates/exp-sys-landscape/gradient-ascent-general/` and `gradient-ascent-products/` (missing omega_0 detection)

Either unify into library, or copy the enriched version into the gradient-ascent experiments. Check if the implementations are different, understand why, pick the best approach.

### 2. Wiggle strength justification
`gradient-ascent-general` and `gradient-ascent-products` use 5% wiggle strength (inherited from deleted gradient-search, unjustified). `cell-widths` provides per-facet cell widths (0.12-0.26) that could inform this. Read both experiments, determine if 5% is reasonable given the cell width data, and either justify it in a comment or propose a better value.

### 3. gradient-ascent + multiple-crossings overlap
`multiple-crossings` (exp-combinatorial-cells) does multi-boundary sweeps with sys tracking, which answers a gradient-ascent question. Check if gradient-ascent experiments duplicate this capability. If so, recommend whether to remove multiple-crossings or keep both.

### 4. Math.tex stubs audit
Scan all `crates/**/math.tex` files for `[TODO: JÖRN` entries. For each, check: was there ever a proof or citation? Did it get dropped during migration? Specific known examples:
- `lem:positive-span` and `lem:vertex-enumeration` in `geom/math.tex` — proof-less since first commit
Report what you find. Don't write proofs — just audit and document gaps.

### 5. Draft `[lem:dual-vertex-qp]` proof
In `crates/library/src/kkt/qp_assembly.rs:58-61` there's a TODO to prove the a_i QP formulation recovers the same optimal action as (n,h). The equivalence is mechanical: the a_i formulation is (n,h) with h=1 as a choice. The proof should:
- State the (n,h) formulation
- Substitute h_i=1, a_i=n_i
- Confirm |n_i|=1 (unit length) was never used in the derivation
Write this as a draft in `crates/library/src/kkt/math.tex` wrapped in `\begin{unverified}...\end{unverified}`.

## Verification
- `cd crates/library/ && cargo test --release --lib` must still pass
- `cd crates/library/ && cargo clippy --lib -- -D warnings` must be clean
- `cd crates/ && latexmk` must compile

## Conventions
- Read `.claude/rules/*.md` for project conventions
- Read `CLAUDE.md` for general guidelines
- Work in a branch, not on main. Don't merge — report what you did.
