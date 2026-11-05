# Single-facet invariant splitting obstruction

Let H={x:a_i·x<=1, 0<=i<10} be the repository's support-normalized HKO
product, with coordinates and rows in [the parent account](../README.md).
Fix a facet label F and any finite list D_1,...,D_m satisfying
D_j·J a_F=0 (the sign of J is immaterial for this condition). Consider the fixed
finite bank

    H_t={x:a_i·x<=1 for every i; (a_F+tD_j)·x<=1 for every j}, t>=0.

Thus every old row is retained and the other nine facets have zero first-order
profile. Then the right logarithmic systolic derivative is nonpositive; it is
strictly negative when the induced depth profile is nonzero.

Write the actual depth profile in Appendix C quotient coordinates as
f(xi,eta)=max(0,l_1(xi,eta),...,l_m(xi,eta)), with all l_j affine. It is
nonnegative, continuous, and convex in xi. More generally the following estimate
holds for every nonnegative continuous f convex in xi, whether or not it is a
polyhedral profile:

    R(f) >= (5/4) nu(f),

where nu(f)=E f/10 and R is the maximum of the profile integral over the weak
closure of labeled occupation measures of all finite maximizing facet words,
as defined in the simultaneous-splitting theorem. In particular this class
contains the four Appendix C literal families.

Proof. Let k=sqrt(5)-2. Put A=(5-sqrt(5))/40, B=(5+sqrt(5))/40,
b=sqrt(5)/10, C=(5+2sqrt(5))/10. The quotient probability is dxi/2 times
p(y)dy, where p=b(y+3) on [-1,k] and C(1-y) on [k,1].
For either sign eps=+/-1 the literal lower/upper shortest families give

    R(f) >= A(y+3) f(eps,y),  -1<=y<=k,
    R(f) >= B(1-y) f(eps,y),   k<=y<=1.

These families are reconstructed globally maximizing finite facet words; they do not rely
on a numerical catalogue maximization. Therefore, ignoring the zero-measure
endpoint y=1,

    integral f(eps,y)p(y)dy
      <= R(f) [(b/A)(k+1)+(C/B)(1-k)] = 8R(f),

because both bracketed terms are exactly 4. Convexity gives

    integral_-1^1 f(xi,y) dxi/2 <= [f(-1,y)+f(1,y)]/2.

Average the two edge estimates: E f <=8R(f), hence nu(f)<=4R(f)/5.
The fixed-bank derivative theorem (with full-rank support, already established
for H) gives D+log sys=4(nu(f)-R(f)) <=-nu(f). A nonzero nonnegative continuous
profile has positive nu. Facet-transitive symplectic symmetry covers any F.

Scope: any finite number of invariant new cuts on one original facet, all old
rows retained. Does not cover cuts on several facets, non-invariant depths,
old-row motions, or nonlinear higher-order deformations with zero first-order
profile. Not an unrestricted facet-count or neighborhood theorem.

Inputs, all relative to the frozen report packet:

- [Appendix C](../../../capacity-and-qp/convex-systolic-optimization/sections/hko-two-hinge-appendix.tex): quotient density and four literal families.
- [Literal exact checker](../../../capacity-and-qp/convex-systolic-optimization/certificates/hko-two-hinge/certify_amplification.py): finite orbit/density predicates.
- [Simultaneous-splitting theorem](../../../capacity-and-qp/convex-systolic-optimization/certificates/hko-ascent/multicut-variation.tex): derivative and full-rank premise.

This elementary consequence was derived in this continuation sprint. Its novelty
in the literature has not been investigated. Independent agent mathematical
review is retained in AUDIT.md; it is not human acceptance or formal verification.
