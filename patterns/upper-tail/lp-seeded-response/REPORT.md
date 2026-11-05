# Completing LP conditional responses recovers twelve fresh exact capacities

17 October 2026. A frozen LP-seeded response rule recovered the **exact capacity of all twelve new rational near-HKO products**, including six with exact systolic ratio greater than one. Completing the missing opposite-factor response repairs two fresh failures of the previous one-response extractor. However, strict alternating response remains trapped at nonglobal support pairs from many LP seeds. The useful mechanism is coverage by several LP-derived seeds and both angular words, not a guarantee that coordinate ascent finds the global pair.

This is a prospective, constructed local challenge, not a population holdout, a simple scalar invariant, a global capacity predictor, or a global systolic bound. No manuscript claim is added. It gives exact new capacity predictions and a support-selection explanation; it does not close the upper-tail prediction objective beyond this finite panel.

## Frozen method, inputs, and separation from truth

The [plan](PLAN.md) compared a fixed additional response, full strict alternating response, conditional decomposition, and exhaustive pair search. It chose all unchanged LP row/column conditionals as seeds, with first-max tie conventions and no parameter fitting or capacity labels. The numerical LP pilot and exact primal/dual certificate reconstruction remain unchanged. The resulting selector is exact conditional on these reconstructed LP optimizers; exact reconstruction does not make the numerical LP implementation wholly exact or prove tie-independent transfer.

For each positive row conditional, respond in A and then complete the missing B response; for each positive column conditional, respond in B and then A. These two unconditional maximizers use the first closure vertex attaining the maximum, even on an exact tie. They never decrease the objective and place both factors at closure vertices. Then alternate in that same factor order, accepting only a strict objective increase and preserving the current vertex on ties. Stop after a full sweep with no update. Take the first largest terminal objective across both words and every seed. [analyze.py](analyze.py) implements separate immutable `diagnostic`, `predict`, and `oracle` stages.

After initialization, strict updates increase a potential on a finite vertex-pair set, so no pair repeats at accepted updates. The algorithm therefore terminates at a coordinatewise maximum for its fixed word. The independently verified fresh factors each have five closure vertices, hence at most 24 accepted strict updates after initialization. The old coordinate-trap diagnostic instead has 4×5 closure vertices; facet count alone does not imply a 5×5 closure set. Neither finite termination nor coordinatewise maximality implies global optimality.

The base is the exact HKO normal presentation with every $\sqrt5$ replaced by $2236068/1000000$. Six new SHA256-derived directions, under the namespace `msc-math/thesis/lp-seeded-response/validation/v1:`, independently perturb all twenty normal coordinates at scales $1/100$ and $1/20$. All twelve bodies belong to the sample in advance; neither capacity nor an above-one label affects membership. [freeze.py](freeze.py) and [sample.json](sample.json) contain the complete rule and geometry.

Source, plan and input hashes were sent to the coordinator before execution. Sample SHA256 is `668594f0b2b82dc2c5ed9237c7a14314e77abab6b0fa7f7dc0206fce0a0a3154`. Every factor was independently checked bounded with five genuine facets before scoring. All twelve predictions were frozen and sent to the coordinator before any fresh oracle scoring: [predictions.json](predictions.json), SHA256 `0c5ca4cb4b6599a2be092b6a82c2fef3a4cfd57c8d1596beceb7eaf15f207ef7`. No reconstruction failed and no body was replaced.

Only then did the oracle enumerate 25 closure pairs and both angular words per fresh body. The [six-facet reduction](../../../topics/products/product-qp-six-facet-reduction.tex) and [two-shadow theorem](../mixed-area-order/REPORT.md) justify coverage. Full-pair objectives are truth only and never enter the selector. Exact planar halfplane areas give volume and exact systolic ratios. All twelve recovered exactly, so the first-strict-miss stop did not truncate this panel.

## What completing the response fixes

All six scale-$1/100$ cases have exact ratio above one, ranging from $1.025703\ldots$ to $1.035587\ldots$. All six scale-$1/20$ cases lie below one, from $0.950162\ldots$ to $0.991048\ldots$; they remain included. [results.json](results.json) contains every exact capacity and ratio. This is a deliberately local, paired construction, so twelve successes do not estimate performance on an unobserved population.

Replaying the previous one-response extractor as the first initialization state reveals two fresh misses:

| Fresh body | Exact ratio (display) | Old relative capacity error | Completed-response recovery |
|---|---:|---:|---|
| `lpseed_near_hko_s4_e100` | 1.0284576741 | 0.2877597% | Exact |
| `lpseed_near_hko_s4_e20` | 0.9637302713 | 1.4702625% | Exact |

The new selector was frozen before either truth was known. For both bodies its selected negative-shadow row-two seed already finds the winning A support $\{0,2,4\}$ on its first response. Its fixed B conditional is a strict mixture of the winning B closure with support $\{0,1,3\}$ and a losing B closure with support $\{1,3,4\}$. Writing those B vertices as $\gamma_*$ and $\gamma_{\rm bad}$,

$$\widehat\gamma=\lambda\gamma_*+(1-\lambda)\gamma_{\rm bad},\qquad
Q_*-Q(\alpha_*,\widehat\gamma)=(1-\lambda)\bigl(Q_*-Q(\alpha_*,\gamma_{\rm bad})\bigr)>0.$$

The good mixture weights are approximately $0.6080$ and $0.5689$, respectively. [mechanism.json](mechanism.json) retains the full rational weights, exact loss identities and conditional support coverage; [describe.py](describe.py) verifies these identities from the frozen outcomes with independent rational HK calculations. The true B vertex is absent as a pure row conditional in both words. The winning A vertex appears as a pure column conditional only in the positive word, whereas the displayed recovering seed uses the negative word; the positive-word response has strictly smaller objective. The exact finite candidate comparison establishes the old misses; pure-conditional coverage alone would not.

