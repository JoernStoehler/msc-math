# Mixed-area closure response finds new competitors, but has an exact coordinate trap

17 October 2026. Two geometric angular orders and one factor-by-factor closure
response produce material finite improvements over the **existing minimum of
width, minimum-area triangles and width-contact**. The new frozen bound is lower
on **6/20** retrospective controls, including **3/9** principal upper-tail
controls. One exact capacity bound decreases **13.40%**. A signed mixed-area
identity explains the mechanism: changing the other factor's prefix geometry can
make a different closure support preferable.

This is useful partial insight, not a successful uniformly tight tail predictor.
A retained exact competitor still proves a **4.4275% capacity miss** at a
coordinatewise stationary point, so repeating the same updates cannot repair it; the largest principal-tail miss against inherited numerical labels is
**8.1482%**. Both angular variants recover the known numerical HKO reference,
using the inherited contact seed. No unrestricted maximality, exact true-capacity
formula, prospective holdout success or external novelty is claimed.

## End state, comparison and frozen rule

The intended end state was a small target-free geometric mechanism explaining
random upper tails while preserving the known above-one reference. Comparing
mechanisms, freezing controls before evaluation, checking actual upper-tail
bodies, and independently verifying feasibility cannot be skipped: each prevents
an easy reference or bulk score from masquerading as that end state.
[PLAN.md](PLAN.md) records the pre-execution choice.

| Option | Potential useful result | What a misleading rubric rewards | Decision |
|---|---|---|---|
| Maximum weighted-pairing L1 surrogate over closure pairs | Cheap signed compatibility score | Incompatible pair signs; an all-pair surrogate search shadows the existing capacity algorithm | Not selected |
| One geometric-word closure response | Other-factor prefix geometry changes support, without searching support pairs | HKO inheritance or tiny bound cuts can hide tail failures | Selected and frozen below |
| More angular harmonic moments | Captures relative factor angle | Repeats the existing fifth-harmonic regular control without a random-tail upper constraint | Not selected |

An initial extension inserting inactive normals immediately after same-factor
seed labels was rejected **before execution**. It preserves alternating block
count. A four-block word attaining width therefore remains in the two-bounce
class, whose global maximum already equals width; any number of closure updates
on that word cannot improve it. This exact block trap explains why merely adding
zero weights to a grouped seed is insufficient. An opposite-gap insertion was
also considered before execution, then rejected in favor of the geometric order.

For factor facet covectors $a_i,b_j\in\mathbb R^2$, let
$J(x,y)=(-y,x)$. The selected two full words circularly sort the rays

$$\{a_i\}\cup\{Jb_j\},\qquad \{a_i\}\cup\{-Jb_j\}.$$

The rational controls use an exact half-plane/cross-product comparator; coincident
rays use input label order (A before B). Lift the unchanged width-contact
selector's closure weights into each word, assigning zero to inactive facets.
Reverse the entire word iff its initial actual HK objective is negative; zero
keeps the counterclockwise word. At that fixed word, optimize the normalized A
closure once, then the normalized B closure once. Strict exact ties retain the
current closure. Factor masses remain $1/2$. Return the smaller capacity bound
among these two responses and the original contact bound. No sweep repetition,
word revision, fitting, support-pair maximization or later variant is used.

Each response LP has three equality constraints, and all its vertices have at
most three active normals. Enumerating each factor's vertices separately is the
existing sparse-reduction lemma applied to **one** fixed other closure; no
Cartesian product of closure supports is optimized. These controls have 3–8
vertices per factor. The method is an HK feasible-bound component, not an
independent computation of exact original-body capacity.

## Why the angular order has geometric content

Let $\alpha,\gamma$ be normalized nonnegative closures:
$\sum\alpha_i=\sum\gamma_j=1$ and
$\sum\alpha_i a_i=\sum\gamma_j b_j=0$. Their cyclic edge lists
$\alpha_i a_i$ and $\gamma_j Jb_j$ define convex closed polygons $U,V$,
possibly degenerate. The positive shadow word merges the edge lists of their
Minkowski sum. Shoelace gives

$$\operatorname{area}(U+V)-\operatorname{area}(U)-\operatorname{area}(V)
 =\frac12\sum_{i,j}\varepsilon_{ij}\alpha_i\gamma_j a_i\cdot b_j,$$

where $\varepsilon_{ij}=1$ when A precedes B and $-1$ otherwise. Thus the
**actual four-dimensional** HK objective, with each factor mass $1/2$, is

$$Q_+=\frac14\sum_{i,j}\varepsilon_{ij}\alpha_i\gamma_j a_i\cdot b_j
 =M(U,V),$$

