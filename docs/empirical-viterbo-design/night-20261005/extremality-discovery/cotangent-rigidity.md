# Nonlinear cotangent equality construction: an exact convexity obstruction

Owner: extremality_discovery_lead. Derived 2026-10-04, initial night research.
Status: elementary proof supplied; specialist review requested. No external
novelty claim. This is an adverse scientific return, not a new maximizer.

## Named statement

Let K,Q be compact planar convex bodies with nonempty interior. Suppose Q is a
polygon whose facet normals contain at least three distinct **unoriented**
directions. Let f be a C^3 diffeomorphism on a neighborhood of K. Its cotangent
lift is

    Phi_f(q,p) = (f(q), Df(q)^(-T) p).

If Phi_f(K x Q) is convex, f is affine on int K (and hence on K by continuity).
Thus this broad nonlinear canonical-coordinate construction cannot create a
new convex equality body from the HKO pentagon product or the P* product. Both
momentum factors have at least three unoriented normal directions. All surviving
images in this class are linear symplectic images followed by translation.
The statement does not cover arbitrary Hamiltonian diffeomorphisms.

## Proof

Write x=f(q), y=Df(q)^(-T)p. For an input facet n_i.p=h_i of Q, the transformed
fiber facet is

    m_i(x).y = h_i,       m_i(x)=Df(f^(-1)(x)) n_i.

Fix x0 in f(int K), and a point y0 in the relative interior of this facet of
the fiber. All other inequalities are strict there. In a small open product
neighborhood in (x,s), with s one momentum coordinate, a fixed choice of the
other coordinate as graph direction gives this boundary as

    y_graph = a_i(x) + b_i(x) s.

The convex body's interior is on a fixed side of this graph. Therefore its graph
function is locally concave or locally convex, as appropriate. Its Hessian is
semidefinite. Its s,s entry is zero; every mixed x,s entry must consequently be
zero (a semidefinite matrix with a zero diagonal entry has zero corresponding
row). Thus Db_i=0. The slope, equivalently the unoriented direction of m_i(x),
is locally constant. This holds for every facet and every x0. Connectedness of
f(int K) makes every transformed facet-normal direction globally constant.

Choose two independent normals n1,n2, and a third n3=a n1+b n2 with a,b nonzero.
Fix a base point and call its derivative A. Fixed projective images of n1,n2
mean

    Df(q)n1 = lambda1(q) A n1,
    Df(q)n2 = lambda2(q) A n2.

The scalars never vanish. Fixed projective image of n3, and independence of
A n1,A n2, imply lambda1/lambda2 is constant; its base-point value is one. Hence
Df(q)=lambda(q)A. Equality of mixed partial derivatives now gives

    (partial_2 lambda) A e1 = (partial_1 lambda) A e2.

The two columns of A are independent, so both derivatives vanish. Therefore
lambda is constant, Df is constant, and f is affine. This proves the statement.
No capacity calculation or numerical sample is used.

## Essential hypothesis: exact nonlinear rectangle control

The conclusion fails with only two unoriented facet-normal directions. Choose
0<a<1, b,d>0, g(x)=x-x^3/3 on [-a,a], and f1=g^(-1). Let

    K=[g(-a),g(a)] x [-b,b],     Q=[-1,1] x [-d,d],
    f(q1,q2)=(f1(q1),q2).

Then g'(x)=1-x^2>0 and the cotangent image is exactly

    |x1| <= a,  |x2| <= b,  |y1| <= 1-x1^2,  |y2| <= d.

This is convex because 1-x1^2 is concave. The map f is nonlinear. It is a
symplectic map, so volume and every intrinsic symplectic capacity remain exactly
fixed. The planar lens has area 4(a-a^3/3), equal to the area of the original
(q1,p1) rectangle; the other symplectic plane has area 4bd. Their symplectic
product has EHZ capacity equal to the smaller planar area. Choosing
bd=a-a^3/3 gives ratio 1/2 throughout the nonlinear family. This exact control
shows that the obstruction detects projective rigidity of the polygonal normal
fan, rather than merely asserting that nonlinear maps fail to preserve convexity.
The capacity-of-symplectic-product formula is an external standard result;
the convexity obstruction itself does not depend on it.

## Simpler independent obstruction

For any continuous g:K -> R^2, the fiber-translation image

    {(q,p+g(q)): q in K, p in Q}

is convex only if g is affine. Convexity implies
Q+(1-t)g(q0)+t g(q1) is contained in Q+g((1-t)q0+tq1).
A bounded convex body cannot properly contain a translate of itself: support
functions in both opposite directions force the translation vector to vanish.
Therefore g obeys Jensen equality. Continuous Jensen maps are affine.
For Hamiltonian shears g=gradient F, the surviving affine linear part is
symmetric and produces a linear symplectic map.

## Decision implication and remaining quantifier

A nonlinear equality-construction group should avoid pure fiber translation and
cotangent lifts when working from HKO. It must use a map mixing base and momentum
coordinates or permit varying fiber combinatorics. A genuine construction needs
both convexity and equality, and must show inequivalence to the existing linear
symplectic orbit. No claim about arbitrary symplectomorphisms is established.
A counterexample to the named theorem would require a nonlinear C^3 f and a
convex image with at least three unoriented momentum-normal directions; a
rectangle example is a control, not a falsifier.
