# Independent check: fixed regular pentagon, fixed partner normals

Checked 2026-10-04 23:09 UTC by product_covariance. Receiving/integrating owner:
geometry_discovery_lead. This is an independent mathematical derivation within
an **open support chamber of genuine planar polygons with fixed normals**.
No ambient four-dimensional normal-chart claim, global optimization claim,
novelty claim, or implementation certificate follows.

**Verdict:** the finite linear envelope and rank necessity are valid. The
stronger full-interior convex-hull criterion is also valid and characterizes
local maxima in this chart. The bound of twelve partner edges needs the
additional unique-optimal-dual premise; it is not an unconditional edge bound.

## 1. Setup and precise completeness input

Fix a regular planar pentagon Q and use the Lagrangian product K×Q. Let
T₁,…,T₁₀ be its normal triangles of unit Q-perimeter, with orientations fixed;
they may be translated but not freely rotated or reflected while fitting.

The external completeness input is
[Balitskiy–Mitrofanov–Polyanskii, arXiv:2603.12495v1](https://arxiv.org/html/2603.12495v1),
Definition 3.2, Theorem 3.3, and Section 6.1: capacity at least one is equivalent
to fitting every normal triangle by translation; regular Q has ten such classes,
all genuine triangles. This input is used at arbitrary positive scale below.
The normal-triangle list can also be counted directly: five positive closure
triples of pentagon facet normals, with two cyclic edge orders each. There are
no antipodal normal pairs and hence no segment templates for this Q.

Let

\[
 K(h)=\{x\in\mathbb R^2:n_i\cdot x\le h_i,
                         \quad i=1,\ldots,N\},\qquad
 \mathsf N=[n_1\ \cdots\ n_N]\in\mathbb R^{2\times N}.
\]

Normals are fixed, nonzero, cyclically ordered, distinct directions and positively
span R². The base support h₀ belongs to an open chamber where every inequality
supports a genuine positive-length edge. Adjacent normal gaps lie in (0,π).
Thus rank(𝖭)=2. Supports and normals need not be normalized to unit length.
Translations of K send h to h+𝖭ᵀu; positive dilation sends h to sh.

Write A(h)=area(K(h)), A_Q=area(Q), and c(h)=c_EHZ(K(h)×Q)>0. One-factor
homogeneity is degree one, so sys(h)=c(h)²/(2A_Q A(h)).

## 2. LP duality, compactness, and finite circuits

Set sᵢⱼ=h_{Tⱼ}(nᵢ). Containment of a positive homothetic copy is exactly

\[
 n_i\cdot u+r s_{ij}\le h_i\quad\text{for all }i.
\]

The maximum feasible r is positive and finite: K has interior and each Tⱼ is
bounded with positive diameter. Positive r feasibility is strict for small r.
The fitting LP can take r free; the optimum is positive, and the free-r dual is

\[
 \lambda_j(h)=\min_{a\in D_j}a\cdot h,\qquad
 D_j=\{a\ge0:\mathsf Na=0,\ s_j\cdot a=1\}.
\]

Taking r≥0 instead initially gives sⱼ·a≥1; positivity of the optimum permits
the same normalized equality formulation. Finite feasible primal optimum and
linear-programming duality give a nonempty dual and equality of optimal values.

**Compactness holds for this selected Q.** Translate each genuine triangle so
that 0 lies in its interior. Then for some ρⱼ>0,
`sᵢⱼ≥ρⱼ||nᵢ||>0`. Translating Tⱼ by z replaces sⱼ by sⱼ+𝖭ᵀz, which does
not change Dⱼ because 𝖭a=0. The constraint sⱼ·a=1 consequently bounds every
coordinate of a. Dⱼ is a nonempty compact polytope.

Each vertex has at most three positive coordinates. If its positive support
had more columns than the rank of the three-row matrix `[𝖭;sⱼᵀ]`, a nonzero
null vector on that support would allow a small two-sided nonnegative feasible
perturbation. In fact, vertices are normalized positive minimal closure circuits
of 𝖭: their supports are antiparallel pairs or positive noncollinear triples.
A collinear/repeated-direction triple is not a genuine three-coordinate vertex;
singular three-column systems must not be inverted blindly. Pair vertices and
zero-coordinate degenerate basic solutions must be retained. The normal closure
cone has finitely many circuit rays; normalization by sⱼ fixes their scales.

Let Bⱼ=vertices(Dⱼ), and let B be the union of all ten Bⱼ, preserving distinct
vectors and any template ownership needed for interpretation. Then

\[
 \lambda_j(h)=\min_{a\in B_j}a\cdot h,\qquad
 c(h)=\min_{1\le j\le10}\lambda_j(h)
     =\min_{a\in B}a\cdot h. \tag{1}
\]

To justify the second equality geometrically, for each r>0 the completeness
input applied to K/r says c(h)≥r exactly when every rTⱼ fits into K.
That means r≤λⱼ(h) for **all ten** j. Taking the maximal common r proves (1).
One fitting bottleneck alone provides an upper bound, not a full capacity value.

This is a single finite concave piecewise-linear formula in h. The list B is
independent of h because normals and templates are fixed. Duplicate vectors
can be deduplicated mathematically, but ties between distinct vectors cannot
be discarded. Choosing one optimal LP basis per template is insufficient when
that template has multiple optimal duals.

The formula respects translations since a·𝖭ᵀu=0, and respects one-factor
dilation since a·(sh)=s(a·h). Rescaling a normal and its support by a positive
factor rescales the corresponding dual coordinate inversely, preserving the
closure equations and objective. Hence no unit-normal premise is hidden here.

**Extension hazard only:** compactness would fail for some segment templates.
For T=[−e₁,e₁] and square normals e₁,−e₁,e₂,−e₂, the dual permits
`a₁=a₂=1/2` and arbitrary `a₃=a₄≥0`. This does not occur for the ten regular
pentagon triangles. A more general segment-template argument can use pointed
polyhedra and coercive objectives rather than assert compactness.

## 3. Strict planar area Hessian, with translations removed

The external niche input is the planar support-area signature stated explicitly
in [Fillastre–Izmestiev, arXiv:1310.1560](https://arxiv.org/pdf/1310.1560),
Section 2.1.2, printed page 32 (PDF page 32). The area quadratic form has
signature `(+,0,0,−,…,−)`. The following calculation identifies its two-dimensional
kernel and the precise strictness needed here.

For unit normals let αᵢ be the gap from normal i to i+1, cyclically. Edge lengths
are linear functions of supports d:

\[
 \ell_i(d)=\frac{d_{i-1}-d_i\cos\alpha_{i-1}}{\sin\alpha_{i-1}}
          +\frac{d_{i+1}-d_i\cos\alpha_i}{\sin\alpha_i}.
\]

There is therefore a symmetric matrix M with `ℓ(d)=Md` and

\[
 A(d)=\tfrac12d^TMd,\qquad DA(d)=Md,\qquad D^2A=M.
\]

M has adjacent off-diagonal entries csc(αᵢ) and diagonal entry
`−cot(αᵢ₋₁)−cot(αᵢ)`. Its kernel consists exactly of translations: if all formal
edge lengths ℓᵢ(v) vanish, consecutive intersections of the lines
`nᵢ·x=vᵢ` coincide, so all lines pass through one point u and
`vᵢ=nᵢ·u`. The converse follows from translation invariance.

For nonunit normals put L=diag(||nᵢ||), d=L⁻¹h. The support-area Hessian is
`H=L⁻¹ML⁻¹`; it has the same inertia and kernel im(𝖭ᵀ). In particular

\[
 DA(h_0)[v]=0,\quad v\notin\operatorname{im}(\mathsf N^T)
 \quad\Longrightarrow\quad D^2A[v,v]<0. \tag{2}
\]

Indeed A(h₀)>0 makes h₀ a positive vector for the Lorentzian quadratic form.
Its bilinear orthogonal complement is negative definite after quotienting the
translation kernel. Equation (2) concerns the **area-gradient hyperplane**;
the area Hessian is not negative definite on every direction or on the entire
translation quotient. The positive scaling direction remains.

## 4. Necessary full rank of all active dual vectors

Let c₀=c(h₀) and

\[
 B_0=\{a\in B:a\cdot h_0=c_0\},\qquad
 E=\operatorname{span}(B_0)\subseteq\ker\mathsf N.
\]

The finite inactive branch list has a positive gap at h₀. If a vector v is
orthogonal to every a∈B₀, then (1) gives c(h₀+sv)=c₀ for all sufficiently
small positive and negative s. These perturbations remain in the open chamber.
At a local maximum of sys, A restricted to that constant-capacity line must
have a local minimum, so DA(h₀)[v]=0. If v is not a translation, (2) makes
A(h₀+sv)<A(h₀) for all sufficiently small nonzero s, a contradiction.
Thus E⊥ contains only translations, and hence

\[
 \boxed{\operatorname{span}(B_0)=\ker\mathsf N,\qquad
        \operatorname{rank}(B_0)=N-2.} \tag{3}
\]

This is a necessity for even a non-strict local maximum. Mere coverage of all
N edges is weaker. A single positive equilibrium combination of active vectors
is also weaker than rank (3).

## 5. Stronger exact local-maximum criterion

Define normalized active vectors and normalized area gradient

\[
 w_a=a/c_0\ (a\in B_0),\qquad
 t=\frac{DA(h_0)}{2A(h_0)},\qquad
 \mathcal H=\{w:\mathsf Nw=0,\ h_0\cdot w=1\}.
\]

Translation invariance and Euler's identity put both wₐ and t in 𝓗.
Since 𝖭 has rank two and h₀ is not in im(𝖭ᵀ), dim(𝓗)=N−3.

**Theorem.** The following are equivalent:

1. h₀ is a local maximum of sys in its fixed-normal open chamber.
2. `t∈int_𝓗 conv{wₐ:a∈B₀}`, where interior is taken in the **full** affine
   hyperplane 𝓗, not merely relative to the hull's own affine span.
3. Active vectors have rank N−2 and t is a convex combination of all the
   distinct normalized active vectors with strictly positive coefficients.

When these conditions hold, the local maximum is strict modulo translations
and positive dilation. For N=3, 𝓗 is a singleton and the fixed-normal chart
consists entirely of these symmetries; strictness in its zero-dimensional shape
quotient is vacuous.

### Necessity

By (1), the directional derivative of log(sys) at h₀ is

\[
 D_v\log\operatorname{sys}
 =2\min_{a\in B_0}(w_a\cdot v)-2t\cdot v.
\]

If t lies outside the active convex hull, strict separation supplies v with
`min wₐ·v>t·v`, contradicting local maximality. Thus t belongs to that hull.

Suppose t fails to lie in its interior in the full 𝓗. A supporting/separating
functional on 𝓗 exists, nonconstant on 𝓗, with all active values at least its
value at t. Represent it by v and replace v by
`v−(t·v)h₀`. Since every w∈𝓗 satisfies h₀·w=1, this gives

\[
 t\cdot v=0,\qquad w_a\cdot v\ge0\ \forall a\in B_0,
 \qquad\min_{a\in B_0}w_a\cdot v=0.
\]

The last equality uses that t is a convex combination of active w's.
Nonconstancy of the functional means v is not a translation. In the deficient
hull case one can equivalently take a nonzero functional vanishing on that
hull and on t. For sufficiently small s≥0, finite inactive gaps give
`c(h₀+sv)=c₀` exactly. Meanwhile DA(h₀)[v]=0 and (2) give a strict area decrease.
This contradicts local maximality and proves the full-interior requirement.

### Sufficiency

Take the local feasible displacement cone

\[
 C=\{v:a\cdot v\ge0\text{ for every }a\in B_0\}.
\]

Near h₀, by finite inactive gaps, `c(h₀+v)≥c₀` is equivalent to v∈C.
A full-interior convex combination gives `t·v>0` for every v∈C that is not a
translation: equality would force all active values to vanish, and (3) would
make v a translation. This strict positivity is uniform on the unit sphere
of C modulo translations, which is compact. Thus for some δ>0,
`DA(h₀)[v]≥δ||v||` on the translation quotient cone. The quadratic remainder
is O(||v||²), so A(h₀+v)>A(h₀) for every sufficiently small nontranslation
feasible v. If the quotient cone's unit section is empty, the conclusion is
immediate.

For any nearby h, normalize it by

\[
 h'=\frac{c_0}{c(h)}h.
\]

Continuity and positivity give h' near h₀, with c(h')=c₀ and unchanged sys.
The local area minimum just proved then gives strictly smaller sys unless
h' differs from h₀ by a translation. That exception is exactly translation
and positive dilation of h₀. This proves sufficiency and the strictness claim.

The equivalence of conditions 2 and 3 is finite convex geometry: full affine
span is equivalent to rank N−2 because every normalized point lies on
`h₀·w=1`; an interior point admits a convex combination assigning positive
mass to every point in the finite list. Conversely such a combination with
full affine span is interior.

## 6. What the edge bound does and does not say

Let r be the number of templates with λⱼ(h₀)=c₀, so 1≤r≤10. If **each active
template has a unique optimal dual vector**, it contributes exactly one vector
to B₀. Then (3) gives

\[
 N-2\le|B_0|\le r\le10,\qquad\boxed{N\le r+2\le12}. \tag{4}
\]

Unique optimum is stronger than returning one optimal vertex from a solver.
If an optimal face has positive dimension, all its vertices contribute active
branches; counting one per template loses the rank obligation. Multiple primal
placements do not by themselves imply multiple duals, and multiple primal
active constraints do not by themselves imply multiple duals either.

Without uniqueness, r alone gives no bound on the number of active vectors.
The correct unqualified count is `N≤|B₀|+2`. Generic supports usually avoid the
finite dual tie hyperplanes, but a local maximizer can lie precisely on those
hyperplanes. Generic uniqueness is not a substitute for checking uniqueness
at the candidate. Moreover rank alone cannot certify a local maximum: the
normalized area gradient must lie in the full interior, as in Section 5.

## 7. Premises that must survive integration

- K must have N genuine edges and lie in an open fixed-normal support chamber.
  Redundant halfspaces, duplicate facets, zero edge lengths or a chamber boundary
  invalidate the unrestricted two-sided support variations used above.
- Capacity must use all ten triangles with the correct unit-Q-perimeter scaling
  and both cyclic order classes. A proper subcatalogue supplies upper bounds.
- Every optimal dual vertex of every active template must be included, including
  pair supports and degenerate bases; floating solver basis selection is not
  completeness or exact uniqueness evidence.
- Negative area curvature applies on DA=0 after removing translations; it is
  false on the full support quotient because scaling has positive curvature.
- The conclusion concerns varying supports of K with Q and partner normals
  fixed. Moving normals, changing Q, adding edges, arbitrary convex partners,
  or ambient four-dimensional perturbations require separate arguments.
- The local equivalence does not prove a global sharp ratio or exclude all
  competitors with more than twelve edges: (4) has a uniqueness premise and
  global minimizers could occur on degeneracy loci.

No broad producer or paid API was used. External theorem inputs were read in
primary sources. The rank and full-interior implications were independently
reconstructed from the finite LP envelope and planar area form. Numerical
capacity or rank diagnostics are not used as premises of this note.

## 8. Exact positive control: every regular partner in its own fixed fan

The stronger criterion gives a nonvacuous corollary without computing a
capacity value:

**Corollary.** Every regular pentagon K, at any relative orientation to the
fixed regular pentagon Q, is a strict local maximum of sys in K's own
fixed-normal support chart, modulo translations and positive dilation.

To verify this, use unit normals and put h₀=b·1, b>0. Rotational symmetry makes
all five edges equally long, so

\[
 t=\frac{DA(h_0)}{2A(h_0)}=\frac1{5b}(1,1,1,1,1)=:w_0.
\]

For five equally spaced normal directions, ker(𝖭) is the direct sum of the
constant Fourier mode and the real two-dimensional mode with frequencies ±2.
The affine hyperplane 𝓗 has fixed constant component w₀ and tangent equal to
that two-dimensional mode.

Choose any active dual vertex a. It has at most three positive coordinates,
so w=a/c₀ is nonuniform and w−w₀ is nonzero. Rotation through 2π/5 preserves
both regular pentagons and permutes all ten Q-template classes. It cyclically
permutes K's five facets and transports an active dual vertex to another active
dual vertex. Consequently all five cyclic shifts of w are active.

On the ±2 Fourier mode, cyclic shift is rotation through ±4π/5. The orbit of
the nonzero vector w−w₀ is the vertex set of a regular pentagon in that plane,
with zero strictly inside its convex hull. Hence w₀ belongs to the interior
in the full 𝓗 of the active normalized-dual hull. Section 5 proves the corollary.

This does not assert maximality under relative rotations, changes of either
normal fan, or arbitrary planar perturbations. A relative orientation having
suboptimal capacity among rotations can still be locally maximizing when its
normals are held fixed. The corollary is a check of the local-chart criterion,
not a global sharp inequality or a new capacity formula. Novelty is unknown.
