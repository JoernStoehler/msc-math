# A joint coupling yields exact capacity extraction and a strict random-body gap

17 October 2026. The frozen conditional-coupling rounding test settles two previously unresolved certificate questions on its three retained rational pilots, without evaluating all closure-support pairs:

- `random_6x6_s3_124` has **exact capacity** $4.474980102946307\ldots$, closing the entire prior factor-$1.029350\ldots$ interval. Its retained LP optimum was already rank one; recognizing and extracting that feasible factorization supplies the missing exact upper witness. The new result is exact optimality certification, not a new numerical capacity value.
- `random_5x6_s1_198` has a **strict relaxation gap**: the retained conditional LP's lower capacity bound is strictly below true capacity. A dual-tight graph excludes every feasible rank-one optimum. Thus this remaining interval is not merely a failure to find the right witness at its lower endpoint.
- At HKO, the retained nonseparable couplings round to an exact true-capacity witness even though their LP values have the established relaxation gap. The explicit circulant optimum also has a structural explanation: its five conditional closures cover all five closure vertices.

This provides a joint signed geometric selector and two exact decisions, rather than a uniformly tight upper-tail predictor or global-maximality conjecture. The fixed small test stops here; no expanded cohort, tuning or manuscript change is selected.

## What was selected, and why

The concrete end state was a small target-free feasible capacity upper bound that recovers HKO and either closes a material retained interval or supplies a decisive obstruction. [PLAN.md](PLAN.md) records the necessary backward steps, alternatives and frozen controls before new scores. Outer affine-pentagon envelopes were already analytically excluded: their family capacity equals their width, which cannot be less than the original body's width under inclusion. Repeating them would produce no new constraint. Simultaneous second-best coordinate responses were rejected as tuning the stopped coordinate rule without a new support principle.

The selected route uses the previous conditional LP's **joint signed coupling** to select conditional closures, followed by one opposite-factor linear response. It differs from an iterative coordinate response seeded by width. A rubric rewarding only HKO agreement would reward width; one rewarding only an exact LP solution would conceal its gap; a bulk fit could conceal upper-tail misses. This test instead asks exact HKO recovery and compares with the strongest applicable retained feasible upper bounds. All pilots and labels were previously exposed; this is a retrospective method diagnostic, not held-out prediction.

The inputs are [the previous LP results](../conditional-coupling/results.json) and [exact HKO certificates](../conditional-coupling/hko-exact.json). No LP was solved again. The original LP packet imported the HK capacity formula, sparse closure reduction, two-shadow ordering theorem and established HKO capacity formula; those same source assumptions apply here.

## Conditional rounding and its guarantee

For normalized closures $\alpha\in C_A$, $\gamma\in C_B$, and either fixed shadow word $w$, write

$$Q_w(\alpha,\gamma)=\sum_{ij}c_{ij}(w)\alpha_i\gamma_j,\qquad c_{ij}=\tfrac14\varepsilon_{ij}a_i\cdot b_j.$$

The imported LP has $W\ge0$, $\sum W=1$, row conditional closure $\sum_jW_{ij}b_j=0$, and column conditional closure $\sum_iW_{ij}a_i=0$. Each nonzero row therefore normalizes to a B closure $\gamma^{(i)}$. Freeze that closure and maximize $Q_w$ over $C_A$. This linear objective attains a maximum at a closure vertex, with at most three active entries by the three equality constraints (mass and two coordinates). Symmetrically, every nonzero column supplies an A closure and one B response. Each resulting rank-one coupling is feasible, so its positive objective gives a **capacity upper bound** $1/(2Q_w)$.

The rule takes the largest rounded feasible objective over these candidates and both fixed words. Its standalone bound $C_{\rm round}$ is reported separately from $\min(C_{\rm round},C_{\rm incumbent})$. No repeated response, perturbation, support-pair Cartesian search, altered word, fitting or capacity-label selection occurs. There are 12, 21 and 12 conditional-response candidates on the three rational pilots.

