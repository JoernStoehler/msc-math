---
name: python-conventions
description: Python script conventions for experiments. Load when writing or editing .py files under experiments/. Covers script headers, path conventions, error messages, figure sizing, DPI, visual clarity, and caption rules.
---

# Python Conventions

## Script Conventions

**Independent scripts with shared figure config:**
- No `__init__.py`, no shared imports between scripts — except `figure_config.py`
- Each script is self-contained: reads data, performs analysis, writes output
- If two scripts share logic, copy-paste until it stabilizes
- Exception: `experiments/figure_config.py` provides shared figure styling (fonts, sizes, rcParams). All scripts import it:
  ```python
  sys.path.insert(0, str(Path(__file__).resolve().parent.parent))
  from figure_config import setup, FIGSIZE_SINGLE  # import what you need
  setup()
  ```

**No framework:** Use plain Python with standard data science libraries (numpy, pandas, matplotlib, scipy). No custom framework. Dependencies in `experiments/requirements.txt`.

**Script headers:** Every script must document in the docstring:
- **Goal**: What question does this answer?
- **Input**: What data does it read?
- **Output**: What files does it write?

Example:
```python
#!/usr/bin/env python3
"""
Analyze systolic ratios across polytope dataset.

Goal: Identify distribution of sys values, locate counterexamples
Input: experiments/<name>/data.jsonl
Output: experiments/<name>/histogram.png
"""
```

**Path conventions:**
```python
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent.parent
EXPERIMENT_DIR = Path(__file__).resolve().parent  # data/figures are colocated
```

No hardcoded paths outside `REPO_ROOT`.

**Error messages:** Make them actionable. Bad: "File not found". Good: "File not found: data.jsonl. Run Rust binary first."

## Figures and Tables

All figure formatting is handled in Python. LaTeX is a 1:1 pass-through (`\includegraphics{file.png}`, no `width=`/`scale=`).

**Sizing and fonts (via `figure_config.py`):**
- `setup()` configures rcParams: font family (CM serif), sizes, grid, dpi, bbox. Call it once at module level.
- Use `FIGSIZE_SINGLE`, `FIGSIZE_DUAL`, `FIGSIZE_TRIPLE`, etc. from the config — don't hardcode figsize.
- `figsize` = the physical size in the printed PDF. `\textwidth` ≈ 5.4" (A4, 12pt article, default margins).
- Multi-panel figures at 5.4" are often too cramped. Prefer separate figures over wider canvases.
- Multi-panel figures: use consistent axis scales where cross-panel comparison is intended.
- Long titles on subplots will collide. Use `\n` to wrap, or shorten.

**Axis labels with math:** Use `r"$...$"` for all mathematical notation in labels. Never use LaTeX syntax (`_{n_k}`, `^{2}`) outside of `$...$` — matplotlib renders it as literal text.

**Visual clarity:**
- Use markers (not just color) for grayscale compatibility in scatter/line plots.
- Avoid red-green only distinctions; use colorblind-friendly palettes.
- Consistent colors for the same data categories across all figures in the same experiment.
- Axis labels must include the quantity name (not just the symbol), or be self-evident from context.

**Captions (in .tex, but Python generates the figure):**
- Captions state observations and comparisons (relating to an explicit reference).
- Interpretations and speculation belong in body text, NOT in captions.
