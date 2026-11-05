# Simultaneous facet births cause an order-one star-payoff error

This construction refutes the **standalone** proposed inequality
$C_\star(P,Q)^2/(2|P||Q|)\le(3+\sqrt5)/5$ for genuine polygon pairs arbitrarily close to HKO. It does **not** refute the combined width/star proposal: width settles the explicit finite example. Unlike the preceding one-facet obstruction, this family makes the actual sampled payoff lose an order-one fraction, rather than merely making its coverage certificate conservative.

## Fixed family and finite result

Use the existing exact transformed HKO normals, with $d=\sqrt5$,

$$a_0=(1,0),\ a_1=((d-1)/4,1),\ a_2=(-(d+1)/4,(d-1)/2),$$
$$a_3=(-(d+1)/4,-(d-1)/2),\ a_4=((d-1)/4,-1),\quad b_i=Ja_i.$$

Here $J(x,y)=(-y,x)$. Append **all five** adjacent midpoint normals in each factor:

$$a_{5+i}(\delta)=\frac{a_i+a_{i+1}}{2(1-\delta)},\qquad b_i(\delta)=Ja_i(\delta),$$

where indices on the original normals are cyclic. Let $P_\delta,Q_\delta$ be the resulting factors. This is a deterministic geometric truncation of every original vertex. For $0<\delta<\sin^2(\pi/5)$ all ten facets in each factor are genuine. The area centroids remain zero by the inherited affine fivefold symmetry. The finite input is therefore irredundant, with no duplicated rows.

At the **single frozen depth** $\delta=1/1000$, exact arithmetic in $\mathbb Q(\sqrt5)$ gives:

| Quantity | Value |
|---|---:|
| Star capacity ceiling | 5.590698107987626… |
| Star ratio ceiling | 1.309271939418110… |
| HKO ratio | 1.047213595499958… |
| Two-bounce width capacity | 4.994473663819044… |
| Width ratio ceiling | 1.044906021839739… |

Thus the stronger standalone claim fails by about25% in ratio. The combined minimum passes on this body. No true capacity was computed. The only finite optimization is exact linear response at each of21 star points per factor, against25 individual closure vertices per opposite factor, for both prescribed shadows.

## Why every star ray collapses

At $\delta=0$ the appended normals lie on old polar edges. The limiting cone-volume center is

$$c^0=(1/5,1/5,1/5,1/5,1/5,0,0,0,0,0).$$

Nevertheless the ten-coordinate closure tangent has dimension7. Its last five coordinates are the newly born facets.

The projector signs can be proved without numerical tolerances. Transform the normal coordinates back to a regular pentagon $n_i=(\cos(2\pi i/5),\sin(2\pi i/5))$. This does not change the closure tangent or its Euclidean weight-space projector. Put $h=\cos(\pi/5)$ and $m_i=(n_i+n_{i+1})/2$. For the ten rows $u=(n_0,\ldots,n_4,m_0,\ldots,m_4)$,

$$\Pi^0_{kl}=\delta_{kl}-\frac1{10}-\frac{2u_k\cdot u_l}{5(1+h^2)}.$$

This follows from $\sum u_k=0$ and $\sum u_ku_k^T=5(1+h^2)I/2$.

For an original column $i$, a nearest midpoint row has dot product $h^2>0$, hence negative projector entry. The midpoint row opposite $n_i$ has dot product $-h$, giving the positive entry $-1/10+2h/[5(1+h^2)]>0$ (equivalently $4h>1+h^2$). For a new column, its diagonal is positive, whereas either adjacent midpoint row has positive dot product $h^2\cos(2\pi/5)$ and hence negative off-diagonal projector entry. Consequently **every column has both a positive and a negative entry among the five new coordinates**. The same conclusion holds for the other factor because $J$ preserves the row space.

Let $r_i(\delta)=\Pi(\delta)e_i$. The projector varies continuously. Every new center coordinate tends to zero, while the displayed limiting positive/negative components are nonzero. A negative new component bounds the positive chord parameter by $c_k(\delta)/[-r_{ik}(\delta)]\to0$; a positive one similarly forces the negative parameter to zero. Thus **both endpoints of every projected coordinate chord tend to $c^0$**. The entire star hull converges to the singleton $\{c^0\}$.

This also gives $\eta\to0$, but the stronger payoff conclusion below is the material distinction from the preceding one-facet result.

## Exact limiting payoff and true-capacity comparison

At zero, compute the two prescribed signed angular-word linear responses at $c^0$, in both factor roles. The exact finite closure calculation gives

$$\max_{s,\gamma\in C_B^0}Q_s(c^0,\gamma)
=\max_{s,\alpha\in C_A^0}Q_s(\alpha,c^0)=\frac{\sqrt5}{25}.$$

