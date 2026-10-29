# Independent audit of the explicit improving parameter

29 October 2026. Reviewer `/root/hko_ascent_math_audit`; root owns integration.

**PASS, with the corrected minor lemma.** The argument proves

    q(t) <= q0+(2q0/125)t+10^15 t^2       (0<t<=10^-30),
    sys(H_t)/sys(H) > 1+t/200.

This is a bound for the global optimum, not merely for a selected feasible
branch. The square-root rounding argument supplies the missing bridge that
a positive constraint minor alone would not supply.

## Frozen inputs and the correction

The effective-step draft was hashed before inspection; its mathematical body
remained unchanged. A later status-only update is recorded below:

    effective-step.md
    a70eef8a6ea4cfc3e84fbb47b1e7dc1f78dd438f856766bb3117ea966e1074fb

The initially inspected minor-bound.md had hash
`6b62c610bd02efbe43033b0c7d5720a5edd001384371276e9f3ca071cd3be854`.
It contained an incorrect common-factor closure equation for the mixed bank
of retained and moved rows. I reported this to the author. The author and
derivative reviewer repaired it using

    ||sum beta_r a_r|| = t ||sum beta_r D_r|| <= (5/4)t.

The corrected final minor-bound.md inspected for this approval has hash
`777b0bec203cc5b66805746aaa4ea4bffa7e52fe18aa183e677033ea6d869701`.
It also correctly writes the moving-velocity norm as an upper bound, allowing
retained rows with D=0, and notes that division by N>1 only decreases the
unit-normal residual. These corrections preserve all subsequent constants.
Snapshots of the initially inspected drafts are in
`/tmp/msc-math/restart/hko-ascent-audit/effective-freeze/`.

The prior derivative, repeated-word reduction, and full base response
certificate are established dependencies of this audit. I rechecked their
specific interface here: every exactly maximizing base word in the twenty-row
bank, with a feasible smooth continuation, has action derivative at most
g*=2q0/125. Repetition of a base normal is allowed. No near-maximizer is
silently assumed to satisfy that exact-maximizer derivative bound.

## Global optimizer and pivot coverage

The HK formula on the fixed twenty-row presentation supplies a maximizing
simple word, with zero weights permitted. Each base normal occurs at most
twice. Because the old rows remain, H_t is contained in H and q(t)>=q0.
Thus every chosen moving maximizer meets the minor lemma's action threshold.

The near-closed pivot argument checks as follows. The action perturbation
bound follows from bilinearity, normal/velocity norms <5/4, and
sum_{r<s} beta_r beta_s <=1/2. It gives base action >q0/3>1/30. Same-factor
pairings vanish even without closure, so |B|<=25/16 gives each factor mass
>8/375. The corrected residual bound gives normalized factor residual at
most 15/256 at t<=1/1000, already smaller than 1/8; the draft's looser
3/32 bound also holds.

For the two largest aggregated directions in a regular pentagon, their
weighted resultant has length at least their total mass times cos(72 degrees)
in unit-normal coordinates. This follows by minimizing the two-vector norm
over weights at fixed total mass; the largest possible angle is 144 degrees.
If r is the remaining three directions' mass, the full mean norm is at least
(1-r)cos(72 degrees)-r. A mean below 1/8 implies r>1/16. Since all three tail
weights are at most the third largest, each top-three weight exceeds 1/48.
Returning to unnormalized weights and selecting one of at most two copies
gives an occurrence weight >1/4500 in each of those directions.

Two distinct q directions and three distinct p directions form an augmented
five-column determinant factoring into the planar q determinant and the
affine p-triangle determinant. The former exceeds 1/2; the latter is one of
(5tau+tau^3)/2 and 5tau-tau^3, both >1. Thus the determinant exceeds 1/2.
Column norms <8/5, Hadamard's bound for every cofactor, and Frobenius control
of the adjugate give spectral inverse norm <66. In dimension five this gives
infinity norm <sqrt(5)*66<200. The selected occurrence weights are positive
uniformly over all moving maximizing words; no finite-order scan is assumed.

Repairing closure at zero while holding complementary weights fixed changes
each pivot weight by at most 2Mt. Hence beta0 is base feasible and each pivot
weight is >c=10^-4. Its other weights remain nonnegative. Normalization is
included in the augmented system, so beta0 is a probability vector.

## Chart derivatives and Lipschitz constants

The selected constraint matrix, not an optimizer Hessian, is inverted.
On 0<=s<=10^-8, its perturbation has infinity norm <=10s and the Neumann
bound gives inverse norm <=2M. The exact difference identity gives
||beta(s)||_1<=1+20Ms<2, even without assuming intermediate positivity.
Differentiating the constraints yields the stated bounds

    ||beta'||_1 <=40M,       ||beta''||_1 <=1600M^2.

For the objective matrix, entries of H,H',H'' are bounded by 4,8,8.
Directly differentiating F=beta^T H beta/2 gives

    F'' = beta'^T H beta' + beta''^T H beta
          +2 beta'^T H' beta + beta^T H'' beta/2.

Its absolute value is at most 19200M^2+1280M+16, below 2R for
R=10^5 M^2. This verifies the Taylor remainder; the positive minor is not
being used as a substitute for this differentiation.

At zero, u_I=-B_I(0)^-1 D beta is linear in beta and has L1 norm <=10M.
The direct quadratic term in g has absolute value <=4 and Lipschitz constant
8 on probability vectors. The remaining bilinear term has absolute value
<=40M and Lipschitz constant <=80M. Thus |g|<=44M and the stated Lipschitz
constant 88M both hold. Monotonicity and the Taylor upper bound imply
d=q0-Q0(beta0)<=44Mt+Rt^2<10^-20.

## Exact action gap and rounding in the same word

