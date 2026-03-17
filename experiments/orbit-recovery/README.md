# Orbit Recovery Validation

Validate the base point recovery algorithm (Lemma `lem:base-point-recovery`): given the EHZ algorithm output (sigma, beta), recover the base point b = gamma(0) and verify it produces a valid Reeb orbit on the polytope boundary.

## Status
Complete

## Design

- 112 polytopes: 7 known + 105 random (F=5..10)
- For each polytope: recover base point from optimal (sigma, beta), then verify closure, on-facet, inequality, and action constraints
- Validation thresholds: 1e-6 for closure/on-facet/violation, 1e-5 for action

## Key findings

- **112/112 polytopes pass** all validation checks
- Known polytopes achieve machine epsilon (~1e-14); random F=10 polytopes show accumulated numerical error up to ~1e-6
- Base point is generically unique (96.4% have dim=0). Non-uniqueness appears only for products and hypercube
- Recovery is fast (mean 0.025ms), negligible compared to capacity computation (mean 4.7ms)
- Null space optimization works for underdetermined cases (dim>0)

## Files

| File | Purpose |
|------|---------|
| `orbit_recovery.rs` | Rust binary: recovery + validation |
| `orbit_recovery.py` | Python: summary statistics |
| `plot_orbit_recovery.py` | Python: error distribution plots |
| `orbit-recovery.jsonl` | Dataset (112 rows) |
| `orbit-recovery.tex` | Thesis writeup |
| `orbit_recovery_errors.png` | Figure: error distribution |

## Run

```bash
cd experiments/
cargo run --release --bin orbit_recovery    # -> orbit-recovery/orbit-recovery.jsonl
python3 orbit-recovery/orbit_recovery.py    # -> summary statistics
```

## Known limitations

- Validation thresholds chosen empirically; may need adjustment for higher F
- Only F <= 10 tested; numerical error grows with F
- 1e-5 action tolerance is looser than other thresholds due to accumulated rounding over full orbit
