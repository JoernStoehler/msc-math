# HKO explanatory toy figures

Generated 2026-09-14 by `make_figures.py`:

Reproduce with `sage -python .git/codex/hko-figures/make_figures.py` from the
repository root (the system Python does not provide NumPy/Matplotlib).

- `upper-bound-mechanisms.pdf` / `.png`: one-dimensional `min(1+x,1-x)`
  envelope and a three-plane two-dimensional analogue.
- `symmetry-extension.pdf` / `.png`: `f(u,v)=1-|u|`, constant along the
  symmetry ridge and strictly decreasing transversely.
- `captions.tex`: proposed reader-facing captions.

The gradients in the two-dimensional panel are
`(1,0)`, `(-1/2,sqrt(3)/2)`, and `(-1/2,-sqrt(3)/2)`; they sum to zero and
their positive convex relation is equal weighting. For any nonzero direction,
not all three dot products vanish and their positive sum is zero, so one is
negative. The one-dimensional and symmetry examples have maxima at the origin
(or ridge `u=0`) with value 1. These are illustrative upper-bound toy
functions, not plots or equalities of the actual systolic ratio.

The HKO proof retains `T` as the 15-dimensional symmetry tangent space and
`S` as the 25-dimensional transverse complement; the figures deliberately use
generic `x`, `y`, `u`, and `v` labels.

The symmetry plot draws the exact ridge at `z=1` and the exact V-shaped slice
at `z=1-|u|`; explicit Matplotlib drawing order and a translucent surface keep
these guides visible without changing their mathematical values.
