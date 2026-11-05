# Signed ridge spectra miss a continuum of irregular above-one products

The **complete normalized signed ridge spectrum** does not determine the systolic ratio even on a continuum of non-affine-regular pentagon products whose ratios are all strictly above one. Two frozen rational endpoints have exact ratios **1.008227769322672…** and **1.044903535387585…**, with identical signed ridge matrix squares entry by entry. Every prediction using only that spectrum, together with intrinsic factor geometry and volume, has at least **1.7863% relative ratio error** on one endpoint; no such rule can decide the upper-tail threshold $\operatorname{sys}>1.03$. This is an exact upper-tail obstruction, not a fitted predictor, a random-population accuracy statement or an obstruction to unsigned ridge spectra.

## Explicit family and invariant

Let

\[
P=\frac1{450115}\operatorname{conv}\{(-428065,139073),
(-264600,-364142),(264600,-364142),(428065,139073),(0,450138)\}.
\]

For rational or real $t$, write

\[
R(t)=\frac1{1+t^2}
\begin{pmatrix}1-t^2&-2t\\2t&1-t^2\end{pmatrix},
\qquad K_t=P\times_LR(t)P.
\]

The exact checks establish:

1. $P$ is a strictly convex genuine pentagon with the origin in its interior. It is **not an affine image of a regular pentagon**: its consecutive oriented edge determinants are not all equal, whereas every affine regular pentagon has equal such determinants.
2. Every $K_t$ for $1/8\leq t\leq4/25$ has $\operatorname{sys}(K_t)>1$.
3. All these bodies have the same complete normalized signed ridge matrix square and characteristic polynomial, although the endpoint ratios differ as follows.

| Frozen $t=\tan(\theta/2)$ | Exact ratio, decimal presentation | Status |
|---|---:|---|
| $1/8$ | $1.008227769322672\ldots$ | Rational exact HK maximum; above one |
| $4/25$ | $1.044903535387585\ldots$ | Rational exact HK maximum; above one |

All ratio fractions, primal/polar vertices, closure supports and winning words are retained in [sample.json](sample.json) and [results.json](results.json). The relative ratio separation is **3.63765%**, and the relative capacity separation is **1.80258%**. The ratio gap $0.036675766\ldots$ is about 78% of the gap between one and the known HKO ratio. These are substantial upper-tail differences rather than a new record or a tiny bound improvement.

Define the invariant intrinsically as follows. For facets $i,j$ with a common two-face $F_{ij}$, orient that face by the ordered outward conormals $(i,j)$ and the ambient coordinate orientation $dq_1\wedge dq_2\wedge dp_1\wedge dp_2$. Put

\[
 S_{ij}=\frac{\int_{F_{ij}}\omega_0}{\sqrt{\operatorname{vol}_4K}},
 \qquad S_{ji}=-S_{ij},
\]

with zero for a missing ridge or $i=j$. The spectrum means the entire eigenvalue multiset, including zero multiplicities; for this real skew matrix it is equivalent to the entire singular-value multiset. Reordering facets conjugates $S$ by a permutation. A symplectic map preserves the ambient orientation, the induced ridge orientations and every integral, so this spectrum is a symplectic invariant. Translations and uniform dilations preserve its normalized value.

For a Lagrangian product let $E,F$ have the counterclockwise primal edge vectors of its two factors as rows. The same-factor two-faces have zero integral. With the orientation just specified, the mixed block is $-EF^T$, hence

\[
 S=\frac1{\sqrt{A_PA_Q}}
 \begin{pmatrix}0&-EF^T\\FE^T&0\end{pmatrix}.
\]

The sign convention is coherent across facets; arbitrary separately signed triangle orders are not being treated as geometric data. Changing the common global sign would leave every claimed spectrum and square unchanged.

## Why the spectra coincide

The construction enforces the exact edge isotropy identity

\[
 E^TE=\lambda I_2.
\]

For $Q=R(t)P$, $F=ER(t)^T$, and $A_P=A_Q=A$. Therefore both $BB^T$ and $B^TB$, for $B=EF^T$, equal $\lambda EE^T$, independent of $t$. Consequently

\[
 S^2=-\frac\lambda{A^2}
 \begin{pmatrix}EE^T&0\\0&EE^T\end{pmatrix},
\qquad
 \det(xI-S)=x^6\bigl(x^2+\nu^2\bigr)^2,
\quad
\nu=\frac\lambda A=\frac{6999735409}{4817154572}.
\]

Thus even the full labelled matrix $S^2$ loses the varying capacity, not merely a selected scalar eigenvalue. Every even matrix power and every spectral summary agree. The signed matrix $S$ itself and its unsigned entrywise absolute value are **not** asserted equal. No claim about unsigned spectra, adjacency-dependent nonlinear functions of signed entries, arbitrary ridge functionals or generic nonproduct bodies follows.

The rational construction came from reflection-symmetric vertices $(0,t_0+u),(b,u),(d,0),(-d,0),(-b,u)$ before centering and reversal. Edge isotropy is exactly the quadric

