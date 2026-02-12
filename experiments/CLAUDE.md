# Experiments

Python scripts for exploring Viterbo's conjecture through computational data science.

## Philosophy

Experiments are **always investigative**, never "stable" or "finished".

### Continuous spectra, no discrete stages

Progression is fluid, with no clear cutoff points:

- From [nothing] → [idea] → [plan with preliminary findings] → [active hypotheses with mysteries] → [findings from non-runnable code] → [failed attempt summary] → [cleanup commit in git log]
- From [nothing] → [full bundle: scripts + thesis section + datasets + extra rust code]

### What agents do constantly

- **Comment on and iterate** experiments — tweak parameters, try variations, explore edge cases
- **Clean, refactor, narrow** experiments — simplify code, focus scope, remove dead ends

### Cleanup and archiving (continuous spectrum)

No clear cutoff for "when to archive". It's continuous prioritization:
- Blockers: lack of ideas for improvements
- When cleaning up code that's no longer useful:
  - If learnings worth preserving: create `experiments/<topic>.md` with git ref to last commit
  - Otherwise: just delete (it's in git history)
- Purpose: keep `scripts/` focused, don't distract with low-priority work

## Directory structure

```
experiments/
  CLAUDE.md              This file
  IDEAS.md               Ongoing thoughts, ideas, edge cases, preliminary findings
  <topic>.md             Standalone writeups (investigation findings, learnings)
  scripts/               Experiment scripts (all investigative)
    <name>.py            Script
    <name>.md            Colocated writeup (findings, methodology, key results)
    <name>_<suffix>.py   Multi-script experiment variants
  profiling/             Profiling raw data
  data/                  Generated datasets (gitignored)
  figures/               Generated plots (gitignored)
```

## Script conventions

**File naming:**
- Scripts: `scripts/<name>.py`
- Colocated writeup: `scripts/<name>.md` (findings, methodology, key results)
- Multi-script experiments: `scripts/<name>_<suffix>.py` (e.g., `timing_model_fit.py`, `timing_model_plot.py`)
- Standalone writeups (no script): `experiments/<topic>.md` (e.g., investigation findings, learnings)

**Independent scripts, not a package:**
- No `__init__.py`, no shared imports between scripts
- Each script is self-contained: reads data, performs analysis, writes output
- If two scripts share logic, copy-paste until it stabilizes (don't prematurely abstract)

**No framework:**
- Use plain Python with standard data science libraries (numpy, pandas, matplotlib, scipy)
- No custom framework, no complex dependencies
- Dependencies listed in `experiments/requirements.txt`; install with `pip install -r experiments/requirements.txt`

**Script headers:**
Every script must document in the docstring:
- **Goal**: What question does this answer?
- **Input**: What data does it read?
- **Output**: What files does it write?

Example:
```python
#!/usr/bin/env python3
"""
Analyze systolic ratios across polytope dataset.

Goal: Identify distribution of sys values, locate counterexamples
Input: experiments/data/polytopes.jsonl
Output: experiments/figures/sys_histogram.png
"""
```

**Path conventions:**
```python
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent.parent
DATA_DIR = REPO_ROOT / "experiments" / "data"
FIGURES_DIR = REPO_ROOT / "experiments" / "figures"
```

No hardcoded paths outside `REPO_ROOT`.

**Error messages:**
Make them actionable. Bad: "File not found". Good: "File not found: data/polytopes.jsonl. Run run_dataset.py first."

## Pipeline direction

Rust → datasets → Python → figures/tables → thesis

**Data flow:**
1. Rust crates generate JSONL datasets → `experiments/data/`
2. Python scripts load JSONL, compute statistics, generate figures
3. Figures and tables copied into `thesis/figures/`
4. LaTeX includes figures

**No circular dependencies:**
- Python never calls Rust directly
- Use `run_dataset.py` as orchestrator (builds Rust binary, runs it, loads results)
- If Rust API changes, only `run_dataset.py` needs updates

## Quality standards

**Rerunnable from zero:**
- Starting from empty `data/` and `figures/`, running all scripts should reproduce all outputs
- No manual steps
- No "run this once, then comment it out"

**Document assumptions:**
- If script assumes file exists, document it in header and error message
- Example: "Assumes data/polytopes.jsonl exists. Run run_dataset.py first."

**Verification:**
- Results checked by Jörn before inclusion in thesis
- Plots visually inspected for sanity
- Statistical claims require reproducible computation
- Agent-generated figures are drafts until Jörn reviews

**Not production code:**
- No exhaustive testing required (not like Rust crates)
- But must be reproducible
- Focus on clarity and correctness over performance
