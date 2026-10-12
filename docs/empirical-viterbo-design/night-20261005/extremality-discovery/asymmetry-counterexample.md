# A scalar asymmetry–capacity-retention tradeoff is too strong

Derived and exactly checked 2026-10-04. Owner: extremality_discovery_lead.
This is an adverse branch of a possible global fixed-pentagon inequality route,
not evidence against HKO globality.

Fix the circumradius-one regular pentagon Q, with normals at angles 2pi i/5.
Write A2=W(K) for its two-bounce/difference-body ceiling and

    1+delta(K)=area(K−K)/(4 area K),   r(K)=c(K×Q)/A2.

The known symmetric-partner sharp bound has constant
k_sym=(10+6sqrt(5))/25. At the HKO orientation, r=1 and 1+delta=sqrt(5)/2,
with k_sym sqrt(5)/2=(3+sqrt(5))/5. This suggests the stronger scalar target

    (1+delta) r² ≤ sqrt(5)/2.                             (FALSE)

It would imply the desired fixed-Q global bound, but it is false. An exact
rational quadrilateral witness is

    K=conv{(-3/5,-1/2),(3/5,-4/5),(0,1),(-3/5,-3/10)}.

The retained ordered-algebraic certificate verifies

    area K=21/20, area(K−K)=303/50, 1+delta=101/70;
    every one of the ten unit triangles fits after scaling by 9/5;
    A2≤203/100.

Hence c≥9/5 and

    (1+delta) r² ≥ 327240/288463 > sqrt(5)/2.

The A2 upper bound is explicit: v=(-9/10,-3/5) lies on a supporting edge of
K−K, and h_(Q−Q)(v)≤203/100. Since A2 is the minimum of this gauge on the
difference-body boundary, the bound follows. The ten feasible translations and
all exact signs are retained in [asymmetry-counterexample.json](asymmetry-counterexample.json).

Reproduce from the checkout root:

    uv run --with sympy python docs/empirical-viterbo-design/night-20261005/extremality-discovery/verify_asymmetry_counterexample.py

The verifier reuses only the existing exact ordered-field arithmetic/hull/linear
solve helper; it does not import the floating fitting oracle or rerun retained
producers. The elementary rational area calculation is independent of capacity.
The interpretation c≥9/5 uses [normal-triangle covering, Theorem3.3]
(https://arxiv.org/html/2603.12495v2); the A2 definition is owned by the existing
symmetric-partner note.

## Why it does not refute the intended inequality

The symmetric reference body's area excess cannot be discarded. With
S=(K−K)/2 and the minimal symmetric reference B at capacity1, define

    sigma=area(S)/(A2² area B) ≥1.

Then exactly

    sys(K×Q)=k_sym (1+delta) r² / sigma.

The desired fixed-Q HKO bound is therefore the three-factor inequality
(1+delta)r²/sigma≤sqrt(5)/2. The failed scalar conjecture omitted sigma.
Restoring it is an exact reformulation of the global area-capacity target,
not a new proof method. This witness tells an inequality group what cannot be
eliminated; it does not establish a universal compensating estimate.
