# Full-partner containment selects supports that escape the known coordinate trap

17 October 2026. A new geometric support rule uses a **minimum translated homothet containing the entire rotated partner**, then one response from the other factor. It recovers the exact feasible competitor missed by the earlier nonglobal coordinate fixed point. The new rule cuts that prior capacity bound by **4.2398%**, without inheriting width, contact or minimum-area-triangle seeds. An elementary LP duality identifies the selected sparse supports with contact facets of the full-partner containment problem.

On the same **20 retrospective controls**, it improves the strongest previous frozen bound on **8/20**, including **5/9** principal upper-tail controls. The principal `random_5x5_s1_221` bound falls **7.4528%**; its former 8.05% miss against the historical numerical capacity becomes numerical agreement. The new bounds agree with those labels on **17/20** controls and **8/9** principal controls. They are exact feasible upper bounds, not certificates of equality with true capacity. The separate regular-pentagon controls recover the known profile without a fallback.

The standalone rule still misses exact retained competitors: it is **2.8299%** above one existing feasible capacity bound and **2.6112%** above another. Thus this is a new support-selection mechanism with substantial finite explanatory value, rather than a uniformly exact predictor or a global maximality theorem. The frozen test stops here; no support tuning, repeated sweeps or population scan follows.

## Success story, alternatives and frozen choice

The intended end state was a compact, target-free support construction using relative factor geometry, with a proof of what it measures and a decisive test against the known support trap. Deriving its geometry, freezing ties and variants, checking HKO independently of inherited seeds, and comparing the strongest incumbent cannot be skipped: otherwise renaming an old rule, exploiting numerical symmetry or reporting redundant gains could masquerade as progress.

[PLAN.md](PLAN.md) records the pre-score choice and resources. Maximum-area normalized closure triangles were rejected analytically as a renamed old rule: if $T$ is the supporting outer triangle and $U$ its normalized closure edge triangle, then

$$U=-JT/(2\operatorname{area}T)\quad\text{up to translation},\qquad
\operatorname{area}U=\frac1{4\operatorname{area}T}.$$

Thus maximizing closure-triangle area is exactly minimizing outer-triangle area. Fixed full cone-volume closures alone already have a known HKO obstruction. All-pair support maximization would repeat the capacity algorithm. Full-partner containment instead uses every partner edge before selecting a sparse contact closure. A bulk-fit rubric rewards easy bodies; HKO alone rewards symmetry; any positive numerical gain rewards polishing. Exact trap escape and a new contact interpretation were the selected first discriminant.

The frozen rule has four variants: both shadow signs, with either factor optimized first. Start with both canonical cone-volume closures. Optimize the first factor once at that fixed full shadow word, then the other factor once. **Every response selects a closure vertex**, including on ties. Enumerate supports of size two, then three, and within each size use lexicographic input indices; choose the first objective maximizer. The best objective among the four variants gives the final bound. No prior seed or bound is part of this rule. A pre-score review caught that retaining a tied dense canonical closure would preserve it at regular pentagons and inherit the old failure; that design was corrected before execution.

Exact rational controls use zero comparison tolerance. Separate floating regular controls choose the first vertex within $10^{-11}$ of the true computed objective maximum. This is a deterministic computational tie-break, not a claim of a unique intrinsic contact triangle or invariance under arbitrary reindexing. The containment radius is geometric; tied support selection can depend on input indices.

## Why the first support solves a full-body containment problem

Let full-dimensional polygons with irredundant facets and interior origin be

$$P=\{x:a_i\cdot x\leq1\},\qquad Q=\{y:b_j\cdot y\leq1\}.$$

Write $A_P=\operatorname{area}P$, $A_Q=\operatorname{area}Q$ and $J(x,y)=(-y,x)$. A normalized closure has nonnegative weights summing to one and weighted facet covectors summing to zero. For $Q$, its canonical weights are

$$\gamma_j^{\rm can}=\frac{l_j h_j}{2A_Q},$$

where $l_j,h_j$ are edge length and support height. Its counterclockwise edge is $e_j=l_jJn_j$, with unit outward normal $n_j$ and $b_j=n_j/h_j$. Consequently

$$\gamma_j^{\rm can}b_j=-\frac{Je_j}{2A_Q},\qquad
J\gamma_j^{\rm can}b_j=\frac{e_j}{2A_Q}.$$