\[
2b^2-2bd+3d^2=t_0^2+u^2.
\]

The second intersection of a line through the rational point $(1,0,1,1)$ gives

\[
(b,d,t_0,u)=\frac1{90023}(85613,52920,62213,100643).
\]

This constructs an irregular polygon directly; it is not a continuity argument from regular products or an approximate numerical whitening operation.

## Exact capacity and interval certificate

The imported [product closure-vertex reduction](../../../topics/products/product-qp-six-facet-reduction.tex) states that some global HK maximizer uses a vertex of each factor's normalized closure polytope, with each factor carrying mass $1/2$. The frozen pentagon has exactly five such vertices; all are positive three-facet closures, and exact pair determinants exclude two-facet closures. For every one of the $25$ pairs, [score.py](score.py) enumerates all $120$ cyclic six-facet orders. The maximum gives $c=1/(2Q_{\max})$ in the repository's HK convention. This is **6,000 exact rational orders across the two endpoints**, without numerical labels, a production capacity evaluator or reliance on the newer two-shadow theorem.

[verify.py](verify.py) is a second implementation: it obtains triangle weights from determinant coordinates, evaluates the HK form directly on four-dimensional weighted normals, reconstructs all geometric identities and reproduces both rational capacities. It imports neither construction nor scoring code. This is independent implementation evidence inside the lane; separate scientific review remains root-owned.

For the whole interval, every one of the $3,000$ active support/order objectives has the form $A_0\cos\theta+B_0\sin\theta$. There are $351$ distinct coefficient pairs. Since $t=\tan(\theta/2)$ and $V=A^2$, the condition

\[
(1+t^2)^2-8V\bigl(A_0(1-t^2)+2B_0t\bigr)^2>0
\]

for every candidate ensures $8VQ_{\max}^2<1$, equivalently $\operatorname{sys}>1$. The verifier converts each quartic on $[1/8,4/25]$ to degree-four Bernstein form. **Every one of the five coefficients for every objective is positive**, the smallest being the rational fraction retained in [verification.json](verification.json), approximately $0.0084176374$. Nonnegative Bernstein basis functions sum to one, so this proves strict positivity over the entire interval. It is a finite exact certificate covering all required candidates, not an angle grid.

For endpoint ratios $L<H$, a common prediction $p$ cannot have both relative errors below $(H-L)/(H+L)$: the two required intervals $[L(1-\varepsilon),L(1+\varepsilon)]$ and $[H(1-\varepsilon),H(1+\varepsilon)]$ are disjoint below that value. Here the lower bound is $0.0178633319\ldots$. Intrinsic factor shape descriptors, perimeters and areas are unchanged by the rotation, so adding only those descriptors cannot resolve this pair either.

## Selection, novelty boundary and held-out status

The frozen [PLAN.md](PLAN.md) compared covariance-equivalent canonical closure spectra, unsigned operator norms, arbitrary nonlinear entrywise powers and the selected exact nonregular signed-spectrum obstruction. Pooled fit rewards degenerate low-ratio cases; an existence-only obstruction can miss the upper tail; an arbitrary power search would repeat stopped ridge mining. The chosen endpoint criterion required exact irregularity, both ratios above one and more than two percent ratio separation **before scoring**. No endpoint or feature was tuned after its capacity became available. The subsequent continuum certificate deepens the same frozen family.

The earlier [ridge packet](../../ridge-functionals/REPORT.md) already proved signed-spectrum blindness on **regular** pentagon rotations. The added content here is an explicit rational **non-affine-regular** above-one family, with exact capacity separation and an exact certificate for the entire interval. No literature novelty, sharp global maximum, working predictor, or empirical held-out-population score is claimed. The fresh adversarial endpoints establish a deterministic upper-tail error floor for this feature class; no model was fitted and no historical labels were used. This is a constrained family, not an open set of generic four-dimensional polytopes.

Focused sources inspected: repository entry/research guide, pattern gallery/success and root targets, collaboration rationale, prior ridge-functionals/unsigned-list/fixed-closure packets, and the product closure-vertex proof. The verified source/invariant caches were identified but no corpus outcome entered this construction. This is not an exhaustive repository or literature survey. No thesis source, production evaluator, cloud work, public action or new dependency was used.

## Reproduce

All commands use only Python's standard library and write exclusively beneath the given scratch directory:

```sh
python3 patterns/upper-tail/spectral-geometry/construct.py --out /tmp/msc-math/spectral-replay
python3 patterns/upper-tail/spectral-geometry/score.py --directory /tmp/msc-math/spectral-replay
python3 patterns/upper-tail/spectral-geometry/verify.py --directory /tmp/msc-math/spectral-replay
```

`construct.py` reads its adjacent plan to bind the frozen choice; the scorer and verifier consume the resulting sample and manifests. The retained-path verification can also be replayed with `--directory patterns/upper-tail/spectral-geometry`; verification writes only `verification.json` there. Measured individual scoring and verification calls were each below one second on this workstation. Root owns integration, external scientific review and the local branch commit; the bounded lane stops after this exact obstruction.

[review: /root/review_spectra]
