# Fixing cone-volume weights loses tightness at HKO

17 October 2026. A distinct signed-ridge candidate is not sharp at the known
HKO reference, as proved before any data scan: fixing the cone-volume closure weights and
optimizing only facet order gives a capacity upper bound at least about 4.95%
above the known HKO capacity. The induced ratio upper bound is at least about
10.15% above HKO's ratio. This is a scoped obstruction to that standalone rule,
not to all signed-ridge predictors. No corpus scan, fitting or capacity producer
was run, and no external novelty or manuscript inclusion is claimed.

## Success story and selection

A useful outcome would have been a natural signed-ridge functional exposing the
short competitor missed by width, without optimizing closure supports or KKT
systems. First checking the known positive reference cannot be skipped: a corpus
fit could obscure information already lost by fixing weights. The rule retains
signed incidence and relative geometry, unlike the stopped unsigned summaries.

| Option | Possible material outcome | What a misleading rubric rewards | Choice |
|---|---|---|---|
| Fixed cone-volume weights plus best signed ordering | A compact geometric closure supplies the missing upper constraint | Hiding a control failure by taking minimum with width; calling an HK component an independent exact formula | Analytic reference fails below; stop this standalone rule before a scan |
| Jointly selected closure triangles | A geometric support rule explains the missing competitor | Enumerating or target-tuning supports reproduces the existing capacity algorithm | Existing [route comparison](../ROUTE-TRIAGE.md) found no new selector |
| Improve the inscribed pentagon core | A large near-HKO core supplies a useful lower bracket | Exact HKO recovery hides poor random-tail core coverage | [Frozen test](../geometry-aware-probe/REPORT.md) already failed; no tuning selected |
| General-matrix cone positivity | A certificate changes the sharp theorem's domain | Treating the existing inequalities as solved quadratic positivity | Remains parked; no new certificate method supplied |

Bulk fit, threshold counts and small gap changes reward easy cases or numerical
polishing. Exact reference recovery, information retained, proof-domain change
and bounded cost are more useful here. The conclusion is **do not launch a tail
scan for this standalone fixed-weight rule**. Taking its minimum with width is
not ruled out: width is already exact at HKO and would conceal this obstruction.
No positive mechanism for that combination was established in this pass. The
selected aim is a short upper constraint sharp enough to motivate HKO maximality.
This does not test whether a looser approximation is empirically useful on random
products, and no universal approximation tolerance is inferred from this result.

## Candidate and support

For a planar polygon $P$ with interior origin, area $A$, unit outward normals
$n_i$, edge lengths $l_i$ and positive heights $h_i$, set

$$a_i=n_i/h_i,\qquad \gamma_i=l_i h_i/(2A),\qquad
x_i=\gamma_i a_i=l_i n_i/(2A).$$

The area identity and Minkowski closure give $\sum\gamma_i=1$ and
$\sum x_i=0$. Fix these factor weights for $P$ and $Q$, assign each factor mass
$1/2$, and define $Q_{\rm can}$ as the maximum HK objective over full facet
orders at those fixed weights. Then $C_{\rm can}=1/(2Q_{\rm can})$ is a capacity
upper bound. The normalized steps are rotated primal edge vectors divided by
$2A$; translating the chosen origin does not change them. This retains signed
geometry but freezes the closure distribution. Optimizing orders is still an HK
component; it is not a new independent exact capacity algorithm.

The [affine-pentagon proof](../../../topics/products/pentagon-affine-products/research.tex)
owns the HK normalization, sparse reduction, six-block formula, complete
12-order classification and known HKO capacity used below. This note specializes
those identities. It does not replace their proof or extend their theorem domain.

## Exact loss at HKO

Put $h=\cos(\pi/5)$, $s=1/(1+h)$, $\rho=s^2/2$,
$t=\tan(\pi/5)$ and $m=\cos(\pi/10)$.
Let $n_i$ be the regular-pentagon normals at angles $2\pi i/5$.
The five closure vertices have weights $h/(1+h)$ at $r$ and
$1/[2(1+h)]$ at $r+2,r+3$. Their average is exactly the uniform factor
closure $(1/5,\ldots,1/5)$, also the cone-volume closure. Their steps are rotations
of $s e,s a,s b$, with

$$e=(1,0),\qquad a=(-1/2,t/2),\qquad b=(-1/2,-t/2).$$

At HKO relative angle $\theta=\pi/10$, the existing formula gives
$Q_* =\rho m$ and $c_*=(1+h)^2/m$.
For any fixed full order, bilinearity makes its uniform-closure objective the
average of the 25 objectives at pairs of closure vertices, restricted to that
same order. Every restricted objective is at most $Q_*$.

For five of these pairs, the distinguished normals differ by $\pi/2$:
$\theta+2\pi(j-r)/5=\pi/2$ when $j-r=1$ modulo five. Independent rotation to the
representative triples leaves relative matrix $J=R_{\pi/2}$. The complete
six-block classification gives, after dividing by $\rho$, sign/reflection
versions of

$$T_1=ee^T-aa^T,\qquad T_2=eb^T-ae^T,\qquad F=\operatorname{diag}(1,-1).$$

The absolute Frobenius pairings of $T_1,T_1F,FT_1,FT_1F$ with $J$ are zero
or $t/2$; those from $T_2$ are zero or $t$. Four-block words give at most
$\rho t/2$, since a proper grouped sum is a step or its negative and
$\max_{u,v\in\{e,a,b\}}|u\cdot Jv|=t/2$. Two-block words give zero.
Consequently the complete maximum for each of these five triple pairs is
$\rho t$. The [independent exact check](verify.py) also enumerates all $6!=720$
restricted full orders using rational coefficients and obtains exactly
$\max Q/(\rho t)=1$.

Thus, for every full order and hence its maximum,

$$Q_{\rm can}\leq\frac{20\rho m+5\rho t}{25}
 =r Q_*,\qquad r=\frac{4+t/m}{5}=\frac{7-\sqrt5}{5}<1.$$

Here $t/m=3-\sqrt5$. Let $V=\operatorname{area}(P_5)^2$ be the
four-dimensional volume of the HKO product. Inverting the positive objectives
and then squaring yields

$$\frac{C_{\rm can}}{c_*}\geq\frac1r\approx1.049553,
\qquad
\frac{C_{\rm can}^2/(2V)}{\operatorname{sys}_*}
\geq\frac1{r^2}\approx1.101562.$$

In particular the ratio upper bound is at least approximately $1.15357$ at HKO,
whose exact ratio is $(3+\sqrt5)/5\approx1.04721$.
This proves a lower bound on the rule's error; it does not compute its exact
maximum objective or exact capacity bound.

## Verification and limits

Reproduce the independent exact finite check with:

```sh
python3 patterns/upper-tail/fixed-closure/verify.py
```

[Receipt](verification.json): all 720 restricted orders have rational
$Q/(\rho t)$ between $-1$ and $1$, with six positive maximizers; the constant
identity is checked in $\mathbb Q(\sqrt5)$ after squaring positive quantities.
The finite check verifies the restricted objective and algebra, while the
bilinear averaging argument above supplies the full-rule gap.

Source inspection was targeted to current pattern/product/empirical accounts
and proofs in this branch; no exhaustive repository or literature search is
claimed. Independent mathematical review owns the normalization and implication
check. Root retains, integrates and commits this branch separately; no changes
from it are merged into the thesis manuscript.

[review: /root/closure_math_review]
