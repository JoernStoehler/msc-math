# Independent review of the F11 reconstruction and polynomial bridge

4 October 2026, 21:38 UTC. Receiving owner: root / research leaf. Owned output: this note only.

The durable ZIP in `docs/pro-handoffs/hko-f11-complete-20261004/` is byte-identical to the source of the earlier safely extracted scratch packet. I inspected its exact helper and verifier source, the stored dimension/cone metadata, and the retained timing ledger before executing anything. I did **not** run a certificate verifier, discovery job, producer, or full replay. Two small read-only arithmetic/metadata commands took about 0.024 and 0.026 seconds respectively and wrote no files. One loaded the inspected `integer_bernstein.py` helper (pure arithmetic and monomial-index initialization; no command entry point or writes), not the verifier pipeline.

## Outcome

**No failed continuum implication or counterexample was found.** The two main bridge uncertainties named by my first review can now be narrowed substantially:

1. The literal section derivative and duplicate aggregation in `calc_exact` agree with an independent differentiated-constraint derivation.
2. The polygon moment formula, interval normalization, denominator clearing, and degree-nine homogeneous polynomial construction agree with an independent algebraic derivation valid throughout each sign cell. This conclusion does not rest on the verifier's one-point-per-cell crosschecks.

The candidate proof still needs complete finite-predicate acceptance/replay and the separate inherited endpoint/other-face audit. Those checks were not run here. This review strengthens the interpretation of the packet's finite predicates; it does not certify all stored rows or all 195,140 coefficient signs.

## A. Actual feasible-section derivative

Let ordered normals be a_i with weights beta_i, C=(a_i;1), and Q=sum_(i<j) beta_i beta_j omega(a_i,a_j). At the base, the exact constraint is C beta=e. A selected invertible five-column minor continues pivot weights while holding all others fixed.

Write g_beta=H beta, where H_ij=omega(a_min(i,j),a_max(i,j)) for i!=j and H_ii=0. This is the symmetric weight Hessian of the literal ordered sum. The source computes ell=C_I^(-T)(g_beta)_I and checks C^T ell=g_beta. For a perturbation of normal a_i by h,

    C d beta = -(beta_i h, 0),
    g_beta . d beta = ell . C d beta = -beta_i ell_(1:4) . h.

The explicit normal variation is

    beta_i [omega(P_i,h) + omega(h,F_i)],

where P_i=sum_(j<i) beta_j a_j and F_i=sum_(j>i) beta_j a_j. Closure gives F_i=-P_i-beta_i a_i, so this becomes

    beta_i omega(2 P_i + beta_i a_i, h).

Therefore

    grad_(a_i) Q = beta_i [(-v_p, v_q) - ell_(1:4)],
    v = 2 P_i + beta_i a_i.

This is exactly `calc_exact`'s prefix/explicit/multiplier formula in the stated (q,p) coordinates. The factor c=2 grad_b Q/Q_H follows by differentiating (Q_H/Q)^2. When b=a_0(A)+tD, old motion of a_0 contributes both grad_(a_0) Q and grad_b Q, so summing those two entries is necessary and matches the source. G=-2 grad_old Q/Q_H-DV/V_H then has the claimed sign and normalization.

Strict beta positivity and the invertible minor provide actual smooth feasible sections; no optimizing-branch claim is required. Actual stored data have consistent dimensions: all 710 family beta vectors and directions match their word lengths; all their pivots contain five distinct in-range indices; all 65 old seeds have matching word/weight lengths. This was a metadata check only, not positivity, stationarity or touching verification.

## B. Affine paths and cancellation-response policies

For beta(s)=(1-s)beta_start+s beta_end, positivity holds for every s>0 if beta_start>=0 and beta_end>0. The fixed constraint matrix and its pivot inverse do not depend on beta. Consequently closure and stationarity are affine identities in s. Checking them at s=1/2 and s=1 proves them identically; the quadratic Q is constant if it equals Q_H at 0,1/2,1. The code's two positive nodes and three objective nodes therefore have the advertised algebraic strength.

