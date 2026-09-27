# A rational non-HKO local maximum with ten genuine facets

## 1. Statement and scope

Use coordinates `(q1,q2,p1,p2)` and the symplectic matrix

\[
\Omega=\begin{pmatrix}0&I_2\\-I_2&0\end{pmatrix}.
\]
For a matrix of ten row normals define
\(P(A)=\{x\in\mathbb R^4:Ax\leq\mathbf1\}\), and put
\(\rho(P)=c_{\rm EHZ}(P)^2/(2\operatorname{Vol}_4(P))\).

**Theorem.** Let

\[
A_*=
\begin{pmatrix}
0&2&0&0\\
12/7&4/7&0&0\\
12/7&-4/7&0&0\\
0&-2&0&0\\
-12/7&-4/7&0&0\\
-12/7&4/7&0&0\\
0&0&4/3&2\\
0&0&-4&2\\
0&0&-4&-2\\
0&0&4/3&-2
\end{pmatrix}.
\]

Then \(P_*=P(A_*)\) is a bounded, full-dimensional, simple polytope with exactly
ten genuine facets and 24 vertices. Its volume is \(1/2\), its EHZ capacity
is 1, and its systolic ratio is 1. There exists a Hausdorff neighborhood
\(\mathcal U\) of \(P_*\) such that

\[
P\in\mathcal U,\quad P\text{ has exactly ten genuine facets}
\quad\Longrightarrow\quad \rho(P)\leq1.
\]

In particular, this is a full-body local maximum, not merely a maximum within
products or another symmetry-restricted family. It is non-strict: a smooth
local two-parameter family of equality bodies remains after quotienting by
translation, positive scaling, and linear symplectic transformations. The body
is not equivalent to HKO, including by an anti-symplectic transformation.

This is a computer-assisted proof using exact rational arithmetic and rational
function identities, plus the elementary local argument below. The finite
witness is `certificate/certificate_base.json`; `python verify.py` recomputes
all the stated finite predicates. The proof is not a formalization in a proof
assistant. A numerical value of the neighborhood radius is not asserted.

## 2. Geometry, boundedness, and the local normal chart

The body is the Lagrangian product \(H_*\times T_*\), where the hexagon, in
clockwise order, has vertices

\[
(-5/12,1/2),\ (5/12,1/2),\ (7/12,0),\ (5/12,-1/2),\
(-5/12,-1/2),\ (-7/12,0),
\]

and the quadrilateral, in counterclockwise order, has vertices

\[
(3/4,0),\ (0,1/2),\ (-1/4,0),\ (0,-1/2).
\]

Their areas are 1 and \(1/2\). Each displayed edge lies on the corresponding
row inequality of \(A_*\), and the other vertices lie on the correct side.
The 24 Cartesian-product vertices are feasible. Every vertex has exactly four
active facets, with independent active normals; every row supports a
three-dimensional facet. The verifier independently enumerates all
\(\binom{10}{4}=210\) four-row intersections and recovers exactly this vertex set.

For a direct boundedness check, the vector

\[
b=(1/12,1/12,1/12,1/12,1/12,1/12,3/16,1/16,1/16,3/16)^T
\]

has strictly positive entries, \(\sum b_i=1\), and \(A_*^Tb=0\), while
\(\operatorname{rank}A_*=4\). Hence the origin is in the interior of the convex
hull of the ten normals, and \(P(A_*)\) is bounded. Its own interior contains
the origin because all its right-hand sides are 1. These checks remove any
reliance on a numerical hull or a padded facet presentation.

The simple realization chamber is open in the 40-dimensional space of labelled
normal rows. Active four-row inverses and strict inequalities persist there.
Volume is real analytic in this chamber: use the fixed pulling triangulations
of the three-dimensional facets and cone them to the origin. If \(X_r(A)\) are
the rational vertex functions, this gives

\[
V(A)=\frac1{24}\sum_{(r_1,r_2,r_3,r_4),\epsilon}
\epsilon\det\begin{pmatrix}X_{r_1}(A)\\X_{r_2}(A)\\X_{r_3}(A)\\X_{r_4}(A)\end{pmatrix},
\]

