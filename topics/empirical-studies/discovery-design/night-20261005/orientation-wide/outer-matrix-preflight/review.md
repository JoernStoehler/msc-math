# Independent general-matrix cone preflight review

New stage received and full contract read 2026-10-05 02:11 UTC. Receiving
owner: `direct_cover_lead`. Owned outputs: this review and independent review
code/evidence in this directory only. Old certificates and other owners'
files are preserved. This stage does not prove positivity or enumerate rays.

## Physical and scaled transfer

Let tau²=5-2sqrt(5), tau>0, and S=diag(1,tau). Source computational points
p have physical coordinates Sp; source regular covectors N have physical
coordinates S^{-T}N. For a general physical normal transformation M, set
L=S^T M S^{-T}. Then the transformed computational covectors are LN,
Y_comp=L^T K_comp, and S Y_comp=M^T K_phys. Thus the prescribed Y=M^T K
transfer is exact in physical coordinates, detL=detM, and area(Y_phys)
=|detM| area(K_phys). The seven-variable coefficient inventory may use the
four arbitrary real entries of L; its coefficients stay in Q(sqrt(5)).
Physical M and L are conjugate parameterizations, not identically labeled
matrix entries when off-diagonal entries are nonzero.

The accepted scaled chart C_u=[[1,-u],[tau²u,1]] conjugates to
[[1,-tau*u],[tau*u,1]]=R_theta/cos(theta), with tan(theta)=tau*u. Hence
L=diag(x,1)C_u reproduces the earlier fixed-axis slice and determinant
x(1+tau²u²). This is a source/control identity, not a general-matrix bound.

For detL<0, let J=diag(1,-1). Because J N_i=N_{-i mod5}, L'=LJ has
positive determinant and the same transformed ray set after relabeling
h'_i=h_{-i}. The regular quadratic area P is reversal invariant, the
five edge inequalities are permuted, and closure circuit l maps to 4-l.
Consequently the negative determinant branch is genuinely covered by
reflection/permutation; its physical ratio uses |detL|. Singular matrices do not parameterize admissible bodies. They can lie strictly
inside the polyhedral coefficient cone; det=0 separates the admissible
determinant-sign sectors rather than necessarily bounding that cone.

## Complete fit and capacity normalization

Each of the five positive closure circuits has three active rays. For each
of the ten source templates, the sum of its three transformed supports is
the maximum over every independent choice of a template vertex on those
rays. Each choice is a linear form in the four entries of L. Complete
capacity at least one therefore requires every expanded choice inequality
lambda_l.h >= sum_i lambda_li (L N_i).v_choice(i), for all five circuits
and all ten templates, using the inherited complete fitting/capacity input.
No maximizing-label pruning belongs to this preflight.

With actual capacity c>0 and canonical supports h, the ratio is
c² |detL|/(5P(h)). Normalizing supports to h/c gives complete capacity
exactly one and P(h/c)=P(h)/c². Therefore proving the proposed inequality
on the complete capacity-at-least-one cone would imply the desired ratio
bound without dropping the c² factor. This preflight does not establish
that inequality.

Gauge h_3=h_4=0 is obtained by an explicit translation of Y: solve the
two independent rows N_3,N_4 for its translating vector. This leaves three
support coordinates. Closure sums are invariant because lambda_l.N=0;
edge sums and P are invariant because N_{i-1}+N_{i+1}=2a N_i,
a=(sqrt(5)-1)/4. Canonical edge inequalities retain zero-edge and
lower-dimensional Y boundaries rather than silently requiring strictness.

Exact source binding, coefficient inventory, positive normalization,
constraint rank, lineality and strict-feasibility audit passed independently
at the 02:18 checkpoint. The next burden is quadratic nonnegativity on the
seven-dimensional cone with the admissible determinant/body conditions;
rank, sample rays and a PSD label do not prove it.

## Independent source and packet acceptance, 02:18 UTC