There is **no universal sharpness or approximation-factor guarantee**. A conditional closure can lie inside its closure polytope; arbitrary LP optimum choices can alter the conditionals. If an optimal coupling is rank one, however, every nonzero row and column normalizes to its marginals. An opposite response cannot exceed the LP optimum and cannot be worse than those marginals, so rounding recovers that word's optimum exactly. If the word also attains the largest of the two LP bounds, equality certifies true capacity.

## The frozen rational controls

The strongest retained incumbents here combine the applicable minimum-area triangle, width/contact, signed-prefix and full-partner feasible methods, as already selected and witnessed in the previous LP packet. They are not an obsolete single-method baseline.

| Retained control | Previous exact interval | Standalone rounded upper bound | Combined interval after rounding | Retained LP rank one? |
|---|---:|---:|---:|---|
| `random_5x5_s1_64` | Exactly $6.2032265061\ldots$ | $6.2032265061\ldots$ | Same exact value | Both words |
| `random_5x6_s1_198` | $[4.4365955820\ldots,4.4842275196\ldots]$ | $4.4842275196\ldots$ | Same endpoints; lower endpoint now proved strict | Neither word |
| `random_6x6_s3_124` | $[4.4749801029\ldots,4.6063214932\ldots]$ | $4.4749801029\ldots$ | Exactly $4.4749801029\ldots$ | Both words |

All displayed decimals abbreviate exact rationals retained in [results.json](results.json) and the source certificates. The $6\times6$ feasible upper bound improves by **2.851329%**, closing the whole previously unresolved gap rather than polishing an already certified constant. Both retained couplings factor exactly as the outer product of their normalized marginals; the better word's primal value equals its exact dual bound. Thus the new upper witness proves the exact capacity immediately. Conditional rounding extracts it, but a direct rank-one check would have sufficed. The historical numerical capacity already agreed with this value; it is not promoted to a proof or credited as a discovery.

The unchanged rule gains nothing on the $5\times6$ control. Its standalone rounded bound equals the old upper endpoint, while the LP interval factor remains $1.010736\ldots$. The separate exact graph argument below proves that reaching the lower endpoint is impossible for a true-capacity witness.

## Why the remaining random-body LP gap is genuine

For the $5\times6$ control, the exact negative-shadow LP value $L_-$ is strictly larger than $L_+$. Fix its retained optimal dual and let $S_{ij}\ge0$ be its slack. A rank-one coupling attaining $L_-$ must have $S_{ij}=0$ throughout its active support rectangle, because $\sum S_{ij}\alpha_i\gamma_j=0$.

If any such A closure $\alpha$ existed, the face of $C_A$ restricted to its active coordinates would contain a closure vertex. Replacing $\alpha$ by that vertex retains zero slack on the support of $\gamma$. The five possible A closure vertices have these common dual-tight B columns:

| A support | Common tight B columns |
|---|---|
| $\{0,1,3\}$ | $\{5\}$ |
| $\{0,2,3\}$ | $\{1,5\}$ |
| $\{0,2,4\}$ | $\{3,5\}$ |
| $\{1,2,4\}$ | $\{2,5\}$ |
| $\{1,3,4\}$ | $\{4,5\}$ |

No set supports a nonnegative normalized B closure: the singleton normal is nonzero, and all four pairs have nonzero exact determinant. [graph-obstruction.json](graph-obstruction.json) retains these determinants; [check_gap.py](check_gap.py) verifies the duals, all five individual A vertices and this incidence argument. It computes no Cartesian-pair capacity objectives.

Therefore no rank-one closure coupling attains $L_-$. Compactness makes the fixed-word true maximum strictly smaller. The other shadow's true maximum is at most $L_+<L_-$. The imported sparse/two-shadow theorem then gives $Q_{\rm true}<L_-$, hence $c_{\rm true}>1/(2L_-)$. No quantitative size of the strict gap is claimed. This supplementary proof uses the same frozen control and existing dual, without altering the rounding rule or adding a validation cohort.

