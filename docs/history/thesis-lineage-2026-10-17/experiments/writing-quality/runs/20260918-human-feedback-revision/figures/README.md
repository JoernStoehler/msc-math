# Historical ridge association figures

`association-overview.svg` (and PNG) shows all 14,336 historical rows, separating generic polytopes and Lagrangian products. `association-stratified.svg` (and PNG) further separates all eight generic facet counts and all ten polygon-factor size buckets. All axes are shared across panels. Every row is included; no tail is clipped. Transparency only reduces overplotting; there is no fitted curve, subsampling, or binning.

Suggested main-text caption:

> The numerical systolic ratio decreases as the normalized sum R of unsigned symplectic ridge areas increases. Each point is one sampled polytope; generic polytopes and Lagrangian products are displayed separately. The horizontal axis is logarithmic (equal distances correspond to equal multiplicative changes in R); the vertical axis is untransformed. These are the historical numerical evaluations. The stratified figure shows that the association also persists within each sampled facet/factor-size class.

Suggested stratified caption:

> The same observations separated by sampling class: generic polytopes with 5–12 facets and products of polygons with k and m sides, 3 ≤ k ≤ m ≤ 6. Each panel uses the same axes. Negative rank association is present in every class; pooling the classes is therefore not its sole source.

`association-receipt.json` records exact hashes, row counts and statistics. Within-class Spearman coefficients range from −0.982 to −0.841 for generic bodies and −0.997 to −0.878 for products. These are descriptive reanalyses of the same table, not independent replication. The wider mathematical interpretation is not established by these plots.

Reproduce from the worktree root:

```sh
uv run experiments/writing-quality/runs/20260918-human-feedback-revision/figures/plot_association.py
```

The script uses the verified historical artifact in the local cache and the tracked provenance table. Both SHA256 hashes must match before plotting. Override locations with `--table PATH --provenance PATH`; no new capacity evaluation or artifact mutation occurs. It writes only this directory's figures and receipt. Dependencies are pinned inline. Both PNGs were inspected visually after generation.
