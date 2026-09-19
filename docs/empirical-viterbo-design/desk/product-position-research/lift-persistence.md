# A planar lift can persist after losing global minimality

2026-09-18 follow-up, product-position research owner. This resolves the proposed
lift-containment criterion negatively, and identifies exactly what it does
measure. Conventions are those of [the first return](README.md).

## Result

For planar convex factors A,B with area(A)<area(B), the natural characteristic
obtained by lifting the planar boundary of A can exist, with its B-coordinate
strictly inside B, while its action is strictly above c_EHZ(A x B,J_theta).
This occurs on an open set of pairs of convex bodies, not just at a degenerate
triangle example. Therefore the first loss of the one-factor capacity formula
cannot in general be characterized by the point where that lift ceases to fit.

The finite exact construction below proves this negative statement. It does
not classify first-loss behavior or calculate a whole new family of capacities.

## What containment does measure

Let a=area(A), and define the translative circumradius

```
r = inf {r0>0 : a translate of J2 A is contained in r0 B}.
```

On an A-facet the characteristic velocity is proportional to
(cos(theta) J2 n, -sin(theta) n). Its coordinates therefore obey

```
b = b0 + tan(theta) J2 a_coordinate.
```

The resulting closed graph over the planar boundary has action a/cos(theta).
Indeed, pulling back the form matrix by the graph map a -> (a,tan(theta) J2 a)
gives J2/cos(theta), so its enclosed planar action has exactly this scale.
It is an actual generalized characteristic of the product whenever a translate
of tan(theta) J2 A fits in B. Strict fit keeps the B constraints inactive.
Thus the exact existence condition for this lifted orbit is tan(theta) r<=1.

The same condition controls a particular first variation of the HK data. For a
polygon A, its planar maximizing closure has normalized weights
lambda_i=h_i L_i/(2a), where L_i is the length of its i-th edge. Cumulative sums
of lambda_i u_i trace a translate of J2 A/(2a), using u_i=n_i/h_i. Insert B
covectors into this cyclic A order, with normalized B closure weights mu_j.
The mixed reciprocal-action coefficient is

```
z = 4 sum_j mu_j <P_j,b_j>,
```

where P_j is the chosen cumulative A sum at the insertion point. Each B facet
may choose its own insertion point. Maximizing over those points and then over
nonnegative B weights with sum mu=1 and sum mu b=0 gives

```
max z = (2/a) max_mu sum_j mu_j h_(J2 A)(b_j) = 2r/a.
```

The last equality is the linear-programming dual of minimizing the translated
containing dilation of B. Consequently the largest linear change in reciprocal
HK action when introducing B mass epsilon, keeping this planar A configuration,
is

```
(2 cos(theta)/a) [tan(theta) r - 1] epsilon.
```

This explains the containment threshold variationally. It is a local insertion
test, not a global maximization theorem. The upper bound on first loss from this
test, combined with the previous lower bound, gives

```
ell/(2a) <= tan(first loss) <= 1/r
```

when first loss means the endpoint of the initial interval on which
c_theta=a/cos(theta), with a<=b and the fixed relative identification retained.
The following example shows that the right endpoint need not be attained.
The lower bound came from the general mass-splitting theorem; the upper bound
follows because an insertion improves the reciprocal action above 1/r.

## Exact counterfamily with strict area inequality

Set

```
A = conv{(2,0),(-1,2),(-1,-2)},
R = [[4/5,-3/5],[3/5,4/5]],
B_k = k R A,               1 < k <= 21/20,
tan(theta) = 4/5.
```

Then a=6 and b=6k^2>a. Direct support calculation gives
r=6/(5k), hence tan(theta) r=24/(25k)<1. The lifted planar boundary therefore
exists with a strict margin inside the other factor.

Use the cyclic six-facet word (0,3,2,5,1,4), where A facets come first in the
stored list and each polygon's facets follow its given vertex order. Give the
three A facets total weight t=1/5 and the three B facets total weight 4/5,
uniformly within each factor. All weights are positive and both closure
equations hold. The within-A, within-B and mixed coefficients are exactly

