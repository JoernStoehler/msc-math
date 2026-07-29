# Optimizer trajectory analysis

Strict validation passed for 16 runs, 506 evaluations, 490 proposals, and 490 rounds.

Facet-count-specific results are primary. The pooled table below is descriptive only: it must not override a reversal or failure at any individual facet count.

## Best median by facet count

| facets | best algorithm | median final sys | n |
|---:|---|---:|---:|
| 10 | history-baseline | 0.985194 | 16 |

## Pooled final outcomes (secondary)

| algorithm | n | median final sys | 10–90% | invalid | exact fallback |
|---|---:|---:|---:|---:|---:|
| history-baseline | 16 | 0.985194 | 0.961244–0.998374 | 0.0% | 0.0% |

The 10–90% interval is the across-start distribution, not uncertainty about its median. `final-summary.csv` also contains bootstrap intervals.

## Best median by budget

| charged calls | best algorithm | median best sys |
|---:|---|---:|
| 8 | history-baseline | 0.965607 |
| 16 | history-baseline | 0.982088 |
| 32 | history-baseline | 0.984843 |
| 64 | history-baseline | 0.985186 |
| 128 | history-baseline | 0.985194 |
| 256 | history-baseline | 0.985194 |
| 512 | history-baseline | 0.985194 |
| 640 | history-baseline | 0.985194 |

## Evaluator profile

| algorithm | median eval ms | geometry | volume | capacity | evaluator / run wall | ask+tell / run wall |
|---|---:|---:|---:|---:|---:|---:|
| history-baseline | 11.94 | 1.5% | 0.9% | 97.7% | 23.9% | 74.6% |

## Termination and observed maxima

| algorithm | median charged calls | 10–90% calls | maximum best sys | runs reaching sys >= 1 | stop reasons |
|---|---:|---:|---:|---:|---|
| history-baseline | 19.5 | 15–72.5 | 0.999826845 | 0 | compute_budget_exhausted=2; minimum_inner_distance=3; optimizer_returned_no_proposals=11 |

See `best-sys-by-call.png`, `best-sys-by-measured-compute.png`, `best-sys-by-call-and-facet.png`, `checkpoint-selection.json` for the trajectory curves and independent-probe input.
