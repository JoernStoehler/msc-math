# Primary-source verification of the exact volume formula

29 October 2026. Reviewer `/root/hko_ascent_math_audit`.

**PASS for the formula and its implementation normalization.** This resolves
the outstanding formula-reference item in the initial geometry summary.
It does not rerun the producer or certify the still-separate capacity result.

The primary source is Jim Lawrence, *Polytope volume computation*, NISTIR
89-4123 (October 1989), §2, printed page 4 (PDF page 8). The theorem was
checked in the [official NIST PDF](https://nvlpubs.nist.gov/nistpubs/Legacy/IR/nistir89-4123.pdf),
including a rendered inspection because OCR omitted its numerator. The
downloaded PDF and `lawrence-theorem.png` are retained in this directory.
The published version is *Mathematics of Computation* 57 (1991), 259–271;
the AMS PDF fetch returned HTTP 403, so the inspected primary text is the
author's NIST report rather than an asserted inspection of that journal PDF.

Lawrence's theorem applies to a bounded full-dimensional polytope with
exactly n active inequalities at each vertex. For an affine functional
f(x)=c.x+d nonconstant on every edge, write c=sum gamma_i a_i in the basis
of active outward normals. Its vertex contribution is

    f(v)^n / [n! delta_v product_i gamma_i],

where delta_v is the absolute determinant of that normal basis. Summing
these contributions gives volume. The report proves the result using a
signed decomposition into polarized tangent cones and their truncations.

## Translation into the actual code

Let M_v be the four-by-four matrix with active normals as **rows**, as in
exact_geometry.py. With c=ell as a row covector, the equation
ell=sum_i gamma_i a_i is gamma M_v=ell. Thus

    gamma = ell M_v^-1,     delta_v=|det M_v|.

Putting n=4 and d=0 gives precisely the implemented expression

    sum_v (ell.v)^4 / [24 |det M_v| product_i(ell M_v^-1)_i].

There is no omitted normal-length or edge-length factor. The inward tangent
edge generators may be chosen as w_i=-M_v^-1 e_i: applying M_v gives -e_i,
so the ith active inequality decreases and the other three remain equalities.
Their slopes are ell.w_i=-gamma_i. These generators are not unit vectors;
that is appropriate. Their determinant magnitude is 1/|det M_v|, and any
positive rescaling of individual generators cancels between their determinant
and slope product. In four dimensions the four minus signs multiply to +1.

The signed product of gamma_i must be retained. Taking absolute values of
individual slopes would destroy the cancellation between vertex contributions.
The code correctly takes an absolute value only on det M_v. Reordering the
active rows changes neither that absolute determinant nor the slope product.

## Hypotheses and exhaustive vertex list

The origin strictly satisfies all twenty inequalities and the old bounded H
contains the intersection. Thus this polytope is full-dimensional and bounded.
Every vertex of a full-dimensional polytope has four independent active
normals. Enumerating every four-element subset, rejecting singular matrices,
and solving M_v v=1 therefore encounters every vertex. Exact checks against
all twenty inequalities retain exactly the feasible intersections.

For every retained point, the source recomputes the complete active set and
asserts its size is four. Since the generating four normals were independent,
this proves the required simple-vertex condition. Using that active tuple as
the dictionary key cannot lose distinct vertices: the invertible active
system has a unique solution. Conversely a vertex cannot appear under two
different four-element active sets when its complete active set has size four.

The code checks every component of ell M_v^-1 is nonzero for every vertex.
Since these are the negatives of the actual incident edge slopes, this
establishes the theorem's edge-genericity condition. No condition that ell.v
be positive, or that all vertex values be pairwise distinct, is required.

The observed completed output has 75 simple vertices and reports the exact
volume

    300748771/52178240 - (99664289/260891200) tau^2.

The second generic functional is a useful independent algebraic evaluation
of the same identity. Agreement does not prove the identity; the primary
theorem and the hypothesis/normalization checks above supply that premise.
This audit did not rerun the vertex enumeration or independently derive the
reported field element by a second volume algorithm.
