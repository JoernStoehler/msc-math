# Maximum symplectic ridge area gives an exact affine-pentagon law, with an exact scope obstruction

Let $R_\infty(K)=\max_F|\int_F\omega_0|/\sqrt{\operatorname{vol}_4K}$, where the maximum is over the two-dimensional faces of a four-polytope. With the repository's coordinate order $(q_1,q_2,p_1,p_2)$, a mixed face of a Lagrangian product $P\times_LQ$ is the product of primal factor edges $e_i,f_j$ and has absolute symplectic area $|e_i\cdot f_j|$. The two same-factor face types have zero symplectic area. This is an integral of the symplectic form, rather than Euclidean face area.

**Exact law on a stated family.** If both factors are independently affine regular pentagons, then

$$
\boxed{\quad\operatorname{sys}(K)\,R_\infty(K)^2=\frac8{25}.\quad}
$$

This is a new bridge in this packet between the existing affine-pentagon capacity formula and primal ridge geometry. It is not a new capacity theorem or an external novelty claim. In particular, the inverse maximum-ridge relation really does operate on the upper tail of this family, including the known above-one HKO reference.

**Exact obstruction beyond that family.** There is a rational pentagon $P$ for which

$$
\operatorname{sys}(P\times_LP)\,R_\infty(P\times_LP)^2
\ \geq\ \frac{121550625}{50823362}\ >\ 2.3916\ >\ 7\cdot\frac8{25}.
$$

Thus the equality's arbitrary-pentagon extension fails, and even its proposed upper-envelope version fails by more than a factor seven. This is an exact inequality, with no numerical capacity evaluation. It does not refute every weighted or nonlinear ridge functional.

## Beyond a single statistic

The [complete unsigned-list obstruction](unsigned-list/REPORT.md) is stronger in
a different domain: two explicit hexagon–triangle products match all normalized
unsigned ridge areas, volume and factor difference bodies, yet their exact
ratios differ by 12.56%. No universal exact formula based only on that complete
list can recover the ratio; every common point prediction incurs at least 5.9%
relative error on one body. Both ratios are below one. This leaves geometric
weights and approximate prediction open and makes no arbitrary-pentagon claim.

## Derivation of the positive law

Use the regular pentagon $P_5$ with circumradius one and outward unit normals $n_i=(\cos(2\pi i/5),\sin(2\pi i/5))$. Put $h=\cos(\pi/5)$, $w=1+h$, $\ell=2\sin(\pi/5)$, and $A_0=5\sin(\pi/5)h$. Let $G,H$ be nonsingular real matrices and $K=(a+GP_5)\times_L(b+HP_5)$. Translations do not affect any quantity used below.

The imported [affine-pentagon theorem](../../topics/products/pentagon-affine-products/README.md), proved in [the maintained note](../../topics/products/pentagon-affine-products/research.tex), states

$$
c(K)=\frac{w^2}{m(M)},\qquad M=G^{-1}H^{-T},\qquad
m(M)=\max_{i,j}|n_i^TMn_j|.
$$

Let $J(x,y)=(-y,x)$. Up to cyclic indexing, the base primal edge vectors are $\ell Jn_i$. Therefore the affine mixed-face pairings obey

$$
(\ell GJn_i)\cdot(\ell HJn_j)
=\ell^2\det(G)\det(H)\,n_i^TG^{-1}H^{-T}n_j,
$$

using the two-dimensional identity $J^TG^THJ=\det(G)\det(H)G^{-1}H^{-T}$. Reflections reverse edge orientation; absolute areas absorb both determinant signs and edge reversals. For $D=|\det(G)\det(H)|$ this gives

$$
A_{\max}=\ell^2D\,m(M),\qquad V=A_0^2D,\qquad
\operatorname{sys}(K)R_\infty(K)^2
=\frac{w^4\ell^4}{2A_0^4}=\frac8{25}.
$$

For the last simplification use $(1+h)/h=\sqrt5$. This proves the law for all independent shears, stretches, rotations and reflections of the two regular factors, conditional only on the named existing capacity theorem. The law is invariant under linear symplectic maps, translation and uniform scaling because both quantities on the left are invariant under those operations; consequently it also holds on their images of this family.

## Why the rotation control does not favor a new weighted sum

For $K_\theta=P_5\times_LR_\theta P_5$ put $d=\min_{k\in\mathbb Z}|\theta-k\pi/5|\in[0,\pi/10]$. The maximum of the normal pairings is $\cos d$. On one fundamental interval the root-sum identity is

