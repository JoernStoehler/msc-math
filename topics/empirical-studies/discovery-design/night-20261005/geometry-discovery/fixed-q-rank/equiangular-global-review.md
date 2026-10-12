# Independent consequential review: equiangular pentagon capacity core

Reviewed 2026-10-04 23:27 UTC by product_covariance. Receiving/integrating owner:
geometry_discovery_lead/root. Reviewed source:
[equiangular-capacity-core.md](equiangular-capacity-core.md), with the imported
regular-product formula checked against
[topics/products/pentagon-affine-products/research.tex](../../../../../products/pentagon-affine-products/research.tex).
This review does not promote a thesis theorem or assert literature novelty.

**Verdict: passes.** The capacity-core identity, all-angle sharp global bound,
and equality classification hold for every full-dimensional bounded body
represented using a subset of the prescribed five equally spaced normal
directions. Genuine five-edge equiangular pentagons and collapsed-edge
triangles/quadrilaterals are included. The conclusion does not cover arbitrary
pentagons, arbitrary partners, or ambient four-dimensional perturbations.

## 1. Independently reconstructed theorem

Let Q be the regular circumradius-one pentagon with unit outward normals
`n_i=(cos(2πi/5),sin(2πi/5))`. Set `b=cos(π/5)`, `w=1+b`,
`Pθ=RθQ`, `m_i=Rθn_i`, and

\[
 K_h=\{x:m_i\cdot x\le h_i\quad(i=0,\ldots,4)\}.
\]

Assume K_h is bounded and has nonempty interior. Its inequalities may be
redundant. Let ιθ(K_h) be the maximum t≥0 such that a translate of tPθ is
contained in K_h. Then

\[
 \boxed{c_{\rm EHZ}(K_h\times_LQ)
       =c_{\rm EHZ}(P_\theta\times_LQ)\,\iota_\theta(K_h).} \tag{1}
\]

With `d=dist(θ,(π/5)Z)∈[0,π/10]`, the imported regular-product formula gives

\[
 c_{\rm EHZ}(P_\theta\times_LQ)=w^2\sec d,
 \qquad
 \operatorname{sys}(K_h\times_LQ)
 \le\frac{5+2\sqrt5}{10}\sec^2d. \tag{2}
\]

At a fixed angle, equality holds exactly for translates and positive dilates
of Pθ. Across all angles the upper bound is `(3+√5)/5`, with equality exactly
when K_h is regular and `d=π/10`, equivalently `θ=π/10 mod π/5`.

## 2. Five circuits: completeness, weights, and one orbit

Consider

\[
 C=\{\lambda\ge0:\textstyle\sum_i\lambda_i=1,
                         \sum_i\lambda_i m_i=0\}.
\]

A vertex has at most three positive coordinates, by linear dependence in its
three scalar constraints. No single normal is zero, no pair is antipodal,
and three distinct circle points are affinely independent. Thus every vertex
uses exactly three normals with strictly positive coefficients.

For a triple of normals, positive closure is equivalent to the origin lying
inside their triangle, equivalently every cyclic angular gap is less than π.
The three positive integer index gaps sum to five and must each be at most
two. They must therefore be `(1,2,2)` in some order. Exactly five triples have
this property; they are the cyclic shifts of `{0,1,3}`. The other five triples
have a gap of three and cannot close positively. There are no missing boundary
pair circuits because there are no antipodal pairs.

The coefficient identity is exact:

\[
 n_0+n_1=-2b\,n_3,\qquad
 \lambda^{(0)}=\frac{(1,1,0,2b,0)}{2(1+b)}.
\]

The coefficients sum to one and close the normals. All five vertices
λ⁽ˡ⁾ are its cyclic shifts. Applying Rθ changes no closure coefficient.
Thus the entire ray catalogue is one C₅ orbit at every angle.

## 3. Ten templates, positive denominators, and the min/max direction