where the orientation signs are fixed at the base. The triangulation and its
incidences are recomputed and retained in `evidence/exact/certificate_base_exact.json`.

This normal chart also covers a full fixed-ten-facet Hausdorff neighborhood.
Indeed, sufficiently close bodies contain the origin, and polarity preserves
local Hausdorff convergence for such bodies. The ten vertices of \(P_*^\circ\)
have disjoint small exposed neighborhoods. A close polar with exactly ten
vertices must have at least one vertex in each, and therefore exactly one in
each. Relabel those vertices to obtain normals close to \(A_*\). Thus the normal
chart does not omit nearby ten-facet bodies or impose a product restriction.

## 3. The capacity formula and the direction of the bound

The external symplectic input is the Haim-Kislev formula [HK, Theorem 1.1].
For a word \(\sigma=(\sigma_1,\ldots,\sigma_m)\) of distinct facet labels, put

\[
C_\sigma(A)=\begin{pmatrix}A_\sigma^T\\\mathbf1^T\end{pmatrix},
\qquad e_5=(0,0,0,0,1)^T,
\]

and, when \(C_\sigma\beta=e_5\), \(\beta\geq0\), put

\[
Q_\sigma(A,\beta)=\sum_{i<j}\beta_i\beta_j
 a_{\sigma_i}\Omega a_{\sigma_j}^T.
\]

The orientation is the opposite of one common statement of [HK]; reversing
all words makes the maxima identical. In these conventions,

\[
c_{\rm EHZ}(P(A))=\frac1{2\max_{\sigma,\beta}Q_\sigma(A,\beta)}.
\]

A feasible section with \(Q>0\) therefore gives the **upper**, not lower, bound

\[
\rho(P(A))\leq U_\sigma(A):=\frac1{8V(A)Q_\sigma(A,\beta(A))^2}.\tag{3.1}
\]

We use a complete product computation to prove exact base and equality-family
capacity. Away from the product family, we use only valid upper sections; we
make no completeness assertion about a short-word or selected-branch bank.

## 4. A two-parameter equality family and its exact capacity

For \((v,s)\) near \((1/2,1/12)\), set

\[
u=1-v+v^2,\quad a=1/2-s,\quad b=1/2+s,\quad t=v-1/2.
\]

Let \(H(v,s)\) have clockwise vertices

\[
(-a,1/2),\ (a,1/2),\ (b,t),\ (a,-1/2),\ (-a,-1/2),\ (-b,-t),
\]

and let \(T(v)\) have counterclockwise vertices

\[
(u,0),\ (0,v),\ (u-1,0),\ (0,v-1).
\]

These are genuine convex polygons near the base. Define
\(E(v,s)=H(v,s)\times T(v)=P(A(v,s))\). With
\(D_1=b/2-at\), \(D_2=b/2+at\), its first six normals are the following
vectors embedded in the q coordinates:

\[
(0,2),\quad ((1/2-t)/D_1,2s/D_1),\quad
((1/2+t)/D_2,-2s/D_2),
\]

followed by their negatives in the same order. Its final four normals, in the
p coordinates, are

\[
(1/u,1/v),\quad(1/(u-1),1/v),\quad
(1/(u-1),1/(v-1)),\quad(1/u,1/(v-1)).
\]

Shoelace identities give \(\operatorname{Area}H(v,s)=1\) and
\(\operatorname{Area}T(v)=1/2\), identically. The verifier checks these identities
and the edge-support identities over \(\mathbb Q(v,s)\).

Here is why the small capacity enumeration is complete **on this product
family**. Split any feasible weight vector into q and p blocks, with respective
masses \(r\) and \(1-r\). When the masses are nonzero, divide each block by its
mass to obtain vectors in the planar closure polytopes

\[
\mathcal C_q=\{\gamma\geq0:A_q^T\gamma=0,\ \mathbf1^T\gamma=1\},
\qquad
\mathcal C_p=\{\eta\geq0:A_p^T\eta=0,\ \mathbf1^T\eta=1\}.
\]