$$
\sum_{j=0}^4|\cos(d+2\pi j/5)|=(1+\sqrt5)\cos d.
$$

Hence the total normalized symplectic ridge area $R_1$ satisfies

$$
R_1=5(1+\sqrt5)R_\infty,\qquad
\operatorname{sys}(K_\theta)R_1^2=48+16\sqrt5.
$$

So lowest ridge sum and highest systolic ratio coincide **exactly** in this rotation family. The sum has not discarded the relevant relative angle here. A weight determined only by individual congruent factor edges or their local congruent neighborhoods is constant on those edges and reduces to this same sum. This statement does not cover pair-dependent weights.

Several natural nonlinear alternatives actually discard the angle:

- Every even entry power sum $\sum_{i,j}|e_i\cdot f_j|^{2r}$ for $r=1,2,3,4$ is constant along the rotation family. Expand $\cos^{2r}$ into harmonics; the five-root sum kills every nonconstant frequency because its even frequencies are at most eight. Power ten is the first even power sum that can carry the missing frequency ten.
- The signed pairing matrix $B_{ij}=e_i\cdot f_j$ has just two nonzero singular values, both $5\ell^2/2$, independently of relative rotation. This follows from $E^TE=F^TF=(5\ell^2/2)I_2$. Signed singular-value summaries therefore cannot reconstruct this family's varying ratio.
- For the unsigned matrix $A=|B|$, every row and column has the same sum $r$. Its operator norm is exactly $r$: the constant vector attains it, while $\|A\|_2\leq\sqrt{\|A\|_1\|A\|_\infty}=r$. Thus its operator norm is just one fifth of total ridge area on this family, with no additional information.

These are exact representation obstructions and identities on the regular rotation family, rather than empirical performance claims on arbitrary products.

## Rational pentagon obstruction

Take

$$
T=\operatorname{conv}\{(-1,-1),(2,-1),(-1,2)\},\qquad
P=T\cap\{x\leq7/4,\ y\leq7/4\}.
$$

The five vertices of $P$, in cyclic order, are

$$
(-1,-1),\quad(7/4,-1),\quad(7/4,-3/4),\quad(-3/4,7/4),\quad(-1,7/4).
$$

Then $(7/8)T\subset P\subset T$, $\operatorname{area}P=71/16$, and the largest absolute edge pairing is $25/2$ (the long diagonal edge paired with itself). The [existing exact triangular control](../product-shape/exact_controls.py) proves $c(T\times_LT)=9/2$ by complete finite rational word enumeration. Capacity monotonicity and degree-two conformality give

$$
c(P\times_LP)\geq(7/8)^2(9/2)=441/128.
$$

Consequently

$$
\operatorname{sys}(P\times_LP)\geq\frac{194481}{645248},\qquad
R_\infty(P\times_LP)=\frac{200}{71},
$$

and multiplying yields the displayed obstruction. Neither the exact capacity nor the exact ratio of $P\times_LP$ is needed. This is a genuine pentagon product, with five positive-length edges in each factor.

## Retained product check and rejected lower-envelope conjecture

A geometry-only reconstruction of all **10,240** retained random products reproduces the existing normalized maximum-ridge field with maximum relative discrepancy **1.12e-12**, and stored four-volume within **2.23e-12**. It makes no capacity call. For all **1,024 random 5×5 products**, $\operatorname{sys}R_\infty^2$ lies **above** $8/25$: minimum **0.3543099**, median **1.8709673**, maximum **7.4823347**. Among the 103 highest-ratio rows in this bucket its median is **1.1392756**. The largest-ratio row, `random_5x5_s2_104`, has historical sys **0.8437955** and product **0.8702873**. Thus replacing the sum by the maximum does not transfer the exact affine formula into an accurate general predictor.

That consistent direction originally suggested the following lower-envelope conjecture, **falsified by a targeted exact construction later on 17 October**:

$$
\operatorname{sys}(P\times_LQ)R_\infty(P\times_LQ)^2\geq8/25
\quad\text{for arbitrary convex pentagons }P,Q.
$$

Equivalently, $c(P\times_LQ)A_{\max}\geq(4/5)\operatorname{area}P\operatorname{area}Q$. Every independently affine regular pair is an equality case of the displayed expression, but the arbitrary-pentagon inequality is false. The rational obstruction above is consistent with this **lower** envelope. The existing hexagon control had already ruled out an all-side-count version: for the exact hexagon control $S\times_LJS$ from [the symmetrization packet](../product-shape/REPORT.md), $\operatorname{sys}=8/9$, $\operatorname{area}S=27/4$ and $A_{\max}=9/4$, giving $\operatorname{sys}R_\infty^2=8/81<8/25$. The retained all-side-count minimum **0.2089868** supplies a separate numerical contrast.

