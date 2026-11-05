# Conditional closure gives a certifying LP, with an exact HKO obstruction

17 October 2026. A nonnegative conditional-closure coupling LP supplies a capacity **lower** bound that complements the existing feasible capacity upper bounds. On the retained `random_5x5_s1_64` coordinate trap, exact rational primal/dual certificates match the existing feasible witness and therefore prove its capacity exactly, without enumerating closure-support pairs. The relaxation is nevertheless **not sharp at the HKO regular-pentagon control**: its exact HK objective is larger than the true optimum by the factor $(1+3/\sqrt5)/2\approx1.170820393$. Thus this particular relaxation loses 14.59% in its capacity lower bound at the main extremal control. The HKO-sharp route stops here; no extra cohort, fitting or tuning follows.

The positive structural result is that the relaxation is exact whenever either factor's nonnegative closure cone is simplicial. In particular, this covers every product with a triangular or quadrilateral factor. These results are conditional on the repository's imported HK capacity formula, sparse closure reduction, two-shadow theorem and established regular-pentagon capacity formula. They are research evidence outside the selected manuscript, with no outside-literature novelty claim.

## The relaxation and the bound

Write the dual facet vertices of two full-dimensional planar factors containing zero in their interiors as $a_i,b_j$. Their normalized closures satisfy

$$C_A=\{\alpha\ge0:\sum_i\alpha_i=1,\ \sum_i\alpha_i a_i=0\},\qquad C_B=\{\gamma\ge0:\sum_j\gamma_j=1,\ \sum_j\gamma_j b_j=0\}.$$

For a full word $w$ of the A and B labels, put

$$c_{ij}(w)=\tfrac14\varepsilon_{ij}(w)a_i\cdot b_j,\qquad\varepsilon_{ij}=1\text{ if A}_i\text{ precedes B}_j,\ -1\text{ otherwise}.$$

The factor $1/4$ is exactly the product of the two factor masses $1/2$: $\sum c_{ij}\alpha_i\gamma_j$ is the actual HK objective $Q$, not another quarter of that objective.

Define the linear program

$$L_w=\max_{W\ge0}\sum_{ij}c_{ij}(w)W_{ij},\quad \sum_{ij}W_{ij}=1,\quad \sum_jW_{ij}b_j=0\ (\forall i),\quad \sum_iW_{ij}a_i=0\ (\forall j).\tag{1}$$

Every rank-one coupling $W_{ij}=\alpha_i\gamma_j$ is feasible. More generally every convex combination of such couplings is feasible. The converse is not assumed. The row and column marginals of every feasible $W$ are normalized closures, but conditional zero means allow additional dependence between the factors.

Let $w_+$ merge A directions with $J B$ directions counterclockwise; let $w_-$ be the **reverse** of the counterclockwise merge of A with $-J B$. The existing sparse/two-shadow theorem provides a true maximizer whose nonzero labels have one of these restricted orders. Positive closure weights do not change angular order, and equal-ray cross ties have zero dot product. Inserting zero-weight labels into the full word changes nothing. Consequently

$$Q_{\max}\le L:=\max(L_{w_+},L_{w_-}),\qquad \boxed{\frac1{2L}\le c_{\rm EHZ}}.\tag{2}$$

A fixed cyclic cut is harmless even for feasible nonseparable $W$: moving an A label across the cut changes its row contribution by a multiple of $a_i\cdot\sum_jW_{ij}b_j=0$; moving B uses column closure. No claim that two words optimize an arbitrary coupling over all words is needed for (2).

An existing feasible HK witness with objective $q$ gives the other direction, $c_{\rm EHZ}\le1/(2q)$. Equality $L=q$ therefore certifies exact capacity. An interval with unequal endpoints does **not** establish that the relaxation has a true gap: the feasible witness might be suboptimal.

### Dual certificates and size

A dual certificate is a scalar $t$ and vectors $u_i,v_j\in\mathbb R^2$ such that

$$t+u_i\cdot b_j+v_j\cdot a_i\ge c_{ij}(w)\quad\text{for every }i,j.$$

Summing against feasible $W$ gives $\sum c_{ij}W_{ij}\le t$. Exact feasible primal and dual solutions with equal objective certify $L_w=t$. The LP has $nm$ nonnegative variables and $1+2n+2m$ displayed equalities; four closure equalities are dependent. For full-rank factor normal matrices their rank including normalization is $2n+2m-3$. No closure-vertex pairs are generated or searched by this method.

## Exactness when one closure cone is simplicial

