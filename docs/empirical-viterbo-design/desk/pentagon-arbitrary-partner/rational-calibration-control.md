# One exact nonregular calibration control

3 October 2026 UTC. The producer `rational-calibration-control.py` retains a
specific rational nonregular pentagon `Q`, its reference body
`B_Q=(Q-Q)^polar/2`, and exact translation covers for every normal template.
It is a small constructive example of the calibration method, not a claim that
an unspecified perturbation lies inside the stability neighborhood.

## Mathematical conclusion

For this explicit `Q`, the BMP covering criterion and the documented segment
bound prove `c_EHZ(B_Q ×_L Q)=1`. The symmetric-containment transfer therefore
computes `c_EHZ(K ×_L Q)=W_Q(K)` for every centrally symmetric partner `K`.
The exact control product also has rational area, four-volume and systolic
ratio labels in `rational-calibration-control.json`.

Displayed decimal values are only approximations to those retained rationals:

| Quantity | Approximation |
| --- | ---: |
| Capacity of `B_Q ×_L Q` | 1 (exact) |
| Area of `Q` | 2.3750549769682454 |
| Area of `B_Q` | 0.224756986243347 |
| Four-volume of the product | 0.5338101987856447 |
| Systolic ratio | 0.9366625087670506 |

This example's ratio need not equal the regular-pentagon sharp constant:
its factor is a different, nonregular polygon.

## What is checked

All arithmetic uses Python's `fractions.Fraction` with no floating-point
geometry. The producer enumerates all facet-line intersections, filters the
feasible ones, checks five genuine edges and finds positive noncollinear closure
triples. These give boundedness and strict origin containment. Facet rows are
not assumed to be unit normals; normalization uses their actual support values.

It examines all ten three-normal subsets, obtains five positive closures,
checks exact closure and unit support-function perimeter, and supplies a
translation of each triangle into `B_Q`. The other cyclic order is covered by
central inversion and translation, since `B_Q=-B_Q`. Nonparallel normal pairs
exclude degenerate normal segments.

The triangle placements are checked against every vertex difference of `Q`:
`d.x<=1/2`, precisely the halfplanes of `B_Q`. The reference body's complete
vertex inventory is also checked by exact halfplane intersections before its
shoelace area is used. Angular sorting for area uses exact cross products,
not floating-point angles.

An independent bounded audit by `autonomous_portfolio` found no material
arithmetic, geometric, enumeration or transfer gap. Its separate exact
halfplane enumeration confirmed the ten reference vertices. The producer was
subsequently strengthened to check that inventory directly and reject Python's
disabled-assertion mode.

## Reproduction and evidence boundary

From the repository root, with assertions enabled:

```bash
python3 docs/empirical-viterbo-design/desk/pentagon-arbitrary-partner/rational-calibration-control.py
```

The command prints a freshly generated certificate and does not overwrite any
retained artifact. Its output can be compared with the retained JSON. It is a
producer for this input, not an independent checker of arbitrary edited JSON.
The program rejects `python -O`; its exact checks must execute.

The mathematical implication imports the
[BMP translation-cover characterization](https://arxiv.org/html/2603.12495v2)
and the transfer argument in `odd-regular-symmetric-partner.md`.
Exact arithmetic alone would not provide those implications.

No production capacity solver was run on this input. These labels are not a
certificate of the Rust implementation, a quantified perturbation radius,
general empirical adequacy, human proof acceptance or publication novelty.
Converting rational facet rows to binary64 changes the input body unless those
rows are exactly representable; a future implementation comparison must retain
the actual bits and certify the corresponding body.
