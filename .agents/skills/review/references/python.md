# Python Review Checklist

Load `$python-conventions` first. For experiment scripts, also load `$experiment-conventions`.

Check:
- The script is self-contained: reads data, analyzes, writes outputs.
- Paths are relative to `Path(__file__).resolve().parent` or a clearly named repo root.
- `uv run analyze.py` is the intended execution path.
- PEP 723 metadata exists when the script uses non-stdlib dependencies.
- Figures use `experiments/figure_config.py` setup and named size constants.
- `savefig()` does not override dpi or bounding box set by the figure config.
- Math labels use raw strings with `$...$`.
- Generated figure/table captions state observations, not interpretations.

Report missing data or pointer-file blockers as `UNVERIFIABLE`, not as code failures.
