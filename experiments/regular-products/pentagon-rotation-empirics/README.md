# Pentagon Rotation Empirics

This folder owns sampled data, static figures, and the interactive viewer for

```text
P_5 x_L R(theta)P_5.
```

These artifacts are thesis-support and orientation outputs. They are not proof
inputs. The exact proof lives in the sibling folder
`../pentagon-rotation-formula-proof/`.

Local filenames below are relative to this folder. Paths outside this folder
are repo-root relative unless they begin with `../`.

Read this file if you need figures, sampled motivation, or regeneration
commands.

Do not read this folder for theorem verification. For proof status, go to
`../pentagon-rotation-formula-proof/README.md`.

Do not open generated JSONL, PNG, or HTML files by default. Use the figure
recommendations in `thesis/rotated-regular-polygons-content.md` first, then
open only the specific artifact you need.

## Source Files

```text
main.rs
```

Rust producer for sampled theta sweeps and optional 3-bounce branch datasets.
It is wired into `experiments/regular-products/Cargo.toml` as
`regular-pentagon-rotation-empirics`.

```text
analyze.py
```

Python analyzer for text summaries and static figures.

```text
build_interactive_orbit_viewer.py
```

Python producer for the standalone HTML orbit-projection viewer.

## Generated Data

```text
theta-sweep.jsonl
minimum_orbit_projection_dataset.jsonl
```

`theta-sweep.jsonl` is the sampled minima sweep. The viewer dataset is derived
from it and committed next to the viewer. These are generated artifacts, not
proof inputs.

## Figures And Viewer

```text
labeled_pentagons_theta.png
trajectory_projections_theta14.png
trajectory_projections_theta14_affine.png
three_bounce_branch_actions.png
signature_state_table_full.png
signature_state_table_competitive.png
signature_legend.txt
minimum_orbit_projection_viewer.html
```

Use these as thesis illustrations only. The exact proof does not depend on
them.

## Regeneration Commands

Sampled minima sweep:

```bash
cargo run -p exp-regular-products --release --bin regular-pentagon-rotation-empirics -- --canonical
```

Static analysis figures:

```bash
uv run --script experiments/regular-products/pentagon-rotation-empirics/analyze.py
```

Interactive orbit viewer:

```bash
uv run --script experiments/regular-products/pentagon-rotation-empirics/build_interactive_orbit_viewer.py
```
