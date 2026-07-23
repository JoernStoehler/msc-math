# First-Boundary Characterization

Question: what event occurs at the first combinatorial boundary along selected
directions, and how do `sys`, the producer-selected minimizing
characteristic, and its gradient change immediately across it?

`main.rs` consumes `../polytopes.jsonl` and writes the retained anatomy,
crossing, and gradient JSONL files. The Rust producer has no smoke mode and
refreshes tracked evidence:

```bash
cargo run -p exp-combinatorial-cells --release \
  --bin cell-boundary-characterization
```

`analyze_transition_atlas.py` joins the three retained outputs by their unique
direction key and writes:

- `first-boundary-transition-summary.json`;
- `first-boundary-transition-exceptions.json`;
- `first-boundary-transition-report.md`.

The readable report is the first result to inspect. The atlas supports
hypothesis generation about omega-sign crossings, selected-sigma changes, and
gradient kinks; it is not a continuity proof or mechanism result. Selected
sigma is the producer's returned representative, not an enumeration of tied
minima, and the starting action gap is not a certified immediate-boundary gap.

`gradient-discontinuity/` consumes this packet's anatomy and gradient outputs.
Changes to keys, epsilon policy, sigma semantics, or output schema require
checking that consumer and rerunning the joined-atlas validation.
