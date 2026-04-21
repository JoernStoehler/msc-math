# HKO Local Maximum

This topic directory mixes theorem-facing exact work and supporting evidence.

Start with `research/hko-local-maximum-status.md` before reading subfolders.

## Directory Roles

- `exact-clarke/`
  The intended theorem route for the `M_10` result. This is where the exact
  witness contract, exact artifacts, and independent Sage verification live.
- `gradient-analysis/`
  First-order numerical support and gradient/orbit bookkeeping.
- `second-order/`
  Older flat-direction curvature evidence. Keep this as supporting evidence,
  not as the preferred final theorem route.
- `perturbation-neighborhood/`
  Random local perturbation evidence in the fixed `F=10` neighborhood.
- `facet-splitting/`
  `F=10 -> 11` ambient-space falsification attempts.
- `cut-and-ascent/`
  Cut-then-ascent falsification attempts beyond the fixed `F=10` cell.
- `lagrangian-boundary/`
  Local `sys > 1` neighborhood geometry in the Lagrangian-product parameter
  surface.
- `sage-validation/`
  Sage cross-checks for existing exact row-bank artifacts; not the final
  theorem certificate by itself.
- `subdifferential-lp/`
  Historical or inactive route from the older `(n,h)` parameterization. Read
  only when reconstructing provenance.
- `src/`
  Topic-local shared Rust helpers.

## Fast Reading Order

1. `research/hko-local-maximum-status.md`
2. `research/hko-local-maximum.md`
3. `research/hko-local-maximum-exact-clarke.md`
4. `exact-clarke/`
5. supporting-evidence directories as needed

## Rule Of Thumb

If a question is "what proves the theorem?", start in `exact-clarke/`.

If a question is "what evidence supports the local-maximality story?", read the
other experiment folders after the status note.
