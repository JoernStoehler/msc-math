# Fresh above-one obstruction to conditional support extraction

The unchanged joint-closure selector is **not exact on every product near HKO**.
A predeclared rational 5×5 product has exact systolic ratio
**1.02888100196 > 1**, but its selected capacity bound is **0.0859128% too high**.
The small error identifies a structural failure rather than a material bound gain:
the procedure purifies one factor's support and leaves the other as a mixture.
Exact strict complementarity shows that this failure persists on an open
neighborhood of the example, with no radius or minimum practical error claimed.

The [scientific packet](../../../../../patterns-2026-10-17/patterns/upper-tail/near-hko-transfer/REPORT.md)
contains all twelve frozen new geometries, all twelve frozen predictions,
ten exact capacity scores, the stopping decision and its proof. The first nine
scored products are exact recoveries. The tenth is the obstruction; its discovery
stops the predeclared test and leaves two members unscored. All ten scored
ratios exceed one. This is a new constructed local challenge, not a random
population sample or a success-rate estimate. The earlier 35 exposed controls
and their exact certificates remain valid.

## Geometric insight

The unique globally optimal closure pair uses A facets `{0,2,3}` and B facets
`{1,2,4}` (zero-based). Neither winning closure occurs as a normalized conditional
in either LP optimizer. Nevertheless, an opposite response extracts the true A
closure. Its remaining B closure is a strict mixture of the true B closure and
the losing closure on `{1,3,4}`. With true-closure weight $0<\lambda<1$,

$$Q_* - Q_{\rm round}=(1-\lambda)(Q_* - Q_{\rm bad})>0.$$

This explains the missed capacity without attributing it to word ordering,
numerical solver error or a choice between alternative optimal LP solutions.
Both LP optima are unique: each has 17 positive independent basic entries and
eight strictly positive nonbasic reduced costs. Cyclic angular order is locally
fixed; these bases and all positive conditional masses persist. Closure vertices,
the finite best-response maxima and true capacity vary continuously. The strict
objective deficit and the strict above-one ratio therefore persist locally.
This proves a nonempty open family of failures of the frozen mathematical rule.
It supplies no specified neighborhood of HKO or claim of arbitrarily close failures.

The existing exact oracle also proves that choosing B's best response to this
already extracted A closes this particular deficit. That is an explanation and
a construction idea, not a validated replacement selector or a universal repair.
The [optional reader passage](CHAPTER8-INSIGHT.md) develops the same distinction.

## Independent checks and reproduction

[owner-verify.py](owner-verify.py) uses standard-library exact arithmetic, no
scientific producer or optimizer. It reconstructs the frozen generator and input
hashes, both factor polygons, all Gaussian closure vertices, 20 LP primal/dual
certificates and 200 conditional responses. Its capacity oracle independently
maximizes B insertions over the two cyclic orders of each sparse A closure,
checking **250 closure pairs**. This uses the imported sparse reduction and
support-prefix identity, without importing the worker's angular-word oracle.
[verification.json](verification.json) binds the inputs and reports PASS.

[owner-stability.py](owner-stability.py) separately checks the obstruction's
unique winning pair, missing conditionals, exact mixture identity, correct B
response, constraint/active-column ranks, strict slacks and angular cross signs.
[stability.json](stability.json) records PASS. Arithmetic checks support the local
continuity argument above; neither replaces its implication proof.

From this thesis checkout with the separate patterns checkout present:

```sh
python3 -B thesis/review/2026-10-17/near-hko-transfer-integration/owner-verify.py \
  --directory ../patterns-2026-10-17/patterns/upper-tail/near-hko-transfer \
  --repo ../patterns-2026-10-17 --out /tmp/msc-math/near-hko-owner-replay.json
python3 -B thesis/review/2026-10-17/near-hko-transfer-integration/owner-stability.py \
  --directory ../patterns-2026-10-17/patterns/upper-tail/near-hko-transfer \
  --out /tmp/msc-math/near-hko-stability-replay.json
```

The explicit premortem selected this bounded B-priority investigation. Alternatives
and resource limits were frozen before scoring in the scientific PLAN. All files
began under `/tmp/msc-math/near-hko-transfer/20261017T1719Z/`; retained evidence is
split deliberately between the scientific patterns branch and independent thesis
assessment. Source access covered the named product, two-shadow and conditional
packets, without an exhaustive literature survey. No fresh cohort, tuning or
manuscript insertion follows. The 105-page candidate and Kai packet are unchanged;
root owns separate local commits and cleanup.

[review: /root/intake_review]