The [exact five-edge counterexample](lower-envelope/REPORT.md) now settles this uncertainty negatively. For explicit integer pentagons, a closed HK word and a separate symplectic-cylinder argument both give $c\leq7$, while $A_{\max}=10$ and $\operatorname{area}P\operatorname{area}Q=225/2$. Hence

$$
\frac{cA_{\max}}{\operatorname{area}P\operatorname{area}Q}
\leq\frac{28}{45}<\frac45,
\qquad \operatorname{sys}R_\infty^2\leq\frac{392}{2025}<\frac8{25}.
$$

The exact affine identity remains valid. Together, the two rational constructions show that its constant gives neither an upper nor a lower envelope on arbitrary pentagon products. The earlier 1,024-row observation remains finite numerical evidence; it did not establish the proposed inequality. No exact capacity value, general predictor, or global-maximality theorem is needed or obtained. The bounded falsification probe is complete; no further tuning is selected.

[summary.json](summary.json) pins source/table/join SHA-256 identities and complete summary statistics. The retained targets are historical numerical capacity/sys fields, with numerical volume; these numbers are neither new capacities nor certified ratio intervals. Known regular/HKO controls are separate from the random data and do not enter its count. Numerical corroboration evaluates the imported formula on 256 independent affine controls, including 127 negative determinant products: maximum relative error in the $8/25$ identity is **2.23e-15**. The 101-angle rotation checks corroborate all stated matrix/power identities to relative error below **1.8e-15**, and the unnormalized sum-law constant to absolute error below **8.6e-14**.

## Method selection and evidence boundary

End state: settle whether replacing ridge sum by a simple nonlinear functional can give a genuine upper-tail law, then establish the law's domain before any new search. An adversarial exact family cannot be skipped because broad pooled scores conceal the relevant failure; the affine identity cannot be skipped because it supplies the geometric mechanism; the pentagon obstruction cannot be skipped because extending that mechanism to arbitrary products is the consequential conjecture.

Options considered were ordinary $L^p$/concentration summaries, matrix singular-value summaries, incidence-aware weighted sums, and a maximum-area law guided by the known affine formula. A raw prediction rubric rewards bulk variance and repeats old scalar mining; a best-ratio rubric rewards uninformative numerical polishing; a representation/exact-law rubric can undervalue an imperfect but useful general predictor. The chosen bounded comparison favors exact discrimination and a large scope violation. Max/share/entropy concentration already has substantial [historical analysis](../../topics/empirical-studies/sys-datascience/methods/ridge-mechanism-discriminator/README.md), including independent sub-threshold validation; this packet does not repeat that as a claimed discovery.

New scripts began in `/tmp/msc-math/thesis-owner/2026-10-17/ridge-functionals/` and were intentionally retained here. [exact_control.py](exact_control.py) checks the rational containment, polygon area, all 25 edge pairings and the factor-seven inequality with `Fraction`. It also checks the exact ridge geometry of the imported hexagon control. It imports monotonicity/conformality and the named exact triangle/symmetric-product capacities; it does not independently prove those theorems. The positive-law derivation is algebraic; numerical checks described in the accompanying JSON are corroboration, not proof premises. No new capacity producer, cloud work, installation, or public action is required.

Focused sources inspected: repository entry and research guide, collaboration rationale, empirical method and ridge-anatomy owners, retained feature schema, existing pattern packets, and affine-pentagon theorem owner. This is not an exhaustive repository or literature search. Mathematical novelty and human acceptance remain unclaimed.

## Reproduce

```sh
python3 patterns/product-shape/exact_controls.py
python3 patterns/ridge-functionals/exact_control.py
OPENBLAS_NUM_THREADS=1 python3 patterns/ridge-functionals/analyze.py \
  --repo "$PWD" --source /path/to/verified/random-product.jsonl \
  --out /tmp/msc-math/ridge-functionals-reproduce
```

The numerical analyzer requires only the already-available NumPy dependency. It checks pinned input hashes and writes exclusively beneath `--out`; default repository resolution is appropriate when run from its retained path. The exact verifier needs only the standard library.

[review: /root/message_reviewer]
