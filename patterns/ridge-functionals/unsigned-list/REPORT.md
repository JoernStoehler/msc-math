# Complete unsigned ridge areas and widths do not determine capacity

Two explicit Lagrangian products of a hexagon and a triangle have the same
volume, the same complete multiset of normalized unsigned symplectic two-face
areas, and the same factor difference bodies, but their systolic ratios differ
by **12.56%**. Thus no universal exact formula can recover capacity or systolic
ratio from those data alone. Any common point prediction of their systolic
ratios incurs at least **5.9% target-relative error** on one of the two bodies.

This prunes a whole class of ridge functionals: sums, moments, order statistics,
and arbitrary weights determined only by unsigned ridge values. It leaves
weights using incidence, signs or other geometry open. Both ratios here are
below one; this is a controlled information obstruction, not a held-out
upper-tail test, a failure of all approximate predictors, or a maximality result.

## The construction and the lost information

Put

$$
\begin{aligned}
 A&=\operatorname{conv}\{(-2,-1),(2,-1),(0,3)\},\\
 B&=\operatorname{conv}\{(-1,-2),(3,0),(-1,2)\},\\
 Q&=\operatorname{conv}\{(-1,-1),(4,-1),(-1,2)\},\\
 P_+&=A+B,\qquad P_-=A-B,\qquad K_\pm=P_\pm\times_LQ.
\end{aligned}
$$

Here $A-B$ means the Minkowski sum $A+(-B)$. Their counterclockwise vertices are

| Factor | Vertices |
|---|---|
| $P_+$ | $(-3,-3),(1,-3),(5,-1),(3,3),(-1,5),(-3,1)$ |
| $P_-$ | $(-5,-1),(-1,-3),(3,-3),(3,1),(1,5),(-3,3)$ |

Reflection in the $y$-axis fixes $A$ and sends $B$ to $-B$, so it sends $P_+$
to $P_-$. Both have area $44$; $Q$ has area $15/2$, giving common four-volume
$V=330$. The polygons are full dimensional, have genuine listed edges, and
contain the origin strictly inside.

The summand edge directions have no parallel overlaps. Minkowski addition
merges their counterclockwise edge lists. The edges of $P_+$ therefore consist
of the edges of $A$ and $B$, while those of $P_-$ consist of the edges of $A$
and the negatives of the edges of $B$, in a different cyclic order. A mixed
face $e\times f$ has unsigned symplectic area $|e\cdot f|$ in the repository's
$(q_1,q_2,p_1,p_2)$ convention. Negating a summand edge leaves every such area
unchanged. The 18 mixed-face areas, including two zeros, are exactly

$$
0,0,2,6,6,10,10,12,12,12,12,14,20,20,20,20,22,26.
$$

The nine remaining two-faces are $P\times\{v\}$ or $\{v\}\times Q$, with
symplectic area zero. Thus the **complete 27-entry lists match**, with eleven
zeros each. Dividing every entry by $\sqrt{330}$ gives identical normalized
lists. The signed mixed-area lists differ under the fixed counterclockwise
edge conventions; the unsigned lists lose that information and cyclic order.

There is a second equality:

$$
 P_+-P_+=(A-A)+(B-B)=P_--P_-.
$$

Consequently all directional widths and the complete mathematical two-bounce
capacity competitor also agree, by the [difference-body width formula](../../../topics/products/product-two-bounce-class.tex).
The capacity difference therefore involves asymmetric information absent from
both the width competitor and the unsigned area list. This does not classify
which geometric weights retain enough information in general.

## Exact capacities and the prediction obstruction

The [accepted product reduction](../../../topics/products/product-qp-six-facet-reduction.tex)
provides a global HK maximizer supported on at most three normals per factor,
with total weight $1/2$ in each factor. For these inputs, the hexagon normalized
closure polytopes have eight vertices each, all supported on three normals;
the triangle has one. All two-normal supports are also checked and none is
feasible. Nonzero normals exclude singletons.

