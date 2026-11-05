# Two-parameter HKO candidate

29 October 2026. **Candidate, not certified capacity or gain.**

Use the [parent account's](../README.md) old rows N0 and put

    T(q,p)=(-J2 p,J2 q),
    K(a,b)={x:N0 x<=1; (a N0+b N0 T)x<=1}.

The old family is a=1−3t/5, b=t cos(pi/5). Allowing a and b to vary independently
leads to the simple exact candidate

    a=99/125, b=(7/20)cos(pi/5).

A floating word-bank search found no witness above
Q≈0.1463869294740322 among its numerically feasible KKT solutions, giving an
estimated ratio upper bound1.05276175981 with numerical volume5.54083616531.
The strongest reported word is (0,6,5,2,19,11,8,4). An independent exact
Q(tau) calculation verifies its positive weights, closure, normalization and

    Q=(5328025872481 tau−3162037824 tau³)/26435588952481.

That exact feasible word gives a capacity upper bound. A larger unexamined Q
would lower the capacity, possibly eliminating the apparent gain. Numerical
volume alone is not an exact ratio bound. Full graph, curvature, leaf/singular
feasibility and volume checks remain necessary.

## Method and replay

[probe.py](probe.py) uses the frozen t=.3 packet's2326 leaf words and both
retained seven/eight-label branches as a **proposal bank**, never as a complete
new-parameter catalogue. It solves numerical KKT systems, accepts nonnegative
solutions with small residuals, enumerates four-row vertex intersections and
computes a Qhull volume. [controls-and-candidate.json](controls-and-candidate.json)
retains input, witness weights, residuals and the two original-family controls.
The t=.3 control agrees with the certified ratio within1e−8.

From the checkout root, use a fresh output path:

```sh
OPENBLAS_NUM_THREADS=1 uv run topics/variation-and-local-maxima/hko-cut-family/two-parameter-continuation-20261029/probe.py \
  --out /tmp/msc-math/viterbo-continuation-20261029/two-parameter/NEW.json
```

Add `--search` for the bounded reproducible 2D differential-evolution probe
(seed324, bounds a∈[.72,.85], b∈[.18,.33], popsize8, maxiter25). Its optimum is
an optimum of an incompletely sampled feasible-word upper bound, not of actual
capacity. The original exploratory optimization using only two branches gave
very loose estimates; the wider bank rejected its apparent large gains.

The exact witness and source audit is retained in AUDIT.md when complete.
Next discriminator: reuse the new-point complete-capacity machinery on this
body, with freshly derived geometry, pruning, singular checks and inclusion
constant. The old1+2t/5 bound must be replaced; the proposed support bound here
is a+b/cos(pi/5)=571/500 and still needs the certificate's exact support check.