**Proposition.** Suppose the cone $K_A=\{x\ge0:\sum_i x_i a_i=0\}$ has linearly independent extreme-ray generators $r^1,\ldots,r^d$. Then every feasible coupling in (1) is a convex combination of normalized rank-one closures. In particular, every fixed-word LP and the combined two-word capacity bound are exact. The same holds with A and B exchanged.

**Proof.** Each column of $W$ belongs to $K_A$, so write $W_{\cdot j}=\sum_k r^k z_{kj}$ with $z_{kj}\ge0$. Row conditional closure gives

$$0=\sum_jW_{\cdot j}b_j^{\mathsf T}=\sum_k r^k\left(\sum_j z_{kj}b_j\right)^{\mathsf T}.$$

Linear independence of the $r^k$ implies $\sum_jz_{kj}b_j=0$ for every $k$. Discard zero terms, put $R_k=\sum_i r_i^k>0$, $Z_k=\sum_jz_{kj}>0$, $\alpha^k=r^k/R_k$ and $\gamma^k=z_{k\cdot}/Z_k$. These are normalized closures and

$$W=\sum_k\lambda_k\alpha^k(\gamma^k)^{\mathsf T},\qquad\lambda_k=R_kZ_k\ge0,\quad\sum_k\lambda_k=\sum_{ij}W_{ij}=1.$$

A linear objective on this convex hull attains no larger value than its best rank-one term. Conversely rank-one terms are feasible. For the combined bound, the sparse/two-shadow theorem gives the global maximum among the two rank-one word maxima. This proves the proposition. $\square$

For an actual triangular or quadrilateral factor, the two-dimensional normal matrix has rank two and admits a strictly positive closure, so $K_A$ has dimension $n-2$, respectively one or two. It is pointed because it lies in the nonnegative orthant. A full-dimensional pointed polyhedral cone in dimension one or two is simplicial. Thus the proposition covers every such factor, including products against an arbitrary polygon. No analogous automatic simpliciality holds in dimension three, which is already reached by pentagons.

## Exact HKO obstruction

To avoid trigonometric extensions beyond $\mathbb Q(\sqrt5)$, set $s=\sin(2\pi/5)$, $T=\operatorname{diag}(1,1/s)$ and transform regular unit normals $n_i=(\cos(2\pi i/5),\sin(2\pi i/5))$ to

$$A=\left[(1,0),\left(\tfrac{\sqrt5-1}4,1\right),\left(-\tfrac{\sqrt5+1}4,\tfrac{\sqrt5-1}2\right),\left(-\tfrac{\sqrt5+1}4,-\tfrac{\sqrt5-1}2\right),\left(\tfrac{\sqrt5-1}4,-1\right)\right],\qquad b_i=J a_i.$$

The original relative rotation $\pi/2$ is equivalent modulo pentagon symmetry to $\pi/10$, the HKO angle. The imported regular-pentagon formula gives $Q_{\rm true}=s/10$ for the original unit dual normals. Since $JT=(\det T)T^{-\mathsf T}J$, the new pair differs from a symplectic normal transformation by multiplication of the B normals by $\det T=1/s$. Thus $Q$ scales by $1/s$ and capacity by $s$, giving

$$Q_{\rm true}=\frac1{10},\qquad c_{\rm true}=5$$

in these transformed coordinates.

[The exact certificate](hko-exact.json) gives primal and dual solutions for both full words, proving

$$L_{w_+}=\frac{-5+7\sqrt5}{100},\qquad L_{w_-}=\frac{5+3\sqrt5}{100}.$$

A compact optimal primal for **both** words is the circulant coupling

$$W_{ij}=\frac15w_{(j-i)\bmod5},\qquad (w_0,w_1,w_2,w_3,w_4)=\left(0,\frac1{\sqrt5},0,\frac{1-1/\sqrt5}2,\frac{1-1/\sqrt5}2\right).$$

The checker verifies all conditional closures and both objectives exactly; the retained duals prove optimality. Therefore

$$\boxed{\frac{L}{Q_{\rm true}}=\frac{1+3/\sqrt5}{2}>1,\qquad\frac{1/(2L)}{c_{\rm true}}=\frac{3\sqrt5-5}{2}\approx0.854101966.}$$

This proves that the feasible coupling set strictly exceeds the convex hull of rank-one closures: otherwise this objective could not exceed the known true HK maximum. Matrix rank greater than one by itself would not prove that statement, since mixtures of rank-one closures can also have higher rank. The mechanism is loss of compatibility between rowwise and columnwise closure decompositions; simpliciality rules out that loss, while the pentagon control exhibits it. This is an exact obstruction to HKO sharpness of this relaxation, independent of numerical labels or solver tolerances.

