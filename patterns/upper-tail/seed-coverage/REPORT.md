# LP seeds omit a strict fixed-word basin on an above-one product

17 October 2026. Owner: `/root/upper_tail_analysis`; receiving owner: standing
thesis owner `/root`. This bounded analysis reuses the existing twelve exact
near-HKO controls. It finds a new obstruction to proving the completed LP-seed
extractor exact by claiming that its seeds cover **every strict coordinate basin
for each angular word**: one such basin is missed on the first above-one control.
This is a method-selection result, not a failure of the all-seed capacity
prediction, which remains exact on all twelve retained controls.

A separate, capacity-label-free sufficient condition certifies completion when
first responses cover all vertices of either factor's closure polytope **in each
word separately**. It applies to 8 of the 24 retained words and to both words on
only 2 of the 12 bodies. These are certificate applicability counts, not new
accuracy estimates or population rates.

## End state and choice of crux

The desired end state was either a structural explanation of LP seed coverage
that can support an exactness theorem, or an exact obstruction identifying the
missing hypothesis. The backward plan was: inspect the existing selector and
its local theorem (to avoid recounting known successes/traps); formulate a
label-free coverage condition; reconstruct seed responses and fixed-word basin
membership exactly; stop on the first substantive obstruction. These steps
cannot be skipped because successful selected predictions do not establish
coverage of losing basins, and a union across words can hide word dependence.

| Candidate crux | Useful outcome | Misleading rubric | Decision |
| --- | --- | --- | --- |
| Recount nonglobal seed endpoints | Diagnose failure of coordinate ascent | More traps looks like more insight although already known | Reject: existing 185/240 and 159 counts settle it |
| Prove a whole HKO neighborhood theorem | Turn finite recovery into local exactness | Agreement at a symmetric, tied point hides support chambers | Too large for this bounded pass |
| Audit first-response coverage and unseeded strict basins | A simple sufficient condition or an exact missing hypothesis | Selected accuracy alone hides omitted losing basins | Select; no new geometry or optimizer |

A rubric combining structural implication, falsifiability on retained exact
certificates, and bounded cost favors the last route. A rubric rewarding only
extra exact examples would favor another cohort without addressing why the
method succeeds. No numerical polishing is counted as a result.

## A sufficient condition that does not use the capacity label

For a fixed shadow word, write the bilinear objective as
$Q_w(\alpha,\gamma)$ on closure polytopes $C_A\times C_B$. Its maximum is attained
at a vertex pair. Let $R_A(w)$ be the set of first A responses to all nonzero
row conditionals of the retained LP coupling. If $R_A(w)$ contains **every** A
closure vertex, the following B response attains

$$\max_{\alpha\in\operatorname{vert}(C_A)}\max_{\gamma\in C_B}Q_w(\alpha,\gamma)
=\max_{C_A\times C_B}Q_w.$$

The symmetric column/B condition is equally sufficient. This uses no assumed LP
sharpness: any feasible conditional coupling supplying these seeds would do.
Later strict ascent cannot spoil a global optimum. Applying the condition to
both words, together with the imported sparse-product and two-shadow theorems,
certifies exact capacity. Taking a union across the two words does not establish this wordwise sufficient-condition proof: a support pair may have different objective values in the two words, so those values still need separate control. No union-coverage exactness theorem is settled here.

The exact audit checks this condition on all 24 existing word/coupling pairs.
Only `lpseed_near_hko_s2_e100` and `lpseed_near_hko_s2_e20` satisfy it for both
words. Thus it is a valid sufficient condition but presently too strong to
explain ten of the twelve successes. Failure of the condition does not imply a
prediction error.

## An exact unseeded basin

On `lpseed_near_hko_s0_e100`, whose exact ratio is
$1.0257032309495246\ldots>1$, use the **negative** angular word. The pair

$$\operatorname{supp}\alpha=\{0,2,4\},\qquad
\operatorname{supp}\gamma=\{0,1,3\}$$

is a strict coordinatewise maximum with

$$Q_{\rm omitted}=\frac{4984202877199308739581412479}{49857662641957956148810840000}.$$

