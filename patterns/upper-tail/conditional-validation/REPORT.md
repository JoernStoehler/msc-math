# Joint closure rounding recovers the frozen upper tail, with one material new exact bound

17 October 2026. The unchanged conditional-closure LP and row/column rounding rule numerically matches the inherited capacity label on **all 26 principal upper-tail products and all nine bulk controls** in a frozen panel excluding its three rational design geometries. It improves the strongest existing feasible bound on two tails, with **one material 1.49596% cut**, and has no regressions. Exact LP dual and feasible-word certificates now prove capacity equality on **26/35 bodies**, including **17/26 tails**. Nine other capacities remain uncertified despite numerical agreement.

The material example is `random_5x5_s3_227`: its old feasible bound was 1.51868% above the inherited numerical label. The new jointly selected support pair gives capacity approximately **5.69193709365**, exactly equal to the new LP lower bound. This eliminates the earlier numerical miss and certifies the previously numerical capacity value, without a Cartesian support-pair enumeration. There was no previously certified interval for this body; both its new lower certificate and new attaining witness are produced here.

This is strong finite evidence for a joint signed-geometry support mechanism, the priority Jörn rated about 2. It remains an optimization procedure rather than a simple scalar invariant. Previously exposed labels, a small deliberately selected panel, solver-dependent optimal couplings and nine remaining LP intervals prevent claims of prospective prediction, universal exactness or a global HKO-maximality theorem. The bounded test stops without tuning or another cohort.

## Frozen choice and test

[PLAN.md](PLAN.md) records the end state, three distinct alternatives, criticized rubrics and the receiving owner's bounded authorization before scoring. A containment/asymmetry scalar would be simpler but had no principled new formula overcoming the known canonical-closure obstruction; exact simplicial-cone disjunctions risk disguising the existing exhaustive capacity method. The selected test instead challenges whether the new joint mechanism transfers beyond its design geometries. Bulk accuracy, HKO recovery alone, tiny improvements and exact arithmetic alone could each reward the wrong work. The material criterion was an exact capacity-bound reduction of at least 1% on a principal tail against the strongest retained comparator.

The [sample](sample.json) takes the previous 36-body full-partner validation panel and excludes `random_5x5_s1_64`, `random_5x6_s1_198` and `random_6x6_s3_124`, the three conditional-LP design bodies. Only the middle name was present, leaving **35 bodies: 26 principal top-2% controls, with 9/8/9 from 5×5/5×6/6×6, plus nine prior hash-selected bulk controls**. Original family ranks, roles and earlier 20-body exclusions remain unchanged. All geometries and labels had already been exposed to other pattern scans; design exclusion does not make this prospective holdout data. Source SHA256 values bind the retained geometries, original panel, prior method code and exact HKO certificates.

Frozen PLAN SHA256: `86f578bea89c4db521e87764642a88993e2a790cf060e86e4fd3bf34f9d28838`. Frozen sample SHA256: `bc456cc2c24f8c4140d2fb4aac8de687edbd7a34a93ed98f87ee2ec8bbbee176`. Both were sent to the receiving owner before scoring.

## What the selector computes

For normalized factor closures $\alpha,\gamma$, the fixed-word HK objective is $Q_w=\sum_{ij}c_{ij}(w)\alpha_i\gamma_j$, where $c_{ij}=\varepsilon_{ij}a_i\cdot b_j/4$. The unchanged [conditional LP](../conditional-coupling/REPORT.md) maximizes the same linear objective over nonnegative couplings $W$ of total mass one, with every row closing in the B normals and every column closing in the A normals. Rank-one products are feasible, so the larger of its two angular-shadow optima gives a capacity lower bound.

The unchanged [rounding rule](../conditional-rounding/REPORT.md) normalizes each nonzero row to a B closure, takes one A best response, and symmetrically normalizes each nonzero column and takes one B response. It keeps the largest feasible objective across both shadow words. This uses one-factor closure vertices, without a Cartesian search of their pairs. It performs no extra sweep, restart or altered word. No capacity label or incumbent enters support selection. This panel required **470 conditional responses**, 12–20 per body, after two LP solves per body; these small polygon sizes do not establish asymptotic or practical superiority over every exact capacity implementation.

The standalone upper bound is $C_{\rm round}=1/(2Q_{\rm round})$. The strongest incumbent is the minimum of the existing full-partner bound and its comparator minimum over width, selected triangle, width-contact, earlier signed-prefix response and canonical closures. The combined upper bound is $\min(C_{\rm round},C_{\rm inc})$. Standalone and combined results coincide on this panel because the new rule never loses. The exact HKO control is replayed first over $\mathbb Q(\sqrt5)$ and again recovers $Q=1/10$ from the retained optimal couplings; no new HKO theorem or claim of uniqueness follows.

## All upper-tail results

“Agreement” means relative difference from an inherited numerical capacity label below $10^{-10}$; it is not an exactness certificate. Counts within 1% or 5% likewise use those numerical labels. Exact reductions compare rational feasible bounds. Every tail falls in the predeclared standalone 5% bucket; that finite result supplies no uniform guarantee.

