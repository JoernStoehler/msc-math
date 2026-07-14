# Ridge-tail anatomy (step 2)

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
  decomposition checks, and retained target values;
- `artifacts/current/group-summary.json`: fixed generic comparisons and the
  product `5x5` rho-only/ridge-only/overlap/matched-control contrasts;
- `artifacts/current/validation.json`: joins, group sizes, face composition,
  source feature agreement, identities, and translation/scaling checks.
- `artifacts/current/analysis-manifest.json`: input/output hashes and row counts
  for byte-identical regeneration checks.

## Fixed comparisons and interpretation

The generic selected-minus-disjoint-baseline mean `sys` contrast is `+0.28714`
(selected mean `0.62571`, baseline `0.33857`). Ranks 1--10 do not harden over
ranks 11--100: the contrast is `-0.03176` (`0.59712` versus `0.62888`). In the
same selected panel, Euclidean ridge area is much smaller than the baseline,
while Euclidean-area-weighted `kappa` is lower (`0.3769` versus `0.5029`).
This is consistent with ordinary morphology acting as the coarse gate and
Kähler alignment attenuating rather than explaining the transfer; it is not a
causal or predictive estimate.

For product `5x5`, the frozen two-seed arms contain 42 rho-only, 42 ridge-only,
8 overlap, and 50 matched-control rows. Relative to controls, mean `sys`
contrasts are `+0.31974` (rho-only), `+0.24769` (ridge-only), and `+0.26476`
(overlap). The overlap is not better than either exclusive arm, so the packet
does not support a tuned intersection rule. Rho-only and ridge-only have
similar weighted `kappa` (0.3837 and 0.3706) but much smaller Euclidean-area
sums than controls (119.6 and 98.3 versus 289.6), again favoring the ordinary
morphology branch over a standalone Kähler-angle mechanism.

Allowed use is retained-data description, source/facet/vertex composition
checks, and routing the next geometric question. Prohibited use is a new
proposer claim, generic rho validation, target-free prediction, a capacity
theorem, causal mechanism claim, or generalization beyond these frozen source
and seed designs. Product action-level joins are not made: the available bounce
artifacts do not carry an unambiguous join to these random `5x5` polytope IDs.

## Wishlist disposition

Updated rough costs are in `wishlist-disposition.tsv`. Shared source inventory,
the decomposition substrate, analyzer/provenance surface, complete face
summaries, generic comparisons, product discordant arms, retained strata, and
negative controls are done. The generic rho replay is postponed (about 27 min
wall and 2.4 CPU-hours, target-free and no longer prospective); bounce joins,
displays, conjecture extraction, and the generator-transfer intervention
interface are postponed until a clean downstream consumer exists.
