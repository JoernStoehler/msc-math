# Subdifferential LP: Logbook

Split from the original `gradient-is-zero/` experiment (Phase C: LP test). See `research/hko-local-maximum/design/gradient-analysis.md` for full history including motivation, interpretation, and theoretical framework.

## Status

**Broken (2026-04-04).** Script crashes with `KeyError: 'normals'`. The (n,h)→(a) parameterization migration updated the Rust binary's output schema (`normals`/`heights` → `dual_vertices`, removed `d_vol_h`/`d_vol_n`) but this script was not updated. The script reimplements symplectic geometry primitives (ω₀, J₀, KKT reconstruction, capacity/volume derivatives) in Python — all of which now exist in the Rust library. Fix: have the Rust binary output per-orbit `∇sys` in a-space, then the script just loads gradients and runs the LP. See TASKS.md.

## How to run

```bash
uv run phase_c_lp_test.py
```

Requires: `gradient-analysis/hko-neighborhood-sensitivity.jsonl` (sibling experiment data).

## Files

| File | Role |
|---|---|
| `phase_c_lp_test.py` | LP test: 0 in conv(per-orbit gradients) in (n,h)-space |
