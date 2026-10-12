# Uniform moving-endpoint cap estimate in global slack coordinates

Lead retention, 2026-10-05 00:04 UTC; the global slack-coordinate route was
independently supplied by `conditional_mechanism`. This note proves the cap
premise for the selected moving p1 endpoint, not the full ratio theorem.

Let the old normal chart be `A=A(v,s)+zW(v,s)+Yy`, with `(v,s)` in a small
compact patch about `(1/2,1/12)`, and put `e=|z|+|y|`. The ten old inequalities
have support height one. Their rows differ byO(e) from the equality normals,
uniformly in this patch. All corresponding bodies lie in a common bounded
ball; this follows from boundedness of the equality bodies and continuity of
their positively spanning normal systems.

Put `u=1−v+v²`, `c=u−eta`, and `D=(0,−2c,1,0)`, where `0<=eta<=eta0`.
The two p facets at the right p vertex `(u,0)` are labels6 and9. Define actual
old slacks `t_i=1−a_i(A).x` for `i=0,6,9`, and retain q1 as a fourth coordinate.
All three slacks are nonnegative on the old body. At the equality family,

    a0=(0,2,0,0),
    a6=(0,0,1/u,1/v),
    a9=(0,0,1/u,1/(v−1)).

Direct substitution gives the exact global identity

    D.x=eta+(u−eta)t0−uv t6−u(1−v)t9.                 (1)

The four-row coordinate matrix consisting of the q1 row and these three
normal rows is nonsingular. At the equality family its determinant is

    −2/[u v(1−v)].

Its absolute value stays bounded away from zero on the compact parameter
patch, and therefore also for all sufficiently small old-normal motions.
Both the coordinate matrix and its inverse stay uniformly bounded.

Consequently D has a unique regular expansion in these actual moving
coordinates. Equation(1) and matrix perturbation give

    D.x=eta+epsilon0+epsilon1 q1+b0 t0−b6 t6−b9 t9,

where `|epsilon0|+|epsilon1|<=C e`, b0 is uniformly bounded, and
`b6,b9>=b*>0`. These bounds hold uniformly also at eta0 and eta=0; eta occurs
only in the bounded coefficient c. Let `|q1|<=R` on every old body.

A removed point satisfies `t0<tau D.x`. Thus

    (1−tau b0)t0+tau b6 t6+tau b9 t9
        < tau[eta+epsilon0+epsilon1 q1]
        <= C tau(eta+e).

For one common sufficiently small tau window, `1−tau b0>=1/2`. Every term
on the left is nonnegative. With `h=eta+e`, all removed points lie in the
single global coordinate box

    −R<=q1<=R,
    0<=t0<=C0 tau h,
    0<=t6<=C1 h,
    0<=t9<=C1 h.

The coordinate Jacobian is uniformly bounded. The volume of this box is
bounded by `C2 tau h³`. Therefore

    DeltaV <= C2 tau(eta+e)³.                         (2)

This proof covers the entire removed set, including endpoints of the q edge;
it needs no projection onto the old facet, no pulling triangulation and no
assumption that a fixed clipping sign pattern persists. If no point is
removed, the same bound holds trivially. Genuine-eleven-facet assumptions are
needed for the final target domain, not for this geometric estimate itself.

Combining(2) with the independently checked transverse-cap reduction shows
that the old witnesses already control every sufficiently small path with
`e>=eta`. Every unresolved path with eta>0 must satisfy

    |y|+z² <= C tau eta³,
    |z| <= C sqrt(tau) eta^(3/2),
    |y| <= C tau eta³.

Only this relative region needs the new fraction-adapted capacity section.
No unproved moving-body cap moment or smoothness across clipping chambers
has been used.