Within-block symplectic pairings vanish. For a fixed full ordering, the objective
is \(r(1-r)\) times a bilinear function of \((\gamma,\eta)\). A bilinear function
on a product of compact polytopes attains a maximum at a pair of vertices:
first replace one factor by a maximizing vertex and then the other. Each
planar closure vertex has at most three positive coordinates, by the rank-three
linear constraint system. Thus one need only enumerate pairs of closure
vertices and all orderings of their combined supports. The positive maximum
uses \(r=1/2\); words with nonpositive bilinear value cannot improve it.
The cases \(r=0,1\) have objective zero. Cyclic rotation preserves \(Q\) for a
closed word, so fixing its smallest label first loses no cases.

At the base the q-supports are
`[0,3], [1,4], [2,5], [0,2,4], [1,3,5]`, and the p-supports are
`[6,7,9], [6,8,9]`. All labels and word positions in the files are zero-based.
There are exactly 624 combined cyclically ordered cases. Their maximum is
\(Q=1/2\), attained in 12 cases. Every other case has
\(Q\leq1/2-1/14\). For example,

\[
\sigma=(0,6,7,3,9),\qquad
\beta=(1/4,1/8,1/8,1/4,1/4)
\]

is feasible and has \(Q=1/2\).

To extend the capacity equality to the family, `product_exact.py` examines
every two- and three-coordinate closure support. Relevant column-rank minors
are nonzero at the base. Inconsistent two-coordinate systems remain
inconsistent near the base. Negative weights remain negative. Every weight
which is zero in a consistent base system is checked to be the zero rational
function (12 checks). Positive weights remain positive. Thus no previously
absent closure vertex can appear arbitrarily close to the base. All 12 tight
ordered cases have the rational-function identity \(Q(v,s)=1/2\). The finite
strict gap for the other cases persists on a common neighborhood. Consequently

\[
\max Q(E(v,s))=1/2,\qquad c_{\rm EHZ}(E(v,s))=1,
\qquad \rho(E(v,s))=1\tag{4.1}
\]

for all parameters sufficiently close to the base. This is the independent
capacity lower-bound/tightness part of the proof; the selected upper sections
alone would not establish it.

## 5. Twenty-four touching feasible sections

The fixed witness records, for each of 24 sections, a distinct-label word,
a positive rational weight vector \(\beta_j^*\), and five word-position indices
\(I_j\) with \(\det C_{\sigma_j,I_j}(A_*)\ne0\). Different sections are allowed
to use the same word with different positive representatives. Their minimum
weight is \(1/320\). The verifier checks exact feasibility and

\[
S_j\beta_j^*=C_j^T\mu_j^*,\qquad \mu_{j,5}^*=1,
\quad Q_j=\tfrac12(\beta_j^*)^TS_j\beta_j^*=1/2,\tag{5.1}
\]

where \(S_{ik}=S_{ki}=a_{\sigma_i}\Omega a_{\sigma_k}^T\) for \(i<k\), and
\(S_{ii}=0\). Equation (5.1) is used for derivatives and continuation, not as
a claim that a stationary quadratic program is globally optimal.

Positive feasible sections exist in the full normal chamber: prescribe the
nonpivot weights smoothly and solve

\[
\beta_{I_j}=C_{I_j}^{-1}(e_5-C_{J_j}\beta_{J_j}),
\qquad J_j=\{0,\ldots,m_j-1\}\setminus I_j.\tag{5.2}
\]

Strict positivity and the nonsingular pivot persist near the base. At the base,
stationarity means the first derivative of \(Q_j\) is independent of the
choice of the free-weight derivatives: the difference of any two feasible
first weight variations is in \(\ker C_j\), which is annihilated by
\((S_j\beta_j^*)^T\).

For completeness, with row-normal variations, the gradient contribution of
word position \(i\) to \(Q\) is

\[
\beta_i\left[
\Omega\left(\sum_{k>i}\beta_k a_{\sigma_k}^T-
                 \sum_{k<i}\beta_k a_{\sigma_k}^T\right)-\mu_{1:4}
\right]^T.
\]

At \(V=Q=1/2\), \(U=1\), so
\(DU=-4\,DQ-2\,DV\). The volume gradient is computed from

\[
DV(A)[\dot A]=-
\sum_i\frac1{\|a_i\|}\int_{F_i}\dot a_i\cdot x\,dS(x).\tag{5.3}
\]

