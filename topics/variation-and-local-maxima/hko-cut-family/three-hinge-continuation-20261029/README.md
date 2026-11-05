# Three characteristic-invariant affine hinges: initial probe

This bounded probe asks whether three inward affine hinges on distinct HKO
facets can have positive first systolic variation while every original row is
retained. The cut on a q facet is

\[
 f_i(\xi_i,\eta_i)=(\alpha_i\xi_i+\beta_i\eta_i+\gamma_i)_+,
 \qquad \xi_i=\frac{e_i\cdot q}{\sin(\pi/5)},\quad
 \eta_i=\frac{e_i\cdot p-m}{h},\quad e_i=\frac{J_2n_i}{|n_i|}.
\]

The exploratory search used the three consecutive q facets `0,1,2`, with
independent affine coefficients. Each 3-vector was positively homogeneous;
the nine coefficients were normalized together by their total L1 norm, so
relative hinge sizes were free. The volume contribution of each profile was
integrated over uniform `xi` and the appendix's exact piecewise-linear `eta`
density, using the closed form for the `xi` integral and 96-point
Gauss–Legendre quadrature on each density interval. For each of the 190 exact
simple-maximizer charts, the script reconstructs exact KKT endpoints, converts
them to floating coordinates, then maximizes the resulting piecewise-quadratic
response over every hinge-zero interval, including interior stationary points.

## Outcome

There is an exact obstruction for **constant** profiles, for any finite number
of hinges. Average the occupation measures of a maximizing word over the HKO
symmetry group, which acts transitively on the ten labeled facets (including
the symplectic factor swap). The average still comes from maximizing words and
has mass `1/10` on each facet. For nonnegative constants `c_i`, the maximum
occupation response is therefore at least its average, `sum_i c_i/10`, equal
to the total first volume variation. The repeated-word reduction applies
because constant profiles are characteristic-invariant. Thus this family
cannot give positive first variation, regardless of hinge count or placement.

The exact replay script gives a tight three-facet example on adjacent q
facets `0,1,2`: constants
`c0=c2=(5-sqrt(5))/10`, `c1=sqrt(5)/5` attain both response and volume
`1/10`. Catalogue chart 0, word `(0,6,5,2,8,4)`, attains the response. This
explains the nearly constant limit found by numerical optimization; it does
not obstruct nonconstant affine hinges.

This particular constant triple is also a symmetry-null variation: its five
q-row support changes can be written `c_i = a + n_i·u` for a common dilation
and q translation, with `c_3=c_4=0`. It is an optimizer sanity check, not a
new nonproduct direction.

Two differential-evolution starts (SciPy, 50 iterations, population factor 7)
on the adjacent q-facet triple `0,1,2` found no ascent. Their best reported
response-to-total-volume ratios were:

| Start | Ratio `max R / sum nu` | Approximate outcome |
| --- | ---: | --- |
| 17 | 1.0002296 | Profiles nearly constant; total volume `0.0999392`, response `0.0999622` |
| 83 | 1.0187093 | Total volume `0.0955271`, response `0.0973143` |

A separate identical-profile search also failed to find ascent and converged to
a truncated radial profile on the parameter boundary, with volume `0.000378960`
and response `0.00180902`. This is consistent with, but does not extend, the
known exact obstruction for the radial profile at `gamma=-3/5`.

Local Nelder–Mead refinement from start 17 reduced the ratio to `1.0001851`;
the three profiles remained within `2.3e-4` of constants. Powell refinement
gave `1.0002171`. A second q-facet arrangement and two independent-profile
starts on mixed facets `(q0,p0,q1)` gave ratios `1.22760`, `1.22173` and
`1.21663`. Full floating profile vectors and volume/response values are in
[`probe-results.json`](probe-results.json).

Requiring aggregate partner-coordinate slope
`sum_i |beta_i| >= 0.10` after common L1 normalization removes the near-neutral
q-factor limit. Two further starts on q facets `0,1,2` gave ratios `1.06084`
and `1.07137`. An exhaustive scan of all 120 facet triples for three
identical radial profiles at thresholds `0.5,0.6,0.7` also found no positive
gap; those chart maxima are evaluated in floating point.

These are numerical search results, not a nonexistence theorem. Differential
evolution did not converge within the iteration limit, only consecutive q
facets were used for the q-only independent-profile runs, only one mixed facet
triple was sampled, and all chart optimization and volume quadrature were
floating point. No candidate with ratio below one was found; arbitrary facet
selections and a certified exact response remain open.

## Reproduction

From the repository root, run with the configured Sage Python environment
(which supplies NumPy and SciPy):

```bash
/workspaces/msc-math/.local-environments/miniforge/envs/sage/bin/python \
  topics/variation-and-local-maxima/hko-cut-family/three-hinge-continuation-20261029/affine-probe.py
/workspaces/msc-math/.local-environments/miniforge/envs/sage/bin/python \
  topics/variation-and-local-maxima/hko-cut-family/three-hinge-continuation-20261029/nonproduct-probe.py
/workspaces/msc-math/.local-environments/miniforge/envs/sage/bin/python \
  topics/variation-and-local-maxima/hko-cut-family/three-hinge-continuation-20261029/refine-local.py
/workspaces/msc-math/.local-environments/miniforge/envs/sage/bin/python \
  topics/variation-and-local-maxima/hko-cut-family/three-hinge-continuation-20261029/mixed-probe.py
/workspaces/msc-math/.local-environments/miniforge/envs/sage/bin/python \
  topics/variation-and-local-maxima/hko-cut-family/three-hinge-continuation-20261029/radial_probe.py
python topics/variation-and-local-maxima/hko-cut-family/three-hinge-continuation-20261029/constant-triple-check.py
```

The script uses [the exact HKO KKT core](../../../capacity-and-qp/convex-systolic-optimization/certificates/hko-ascent/exact_core.py),
[the exact primitive closure vertices](../../../capacity-and-qp/convex-systolic-optimization/certificates/hko-ascent/expected/primitives.json),
and [the complete 190-chart catalogue](../../../capacity-and-qp/convex-systolic-optimization/certificates/hko-ascent/expected/catalogue.json).
