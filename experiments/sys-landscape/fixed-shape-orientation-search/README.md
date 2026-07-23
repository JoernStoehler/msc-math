# Fixed-shape symplectic-orientation and linear scan

This exploratory experiment separates two sources of variation that the random
datasets normally conflate: Euclidean shape and orientation relative to the
standard symplectic form. It selects the highest-`sys` body from each retained
generic/product dataset, holds that Euclidean body fixed, and scans a
two-parameter representative of `SO(4)/U(2)`.

This is a deliberately post-selection diagnostic. It asks whether an already
strong Euclidean candidate can improve through non-symplectic rotation; it is
not a prospective proposer or a population estimate.

`global.rs` extends the same diagnostic from the compact orientation quotient
to determinant-one linear changes modulo symplectic maps. It uses normalized
nondegenerate skew forms

```text
M^T J M = cosh(r) U + sinh(r) V,
```

where `U` and `V` range over the two unit 2-spheres in the Euclidean
self-dual/anti-self-dual splitting. The old `SO(4)/U(2)` family is exactly the
`r = 0` stratum. For `r > 0`, the representative map has condition number
`exp(r)`, so the radial panels test genuinely global distortion scales rather
than only a tangent neighborhood.

For the left action on dual maps the quotient is written
`Sp(4)\SL(4)`. Inversion identifies it with the `SL(4)/Sp(4)` convention.

## Research questions and disposition

1. **Does a high-`sys` body selected from the random datasets still improve
   when only its orientation is changed?** Yes for the selected product body:
   the compact scan raised `sys` from `0.862586` to `0.878308`. No improvement
   was found for the selected generic body. This is post-selection evidence
   that Euclidean candidate quality and symplectic alignment contain partly
   separate information.
2. **After that orientation scan, does sparse random sampling of global
   determinant-one linear distortions find a further improvement?** No. All
   432 sampled noncompact points were worse than the corresponding best sampled
   compact orientation.

The second result only rejects this naive random-transform proposal for these
two selected bodies. It does not show that the compact stratum contains a local
or global maximum. Further random global-transform sampling is stopped because
the negative pilot is sparse but already has low expected value. Reopen this
route only if there is a targeted optimizer or geometric mechanism, a new body
for which the comparison matters, or another reason to expect substantially
better value than additional random quotient samples.

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
the scan does not estimate how often rotation improves a new body. The
companion global scan varies linear Euclidean shape and therefore answers a
different, five-dimensional question.

### Determinant-one linear extension

The retained extension evaluated 216 noncompact points per body: 24 global
angular directions and 12 directions anchored at the best sampled compact
orientation, reused at each of six radial scales.

| Selected body | Compact best | Best noncompact at `r=0.125` | Best over all sampled `r>0` |
| --- | ---: | ---: | ---: |
| generic `random_F10_s3_104` | 0.859560 | 0.804071 | 0.804071 |
| product `random_5x6_s3_168` | 0.878308 | 0.836228 | 0.836228 |

No sampled determinant-one distortion improved either best sampled compact
orientation. The sampled maxima decreased further at every larger radial
scale, reaching below `0.001` at `r=4` (condition number about 55). This is
evidence about these two selected bodies and this sparse deterministic panel,
not evidence that the compact stratum contains the global maximum. In
particular, the four-dimensional angular space was not covered densely and the
noncompact radial coordinate was only sampled through `r=4`.

## Reproduction

From the repository root:

```bash
cargo run -p exp-sys-landscape --release --bin sys-fixed-shape-orientation-search
uv run --script experiments/sys-landscape/fixed-shape-orientation-search/analyze.py \
  --input experiments/sys-landscape/fixed-shape-orientation-search/evaluations.jsonl \
  --output experiments/sys-landscape/fixed-shape-orientation-search/analysis.json

cargo run -p exp-sys-landscape --release --bin sys-fixed-shape-linear-search
uv run --script experiments/sys-landscape/fixed-shape-orientation-search/analyze_global.py \
  --input experiments/sys-landscape/fixed-shape-orientation-search/global-evaluations.jsonl \
  --output experiments/sys-landscape/fixed-shape-orientation-search/global-analysis.json
```

- `main.rs` performs selection, orientation, capacity evaluation, and JSONL
  output.
- `evaluations.jsonl` retains every evaluated point.
- `analyze.py` derives `analysis.json` and compares the best rotated value with
  both retained source distributions.
- `global.rs` retains the compact best as a control, then samples common
  angular directions at radial distortions from `r = 0.125` through `r = 4`.
  It combines a quotient-wide angular panel with a panel anchored at the best
  sampled compact orientation.
- `global-evaluations.jsonl` and `global-analysis.json` are the corresponding
  retained global evidence and summary.
