# What remains of the ridge correlation after selection?

The strong ridge-sum/systolic-ratio association weakens when attention is
restricted to either the smallest ridge sums or the largest numerical systolic
ratios. In the original 14,336-body table, the pooled within-group rank
correlation is −0.944. Among the highest 10% of ratios in each source/facet or
polygon-pair group it is −0.286, and among the highest 5% it is −0.239.
Selecting the lowest 10% and 5% of ridge sums instead gives −0.222 and −0.113.
These direct high-ratio-conditioned calculations fill a previously uncomputed
part of the chapter's empirical question.

| Fraction retained within every group | Bodies | Largest numerical sys | Smallest R |
| --- | ---: | ---: | ---: |
| 100% | 14,336 | −.944 | −.944 |
| 50% | 7,168 | −.759 | −.746 |
| 20% | 2,874 | −.496 | −.434 |
| 10% | 1,446 | −.286 | −.222 |
| 5% | 728 | −.239 | −.113 |
| 2% | 298 | −.117 | +.019 |
| 1% | 158 | −.036 | +.035 |

The smallest subsets contain only six generic or eleven product bodies per
group. Their coefficients are unstable descriptive quantities, and the near-zero
pooled value does not say every group loses its association. At 5%, the
high-sys-conditioned coefficients remain negative in 16/18 groups, ranging
from −.579 to +.151. All individual values are retained.

This calculation cannot distinguish an underlying geometric mechanism from
noise in a proxy. Correlations can attenuate through range restriction alone.
It shows why a strong broad-population rank ordering should not be extrapolated
to the restricted populations relevant to a search for unusually large ratios.
It does not show that selecting still smaller R worsens expected sys.

## Definition, source and reproduction

`analyze.py` reads the exact historical invariant-table snapshot identified by
SHA256 `607c8731fa03d190d497edc3e8f1b4cca88f7d238260cce527680f568bc33d59`
and its tracked provenance table. The script checks matching unique identities,
18 group sizes, complete ridge-face ordering, finite values and the absence of
selection-boundary ties. The target remains the historical stored numerical
sys value; no capacities or volumes are reevaluated.

There are eight generic groups of 512 bodies and ten product groups of 1,024.
For fraction f, each group contributes ceil(f n) bodies with largest sys or
smallest R. R and sys are **reranked within each selected group**, and ranks are
divided by its selected size. Spearman correlation of the pooled conditional
percentiles is the plotted statistic, following the existing conditional-tail
atlas convention. Each body has equal weight; the result is not an equal-group
average. Equal-group mean and median correlations are also in the output.

```sh
OPENBLAS_NUM_THREADS=1 uv run --script \
  experiments/sys-datascience/methods/selection-mechanism/analyze.py
```

The default table path is the standard local artifact cache. `--table` and
`--provenance` override paths; expected content hashes remain mandatory. Use
`--no-figure` with ordinary Python plus NumPy to skip Matplotlib. Only the
chosen output directory is written; no source dataset is modified.

- `artifacts/conditional-ranks.tsv`: plotted coefficients and group summaries.
- `artifacts/conditional-ranks-by-group.tsv`: every group's coefficient and ranges.
- `artifacts/selection-checksums.tsv`: hashes of the selected identities.
- `artifacts/metadata.json`: exact input/script hashes and interpretation.
- `artifacts/conditional-ranks.pdf`: static thesis figure, also available as PNG.

Independent review on 18 September verified identities, grouping, cutoffs,
weighting, all summary/group-count correspondence and the two 5% coefficients
using a different rank implementation. Those values agreed to 3e−17. This is
an exploratory analysis of known targets, not a prospective selector test.
