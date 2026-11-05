# Conditional closures: exact capacity certificates and an HKO obstruction

17 October 2026. The explicit premortem selected a bounded investigation independent of the five unanswered human judgments. Jörn's B rating prioritizes a simple explanation of upper-tail ratios, possibly toward global maximality. Existing geometric support choices give complementary finite gains but exact misses. The new question was whether a small linear relaxation could certify tightness of those feasible witnesses without enumerating all support pairs.

## What changed

The [scientific packet](../../../../../patterns-2026-10-17/patterns/upper-tail/conditional-coupling/REPORT.md) replaces a product of normalized closure weights by a nonnegative matrix whose rows and columns each satisfy the opposite factor's closure equations. Product weights remain feasible. Optimizing each of the two fixed angular words therefore bounds the true maximum HK objective **above**, and capacity **below**. Existing feasible words bound capacity above. This gives a certified interval, rather than another capacity upper-bound selector.

If either closure cone is simplicial, every such matrix decomposes into nonnegative products of actual closures. The LP is then exact. The proof uses linear independence of the cone rays to transfer the other closure equation to each decomposition component. In particular, a genuine triangle or quadrilateral factor guarantees exactness; this is an exactness mechanism, without a literature-novelty or measured-speedup claim.

On the predeclared `random_5x5_s1_64` coordinate-trap control, exact primal/dual LP certificates match the existing directly checked feasible word. Thus its capacity is now certified exactly (approximately 6.2032265061), without a new full support-pair enumeration. Two further frozen stress controls have certified capacity interval factors 1.01073615 and 1.02935016. Their interval widths do not prove a gap between the relaxation and true capacity; numerical agreement with inherited labels does not establish endpoint equality. The sample is deliberately retrospective and contains only three rational stress cases.

The decisive limitation already occurs at HKO. In transformed dual coordinates over $\mathbb Q(\sqrt5)$, the known exact objective is $Q_*=1/10$, while exact primal/dual certificates give

$$L_+=(-5+7\sqrt5)/100,\qquad L_-=(5+3\sqrt5)/100.$$

Hence $L_{\max}/Q_*=(1+3/\sqrt5)/2\approx1.17082039$. The LP capacity lower bound is only $(3\sqrt5-5)/2\approx0.85410197$ times the true capacity. An explicit circulant conditional coupling attains the larger LP value but cannot be a convex mixture of genuine product closures. The obstruction is structural, rather than numerical tolerance. This stops this relaxation as an HKO-sharp certificate before any larger sample; it does not reject all coupling refinements or approximate predictors.

## Why this route and why it stopped

The [frozen plan](../../../../../patterns-2026-10-17/patterns/upper-tail/conditional-coupling/PLAN.md) compares additional support sweeps, new geometric seeds, exact support-pair enumeration and the chosen conditional coupling. Small fit gains would reward polishing; low solve cost alone would overlook the HKO gap; perfect answers through full enumeration would restate the existing algorithm. Derivation of the bound direction, the HKO reference, named traps and exact independent certificates cannot be skipped. The pilot fixes three rational cases and three regular-pentagon angles; no corpus scan, fitting, new population or continuation followed the HKO failure.

## Independent support and disposition

[owner-verify.py](owner-verify.py) imports no scientific selector or LP solver. It reconstructs all six rational LP matrices and angular objectives, verifies exact primal feasibility, dual inequalities and equal objectives, joins three source geometries and retained source hashes, and directly checks all three upper-bound words in four dimensions. [The rational receipt](verification.json) records PASS. [owner-hko-verify.py](owner-hko-verify.py) uses a separately implemented rational-pair field with exact square comparisons. It reconstructs both HKO matrices/words, checks primal/dual optimality, and independently constructs the circulant coupling and averaged coefficients; [the algebraic receipt](hko-verification.json) records PASS. The receiving owner also reconstructed the simplicial decomposition proof; separate scientific review is retained with the source packet.

Reproduce from this checkout with `P=../patterns-2026-10-17`:

```sh
python3 -B thesis/review/2026-10-17/conditional-coupling-integration/owner-verify.py \
  --patterns "$P" --results "$P/patterns/upper-tail/conditional-coupling/results.json" \
  --out /tmp/msc-math/conditional-coupling-rational-recheck.json
python3 -B thesis/review/2026-10-17/conditional-coupling-integration/owner-hko-verify.py \
  --certificate "$P/patterns/upper-tail/conditional-coupling/hko-exact.json" \
  --out /tmp/msc-math/conditional-coupling-hko-recheck.json
```

These findings remain outside the selected manuscript, applying Jörn's existing preference for method obstructions. They do not supply the requested working held-out upper-tail predictor or a global-maximality conjecture. The current 105-page candidate and Kai agenda/checklist remain unchanged. No new human answer or acceptance is inferred. Five existing questions retain their independent decisions; neither pending editorial task blocks this completed investigation. Root owns thesis integration, local commit and cleanup; the scientific worker owns its separately reviewed patterns commit. No merge, push, send, form submission or public deployment occurred. Focused local sources and retained evidence were inspected; no exhaustive repository/literature novelty search was performed.

[review: /root/delivery_review]
