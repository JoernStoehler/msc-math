# Two angular words exactly optimize a planar closure-vertex pair

17 October 2026. **For every fixed pair of normalized closure vertices of planar polygon factors, the maximum Haim–Kislev objective over all cyclic words is the larger of two mixed areas.** The two corresponding words are obtained by sorting the factor normals together with the positive or negative quarter-turn of the other factor's normals, then reversing the negative-shadow word. This replaces the source corollary's at most 120 cyclic-word evaluations per support pair by two exact geometric evaluations. It supplies a geometric formula for the existing capacity enumeration, not a new method for choosing its optimal support pair, a held-out ratio predictor, or a global maximality theorem.

## Definitions and theorem

Let $a_i,b_j\in\mathbb R^2$ be the dual facet vertices of two full-dimensional planar polygons containing the origin in their interiors. A normalized factor closure satisfies

$$\alpha_i,\gamma_j\geq0,\qquad \sum_i\alpha_i=\sum_j\gamma_j=1,
\qquad \sum_i\alpha_i a_i=\sum_j\gamma_j b_j=0.$$

Delete zero-weight entries and set $x_i=\alpha_i a_i$, $y_j=\gamma_j b_j$, $J(u,v)=(-v,u)$. All retained facet normals and weighted steps are nonzero. For a word $w$ on these distinct labels, give each factor total mass $1/2$, so

$$Q_w=\frac14\sum_{i,j}\varepsilon_{ij}(w)x_i\cdot y_j,
\qquad \varepsilon_{ij}=\begin{cases}1&A_i\text{ precedes }B_j,\\-1&\text{otherwise.}\end{cases}$$

Let $w_+$ be the counterclockwise angular merge of $\{x_i\}$ and $\{Jy_j\}$, and $w_-$ the counterclockwise merge of $\{x_i\}$ and $\{-Jy_j\}$. Any fixed rule may break equal-ray ties. Positive scaling does not affect angular order, so the words can equivalently sort the unweighted active normals.

**Theorem.** If the active A closure has at most three entries, while the active B closure is arbitrary, then

$$\boxed{\max_w Q_w=\max\{Q_{w_+},-Q_{w_-}\}.}$$

Both values in braces are nonnegative. The negative sign is realized by reversing $w_-$. No general-position or strict cross-factor pairing assumption is needed. By exchanging the factors, the statement also holds if only B has at most three entries.

## Proof by independent gap placement

Fix a cyclic A order and take a linear representative starting at an A label. Its prefix sums are $p_0=0,p_1=x_0,\ldots,p_{r-1}=x_0+\cdots+x_{r-2}$; the final sum is zero. If a B label with step $y_j$ is inserted in the gap with A-prefix $p$, closure gives

$$\frac14\left(\sum_{A_i\text{ before }B_j}x_i-\sum_{A_i\text{ after }B_j}x_i\right)\cdot y_j
=\frac12p\cdot y_j.$$

Same-factor symplectic pairings vanish. Consequently each B label may independently choose any A gap, with no restriction from the placement of other B labels. Writing $U=\operatorname{conv}\{p_0,\ldots,p_{r-1}\}$ and $h_U(v)=\max_{p\in U}p\cdot v$, the exact maximum among words with this cyclic A order is

$$\frac12\sum_j h_U(y_j).\tag{1}$$

Translation of $U$ changes this sum by $\frac12t\cdot\sum_j y_j=0$. Choosing a translate containing zero proves nonnegativity.

Sort the A steps counterclockwise. Because their sum is zero, their closed edge path is a convex triangle or a degenerate segment when there are at most three steps. Inserting $Jy_j$ in that angular order places B at a prefix maximizing $p\cdot y_j$: along the edge cycle, the increments $x_i\cdot y_j$ switch from positive to negative at the angle of $Jy_j$. The positive and negative runs are contiguous around the circle; the vertex between them is a support vertex in direction $y_j$. When a step is parallel to $Jy_j$, its dot product with $y_j$ is zero and every tied placement attains the same value. Thus $w_+$ attains (1).

Similarly, inserting $-Jy_j$ places B at a minimizing prefix, and

$$Q_{w_-}=-\frac12\sum_j h_U(-y_j).\tag{2}$$

For at most three A labels there are at most two cyclic orders. They are the sorted order and its reverse; cyclic cuts do not change $Q$ by closure. Reversing a zero-sum edge list negates its prefix set up to translation. Therefore the other cyclic A order has prefix hull $-U$ up to translation and exact placement maximum equal to $-Q_{w_-}$ by (1)–(2). These exhaust all words. With two A labels the two orientations are equivalent and the same argument applies. This proves the theorem. ∎

