# Completing the conditional response restores exactness on an open above-one region

17 October 2026. There is a **nonempty open set in all twenty planar normal coordinates** around the existing rational product `near_hko_s4_e100` on which the two-word conditional LP has unique optima, the completed conditional-response rule recovers exact capacity, the former one-response rule strictly misses it, and the systolic ratio remains above one. This is a qualitative separation of two extraction rules, not a smaller numerical error on one sample. The mechanism is strict recovery of the winning B support from a conditional mixture after the already correct A response. The claim concerns exact mathematical LP solutions and responses, not numerical solver reliability.

This is an open region around a specified perturbed product. It is **not** a whole neighborhood theorem around the symmetric HKO product, an all-input extraction theorem, or a global-maximality result. A separate reference lemma shows that at exact HKO *every* conditional seed from *any feasible coupling* completes exactly; that lemma alone does not settle local exactness.

## What was selected

[PLAN.md](PLAN.md) compares three structural cruxes and criticizes misleading rubrics before selecting the strict stable-repair certificate after a reporting checkpoint. The full-HKO-neighborhood question has ten tied winning support pairs at its reference point and unresolved optimizer/support chambers. The selected crux reuses the already exposed strict one-response miss; it does not select a new geometry from capacity labels or refit a predictor. Its end state is a proof of nonempty open exact repair or a precise missing hypothesis. The fixed finite certificate succeeds.

The earlier [one-response packet](../near-hko-transfer/REPORT.md) already established an open above-one region of failure, and the [completed-response packet](../lp-seeded-response/REPORT.md) recovered the displayed control. This packet adds the strict response/support and cross-word identities needed to prove that the repair itself holds on an open region. It does not credit the known capacity as a new numerical discovery.

## Exact local certificate at the rational control

Write $x_0=(a_0,\ldots,a_4,b_0,\ldots,b_4)$ for the twenty retained rational normal coordinates in [fixture.json](fixture.json). The normalized closures are

$$C_A=\{\alpha\ge0:\mathbf1^T\alpha=1,\ A^T\alpha=0\},\qquad C_B=\{\gamma\ge0:\mathbf1^T\gamma=1,\ B^T\gamma=0\}.$$

For either frozen shadow word $w$, use the actual HK normalization

$$Q_w(\alpha,\gamma)=\frac14\sum_{ij}\varepsilon_{ij}(w)\,\alpha_i\gamma_j\,a_i\cdot b_j.$$

The imported sparse closure reduction and two-shadow theorem identify true capacity with $1/(2Q_*)$, where $Q_*$ maximizes over the five A vertices, five B vertices, and both words. These are proof dependencies, not consequences of this certificate. The retained pair enumeration is a diagnostic optimality certificate; pair scores are not inputs to the extraction rule.

The exact checker verifies every old one-response candidate, both LP primal/dual certificates, the chosen completed seed, and all fifty existing pair-word objectives by the direct four-dimensional HK sum. All displayed decimals below only abbreviate exact rationals in [verification.json](verification.json).

| Strict property at $x_0$ | Positive shadow | Negative shadow |
|---|---:|---:|
| Positive basis entries / equality rank | $17/17$ | $17/17$ |
| Nonbasic entries, all with positive dual slack | $8$ | $8$ |
| Minimum positive primal entry | $0.0207190923\ldots$ | $0.0207190923\ldots$ |
| Minimum nonbasic dual slack | $0.00008227488\ldots$ | $0.003430836\ldots$ |
| Absolute determinant of the selected reduced basis | $2.6769438718\ldots$ | $2.6769438718\ldots$ |

All same-factor pair determinants and triple affine determinants are nonzero. The factors are genuine bounded five-facet polygons with zero strictly inside. All shadow-ray pair determinants are nonzero (minimum absolute value $0.002085255$), and the angular cut is avoided (minimum absolute y coordinate $0.005$). Thus no normal/facet, closure-support, or word boundary is met.

For both words the same **unique closure-vertex pair** wins:

$$\operatorname{supp}\alpha_* =\{0,2,3\},\qquad
\operatorname{supp}\gamma_* =\{1,2,4\},\qquad
Q_* =\frac{235970890203063841411016855771}{2332456764724528410000000000000}.$$

Every other closure pair in either word has objective at most $Q_*-\Delta_{\rm pair}$, with exact $\Delta_{\rm pair}>0$ and $\Delta_{\rm pair}=0.000153219849371\ldots$ for display. The word tie on this one pair needs a structural justification for a local theorem; equality at one point would not suffice.

On the active supports, the words reduce to