`review_cone.py` uses separate rational-pair arithmetic and independently
reconstructs all ten source templates, regular normals and five closure
rays. Each computational normal has physical dual-metric norm one. All
1,350 original source/vertex-choice tuples are bound to the previously
accepted catalogue by the exact scalar-chart pullback: a general linear
fit coefficient [A,B,C,D] becomes [D,A,tau²C,-B] in the accepted
[1,x,u,xu] convention. The earlier HKO canonical equality point passes
the complete general cone and exact determinant/area target as a control.
No earlier producer or full certificate was rerun.

Every occurrence in `cone.json` was rederived and checked against its
original positive normalization divisor. The measured inventory is:

- 270 fitting inequalities per closure circuit, 1,350 total; five edges.
- 605 distinct positively normalized fit rows plus five edge rows: 610 total.
- Coefficient field Q(sqrt(5)), maximum rational numerator/denominator
  bit-size five under the retained normalization.
- Full constraint rank seven, verified independently and on the retained
  seven-row rank witness; common row kernel and cone lineality are zero.

The exact strict witness is L=I and constant regular supports one. Its
translated h_3=h_4=0 gauge is

    h=((5-sqrt(5))/2, sqrt(5), (5-sqrt(5))/2, 0, 0).

All 610 inequalities are strict, proving full dimension independently of
rank. The same h with L=0 is also strict and demonstrates that singular
matrices can be **interior** points of the coefficient cone. Retained
witness minimum slacks were independently checked.

The exact reflection/re-gauging seven-variable map preserves all 610 rows;
the retained complete row permutation was independently checked. The
retained canonical edge and quadratic area matrices, source normals and
all template vertices were also checked. A direct fixed-Q shoelace proves
2 tau² area(Q_comp)/kappa=5, kappa=(5-sqrt(5))/4, independently binding
the physical ratio constant five. The final replay took 5.882 seconds.

Read-only reproduction:

```sh
python -B topics/empirical-studies/discovery-design/night-20261005/orientation-wide/outer-matrix-preflight/review_cone.py
```

## Coordinate correction and accepted reduction boundary

The lead report initially inverted the off-diagonal scaling in its explicit
physical inverse matrix. Review identified and the lead corrected it:
L=S M S^-1 implies **M=S^-1 L S**, namely
M=[[L00,tau L01],[L10/tau,L11]]. The defining conjugation, producer
coefficients and source scalar chart were correct throughout. The corrected
report agrees with the independently reviewed transfer above.

Canonical edge inequalities imply P(h)>=0 by the actual regular-fan
polygon-area interpretation. Therefore Q_+=P-detL/(3+sqrt(5)) is
automatically nonnegative on detL<=0. Right reflection identifies every
det-negative physical body with a det-positive representative and preserves
the coefficient cone and P. Thus proving Q_+ nonnegative on the entire
cone is equivalent to proving the desired bound on its admissible
det-positive body sector; the trivial det<=0 portion loses no body. This
is a reduction of the next obligation, not a proof of that obligation.

**Accepted preflight:** the complete inherited fitting/capacity model for
the selected general normal matrix family reduces exactly to the retained
seven-variable homogeneous linear cone and quadratic target, with correct
physical coordinates, determinant signs and capacity-squared normalization.
No transfer or inventory predicate remains unresolved. Quadratic
copositivity, equality classification, ray enumeration, redundancy pruning,
CAD, optimizer search and a general-matrix HKO theorem were not performed.
The source-completeness and canonical-area inputs are explicitly inherited;
every newly generated source-to-cone arithmetic arrow was independently
checked.

## Ambient target control, 02:21 UTC

The retained 7x7 positive-determinant target matrix was independently
reconstructed and checked. Its support block has exact leading principal
minors (1-sqrt(5))/4, (1-sqrt(5))/8, and 1/8; their signs and LDL pivots
give one positive and two negative directions. The determinant block
consists of two independent nonzero off-diagonal 2x2 pairs, each with one
positive and one negative direction. Thus the ambient inertia is exactly
(3 positive, 4 negative, 0 zero). This is a constant algebraic workload
control and explains why ambient PSD is not the desired proof predicate.
It establishes no cone positivity. The final independent packet replay,
including these new constant fields, passed in 5.971 seconds.

Reviewer work completed at 02:21 UTC, before the 02:28 integrated cap.
No additional scientific execution is proposed within this preflight.
