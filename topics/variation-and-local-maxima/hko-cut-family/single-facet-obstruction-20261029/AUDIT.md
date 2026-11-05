# Independent audit: arbitrary finite invariant splitting on one HKO facet

Reviewer: `/root/single_facet_audit`, 29 October 2026.
Receiving and integration owner: `/root`. Read-only review of the assigned
checkout `/workspaces/msc-math-viterbo-continuation`; no project files edited.

## Verdict and exact scope

**PASS**, with the response class and deformation bank stated precisely as
below. No mathematical gap was found in the new bound

    R(f) >= (5/4) nu(f),
    D+ log sys(H_t) = 4(nu(f)-R(f)) <= -nu(f).

For an original facet F with support-normalized normal a_F, the directly
supported deformation is each **fixed finite** bank

    H_t = { x : a_i·x <= 1 for all ten old rows,
                (a_F+tD_j)·x <= 1 for j=1,...,m },
    D_j·J a_F = 0.

Here f is max(0,D_1·x,...,D_m·x) on the labeled copy F and zero on every
other labeled facet copy. Let R be the maximum over the weak closure M of
the labeled occupation measures of all finite maximizing base words,
including repetitions, exactly as in the cited simultaneous-splitting
theorem. This is enough for the proof. The draft's unqualified phrase
“all shortest characteristics” should be replaced by this precise class:
the cited derivative equality is stated for M, and no extra coverage theorem
for a broader class of possibly non-polygonal characteristics is needed.

The new integral estimate itself holds for every nonnegative continuous
f on the quotient square that is convex in xi. Such an arbitrary continuous
function need not itself be the profile of a finite row bank; the derivative
claim applies to the finite maxima specified above. Any finite number is
allowed, but the bank is fixed while t tends to zero. The result neither
allows several perturbed old facets nor asserts a neighborhood theorem.

## Reviewed sources and execution evidence

Read the complete draft, Appendix C
`sections/hko-two-hinge-appendix.tex`, its exact checker
`certificates/hko-two-hinge/certify_amplification.py`, and
`certificates/hko-ascent/multicut-variation.tex` under
`topics/capacity-and-qp/convex-systolic-optimization/`.
Also inspected the exact arithmetic and base geometry in
`certificates/hko-ascent/exact_core.py`, the companion READMEs, and the
facet-transitive symmetry argument in `sections/hko-seventeen-facets.tex`
with its literal symmetry checker. The retained prior two-hinge audit was
consulted as component context. This was not an exhaustive repository audit
or a fresh proof of the known HKO capacity.

The reviewed draft SHA-256 is
`bbada73eeb6b8656b4e1f1680bc7a1e66434132217a6cc4b3ac752b0860f0c67`.
Other inspected source hashes are recorded in `audit-literal-constants.json`.

The following fresh replay passed:

    python certificates/hko-two-hinge/certify_amplification.py \
      --source certificates/hko-ascent \
      --out /tmp/msc-math/viterbo-continuation-20261029/single-facet/audit-amplification.json

I separately compared every literal word and endpoint vector printed in
Appendix C against that output, ordering endpoints by increasing eta. All
four match. Using the declared inherited exact number field, I checked their
action at endpoints and midpoint, positive-support augmented rank five at
each endpoint, and both new integrated ratios exactly. Those additional
results are in `audit-literal-constants.json`. These checks reuse the stated
field arithmetic; they are not a separate implementation of that arithmetic.

## Geometric and occupation checks

For F_0, the facet is the product of a pentagon edge in q with the partner
pentagon in p. Its normal q-coordinate is fixed. The characteristic direction
is (0,n_0), so invariance removes the p-coordinate parallel to n_0. The two
remaining coordinates are precisely xi and eta in Appendix C. Thus an
invariant affine depth is alpha xi+beta eta+gamma, with arbitrary constant
term supplied by a multiple of the old support normal. Retaining that old
row supplies the zero branch of the maximum. This justifies the claimed
profile and its convexity in xi.

Under facet-uniform measure xi is uniform on [-1,1] independently of eta.
The projected partner-pentagon density is

    p(y)=b(y+3) on [-1,k], and p(y)=C(1-y) on [k,1],
    k=sqrt(5)-2, b=sqrt(5)/10, C=(5+2sqrt(5))/10.

