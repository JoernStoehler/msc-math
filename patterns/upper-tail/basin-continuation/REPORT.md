# The existing s0 radial continuation cannot promote the omitted basin

17 October 2026. On the entire fixed interval $1/100\le t\le1/20$ of the
existing seed-zero near-HKO perturbation, the omitted negative-word basin
$A\{0,2,4\}/B\{0,1,3\}$ remains a strict coordinate maximum but is strictly
dominated by the feasible pair $A\{0,2,3\}/B\{1,2,4\}$. The domination holds
against **both** angular words of the target. By the imported two-shadow
formula, no reordering of that target pair can make it globally winning on
this interval. Thus this particular continuation cannot supply an all-seed
recovery counterexample, regardless of whether or when LP seeds capture it.

This is a route obstruction, not an all-direction impossibility theorem,
a proof of the LP-seed extractor's exactness, or a ratio predictor. The
starting control has systolic ratio $1.0257032309\ldots>1$; the endpoint at
$t=1/20$ has ratio $0.9501617960\ldots$. The full interval is not asserted
to stay above one. No new sampled cohort or numerical optimizer was selected.

## Route and why this settles the selected crossing question

[PLAN.md](PLAN.md) records the concrete end state, alternatives, rubric
criticisms, fixed parameter and receiving owner selected at the native checkpoint
before execution. Let $h$ be
the existing rational HKO presentation with $\sqrt5$ replaced by $2.236068$,
and $d$ the existing seed-zero digest direction. We continue $x(t)=h+td$,
with the same twenty coordinates and digest namespace as
[the frozen geometry producer](../lp-seeded-response/freeze.py).
[check.py](check.py) independently reconstructs the entire line from the two
existing endpoints and checks its equality with $h+td$. In particular, $t=0$
is neither included nor identified with the exact algebraic HKO control.

The earlier [seed audit](../seed-coverage/REPORT.md) found that no negative-word
seed visits this strict basin at $t=1/100$; it also records the same omission
on the paired scale-$1/20$ control. Merely following that omission at more
sample points would not determine whether its payoff can overtake a winner.
Instead the present certificate rules out that payoff crossing everywhere
on the fixed interval. It does not need to prove LP-basis or seed-path
persistence inside the interval, and makes no such interior claim.

Let $T(t)$ denote the target closure-vertex pair, $C(t)$ the competitor above,
and $w_+(t),w_-(t)$ the two angular words, including the negative word's reversal.
The certified strict inequalities are

$$Q_{w_-}(C(t))>Q_{w_-}(T(t)),\qquad
  Q_{w_-}(C(t))>Q_{w_+}(T(t))$$

for every $t$ in the closed interval. Both pairs remain feasible throughout.
The [two-shadow theorem](../mixed-area-order/REPORT.md) identifies the best
word on a pair of support-three closures with the maximum of those two
angular scores. Hence $\max_wQ_w(T(t))<Q_{w_-}(C(t))$, which excludes the
target from the global HK objective maximum determining that product’s capacity,
independently of the extractor.

The eight other strict inequalities compare $Q_{w_-}(T(t))$ against each of
the four alternative A closure vertices with target B fixed, and each of
the four alternative B vertices with target A fixed. The target therefore
stays a strict coordinate basin on this route: it is not excluded merely
by ceasing to be a basin or meeting an angular boundary.

## Exact interval certificate

Each normal coordinate is affine in $t$. For any support triple $(i,j,k)$,
its normalized closure weights are

$$\alpha_i=\frac{\det(a_j,a_k)}{D_A},\quad
\alpha_j=\frac{\det(a_k,a_i)}{D_A},\quad
\alpha_k=\frac{\det(a_i,a_j)}{D_A},\quad
D_A=\det(a_j,a_k)+\det(a_k,a_i)+\det(a_i,a_j),$$

and likewise for B. The checker chooses the denominator's positive sign and
retains every polynomial coefficient. Substituting these formulas into the
actual HK objective $Q_w=\frac14\sum_{ij}\varepsilon_{ij}\alpha_i\gamma_j
(a_i\cdot b_j)$ gives rational functions. Clearing their certified positive
denominators turns all comparisons into polynomial signs.

For a polynomial $P(t)$ on $[l,u]$, write
$P(l+(u-l)s)=\sum_{k=0}^nc_ks^k$. Its degree-$n$ Bernstein coefficients are
$b_i=\sum_{k=0}^ic_k\binom{i}{k}/\binom{n}{k}$. Since the Bernstein basis
is nonnegative and sums to one, $\min_i b_i>0$ proves $P(t)>0$ on the
whole closed interval. Every sign here passes on the whole interval without
subdivision. This is an exact rational sign proof, not a dense sample.

[verification.json](verification.json) retains 212 positive-polynomial
certificates: the active closure weights and denominators, signs excluding
all infeasible triples and every support-two boundary, affine-triple
nondegeneracy, both angular orders and cuts, the two domination comparisons,
and the eight strict coordinate-response comparisons. These signs preserve
exactly the five feasible support triples in each factor and the genuine
polygon/word chamber of the already verified endpoint. The checker also
independently verifies polygon geometry at both endpoints and replays all
four retained exact LP primal/dual certificates there. LP continuation is
not part of the proof.

The direct four-dimensional HK replay at both endpoints agrees exactly with
the polynomial formulas for the target's two words and competitor's negative
word. The negative-word competitor-minus-target differences have display
values $0.000856401458966\ldots$ at $t=1/100$ and
$0.003998346663293\ldots$ at $t=1/20$. The material result is exclusion
of a crossing on the entire interval, not a small improvement of a numerical
bound. Both exact gaps, the cross-word gaps, all rational polynomial
coefficients and all Bernstein coefficients are retained in the receipt.

## Verification and scope

From the patterns checkout, write a fresh receipt to scratch:

```sh
python3 -B patterns/upper-tail/basin-continuation/check.py \
  --repo "$PWD" \
  --out /tmp/msc-math/basin-continuation/replay-verification.json
```

The checker uses Python's standard library and imports only the existing
independent rational geometry/HK/LP verifier; it imports no optimizer or
producer. SHA256 bindings identify the selected source geometry, the seed
obstruction, the imported proof dependencies and the checker. All new files
began under `/tmp/msc-math/basin-continuation/20261017T1946Z/` before deliberate
retention. Total new symbolic/certificate computation is below one CPU second,
against the assigned 120-second ceiling; no numerical solve was run.

Inspected sources were the checkout README/AGENTS and collaboration rationale,
the research guide and products topic, the LP-seeded report/producer/freeze,
the seed-coverage report/checker, the local-completion report, the two-shadow
proof, and the independent rational replay. The imported two-shadow theorem
and six-facet reduction own the passage from finite word/pair calculations to
capacity; this checker does not prove those general reductions. No external
literature search or exhaustive novelty claim is made. Amplifying this fixed $d_0$ direction within the selected interval cannot
realize the proposed counterexample; a direction change or a route outside the
interval is needed. The target may still be promoted on such a different route.
Root owns independent scientific review, integration, commits and cleanup.

[review: /root/continuation_review]
