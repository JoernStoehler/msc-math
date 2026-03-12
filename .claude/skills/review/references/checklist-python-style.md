# Review Checklist: Python Script Style (Phase 1)

Detection rules for Python scripts in `experiments/`.

## 1. Script Headers

Every `.py` script must have a docstring documenting:
- **Goal**: What question does this answer?
- **Input**: What data does it read?
- **Output**: What files does it write?

## 2. Path Conventions

```python
REPO_ROOT = Path(__file__).resolve().parent.parent.parent
EXPERIMENT_DIR = Path(__file__).resolve().parent
```
- No hardcoded paths outside `REPO_ROOT`.
- Detection: grep for string literals containing `/workspaces/`, `/home/`, or absolute paths.

## 3. Error Messages

- Must be actionable.
- Detection: grep for `raise`, `sys.exit`, `print.*error` — check if the message tells the user what to do.
- Bad: "File not found". Good: "File not found: data.jsonl. Run: cargo run --bin experiment --release"

## 4. Independence

- No `__init__.py`, no shared imports between experiment scripts.
- Each script is self-contained.
- Detection: grep for `from experiments` or `import experiments`.

## 5. Dependencies

- Only standard data science libraries (numpy, pandas, matplotlib, scipy).
- No custom framework.
- Detection: check imports against `experiments/requirements.txt`.

## 6. Figure Sizing

- `figsize` width must be <= 5.4" (since `bbox_inches='tight'` expands beyond figsize).
- Detection: grep for `figsize=` — flag if width component >= 5.4.
- `fontsize=` values above 14pt on individual elements flag a figsize mismatch.
- Multi-panel figures at 5.4" are often too cramped — flag and suggest separate figures.

## 7. DPI

- `savefig(dpi=150)` minimum for print quality.
- Detection: grep for `savefig` — check dpi parameter exists and is >= 150.
- If no dpi parameter, flag (matplotlib default is 100, too low for print).

## 8. bbox_inches

- `bbox_inches='tight'` should be used in `savefig()` to avoid clipping labels.
- Detection: grep for `savefig` without `bbox_inches`.

## 9. Visual Clarity

- **Markers**: scatter/line plots should use markers (not just color) for grayscale compatibility.
- Detection: grep for `plt.plot` or `ax.plot` — check if `marker=` is specified for line plots.
- **Colorblind-friendly**: avoid red-green only distinctions.
- Detection: check color specifications for red/green pairs without alternative markers.
- **Consistent colors**: same data categories use same colors across all figures in the experiment.
- **Axis labels**: must include quantity name (not just symbol) or be self-evident.
- Detection: grep for `set_xlabel`, `set_ylabel`, `xlabel`, `ylabel` — check content.

## 10. Multi-Panel Figures

- Consistent axis scales where cross-panel comparison is intended.
- Detection: check `subplot` figures for matching `set_xlim`/`set_ylim` across panels.

## 11. Caption Epistemology (in .tex files referencing figures)

- **Observations** (what the figure shows) and **comparisons** (relating to explicit reference) go in captions.
- **Interpretations** (speculation, analysis) belong in body text, NOT captions.
- Detection: grep captions for "suggests", "indicates", "means that", "because", "implies", "consistent with", "due to" — each is a potential violation.
- Comparisons require an explicit target ("than general polytopes", "relative to the diagonal").

## 12. Visual Inspection

Read each `.png` file and check:
- Labels are readable (not clipped, not overlapping).
- Legend is present and positioned sensibly.
- Figure serves a clear purpose (hypothesis education OR data immersion).
