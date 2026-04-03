# Gradient Analysis: Logbook

Split from the original `gradient-is-zero/` experiment (Phase A: sensitivity analysis + gradient ascent).

## Status

**Active.** Data generated, figures produced.

## How to run

```bash
cargo run -p exp-hko-local-maximum --release --bin hko-gradient-analysis
python3 analyze.py
```

## Files

| File | Role |
|---|---|
| `run.rs` | Rust binary: sensitivity analysis + gradient ascent |
| `analyze.py` | Python figures + analysis |
| `hko-neighborhood-sensitivity.jsonl` | Gradients at HKO2024 (1 row, all 44 orbit gradients inline) |
| `hko-neighborhood-ascent.jsonl` | Gradient ascent trajectory (1 row) |
| `hko-neighborhood-gradient.png` | Bar chart of d_sys/d_h_k |
| `hko-neighborhood-orbits.png` | Orbit structure visualization |
