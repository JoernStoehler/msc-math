# Session: Paranoia Check — Numerical Claims

Verify every numerical claim in experiment logbooks and TASKS.md against actual data. Fix obvious mismatches, flag the rest.

## Scope

All experiments under `crates/exp-*/` and `crates/dev-*/`. Each has a `logbook.md` with findings. Cross-check those findings against:
- JSONL data files in the same directory
- Python analysis scripts (run them if needed: `cd <dir> && uv run analyze.py`)
- Rust binary output (if JSONL is missing, check if `run.rs` produces the data)

## What to check

For each experiment logbook:
1. **Counts** — "70 random polytopes", "536 cuts", "100 perturbations" — do these match the JSONL row counts?
2. **Extrema** — "max sys=0.578", "best sys=0.9127", "curvatures -0.31 to -0.02" — verify against data
3. **Statistics** — "mean=0.8226", "rho=-0.02", "slope=2.00" — recompute from data if feasible
4. **Qualitative claims** — "all decrease sys", "0% vs 100% transition failures" — spot-check
5. **Cross-references** — claims in TASKS.md that cite experiment findings — do they match the logbook?

Also check `thesis/handwritten-notes.md` claims against data:
- "15-dim space of directions with D_d sys = 0" — verify rank computation
- "characteristic per-component radius epsilon*~0.035" — verify
- "7x aspect ratio" — verify

## Output

Produce a report file at `crates/paranoia-numerical-claims.md`:
- For each experiment: what was checked, what matched, what didn't
- Mismatches: quote the claim, quote the data, note the discrepancy
- Fix obvious errors in logbooks/TASKS.md directly (wrong number, typo)
- Flag anything where the data doesn't exist to verify (experiment not re-run since migration, etc.)

## Conventions
- Read `.claude/rules/*.md` for project conventions
- Read `CLAUDE.md` for general guidelines
- This is a READ-HEAVY session. Editing is limited to fixing verified mismatches.
- Work in a branch, not on main. Don't merge.