The positive-shadow maxima in both roles equal $\sqrt5/25$; the other shadow maxima equal $(5+\sqrt5)/100$. These are checked over all25 individual closure vertices, including vertices using the redundant limiting rows. Merely dropping those rows would not justify this step. The finite exact table is retained in `finite.json.gz`, and the coordinator's independent implementation reconstructs the closures and these responses.

The limit passes through the response maximum without an unstated regularity assumption. From a limiting feasible closure $\alpha$, multiply its last five coordinates by $(1-\delta)$ and normalize by $1-\delta\sum_{i=5}^9\alpha_i$. This is an explicit bijection $C^0\to C^\delta$ converging uniformly to the identity. The normal directions and angular words are fixed; at tied directions the relevant cross-pairing is zero. The payoff is continuous. Together with collapse of all finitely many star points, this proves

$$q_\star(\delta)\longrightarrow\frac{\sqrt5}{25},\qquad
 C_\star(\delta)\longrightarrow\frac{5\sqrt5}{2}.$$

The physical containment sandwich is

$$(1-\delta)P_0\subset P_\delta\subset P_0,\qquad
 (1-\delta)Q_0\subset Q_\delta\subset Q_0.$$

Capacity monotonicity, product homogeneity and the imported exact HKO value5 therefore give

$$5(1-\delta)^2\le c(P_\delta\times_LQ_\delta)\le5.$$

Hence the true capacity tends to5 while the star ceiling tends to$5\sqrt5/2$: its relative capacity error tends to$\sqrt5/2-1\approx11.80\%$. The area product converges to HKO volume, so

$$S_\star(\delta)\longrightarrow\frac{3+\sqrt5}{4}
=\frac54\rho_H>\rho_H.$$

This proves failure for every sufficiently small positive depth, rather than a rounded finite observation. No exact capacity computation for the truncated bodies or counterexample to HKO maximality follows. The elementary sandwich certifies a true ratio above one at the frozen depth; it does not improve a prior sharp bound.

## Combined-proposal disposition

The width lane independently supplies the reason uniform HKO cuts remain protected: their width capacity decreases linearly in depth, while area changes quadratically. The exact width ceiling at the frozen depth is already below HKO. More strongly, the [independent width lemma](WIDTH-PROTECTION.md) proves an exact width-ratio formula strictly below HKO throughout the genuine ten-facet interval. The combined width/star implication remains unresolved.

**Park the combined counterfamily route here.** A counterexample from this mechanism requires a body whose width and center-response ratio ceilings both exceed HKO, together with a genuine refinement realizing ray collapse. The current checks provide no such body. The standalone universal star claim is now false, so it cannot be used as an unqualified proof route.

## Reproduction, inspected sources and limits

Run `python3 -B finite.py --out /tmp/msc-math/facet-refinement-finite.json` from this packet. Replay the independent reconstruction with `python3 -B check.py --out /tmp/msc-math/facet-refinement-root.json`, and the symbolic width identities with `python3 -B width-check.py --out /tmp/msc-math/facet-refinement-width.json`. It imports the existing chord-star geometric helper and signed-shadow response code, and the existing two-bounce width helper from the patterns checkout; it does not import or invoke a paired capacity oracle. The checkout root is explicit in the script. It writes the selected scratch JSON output and asserts exact standalone failure, positive ten-facet cone weights, zero centroids, limiting mixed signs, and the exact limiting response value. Root independently reconstructs all calculations without importing these geometric helpers.

Inspected sources: repository `README.md`, `docs/research-guide.md`, `AGENTS.md.commentary.md` and its collaboration rationale; `patterns/upper-tail/chord-star/{REPORT.md,FACET-AMENDMENT.md,PRUNING-AMENDMENT.md,probe.py}`; `patterns/upper-tail/ceiling-crux/{REPORT.md,probe.py,compact-witnesses.json.gz}`; the signed-prefix response, spectral-chord width and quadratic-field helpers linked by those sources. This is a bounded local source investigation, not an exhaustive repository or literature search. Imported premises are the HKO capacity, capacity monotonicity/product homogeneity, and the existing definition of the star construction. No sampled cohort, capacity producer, tuning, cloud, installation, network, retained-source edit or commit was performed by this worker. Root owns independent review, retention, integration and commits.

Root independent reconstruction: [check.py](check.py) and [root-verification.json](root-verification.json) reconstruct the limiting and finite normal geometry, all projector identities and mixed signs, exact closure vertices and star points, 2,200 linear-response values and 88 literal four-dimensional HK objectives. At the frozen depth the capacity sandwich gives sys ≥ 1.0430370581926305. Both finite computations use exact Q(sqrt(5)) arithmetic; neither computes true capacity. [sources.json](sources.json) binds retained files and local arithmetic/geometry dependencies. The root retains the scientific review in [review.md](review.md).

[review: /root/message_reviewer]
