# Capacity interface check, 22 October 2026

Bounded follow-up to [the continuum review](continuum-review-20261022.md),
started 2026-10-22 13:53:26 UTC by
`/root/msc_today_scope/continuum_review`. This checks applicability of a
published theorem to the exact fixture; it does not independently audit
the published proof. No source or manuscript is changed.

**Conclusion:** the fixture matches the published normal-triangle
criterion. Combined with the continuum result and the
[fresh finite verification](verification/20261022/report.md), this supports
the stated local EHZ systolic-ratio maximum and translation/homothety
equality classification, relative to the imported theorem. There is no
remaining normalization or template-completeness gap in this interface.

## Imported statement and checked scope

I directly inspected [Balitskiy–Mitrofanov–Polyanskii,
arXiv:2603.12495v2](https://arxiv.org/html/2603.12495v2), specifically
Theorem 2.5, §3.1, Definition 3.2 and Theorem 3.3. Their convex shapes are
compact planar convex sets with nonempty interior. Theorem 2.5 explicitly
identifies the capacity as Ekeland–Hofer–Zehnder. For polygonal `Q`,
Definition 3.2 uses directed facet-normal edges and total support-function
perimeter one; degenerate segments require opposite facet normals.
Theorem 3.3 gives the equivalence between fitting every such triangle by
translation and `c_EHZ(K ×_L Q)≥1`. This includes nonsmooth `K` and `Q`;
no facet bound on `K` is imposed. These statements are imported inputs
whose scope and conventions have now been source-checked.

## Fixture normal vectors and perimeter

The following is a direct check of [pentagon_fixture.py](pentagon_fixture.py),
in its original Euclidean coordinates, before the verifier's `y/τ` change.
Write `s=√5`, `τ=tan(π/5)=√(5−2s)`, and `h=cos(π/5)=(1+s)/4`.
The fixture's five `Q` vertices are

\[
 q_i=(\cos(2\pi i/5),\sin(2\pi i/5)),\quad 0\leq i<5.
\]

Its five vectors `A_i` are exactly

\[
 A_i={n_i\over h},\qquad
 n_i=(\cos((2i+1)\pi/5),\sin((2i+1)\pi/5)).             \tag{1}
\]

For example `A₀=(1,τ)`,
`A₁=(-(3−s)/2,τ(1+s)/2)`, and `A₂=(-(s−1),0)`;
the remaining two follow by reflection. These agree with the explicit
source arrays by elementary pentagon trigonometry. The side
`[q_i,q_(i+1)]` lies on `n_i·x=h`, and all other vertices satisfy the
strict lower inequality. Therefore `A_i` points along the corresponding
outward facet normal and

\[
 h_Q(A_i)=1.
\]

For each selected triple, the fixture solves

\[
 \sum_k\beta_k A_{i_k}=0,\qquad \sum_k\beta_k=1,
 \qquad \beta_k>0.                                    \tag{2}
\]

It constructs successive edges `β_k A_(i_k)`. Equation (2) closes the
curve, and positive homogeneity of the support function gives perimeter
`Σ_k h_Q(β_k A_(i_k))=Σ_k β_k=1`. Subtracting the vertex centroid changes
only the translation. Thus every constructed template meets the source
definition with exactly the intended normalization.

## Completeness of the ten translation classes

Conversely, any nondegenerate normal triangle has three distinct outward
normal directions. Write its directed edges as `β_k A_(i_k)` with positive
coefficients. Its closure and unit perimeter give precisely (2).
Any three distinct `A_i` are affinely independent, since they are distinct
points on one circle. Hence the three-by-three system in (2) has a unique
solution. There is no continuous parameter omitted by the fixture.

The positive triples are exactly

\[
 \{0,1,3\},\ \{0,2,3\},\ \{0,2,4\},\
 \{1,2,4\},\ \{1,3,4\}.
\]

Indeed, a triple of these equally spaced directions contains the origin
in its triangle precisely when its cyclic index gaps are `1,2,2` in some
order. All other triples have a gap of three steps and lie in an open
semicircle, so cannot admit strictly positive closure coefficients.
This also agrees with the fixture's exhaustive `combinations(range(5),3)`
and exact positivity tests.

For a fixed triple, the six orders of its three edges give two orders
modulo cyclic permutation. Cyclic permutation only changes the starting
vertex and therefore the translation class. The fixture explicitly
constructs both representatives `(0,1,2)` and `(0,2,1)`. Both are needed:
they are reflected triangles up to translation. Finally, no two normal
directions are opposite, because their angle differences are multiples of
`2π/5`, never `π`. Thus there are no missing degenerate segment templates.
This proves that the ten stored templates exhaust the source's family.

## Scaling, capacity identification and consequence

For `λ>0`, the map `D_λ(q,p)=(λq,p)` factors as the uniform dilation
by `√λ` after the symplectic map

\[
 S_\lambda(q,p)=(\sqrt\lambda q,p/\sqrt\lambda).
\]

Symplectic invariance and degree-two conformality of EHZ capacity give

\[
 c_{\mathrm{EHZ}}((\lambda K)\times_L Q)
      =\lambda c_{\mathrm{EHZ}}(K\times_L Q).           \tag{3}
\]

These are the usual capacity axioms; equivalently, `D_λ` multiplies the
symplectic form and the action form `p·dq` by `λ`.

Let `C_T` be the geometric cover functional defined in the continuum
review. For every `λ>0`, the imported criterion, the just-proved template
completeness, and (3) imply

\[
 \begin{aligned}
 &\text{each }\lambda T_j\text{ fits into }K\\
 &\quad\Longleftrightarrow
   \text{each }T_j\text{ fits into }K/\lambda\\
 &\quad\Longleftrightarrow
   c_{\mathrm{EHZ}}((K/\lambda)\times_L Q)\geq1\\
 &\quad\Longleftrightarrow
   c_{\mathrm{EHZ}}(K\times_L Q)\geq\lambda.
 \end{aligned}
\]

Taking suprema yields `C_T(K)=c_EHZ(K ×_L Q)`. The continuum review
already proves `C_T(H)=1` from the exposed difference-pair certificate
and, locally, `C_T(K)²/A(K)≤1/A(H)`, with equality precisely for translates
and positive homothets of `H`. Consequently the same local conclusion
holds for `c_EHZ(K ×_L Q)²/(2A(K)A(Q))`. Reflection of the template bank
gives the corresponding conclusion at `−H`.

Before this identification, undo the finite verifier's map
`L(x,y)=(x,y/τ)` on the entire geometric covering statement. It transfers
containment, area inequalities and neighborhoods by an invertible planar
map; it is not a claimed symplectic symmetry on one factor. The fixture's
pentagon orientation also differs by a common rotation from the charter's
displayed convention. Applying the same orthogonal rotation to both
factors is symplectic, so this is a coordinate convention, not a change
in the capacity problem.

This closes the applicability check relative to the stated published
theorem. It does not independently establish that theorem's proof,
global optimality, novelty, human acceptance or manuscript readiness.
