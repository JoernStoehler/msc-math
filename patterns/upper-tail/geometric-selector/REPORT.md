# Width contact finds some missing competitors, but cannot give an HKO-sharp envelope

17 October 2026. The frozen **width-contact closure selector** uses one selected
triangle pair on each of twelve irregular products. It recovers several short
competitors missed by the stopped minimum-area rule, including its lone
above-one survivor. Two exact checks nevertheless stop this selector:

- A feasible competing word proves its capacity bound is **at least 9.90% above
  true capacity** on `random_5x5_s1_64`, without trusting a historical label.
- On `random_6x6_s1_194`, the selector's exact ratio upper bound is
  **1.1646061944**, exceeding HKO's $(3+\sqrt5)/5\approx1.0472135955$.
  Therefore this particular bound cannot itself certify universal HKO maximality.
  It makes no claim that the body's true ratio exceeds HKO.

This is a small, signed, geometric rule with a real mechanism and an exact
obstruction. It is stopped without adding supports, fitting, a corpus scan or
manuscript inclusion. Approximate utility outside these controls remains open;
no unrestricted theorem or external novelty is claimed.

## Success story and choice

The desired end state was a small target-free closure that explains hard-tail
capacity and might support an HKO-sharp global ratio bound. The necessary steps
were to derive a natural support choice, freeze it before its new label
comparison, check HKO and old hard controls, and verify consequential failures
by exact geometry. Source-only triage and bulk scores cannot settle that end
state. [PLAN.md](PLAN.md) fixes the first stage; [AMENDMENT.md](AMENDMENT.md)
records the two additional width extremes authorized after its result.

| Option | Why it could help | What an easy rubric rewards wrongly | Selection |
|---|---|---|---|
| Independently minimum-area outer triangles | Good individual geometric approximations | Small area ignores relative signed compatibility | Already stopped |
| Largest unweighted normal pairing | Very cheap anchor | Origin and normalization affect the choice | Reject |
| Width-attaining anchor closures, then signed word order | Geometric extremal directions choose a small pair; a three-cycle may improve width | HKO recovery can merely reproduce width; bulk fit hides misses | Frozen candidate below |
| Maximize over every closure pair | Tight capacity | Repackages the existing capacity algorithm | Exclude from selector |

The rubric prioritizes a material upper-tail miss, feasible exact witnesses and
whether the bound itself respects the HKO ceiling. A rejection count alone
would reward easy sub-one examples. The initial seven controls are published
failures of the old rule; three additional names minimize SHA256(name) within
the 5×5, 5×6 and 6×6 buckets. This is a **retrospective stress test**, not a
held-out predictive benchmark. Supports use no capacity labels.

## The geometric mechanism

Let $P=\{q:a_i\cdot q\le1\}$ be a bounded planar polygon with the origin
inside and irredundant facets. For each $i$, put

$$H_i=h_P(-a_i),\qquad \alpha_i=\frac1{1+H_i},
\qquad x_i=\alpha_i a_i.$$

Then $\alpha_i$ is the largest possible coefficient of $a_i$ in a normalized
nonnegative closure $\sum_k\gamma_k a_k=0$. Indeed, at an opposite support
point $v$ with $a_i\cdot v=-H_i$, dotting any closure with $v$ gives
$0\le-\gamma_iH_i+(1-\gamma_i)$, hence $\gamma_i\le\alpha_i$.
Conversely, $-a_i/H_i$ lies on the polar edge dual to that support point.
Its convex combination of the two adjacent normals supplies equality. Thus
one opposite vertex gives a triangle closure containing $a_i$. If the opposite
support is an edge, the closure reduces to two normals; it is retained as such.
No near-maximal replacement is introduced.

The normal directions of $P-P$ are the union of those of $P$ and $-P$.
Since $h_{P-P}(a_i)=1+H_i$, its polar is

$$(P-P)^\circ=\operatorname{conv}\{\pm x_i\}.$$

Duplicates are allowed, and **both signs** are necessary. For the analogous
$y_j$ of $Q$, the existing [two-bounce formula](../../../topics/products/product-two-bounce-class.tex)
gives

$$M=\max_{i,j}|x_i\cdot y_j|,\qquad C_2=1/M.$$

The rule selects only anchor pairs attaining $M$, with **all exact ties**, and
only their opposite-support closures. It then maximizes the HK objective over
cyclic orders of those at most six normals, with half the total mass in each
factor. This is an HK component, not an independent exact-capacity formula.

Its bound $C_{\rm contact}$ always satisfies $c\le C_{\rm contact}\le C_2$.
For example, if the selected weighted anchors obey $x_0\cdot y_0=M>0$,
closure gives $x_1+x_2=-x_0$ and $y_1+y_2=-y_0$. The grouped order
$A_0,B_0,A_1,A_2,B_1,B_2$ has $Q=M/2$, including the factor-mass $1/4$.
A negative anchor pairing is handled by reversing the order. Other orders may
improve this width witness through a signed three-cycle. Taking the minimum
with width is therefore redundant. Translations preserve the weighted steps. Under paired linear symplectic
factor changes the steps transform contragrediently, preserving pairings and
selector output.

## Fixed controls and exact failures

Every irregular control has one exact width-maximizing anchor pair and one
selected closure pair, requiring 120 cyclic-order evaluations. Capacity gaps
below use inherited numerical capacity labels and are descriptive only.

