# Conditional-coupling witness extraction and a strict relaxation gap

17 October 2026. The explicit owner premortem selected one bounded investigation
on the separate patterns branch. The frozen test reused three rational pilots
and three regular numerical controls; it solved no new LP and ran no population
or Cartesian support-pair capacity search. The selected manuscript is unchanged.

The [scientific report](../../../../../patterns-2026-10-17/patterns/upper-tail/conditional-rounding/REPORT.md)
and its pre-score PLAN own the mechanism, alternatives, proofs and controls.
The rule normalizes each nonzero row/column of a retained optimal coupling to
a closing combination of normals, then maximizes the actual fixed-word HK
objective over the opposite factor's closure polytope once. Every resulting
positive feasible objective supplies a capacity upper bound. There is no
universal approximation guarantee; the retained LP optimizer can affect the
conditionals.

The test settles two exact questions:

- On `random_6x6_s3_124`, the retained coupling already factors as the outer
  product of its closure marginals. Extracting this witness matches the LP dual
  and proves capacity exactly `4.474980102946307…`. The strongest previous
  feasible upper bound was `4.606321493224962…`: the 2.851329% cut closes the
  entire previous certificate interval. The numerical capacity value was
  already known; the new result is exact optimality certification, and a
  direct rank-one check suffices for this example.
- On `random_5x6_s1_198`, the dual-tight graph of the uniquely larger shadow LP
  cannot contain a feasible product of closures. Each of the five possible A
  closure supports leaves at most two common tight B columns, and none closes.
  Hence the true HK maximum is strictly below the LP maximum: its capacity
  lower bound is strictly below true capacity. The existing factor-1.010736
  certificate interval remains; no positive quantitative gap size is claimed.

At HKO, the retained nonseparable couplings instead yield exact rounded
objective `1/10` in the packet's transformed coordinates. The explicit
circulant optimum supplies all five closure vertices as conditionals, explaining
recovery. This does not remove the established LP relaxation gap. The already
certified 5×5 pilot is unchanged. Six deliberately selected, previously exposed
pilots establish no held-out upper-tail prediction or global ratio theorem.

## Independent verification

[independent.py](independent.py) imports none of the new producer/checker code.
It reconstructs each individual factor closure vertex by Cramer's rule, checks
all normalized conditional closures and opposite response maxima, and evaluates
every resulting HK sum directly in four dimensions. It checks 45 rational and
20 quadratic-field candidates, reconstitutes six rational LP dual inequalities
from geometry, checks rank-one factorizations and the strict-gap tight graph.
It reuses the previous packet's ordered quadratic-field arithmetic; exact HKO
capacity and the sparse/two-shadow theorem remain imported mathematical inputs.
[verification.json](verification.json) binds the source/results/checker bytes.
Scientific review is retained with the separate scientific packet.

Run from this thesis checkout after obtaining the separate patterns checkout:

```sh
python3 -B thesis/review/2026-10-17/conditional-rounding-integration/independent.py \
  --repo ../patterns-2026-10-17 \
  --result ../patterns-2026-10-17/patterns/upper-tail/conditional-rounding/results.json \
  --out /tmp/msc-math/conditional-rounding-independent.json
```

Outputs began under `/tmp/msc-math/conditional-rounding-integration/` and were
retained deliberately. Focused source access covered the named pattern packets,
product reductions and current project state; no literature novelty or exhaustive
repository-search claim is made. The scientific worker owns the separate
patterns commit; the standing owner owns verification, coordination and this
thesis-side integration commit. No new cohort, tuning, manuscript insertion,
public deployment, push, merge or university communication was selected.

[review: /root/delivery_review]
