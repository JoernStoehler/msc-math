# Reflected-cover contacts collapse to a wrong pair near HKO

The frozen Minkowski-asymmetry contact-face feature recovers HKO and the exact
random-product counterexample, but it is **discontinuous at HKO**. Along an
explicit genuinely nonregular path tending to HKO, its ratio upper bound tends
to `(9+4 sqrt5)/10 =1.794427190999...`, while the true ratio tends to
`(3+sqrt5)/5 =1.047213595499...`. Stop this selector without adding supports,
fitting or running a broader population. This is an exact failure mechanism of
a new nonsmooth covering-geometry feature, not a global ratio counterexample.

## Geometry and feasible upper-bound direction

For P={x:a_i.x<=1}, let H_i=h_P(-a_i). The least reflected homothetic cover
`-(P-z) subset lambda(P-z)` is equivalent, with c=(1+lambda)z, to

`a_i.c+H_i<=lambda` for every i.

Thus the intrinsic Minkowski asymmetry is a three-variable geometric LP,

`lambda_P=min_c max_i(a_i.c+H_i)`.

Its dual maximizes `sum_i alpha_i H_i` over the normalized closure polytope
`C_A={alpha>=0:sum alpha=1,sum alpha_i a_i=0}`. Keep **all** vertices of this
geometric exposed face; do likewise for Q. For each pair from these two faces,
take its exact best word objective using the imported two-shadow theorem.
Maximize only over these selected pairs, giving Q_M, C_M=1/(2Q_M), and
S_M=C_M^2/(2 area(P)area(Q)). Since every candidate is a feasible HK closure
and word, C_M is a mathematical capacity upper bound. It is not an exact
capacity formula on general factors and does not optimize outside the two
geometrically selected faces. Floating regular controls are separate from exact
rational/algebraic assertions.

Facet relabeling and factor exchange preserve this scalar. Product-preserving
symplectic changes preserve the cross pairings. For the translated factor
`P'=P+t`, presented with its origin still interior,
`a_i'=a_i/(1+a_i.t)` and `alpha_i'=alpha_i(1+a_i.t)` preserve normalized
closure, weighted steps and `sum alpha_i H_i`; here
`H_i'=(H_i-a_i.t)/(1+a_i.t)`. This proves translation invariance of the
selected face and score, despite the center-dependent normal presentation.
Independent factor affine changes preserve the reflected-cover geometry;
paired symplectic changes preserve its HK output.

Convention clarification,17 October2026: the frozen MINKOWSKI-PLAN.md uses
the opposite signs, which correspond to `P'=P-t` (shifting the coordinate
origin by+t). Its mathematical identity is unchanged; this report specifies
the translated set explicitly to remove that ambiguity. The frozen plan is
preserved as written.

Unlike a single canonical positive closure, the face retains the complete
fivefold orbit at regular HKO. This escapes the preceding single-witness
symmetry obstruction. It differs from minimum-area outer-triangle selection
and from cross-factor width-contact anchoring: only factorwise reflected-cover
geometry selects its supports. The comparison, rubric and fixed controls were
frozen in MINKOWSKI-PLAN.md. A rigorous bound and exact regular recovery could
still conceal a catastrophically loose nonregular limit; the following check
exposes precisely that problem.

## Exact discontinuity theorem

Write d=sqrt5 and use the HKO normal presentation in POSITIVE-CLOSURE.md,
with B=JA and Q_H=1/10. Define A(t)=A+t E and B(t)=B+t F, where

```
E = [(93/128,-39/64), (3/8,1/128), (-87/128,-7/64),
     (-65/128,-49/128), (1/128,3/16)]
F = [(25/64,47/128), (-33/64,105/128), (-87/128,23/32),
     (3/32,-37/64), (95/128,-1/64)].
```

These are the fixed rational increments from the published full_s0 panel
direction, now based at **exact** algebraic HKO. This post-control explanatory
path was selected after the frozen feature failed on that direction; it is not
a fresh held-out predictive validation or a new tuned feature.

**Proposition.** There exists epsilon>0 such that for0<t<epsilon both factors
are genuine non-affine-regular pentagons, each reflected-cover contact face is
the unique closure supported on{0,1,3}, and

```
lim_(t→0+) C_M(t)/c_true(t) = (3+d)/4 =1.309016994374...,
lim_(t→0+) S_M(t) = (9+4d)/10 =1.794427190999... .
```

At t=0 the contact faces contain all five closure vertices and
S_M(0)=(3+d)/5. Hence S_M is discontinuous there. True ratios remain above
one for all sufficiently small positive t by continuity and the strict HKO
excess; this is a nonregular above-one obstruction, not just a sub-one example.

**Proof.** All consecutive normal determinants at t=0 are nonzero; primal
vertices have exactly their two adjacent active inequalities, with every other
inequality strict. Each H_i has the unique opposite primal vertex i+2
(indices modulo5). These facts persist on an interval by continuity. Thus all
five candidate closure vertices and all H_i are differentiable rational
functions of t there, with coefficients in Q(d); their positive supports and
the list of feasible closure triples remain fixed.

At t=0 all five dual objectives equal d-1. Their exact first derivatives are:

| Closure support | Factor A | Factor B |
| --- | --- | --- |
| 013 | -297/128+35d/32 | 81/512-23d/2560 |
| 023 | 1767/512-771d/512 | 1415/512-3223d/2560 |
| 024 | -1623/512+709d/512 | -75/16+1293d/640 |
| 124 | 817/512-395d/512 | 2371/512-5363d/2560 |
| 134 | 227/512-103d/512 | -1467/512+3437d/2560 |

In each column the013 derivative is strictly largest; every difference is
checked by exact ordered-field arithmetic. Finite differentiable comparisons
therefore select exactly013 for each factor for every sufficiently small
positive t. There are no other feasible closure vertices near the base.

For this one pair, independent complete enumeration of all120 cyclic six-label
words at t=0 gives

`Q_013,013=(3-d)/10 < Q_H=1/10`.

The positive angular shadow explicitly attains this value with the full word
`[0,8,1,9,2,5,3,6,4,7]` (A labels0..4, B labels5..9; zero weights may be
deleted). Its ten rays are A union JB=A union(-A), all distinct at the base,
so its cyclic angular order persists for sufficiently small positive t.
Thus the actual two-shadow feature attains the computed limiting maximum;
this is not merely a lower bound on its error inferred from all-word
optimization. Equivalently the imported two-shadow theorem applies to the
two three-entry selected closures. The maximum of finitely many continuous
word objectives is continuous for this fixed pair. Inverting therefore gives

`lim C_M/c_H=1/(3-d)=(3+d)/4`.

The true capacity tends c_H by monotonicity and uniform containment
sandwiches as the normal presentations converge; area is continuous as well.
Thus true ratio tends(3+d)/5. Squaring the capacity inflation and multiplying
by this ratio gives `(9+4d)/10` as asserted.

Finally, let e_i(t) be consecutive primal edge vectors. Affine images of a
regular pentagon have all determinants det(e_i,e_(i+1)) equal. Here the
derivative at0 of `det(e_0,e_1)-det(e_1,e_2)` is
`(3805-1553d)/512` for A and `(-4115+1537d)/512` for B. Both are nonzero,
while the contrast itself vanishes at0. Both factors are therefore
non-affine-regular for all sufficiently small positive t. This proves the
proposition. No explicit radius is claimed. ∎

The mechanism is **symmetry-face collapse**: retaining all exact ties fixes the
regular control, but an arbitrarily small generic perturbation chooses an
unfavorable sole vertex of each factor's contact face. Their geometric
asymmetry objectives do not impose the relative signed compatibility required
by capacity. The theorem concerns this frozen contact-face rule, not every
covering geometry or nonsmooth feature. Enlarging to near-active faces could
change its continuity, but no such variant is selected, tested or justified by
this result.

## Frozen finite controls

The64 exact rational geometries comprise the existing60 constructed panel,
the known exact counterexample, and three published hard random controls.
Feature values were written before loading their oracle table; the source
corpus is historically explored, so this is retrospective evidence.

| Control set | n | Median capacity gap | Largest capacity gap |
| --- | --- | --- | --- |
| Entire constructed panel | 60 | 28.28% | 35.72% |
| Constructed above-one subset | 41 | 29.67% | 32.73% |
| random_5x5_s0_92 | 1 | exactly0 | exactly0 |

Every generic rational factor selects one exposed-face closure. On the
above-one subset,38/41 selector ratio bounds exceed the HKO value. The
nonregular rational `contact_full_s0_e3of1000` has imported exact true ratio
1.042631725956..., selector bound1.791607981034..., and31.09% capacity gap.
Independent verification proves its selector bound itself exceeds HKO using
exact rational comparison, without relying on its true capacity label.

The three separately fixed broad controls have capacity gaps10.65%,12.74%,
7.44%, respectively, for random_5x5_s1_64, random_5x6_s2_150 and
random_6x6_s3_124. These truth comparisons retain their historical numerical
status; they are not new capacity certificates. The exact counterexample
random_5x5_s0_92 is recovered exactly (ratio0.838532815386...), so the new
rule avoids that particular global failure but fails much more severely near
HKO. Numerical regular controls at0,pi/20,pi/10 retain5×5 tied closure pairs
and match the imported regular capacity formula to rounding.

## Verification and reproduction

`limit.py` uses exact forward derivatives over Q(sqrt5), checks all strict
base geometry/support facts, all five objective derivatives in each factor,
their strict order, nonregularity contrasts and all120 limiting selected words.
`limit-certificate.json` retains the complete inputs and ordered-field gaps.
`verify_minkowski.py` imports no discovery functions, reconstructs rational
vertices/areas/closures, proves every contact-face LP by exact primal/dual
equality and strict complementary slacks, and checks all7,680 selected cyclic
words across64 bodies. It reports PASS. The true HKO value and the retained
panel capacities remain explicit imported mathematical results.

```
python3 minkowski.py --repo REPO --source VERIFIED_RANDOM_PRODUCT --out PACKET
python3 verify_minkowski.py --repo REPO --source VERIFIED_RANDOM_PRODUCT --directory PACKET
python3 limit.py --repo REPO --directory PACKET
```

Keep the adjacent plans in PACKET before execution. Source paths/hashes and
receipt files retain the exact input identities. Only Python's standard library
and the repository's exact Q(sqrt5) helper are used. No all-body capacity
enumeration, broad3,072-row scan, fitting, production edit, thesis edit,
installation, network, cloud or public action occurred. New files began in
the assigned scratch directory; root owns independent scientific review,
retention, integration and commit.

[review: /root/closure_review]