with $M(U,V)=[\operatorname{area}(U+V)-\operatorname{area}(U)-
\operatorname{area}(V)]/2$. Same-factor shadow determinants cancel in this area
difference and never enter the HK objective. The negative shadow has objective
$-M(U,-V)$; reversing its word makes it $M(U,-V)$. This is a geometric identity
for these words, not a claim that the words are globally optimal.

At fixed B, the A coefficient is

$$d_i=\frac14a_i\cdot\left(\sum_{B_j\text{ after }A_i}\gamma_j b_j
 -\sum_{B_j\text{ before }A_i}\gamma_j b_j\right).$$

The A response maximizes $\sum_i\alpha_i d_i$ over its closure polytope; the B
response is analogous. Signed prefix vectors, rather than an independent factor
area score or an unsigned ridge list, determine which support pays. Each update
preserves feasibility and cannot decrease that word's $Q$. Retaining contact
therefore guarantees $c\le C_{\rm new}\le C_{\rm contact}\le C_2$ in exact
arithmetic. This follows from feasible HK weights and the existing capacity
formula, not from historical labels.

## What actually improved over the incumbent

The 20 controls are the previous twelve contact stress cases plus the three
largest retained numerical ratios in each 5×5, 5×6 and 6×6 bucket, deduplicated.
Ties use source name. These labels and bodies were already exposed: this is a
**retrospective** diagnostic, not a prospective holdout. The comparison below
uses the strongest pre-existing frozen bound
$C_{\rm old}=\min(C_2,C_{\rm triangle},C_{\rm contact})$.
Since contact already dominates width, the exact comparison needs only the last
two. [incumbent-comparison.json](incumbent-comparison.json) retains all 20.

| Principal control (prefix `random_`) | Old capacity gap | New capacity gap |
|---|---:|---:|
| `5x5_s2_104` | 2.33% | 8.15% |
| `5x5_s0_92` | 0.00% | 0.00% |
| `5x5_s1_221` | 8.05% | 8.05% |
| `5x6_s3_168` | 2.59% | **1.55%** |
| `5x6_s0_25` | 0.99% | 1.14% |
| `5x6_s1_100` | 0.00% | 1.37% |
| `6x6_s1_114` | 0.00% | 0.00% |
| `6x6_s2_187` | 4.11% | **0.23%** |
| `6x6_s0_219` | 3.28% | **0.00%** |

Gaps divide exact feasible capacity bounds by inherited numerical capacity
labels. A displayed zero denotes numerical agreement, not a certificate of true
capacity equality. New exceeds old on some controls because the frozen rule
retains contact but does not incorporate the separate triangle rule; the old
minimum is a comparator, not a post-test modification of the new rule.

The strongest new finite competitor is `random_5x5_s3_114`. Its old and new
A supports are both $[0,3,4]$, while B changes from $[0,2,3]$ to $[0,2,4]$.
The negative-shadow word starts at $Q=0.07051963737\ldots$; A stays fixed, then
the B response gives $Q=0.08143098715\ldots$. Exact rational values prove a
**13.3995% capacity-bound reduction against the incumbent**, independent of the
historical target. Its numerical capacity gap falls from 15.47% to zero. On the
principal `random_6x6_s0_219`, A changes $[0,3,4]\to[0,3,5]$ while B stays
$[0,3,5]$, yielding a **3.1742%** exact reduction against the old minimum.
These are support changes explained by signed mixed-area response, not numerical
polishing of fixed coefficients.

Contact alone is improved on 11/20 controls and 5/9 principal controls. Those
larger counts are **not** the incumbent comparison: several recover competitors
already supplied by minimum-area triangles. In particular,
`random_6x6_s1_194` drops from contact ratio bound 1.164606 to 0.848538, but the
old triangle bound was already stronger. All 20 new ratio bounds are exactly
below one (largest 0.986906); this supplies no previously unknown body exclusion.

## HKO and the exact remaining obstruction

The separate regular-pentagon controls at $0,\pi/20,\pi/10$ recover their known
capacity profile numerically. At HKO, both shadow variants themselves give ratio
$1.047213595499958\ldots$, matching $(3+\sqrt5)/5$; no response update is needed.
The contact seed already carries the correct support and value. This verifies
that the new geometric order preserves the positive reference, not that the
response mechanism newly discovers its excess or predicts random upper tails.
The floating reference uses tolerance $10^{-11}$, unlike exact rational controls.

For `random_5x5_s1_64`, the new rule improves the old bound by 4.9835%, yet the
[already retained exact competing word](../geometric-selector/falsifier.json)
still gives