For the family, each facet is an edge times the other polygon. Shoelace
moments give \(\int_Hq\,dq=0\) and
\(\int_Tp\,dp=((2u-1)/6,(2v-1)/6)\). These moments and edge midpoints give
`family_exact.py`'s volume-gradient formula. An independent determinant-cofactor
calculation verifies all 40 components at the base.

Let \(G_*\in\mathbb Q^{24\times40}\) be the resulting gradient rows.
The exact finite checks are

\[
\operatorname{rank}G_*=22,\qquad
G_*^T\lambda_*=0,\qquad\mathbf1^T\lambda_*=1,
\qquad \min_j\lambda_{*,j}=3/36617>0.\tag{5.4}
\]

All coefficients and gradient rows are in the fixed witness; they are
recomputed, rather than merely read and accepted. Rank 22 is not enough to
prove a sharp maximum in the 25-dimensional symmetry quotient. The next
sections handle the three missing directions.

## 6. Continuing the contacts and controlling the kernel

For each word, write \(B=A_\sigma\). Over \(\mathbb Q(v,s)\), solve the system

\[
\begin{pmatrix}S&-B\\C&0_{5\times4}\end{pmatrix}
\begin{pmatrix}\beta\\\mu_{1:4}\end{pmatrix}
=
\begin{pmatrix}\mathbf1_m\\e_5\end{pmatrix}.\tag{6.1}
\]

Choose a maximal nonsingular minor at the base, hold the remaining unknowns
at their base values, and solve that minor. The checker then verifies **every
row** of the original overdetermined system identically in the rational
function field. It also substitutes the base parameters and verifies recovery
of the specified base unknowns. This constructs rational functions
\(\beta_j^0(v,s)\) and \(\mu_j(v,s)\), positive and regular near the base.
Multiplying the upper block of (6.1) by \(\beta^T\) gives
\(\beta^TS\beta=1\); thus these contacts have \(Q=1/2\) throughout the family.
They touch (4.1).

Let \(G(v,s)\) denote their ambient gradient rows. The 15 tangent columns for
translations, scaling, and symplectic transformations are

\[
\dot a_i=-(a_i\cdot t)a_i,\qquad \dot a_i=-a_i,
\qquad \dot A=-AX,\quad X\in\mathfrak{sp}(4,\mathbb R).
\]

The code uses \(X=\Omega R\) with \(R\) symmetric to give ten generators.
Their matrix \(T_*\) has exact rank 15. The two equality-family derivatives
\(E_*=(\partial_vA,\partial_sA)|_*\) satisfy
\(\operatorname{rank}[T_*\ E_*]=17\).

Each gradient annihilates symmetry tangents throughout the family. One way
to see this without assuming that a chosen section is symmetry-equivariant is
to follow the symmetry curve through a touching point. The actual ratio stays
1, the upper section is at least 1 by (3.1), and it equals 1 at that point;
hence its derivative vanishes. It annihilates the two equality tangents because
its restriction to the family is identically 1.

There is one additional rational-function kernel vector \(W(v,s)\), whose
only possibly nonzero entries are the q components of the last four normals.
It is specified without numerical choices as follows. Restrict \(G\) to the
zero-based flattened coordinates

`[24,25,28,29,32,33,36,37]`.

In this 24-by-8 matrix, take rows `[0,12,23,22]`, solve columns `[0,1,2,3]`
with column 4's coefficient set to 1 and columns 5,6,7 set to zero. The chosen
four-by-four minor is nonsingular at the base. All 24 rows of

\[
G(v,s)W(v,s)=0\tag{6.2}
\]

are verified identically over \(\mathbb Q(v,s)\), not just at a finite number
of parameter values. The full rational-function vector is retained in
`certificate_family_exact.json`. Its base value is the 10-by-4 matrix with
rows 0 through 5 and row 9 zero, and

\[
W_{*,6}=(1/2,1/3,0,0),\quad
W_{*,7}=(3/2,-1,0,0),\quad
W_{*,8}=(1,0,0,0).\tag{6.3}
\]

The rank of \([T_*\ E_*\ W_*]\) is 18. Therefore, on a small parameter
neighborhood these 18 annihilated columns remain independent, so
\(\operatorname{rank}G(v,s)\leq22\). A base rank-22 minor persists, proving
**constant rank 22** there. This step is essential: a base-only rank check
would not justify the subsequent uniform tubular argument.