## Frozen six-pilot evidence

The [plan](PLAN.md) froze three rational controls and three regular rotations before evaluation. Rational geometries were read from retained result files, without reading a population producer or selecting new examples. The regular pilots used floating trigonometric geometry only; the HKO conclusion above was then established separately with exact quadratic-field certificates.

| Retained rational body | Certified capacity interval | Upper/lower |
|---|---:|---:|
| `random_5x5_s1_64` | exactly $6.2032265060958105\ldots$ | exactly 1 |
| `random_5x6_s1_198` | $[4.4365955819815195\ldots,\ 4.484227519554728\ldots]$ | $1.0107361459\ldots$ |
| `random_6x6_s3_124` | $[4.474980102946307\ldots,\ 4.606321493224962\ldots]$ | $1.0293501618\ldots$ |

All endpoints are retained as exact rationals in [results.json](results.json); displayed decimals are approximations. The upper endpoint is the strongest already retained feasible bound among the applicable comparison packet's methods and the full-partner bound. Every upper endpoint has an explicit retained feasible HK witness, rechecked directly in four dimensions. On the coordinate trap, equality with the LP bound changes its status from a feasible challenger to a nonglobal response into an exactly optimal capacity witness. The numerical capacity magnitude was already present in the historical labels; the new result is exact optimality certification, not a smaller capacity value. The other two intervals remain unresolved; numerical agreement of the third lower endpoint with its historical capacity is not an exactness proof.

The numerical regular controls have $L/Q_{\rm known}$ equal to approximately 1.11803399, 1.14376510 and 1.17082039 at relative rotations $0,\pi/20,\pi/10$. Their failure at HKO triggered the declared stop; no validation expansion is selected. These six deliberately selected design controls establish no held-out predictor or frequency claim. The LP lower bound also cannot alone prove an upper bound on $\mathrm{sys}=c^2/(2A_PA_Q)$ or global HKO maximality; it supplies the opposite capacity direction.

## Verification, reproduction and scope

[verify.py](verify.py) imports no optimizer or discovery code. Using only Python's standard library and the exact ordered quadratic-field implementation [quadratic.py](quadratic.py), it independently reconstructs both angular words, every objective and equality matrix, checks primal nonnegativity and conditional closure, dual inequalities, complementary slackness and primal/dual equality, and checks three feasible upper witnesses by the direct four-dimensional HK sum. It checks six rational and two quadratic-field LP certificates, plus the explicit circulant HKO witness. [verification.json](verification.json) records PASS and input hashes. Solver output selects an active face; exact systems reconstruct all rational/algebraic values. Free coordinates of a degenerate rational dual face are chosen near the numerical solution and then all equalities and inequalities are verified exactly. Decimal rounding is never accepted as a certificate.

Generation uses the already cached SciPy environment; certificate replay does not require SciPy. From this patterns checkout:

```sh
python3 -B patterns/upper-tail/conditional-coupling/verify.py \
  --directory patterns/upper-tail/conditional-coupling --repo "$PWD"
```

To reconstruct the frozen LP runs, choose a new scratch directory, copy `PLAN.md`, `analyze.py`, `algebraic.py` and `quadratic.py` there, then run:

```sh
uv run --offline --with scipy python "$OUT/analyze.py" --repo "$PWD" --out "$OUT/results.json"
uv run --offline --with scipy python "$OUT/algebraic.py" --out "$OUT/hko-exact.json"
```

No existing original-body capacity producer, all-support-pair enumeration, population scan, fitting, new random generation, download, installation, cloud work or manuscript edit was used. New files began in `/tmp/msc-math/conditional-coupling/20261017T1519Z/`. The receiving owner owns independent verification and thesis integration; this packet remains on the patterns branch.

Focused inspected sources: repository and collaboration entry points; `topics/products/README.md`; the six-facet reduction; `pentagon-affine-products/research.tex` (in particular the sparse reduction and regular-pentagon formula); the two-shadow theorem; full-partner containment and validation reports and retained results; and signed-prefix response implementation. Named-term searches for conditional closure/coupling found no equivalent LP in those inspected pattern/product sources. This is a bounded local comparison, not an exhaustive literature search or novelty assessment. Source hashes bind the reused rational geometries and comparators. Scientific review is retained separately in this packet; it does not constitute human mathematical acceptance.

[review: /root/coupling_science_review]
