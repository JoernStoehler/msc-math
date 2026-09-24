# A small contact-reduction lemma for the remaining global cover problem

Let t=(t_1,...,t_N) be translations of fixed planar polygons. On an open cell
where the labelled convex-hull vertex cycle is fixed and all its defining
inequalities are strict, hull area is a polynomial of degree at most two in t.
It is affine in each two-dimensional translation block separately: terms using
the same block twice have determinant det(t_j,t_j)=0. Thus every diagonal 2x2
block of the Hessian is zero.

At a local minimum in this open cell the gradient is zero and the Hessian is
positive semidefinite. A positive semidefinite matrix with zero diagonal is
zero (use its 2x2 principal minors). Hence the whole Hessian is zero and the
area polynomial is constant on this cell.

Therefore an area-minimizing placement can either be moved at constant area to
a hull/contact boundary, or already lies on such a boundary. In a bounded
sublevel translation domain, a ray inside a constant-area cell eventually
reaches a boundary of the cell. A bound used to compactify the search cannot
be treated as a physical contact without checking its origin.

This is NOT a complete reduction to finitely many rigid contact patterns.
On a boundary stratum the constraints are coupled and possibly nonlinear; the
zero-diagonal Hessian argument does not simply iterate after restriction. It
also does not prove that every hull edge has a ruler-edge direction. These are
precisely the additional geometric questions proposed for the global session.

The lemma explains why a search for isolated minima entirely inside generic
hull-order cells is poorly aimed, and provides a rigorous first step toward
contact classification. No computational evidence is used.