Choose the constant 40-by-22 coordinate-selection matrix \(Y\) corresponding
to flattened indices

`[0,1,2,3,4,5,6,7,8,9,10,11,12,14,15,16,18,24,25,26,28,29]`.

The exact check

\[
\operatorname{rank}[T_*\ E_*\ W_*\ Y]=40\tag{6.4}
\]

shows that these variables, together with the symmetry directions, cover all
normal deformations. Put \(H(v,s)=G(v,s)Y\), which has full column rank 22.
A continuous positive normalized relation near the base is obtained explicitly
from

\[
\widetilde\lambda=
\big[I_{24}-H(H^TH)^{-1}H^T\big]\lambda_*,
\qquad \lambda=\frac{\widetilde\lambda}{\mathbf1^T\widetilde\lambda}.
\tag{6.5}
\]

At the base this equals \(\lambda_*\), so every component stays positive after
shrinking the neighborhood. Since \(H\) and \(G\) have the same column space,
\(\lambda^TG=0\). Thus positivity and rank hold uniformly on a sufficiently
small compact parameter patch; a numerical radius is not needed for this
existence assertion.

## 7. Exact control of the last transverse direction

Near the equality family use the transverse normal parametrization

\[
A(v,s,z,y)=A(v,s)+zW(v,s)+Yy,\quad z\in\mathbb R,\ y\in\mathbb R^{22}.
\tag{7.1}
\]

For section \(j\), prescribe its free coordinates by

\[
\beta_{j,J_j}(v,s,z,y)=\beta_{j,J_j}^0(v,s)+z\,d_{j,J_j},\tag{7.2}
\]

where \(d_j\) is the rational first weight jet recomputed by
`section_second` and recorded in `certificate_base_exact.json`. Solve its
pivot coordinates by (5.2). These sections are smooth, positive, and feasible
on a common neighborhood of the parameter base. They need not be optimal
there; feasibility is sufficient for (3.1).

The jet proposer solves exact linear systems. At the base, it first imposes
\(C d=-(D C)[W_*]\beta\), and then adds a vector in \(\ker C\) chosen by a
rational linear solve. Its acceptance does not require a claim about a global
quadratic-program optimizer: only the resulting feasible jet and the resulting
second derivative are used. Formula (7.2) realizes the free jet by an actual
smooth section, not merely a formal direction.

There is a particularly simple independent check. On the entire germ of the
line \(A_*+zW_*\), the explicit rational vertex and section formulas yield

\[
V(A_*+zW_*)=\frac12+\frac{13}{1728}z^2,\qquad
Q_j(A_*+zW_*,\beta_j(z))=\frac12\quad(1\leq j\leq24).
\tag{7.3}
\]

These are exact identities in \(\mathbb Q(z)\), not finite differences. Hence

\[
U_j(A_*+zW_*)=\frac{864}{864+13z^2},\qquad
\partial_z U_j|_*=0,\qquad \partial_z^2 U_j|_*=-\frac{13}{432}.
\tag{7.4}
\]

`independent_exact.py` derives (7.3) directly by inverting the active vertex
systems, summing signed determinant volumes, and substituting the explicit
five-pivot weight sections. It does not reuse the volume/weight second-jet
calculation. Both implementations agree on all 24 second derivatives.

Let

\[
F(v,s,z,y)=\sum_j\lambda_j(v,s)U_j(v,s,z,y).
\]

It is an upper bound for the actual ratio, as each summand is. At \(z=y=0\),
\(F=1\) and \(D_{(z,y)}F=0\). Equations (6.5) and (7.4) give
\(F_{zz}|_*=-13/432<0\). By continuity, \(F_{zz}(v,s,0,0)\) is bounded above
by a strictly negative constant on a small parameter patch. Mixed second
derivatives are allowed; no unverified Hessian negativity in all directions
is being assumed.

## 8. The uniform local maximum argument

We give the local analytic implication explicitly to avoid mistaking separate
raywise tests for a neighborhood proof. Work on a small compact parameter patch
inside all the domains already described. Positive relation and rank of
\(H=GY\) imply a uniform constant \(\alpha>0\) such that

