# Triangle-position exact replay

Run with a Python interpreter from a SageMath environment:

```sh
/workspaces/msc-math/.local-environments/miniforge/envs/sage/bin/python exact_triangles_compact.py
```

The script reconstructs all 120 cyclic six-facet words, the four action-coefficient groups, and the rational PPL rank-one hull used in `../../sections/triangle-position.tex`. It writes `exact-triangles-compact-summary.json` beside the script. The global determinant bound uses the analytic Cauchy--Binet argument in the paper. The scoped independent audit is `../../reviews/triangle-position-mathematics.md`.