$$w_+: (B_4,A_2,A_3,B_1,B_2,A_0),\qquad
w_-: (A_0,B_4,A_3,A_2,B_2,B_1).$$

Rotate $w_-$ cyclically to start at $B_4$. Its same-factor blocks are exactly those of $w_+$; only the order within the consecutive A and B blocks differs. Cyclic rotation preserves $Q$ under closure, and consecutive same-factor transpositions contribute zero symplectic pairing. Consequently

$$Q_{w_+}(\alpha,\gamma)=Q_{w_-}(\alpha,\gamma)$$

for every closed pair with these supports throughout the fixed word chamber, including the varying closure vertices near $x_0$. This is an identity, not an accidental numerical tie.

## The missing opposite response removes the mixture

Choose the positive-shadow row-zero conditional of its unique LP coupling $W_+$. Its mass is $0.3264902445\ldots>0$. Its normalized B closure is exactly

$$\widehat\gamma=\lambda\gamma_*+(1-\lambda)\gamma_{\rm bad},\qquad
\lambda=\frac{288743526589183882173667635863415637485770895105}{468709669366553352197129050722759042012776652914}\in(0,1),$$

where $\operatorname{supp}\gamma_{\rm bad}=\{1,3,4\}$. The first A best response to $\widehat\gamma$ is uniquely $\alpha_*$. Its objective gap over the other four A vertices is at least $0.00622469338\ldots>0$. The subsequent B best response to $\alpha_*$ is uniquely $\gamma_*$, with gap at least $0.000226174168\ldots>0$ over the other four B vertices. Hence the two unconditional initialization responses, with the frozen first-max rule, reach $(\alpha_*,\gamma_*)$ exactly. Further strict ascent cannot improve a globally maximizing feasible pair.

The old extractor instead stops at $(\alpha_*,\widehat\gamma)$. This is its best candidate across all twenty row/column seeds and both words, as checked exactly. Its strict loss is

$$Q_*-Q_{\rm old}=(1-\lambda)\left(Q_*-Q_{w_+}(\alpha_*,\gamma_{\rm bad})\right)>0,$$

with display value $0.0000868420159062\ldots$. This explains the loss geometrically: the first response already chose the right A support, but the unresponded B closure retains positive weight on a losing support. This is also a stable face mechanism: the coupling entry $W_{00}$ is a nonbasic zero, so the row-zero conditional lies on the edge $C_B\cap\{\gamma_0=0\}$. Its only two vertices are $\gamma_*$ and $\gamma_{\rm bad}$. The continued LP basis keeps $W_{00}=0$; the two endpoint supports persist, and the strict mixture coefficients remain in $(0,1)$. Thus the mixture identity for this seed persists throughout a sufficiently small open region, even if another old seed becomes the largest old candidate. Completion chooses the correct pure B vertex; the proof does not claim that additional ascent or a different optimizer was necessary.

At the control, the exact capacity is $4.942255298349177\ldots$, the former upper bound is $4.946501329713695\ldots$, and the exact systolic ratio is $1.028881001955570\ldots>1$. The material result is the strict open-set separation; the small displayed capacity difference is not counted as numerical polishing.

## Why the statement is uniform on an open set

**Proposition.** There is an open neighborhood $U$ of $x_0$ in $\mathbb R^{20}$ such that every input in $U$ presents genuine bounded planar pentagon factors, has systolic ratio above one, has a unique exact conditional LP optimum in each shadow, and is recovered exactly by the completed all-seed rule while the old one-response rule is strictly inexact.

**Proof.** First shrink to the open geometry and angular chamber certified by the nonzero determinants and cut distances. The same five closure supports remain feasible vertices with positive active weights, given continuously by invertible three-by-three systems; infeasible supports retain their signs. Boundedness, genuine facets and the interior origin persist. Both words remain fixed.

The full homogeneous conditional equality space has dimension nine near $x_0$: rank-two normal matrices have three-dimensional kernels, and matrices with their columns in $\ker A^T$ and rows in $\ker B^T$ form $\ker A^T\otimes\ker B^T$. Thus the homogeneous equality rank is $25-9=16$. The mass functional is independent because a strictly positive normalized product of two full-support closures exists. Adding normalization gives equality rank exactly seventeen throughout this chamber. This structural rank argument ensures that solving seventeen selected equations also satisfies every displayed dependent equality locally.

