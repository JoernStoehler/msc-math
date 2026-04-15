# Figure Review Checklist

Load `$python-conventions` for Python-generated figures and `$thesis-tex-conventions` for thesis inclusions.

Check the production chain:
- Data source exists and is current relative to the script when the task depends on freshness.
- Python script uses shared figure configuration and named size constants.
- Colors, line styles, and labels are consistent for the same category inside the experiment.
- Axis labels include quantities and units when not self-evident.
- Legends and labels are legible at thesis text width.
- Rendered PNGs are not clipped and do not have overlapping labels.
- LaTeX inclusion is pass-through with no width or scale.
- Captions state observations, not interpretations.

When image inspection is required, open the PNG or use available image tooling. Do not infer readability from script code alone.