The normal derivative is quadratic in beta, hence quadratic in s. If C(0)=0, then

    C(s)=s C1+s^2 C2,
    C1=4 C(1/2)-C(1),
    C2=2 C(1)-4 C(1/2).

The source formulas are correct. The corresponding quadratic Bernstein middle control for G is

    B1=2 G(1/2)-(G(0)+G(1))/2,

and its monomial coefficients 2(B1-B0), B0-2B1+B2 also agree with the source.

For the actual vertex depth x_v=(D.v)_+ and s_v=10 x_v, C1=a_v v gives

    lambda C(s_v).D = x_v^2 [10 lambda a_v + 100 lambda C2.D].

Thus the av/bv accumulation in `verify_policies` reconstructs Phi correctly. Old rows and their corrections have zero added-normal response. Anchor mixing simply multiplies Phi by 1-mix, exactly as the source does.

The polynomial correction check has the right meaning: E maps weights to (mass, forty aggregate gradients). Subtracting (1,0,...,0) from the constant coefficient and requiring E_sub delta=-residual at **each** of the 21 monomial coefficients yields mass one and exact all-forty cancellation throughout the independent cube. Nonnegative old Bernstein coefficients give nonnegative original old weights. Since every s_v and s_v^2 is at most one in magnitude on the cube, sum of absolute correction coefficients bounds every possible negative correction. The tested mix*anchor-(1-mix)*bound inequality therefore suffices for nonnegative corrected weights everywhere, without sampling the cube.

At s_v=0, the path may not be an actual smooth positive section. Approaching it by positive s_v, then using the positive old anchor to correct the small aggregate residual, gives closure(C); strict domination for a fixed D survives this approximation. The rank-25 symmetry-annihilator and anchor argument from the first review remains a separate finite-premise dependency, but there is no new boundary-section assumption in this policy argument.

## C. Independent proof of the polygon moment formula

Let W be any convex CCW polygon, e_i=w_(i+1)-w_i, f(w)=c+p.w, and assume p.e_i!=0. Choose a constant vector q with p.q=1. The divergence theorem applied to q f_+^3/3 gives

    integral_W f_+^2 = (1/3) sum_i det(q,e_i) integral_0^1 f(w_i+t e_i)_+^3 dt
                     = (1/12) sum_i det(q,e_i)/(p.e_i)
                                      [f(w_(i+1))_+^4-f(w_i)_+^4].

The coefficient of f(w_i)_+^4 is

    (1/12) [det(q,e_(i-1))/(p.e_(i-1)) - det(q,e_i)/(p.e_i)]
    = -det(e_(i-1),e_i)/[12 (p.e_(i-1))(p.e_i)],

because p.q=1. This proves the packet's formula (12) **as an identity for every such c,p**, including arbitrary positive-part sign patterns. The positive-part primitives used here are continuously differentiable to the needed order. Degenerate projections follow from continuity of the original integral, not evaluation of its singular rational representation.

For the interval factor, write its endpoint constants c_min,c_max with difference a>0. Direct one-dimensional integration gives

    integral_0^1 (c_min+a s+p.w)_+ ds
      = [(c_max+p.w)_+^2-(c_min+p.w)_+^2]/(2a).

The facet's normalized total mass is 2/5, so division by Area(W) produces exactly

    v(D) = [I2(c_max,p)-I2(c_min,p)]/[5 a Area(W)].

The source's denominator and factor 60 Area(W) are thus correct. This independently establishes the continuum integral identity; a successful exact point test in each cell is only a supplementary implementation check.

Six independent rational tests compared the formula with separately implemented clipped-polygon triangle integration for an asymmetric integer pentagon, p=(3,-2), and offsets -13,-9,-1,0,3,12. All agreed exactly. These six tests support the implementation convention but do not supply the universal quantifier; the divergence derivation above does.

## D. Cleared polynomial and homogeneous scaling

Inside a normalized tilt cone, the interval oscillation a is the first tilt barycentric coordinate u. Let L_i=sigma_i p.e_i>0 with sigma_i the fixed interior projection sign. On a depth-sign cell, x_(q,i)=D.(U_q,W_i) has fixed sign. The source uses

    k_(q,i) = -det(e_(i-1),e_i) sigma_(i-1) sigma_i
               * endpoint_sign(q) / [60 Area(W)],

