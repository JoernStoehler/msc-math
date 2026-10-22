# Sharp affine heptagon product inequality

22 October 2026. Root derivation during the ten-minute sprint, conditional on
the [new exact 7 by 7 tensor certificate](tensor-extensions-20261022/report.md) and the maintained product reduction.
External novelty and human acceptance remain unassessed.

Let P be the circumradius-one regular heptagon, with outward unit normals
n_i=(cos(2pi i/7),sin(2pi i/7)), support height h=cos(pi/7), and area
S=(7/2)sin(2pi/7). Let D=conv{+-n_i}; it is a regular fourteen-gon of
circumradius one. Write alpha=pi/14 and theta=pi/7.

The full tensor certificate, combined with the existing sparse product
decomposition and affine covariance, gives for invertible G,H

    c_EHZ(GP x_L HP)=(1+h)^2/m(M),
    M=G^-1 H^-T,  m(M)=max_ij |n_i^T M n_j|.

Consequently the sharp inequality within this independent-affine class is

    sys(GP x_L HP) <= (1+cos(theta))^3 /
                     [49 sin(theta)^2 cos(theta)^2].

Equality holds exactly when M is a nonzero scalar multiple of
R_alpha U, with U a Euclidean symmetry of D. Equivalently, after division
by m(M), M carries D onto D^polar. Translations of either factor do not
change the statement.

## Proof of the sharp optimization

Homogeneity allows m(M)=1. This means exactly MD subset D^polar: every vertex
of MD satisfies every polar inequality |n_i^T x|<=1. Therefore

    |det M| area(D) <= area(D^polar).

Since D is regular, D^polar=sec(alpha) R_alpha D. Thus
|det M|<=sec(alpha)^2, and this is attained by M=sec(alpha) R_alpha.
Any such invertible M is realized by choosing G=I and H=M^-T.

Equality of areas for nested full-dimensional convex bodies forces
MD=D^polar. The linear maps taking D to D^polar are exactly
sec(alpha) R_alpha U, where U is a linear automorphism of D. Such an
automorphism permutes the vertices; since their second moment is a positive
scalar multiple of the identity, U U^T=I. Hence U is a Euclidean symmetry
of D, proving the equality classification.

The capacity formula and volume give

    sys=(1+h)^4 |det M| / [2 S^2 m(M)^2].

Substitute sec(alpha)^2=2/(1+h), S=(7/2)sin(2theta), and
sin(2theta)=2sin(theta)cos(theta). The sharp constant simplifies to the
displayed expression.

## Boundary of the conclusion

The area/equality argument is analytic and requires no optimizer search.
Its capacity input is a finite, exact computer-assisted containment theorem
for 7 by 7. It does not establish the all-odd independent-affine capacity
formula, nor the arbitrary-polygon Viterbo conjecture. No claim about sharp
5 by 7 or 5 by 9 affine optimization follows from this equal-factor argument.
