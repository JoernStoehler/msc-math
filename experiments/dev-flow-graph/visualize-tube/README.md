# Exact Tube Visualization

Question: what does one exact closed flow-graph tube and orbit look like in its
ordered local two-face sections and in global projections?

`main.rs` generates a deterministic polytope and closed word, performs
incidence, sign, tube, fixed-point, and orbit decisions with exact rational
arithmetic, and emits JSON. Finite floats enter at JSON serialization and
plotting. `render.py` consumes only the explicitly named plotting-coordinate
fields for local panels; exact construction-chart vertices and inequalities
remain in the JSON for provenance and are not interchangeable with those
coordinates.

The retained thesis example is `F=6`, master seed `20260605`, attempt `3`, and
sigma `1,2,4,5,3`. Its packet consists of:

- `flow-graph-f6-tube.json`;
- `flow-graph-f6-tube-sequence.pdf`;
- `flow-graph-f6-projection.pdf`.

Regenerate it from the repository root:

```bash
cargo run -p exp-dev-flow-graph --release \
  --bin flow-graph-visualize-tube-data -- \
  --facet-count 6 --attempt 3 --sigma 1,2,4,5,3 \
  --output experiments/dev-flow-graph/visualize-tube/flow-graph-f6-tube.json
uv run --script experiments/dev-flow-graph/visualize-tube/render.py \
  --layout sequence \
  --input experiments/dev-flow-graph/visualize-tube/flow-graph-f6-tube.json \
  --output experiments/dev-flow-graph/visualize-tube/flow-graph-f6-tube-sequence.pdf
uv run --script experiments/dev-flow-graph/visualize-tube/render.py \
  --layout projection \
  --input experiments/dev-flow-graph/visualize-tube/flow-graph-f6-tube.json \
  --output experiments/dev-flow-graph/visualize-tube/flow-graph-f6-projection.pdf
```

Ordered local frames make `F_i cap F_j` and `F_j cap F_i` distinct panels when
both occur. Sequence layout uses tube-focused crops and a shared frame for the
start/return comparison. Projection layout uses radial and stereographic
views, so it intentionally distorts global geometry.

These are active explanatory assets, not proof or numerical validation.
Changes to the exact tube snapshot schema, fixture/word, plotting-coordinate
construction, or renderer require regenerating and visually reviewing all
three retained artifacts.