where endpoint_sign=+1 at q_max and -1 at q_min. It obtains this using the absolute determinant of (-e_(i-1),e_i); strict convex CCW W makes that absolute determinant equal det(e_(i-1),e_i). Hence

    N = sum_(active q,i) k_(q,i) x_(q,i)^4
                                product_(j notin {i-1,i}) L_j,
    B = u product_i L_i,
    v = N/B.

The cleared gap is

    P = B [Phi - gamma mu^2] - N, gamma=1/1000.

Phi has degree <=3, B has degree 6, and N degree 7. This is precisely the source's degree-nine expression, with N homogenized by multiplying by (sum lambda)^2 and the quadratic margin first homogenized to degree three. On simplex barycentric coordinates sum lambda=1, this is equality, not a bound.

The integer conversion is also consistent. If pden clears the policy coefficients and gden clears the affine vertex coordinates, the response polynomial has denominator pden*gden^3 and multiplication by B raises it to pden*gden^9. The volume numerator initially has denominator pden*gden^7, so the source's multiplying it by gden^2 before subtraction is correct. All these clearing factors are positive integers.

Nonnegative degree-nine homogeneous monomial coefficients imply P>=0 for nonnegative barycentric coordinates. Equivalently one may divide coefficients by positive multinomial factors to get Bernstein coefficients. No stronger assertion about polynomial nonnegativity being necessary is used.

I compared 100 products in the inspected integer ring helper against separately implemented degree-six polynomial multiplication followed by the relation tau^4=10tau^2-5; all matched. A degree-nine product of nine rational-coefficient affine forms matched a separate exponent-dictionary convolution at every monomial. These are limited arithmetic checks, not reconstruction of the actual coefficient list.

## E. Cover and boundary implication

The arrangement algorithm's logic is adequate in principle: every vertex of a bounded full-dimensional sign-cell polytope is the intersection of three independent defining planes. Enumerating all such intersections, retaining those in the prism, and assigning consistent signs therefore includes every cell vertex. Full affine rank selects full-dimensional cells. Each actual two-dimensional constraint face is convex-hulled in an injective coordinate projection, triangulated, and coned to the cell vertex barycenter. The strict interior barycenter check supplies a proper star triangulation. Lower-dimensional sign cells lie in the closures of full-dimensional cells and follow by continuity. Midpoint tree splits then preserve coverage.

The actual policy list is in cone order 0,...,19. The earlier metadata check confirmed the twenty selected root triangles represent twenty distinct prism IDs. This matters because the helper selects cones positionally; the present data satisfy that convention. The source derives the complete cone domain from the polar decagon of W-W, so a finite cover of these cones covers all normalized tilt directions.

P/B>=0 gives the claimed original gap only where B>0. At a boundary where u=0 or a projection factor vanishes, one takes limits of the original continuous v and the polynomial Phi from a neighboring interior. This is valid for each fixed cone policy. Consequently mu>0 still gives the strict gamma mu^2 margin even on a degenerate tilt boundary. One does not divide by zero or infer a positive uniform lower bound as mu->0.

## Remaining exact work and source-based workload

No new proof-search obligation emerged from this review. The unperformed task is acceptance of the finite premises (stored section feasibility/touching/stationarity, mass/gradient coefficients, exact cover topology, and coefficient signs), plus the separate inherited analytic review. In the packet's retained environment, the core replay took about 146 seconds, collar about 36 seconds, and inherited ridge/endpoint predicates about 4+7 seconds. These are source measurements, not predictions or monetary estimates for this workstation. I did not run them. Reimplementation and human/agent review time are unmeasured.

For a second analytic reader, the remaining narrow high-consequence task is inherited `ENDPOINTS.md` E13--E17 and the moving two-face cap parametrization: verify the relative o(delta^2/s) errors under e/s->0, delta/s->0 and r=O(delta^2/s), then label interchange. This bridge review found no reason to reject the new universal interior certificate or repeat broad Pro research.