```
x=-1/6,       y=1/(6k^2),       z=2/(3k).
```

After dividing reciprocal action by cos(theta), this feasible datum gives

```
q(k) = -1/150 + 8/(75k^2) + 32/(375k) > 1/6.
```

Indeed the last inequality is equivalent to
65k^2-32k-40<0; the polynomial is increasing for k>=1 and still negative at
21/20. The HK formula now proves

```
c_theta <= 1/[cos(theta) q(k)] < 6/cos(theta).
```

The rightmost quantity is the action of the persisting lifted orbit. A single
feasible HK datum suffices; exhaustive capacity evaluation is unnecessary.
The witness's reversed within-A contribution is negative. It wins through
mixed interaction and the B contribution, rather than by a small insertion
into the original planar A configuration.

For example, k=21/20 retains a strict area gap and strict margins in both the
containment and action inequalities. Capacity, planar areas and translative
circumradius are continuous under small Hausdorff perturbations of full-
dimensional convex bodies. Thus all three strict inequalities persist nearby.
This includes perturbations with arbitrarily many polygon edges and smooth
strictly convex approximations. The general insufficiency of containment is
therefore robust; no triangle-specific equality is needed for that conclusion.

## Certificate and scope

[lift_persistence.py](lift_persistence.py) uses rational arithmetic to verify the
chosen word, positive closure weights, action inequality and an explicit
translation of the lifted triangle. It records all support-facet checks in
[lift-persistence.json](lift-persistence.json). The k-dependence above is
algebraic: B covectors scale as 1/k, so their within coefficient scales as
1/k^2 and their mixed coefficient as 1/k. The interval polynomial check extends
the witness to the full interval. The certificate is an exact feasible-action
upper bound, not a computation of the exact global capacity.

Run from the checkout root:

```
PYTHONDONTWRITEBYTECODE=1 python3 docs/empirical-viterbo-design/desk/product-position-research/lift_persistence.py
```

The independent `mass_split_audit` agent checked the containment LP, first-
variation normalization and lift signs, and independently reconstructed the
rational dual geometry and witness coefficients. It found no gap. At k=21/20
it confirmed q-1/6=31/6615>0, while the maximal local insertion derivative is
strictly negative. No general producer or additional population campaign was
needed. The code and mathematical argument remain separate evidence layers;
neither is claimed to be a formal proof-assistant verification.

## Targeted prior-art check

The independent audit read the full text and relevant statements/proofs of
[Zediker's 2023 thesis](https://egrove.olemiss.edu/etd/2602/). Its principal bounds
concern rotated hypercubes and minimum-area orthogonal complex shadows; it did
not supply the present tilted arbitrary-factor criterion. In
[Haim-Kislev–Ostrover, Proposition 1.5](https://arxiv.org/html/2111.09177), the
symplectic p-product formula keeps factors symplectically orthogonal, unlike this
changing mixed pairing; introductory definitions and results were inspected.

[Abbondandolo–Edtmair–Kang, Corollary 1](https://arxiv.org/html/2412.01777v1)
already equates nonlinear cylindrical and EHZ capacities for all convex bodies
in R4. The natural cylinder over A in the present coordinate splitting is a
specific containing cylinder, so equality with its capacity is a different,
more restrictive statement. The audit read the introduction through this result.

These checks found no direct prior-art match to the initial mass-splitting
regime or the present product-specific counterexample. They do not establish
novelty. The mathematical input remains HK Theorem 1.1; the deductions and
certificates are local research results pending coordinator promotion.

## Decision

The proposed geometric shortcut has now been resolved: lift existence and
global capacity minimization diverge robustly. There is no reason to spend the
next window attempting to prove containment sufficiency or cataloguing more
triangle profiles. The general compatibility question remains open, but would
need a sharper purpose than rewriting the full HK optimization in geometric
language. Return this packet to the desk for synthesis with the representation
and relation-discovery lines before launching another investigation.