For each word, continue its selected seventeen-column basis using the same seventeen independent rows. Its nonzero determinant persists. Its primal solution varies continuously. For the dual, keep the four coordinates outside the selected independent rows fixed at their certified base values, and solve the seventeen active-column tightness equations for the other seventeen dual coordinates using the invertible transposed basis. This construction agrees with the full certified dual at the base and varies continuously; it does not assume that the twenty-one-coordinate displayed dual is unique. All seventeen primal coordinates remain positive, all eight nonbasic dual slacks remain strictly positive, and all equalities hold by the constant-rank argument. The continued primal and dual therefore certify optimality. Strict nonbasic slack forces every optimal coupling onto these seventeen columns, and the invertible basis then forces uniqueness. Thus this reasoning applies to the mathematical LP optimum independently of an optimizer's tie choices. The row-zero mass remains positive and its conditional closure varies continuously.

The positive-shadow row-zero response gap and its subsequent opposite-response gap stay strictly positive. Therefore these two exact responses continue to choose the vertices with supports $\{0,2,3\}$ and $\{1,2,4\}$. Every competing pair-word gap stays positive since there are finitely many continuously varying values. The winning pair's two word values remain identically equal by the cyclic block argument above. Hence this same pair attains the global HK objective throughout a sufficiently small open neighborhood. The all-seed completed rule includes this seed, and every other extracted pair is feasible, so its selected maximum equals true $Q_{\max}$.

Every old conditional exists locally because its row/column mass stays positive. Its best-response *value* is a finite maximum of continuous pairings, regardless of response ties. The maximum over the twenty old candidates is therefore continuous. The strict gap $Q_*-Q_{\rm old}>0$ persists. Finally polygon area and the positive true objective vary continuously, so the strict inequality $\operatorname{sys}>1$ persists. Intersecting these finitely many open neighborhoods gives the claimed nonempty open $U$. $\square$

No numerical radius is supplied. The proposition is about all twenty real normal coordinates in some open set around the named rational product, not just a one-dimensional perturbation slice. It does not extend to arbitrary four-dimensional nonproduct perturbations, and it does not promise that floating-point LP solving or exact reconstruction will succeed for every nearby machine input.

## Secondary HKO reference lemma and its limitation

At the exact algebraic HKO presentation from [the coupling certificate](../conditional-coupling/hko-exact.json), independently recomputed five-by-five tables show that **every A vertex has exactly two B best responses with value $Q=1/10$, and every B vertex has exactly two A best responses with that same value**, in each shadow separately. The ten winning pairs are identical in both shadows. The minimum gap to a nonpartner is $(7-3\sqrt5)/20>0$ in the positive shadow and $(3-\sqrt5)/20>0$ in the negative shadow.

Starting from any feasible fixed closure, the first response selects some closure vertex. The second response then achieves $1/10$ by the table property. Consequently every nonzero row/column seed of **any feasible conditional coupling**, whether LP optimal or not, completes to true HKO capacity. First-max ties cannot invalidate this conclusion. This answers the proposed every-optimal-coupling HKO crux positively, by a stronger result that does not require enumerating the optimizer face.

The strict reference gaps exclude nonpartner edges nearby but leave ten formerly tied winning edges. They do not ensure that any specified seed or all seeds reach whichever of those edges becomes globally best after perturbation. No theorem of exact completed-rule recovery on an entire neighborhood of HKO, and no counterexample to that stronger theorem, is established here. The strict rational-control proposition supplies a genuine local result without conflating these two scopes.

## Reproduction, evidence owner, and inspected sources

Run from the patterns checkout, writing the receipt to scratch:

```sh
python3 -B patterns/upper-tail/local-completion/check.py \
  --repo "$PWD" --directory patterns/upper-tail/local-completion \
  --out /tmp/msc-math/local-completion/replay-verification.json
```

[check.py](check.py) uses standard-library exact rational and ordered $\mathbb Q(\sqrt5)$ arithmetic, importing only the previously independent geometry/HK/LP replay routines. It imports no optimization or prediction producer. [fixture.json](fixture.json) freezes the one unchanged rational control, the previously retained completed seed, and the exact HKO control; its source hash map binds their owners. [verification.json](verification.json) records PASS, exact margins/determinants, all HKO table entries, and its own input hashes. The new scientific check consumed under one CPU second. No new geometry, perturbation, optimizer call, cohort, fit, network, installation, cloud work, or selected-manuscript change occurred. The existing fifty pair-word values and fifty HKO pair-word values were replayed only for structural certification. Root owns independent scientific review, integration, commits and cleanup; this worker does not commit.

[scope-sources.md](scope-sources.md) identifies the inspected sources and support boundaries. This is a local proof/certificate investigation, not an external-literature search, an exhaustive novelty claim, a numerical-radius certificate, or human mathematical acceptance.

[review: /root/intake_review]