\[
\min_j (G_jY)y\leq-\alpha\|y\|\quad\text{for all }y.
\tag{8.1}
\]

To prove this, for \(y\ne0\) not all values can be nonnegative: their strictly
positive weighted sum is zero, and if all are zero then full column rank gives
\(y=0\). Compactness of the unit sphere times the parameter patch makes the
strict bound uniform. Since each \(G_jW=0\), common second-order Taylor bounds
then give some \(C>0\) with

\[
\min_j(U_j-1)\leq-\alpha\|y\|+C(\|y\|^2+z^2).
\tag{8.2}
\]

For \(F\), the vanishing first derivatives and the negative \(zz\) coefficient
give constants \(b>0,C'>0\), after further shrinking, such that

\[
F-1\leq-bz^2+
C'\big(\|y\|^2+|z|\|y\|+(|z|+\|y\|)^3\big).
\tag{8.3}
\]

All functions used here are rational and regular, hence have the required
uniform derivatives. Choose \(M>2C/\alpha\).
If \(\|y\|\geq Mz^2\) and \((z,y)\ne0\), (8.2) is strictly negative once
\(\|y\|\) is sufficiently small. If \(\|y\|<Mz^2\), then \(z\ne0\) and the
positive terms of (8.3) are \(O(|z|^3)\), uniformly; (8.3) is strictly negative
for sufficiently small \(|z|\). At \(z=y=0\) the actual ratio is 1 by (4.1).
Consequently the actual ratio is at most 1 on a genuine neighborhood in (7.1).

Finally add local group parameters. For \(x\mapsto\ell Sx+t\), with \(S\)
symplectic and \(\ell>0\), the normal row transforms as

\[
a\longmapsto\frac{aS^{-1}}{\ell+aS^{-1}t}.
\]

This preserves the ratio. Compose this action with (7.1). Its differential has
the 40 independent columns (6.4), so the inverse function theorem makes the
result a full local normal coordinate system. It follows that **all** nearby
normal matrices, not just those on a product or affine subspace, satisfy the
ratio inequality after applying the harmless group action. Section 2 converts
this to the stated fixed-ten-facet Hausdorff neighborhood theorem.

The independent equality derivatives modulo symmetries show that the maximum
is non-strict in two genuine local parameter directions. No uniqueness claim
away from this neighborhood, and no assertion about adding facets, is made.

## 9. Nonequivalence to HKO

HKO, the product of two regular pentagons, has 25 vertices and ratio
\((3+\sqrt5)/5>1\), as specified in the handoff and [HKO]. Our body has 24
vertices and ratio 1. Vertex count alone rules out every invertible affine
image, so in particular it rules out translations, scaling, linear symplectic
and anti-symplectic transformations, and facet relabeling. The exact ratio is
a second, consistent separation invariant. A numerical alignment test is not
used.

## 10. Verification boundary and provenance

The exact verifier needs only Python, SymPy, the four core source modules, and
the fixed rational witness. Its acceptance uses no floating-point inequality,
no numerical optimizer, no general-purpose capacity oracle, and no retained
result file. Timing fields are of course floating point. The rational-function
identities, positivity tests, complete product enumeration, and ranks are
recomputed. The independent direct check shares the rational family and input
witness, but implements the decisive volume and section differentiation by a
separate method. This is a second implementation, not an outside peer review.

The mathematical trust boundary is the HK formula, elementary convex/polytope
geometry, the inverse function theorem, and the analytic argument in Section 8,
together with correct exact algebra implementation. There is no unresolved
mathematical blocker identified in this packet, but no proof-assistant claim
or explicit-radius claim is made. Reproduction and exact source hashes are in
`README.md`, `RUN_LEDGER.md`, and `evidence/exact/RUN.json`.

The product equality construction was motivated by [BMP, Section 5.2]. That
paper is not being credited with the full nonproduct local-maximality theorem
proved here, and this packet does not claim bibliographic priority for the
underlying equality polygons. Its equality and capacity assertions are checked
independently above. The HKO local-maximality theorem was used only as accepted
calibration context, not reproved. See `SOURCES.md` for source details.
