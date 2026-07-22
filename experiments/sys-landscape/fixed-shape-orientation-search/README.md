# Fixed-shape symplectic-orientation scan

This exploratory experiment separates two sources of variation that the random
datasets normally conflate: Euclidean shape and orientation relative to the
standard symplectic form. It selects the highest-`sys` body from each retained
generic/product dataset, holds that Euclidean body fixed, and scans a
two-parameter representative of `SO(4)/U(2)`.

This is a deliberately post-selection diagnostic. It asks whether an already
strong Euclidean candidate can improve through non-symplectic rotation; it is
not a prospective proposer or a population estimate.

## Result

Each body received 164 evaluations: a coarse grid followed by two local
refinement rounds.

| Selected source champion | Source size | Identity `sys` | Best rotated `sys` | Change |
| --- | ---: | ---: | ---: | ---: |
| generic `random_F10_s3_104` | 4,096 | 0.859560 | 0.859560 | 0 |
| product `random_5x6_s3_168` | 10,240 | 0.862586 | 0.878308 | +0.015722 |

The rotated product value exceeds every `sys` value in both retained source
datasets (14,336 rows total). Thus Euclidean candidate quality and symplectic
alignment contain partly separate information even after selecting the source
champion. The scan still found no `sys > 1` case.

This does not establish that the displayed rotation is a local or global
orientation optimum. The two bodies were selected after observing `sys`, and
the scan does not estimate how often rotation improves a new body. A broader
`GL(4)/(Sp(4) x R_+)` search would also vary linear Euclidean shape and answers
a different, five-dimensional question.

## Reproduction

From the repository root:

```bash
cargo run -p exp-sys-landscape --release --bin sys-fixed-shape-orientation-search
uv run --script experiments/sys-landscape/fixed-shape-orientation-search/analyze.py \
  --input experiments/sys-landscape/fixed-shape-orientation-search/evaluations.jsonl \
  --output experiments/sys-landscape/fixed-shape-orientation-search/analysis.json
```

- `main.rs` owns selection, orientation, capacity evaluation, and JSONL output.
- `evaluations.jsonl` retains every evaluated point.
- `analyze.py` derives `analysis.json` and compares the best rotated value with
  both retained source distributions.
