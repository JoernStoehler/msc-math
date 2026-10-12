# Ridge area under fixed-normal deformation

The inverse relation between ridge area and systolic ratio also occurs under
controlled changes to individual products. On the retained paired
tangentialization panel, the two quantities change in opposite directions in
15 of the 16 interventions that set both factors' support heights to one.
Thus the inverse association is visible within fixed sets of normal
directions; it is not confined to comparisons between unrelated products.
This does not identify the mechanism or establish a universal deformation law.

| Intervention | Opposite signs of ΔR and Δsys | Mean ΔR | Mean Δsys | Spearman(ΔR, Δsys) |
|---|---:|---:|---:|---:|
| Both factors, 4×4 | 8/8 | +0.00437 | +0.00398 | −0.619 |
| Both factors, 4×6 | 7/8 | −0.36357 | +0.02656 | −0.095 |
| Both factors, combined | 15/16 | −0.17960 | +0.01527 | −0.550 |
| First factor only, combined | 13/16 | −0.02208 | +0.00125 | −0.526 |
| Second factor only, combined | 14/16 | −0.15945 | +0.01044 | −0.671 |

The first two rows sum to the third. The last two rows use the same sixteen
latent products and are not additional independent samples. In the primary
both-factor contrast, ten interventions decrease R and increase sys, five
increase R and decrease sys, and one increases both. Correlations concern
changes across interventions, not the relation along each continuous path.
The positive means for both quantities in the 4×4 row are compatible with
opposite signs within all eight pairs because effect sizes differ.

The exception is the quadrilateral×hexagon pair at retained row 3:
R increases from 11.073699 to 11.218174 while sys increases from 0.418712 to
0.436306. Even this restricted intervention therefore has no strict inverse
ordering. The weak correlation between effect sizes in the 4×6 subgroup also
warns against treating the direction agreement as a quantitative capacity
formula.

These bodies cover R in [11.064695, 30.541553] and numerical sys in
[0.112000, 0.741470]. They do not probe the extreme low-R region or the
largest known systolic ratios. The pre-existing endpoint rotation paths give
the contrasting phenomenon: in each retained path, R and sys both decrease
toward the ridge-minimizing endpoint. Together, these observations support a
distinction between an inverse relation visible among ordinary products and
its failure as a universal optimization rule. They do not choose between a
latent structural law plus noise and a distribution-specific correlation.

## Data and computation

This is a new, exploratory analysis of the frozen
`paired-tangentialization/artifacts/{inputs,results}.jsonl` packet. It adds no
capacity requests and changes no geometry. The original packet contains eight
latent draws in each of the 4×4 and 4×6 buckets, with all four arms retained.
Acceptance required all arms to remain bounded and nonredundant before any
capacity evaluation. Consequently the sample describes that jointly
admissible conditional design; it is not a population sample of all products.
The original report's mean effects alone did not compare ΔR with Δsys.

`controlled.py` reconstructs planar vertices from each arm's recorded normal
directions and support heights. It uses

    R(P × Q) = sum_ij |edge_i(P) · edge_j(Q)| / sqrt(area(P) area(Q)).

The script checks the complete sixteen-by-four grid, successful evaluator
results, matching input hashes and geometry, the capacity/sys formula,
unchanged normal directions across arms, positive orientation, all support
inequalities, and agreement between reconstructed product volume and the
retained evaluator volume. Maximum area-normalization error is 6.67e−16;
maximum volume disagreement is 2.23e−15. An independent width formula checks
each edge sum. The same computation agrees with the retained four-dimensional
ridge API values for all eight endpoint-path geometries. All primary ΔR
magnitudes exceed 0.0439 and all primary Δsys magnitudes exceed 0.00157, far
above the observed reconstruction roundoff.

The stored capacities and volume-derived systolic ratios retain their original
numerical status; this analysis does not certify them symbolically. No
significance test or confidence interval is assigned to the sixteen signs.
The packet is small, and the analysis was proposed after the original
tangentialization outcomes were known.

Reproduce from any working directory with Python 3 and NumPy:

```sh
python3 experiments/sys-datascience/methods/selection-mechanism/controlled.py
```

`controlled-artifacts/summary.json` contains input hashes, all summaries, the
exception, and the endpoint values. `geometries.jsonl` contains every
reconstructed R and retained sys. `contrasts.jsonl` contains every paired
difference. The script writes only those three artifacts.