The positive-shadow edge polygon $V$ is therefore $Q/(2A_Q)$ up to translation. For an arbitrary normalized A closure $\alpha$, let $U_\alpha$ have counterclockwise edges $\alpha_i a_i$. The mixed-area support-edge formula gives

$$Q_+(\alpha,\gamma^{\rm can})
=M(U_\alpha,V)
=\frac1{4A_Q}\sum_i\alpha_i h_Q(-Ja_i).\tag{1}$$

This is the actual four-dimensional HK objective when each factor has mass one half. It is not an area score substituted for that objective. Define the translative containment radius

$$R(P,JQ)=\min_z\max_i\bigl(h_Q(-Ja_i)-z\cdot a_i\bigr)
=\min\{t:\exists z,\ JQ-z\subseteq tP\}.$$

The three-equality closure LP and its dual are

$$\max_{\alpha\geq0,\ \sum\alpha=1,\ \sum\alpha a=0}
\sum_i\alpha_i h_Q(-Ja_i)
=\min_{t,z}\{t:t+z\cdot a_i\geq h_Q(-Ja_i)\ \forall i\}
=R(P,JQ).\tag{2}$$

The closure polytope is nonempty and compact. The dual is feasible and attains its minimum: full-dimensional $P$ makes the maximum coercive as a function of $z$, up to the irrelevant factor translations. Standard finite-dimensional LP duality applies. Combining (1)–(2),

$$\boxed{\max_\alpha Q_+(\alpha,\gamma^{\rm can})
=\frac{R(P,JQ)}{4A_Q}.}$$

A maximizing vertex uses at most three facet covectors. Complementary slackness places its active facets among the contact facets of a minimum translated homothet of $P$ containing $JQ$. The negative shadow, reversed to make its objective nonnegative, replaces $JQ$ by $-JQ$. If B is optimized first, the corresponding support coefficients are $h_P(\pm Jb_j)/(4A_P)$, giving $R(Q,\mp JP)/(4A_P)$.

Translation does not change the radius: translating the contained body changes $z$, and translating the container changes $tP$ by a vector absorbed into $z$. Canonical weighted edge steps are also translation invariant. If the interior origin changes by a translation while facet indices are preserved, the closure weights rescale inversely to the covectors, leaving each weighted step unchanged. The deterministic selected step polygons therefore preserve this translation invariance; arbitrary relabeling at ties remains a separate issue.

For the other-factor response, the same fixed shadow word supplies linear signed-prefix coefficients. Maximize that closure LP once; no loop or word revision occurs. The two-shadow theorem then supplies the geometric interpretation, while feasibility alone gives the capacity bound

$$c_{\rm EHZ}(P\times_LQ)\leq C_{\rm full}=
\frac1{2\max_{\rm four\ variants}Q},\qquad
\operatorname{sys}(P\times_LQ)\leq\frac{C_{\rm full}^2}{2A_PA_Q}.$$

All denominators use the original factor areas. Enumerating each individual closure polytope to solve a linear response does not enumerate its Cartesian product with the other closure polytope.

## Decisive controls and strongest-incumbent comparison

The first control was the already retained `random_5x5_s1_64` trap; it was evaluated before the conditional 20-control diagnostic. The positive-shadow, A-first variant chooses supports A=$[0,1,4]$, B=$[0,2,4]$. Its exact rational capacity equals the old retained feasible challenger exactly. The old coordinate-fixed capacity divided by this new bound is greater than $1.044275$; the new bound is **4.239786% lower**. This proves escape from that particular trap and does not establish true-capacity equality.

The strongest pre-existing comparator is

$$C_{\rm incumbent}=\min(C_2,C_{\rm triangle},C_{\rm contact},C_{\rm prior\ response}).$$

Contact already dominates width. [comparison.json](comparison.json) compares the new standalone rule with that full minimum. Gains against the prior response alone are not counted if an old triangle bound already supplied them.