$$\frac{C_{\rm new}}{c}\geq
\frac{C_{\rm new}}{C_{\rm competitor}}
 =1.044275027674890\ldots>1.044275.$$

This is a certified **remaining** miss, independent of inherited labels and
without a fresh support search. The competitor remains a verifier input, never
an added support for the frozen rule. Its bound need not equal true capacity.

This is an exact **nonglobal coordinate fixed point**, rather than merely a
consequence of allowing only one sweep. For both shadow variants on this body,
the final B closure equals the original seed B closure. The exact LP checks
therefore show that the final A closure is optimal against the final B closure,
and the final B closure is optimal against the final A closure. Repeating the
same one-factor responses with strict tie retention changes neither pair. Yet
placing the retained competitor's two closures in the positive shadow order
gives exactly its larger retained objective, above both fixed-point objectives
by the displayed factor relative to the selected one. Thus even the same
positive-shadow mixed-area problem has a better feasible pair requiring a
joint support change. This prunes repeated unchanged closure sweeps from these
seeds; it does not rule out other initializations or geometric selectors.

**Disposition:** retain the signed mixed-area mechanism and its material finite
competitors. This frozen one-sweep rule has not supplied a uniformly tight
upper-tail explanation or a global-maximality conjecture. Stop the bounded test
here; no further sweep, word selection, scan or manuscript inclusion is selected.

## Evidence, scope and reproduction

[results.json](results.json) retains both full words, normalized closures,
seed/after-A/final objectives, response coefficients, final witness, exact
volume and ratio for each body. [lp-verification.json](lp-verification.json)
checks every factor LP vertex using independent generic rational Gaussian
elimination, including opposite pairs, coefficient reconstruction and strict
tie retention. [geometry-verification.json](geometry-verification.json) records
the separate root check: 161 direct four-dimensional objective evaluations,
120 exact mixed-area stage identities, exact volumes and the reused competitor
and coordinate trap. [review.md](review.md) records independent scientific,
angle/control accounting and exact-text review. These checks verify feasible
bounds and this computation, not true-capacity optimality.

Source geometry SHA-256 is
`66bf82010e92e0f26b0df226f4e6c0eef05d21eb22a0967c7f669530f6545736`;
width archive SHA-256 is
`dcffead16ab5ad0fcdf6ae55d78375bea5b6ee7274877a33846cf4c3c2a7d57f`.
Input coordinates are exact dyadic representations of historical binary64
bodies. Capacity/ratio labels remain historical numerical values. The inherited
[tail diagnostic](../FROZEN-TAIL.md), [contact method](../geometric-selector/REPORT.md),
[product sparse reduction](../../../topics/products/product-qp-six-facet-reduction.tex),
[two-bounce theorem](../../../topics/products/product-two-bounce-class.tex) and
[current dispositions](../../OWNER-FOLLOWUP.md) supply the source context.
These targeted reads are not an exhaustive literature or repository survey.

From repository root, set `SOURCE` to the registered `random-product.jsonl`:

```sh
mkdir -p /tmp/msc-math/signed-prefix-replay
cp patterns/upper-tail/signed-prefix-response/PLAN.md /tmp/msc-math/signed-prefix-replay/
PYTHONDONTWRITEBYTECODE=1 python3 patterns/upper-tail/signed-prefix-response/analyze.py \
  --source "$SOURCE" --width patterns/product-shape/features.csv.gz \
  --out /tmp/msc-math/signed-prefix-replay
python3 patterns/upper-tail/signed-prefix-response/verify_lp.py \
  --source "$SOURCE" --run /tmp/msc-math/signed-prefix-replay
python3 patterns/upper-tail/signed-prefix-response/compare_incumbent.py \
  --run /tmp/msc-math/signed-prefix-replay \
  --tail-rows patterns/upper-tail/tail-rows.jsonl.gz \
  --falsifier patterns/upper-tail/geometric-selector/falsifier.json
python3 patterns/upper-tail/signed-prefix-response/verify_geometry.py \
  --source "$SOURCE" --results /tmp/msc-math/signed-prefix-replay/results.json \
  --challenger patterns/upper-tail/geometric-selector/falsifier.json \
  --out /tmp/msc-math/signed-prefix-replay/geometry-verification.json
```

The feature run took 1.24 seconds and LP verification 0.60 seconds on this
workstation; these are measured components, not a total-spend estimate. Only the
standard library and retained inputs were used. New files began in
`/tmp/msc-math/patterns-require-both/20261017T1336Z` and were deliberately retained.
No original-body capacity producer, dependency installation, network/cloud work,
shape optimization, thesis edit or external action was performed. The standing
owner owns integration across the separate branches; this packet belongs to the
patterns branch.

[review: /root/intake_triage]
