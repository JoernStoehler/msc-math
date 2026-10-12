# P* fixed-cut strata and the cuts-extremizer consequence

Lead derivation 2026-10-04 23:15 UTC. Status: exact predicates executed;
consequential independent mathematical review requested. Base P* F10 theorem
is inherited. No full F11 Hausdorff-neighborhood theorem is asserted.

## Statement and domains

For the rational P* of capacity1 and volume1/2:

1. For every fixed normal exposing a face of codimension at least2, every
   sufficiently shallow cut strictly decreases its systolic ratio.
2. Facet-parallel shallow cuts have ratio at most1 by the inherited F10 theorem.
3. Every affine hyperplane cutting the interior of P* has both cut pieces of
   capacity strictly less than1: P* is a **cuts extremizer**, in the terminology
   of [Haim-Kislev, Definition1.2](https://arxiv.org/html/2511.16644v1).

The depth threshold in1 depends on the direction. These statements do not imply
one ratio neighborhood for coupled directions, old-normal motion, or drift
along the equality family. The cuts-extremizer property concerns capacity,
not the ratio of pieces at arbitrary cut depth.

## Closed-cone capacity response

The24 independently retained vertex-cone witnesses remain feasible on each
closed coefficient simplex t_i>=0,sum t_i=1: zero-base-weight coordinates have
nonnegative linear coefficients, and C(t) has nonnegative quadratic coefficients.
For a fixed direction with at least two positive incident coefficients the
new facet and all old facets are genuine when the cut is sufficiently shallow.
An interior point of any normal face belongs to the normal cone of some vertex,
so these boundary restrictions cover all such fixed directions.

If C(t)>0, choose delta=epsilon/(2C(t)) to obtain

    c(Pcut)<=1−epsilon²/(2C(t))<1.

If C(t)=0, choose any sufficiently small fixed delta>0 for which the old positive
weights remain nonnegative. The same exact witness gives Q=1/2 and

    c(Pcut)<=(1−epsilon delta)²<1

for sufficiently small epsilon. Consequently every nonfacet direction strictly
decreases capacity, even where the interior C>0 argument degenerates. This
capacity conclusion does not require any volume comparison.

## Codimension3 edges

A fixed codimension3 cut removes volume O(epsilon³). If C>0 the capacity upper
bound loses order epsilon²; if C=0 it loses order epsilon. Either gives strict
ratio loss for sufficiently shallow fixed cuts. The original vertex m=4 theorem
supplies the corresponding codimension4 statement.

## Pure-factor codimension2 ridges

A pure-factor vertex cut preserves Lagrangian product structure: either
Hcut×T or H×Tcut. Its varying factor gains one genuine facet, with fixed normals
and only the new support1−epsilon varying. The complete finite product circuit
envelope in the sibling geometry packet has branches

    A=2 Hq Hp/kappa.

One factor is fixed, so every branch is an AFFINE function of epsilon. Although
at epsilon0 the new normal is redundant, the finite formula holds for all small
positive epsilon and extends to0 by capacity continuity. Hence the true capacity
is a finite lower envelope of affine functions. Since the closed-cone witnesses
above prove c(epsilon)<1 for every sufficiently small positive epsilon, an active
line at0 must have strictly negative slope. Thus

    c(epsilon)=1−m epsilon,  m>0,

for all sufficiently small epsilon. Removed volume is O(epsilon²), so the ratio
strictly decreases. This avoids falsely requiring C=0 for the particular
retained vertex witness. Some retained witnesses have positive pure-factor
quadratic terms; the finite product envelope supplies a better branch.

## Mixed q/p codimension2 ridges: new exact adjacency construction

Let a be a q facet normal and b a p facet normal. Among12 exact positive primitive
active words there is, for every one of the24 such facet pairs, a consecutive
occurrence of a,b or b,a with positive omega=omega(a,b)>0 in that order.
The [standalone exact verifier](cut-strata/verify.py) rebuilds those words from
feasible base weights, verifies closure and Q=1/2, and checks all24 adjacencies.
It does not need those12 words to be a complete catalogue.

Write the ordered adjacent normals as a,b and let w=t a+(1−t)b,0<t<1. Insert
the new label between them, replace their weights by beta_a−t delta and
beta_b−(1−t)delta, and give the new label weightdelta. Normal closure and total
weight are unchanged. Contributions from labels outside the adjacent block
cancel. The local quadratic calculation gives exactly

    Q(delta)=1/2−omega t(1−t)delta².

Both old weights are positive, so the surgery is feasible for small delta>0.
This is a cleaner branch than some original vertex witnesses, whose restriction
had extra pure-factor terms. Taking delta=epsilon/[2omega t(1−t)] yields

    c(Pcut)<=1−epsilon²/[2omega t(1−t)].

Let Rq=edge_length_q/|a_q| and Rp=edge_length_p/|b_p|. These are the cross-section
area densities in the slack variables sq=1−a.q,sp=1−b.p. Near the exposed
edges, each density is affine in slack. The removed cap satisfies

    t sq+(1−t)sp<epsilon,

so integration gives

    DeltaV=Rq Rp epsilon²/[2t(1−t)] + O(epsilon³).

Thus its ratio upper bound has expansion

    rho(Pcut)<=1 + [Rq Rp−1/omega] epsilon²/[t(1−t)]
                    + O(epsilon³).

At P* the exact densities are

    Rq=(5/12,7/24,7/24,5/12,7/24,7/24),
    Rp=(3/8,1/8,1/8,3/8).

The new24-pair verifier checks Rq Rp omega<=5/8<1 for every pair. The leading
coefficient is therefore strictly negative. Exact selected words, weights,
normal pairings and margin products are retained in
[cut-strata/ridge-margins.json](cut-strata/ridge-margins.json).

The first exploratory probe evaluated C at t_q=t_p=1 and concealed pure-factor
terms for two pairs. Independent review caught this. That estimate is discarded;
the accepted candidate is the newly constructed adjacent-word surgery, with
its full symbolic identities. No theorem is inferred from the failed probe.

## Facet cuts and all hyperplane cuts

A normal in the relative interior of a facet normal ray produces a parallel
support-number change. The old parallel inequality becomes redundant; the
result has ten genuine facets and is close to P*. The inherited F10 theorem
therefore gives rho<=1. Its volume is strictly smaller than1/2, hence
c²<=2V<1. This proves strict capacity reduction for the remaining directions.

Fix any hyperplane intersecting int P*. For either retained side, choose a
shallower parallel cut on that side to which the corresponding capacity-strict
argument applies. The actual cut piece is contained in the shallower piece.
Capacity monotonicity gives its capacity strictly below1. Repeating for the
opposite side proves the cuts-extremizer property for every interior hyperplane.
No uniform cut-depth radius is needed for this quantifier, and no volume or ratio
claim at finite cut depth is deduced.

## Remaining universal bridge and concrete next test

The unanswered F11 local-maximality quantifier permits the normal direction and
epsilon to vary together, with old normals moving along or transverse to the
two-parameter equality family. In transitions epsilon may be comparable to a
small cone coefficient, so cap combinatorics and volume order change. Neither
the fixed-ray strictness nor cuts extremizer implies the required uniform ratio
control. The pure-factor linear response and mixed-ridge margin are promising
endpoint inputs, but they must be assembled with their feasible ranges.

A precise adverse result would be an improving coupled sequence approaching a
facet split, or an exact failure of the selected cancellation-response bank.
Without such a witness, the remaining bridge is unresolved rather than refuted.