One additional opposite response removes the B mixture and attains the global objective in both fresh misses. In fact the maximum across all immediately completed seeds already recovers all twelve fresh capacities. Subsequent strict iterations improve some individual seeds, but add **no selected capacity improvement** on this panel. Thus this investigation supports the missing-response repair and finite transfer of the frozen stronger rule; it does not establish that repeated ascent is necessary or has a broader advantage.

The exposed `near_hko_s4_e100` diagnostic is also repaired, recovering its previously certified capacity $4.942255298349177\ldots$. The old `random_5x5_s1_64` coordinate-trap body recovers $6.2032265060958105\ldots$, equal to its already exact retained LP interval. These are design diagnostics, not new validation successes. Their geometries and LP certificates were reused unchanged; no fresh diagnostic full-pair oracle was run.

## Why alternating response still gets trapped

Among 240 fresh seed trajectories, 185 terminate at a strict coordinatewise maximum for their fixed word whose value is below the global product objective. This count does not say that all 185 are suboptimal for that same fixed word. For 159 of those trajectories, even re-evaluating the other angular word on the same two terminal closures remains below the true objective. Counts refer to seed trajectories, which can share endpoints, not distinct traps or independent trials. The remaining 26 demonstrate why losing word orientation must be separated from losing supports.

A concrete strict support trap occurs already on the first above-one fresh product, `lpseed_near_hko_s0_e100`, with exact ratio $1.0257032309\ldots$. Positive-shadow row-four initialization/ascent terminates at

$$\operatorname{supp}\alpha=\{1,2,4\},\quad
\operatorname{supp}\gamma=\{0,2,3\},\quad
Q_{\rm trap}=\frac{24751765278273751790691272836009}{247533028083666522052000000000000}.$$

Its exact best-response margins over every other A closure vertex and every other B closure vertex are strictly positive, approximately $0.00020008$ and $0.00084001$. Thus no additional single-factor response can leave this pair. The two shadow objectives on this pair are equal; by the two-shadow theorem this pair is already optimal over every word. Exhaustive word optimization on these same supports cannot fix the loss.

Nevertheless the same positive word has a strictly better pair, with supports A $\{1,3,4\}$ and B $\{0,2,4\}$ and exact objective

$$Q_{+,\rm better}=\frac{100278157491270831180990668725143}{994612709865517349156000000000000}>Q_{\rm trap}.$$

Thus this is a strict coordinate trap even for one fixed word, rather than merely a loss from choosing the wrong orientation. The global product objective is larger still; its exact gap is retained in `mechanism.json`.

A different frozen seed, negative-shadow row zero, reaches the global pair with supports A $\{0,2,3\}$ and B $\{1,2,4\}$. The full rational closure weights, both word values, strict response margins and winning seed are in `mechanism.json`. This is a support-choice obstruction to a single response trajectory even above ratio one, not a counterexample to the all-seed extractor: the competing seed is present and selected. The small objective gap matters here because its strict sign certifies a qualitative trap; no numerical-polishing gain is claimed.

## Verification and reproduction

[verify.py](verify.py) imports no producer, optimizer or numerical dependency. It uses the previous independent standard-library rational geometry and direct four-dimensional HK checker to reconstruct all frozen inputs, verify every LP primal/dual certificate, replay deterministic initialization and strict updates, check terminal coordinatewise optimality, and evaluate every scored pair-word objective with genuine four-dimensional closure weights. [verification.json](verification.json) records PASS for all twelve fresh capacities, 28 LP certificates and 272 seed trajectories including the two separate diagnostics, and 600 fresh direct-HK oracle objectives. The coordinator independently replays the fresh trajectories and uses a support-prefix oracle instead of the producer's signed matrix formula.

From the patterns checkout, audit the retained packet:

```sh
python3 -B patterns/upper-tail/lp-seeded-response/verify.py all \
  --repo "$PWD" --directory patterns/upper-tail/lp-seeded-response
python3 -B patterns/upper-tail/lp-seeded-response/describe.py \
  --repo "$PWD" --directory patterns/upper-tail/lp-seeded-response
```

These commands replace only the packet's verification/mechanism receipts. For a fresh replay, copy `PLAN.md`, `freeze.py`, `analyze.py`, `verify.py` and `describe.py` to a new scratch directory; run `freeze.py`, `verify.py geometry`, `analyze.py diagnostic`, `analyze.py predict`, freeze the prediction hash, then `analyze.py oracle`, `verify.py all`, and `describe.py`, each with `--repo` and `--directory`. The `predict` command needs an existing NumPy/SciPy environment; retained execution used Python 3.13.3, NumPy 2.5.3 and SciPy 1.18.1 without installation. Prediction and oracle stages refuse replacement. Alternative numerical LP optimizers may select other exact optimizer certificates, so transfer claims are scoped to the retained choices. No LP uniqueness or open-neighborhood success theorem is asserted.

Scientific diagnostic, prediction and oracle CPU totaled approximately **2.46 seconds**, below the 180-second allowance. All files began under `/tmp/msc-math/lp-seeded-response/20261017T1800Z/`; no network, installation, cloud, broad producer, tuning, manuscript change, push or merge occurred. Inspected sources were the checkout README, research/product/empirical guides, collaboration rationale, previous exact near-HKO/conditional/shadow reports and code, and their sparse/two-shadow proof dependencies. This is no external-literature or exhaustive novelty determination.

[review: /root/intake_review]
