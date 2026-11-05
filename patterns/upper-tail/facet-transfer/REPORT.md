# Six-facet transfer challenge: finite agreement, no new mechanism

17 October 2026. The unchanged LP-seeded completed-response extractor recovers the exact full-pair two-shadow capacity on all twelve freshly constructed rational 6x6 products. Every body has systolic ratio strictly above one, with displayed range 1.01099959–1.03224236. This bounded challenge crosses a genuine five-to-six-facet boundary, but agreement supplies no new obstruction, support-coverage mechanism, scalar invariant, or universal guarantee. Under the frozen selection standard, it produces **no material new scientific result**. The challenge is closed without tuning or another cohort.

## Question and frozen design

[PLAN.md](PLAN.md) compares small vertex cuts against an HKO chamber theorem, more five-facet scans, and exposed six-facet labels. The chosen crux was whether an extra genuine facet destroys support extraction in the intended upper tail. All twelve geometries use six fresh hash perturbations of the rationalized HKO presentation, at fixed perturbation scale 1/100, and two fixed cut depths 1/1000 and 1/100. For each factor, the added normal is `(u+v)/(2*(1-delta))`, where u and v are chosen by fixed seed-dependent adjacent angular positions. No capacity or ratio enters construction, membership, or predictions. This is a deliberately local constructed challenge; it is not a population sample or a scalar-invariant predictor.

Exact halfspace intersections verify six primal vertices and six genuine facets in every factor. Each defining inequality is active at two vertices; the origin satisfies all inequalities strictly. A positive normalized closure using every normal verifies boundedness. Thus these are actual hexagons, rather than pentagons padded by redundant inequalities. [geometry.json](geometry.json) retains the rational witnesses.

The predictor is mechanically adapted from `patterns/upper-tail/lp-seeded-response/analyze.py`: only its hardcoded 5x5 row/column dimensions are generalized to n,m. It keeps the same conditional LP optimizer/reconstruction, angular words, pair-first/lexicographic closure order, first-max ties, two unconditional initialization responses, and later strict-ascent rule. It does not score every closure pair or consult oracle truth in prediction mode.

All geometries and predictor/source hashes were frozen first. All twelve predictions and twenty-four rational primal/dual LP certificates were then saved before oracle evaluation. Root independently recorded the checkpoint and acknowledged its hashes before authorizing oracle. The sample SHA256 is `70c52bc59ed0fc86cf6843791aa129422e825b697f3b2216c6ae7f0ef68bd7ea`; the prediction SHA256 is `d457d959b4e01af9149baf4c99952a3cb4628a77888f872562a2250bf90324cf`.

## Bounded outcome and checks

[results.json](results.json) records all twelve exact recoveries with no unscored suffix. Inspection of its already-produced exact winners shows an added facet in a maximizing support on seven bodies; both added facets occur in a maximizing support on two bodies. In each affected factor the added facet occurs in every maximizing support for that body. Thus the extra facets sometimes participate in the capacity witness, although this observation supplies no general extraction mechanism. This count only inspects the retained oracle winners; no new solve or score was computed. Every factor has eight closure vertices, so exact ground truth evaluates 64 closure pairs in each of two words, 1,536 pair-word objectives overall. The capacity identification depends separately on the sparse-closure reduction in `topics/products/product-qp-six-facet-reduction.tex` and the product two-shadow theorem recorded in `patterns/upper-tail/mixed-area-order/REPORT.md`; the finite calculation does not prove these dependencies.

[verify.py](verify.py) imports no optimizer or prediction producer. Its standard-library exact rational replay uses the previously independently owned direct four-dimensional HK, closure, angular-word, and LP check functions. It verifies genuine geometry, all twenty-four LP primal/dual certificates, all 240 nonzero conditional-response trajectories, every oracle value, the first-max/strict-update rules, exact recovery, and exact area/ratio identities. [verification.json](verification.json) records PASS and source/output hashes. Root's independent Gaussian-closure/direct-four-dimensional-HK and prefix check also passes: twenty-four LP certificates, 240 nonzero seed trajectories, 92 strict updates, and 768 closure pairs with both words reconstructed. Its scientific CPU receipt is 13.986 seconds. Root owns final integration.

Scientific CPU receipts are 0.089 seconds geometry construction/check, 3.689 prediction, 0.702 oracle, and 8.821 exact replay, approximately 13.30 seconds for worker execution and replay, plus 13.986 seconds for root's independent audit, approximately 27.29 seconds before retention. A post-copy replay of the retained packet also passes in 8.815 CPU seconds, bringing recorded scientific CPU to approximately 36.10 seconds. The initial system-Python prediction startup lacked scipy and stopped before any LP or prediction; execution then used an existing cached scipy interpreter with no installation or network access. The twelve input geometries and frozen mathematical rule were unchanged.

Reproduce the read-only audit from the patterns checkout with:

```sh
python3 -B patterns/upper-tail/facet-transfer/verify.py \
  --repo "$PWD" --directory patterns/upper-tail/facet-transfer
```

This writes only a fresh verification receipt in the retained packet. Prediction and oracle files are immutable and their producer rejects replacement. To replay the complete frozen pipeline without overwriting evidence, copy the four source files below into a new scratch directory before construction or prediction:

```sh
facet_replay=$(mktemp -d /tmp/msc-math/facet-transfer/replay-XXXXXXXX)
cp patterns/upper-tail/facet-transfer/{PLAN.md,freeze.py,analyze.py,verify.py} "$facet_replay/"
python3 -B "$facet_replay/freeze.py"
facet_python=/home/joern/.cache/uv/environments-v2/analyze-2981accd25e0cfb8/bin/python
OPENBLAS_NUM_THREADS=1 OMP_NUM_THREADS=1 "$facet_python" -B "$facet_replay/analyze.py" predict --repo "$PWD" --directory "$facet_replay"
# All predictions now exist before the truth stage.
OPENBLAS_NUM_THREADS=1 OMP_NUM_THREADS=1 "$facet_python" -B "$facet_replay/analyze.py" oracle --repo "$PWD" --directory "$facet_replay"
python3 -B "$facet_replay/verify.py" --repo "$PWD" --directory "$facet_replay"
```

The freezer uses the assigned patterns checkout path in its source constant. The cached Python path is a local dependency environment, not a portable installation promise. Timing receipts vary across replay; rational geometries and scientific values are the reproducible content. Retention records a stopped challenge for audit and avoids repeating it; it does not promote agreement to a material result. No manuscript change, new capacity bound, global-maximality claim, or additional investigation is selected. Reporting owner: `/root/facet_transfer`; receiving, retention, review, commit, and cleanup owner: `/root`.

[review: /root/intake]
