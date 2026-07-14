# Ridge-tail anatomy (step 2)

## Decision for `sys > 1` search

Low ridge is a useful coarse filter: on generic `F=10`, the selected 1% has
mean `sys=0.62571` versus `0.33857` for a disjoint baseline. But pushing ridge
lower inside that 1% slightly worsens the result: ranks 1--10 have mean
`sys=0.59712` versus `0.62888` for ranks 11--100. Generating another candidate
decade merely to take more extreme ridge minima is therefore not a supported
route to `sys>1`.

Ridge may still save target evaluations by rejecting poor starting points. A
second-stage coordinate must add information not contained in “make ridge
smaller.” It may be another prospectively validated scalar, several
coordinates, or a local/non-scalar search; this packet does not choose among
those portfolio alternatives.

This packet is a retained-data anatomy analysis for the completed generic
`F=10` ridge panel and the frozen product covariance validation. It makes no
capacity/`sys` call and does not alter either frozen manifest. The scientific
object is the per-planar-ridge decomposition

`A_symp(F) = A_Euclidean(F) * kappa(F)`,

computed with `f64` volume and the repository coordinate order
`(q1,q2,p1,p2)`. A face is eligible only when its incidence-derived cyclic
ordering succeeds and its Euclidean area is positive. Product `5x5` faces are
classified from source-owned dual normals: 25 mixed (one q-factor and one
p-factor facet) and 10 same-factor structural-zero faces.

## Reproduction

The committed `artifacts/input/` files are the deterministic input snapshot;
the product snapshot contains only the 142 retained `5x5` rows from the two
frozen covariance seeds. `artifacts/input/provenance.json` records source paths,
hashes, row identities, and the pre-target/evaluation boundary. Re-run the
analysis without any external cache:

```bash
python3 experiments/sys-datascience/methods/ridge-tail-anatomy/analyze.py \
  --input-dir experiments/sys-datascience/methods/ridge-tail-anatomy/artifacts/input \
  --out-dir /tmp/ridge-tail-anatomy-current
```

The producer can refresh the input snapshot when the retained source caches
are available. Its output directory must be empty; it refuses accidental
overwrites. The producer command used for the committed snapshot is recorded
in `artifacts/input/provenance.json`.

Generated artifacts:

- `artifacts/current/per_face.jsonl`: ordered face vertices, Euclidean and
  symplectic areas, `kappa`, face kind, and invariance diagnostics;
- `artifacts/current/per_polytope.jsonl`: complete safe summary family,
  volume-normalized and raw E/A summaries, separate mixed/structural product
  summaries, decomposition checks, and retained target values;
- `artifacts/current/group-summary.json`: fixed generic comparisons and the
  product `5x5` rho-only/ridge-only/overlap/matched-control contrasts, per-seed
  uncertainty, and fixed post-target descriptive rank associations;
- `artifacts/current/validation.json`: joins, group sizes, face composition,
  source feature agreement, identities, and translation/scaling checks.
- `artifacts/current/analysis-manifest.json`: input/output hashes and row counts
  for byte-identical regeneration checks.

## Fixed comparisons and interpretation

The generic selected-minus-disjoint-baseline mean `sys` contrast is `+0.28714`
(selected mean `0.62571`, baseline `0.33857`). On the scale-invariant anatomy
coordinates, selected versus baseline means are normalized Euclidean ridge
area `0.9167` versus `1.3653`, normalized symplectic ridge area `0.3403` versus
`0.6205`, and Euclidean-area-weighted `kappa` `0.3769` versus `0.5029`.
Thus both normalized ordinary area and `kappa` move in the low-ridge direction;
lower `kappa` reinforces lower symplectic area rather than attenuating it.

Ranks 1--10 do not harden over ranks 11--100: the `sys` contrast is `-0.03176`
(`0.59712` versus `0.62888`). The corresponding normalized Euclidean means are
`0.9419` versus `0.9139`, weighted `kappa` is `0.3400` versus `0.3810`, and
normalized symplectic means are `0.3147` versus `0.3431`; the first ten have
slightly *higher* ordinary area but lower `kappa` and lower `sys`. Within the
selected 100, post-target Spearman associations with `sys` are `-0.263`
(normalized Euclidean mean), `-0.306` (normalized Euclidean sum), `+0.149`
(normalized symplectic mean), `+0.039` (normalized symplectic sum), and `+0.290`
(weighted `kappa`). These are descriptive, selected-after-target associations,
not mechanism or prediction evidence. The fixed within-selected evidence
therefore changes the conclusion from “ordinary morphology versus kappa” to
“both factors contribute to the coarse gate, while neither is yet a supported
causal mechanism or proven interior `sys` coordinate.”

For product `5x5`, the frozen two-seed arms contain 42 rho-only, 42 ridge-only,
8 overlap, and 50 matched-control rows. Using only the 25 mixed faces, the
normalized Euclidean means are `0.7015` (rho-only), `0.6285` (ridge-only),
`0.6392` (overlap), and `1.0556` (controls); mixed weighted `kappa` is `0.6052`,
`0.6080`, `0.5899`, and `0.6586`, respectively. Mixed normalized symplectic
means are `0.4231`, `0.3815`, `0.3768`, and `0.7071`. Relative to controls,
mean `sys` contrasts are `+0.31974` (rho-only), `+0.24769` (ridge-only), and
`+0.26476` (overlap). Seed-specific rho/control contrasts are `+0.323`
(`[0.235,0.411]`) and `+0.317` (`[0.229,0.406]`); ridge/control are `+0.244`
(`[0.155,0.332]`) and `+0.254` (`[0.161,0.347]`). The overlap has only 8 rows
(2 in one seed and 6 in the other): its pooled overlap-minus-rho estimate is
`-0.055` with approximate normal interval `[-0.108,-0.002]`, while
overlap-minus-ridge is `+0.017` with `[-0.038,+0.072]`. Treat these as
proportionate descriptive uncertainty, not a categorical ranking or support
for a tuned intersection. Both normalized mixed area and mixed `kappa` are
lower in the selected arms, but the packet does not isolate a causal mechanism.

Allowed use is retained-data description, source/facet/vertex composition
checks, and routing the next geometric question. Prohibited use is a new
proposer claim, generic rho validation, target-free prediction, a capacity
theorem, causal mechanism claim, or generalization beyond these frozen source
and seed designs. Product action-level joins are not made: the available bounce
artifacts do not carry an unambiguous join to these random `5x5` polytope IDs.

## Wishlist disposition

Updated rough costs are in `wishlist-disposition.tsv`. Shared source inventory,
the decomposition substrate, analyzer/provenance surface, complete normalized
face summaries, generic comparisons, product discordant arms, retained strata,
and negative controls are done. The generic rho replay is postponed (about 27
min wall and 2.4 CPU-hours, target-free and no longer prospective); bounce
joins, displays, conjecture extraction, and any orientation intervention
interface are postponed until a clean downstream consumer exists.