Each occurrence closure polytope has at most 40 vertices: a vertex needs
three distinct normal directions forming one of five positive triangles,
and each direction has at most two occurrence choices. Duplicate columns
cannot both belong to a vertex support. Subwords impose coordinate-zero
faces and introduce no additional vertices. Pairing such q and p vertices
gives a six-word on six distinct base normals, so all values belong to the
same 3000 primitive orders as in the complete catalogue.

I independently reran the 3000 primitive actions with the already audited
exact_core field arithmetic. The run found 50 winners and 13 distinct action
values; its minimum positive deficit is exactly

    Delta=(-tau+3tau^3)/20 >1/50.

This establishes the newly needed gap predicate, rather than inferring a
gap from floating-point separation. The result is retained in
`/tmp/msc-math/restart/hko-ascent-audit/effective-gap-recheck.json`.

The deficit identity

    d=4q0(s-1/2)^2+4s(1-s)(q0-S)

has nonnegative terms. Since Q0(beta0)>q0/2 and S<=q0, it gives
4s(1-s)>1/2, bad-pair product mass epsilon<=2d/Delta, and
2|s-1/2|<=sqrt(d/q0).

Deleting all marginal coefficients below 2sqrt(epsilon) removes at most
80sqrt(epsilon) from either marginal. This is <1, so both renormalizations
exist. A surviving bad pair would itself have product mass >=4epsilon,
contradicting total bad-pair mass epsilon>0. The epsilon=0 case needs no
deletion. The resulting product of surviving mixtures is exactly maximizing.

Renormalization changes each marginal coefficient vector in L1 by at most
160sqrt(epsilon); passing to the convex combinations of closure vertices
does not increase this bound. Including the factor-mass change gives

    ||beta0-beta*||_1
       <=[1/sqrt(q0)+160sqrt(2/Delta)]sqrt(d)
       <2000sqrt(d).

Both beta0 and beta* use the same ordered occurrences. The distance is
<2e-7<c/2, so all five selected pivot weights remain positive at beta*.
Holding its other coordinates fixed therefore gives a feasible smooth
continuation for sufficiently small positive parameters. Applying the
already proved exact-maximizer derivative bound is now legitimate.
This supplies g(beta0)<=g*+A sqrt(d), with A=35,200,000.

## Absorption and the final systolic inequality

Maximizing -d+At sqrt(d) over d>=0 gives A^2t^2/4. Consequently the global
action remainder is at most A^2/4+R=309,764,000,000,000<10^15. All constants
are independent of the particular maximizing word and selected pivot.

For the volume, at an old boundary point the new gauge is at least
1+t f_i(x). Other rows only increase the gauge. Thus the radial volume
formula gives the upper bound in the correct direction. For u>=0,
the second derivative of (1+u)^-4 is at most 20, and 0<=f<=2/5, yielding

    V(t)/V0 <=1-4nu(f)t+(8/5)t^2.

The normalized action square is bounded by 1+2at+3*10^16t^2, a=2/125.
After multiplication by the positive volume upper bound, discarding negative
cross terms leaves quadratic coefficient below 4*10^16 on the stipulated
interval. Since eta=4nu(f)-2a>1/100 and 4*10^16 t<1/200,

    q(t)^2 V(t)/(q0^2 V0) <1-t/200.

This product is positive. Taking its reciprocal proves the claimed strict
relative improvement, including t=10^-30. The lower bound at that parameter
is 5*10^-33.

I also read and reran check_effective_constants.py, hash
`4b3439a00cd18a364f023b7fa2ac4bd4a02fbf87c1ec1e8ed43667f0c5198516`.
All exact gap and rational constant predicates passed; output is
`/tmp/msc-math/restart/hko-ascent-audit/effective-constants-replay.json`.
Those executions support finite arithmetic; the reasoning above checks the
coverage, rounding and Taylor implications separately.

## Publication recommendation and scope

The structural method is worth a short printed corollary and proof outline:
a finite primitive gap gives square-root proximity to a maximizing support,
a positive constraint pivot controls a chart, and completing the square
turns this into a global quadratic remainder. That is useful quantitative
content beyond merely choosing an absurdly small decimal.

Keep the full constant ledger, minor estimates, exact replay and the tiny
gain in the companion. The main geometric result should remain the ascent,
especially its seventeen-facet refinement. This audit does not certify a
visibly useful numerical improvement or a maximizing parameter. No canonical
source edits were made by this reviewer; the author made the identified
minor-lemma corrections, and root owns integration and final disposition.

## Final short-module consistency check

I also checked the complete short printed module
`/tmp/msc-math/restart/multicut/hko-effective-step.tex`, final SHA256
`a9f8963db5a9082c26056c94a7cfa55750e127e363aea3ff537b7aa7928de376`.
**PASS:** its global action direction, constants, volume bound and explicit
parameter match this audited companion. It explicitly restricts the result
to the twenty-row family with all ten unit-speed cuts.

At my request the author clarified the pivot lemma's Q_t>=q0/2 and t<=10^-3
hypotheses, then explicitly switched to a moving global maximizer with
t<=10^-30, repaired closure, and established deficit d<10^-20 before rounding.
The general pivot hypothesis alone would not justify rounding an arbitrary
word to an exact maximizing word. The final wording preserves the correct
order and scope of the companion's reasoning. The final paragraph also
correctly calls 5e-33 a certified lower bound on the relative gain, rather
than claiming it is the actual gain.

The author updated only the initial and final status paragraphs of
effective-step.md after approval. I compared it against the frozen initial
copy and confirmed its mathematical body is unchanged. Its current SHA256 is
`62da0c4b0e40a0651ec50e3401678e88602cdac308501e74bc2797936cf89a7f`.
This approval covers that status-updated companion as well.