For each closure pair, all 120 cyclic word classes, including both
orientations, are evaluated with rational arithmetic. Hence the eight pairs
require 960 words per product. This is complete enumeration for these two
bodies, not just a positive feasible certificate. The convention is
$Q=\sum_{i<j}\beta_i\beta_j\omega(a_i,a_j)$ and $c=1/(2Q_{\max})$.

| Product | $Q_{\max}$ | Capacity | $\operatorname{sys}=c^2/(2V)$ |
|---|---:|---:|---:|
| $K_+$ | $31/1200$ | $600/31$ | $6000/10571\approx0.56759$ |
| $K_-$ | $37/1350$ | $675/37$ | $30375/60236\approx0.50427$ |

The exact capacity ratio is $296/279$, and the systolic-ratio quotient is
$87616/77841$. For a common feature-only prediction $z$ and the two displayed
ratios $s_-<s_+$,

$$
\max\left\{\frac{|z-s_-|}{s_-},\frac{|z-s_+|}{s_+}\right\}
\geq\frac{s_+-s_-}{s_++s_-}=\frac{9775}{165457}\approx5.91\%.
$$

The bound is attained by the harmonic mean $z=2s_-s_+/(s_-+s_+)$. It also
holds if the predictor receives the common volume or factor difference bodies.
A valid upper bound from these data can still exist. This pair does not show
failure within arbitrary-pentagon products, within a specified upper tail,
or for geometry-aware weights. No external novelty or human acceptance is claimed.

## Why this task was selected

The [pre-execution plan](PLAN.md) and [independent comparison](OPTIONS.md)
compare three distinct routes: this full-list obstruction, explaining the
stopped selector through angular compatibility, and a parked global
fixed-pentagon cone problem. A bulk-fit score could reward an uninformative
regression; rejection counts reward easy cases; a slightly tighter bound
rewards numerical polishing. The selected task tests whether the proposed
information can suffice at all, with a complete, inexpensive exact check.
Its direct value is method pruning, consistent with Jörn's lower rating for
counterexamples than for a working upper-tail predictor.

The first prescribed seed separated the ratios materially, so no further
variant was searched. The existing minimum-area selector stays stopped; it was
not tuned or rerun. The five pending human judgments were not dependencies.

## Independent verification and reproduction

[discover.py](discover.py) constructs the hulls, normals, closure vertices and
all cyclic words. [independent_verify.py](independent_verify.py) imports none
of it: it reconstructs supporting inequalities, checks every edge and area,
compares exact difference-body hulls and all 27 areas, and uses rational RREF
to enumerate every two-/three-support closure vertex before direct
four-dimensional HK sums. Both return the same capacities and counts. The
[mathematical review](mathematical-review.md) independently checks the construction,
completeness, conventions, face count and error bound. This is finite exact
computation using the accepted HK/product theorem, not proof-assistant verification.

From the patterns checkout, standard library only:

```sh
python3 patterns/ridge-functionals/unsigned-list/discover.py \
  --out /tmp/msc-math/unsigned-ridge/replay
python3 patterns/ridge-functionals/unsigned-list/independent_verify.py \
  --input /tmp/msc-math/unsigned-ridge/replay/geometry.json \
  --out /tmp/msc-math/unsigned-ridge/replay/independent-verification.json
```

Both scripts write only the explicit output paths. [result.json](result.json),
[geometry.json](geometry.json) and [independent-verification.json](independent-verification.json)
retain exact inputs, maxima and witnesses. [manifest.json](manifest.json) binds
the selected packet. The retained discovery replay took about 0.04 seconds;
account quota and billed spending were not measured. No historical labels,
capacity producer, empirical sample, optimizer, network or new dependency was
used. Inspected sources are the linked product proofs and targeted pattern
packets, not an exhaustive repository or literature survey.

This result remains on the separate pattern branch, outside the selected
105-page manuscript. Root owns integration, separate branch commits and cleanup.

[review: /root/message_reviewer]
