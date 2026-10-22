# Determinant maximization with rank one constraints

22 October 2026. Derived by the root agent during the ten-minute research
sprint; independently checked by `edge_reduction_review`. This is a complete
elementary argument, with external novelty unassessed.

## Theorem

Let P be a compact full-dimensional polytope in the four-dimensional space
of real 2 by 2 matrices. Suppose P is defined by finitely many inequalities
u_i^T M v_i <= b_i, whose matrix normals have rank one. Then each of det M,
-det M and |det M| attains its maximum on an edge or vertex of P.

Consequently the largest-area linear image of one centrally symmetric planar
polygon contained in another can be found by checking the vertices and the
stationary points of determinant on the edges of its matrix containment
polytope. No two-dimensional face optimization is needed.

## Proof

First maximize det M, and take a maximizing point in the relative interior
of its minimal face F. If dim F >= 3, its tangent space V intersects the
two-dimensional subspace {aI+bJ}, where J is rotation by pi/2, in a nonzero
matrix N. Since det N=a^2+b^2>0,

    det(M+sN)=det M+s cof(M):N+s^2 det N

strictly exceeds det M for at least one of the two sufficiently small signs
of s. Both signs are feasible in the relative interior of F. This is a
contradiction. For -det, use instead the two-dimensional subspace of matrices
[[a,b],[b,-a]], whose nonzero determinants are negative. Thus the minimal face has dimension at most two. The zero- and one-dimensional
cases already satisfy the conclusion; consider the two-dimensional case.

The active constraint normals at a relative-interior point span the
two-dimensional normal space of F. Choose two independent active rank-one
normals u_1 v_1^T and u_2 v_2^T.

If u_1,u_2 are independent and v_1,v_2 are independent, the invertible
coordinate change X=U M V, with the u_i^T as rows of U and v_i as columns
of V, makes aff(F) exactly the plane X_11=b_1, X_22=b_2. On this plane

    det X=b_1 b_2-X_12 X_21.

This quadratic has indefinite Hessian and hence no relative-interior local
maximum or minimum. Multiplication by the nonzero constant det U det V does
not change this fact for det M.

Otherwise one factor pair is dependent. Independence of the two matrix
normals forces the other pair to be independent. If the u pair is dependent,
an invertible coordinate change fixes one row of X throughout aff(F). The
determinant is then affine in the other row. If the v pair is dependent,
the same argument fixes one column. An affine function with a relative-interior
maximum is constant on F. Its maximum therefore also occurs on the boundary
of this compact two-face, which consists of edges and vertices.

This proves both signed claims. Their larger maximum is the maximum of the
absolute determinant, proving the final claim.

## Explicit finite optimization rule

On each edge write M(t)=A+t(B-A), 0<=t<=1. The determinant is

    q(t)=a t^2+b t+c,
    a=det(B-A), b=cof(A):(B-A), c=det A.

For det, check endpoints and t=-b/(2a) whenever a<0 and that parameter lies
strictly between zero and one. For |det|, check endpoints and every stationary
point with a!=0 in the interval. If a=0, endpoint values suffice. This is
an exact algorithm when the complete edge list and its arithmetic are exact;
a floating-point face enumeration does not establish completeness.

For polygon containment M D subset C, each vertex v of D and each facet
inequality u^T x<=b of C supplies the rank-one constraint u^T M v<=b.
If C,D are full-dimensional convex bodies containing zero in their interiors,
the resulting matrix polytope is full-dimensional and compact. Full dimension
follows because all sufficiently small matrices are feasible; compactness
follows by bounding the images of two independent vertices of D.

## Application to the affine pentagon and heptagon problem

The exact 5 by 7 capacity formula reduces the sharp systolic ratio to
max |det M| on |(n_i^(5))^T M n_j^(7)|<=1. The theorem reduces the previous
two-face route in `next-obstructions-20261022.md` to its one-skeleton.
It proves no numerical value of that optimum and does not permit checking
vertices alone without checking stationary edge points.

The mathematical review found no gap. A narrow web search for determinant
maximization with rank-one polytope constraints found no matching theorem;
that search does not establish novelty. This note has no human acceptance
or manuscript inclusion.
