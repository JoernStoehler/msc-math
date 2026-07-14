# Literal Branch-Gradient Multi-Start Discussion Packet

## Question and algorithm

This packet asks whether the favorable motivating trajectory survives a small, pre-target multi-start check. Each step chooses the deterministic currently minimizing admissible branch and unconditionally applies `a <- a + eta * grad_a sys_sigma(a)`. There is no normalization, projection, near-active set, maximin direction, line search, acceptance test, or early stopping. Invalid geometry and decreases are retained observations.

## Evaluation population

The evaluation uses the first 6 `F=6` rows in canonical generic-random source order after excluding the already-observed `random_F6_s0_1`. Every start receives all six rates and up to 100 updates. The source generator uses seed 42 and height interval `[0.8,1.2]`. Selection used neither initial `sys` nor optimizer outcomes. The motivating start appears only as a labeled diagnostic in the selected-trajectory figure.

This is a descriptive sample of 6 starts (36 paired trajectories), not a precise estimate for all random `F=6` polytopes and not deterministic rerun replication. The fixed operational threshold used for prefix classification is a best-so-far increase of at least 1% of initial `sys`. It was chosen before full producer execution but was not independently preregistered; treat the counts as descriptive.

## Direct observations

- At least one rate achieved the material-gain threshold on **6/6 starts**.
- **31/36 trajectories** completed 100 updates; **5/36** became mathematically invalid before then. **4/6 starts** had at least one invalid rate.
- The 8-iteration practical class disagreed with the complete/terminal class on **6/36 trajectories**; at 20 iterations it disagreed on **5/36**.
- The best value improved after iteration 8 on **31/36 trajectories** and after iteration 20 on **31/36**. This includes arbitrarily small improvements; use the class-disagreement count for the 1% practical threshold.
- Among complete trajectories, the final state was at least 1% of initial `sys` below an earlier best on **13/31**. Invalid trajectories are excluded from that denominator; for them, no valid 100-update endpoint exists. The producer's legacy `summary.json` field `final_sys` stores the last valid pre-failure state, while `analysis.json` sets evaluative `final_sys` to null and preserves that value separately as `last_valid_sys`.

Exact per-rate denominators, medians, quartiles, switch counts, and censoring are in `analysis.json`; the paired heatmap makes start/rate heterogeneity visible without pooling failures away.

## Interpretation and competing explanations

The motivating success was not unique: useful retained gains occur across the evaluation starts. But the literal rule is not a stable endpoint optimizer. Rate and start jointly control invalidity, late recovery, and whether the final state preserves the best value. Frequent branch switches and raw decreases are compatible with repeatedly following a branch that ceases to minimize after the update; this packet observes that pattern but does not establish a causal mechanism.

A favorable reading is that the rule supplies cheap search directions and that best-so-far retention converts unstable paths into useful candidates. A less favorable reading is that the apparent gains come from a narrow generator slice and a six-rate sweep, while invalidity and endpoint regret mean the literal rule itself is too brittle for deployment. The current sample separates those readings only for this source prefix; another generator or facet count could behave differently.

## Research decision

**Retain literal ascent as a deliberately weak paired search baseline, and next compare it against one minimal safeguarded variant on these same frozen starts and rates.** The safeguard should preserve the same gradient proposal while adding explicit best-state retention plus rejection/backtracking of invalid or decreasing full-`sys` updates. Freeze this packet as the baseline; do not tune the baseline after seeing the comparison.

Testing another population first has lower immediate information value: this packet already shows both cross-start utility and severe trajectory pathology. A same-start safeguard comparison would directly test whether minimal optimizer machinery removes the observed failure modes without attributing generator variation to the method.

## Allowed and prohibited conclusions

Allowed: on this six-start generic-random `F=6` prefix, report exact empirical rates for retained gain, invalidity, prefix disagreement, branch switching, and final regret; use the motivating start only as a diagnostic example; treat best-so-far retention as operationally important for this packet.

Prohibited: population-wide success probabilities; a generally optimal learning rate; claims about other facet counts or generators; independence of trajectories sharing a start; treating deterministic reruns as replication; convergence, monotonicity, local maximality, or a mechanism theorem.

## Reproduction and validation

The retained producer reports `303.5` seconds of trajectory wall time with parallelism `8`. `analysis.json` records source SHA-256, row-count and paired-coverage checks, exact update-identity checks, source-row identity, and generated figure paths. Figure examples are selected post hoc and labeled by role:

- Motivating diagnostic (not evaluation): `random_F6_s0_1`, `eta=0.1`
- Evaluation: largest gain added after iteration 20: `random_F6_s0_2`, `eta=0.001`
- Evaluation: largest final regret: `random_F6_s0_2`, `eta=0.1`
- Evaluation: latest invalidity: `random_F6_s0_4`, `eta=1`