The independent reviewer noted that the cyclic-order argument already covers collinear three-step or zero-step limits (after discarding zero steps), though a genuine full-rank closure vertex of support three is noncollinear. Thus degeneracy does not require an unproved maximal-area rearrangement principle for arbitrary edge counts. No assertion for two arbitrary large supports follows from this proof.

## Exact mixed-area formula for capacity

Let $U$ be the convex edge polygon whose counterclockwise edges are $x_i$, and let $V$ have counterclockwise edges $Jy_j$; allow degenerate segments. Write

$$M(U,V)=\frac{\operatorname{area}(U+V)-\operatorname{area}(U)-\operatorname{area}(V)}2.$$

The boundary of the Minkowski sum merges the angularly sorted edge lists. Expanding its shoelace sum, the same-factor terms give the two individual areas and the cross terms give the actual HK objective. Precisely,

$$Q_{w_+}=M(U,V),\qquad -Q_{w_-}=M(U,-V).$$

This identity was established for the frozen words in the preceding signed-prefix response packet; the new theorem proves that these words optimize a closure-vertex pair. Combined with the accepted six-facet reduction, it gives the exact formula

$$\boxed{c_{\rm EHZ}(K_q\times_LK_p)
=\frac{1}{2\displaystyle\max_{\alpha\in\operatorname{vert}C_q,\ \gamma\in\operatorname{vert}C_p}
\max\{M(U_\alpha,V_\gamma),M(U_\alpha,-V_\gamma)\}}.}$$

The imported reduction ensures both closures have at most three entries, total factor masses $1/2$, and that a global maximizer is represented. The target maximum is positive for full-dimensional factors. This is a scalar capacity statement: it recovers at least one maximizing word, not every maximizing orbit, its physical placement, or production-code behavior. The code currently used by the thesis is not changed by this packet.

There may be fewer than 120 distinct necessary word evaluations in implementations already exploiting blocks, ties or symmetries; **120→2 describes the source corollary's generic candidate list, not a measured speedup**. All support pairs remain in the maximum. That is the existing exact capacity problem, rather than a newly cheap global ratio predictor.

## Why the coordinate trap survives

The earlier signed-prefix method optimized each factor in turn at two fixed full angular words, starting from one chosen support pair. This theorem says that order can already be optimal while the support pair is not globally optimal. A targeted exact check of both shadow values on each of the two final pairs in the retained `random_5x5_s1_64` control confirms that each final pair already attains its full word maximum; [check_trap.py](check_trap.py) and [coordinate-trap-order.json](coordinate-trap-order.json) retain that check. The retained exact nonglobal coordinate fixed point is therefore a support-choice obstruction; exhaustive order search on the same two closures cannot repair it. A better pair still requires a justified support rule or an actual global support-pair optimization. No new response sweep or scan is selected here.

## Verification and inspected sources

[check.py](check.py) independently generates 49 rational closed-step controls, including 3×3, 2×3, 3×2, 2×2, 3×4 and 2×5 closures and parallel/tied directions. It checks **all 9,186 cyclic words** on those cases, the support-prefix maxima, both angular objectives and mixed-area identities, and reconstructs a maximizing objective directly in four dimensions using nonuniform normalized weights. [result.json](result.json) retains every exact input and result. These finite checks validate conventions and degeneracy controls; the proof establishes the universal statement.

A separate read-only agent independently reconstructed and challenged the support-gap proof, including sign, degeneracy and equal-ray ties. Its retained assessment is in [review.md](review.md). No original-body capacity producer, population scan, numerical optimizer, installation, network or cloud work was used for this theorem. For the supplementary exact trap check, one retained control geometry was used from the locally verified registered dataset. The checker hashes the whole source and locates the named row; it performs no population-level analysis or search for new controls.

From the patterns checkout:

```sh
mkdir -p /tmp/msc-math/mixed-area-order/replay
python3 -B patterns/upper-tail/mixed-area-order/check.py \
  --out /tmp/msc-math/mixed-area-order/replay/result.json
```

The result is deterministic, including a SHA256 binding to the checker. [PLAN.md](PLAN.md) records the end state, alternatives, rubric criticisms and bounds; preliminary probes preceded selection and were not promoted as proof.

Inspected local sources: `topics/products/README.md`, `product-qp-six-facet-reduction.tex`, `product-triangle-bounce-classification.tex`, `pentagon-affine-products/research.tex`, `patterns/upper-tail/ROUTE-TRIAGE.md`, and `signed-prefix-response/REPORT.md` plus its implementation. The six-facet corollary explicitly enumerates up to 120 cyclic orders; the affine note gives a two/four/six-block decomposition and twelve alternating orders. The two-shadow sufficiency theorem is not stated in those inspected sources. This is a local source comparison, not an exhaustive literature or repository novelty claim. The result is retained as research evidence outside the selected manuscript pending scientific and thesis selection.

[review: /root/shadow_order_proof]
