# First-Order HKO Checks

Question: what first-order sensitivity and active-orbit structure does the
fixed-`F=10` numerical model report at HKO2024, and does its simple Armijo
ascent accept a nearby improving step?

`main.rs` starts from the hardcoded HKO polytope. The retained phase-A outputs
are:

- `hko-neighborhood-sensitivity.jsonl`: one row with 717 valid orbits and 150
  within the packet's declared one-percent near-optimal gap;
- `hko-neighborhood-ascent.jsonl`: one rejected/no-change ascent row;
- `exact-certification-bank.jsonl`: six selected exact-vs-f64 solver and
  derivative cross-check rows.

The 150-orbit count belongs to this artifact and threshold. Do not silently
substitute older 44-orbit counts from earlier HKO notes. The nonzero
single-branch gradient and stopped ascent are numerical bookkeeping, not a
critical-point test for the nonsmooth minimum envelope. The six-row exact bank
does not establish branch completeness or replace the theorem certificate.

Safe smoke mode computes the sensitivity probe and stops before writing:

```bash
cargo run -p exp-hko-local-maximum --release --bin hko-first-order -- --smoke
```

The full mode rewrites the two tracked phase-A JSONL files. Exact-bank mode
writes an untracked smoke artifact unless `--canonical` is also supplied:

```bash
cargo run -p exp-hko-local-maximum --release --bin hko-first-order
cargo run -p exp-hko-local-maximum --release --bin hko-first-order -- \
  --exact-bank --canonical
```

`uv run --script analyze.py` reads the two phase-A files and rewrites the
tracked gradient and orbit figures. Changes to near-optimal selection,
derivative conventions, orbit canonicalization, or the HKO fixture require
rechecking all three retained outputs and any downstream numerical counts.
