# Frozen pentagon-core diagnostic: integration check

The tested predictor is stopped. In each retained top-2% set of 5×5, 5×6 and
6×6 products, the fixed affine-pentagon core/width center improves **0 of 21**
capacity predictions over width alone. Its mean absolute capacity error is
respectively **3.626, 5.452 and 7.759 times** width's; median relative capacity
errors are **23.31%, 24.79% and 25.89%**. This settles the selected rule's route,
not the usefulness of all inscribed cores or geometry-aware predictors.

The feature uses the existing affine-pentagon capacity theorem on a fixed
inscribed core, giving an exact-arithmetic lower bound L, and combines it with
width W through sqrt(LW). Its known-family HKO control returns ratio
1.0472135955. Independent reconstruction of width from difference-polygon
polars agrees at all nine regular-rotation controls. This is consistency with
an imported theorem, with no new maximality result. The selected cores retain median area-product fractions of only 27–28%;
the identity L²/(2V) = coverage × own-core ratio exposes that attenuation.

The labels had already been explored, so the hash partition is retrospective.
The binary64 containment/bracket checks are numerical checks, not exact
certificates or new certified systolic-ratio intervals. The exact containment
implication and the existing affine theorem remain separate from this numerical
implementation.

## Disposition and independent evidence

Stopping the frozen rule is selected because it loses on all 63 main-tail
examples. Tuning its center or searching for better cores would be another
research selection; neither is authorized by this integration. HKO agreement
alone is an inadequate rubric because the feature imports that known family.
No manuscript passage or fixed Kai PDF changed; existing research-only editorial
scope is preserved. Five current questions remain distinct after independent
triage; no new question duplicates them.

The accompanying verification receipt records the independent native scientific
review, all 10,240 retained lower-formula/bracket calculations, separately
reconstructed synthetic widths, and unchanged four-file Kai packet bindings.
The retained checker uses explicit input/output paths and imports no worker
functions. Its output is disposable; create it under /tmp/msc-math/.

[Pattern source report](../../../../../patterns-2026-10-17/patterns/upper-tail/geometry-aware-probe/REPORT.md)
and its manifest own the research packet. From this thesis worktree, reproduce
the independent arithmetic check without a capacity solve:

```bash
python3 thesis/review/2026-10-17/pattern-core-integration/verify_retained.py \
  --rows ../patterns-2026-10-17/patterns/upper-tail/geometry-aware-probe/rows.jsonl.gz \
  --summary ../patterns-2026-10-17/patterns/upper-tail/geometry-aware-probe/summary.json \
  --out /tmp/msc-math/pentagon-core-independent.json
```

Requires Python and NumPy; writes only the explicit output. The retained
[verification receipt](verification.json) binds the reviewed row bytes. Root owns
this integration check; pattern commit `97089b92` remains on its separate branch.

[review: /root/batch_review]
