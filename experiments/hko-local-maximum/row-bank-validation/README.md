# HKO Row-Bank Validation

Status: live validation machinery. This folder is not the theorem certificate.

The Rust binary exports selected exact-bank rows from hardcoded HKO and control
fixtures. The Sage script independently recomputes the same rows and compares
exact q/action/beta/capacity-gradient values against the Rust export.

Run:

```bash
cargo run -p exp-hko-local-maximum --release --bin hko-row-bank-validation
sage -python experiments/hko-local-maximum/row-bank-validation/analyze.py
```

Use `--canonical` on both commands only when refreshing the tracked canonical
input/report:

```bash
cargo run -p exp-hko-local-maximum --release --bin hko-row-bank-validation -- --canonical
sage -python experiments/hko-local-maximum/row-bank-validation/analyze.py --canonical
```

Tracked canonical artifacts:

- `row-bank-validation-input.jsonl`
- `row-bank-validation-report.jsonl`

The report checks selected exact-bank code against Sage. It does not verify the
26-row feasible-section theorem certificate; that is owned by `../theorem/`.
