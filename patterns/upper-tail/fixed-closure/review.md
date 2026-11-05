# Independent review of the fixed cone-volume closure obstruction

Reviewer: `/root/closure_math_review`, 17 October 2026.
Candidate: `/tmp/msc-math/owner-20261017/pattern-crux.md`, the private memo read at 12:29 UTC.

**Mathematical verdict: PASS, with one decimal presentation correction.** No internal mathematical gap was found in the claimed canonical fixed-closure obstruction. This is a source-conditional review of the derivation and its cited local proofs; it is not human mathematical acceptance or a literature novelty assessment.

## Findings and source anchors

1. **Normalization is correct.** With factor weights normalized to one, the product weights are half of them. Thus a six-block term is `(x1·R y1 − x3·R y2)/2` and the representative scale is `rho=s²/2`, not `s²/4`. The maintained source defines the circumradius-one polygons and ratio at `topics/products/pentagon-affine-products/research.tex:57–67`, the HK convention at `:113–121`, and the half-factor/four-block/six-block normalization at `:140–156` and `:207`. This also checks the direction `Q_can ≤ Q_*`, hence `C_can ≥ c_*`.

2. **The uniform closure really is the average of the five vertices.** Each normal occurs once as distinguished, and twice as nondistinguished. Its average factor weight is `[h/w+2/(2w)]/5=1/5`. The regular polygon's edge lengths and heights are constant, so these are its cone-volume weights. The five triples and their weights are explicitly classified at `research.tex:230–247`; there are no opposite-pair vertices omitted. More generally `sum l_i h_i=2A` and `sum l_i n_i=0` prove the candidate's feasibility. The steps `l_i n_i/(2A)` are translation invariant even though the individual normalized covectors and weights change with the interior origin.

3. **Averaging remains valid for every full order.** For a fixed order, same-factor pairings vanish, and cross-factor terms are bilinear in the two normalized factor closures, with the common product factor `1/4`. Therefore replacing each uniform closure by its average expands into exactly 25 equally weighted restricted triple-pair objectives. Removing the zero entries preserves their order/objective (`research.tex:121`); there is no interchange of maximum and average being assumed. Source bilinearity is stated at `:132`. Taking maxima only after the uniform per-order bound is valid.

4. **Exactly five pairs have relative quarter-turn angle.** With factor angles `18°+72°(j−i)`, the condition `j−i=1 mod 5` gives 90°, and supplies five of the 25 pairs. Rotating each triple to its distinguished normal reduces these pairs to the stated representatives against `J`.

5. **The `rho*t` bound is complete.** All six-block normalized tensors are the twelve possibilities `ev^T−wz^T` classified at `research.tex:286–301`. The two representative matrices and their left/right reflections have absolute pairings with `J` respectively in `{0,t/2}` and `{0,t}`. In particular `T2F` attains magnitude `t`, and reversing the cyclic word gives the positive sign when required. Four-block values are `rho*u·Jv` and have magnitude at most `rho*t/2`; every nonempty proper sum of a three-step closure is an individual step or its negative. Two blocks give zero. The cyclic two/four/six completeness argument is at `:135–156`, independently consistent with `topics/products/product-triangle-bounce-classification.tex:311–315`. No physical billiard transition test is needed to upper-bound the unrestricted HK word objectives.

6. **The global control and conversions are correct.** The source's exact pentagon formula gives `Q_*=rho*cos(18°)` and `c_*=w²/cos(18°)` (`research.tex:304–312`). Consequently `Q_can ≤ r Q_*` with `r=(7−sqrt(5))/5<1`, because `tan(36°)/cos(18°)=3−sqrt(5)`. Taking reciprocals and then squares gives precisely the stated capacity factor `1.049553179...` and ratio factor `1.101561876...`. With the HKO ratio `(3+sqrt(5))/5`, this yields a surrogate ratio at least `1.1535705729...`.

7. **One decimal wording fix is needed.** The proven relative ratio excess is `10.1561876097...%`. Thus the introductory phrase “at least 10.1562%” rounds the lower bound upward. Replace it by “at least 10.1561%” or “approximately 10.1562%”. The exact inequalities and the other displayed rounded lower bounds are sound.

## Interpretation and scope

The gap proves that **this cone-volume canonical closure, optimized only over facet ordering**, cannot itself reproduce the sharp HKO capacity or systolic upper bound. It does not exclude other fixed closure choices, support selection, other signed-ridge functionals, or taking the minimum with width. Width is already exact on the HKO reference, so the memo correctly treats the minimum as unexcluded. Nor is `Q_can` computed exactly: the averaging bound is sufficient for the structural failure.

The method-selection recommendation is proportionate: reject a standalone candidate before a corpus scan when its own exact reference control provably misses by this amount. A user-facing adaptation should retain both the restricted conclusion and the explicit absence of an upper-tail claim.

## Verification performed

I checked the source conventions, normalization, bilinear expansion, closure weights, complete block classification, exact symbolic pairings, and error direction by hand. A bounded independent Python `Fraction` enumeration of all 720 orders of a single quarter-turn triple pair found objective coefficients divided by `rho*t` equal to `{-1,-1/2,0,1/2,1}`, confirming the maximum `rho*t`; no corpus, fitting, support search, or capacity producer was used. This happened before the root coordinator reported its independently completed equivalent check. I also evaluated only the final constants to check their printed decimals. I made no project or PM edits.

## Exact-message boundary

This is **not exact-text approval of a future retained adaptation**. The inspected candidate calls itself a private scratch memo, says independent review is still pending, and has no frozen `[review: /root/closure_math_review]` line. A retained version is scientifically fit in principle after the small correction and accurate status/provenance updates, but the sender must supply the complete actual adapted text for a separate `APPROVE <version>` verdict. Changing only the header cannot automatically carry this review into a user-visible artifact.

## Retained adaptation review sequence

- `closure-report-v1` at `patterns/upper-tail/fixed-closure/REPORT.md`: REJECT pending a definition of `V` and framing exactness as the selected sharp-reference goal rather than a universal admissibility standard for predictors. The complete text, `verify.py`, and `verification.json` were inspected.
- `closure-report-v2`: selected-target framing now adequate, including explicit nonclaims about looser empirical approximation; REJECT solely because `V` remains undefined. No additional mathematical objection.
- `closure-report-v3`: **APPROVE closure-report-v3.** The complete retained report was read again after the explicit definition `V=area(P_5)^2` was added. No unresolved consequential objection remains. This approval covers the exact complete report at `/workspaces/msc-math-worktrees/patterns-2026-10-17/patterns/upper-tail/fixed-closure/REPORT.md` as read at 12:31 UTC, including `[review: /root/closure_math_review]`; it does not apply to future substantive revisions. The mathematical derivation, selected-target framing, limits, and retained verification description are fit to deliver.
