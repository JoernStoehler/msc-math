# Conditional coupling supplies exact upper-tail certificates

17 October 2026. The explicit premortem selected a bounded test of Jörn's B
priority: a geometric explanation that also works in the upper tail. The
[scientific report](../../../../../patterns-2026-10-17/patterns/upper-tail/conditional-validation/REPORT.md)
and frozen PLAN own the method comparison and exact rational certificates.
The unchanged conditional-LP rounding rule was tested on 35 controls from the
previous containment validation panel, excluding all three conditional design
geometries. Exactly one exclusion overlapped: 26 principal tails and nine
hash-selected bulk controls remain. All data had been inspected before; this
is retrospective design-excluded validation.

The standalone rule agrees numerically with all 35 historical capacities.
More substantively, matching exact LP lower bounds and feasible upper witnesses
certify 26 capacities, including 17 of the 26 tails. On 24 bodies, an earlier
feasible upper witness already matches the newly computed lower certificate;
two require the new rounded witness. Numerical agreement on the remaining
nine bodies is not exactness or a true-gap theorem.

On principal tail `random_5x5_s3_227`, the strongest earlier feasible upper
bound was 5.778379336461988…; the new exact capacity is 5.691937093646148… .
The **1.49596% bound cut** removes the entire previous 1.51868% miss against
the historical numerical label. The lower certificate was computed in this
batch: the old numerical comparison was not a previously certified interval.
On `random_6x6_s2_63`, a 0.05412% cut also matches the new LP lower bound;
this small cut is not the scientific success criterion. There are no
regressions against the strongest comparator, which includes full-partner
containment and its earlier incumbents.

The mechanism is joint support selection: each nonzero row or column of an
optimal conditional-closure coupling normalizes to a feasible closure; one
opposite-factor response gives a genuine HK witness. The LP gives the other
capacity direction. This uses optimization and is more involved than a simple
scalar invariant. Exact HKO rounding remains a convention control with an
existing proof; its known nonseparable LP gap remains. All selected ratio
upper bounds were already below one, so there are no new envelope exclusions
or evidence for a global HKO theorem. No fresh-sample or universal prediction
guarantee follows.

## Independent verification

[owner-verify.py](owner-verify.py) imports the earlier thesis-side independent
LP and Cramer's-rule closure checkers, and no scientific producer or solver.
It joins all 35 source geometries, independently reconstructs polygon areas,
checks frozen membership and input hashes, reconstructs both angular words
and all 70 rational primal/dual LP systems, and checks all 470 conditional
responses directly in four dimensions. Twenty exact quadratic-field HKO
responses are checked separately. Metrics use the strongest earlier feasible
bound including containment; exact counts and the material-tail selection
match the scientific results. [verification.json](verification.json) records
PASS and binds the inputs and checker bytes. Separate scientific review is
retained on the patterns branch.

From this thesis checkout, with the separate patterns checkout present:

```sh
python3 -B thesis/review/2026-10-17/conditional-validation-integration/owner-verify.py \
  --thesis "$PWD" --patterns ../patterns-2026-10-17 \
  --results ../patterns-2026-10-17/patterns/upper-tail/conditional-validation/results.json \
  --out /tmp/msc-math/conditional-validation-owner-replay.json
```

New files began in `/tmp/msc-math/upper-tail-next/20261017T1637Z/` and were
retained deliberately. Inspected sources were the named prior conditional,
containment and two-shadow packets, product reductions and project state;
this is not an exhaustive repository or outside-literature survey. The
35-body scoring run used about 5.4 wall seconds and 6.9 CPU seconds in the
cached environment. No new cohort, fitting or further refinement is selected.
The 105-page manuscript and its existing Kai packet remain unchanged.
Root owns the separate branch-local commits, integration and cleanup.

[review: /root/intake_review]