| Principal tails | New agreement | Incumbent agreement | New within 1% | Incumbent within 1% | New exact capacities | Bound cuts ≥1% | Regressions |
|---|---:|---:|---:|---:|---:|---:|---:|
| 5×5 (9) | 9 | 8 | 9 | 8 | 7 | 1 | 0 |
| 5×6 (8) | 8 | 8 | 8 | 8 | 4 | 0 | 0 |
| 6×6 (9) | 9 | 8 | 9 | 9 | 6 | 0 | 0 |
| All (26) | **26** | **24** | **26** | **25** | **17** | **1** | **0** |

All nine bulk controls agree numerically for both methods, and their old feasible witnesses are now certified exactly by the new LP. Across all 35 bodies there are only two strict bound improvements:

| Body | Exact feasible-bound reduction | New certificate |
|---|---:|---|
| `random_5x5_s3_227` | **1.49595999%** | New rounded upper equals new LP lower |
| `random_6x6_s2_63` | 0.05411594% | New rounded upper equals new LP lower |

The second improvement is numerical polishing by itself and does not justify this investigation. The first closes the largest remaining miss of the strongest previously retained bounds in this panel. Its positive-shadow coupling is rank one; the selected A support is `[0,1,4]`, B support `[0,2,4]`, with full word `[9,0,5,1,6,2,7,3,4,8]` and zero weights on inactive labels. Exact weights, objectives, capacities and duals are in [results.json](results.json).

A descriptive post-score check identifies the compatibility mechanism: on exactly **26 bodies**, the larger LP objective has a retained rank-one optimizer, so its normalized rows and columns expose an attaining factor pair. All 26 become exact capacity certificates. On 24 of these, the old feasible witness already attains the newly computed LP bound; those old upper bounds become newly certified here. Two need the new rounding witnesses. Thus the `prior_exact_capacity` field means “old witness matches this newly computed lower bound,” not “the body was already certified before this packet.”

On the remaining nine tails the retained dominating LP optimizer is not rank one; this does not exclude a convex mixture of rank-one closures, while rounding still numerically matches the historical labels. The resulting upper/lower capacity interval factors range from about **1.00001347 to 1.03364105**. The largest, on `random_5x5_s3_54`, remains a 3.3641% interval. An unequal interval does not prove that the true capacity or relaxation has a gap; the feasible upper witness may still be suboptimal. Numerical agreement alone cannot settle those nine cases. No alternate optimal coupling is searched after observing this outcome.

For each exact area product $A_PA_Q$, feasibility also yields the ratio upper bound $C_{\rm round}^2/(2A_PA_Q)$. This packet's scientific result is support selection and capacity certification on the retained panel. The earlier feasible bounds already put all 35 ratios below one, so there is no new ratio exclusion. It supplies no new above-one example, global ratio envelope or conjecture sufficient to prove global maximality.

## Verification and reproduction

[verify.py](verify.py) uses only the Python standard library, imports no optimizer or producer, and independently reconstructs all 35 panel memberships and source-normal factor areas by all-pairs feasible halfplane intersections and a rational convex hull. It checks **70 rational LP primal/dual certificates**, every conditional and one-factor response optimum, and **470 direct four-dimensional HK objectives**, plus the exact HKO coupling and rounding checks in $\mathbb Q(\sqrt5)$. It recomputes every summary, joins all incumbent identities and independently verifies full-partner incumbent witnesses where that branch wins. [verification.json](verification.json) reports PASS and states this scope. The receiving owner's separate checker independently reconstructs the new geometry, areas, LPs, conditional responses and strongest-incumbent metrics. Prior comparator feasibility retains the scope of the pinned [full-partner validation checks](../full-partner-validation/geometry-verification.json); [owner-verification.json](owner-verification.json) reports PASS.

The source methods remain unchanged. The scored pass took **5.40 seconds wall and 6.90 seconds CPU**, below the authorized 120-second CPU cap. All 35 bodies completed with no solver or exact-reconstruction failure; no row was deleted or replaced. SciPy 1.18.1 was available in the existing offline cache. No dependency installation, source-population producer, full support-pair capacity computation, fitting, new body, network, cloud work or manuscript edit occurred.

From the patterns checkout, choose a new scratch directory, copy this packet's `PLAN.md`, `sample.json`, `analyze.py` and `verify.py` into it, then run:

```sh
uv run --offline --with scipy python -B "$OUT/analyze.py" \
  --repo "$PWD" --out "$OUT/results.json"
python3 -B "$OUT/verify.py" --repo "$PWD" --directory "$OUT"
```

`analyze.py` writes only its named output; the verifier writes `verification.json` within the selected output directory. HiGHS chooses an optimal coupling and active basis; exact rational certificates validate the resulting values and feasibility, while cross-version identity of degenerate optimizer choices is not promised. Inputs, code and retained outputs are pinned in [manifest.json](manifest.json).

New files began under `/tmp/msc-math/upper-tail-next/20261017T1637Z/` and were deliberately retained with the pattern research owner. The receiving owner controls cross-branch integration and commits. Inspected sources were repository/research/product entry points and collaboration guidance, the pattern owner follow-up, stopped geometric selectors, two-shadow theorem, full-partner construction/validation, and conditional-LP/rounding reports, code and certificates. This is focused local source access, without an exhaustive literature or repository novelty assessment. The result remains outside the selected manuscript.

[review: /root/conditional_science_review]
