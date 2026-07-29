# Optimizer trajectory analysis

Strict validation passed for 448 runs, 28824 evaluations, 28376 proposals, and 16338 rounds.

Facet-count-specific results are primary. The pooled table below is descriptive only: it must not override a reversal or failure at any individual facet count.

## Best median by facet count

| facets | best algorithm | median final sys | n |
|---:|---|---:|---:|
| 10 | history-baseline | 0.984783 | 64 |

## Pooled final outcomes (secondary)

| algorithm | n | median final sys | 10–90% | invalid | exact fallback |
|---|---:|---:|---:|---:|---:|
| history-baseline | 64 | 0.984783 | 0.957033–0.998515 | 0.0% | 0.0% |
| directional-above-8e-2 | 64 | 0.981647 | 0.955934–0.993962 | 0.0% | 0.0% |
| gap-w1e-1-adaptive-d1e-1 | 64 | 0.977159 | 0.948177–0.994597 | 0.0% | 0.0% |
| literal-eta1e-2 | 64 | 0.881344 | 0.789542–0.923898 | 0.0% | 0.0% |
| safeguarded-adaptive-d1e-1 | 64 | 0.855083 | 0.780427–0.916023 | 0.0% | 0.0% |
| cma-s1e-1-l8 | 64 | 0.818309 | 0.718359–0.869249 | 2.5% | 0.0% |
| pattern-r3e-2 | 64 | 0.60103 | 0.358319–0.786578 | 1.3% | 0.0% |

The 10–90% interval is the across-start distribution, not uncertainty about its median. `final-summary.csv` also contains bootstrap intervals.

## Best median by budget

| charged calls | best algorithm | median best sys |
|---:|---|---:|
| 8 | directional-above-8e-2 | 0.972898 |
| 16 | history-baseline | 0.984475 |
| 32 | history-baseline | 0.984783 |
| 64 | history-baseline | 0.984783 |
| 128 | history-baseline | 0.984783 |

## Post-hoc two-run portfolio on the held-out outcomes

This allocation was selected after seeing these held-out outcomes. It is descriptive only and would need a new independent population before it could be treated as a confirmed optimizer choice.

| facets | allocation | mean regret to observed oracle | worst regret | within 0.01 |
|---:|---|---:|---:|---:|
| all | 32×directional-above-8e-2 + 96×history-baseline | 0.00153207 | 0.0403676 | 95.3% |
| 10 | 32×directional-above-8e-2 + 96×history-baseline | 0.00153207 | 0.0403676 | 95.3% |

## Evaluator profile

| algorithm | median eval ms | geometry | volume | capacity | evaluator / run wall | ask+tell / run wall |
|---|---:|---:|---:|---:|---:|---:|
| cma-s1e-1-l8 | 9.428 | 1.7% | 1.1% | 97.2% | 99.6% | 0.2% |
| directional-above-8e-2 | 9.433 | 1.7% | 1.0% | 97.3% | 20.8% | 78.1% |
| gap-w1e-1-adaptive-d1e-1 | 9.482 | 1.6% | 1.1% | 97.3% | 46.0% | 53.9% |
| history-baseline | 9.243 | 1.8% | 1.1% | 97.2% | 24.0% | 74.9% |
| literal-eta1e-2 | 9.851 | 1.6% | 1.0% | 97.4% | 99.0% | 0.9% |
| pattern-r3e-2 | 9.19 | 1.8% | 1.2% | 97.0% | 99.8% | 0.1% |
| safeguarded-adaptive-d1e-1 | 11.05 | 1.5% | 0.9% | 97.6% | 97.7% | 2.2% |

## Termination and observed maxima

| algorithm | median charged calls | 10–90% calls | maximum best sys | runs reaching sys >= 1 | stop reasons |
|---|---:|---:|---:|---:|---|
| history-baseline | 20.5 | 14–34.1 | 0.999826244 | 0 | compute_budget_exhausted=41; minimum_inner_distance=5; optimizer_returned_no_proposals=18 |
| directional-above-8e-2 | 17 | 12–24 | 0.999711387 | 0 | compute_budget_exhausted=39; minimum_inner_distance=4; optimizer_returned_no_proposals=21 |
| gap-w1e-1-adaptive-d1e-1 | 40 | 32–58.7 | 0.997275562 | 0 | compute_budget_exhausted=48; distance_schedule_finished=16 |
| literal-eta1e-2 | 93.5 | 65.3–128 | 0.948214106 | 0 | budget_exhausted=8; compute_budget_exhausted=56 |
| safeguarded-adaptive-d1e-1 | 57.5 | 37–87 | 0.937474618 | 0 | budget_exhausted=1; compute_budget_exhausted=19; distance_schedule_finished=44 |
| cma-s1e-1-l8 | 97.5 | 72–128 | 0.932068821 | 0 | budget_exhausted=13; compute_budget_exhausted=51 |
| pattern-r3e-2 | 103.5 | 74.6–128 | 0.842538756 | 0 | budget_exhausted=17; compute_budget_exhausted=47 |

See `best-sys-by-call.png`, `best-sys-by-measured-compute.png`, `best-sys-by-call-and-facet.png`, `paired-checkpoint-comparisons.csv`, `checkpoint-selection.json` for the trajectory curves and independent-probe input.
