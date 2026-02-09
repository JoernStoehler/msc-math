---
name: experiments
description: Use when writing Python scripts in experiments/. Contains script conventions, data flow, and library choices.
---

# Python experiments conventions

- Independent scripts, not a package — no `__init__.py`, no shared imports
- Each script is self-contained: reads data, does analysis, writes output
- No framework — plain Python with standard data science libs (numpy, pandas, matplotlib)
- If two scripts share logic, copy-paste until it stabilizes
