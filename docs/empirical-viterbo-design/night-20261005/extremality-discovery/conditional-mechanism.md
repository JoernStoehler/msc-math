# Conditional extremality mechanism: active-support rigidity versus optimizer mobility

Owned discovery note, 2026-10-04 22:55 UTC. Receiving/integrating owner:
`extremality_discovery_lead`. Mathematical status: agent derivation, awaiting
review; no thesis promotion or unrestricted novelty claim. No retained producer
was rerun.

## Finding

Facet coverage and convex active-weight balance do not exhaust the geometry of
support-chart local maxima. Under a complete regular local HK envelope, **the
normal-closure subspaces supported on the active orbit supports must together
span the entire normal-closure space**. This is a conditional incidence/rank
falsifier, stronger than merely covering each facet. There is also a quantitative
curvature condition: in every direction invisible to all active weights,
optimizer mobility must compensate the strictly negative volume curvature.

A small active-weight hull by itself is not an obstruction. An exact 4D box
control below has a three-dimensional support quotient, only a one-dimensional
active-weight hull, and is nonetheless a support-chart local maximum. Weight
reoptimization supplies the missing curvature. Consequently a proposed
unconditional “local maxima require a full-dimensional active-weight hull”
theorem would be false.

## Setup and evidence boundary

Use the open simple fixed-normal chamber of
`formal/active-orbit-facet-coverage.tex`. Write

\[
 N=[n_1\ \cdots\ n_F]\in\mathbb R^{4\times F},\qquad
 E=\ker N,\quad V_0=V(h_0),\quad c_0=c(h_0),\quad
 s=\frac{\nabla V(h_0)}{4V_0}.
\]

The bounded full-dimensional body implies `rank N = 4`. The existing theorem
says local maximality forces \(s\in\operatorname{conv}\mathcal W(h_0)\), with
\(\alpha\cdot h_0=1\), \(N\alpha=0\). This first-order balance alone supplies
no optimizer curvature.