The completeness input is
[Balitskiy–Mitrofanov–Polyanskii, arXiv:2603.12495v1](https://arxiv.org/html/2603.12495v1),
Definition 3.2, Theorem 3.3, and Section 6.1. It identifies capacity ≥1 with
fitting every normal unit-Q-perimeter triangle by translation, and states that
regular Q has ten such classes. This criterion includes polygonal boundaries.
The ten classes comprise the two cyclic edge orders for each of Q's five
positive normal triples; they are all genuine triangles.

For template Tⱼ put `s_ij=h_Tj(m_i)`. The exact fitting dual is

\[
 D_j=\{a\ge0:\textstyle\sum_i a_i m_i=0,
                         \sum_i a_i s_{ij}=1\}.
\]

After translating Tⱼ so zero is interior, all sᵢⱼ are strictly positive.
This translation leaves Dⱼ unchanged because a closes the normals. Therefore
Dⱼ is compact and is a positive transverse section of the nonnegative normal
closure cone. Its complete vertex set is

\[
 a^{(j,l)}=\frac{\lambda^{(l)}}{d_{jl}},\qquad
 d_{jl}=\sum_i\lambda_i^{(l)}h_{T_j}(m_i)>0.
\]

The numerators `λ⁽ˡ⁾·h` are also strictly positive: translate an interior point
of K_h to zero, making every raw inequality support strictly positive, without
changing a closed weighted sum. Redundant inequalities cause no exception.

Consequently the complete minimum over all ten fits is

\[
 c(K_h\times_LQ)
 =\min_{j,l}\frac{\lambda^{(l)}\cdot h}{d_{jl}}
 =\min_l\frac{\lambda^{(l)}\cdot h}{D_l(\theta)},
 \qquad D_l(\theta)=\max_j d_{jl}. \tag{3}
\]

The **maximum** denominator is required because its positive numerator is
fixed for a given l. Replacing it with a minimum would reverse the capacity
bottleneck and invalidate the theorem.

The ten template classes are invariant under R₂π/₅. To check the index action
explicitly, cyclically shifting λ once changes its weighted sum into

\[
 \sum_i\lambda_i h_{T_j}(R_{2\pi/5}m_i)
 =\sum_i\lambda_i h_{R_{-2\pi/5}T_j}(m_i).
\]

The rotated template belongs to the same complete ten-class family; any
translation of its representative drops out under closure. Thus taking the
maximum over all ten j gives `D_{l+1}(θ)=D_l(θ)` for every real θ. This argument
uses commutation of planar rotations and does not require θ to be a special
angle. Call the common value Dθ>0. Equation (3) becomes

\[
 c(K_h\times_LQ)=\frac{\min_l\lambda^{(l)}\cdot h}{D_\theta}. \tag{4}
\]

No HK word coefficient κ occurs in this proof. If the imported regular-product
formula is expressed through the HK maximum, it is
`Q_max=cos(d)/(2w²)>0`, consistently with `c=1/(2Q_max)`. Neither a zero
coefficient branch nor a negative denominator is being used.

## 4. The inner regular pentagon has exactly the same dual rays

Containment `z+tPθ⊆K_h` is equivalent to

\[
 m_i\cdot z+bt\le h_i\quad\text{for all }i.
\]

Farkas' lemma gives feasibility at a proposed t exactly when
`λ·h−bt≥0` for every λ∈C. It suffices to test C's five vertices. Equivalently,
the scale/translation LP has dual `a≥0`, `Σa_i m_i=0`, `bΣa_i=1`, giving

\[
 \iota_\theta(K_h)=\frac{\min_l\lambda^{(l)}\cdot h}{b}. \tag{5}
\]

No support-chamber or positive-edge-length hypothesis was used. Every raw
halfspace enters the same containment LP, including redundant ones.

The maximum is attained. Because 0∈Pθ, a feasible z lies in K_h; because Pθ has
positive diameter, feasible t is bounded by `diam(K_h)/diam(Pθ)`. The feasible
set in bounded `(z,t≥0)` coordinates is closed and nonempty, hence compact.
An interior ball of K_h accommodates a sufficiently small positive copy, so
the attained maximum is positive.

At K_h=Pθ, all hᵢ=b and `min_l λ⁽ˡ⁾·h=b`. Hence (4) reads
`c(Pθ×_LQ)=b/Dθ`. Combining this with (4) and (5) proves (1) independently of
any formula for the regular-product capacity.

## 5. Sharp area inequality and equality

Choose an attained placement `z+ιθ(K_h)Pθ⊆K_h`. Area monotonicity gives

\[
 A(K_h)\ge\iota_\theta(K_h)^2A(P_\theta).
\]

Insert identity (1) into `sys=c²/(2A(K_h)A(Q))` to obtain (2). Both bodies
have interior. A proper containment of compact convex bodies in the plane
has strictly positive area difference: an exterior point of the smaller body,
together with an interior ball, creates a positive-area wedge beyond a
separating support line. Thus area equality forces equality of the bodies,
not merely equality of an area statistic. The equality class is exactly
translation and positive dilation of Pθ. Conversely those operations give
equality by homogeneity and translation invariance.

The imported regular capacity formula has the same convention:
`m(Rθ)=max_ij|n_i·Rθn_j|=cos d`, because signed normal pairings have phase
spacing π/5. Hence `c_reg=w² sec d`. As `d∈[0,π/10]`, sec²d is maximized
exactly at d=π/10; its value gives `(3+√5)/5`. The benchmark equality class
therefore has no additional nonregular support vectors or edge-loss bodies.

## 6. Independent exact aligned-angle coefficient check

I independently checked all fifty denominator values `d_jl` in Q(√5), using
exact comparison rather than numerical max selection. Scratch command:

`uv run --with sympy python /tmp/msc-math/equiangular-global-review/check_denominators.py`

Coordinates `(X,Y)=(x,y/τ)`, `τ=tan(π/5)`, transform covectors and triangle
vertices contragrediently. The listed five transformed covectors in the source
are correct. The canonical triangle
`conv{(-R,0),(R,0),(0,R)}`, `R=(5−√5)/10=1/(2w)`, has physical edge directions
`n₀,n₂,n₃`. Their total Q-length is `2R(b+1)=1`. Rotation by π/5 has the source's
matrix

\[
 \begin{pmatrix}b&-(3\sqrt5-5)/4\\b&b\end{pmatrix},
\]

which follows directly by conjugating physical rotation by diag(1,τ).
Its ten powers produce the full ten translation classes. The check verifies
rotation closure, each normal closure identity, strict positivity of every
one of the fifty denominators, and all five exact maxima:

| Circuit shift l | Maximizing template indices j | D_l(0) |
| --- | --- | --- |
| 0 | 1, 6 | (√5−1)/5 |
| 1 | 3, 8 | (√5−1)/5 |
| 2 | 0, 5 | (√5−1)/5 |
| 3 | 2, 7 | (√5−1)/5 |
| 4 | 4, 9 | (√5−1)/5 |

The exact identity `b/w²=(√5−1)/5` independently matches the source's aligned
coefficient. At other angles, calibrating (4) on Pθ and using the imported
regular capacity gives

\[
 D_\theta=\frac{b\cos d}{w^2}>0.
\]

This fifty-evaluation check corroborates the coefficient; the all-angle result
rests on circuit completeness and exact symmetry, not interpolation from
θ=0 or floating-angle sampling. No body-vertex enumeration or decagon search
was performed. Scratch code is not the retained theorem's sole basis.

## 7. Scope and mechanism boundaries

The theorem applies to a body specified by the regular five-direction fan,
including edge-loss boundaries. It does not assert that arbitrary pentagons
have such normals, nor that changing normals or adding directions preserves
the capacity core. Within the fan it provides a global result rather than only
an open-chamber local result. It does not require unique optimal fitting duals
or differentiability of capacity.

The one-orbit argument is special. A regular hexagonal normal fan has three
antipodal-pair rays and two alternating-triple rays, two different orbit types;
even a C₆-invariant template family would only force equal normalizations
within each type. Against the selected fixed Q=P₅, rotation by π/3 also fails
to preserve Q's ten-template family. Thus no automatic six-normal capacity-core
extension follows from this proof. This is a premise boundary, not a new task
or a counterexample to any six-normal theorem.

The older prescribed ten-direction theorem at the special HKO orientation
must remain attributed to its retained September owner. This review verifies
an all-angle five-direction capacity-core relation and the resulting sharp
bound; it makes no broader literature novelty claim. The independent-affine
regular-pentagon formula is an imported project result, explicitly identified
above. No new paid API or broad experiment was used.