| Control | Contact capacity gap | Gain over width | Contact ratio upper bound |
|---|---:|---:|---:|
| `5x5_s1_64` | 9.90% | 0.00% | 0.92316 |
| `5x6_s2_150` | 0.00% | 11.49% | 0.79466 |
| `6x6_s3_124` | 7.40% | 0.00% | 0.91585 |
| `5x5_s3_114` | 15.47% | 0.00% | 0.60085 |
| `5x6_s3_29` | 0.00% | 18.78% | 0.61717 |
| `6x6_s2_3` | 0.00% | 15.67% | 0.60979 |
| `5x5_s1_221` | 8.05% | 0.03% | 0.93981 |
| `5x5_s0_8` (name-hash) | 0.00% | 6.71% | 0.65146 |
| `5x6_s0_102` (name-hash) | 0.00% | 0.00% | 0.36566 |
| `6x6_s0_194` (name-hash) | 11.98% | 0.00% | 0.91164 |
| `5x6_s2_220` (width extreme) | 16.41% | 26.77% | 0.71302 |
| `6x6_s1_194` (width extreme) | 38.49% | 1.88% | **1.16461** |

All names have prefix `random_`. The last two were selected by descending
retained width ratio among the existing 26 above-one width survivors, with name
ties, **before their contact evaluation**. This second-stage retrospective
amendment keeps the rule fixed; it is not an independent clean test set.

For `random_5x5_s1_64`, the selector keeps supports
$[0,2,3]$ and $[0,1,3]$. A separate falsifier retained supports
$[0,1,4]$ and $[0,2,4]$, with word $[0,3,1,4,2,5]$ and exact positive
closure weights. It stopped after the eighth lexicographic feasible triangle
pair, at the first witness exceeding a predeclared 9% gap; it did not compute
the full capacity optimum or change the selector. The rational packet proves

$$\frac{C_{\rm contact}}{c}\ge
\frac{C_{\rm contact}}{C_{\rm witness}}
=1.099046370702182\ldots.$$

This verification used 960 cyclic-order evaluations, separate from the rule.
For `random_6x6_s1_194`, independent enumeration of the **selected** orders
checks the exact rational output $r=1.1646061943640518\ldots$.
The strict comparison with HKO is rational: $5r-3>0$ and $(5r-3)^2>5$.
Its original numerical ratio is unnecessary for this obstruction to the bound.

Separate regular-pentagon controls at angles $0,\pi/20,\pi/10$ recover the
known capacity formula numerically. HKO returns ratio $1.0472135954999586$.
Regular ties select 5, 5 and 10 pairs respectively (600, 600 and 1,200 orders),
using absolute tolerance $10^{-11}$ in fixed circumradius-one coordinates.
The rational irregular tests use **zero tolerance**. The regular controls use
the known area $5\sin(2\pi/5)/2$; they do not constitute algebraic certificates.

## Evidence and reproduction

[results.json](results.json) retains the ten initial witnesses; the unchanged
selector's two added witnesses are in [stress-results.json](stress-results.json).
[falsifier.json](falsifier.json) contains the separate exact competitor.
[verify.py](verify.py) independently checks geometry, contacts, selected orders
and the two consequential exact inequalities; its scope is recorded in
[verification.json](verification.json). No historical capacity or ratio is
promoted to a certificate.

The original rational-body JSONL SHA-256 is
`66bf82010e92e0f26b0df226f4e6c0eef05d21eb22a0967c7f669530f6545736`;
the width table SHA-256 is
`dcffead16ab5ad0fcdf6ae55d78375bea5b6ee7274877a33846cf4c3c2a7d57f`.
These are the same registered source and retained feature table as the earlier
[tail diagnostic](../FROZEN-TAIL.md). Input coordinates are exact dyadic
representations of historical binary64 bodies. Source inspection was limited
to current pattern reports, the two-bounce and triangle proofs, and the retained
source fields; no exhaustive literature or repository survey is claimed.

Run from repository root, with `SOURCE` pointing to the verified registered
`random-product.jsonl` cache:

```sh
python3 patterns/upper-tail/geometric-selector/analyze.py \
  --source "$SOURCE" --width patterns/product-shape/features.csv.gz \
  --out /tmp/msc-math/geometric-selector/replay
python3 patterns/upper-tail/geometric-selector/stress.py \
  --source "$SOURCE" \
  --names patterns/upper-tail/geometric-selector/additional-names.json \
  --out /tmp/msc-math/geometric-selector/replay/stress-results.json
python3 patterns/upper-tail/geometric-selector/falsify.py \
  --source "$SOURCE" \
  --results /tmp/msc-math/geometric-selector/replay/results.json \
  --out /tmp/msc-math/geometric-selector/replay/falsifier.json
python3 patterns/upper-tail/geometric-selector/verify.py \
  --source "$SOURCE" --directory /tmp/msc-math/geometric-selector/replay
```

The producers use only the standard library and write only their named output
destinations. New code and data began under
`/tmp/msc-math/geometric-selector/2026-10-17-width-contact/` and were deliberately
retained. The initial pass took under a second, the added controls under half a
second, and the separate falsifier under half a second; timings are observations.
No original-body capacity producer, iterative shape optimization, fitting, cloud job or
external action was run. Exact feasible words give capacity upper bounds;
independent selected-order enumeration identifies what this particular rule
returns. Neither establishes a new true-capacity formula or global theorem.

[review: /root/patterns_investigation/contact_review]
