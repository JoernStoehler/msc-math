# Combinatorial Cells Next Steps

## Active thread
Consolidate interpretation debt before adding new experiments.

### Immediate objectives
- Refresh the quantitative baseline from current artifacts and compare against historical values so future claims cite explicit current rows.
- Decide whether sweep robustness should be improved before expanding run scales (especially for larger `MAX_FACET_COUNT`).
- Close the two unresolved interpretation points: formal continuity of `sys` at boundaries and systematic handling of `multiple-crossings` construction failures.

### Blockers
- Open continuity and failure behavior are currently evidence-heavy but not fully formalized.
- The old design files contain multiple hypotheses marked as unresolved/negative; some were based on smaller or older datasets.
- Running full binaries is expensive; many defaults are exponential in facet count and amplify small numerical-policy changes.

### Commands to continue from here
- Recompute shared artifacts after any algorithmic change:
  - `cargo run -p exp-combinatorial-cells --release --bin cell-omega`
  - `cargo run -p exp-combinatorial-cells --release --bin cell-widths`
  - `cargo run -p exp-combinatorial-cells --release --bin cell-boundary-characterization`
  - `cargo run -p exp-combinatorial-cells --release --bin cell-convexity`
  - `cargo run -p exp-combinatorial-cells --release --bin cell-multiple-crossings`
- Re-run all analyses from each directory (`uv run analyze.py`).
- If continuing near-Lagrangian exploration, update `experiments/combinatorial-cells/omega-hypothesis/` rather than adding new legacy notes.

### Stop condition
Pause this thread when either:
- refreshed run outputs are archived (JSONL + plots) and interpreted against the old 140-polytope snapshot, or
- we formalize and accept/reject a concrete route for the two open blockers above with explicit file-level references.