The primary capacity source is [Haim–Kislev, *On the symplectic size of convex
polytopes*](https://arxiv.org/abs/1712.03494), with the project orientation and
normalization already audited in the formal source. The volume input is
[Fillastre–Izmestiev, *Shapes of polyhedra, mixed volumes, and hyperbolic
geometry*](https://arxiv.org/pdf/1310.1560): Corollary 2.11 and Remark 2.14 give
the volume Hessian signature and its kernel modulo translations on a simple
type cone. In dimension four the signature is one positive eigenvalue, four
zero eigenvalues, and all remaining eigenvalues negative. Euler homogeneity
gives \(D^2V(h_0)h_0=3\nabla V(h_0)\), so

\[
 \nabla V(h_0)\cdot v=0,\quad v\notin\operatorname{im}N^T
 \quad\Longrightarrow\quad D^2V(h_0)[v,v]<0. \tag{V}
\]

This stronger signature statement, rather than concavity alone, is what
establishes strictness. All later results are derived here from these inputs;
the literature search does not establish novelty of the resulting corollaries.

## Named statement: regular HK curvature–mobility obstruction

Assume a finite **complete local envelope**
\(c(h)=\min_{j=1}^m A_j(h)\) near \(h_0\), after locally inactive branches
have been removed. Every remaining branch is active at \(h_0\), belongs to a
fixed word and fixed positive support \(I_j\), and arises by reoptimizing that
word on that support. In particular this assumption must rule out additional
right-active support appearances or singular optimizer germs.

For each branch, let \(\alpha_j\) be its base normalized weight. Define

\[
 E_j=\{b\in E:b_i=0\ (i\notin I_j)\},\qquad
 L_j=E_j\cap h_0^\perp,
\]

and choose a full-column-rank basis matrix \(Z_j\) for \(L_j\). The normals-form
quadratic is \(Q_j(b)=\tfrac12 b^T H_jb\), where `H_j` is the symmetric
matrix in the project word convention. Require the regular strict maximum
condition

\[
 C_j=-Z_j^TH_jZ_j\succ0. \tag{R}
\]

An empty tangent space is allowed and contributes zero mobility. Define

\[
 M_j(v)=(Z_j^Tv)^TC_j^{-1}(Z_j^Tv)\geq0.
\]

**Proposition.** If \(h_0\) is a local maximum of \(c^2/(2V)\) and the above
hypotheses hold, then every direction in
\(K=\{v:\alpha_j\cdot v=0\text{ for all }j\}\) satisfies

\[
 \boxed{\max_j M_j(v)\ \geq\ -\frac{c_0}{4V_0}
                         D^2V(h_0)[v,v].} \tag{CM}
\]

Thus each nontranslation direction in this common invisible subspace must be
seen by at least one active optimizer's tangent mobility. The quantifier is
\(\forall v\in K\ \exists j\); a single branch need not control every direction.

### Derivation, including the exact branch formula

Suppress the branch index. Nearby weights on its fixed support, normalized at
the base point, have the form \(b=\alpha+Zx\), with
\(h_0\cdot b=1\). Stationarity and activity yield

\[
 Q(\alpha+Zx)=\frac1{2c_0}-\frac12x^TCx.
\]

For \(h=h_0+tv\), set \(\ell=1+t\alpha\cdot v\) and \(w=Z^Tv\). The action
as a function of the optimizing variable is

\[
 A(t,x)=\frac{(\ell+t w^Tx)^2}{1/c_0-x^TCx}.
\]

Its smooth local minimizing solution is
\(x(t)=-tC^{-1}w/(c_0\ell)\). Positive support and denominator persist for
sufficiently small `t`. Substitution gives the **exact** nearby branch formula

\[
 \boxed{A(h_0+tv)=c_0(1+t\alpha\cdot v)^2-t^2M(v).} \tag{Q}
\]

The stationary solution is the local minimum because `C` is positive definite;
the complete-envelope assumption identifies these local solutions with all
branches determining scalar capacity nearby. Formula (Q) also follows from
the inverse of the quadratic form on \(E_j\).

For \(v\in K\), completeness now gives exactly
\(c(h_0+tv)=c_0-t^2\max_jM_j(v)\) for sufficiently small `t` on that line.
First-order balance implies \(s\cdot v=0\), hence \(DV[v]=0\). Expanding only
volume gives

\[
 \frac{\operatorname{sys}(h_0+tv)}{\operatorname{sys}(h_0)}
 =1+t^2\left[-\frac{2\max_jM_j(v)}{c_0}
              -\frac{D^2V[v,v]}{2V_0}\right]+o(t^2).
\]

The nonpositive coefficient required at a local maximum is exactly (CM).

## Named corollary: active-support closure spanning

Under the proposition's hypotheses,

\[
 \boxed{\operatorname{span}(E_1,\ldots,E_m)=E=\ker N.} \tag{CS}
\]

**Proof.** Each \(E_j=\operatorname{span}\{\alpha_j,\operatorname{im}Z_j\}\).
If their joint span is a proper subspace of `E`, its orthogonal complement
contains a direction `v` outside \(E^\perp=\operatorname{im}N^T\). Such `v`
annihilates every active weight and every tangent basis, so `v` belongs to `K`
and every `M_j(v)` vanishes. By (V), its volume curvature is strictly negative,
contradicting (CM).

This can be checked **from the complete active supports and normals alone**:
compute the kernels of the restricted normal matrices, embed them in
\(\mathbb R^F\), concatenate their bases, and compare the resulting rank to
`F − 4`. No active weights or inverse Hessians are needed for this rank
falsifier, though regularity/completeness remain mathematical prerequisites.
Coverage only asks whether the union of supports contains every label; it does
not establish (CS).

If every active support is a rigid positive circuit, meaning
\(\dim E_j=1\), then \(Z_j\) is empty. Local maximality therefore forces
\(\operatorname{span}\{\alpha_j\}=E\), hence at least `F − 4` independent
active normalized weights. Equivalently their affine hull must have dimension
`F − 5`. This is the correct **conditional** active-hull statement; dropping
the rigidity condition is false, as the next control shows.

For coincident active normalized weights, first-order balance forces
\(\alpha_j=s\), hence strictly positive weights on every facet. The rank test
then passes automatically, but (CM) remains restrictive. Different words can
share the same normalized weight and have different mobility forms; (CM) uses
their directionwise maximum, and does not force any one form to dominate
volume curvature in every direction.

For a **single regular optimizing action branch**, if `U` is any basis of a
complement to translations in \(s^\perp\), its exact necessary matrix test is

\[
 U^T ZC^{-1}Z^T U
   +\frac{c_0}{4V_0}U^TD^2V(h_0)U\succeq0. \tag{SM}
\]

The volume term is negative definite. Thus a single smooth optimizer must be
sufficiently mobile to dominate that prescribed curvature. This is a concrete
second-order check beyond the unconditional identity \(\alpha=s\), rather
than an unjustified impossibility claim about coincident active weights.

## Exact actual geometry control: equal-area 4D boxes

Take the simple fixed-normal box

\[
 K=[-x_1/2,x_1/2]_{q_1}\times[-y_1/2,y_1/2]_{p_1}
   \times[-x_2/2,x_2/2]_{q_2}\times[-y_2/2,y_2/2]_{p_2},
 \qquad a_j=x_jy_j>0.
\]

Here `F = 8`, `dim E = 4`, and the support quotient modulo translations and
scaling has dimension three. Directly from HK,

\[
 c(K)=\min(a_1,a_2),\qquad V(K)=a_1a_2,\qquad
 \operatorname{sys}(K)=\frac12\frac{\min(a_1,a_2)}{\max(a_1,a_2)}.
\]

For completeness, closure pairs equal the weights on opposite normals. Denote
these pair weights by `u_j` on the q pair and `v_j` on the p pair. The support
constraint is \(\sum_j(x_ju_j+y_jv_j)=1\). Cross-plane symplectic pairings
vanish, and each plane's closed four-direction word contributes at most
\(2u_jv_j\), attained by cyclic rectangular order. If
\(t_j=x_ju_j+y_jv_j\), then

\[
 Q\leq\sum_j2u_jv_j\leq\sum_j\frac{t_j^2}{2a_j}
 \leq\frac{t_1^2+t_2^2}{2\min(a_1,a_2)}
 \leq\frac1{2\min(a_1,a_2)}.
\]

A single plane with smallest area attains every relevant bound. When
\(a_1=a_2\), equality requires `t_1 t_2 = 0`; so there are exactly two active
normalized weight vectors, one for each symplectic plane (word multiplicities
do not change the weight set). Each is supported on four facets.

Equal areas are consequently genuine local maxima in the support chart, with
`sys = 1/2`. Their active weights have affine dimension only one, not three.
But each four-facet support has \(\dim E_j=2\), and the two disjoint
normal-closure subspaces span `E`. Each optimizer has one nontrivial mobility
direction. Aspect-ratio changes supply capacity curvature, whereas changing
the ratio of plane areas produces a first-order cusp. This validates (CS) and
shows exactly why the frozen-weight restriction matters. No full-normal-chart
local-maximality claim is made for these boxes.

One exact coefficient check: at \(x_1=y_1=x_2=y_2=2\), take a common invisible
direction of widths \((\dot x_1,\dot y_1,\dot x_2,\dot y_2)=(a,-a,b,-b)\).
Then \(A_1(t)=4-a^2t^2\), \(A_2(t)=4-b^2t^2\), and
\(V(t)=(4-a^2t^2)(4-b^2t^2)\). Consequently (CM) reads

\[
 \max(a^2,b^2)\geq\frac{a^2+b^2}{2},
\]

which holds, with equality precisely when \(|a|=|b|\). This checks the factor
`1/4` in the curvature threshold using an actual capacity envelope.

## Exact obstruction and next useful bounded step

The obstruction to applying this to a numerical orbit list is **local
completeness plus regular support persistence**, not merely lack of enough
floating-point Hessian precision. Positive reduced-Hessian definiteness on
each listed support says nothing about omitted words, a zero multiplier that
lets a new facet enter, or singular optimizing families. Upper touching
branches alone give a sufficient certificate; they cannot justify this
necessary condition because omitted branches could lower the true capacity
and repair the apparent improving direction.

The narrow next step is to choose one existing exact orbit catalogue with
fixed-normal perturbations, certify all active supports and strict reduced
Hessians, then compute the concatenated closure rank. A rank defect is an
actual rejection under those certificates. Full rank only passes a necessary
test; one must still check (CM) or a stronger local argument. It would be
particularly useful to find a covered, convex-balanced catalogue with a rank
defect, rather than merely re-demonstrating missing-facet coverage.

Remaining quantifiers: this note proves the regular finite-envelope statement
on a simple support chamber only. Whether an analogous spanning condition
survives arbitrary singular/zero-weight germs requires a generalized mobility
cone, and is open here. There is no claimed implication that every local
maximum has many distinct active weights, or one orbit using all facets.
