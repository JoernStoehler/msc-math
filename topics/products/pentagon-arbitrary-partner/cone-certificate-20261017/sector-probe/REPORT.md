# Frozen sector row-product LP: numerical infeasibility

17 October 2026. The single authorized feasibility probe returned **HiGHS
status 2, infeasible**, in 2.26 seconds. This is a numerical negative for the
specified certificate class, not an exact mathematical obstruction. No
certificate, dual separator or sharp inequality was established. The probe
stopped without changing the sector or searching another certificate class.

## Choice and exact LP definition

| Option | Value if successful | Rubric failure | Selection |
| --- | --- | --- | --- |
| Nonnegative sector-row products only | Sparse coefficients can be reconstructed over $\mathbb Q(\sqrt5)$ | Treating solver status as proof | One frozen bounded LP, completed |
| Products plus PSD remainder | Strictly broader possible certificate class | Increasing solver complexity after a negative without a new bounded assignment | Not run |
| Full face enumeration | Could establish or refute positivity directly | Large case counts without complete coverage | Not run |

The target and original 610 rows are exactly those in the parent obstruction
packet. Append the two frozen sector rows $u$ and $(5-2\sqrt5)t-u$, where
$t=b_{00}+b_{11}$ and $u=b_{10}-(5-2\sqrt5)b_{01}$.
The proposed identity is

$$q(z)=\sum_{i\le j\le612}a_{ij}\ell_i(z)\ell_j(z),\qquad a_{ij}\ge0.$$

At the original HKO equality point, the two new rows are strictly positive
and exactly 20 original rows vanish. Nonnegativity at this point forces every
retained product to have at least one factor among those 20 active rows.
Thus all potentially nonzero terms are covered by **12,050 columns**:
$20\cdot612-\binom{20}{2}=12050$.
This pruning is exact and does not depend on numerical tolerance or labels.

The LP equates all **28 homogeneous quadratic coefficients** in seven
variables, including the factor two in off-diagonal target coefficients.
It has objective zero, nonnegative variable bounds, and no PSD remainder.
The pair set, sector and target were frozen before the solve. Numerical
coefficients are evaluations of the exact $\mathbb Q(\sqrt5)$ entries.

HiGHS used a 30-second time limit with primal and dual feasibility tolerances
$10^{-9}$. Its result was:

```text
The problem is infeasible.
HiGHS Status 8: model_status is Infeasible; primal_status is None
```

The solver supplied no primal vector, so exact sparse-coefficient
reconstruction was inapplicable. No exact infeasibility certificate was
obtained. The negative does not decide the larger PSD-remainder class,
sector positivity, the inherited general-matrix sharp bound, or arbitrary
product maximality.

## Receipt and reproduction

`probe.py` writes only its explicitly named JSON output. It imports the parent
standard-library checker for exact source and active-set controls, then uses
already unpacked compatible packages through explicit `sys.path` entries:

- SciPy 1.18.1: `/home/joern/.cache/uv/archive-v0/RcJQTM1A-Mr1_OFP`.
- NumPy 2.5.2: `/home/joern/.cache/uv/archive-v0/BdkG5We34Xm1t-b1`.

No installation, environment creation or network access occurred. The source
`cone.json` SHA256 is
`9f02548e305ed586b83b2709719818281847ce2d2e5b66981b35973acc07e9b6`.
`result.json` records the package paths, versions, settings, column/equation
counts, source identity, solver status and elapsed time.

```sh
python3 -B topics/products/pentagon-arbitrary-partner/cone-certificate-20261017/sector-probe/probe.py \
  topics/empirical-studies/discovery-design/night-20261005/orientation-wide/outer-matrix-preflight/cone.json \
  topics/products/pentagon-arbitrary-partner/cone-certificate-20261017/check.py \
  /tmp/msc-math/cone-certificate/sector-replay.json
```

No original-body capacity computation, dataset scan, sector tuning, second
LP, SDP or face enumeration followed. This is the completed bounded control;
the parent exact obstruction remains the material mathematical result.

[review: /root/delivery_review]
