# Fixed s0 omitted-basin continuation

17 October 2026. Scientific worker `/root/basin_continuation`; receiving owner
`/root`, who owns independent review, integration, commits and cleanup.

**Success story:** settle whether the already omitted strict negative-word basin
A024/B013 can become globally winning on the existing seed-zero radial
continuation from scale 1/100 through 1/20. Either retain a genuine exact
all-seed recovery counterexample, or prove a precise route obstruction. This is
a bounded step toward Jörn's preference for simple upper-tail insight; more
success counts or tiny bound improvements do not complete it.

Working backward, the needed ingredients are (1) bind the route to previously
retained geometry, so a direction is not chosen after seeing new capacity
labels; (2) preserve the target's feasibility, support and angular interpretation,
so the same basin is being followed; (3) order its payoff crossing relative to
seed capture, or prove crossing impossible on the whole interval. Point samples
cannot replace the interval step. If one feasible competitor dominates every
word of the target throughout, LP and seed-path continuation is unnecessary:
the target never becomes globally winning, whatever seeds do.

| Option | Useful result | Rubric failure | Selection |
| --- | --- | --- | --- |
| Existing s0 radial line | Exact crossing or whole-interval obstruction | Rewards reused geometry even if it has no winning route | Selected because it directly tests the proposed continuation and has a substantive negative outcome |
| Tailored 20-dimensional payoff direction | Potential winning crossing | Maximizing payoff gain may destroy the LP/seed chamber or above-one condition | Deferred; no optimizer or post-hoc direction search |
| All-direction compatibility theorem | General explanation of seed recovery | Generality rewards an unbounded problem with no checkable endpoint | Outside bounded step |

A rubric combining structural implication, exact falsifiability and bounded
cost selects the first. An accuracy-count rubric would repeat known successes;
a payoff-increase rubric could reward a still losing target.

**Fixed route:** write $h$ for the retained rational presentation obtained by
substituting $2.236068$ for $\sqrt5$ in the HKO fixture, and let
$d_k=(\operatorname{SHA256}(\texttt{msc-math/thesis/lp-seeded-response/validation/v1:0})_k\bmod9-4)/4$ for the first twenty digest bytes. Set
$x(t)=h+td$ for $1/100\le t\le1/20$. This base is a rational approximation,
not the exact algebraic HKO product. The endpoints are exactly the existing
`lpseed_near_hko_s0_e100` and `lpseed_near_hko_s0_e20` controls.

Root confirmed this route before new symbolic calculations. The stop point is
an exact all-interval competitor obstruction or the first exact extractor
counterexample. Optional extension toward zero/first chamber boundary was
allowed only if analytically tractable and was not needed. No new LP solve,
optimizer, broad cohort, predictor fit, network, installation, cloud work or
manuscript edit is selected. Budget ceiling: 25 minutes wall time and 120 CPU
seconds. The retained certificate computation took under one CPU second.
