# Review Checklist: Python Figure Quality

Visual quality checks for experiment PNGs before they go into the thesis.
Figures render at thesis text width (5.4").

## Per-figure checks

- **Title collisions**: Do subplot titles collide with each other? (Common at 1.8" panel width in 1x3 layouts, 2.7" in 1x2)
- **Label clipping**: Does any text extend beyond the figure border?
- **Font readability**: Can all text (titles, labels, ticks, legend) be read at 5.4" width?
- **Legend placement**: Does the legend overlap data? Is it in a clear region?
- **Layout balance**: Any panel disproportionately small? (`set_aspect("equal")` in multi-panel layouts crushes panels)
- **Math rendering**: Do axis labels with math notation render as proper subscripts/superscripts, or as raw text like `_{n_k}`? Labels using LaTeX syntax must be wrapped in `r"$...$"`.

## Common failure patterns

- `set_aspect("equal")` in 1x3 subplot → middle panel becomes tiny
- Subplot titles > ~25 characters → collide at multi-panel widths
- `suptitle` + `tight_layout(rect=...)` → awkward gap between title and panels. Use `suptitle(y=1.02)` + plain `tight_layout()`
- Duplicated legends in both panels → wasted space. Use `fig.legend()` with shared handles
- Unicode LaTeX syntax (`∇_{n_k}`) outside `$...$` → renders as literal text with braces
- Long x-axis labels on rightmost panel → clipped by figure edge