## Why HKO rounds exactly despite its LP gap

The previous exact HKO coordinates have $Q_{\rm true}=1/10$, $c_{\rm true}=5$. The retained optimal couplings are **not rank one**, but the independently checked conditional responses attain $Q=1/10$ exactly in $\mathbb Q(\sqrt5)$. The first retained positive-shadow row conditional has B support $\{1,2,4\}$ and its maximizing A response has support $\{0,1,3\}$; [results.json](results.json) contains their exact quadratic-field weights and full word. Direct four-dimensional HK evaluation, rather than numerical agreement, proves recovery. The known global HKO formula bounds every feasible response from above.

There is also a structural explanation for the previously exhibited circulant optimum: its normalized row conditionals are the five cyclic shifts of weights supported on $\{1,3,4\}$. These are exactly all five B closure vertices; there are no antipodal two-support vertices. The sparse/two-shadow theorem then implies that taking their opposite best responses recovers the true optimum. The verifier checks the five-ray coverage, and the proof supplies the implication without a separate support-pair scoring loop. This is a statement about that explicit circulant optimum, not every LP optimizer. The frozen experiment used the original retained optimal couplings without replacement.

The three existing numerical regular rotations $0,\pi/20,\pi/10$ also recover their known capacities within $3\times10^{-16}$ relative error. These floating controls check conventions only. Exact HKO recovery and the rational certificates carry the scientific conclusions.

## Verification, reproduction and scope

[verify.py](verify.py) imports no producer or optimizer. It independently enumerates single-factor basic feasible closures by Gaussian elimination; checks every conditional, closure, one-factor response optimum and retained selected objective; reconstructs the actual four-dimensional HK sum; verifies the source LP primals and dual inequalities exactly; tests rank-one factorization; and checks HKO over the standard-library ordered quadratic field. [verification.json](verification.json) records PASS. The independent coordinator also reconstructed the rational results with a separate Cramer's-rule checker. [review.md](review.md) records scientific review of the final packet.

From the patterns checkout, with no numerical dependency installation:

```sh
python3 -B patterns/upper-tail/conditional-rounding/analyze.py \
  --repo "$PWD" --out /tmp/msc-math/conditional-rounding/replay-results.json
python3 -B patterns/upper-tail/conditional-rounding/verify.py \
  --repo "$PWD" --directory patterns/upper-tail/conditional-rounding
python3 -B patterns/upper-tail/conditional-rounding/check_gap.py \
  --repo "$PWD" --out /tmp/msc-math/conditional-rounding/replay-graph.json
```

To verify a fresh generated results file, copy the packet's PLAN, producer and verifier beside it under the expected filename `results.json`, then point `--directory` there. Checks write only the named scratch output or `verification.json` within the selected directory; no source certificates are replaced. Source hashes bind the reused LP packets. [manifest.json](manifest.json) binds this retained packet's bytes.

All new files began in `/tmp/msc-math/conditional-rounding/20261017T1600Z/` and were deliberately retained with the pattern research owner. The producer took under a second on these pilots; no population producer, new LP solve, full support-pair capacity enumeration, fit, download/install, cloud work, new human question or thesis edit occurred. The scientific stop is the frozen pilots plus the same-control exact obstruction. This method has no demonstrated performance on the design-excluded tail cohort and supplies no universal ratio bound.

Inspected local sources: repository/research/product entry points; collaboration rationale; `patterns/OWNER-FOLLOWUP.md`; stopped core, width-contact, signed-prefix, two-shadow and conditional-coupling reports and implementations; the existing conditional LP certificates and referenced feasible witnesses. Source access was local and limited to these named reports, implementations, certificates and source excerpts, with no external literature search or exhaustive repository novelty claim. The result remains outside the selected manuscript.

[review: /root/upper_tail_investigation/rounding_science_review]