| Principal control (prefix `random_`) | Strongest old numerical capacity gap | New numerical capacity gap |
|---|---:|---:|
| `5x5_s2_104` | 2.33% | 0.00% |
| `5x5_s0_92` | 0.00% | 0.00% |
| `5x5_s1_221` | 8.05% | 0.00% |
| `5x6_s3_168` | 1.55% | 0.00% |
| `5x6_s0_25` | 0.99% | 0.00% |
| `5x6_s1_100` | 0.00% | 0.00% |
| `6x6_s1_114` | 0.00% | **2.61%** |
| `6x6_s2_187` | 0.23% | 0.00% |
| `6x6_s0_219` | 0.00% | 0.00% |

These gaps divide exact feasible capacity bounds by inherited numerical labels; displayed zeros are numerical agreement at $10^{-10}$ relative tolerance. No new original-body capacity computation was run. The 20 bodies, their historical labels and the nine principal top-three memberships were already exposed; this is retrospective evidence, not a held-out prediction experiment.

The standalone rule is weaker than existing exact feasible witnesses on `random_5x6_s2_150` by **2.829913%** and on principal `random_6x6_s1_114` by **2.611189%**. These are exact obstructions to universal exactness of this frozen construction, independent of the numerical target. On `random_6x6_s3_124`, its historical numerical miss is **2.935016%**, despite improving the strongest incumbent by 2.9244%. All 20 new exact ratio bounds are below one; the largest is about 0.898773. This supplies no previously unknown exclusion, because prior feasible bounds already excluded these bodies.

The separate regular-pentagon angles $0,\pi/20,\pi/10$ give numerical capacity ratios new/known equal to one within $3\times10^{-16}$. Unlike the prior contact-seeded method, recovery here is by the new sparse variants themselves.

There is also an analytic reason for this recovery. A regular pentagon has exactly five normalized closure vertices, one orbit under its 72-degree rotations. For a fixed shadow sign, $F_s(\alpha)=\max_\gamma M(U_\alpha,sV_\gamma)$ is constant on those five vertices: rotating both factors preserves mixed area and their closure sets, for every relative factor angle. The canonical partner closure is rotation invariant, so the first linear response ties on all five vertices; the frozen rule chooses an actual vertex. The other-factor response then gives the global maximum for that sign. Taking both signs and applying sparse reduction proves exact recovery throughout the already known regular-pentagon rotation family. If one factor is a triangle, its closure is unique, and optimizing the other factor at both signs similarly gives exact recovery. These are exactness explanations for this selector, not new capacity values or new regular-family cases. They do not prove sharpness on arbitrary factors.

## Evidence, reproduction and limits

[results.json](results.json) retains the exact rational geometries, all four variants, canonical and final weights, actual full words, first and second LP coefficients, feasible dual planes and their slacks, contact containment radii and exact objectives. [first-controls.json](first-controls.json) records the first-stage control outcome. [analyze.py](analyze.py) imports only existing closure/word/arithmetic helpers; it invokes neither an original-body capacity producer nor an all-support-pair maximizer. [compare.py](compare.py) is a read-only comparator.

The source is the registered retained `random-product.jsonl`, SHA256 `66bf82010e92e0f26b0df226f4e6c0eef05d21eb22a0967c7f669530f6545736`. It is read locally from its existing registered cache; no data producer, download or installation was used. Scripts inspect only the named control geometries after verifying the whole source identity. Retained historical numerical capacity/volume caveats remain in the empirical source owners; the new witness feasibility and factor areas use exact rational source geometry.

From the patterns checkout, set `SOURCE` to that registered local file:

```sh
mkdir -p /tmp/msc-math/full-partner-replay
python3 -B patterns/upper-tail/full-partner-containment/analyze.py \
  --source "$SOURCE" --out /tmp/msc-math/full-partner-replay/results.json --twenty
python3 -B patterns/upper-tail/full-partner-containment/compare.py \
  --run /tmp/msc-math/full-partner-replay/results.json \
  --prior-comparison patterns/upper-tail/signed-prefix-response/incumbent-comparison.json \
  --out /tmp/msc-math/full-partner-replay/comparison.json
```

Focused sources inspected: repository/research entry points, empirical and product topic accounts, the six-facet reduction, the stopped minimum-area/fixed-closure/contact rules, prior signed-prefix responses and the two-shadow theorem. These reads do not establish exhaustive repository or literature novelty. The proof, construction and finite witnesses stay in the patterns research branch; no selected manuscript source was edited. Root owns integration and separate thesis-worktree coordination.

[review: /root/intake_review]