The replay verifies the actual projection levels, widths and Jacobian,
total mass one, and the stated first moment. Each facet has normalized
cone-volume mass 1/10; hence nu(f)=E f/10. There is no missing factor from
the two xi endpoints or the two eta intervals.

The four literal families have nonnegative endpoint weights and positive
interior weights. Closure is affine and their objective is quadratic in the
interpolation parameter. Its equal values Q=tau/5 at the two endpoints and
midpoint make it constant on the full interval. This equals the known HKO
optimum. KKT multipliers and reconstructed positions are affine; the exact
endpoint containment and active-facet checks therefore extend throughout
each family and each segment. They are actual shortest characteristics.

Their facet-zero segments have constant quotient coordinates (xi,eta),
including along the entire segment, because the profile is characteristic
invariant. They give the response lower bounds beta_L(y)f(±1,y) and
beta_U(y)f(±1,y), with

    beta_L=A(y+3), A=(5-sqrt(5))/40,
    beta_U=B(1-y), B=(5+sqrt(5))/40.

Other **labeled** facet copies have zero profile. A segment at a ridge does
not acquire an unintended contribution from a different facet copy. The
upper family's zero weight at y=1 is harmless: its contribution is zero,
and the integrated argument divides by beta_U only for y<1. At y=k the two
different family weights are valid; they are not assumed to coincide.
Completeness of the numerical catalogue is not a premise of this lower bound.

## New inequality and derivative

For either xi edge and every y in the applicable interval with positive
weight, beta(y) f(±1,y)<=R. Multiplication by p(y)/beta(y) and integration
therefore gives

    integral f(±1,y)p(y)dy
      <= R[(b/A)(k+1)+(C/B)(1-k)] = 8R.

Both summands are exactly 4. This was checked separately in the inherited
exact field; it also follows immediately from (sqrt(5))^2=5. There is no
assumption of convexity in eta here. Convexity in xi gives

    integral f(xi,y) dxi/2 <= (f(-1,y)+f(1,y))/2.

Averaging the two edge inequalities yields E f<=8R, so
nu(f)<=4R/5. The different responding orbits for different y or edge signs
cause no problem: each pointwise response is bounded by the same supremum.
No common occupation ensemble is claimed.

The simultaneous-splitting theorem applies to the fixed bank above: each
unmodified facet has the singleton velocity list {0}, while F has
{0,D_1,...,D_m}. The full-rank condition holds for H and also for repeated
maximizing words. Closure separates into its two planar factors; positive
Q uses both, and positive closure without antipodal normals spans each
plane. Weighting an affine dependence by the positive weights removes its
constant part and gives augmented rank five. I checked the theorem's local
feasible-section, placement, midpoint, and volume formulas for application
here and found no extra smooth-optimizer or simple-word hypothesis.

In dimension four n=2, so its logarithmic systolic derivative factor is
2n=4. Consequently 4(nu-R)<=4(nu-5nu/4)=-nu, as claimed.
The quotient density is positive on the interior of the square. A nonzero
nonnegative continuous f has positive integral even if its first nonzero
value is at the boundary, by continuity. Hence the derivative is strictly
negative whenever the induced first-order profile is nonzero.

Simultaneous pentagon rotations and the symplectic involution
S(q,p)=(J_2 p,-J_2 q) preserve H and act transitively on the ten facets.
They preserve characteristic invariance and both occupation and cone-volume
integrals, transferring the result to any one old facet.

## Integration recommendations

1. State the explicit fixed finite bank and the labeled occupation class M.
   This removes the only material ambiguity in the draft's theorem wording.
2. Keep the distinction between the generalized integral estimate and
   realizable finite-bank derivative conclusions.
3. Keep the existing exclusions: several facets, non-invariant profiles,
   old-row motions, and higher-order behavior when the first-order profile
   vanishes. No optimality of the constant 5/4 is established or needed.

No project edits, new delegation, service-tier changes, or spending were
performed. Root owns integration, retention, commit and cleanup.
