# Error-Bounds Plan

Scope: `experiments/numerics/error-bounds`.

Current aim:
1. Keep `num-collect-poly` → `num-error-bounds` → `analyze.py` as the default validation loop for abstract KKT subproblems.
2. Preserve the trinary outcome model (TRUE / FALSE / INDETERMINATE) for continuity-safe solver behavior.
3. Keep Q bounds tied to proven structure:  
   `E = ||H|| · ||β̃|| · ||r|| / σ_min(C)`, plus fallback to exact arithmetic only when needed.
4. Ensure the projection/saddle-point comparison is localized in this crate and reproducible via committed fixtures in `error-bounds/testdata/`.

Current open items documented in design notes:
- false negatives on boundary-like cases are in scope for diagnostics, but capacity-critical logic assumes interior `β>0` cases are the acceptance gate.
- singular/ill-conditioned cases remain the dominant numerical stressor.

Execution rules:
- default runs write smoke outputs; canonical refresh is explicit (`--canonical` where supported by the packet command surface).
- update this doc when packet boundaries or artifact contracts change, not for one-off result notes.
