# Joint closure: concrete explanation and integration choice

17 October 2026. Two optional English passages now explain what the latest data result changes: selecting balanced weights on each factor separately differs from finding two choices that work together. Neither passage is selected into the manuscript. The existing containment inclusion question remains unchanged; this packet supplies an additional concrete option without another question.

The [321-word selection passage](SELECTION-DRAFT.md) is the recommended additional explanation if this scope is selected. It explains the conditional matrix and feasible witness extraction, reports the retrospective numerical comparison and the material bound cut, and avoids importing new global exactness claims. Its insertion still needs a brief methods definition of how the matrix is selected and resolvable evidence references.

The [exact-certificate passage](EXACT-CERTIFICATE-DRAFT.md) also explains the capacity sandwich, rank-one certification and HKO relaxation obstruction. A focused source check found a real extra obligation: Chapter 4 contains the six-facet reduction and all-word enumeration, but lacks the two-angular-order theorem that makes the computed two LPs a global lower-capacity certificate. That theorem and the elementary LP/extraction argument need an appropriate manuscript proof location before this stronger version can be inserted. A short paragraph alone does not provide them.

[Placement and sources](PLACEMENT.md) compare these options with the existing containment explanation and no addition, criticize misleading rubrics, identify exact result fields and state the proof/evidence obligations. Useful understanding per integration hour favors the selection passage over mandatory theorem expansion when hand-in speed matters. This is an editorial recommendation, not human acceptance or authorization for insertion.

The independent research assessment also declined classifying the nine remaining LP intervals merely to accumulate strict-gap certificates. HKO and the rational 5×6 pilot already obstruct universal LP sharpness; more strictness would improve neither feasible upper bounds nor the global ratio envelope. The nine intervals remain unresolved. A support-coverage guarantee would be a different mathematical crux, and is not selected here.

## Evidence and verification

Sources inspected locally were selected Chapters 4 and 8; the [two-shadow theorem](../../../../../patterns-2026-10-17/patterns/upper-tail/mixed-area-order/REPORT.md); the [conditional LP](../../../../../patterns-2026-10-17/patterns/upper-tail/conditional-coupling/REPORT.md), [rounding pilot](../../../../../patterns-2026-10-17/patterns/upper-tail/conditional-rounding/REPORT.md) and [35-body validation](../../../../../patterns-2026-10-17/patterns/upper-tail/conditional-validation/REPORT.md); and the [retained independent integration checks](../conditional-validation-integration/README.md). This was focused source access, not an exhaustive repository or literature review.

[check.py](check.py) independently joins the draft's counts and exact bound reduction to the retained result rows, checks their agreement with the existing scientific receipt, and checks that the current manuscript PDF is identical to the Kai candidate. It records the focused missing-theorem source check. It does not rerun the scientific producer or revalidate the imported mathematics. [verification.json](verification.json) binds the source and draft bytes. Separate [read-only scientific review](review.md) checked the mathematical implications and both passage choices.

From the thesis checkout:

```sh
python3 -B thesis/review/2026-10-17/joint-closure-explanation/check.py \
  --thesis "$PWD" --patterns ../patterns-2026-10-17 \
  --packet thesis/review/2026-10-17/joint-closure-explanation \
  --out /tmp/msc-math/joint-closure-explanation-replay.json
```

All new material began in task-specific `/tmp/msc-math/` directories and was deliberately retained here with the editorial evidence owner. Root owns integration, local commit and cleanup; the patterns branch remains separate. The current 105-page manuscript and Kai packet remain unchanged.

[review: /root/gap_crux]
