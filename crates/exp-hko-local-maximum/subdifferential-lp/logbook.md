# Subdifferential LP: Logbook

Split from the original `gradient-is-zero/` experiment (Phase C: LP test). See `../gradient-analysis/logbook.md` for full history including motivation, interpretation, and theoretical framework.

## Status

**Active.** Script runs, results printed to stdout.

## How to run

```bash
python3 phase_c_lp_test.py
```

Requires: `gradient-analysis/hko-neighborhood-sensitivity.jsonl` (sibling experiment data).

## Files

| File | Role |
|---|---|
| `phase_c_lp_test.py` | LP test: 0 in conv(per-orbit gradients) in (n,h)-space |
