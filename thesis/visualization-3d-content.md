# Visualization In 3D Content Notes

Status: maintenance companion for `thesis/10-visualization-3d.tex`; not source
truth. The section and figure selection were assembled on 2026-07-11 and still
require ordinary integrated-thesis review by Jörn/Kai.

## Thesis Role And Interpretation

Visualization is a small exploratory side result required by `FACTSHEET.md`
items 8.9 and 11. It helps readers imagine a 4D polytope boundary and a
piecewise-linear Reeb orbit. It did not produce a reliable visual hypothesis,
candidate rule, or proof input. The active section preserves Jörn's 2026-06-11
negative framing: projection to three dimensions lost too much visible
Reeb-geometric information, and impressions were projection-dependent.

## Selected Assets

Both files were regenerated from `experiments/visualization/` and deliberately
copied into the self-contained thesis tree.

| Thesis asset | Producer input and rendering | Purpose and status |
|---|---|---|
| `figures/visualization/viz-hypercube-ridges.png` | `hypercube.json`; edges plus interpolated spherical two-face meshes; radial map to `S^3`, stereographic pole `e4`, clipping radius 6, camera `(2.6,1.95,3.25)` | Thesis-ready qualitative explanation of boundary structure. The meshes are not exact metric images or evidence. |
| `figures/visualization/viz-hko-pentagon-min-orbit.png` | `hko_pentagon.json`; HKO Lagrangian product of two regular pentagons; grey one-skeleton plus trajectory index 0, currently a recovered six-segment minimum-action orbit; same pole and clipping radius, camera `(3.2,2.4,4.0)` | Thesis-ready explanation and empirical example. Recovery checks closure and maximum half-space violation before export. Not proof evidence. |

No selected asset contains placeholder trajectory content. The crosspolytope
JSON uses a labelled forward-simulated placeholder because its 16 facets exceed
the producer's orbit-enumeration limit; it is not copied into the thesis.

## Reproduction And Review

The commands, dependency version, numerical checks, projection boundary, and
rejected-asset rationale are maintained in
`experiments/visualization/README.md`. The final review gate is the rendered
figure at thesis width, including line contrast, clipping, caption calibration,
and fit with the surrounding thesis argument.
