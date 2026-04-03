# Facet-Splitting: Logbook

Split from `gradient-is-zero/logbook.md` (Phase B content). See `gradient-is-zero/logbook.md` for full history including motivation, interpretation, and theoretical framework.

## Status

**Active.** Data generated, figures produced.

## How to run

```bash
cargo run -p exp-hko-local-maximum --release --bin hko-facet-splitting
python3 analyze.py
```

## Files

| File | Role |
|---|---|
| `run.rs` | Rust binary: facet-splitting F=10 to F=11 |
| `analyze.py` | Python figures + analysis |
| `hko-neighborhood-splitting.jsonl` | Facet-splitting data (536 rows) |
| `hko-neighborhood-splitting.png` | Splitting results figure |