Its exact strict margins over the four alternative vertices of A and B are
positive; their display values are respectively $0.00039854279885\ldots$ and
$0.00072306342843\ldots$. Its objective remains below the true global objective
by $0.00085640145897\ldots$. The receipt retains the exact rational margins,
normal coordinates through source bindings, closure weights, word, and all ten
negative-word seed paths. None of those paths visits the pair, including after
any accepted strict update. In particular, the first A responses omit precisely
$\{0,2,4\}$, while the first B responses omit $\{0,1,3\}$ and $\{1,3,4\}$.

This refutes the explanation “LP conditionals seed all strict coordinate basins
for each word.” It does **not** refute global recovery by the all-seed rule:
the omitted pair is losing, and a different seed reaches the global winner.
The same word-specific omission also occurs on the already retained paired
scale-$1/20$ control.

The same support pair **is reached in the positive word**, where its value is
only $0.09950068849527\ldots$, strictly below its negative-word value
$0.09996864299460\ldots$. Thus this is not an example of an unseeded support pair
across both words. It exhibits why fixed-word basin coverage and cross-word
support coverage must be distinguished. Cross-word rescoring would expose this
particular missing negative-word candidate, but no general exactness theorem
for that augmented rule is established.

## Consequence and concrete construction idea

A prospective obstruction to an all-input exactness theorem is now specific:
continue this omitted strict negative-word basin while keeping the LP basis and
all seed response paths in their chambers, and ask whether it becomes globally
winning **before** a seed enters its basin. A strict crossing would produce an
actual extractor counterexample; a proof that such a crossing is impossible
would supply a substantive missing compatibility principle. Neither outcome
has been established here. No continuation sweep was launched.

For a safe certificate fallback, one can explicitly respond to each still
missing vertex of the cheaper factor in each word. This fills the stated
coverage condition, but in the worst case it is the ordinary one-factor
vertex enumeration with an opposite best response; it is not a new scalar
invariant or an asymptotic capacity shortcut. Selecting that fallback as a
production method is outside this diagnostic.

The scientific stop is the exact unseeded strict basin, rather than a slightly
improved numerical bound. The full-HKO-neighborhood theorem and global
maximality remain open.

## Sources, verification, and resources

The inputs are the unchanged [LP-seeded predictions](../lp-seeded-response/predictions.json)
and [exact oracle tables](../lp-seeded-response/results.json), together with the
independent [rational geometry/HK/LP checker](../near-hko-transfer/verify.py).
[Sources](sources.json) binds these and the interpreted report/proof dependencies
by SHA256. The imported [six-facet reduction](../../../topics/products/product-qp-six-facet-reduction.tex)
and [two-shadow theorem](../mixed-area-order/REPORT.md) supply completeness of
the truth tables; the finite checker does not prove those reductions. The
[local-completion packet](../local-completion/REPORT.md) supplies the prior local
result and its whole-HKO-neighborhood limit.

[check.py](check.py) imports no producer, optimizer, or numerical library. It
rechecks 24 rational LP primal/dual certificates, 600 existing direct
four-dimensional HK pair values, all 240 deterministic seed paths and their
first responses, exact terminal basin membership, and strict witness margins.
[verification.json](verification.json) is its PASS receipt. From the patterns
checkout, write a new receipt to scratch:

```sh
python3 -B patterns/upper-tail/seed-coverage/check.py \
  --repo "$PWD" --directory patterns/upper-tail/seed-coverage \
  --out /tmp/msc-math/seed-coverage/replay-verification.json
```

All new files began under `/tmp/msc-math/upper-tail-existing/20261017T1933Z/`
and were deliberately retained only after the new structural obstruction was
verified. The exact replay consumes about four CPU seconds per run; actual
receipt timing is recorded. The assigned ceiling was fifteen minutes of worker
work and thirty CPU seconds of small retained-certificate calculations. No new
cohort, LP solve, model fit, network, installation, cloud action, manuscript
edit, commit, push, or merge was performed by this worker. Root owns independent
verification, review, integration, commits, and cleanup.

Inspected sources were the checkout README, research guide, empirical topic
account, collaboration rationale, upper-tail route triage and focused
LP-seeded/local-completion/conditional-rounding reports and checker code. This
is focused local source access, not an exhaustive repository or external
literature search, a global theorem, or human mathematical acceptance.

[review: /root/intake_review]
