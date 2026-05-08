---
name: python-conventions
description: Use when Codex writes, edits, reviews, or delegates Python work in this repo, especially experiment orchestration scripts, analysis scripts, figure generation, smoke paths, or Python code that manipulates experiment artifacts.
---

# Python Conventions

## Conventions

- similar to Rust: write plainly, avoid abstraction, be predictable, and so on
- we mostly script/orchestrate with rust, so imperative style and little typing is fine
- stick to a "data science" style for rapid development
- use `Path(__file__).resolve().parent` for paths relative to the script
- use `experiments/figure_config.py` for figure styling when relevant
- figure captions should state observations before interpretation
