Status: legacy/imported raw source material.

Live flow-graph algorithm control surface:
`crates/symplectic/src/algorithms/flow_graph/README.md`.

This file is preserved as the raw 2026-05-04 Jörn note. Do not treat it as the
current algorithm contract without checking the live flow-graph README and
newer accepted decisions.

mathematical:

define the tube of
- an inner segment count k>=1
- a combinatorics (s_1, ..., s_k+1, s_k+2) \in {1..F}^(k+2)
- an action cutoff Acut
to be the the set {(gamma,t)} of
  - a generalized reeb trajectory gamma \in W^1,2(R \to \partial K)
  - a tuple of times t_1 < ... < t_k+1
  - such that gamma is on [t_i, t_i+1] a linear segment with velocity R_s_i+1 for i=1,...,k
  - and gamma is on [t_1 - eps; t_1] a linear segment with velcoity R_s_1, for some eps>0
  - and gamma is on [t_k+1; t_k+1 + eps] a linear segment with velocity R_s_k+2, for some eps>0
  - the action A(gamma|_[t_1,t_k+1]) = int_t_1^t_k+1 dt lambda_0(gamma(t))(\dot gamma(t)) = t_k+1 - t_1 is <= Acut

Note: the set can be infinite, for example due to time shifts

Lemma: in the generic case where omega(a_i,a_j)!=0 for i!=j the segment interiors are in the interior of a facet, which implies that the breakpoint gamma(t_i) uniquely determines t_i+1 and gamma(t_i+1) via affine maps; the other direction also works
corollary: (t_1, gamma(t_1)) determines uniquely the whole trajectory gamma on [t_1-eps,t_k+1+eps] if such a trajectory exists

Fact: the breakpoints gamma(t_i) lie on H_s_i \cap H_s_i+1 = {x: <x,a_s_i>=1 and <x,a_s_i+1> = 1} for i=1,...,k+1

note: we definitely need to properly show the oorder of the objects here:
t_1-eps s_1 t_1 s_2 ... s_k+1 t_k+1 s_k+2 t_k+1+eps

lemma: the set of points {gamma(t_1)} and the set of points {gamma(t_k+1)} are polygons in H_s_1\capH_s_2 and respectively H_s_k+1\capH_s_k+2 , which by genericity are 2-dim affine spaces [potentially the polygon is empty]
proof: uses the affine maps by chaining them, intersect the polytope inequalities, notice that Acut >= t_k+1-t_1 defines a half-space inequality, and so we have a bounded intersection of inequalities -- aka a maybe-empty, bounded polygon

computational:

we can thus represent the following data
- the segment count k
- the combinatorics s
- the action cutoff Acut
- the set {gamma(t_1)} as a polygon (vertices and/or dual vertices - unsure what's best, probably dual vertices with ccw ordering or sth since we pushforward & intersect inequalities)
- the set {gamma(t_k+1)} as a polygon
- global: for every H\capH we need a base point and basis vectors in R^4 to convert between coordinates
- the composed affine map Phi and inverse Phi^-1 from/to start to/from end polygon (as a 2x2+2 affine matrix+vector in the respective bases)
- the accumulated affine map A from start/end polygon into the action/total-time t_k+1 - t_k (again a 1x2+1 affine function)

mathematical:

the intersection algorithm takes two tubes (s,Acut) and (s',Acut') and returns the tube for (s'',Acut+Acut')
where we need the shape s=(s1, ..., s_k+1, s_k+2) and s'=(s_k+1, s_k+2, ..., s_k+k'+2) and s''=(s1, ..., s_k+k'+2), so we glue together the equal end hyperplane of s and start hyperplane of s'

- the segment count is k+k'
- the combaintroics as said
- the action cutoff as said
- the new forward map is Phi'' = Phi' \circ Phi
- the new start action is A'' = A + A' \circ Phi
- the new start polygon is P''_1 = P_1 \cap Phi^-1 (P'_1)
- the new end polygon is P''_k+k'+1 = P'_k+k'+1 \cap Phi'( P_k+1 )

the lower-action algorithm goes from Acut -> Acut' < Acut by intersecting with the halfspace inequality {A <= Acut'}

the primitive algorithm computes a tube (s1,s2,s3; Acut=+infty)

the closed loop algorithm takes a tube that is 'closed' i.e. (s1, s2, ..., s1, s2) and solves the fixed-point problem
  Phi(x)=x, x \in P_start [which implies x \in P_end=Phi(P_start)]
Generically there is 0 or 1 fixed point like that.

what's missing: describing how one can now search for the minimum action of a polytope; the naive thing is to just
- enumerate all possible combinatorics of a simple Reeb orbit -- i.e. (s1, ..., sk, s1, s2) where s1...sk are pairwise different
- the pairwise different gives us a bound on k, so teh iteration is finite
- build up the tube (s, +infty) from primitives via intersection and then look for a fixed-point (0 or 1 exist)
- take the minimum of the fixed points' actions

but we can ofc do better by building the combinatorics (Words may also be a good name) smartly, caching the tube data, and using Acut < infty values, and even doing a heuristical search first that gives us a tighter upper bound to work with
