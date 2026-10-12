# Gaussian reference for attenuation under selection

Selection weakens the observed ridge/sys rank relation much more than a fitted
Gaussian-copula reference predicts. With the lowest 5% of ridge values retained
within each original group, the observed conditional rank correlation is
−0.113; the reference has mean −0.695 and central 95% simulation interval
[−0.736, −0.647]. Retaining the highest 5% of systolic ratios gives −0.239,
against reference mean −0.695 and interval [−0.738, −0.649]. Thus matching the
strong bulk association with this simple rank-dependence model does not
reproduce the observed loss of association under selection.

| Retained fraction | Observed: high sys | Reference mean: high sys | Observed: low R | Reference mean: low R |
|---|---:|---:|---:|---:|
| 100% | −0.944 | −0.944 | −0.944 | −0.944 |
| 50% | −0.759 | −0.854 | −0.746 | −0.854 |
| 20% | −0.496 | −0.778 | −0.434 | −0.778 |
| 10% | −0.286 | −0.733 | −0.222 | −0.733 |
| 5% | −0.239 | −0.695 | −0.113 | −0.695 |
| 2% | −0.117 | −0.637 | +0.019 | −0.638 |
| 1% | −0.036 | −0.589 | +0.035 | −0.590 |

For both selection directions, all six selected fractions lie above their
pointwise 97.5% reference quantile. The complete-sample statistic lies inside
the reference interval, as expected from calibration. At 1%, the reference
intervals are [−0.701, −0.457] for high sys and [−0.698, −0.471] for low R.

## Construction and scope

Each of the eighteen original groups receives its own bivariate normal model,
with correlation r = 2 sin(πρ/6), where ρ is that group's observed full-sample
Spearman correlation. This is the Gaussian-copula conversion: arbitrary
strictly monotone marginal transformations do not change the ranks or the
selection statistic. Simulations preserve the eight generic groups of 512
rows and ten product groups of 1,024 rows, rather than replacing heterogeneous
groups by one pooled correlation. The seed is 2026091801, with independent
NumPy SeedSequence child streams in alphabetical group order. There are
1,000 replications.

Every replication uses exactly the original statistic: retain
ceil(fraction × group size), re-rank both variables within each selected
group, divide by that group's selected size, concatenate all rows, and compute
Spearman correlation with average ranks for ties in the pooled percentiles.
The optimized computation agrees with the original `analyze.py` function for
the first replicate at all fourteen selection/fraction settings to 1.12e−16.
The run takes approximately seven seconds and makes no capacity evaluations.

This fitted Gaussian rank model does not reproduce the observed attenuation.
It does not rule out range restriction
under other dependence structures, establish a change in a geometric law, or
choose between latent structure and noise explanations. Parameters were
estimated from the data being compared, and the model was chosen after the
attenuation was observed. The intervals hold those parameters fixed, are
pointwise rather than simultaneous, and are descriptive simulation references;
they are not confidence intervals for the observed correlations or a formal
model test.

The existing `tail-dependence-feasibility` packet analyzes tail overlap and
selected-panel dependence but contains no Gaussian benchmark. This analysis
adds that comparison without altering any retained scientific observations.

Reproduce with Python 3 and NumPy:

```sh
python3 experiments/sys-datascience/methods/selection-mechanism/gaussian_benchmark.py
```

`gaussian-artifacts/` retains fitted group parameters, pooled and group-level
reference intervals, all pooled replicate statistics, and source hashes.
