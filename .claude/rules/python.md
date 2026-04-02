---
paths:
  - "**/*.py"
---

# Python Conventions

## Script structure

Scripts are self-contained: read data → analyze → write output. No `__init__.py`, no shared imports between scripts — except `figure_config.py`.

Shared figure config lives at `crates/figure_config.py`. Import it:
```python
sys.path.insert(0, str(Path(__file__).resolve().parent.parent.parent))
from figure_config import setup, FIGSIZE_SINGLE
setup()
```

## Script headers

Docstring with Goal / Input / Output:
```python
"""
Goal: Identify distribution of sys values
Input: crates/exp-<group>/<subdir>/data.jsonl
Output: crates/exp-<group>/<subdir>/histogram.png
"""
```

## Paths

```python
EXPERIMENT_DIR = Path(__file__).resolve().parent
```
Scripts live at `crates/exp-<group>/<subdir>/analyze.py`. No hardcoded absolute paths. Define `REPO_ROOT = Path(__file__).resolve().parent.parent.parent.parent` only if referencing paths outside the experiment directory.

## Figures

- Use named size constants from `figure_config.py` (`FIGSIZE_SINGLE`, `FIGSIZE_DUAL`, etc.) — never hardcode figsize. See `figure_config.py` for the full list.
- `setup()` sets rcParams globally (fonts, dpi, bbox). Don't pass `dpi=` or `bbox_inches=` to `savefig()`
- `\textwidth` ≈ 5.4" in the thesis. Multi-panel at 5.4" is often too cramped — prefer separate figures
- `r"$...$"` for all math in labels. LaTeX syntax outside `$...$` renders as literal text
- Consistent colors for same data categories across figures in an experiment
- `fontsize=` above 14pt signals figsize mismatch

## Captions (in .tex, informed by Python)

Captions state observations. Interpretations belong in body text.
Detection: "suggests", "indicates", "because", "implies", "due to" in a caption → move to body.