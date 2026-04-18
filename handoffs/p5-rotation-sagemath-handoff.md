# Pentagon Rotation SageMath Handoff

Branch handoff written at the pre-merge checkpoint before the planned SageMath
migration.

## Scope

This branch does not finish the theorem. It packages the current proof surface,
the finite-family branch picture, and the owned experiment/witness surfaces into
a mergeable checkpoint so that a later SageMath-backed session can continue from
current mathematical state instead of reconstructing it.

## Files To Start From

- `research/sys-landscape/design/pentagon-rotation-formula.md`
- `formal/sys-landscape/pentagon-rotation-formula.tex`
- `experiments/sys-landscape/pentagon-rotation-formula/cas_witnesses.py`
- `experiments/sys-landscape/pentagon-rotation-formula/main.rs`
- `experiments/sys-landscape/pentagon-rotation-formula/analyze.py`

## What Is Already Established

- The owned sweep and figures support the conjecture
  `sys(theta) = ((5 + 2 sqrt(5)) / 10) sec^2(theta)` on
  `0 <= theta <= pi/10`, mirrored by `theta -> pi/5 - theta`.
- The draft isolates the active `2`-bounce branch and writes it in a
  setup-to-CAS style.
- Three competitive `3`-bounce families are on the same setup-to-CAS surface:
  - `Q:0-1-23|P:2-3-01`
  - `Q:0-1-34|P:3-4-01`
  - `Q:0-1-3|P:0-2-3`
- The exact witness script checks the symbolic identities for those implemented
  families. It checks identities, not the full theorem.

## What Remains Open

- The global `3`-bounce exclusion is still open.
- The live near-minimum family not yet written as a full lemma is
  `Q:0-2-34|P:2-4-01`.
- The midpoint-only family `Q:0-1-3|P:0-1-3` appears as the contraction target
  of that last near-minimum family, but this is only recorded as a logbook
  observation, not a finished formal lemma.
- The theorem statement in `formal/sys-landscape/pentagon-rotation-formula.tex`
  remains conjectural for exactly that reason.

## Recommended Next Steps After SageMath Lands

1. Port the current `cas_witnesses.py` branch descriptors and reduced formulas
   into the new SageMath surface.
2. Use SageMath to replace the current SymPy-only witness layer for:
   - exact identity checks;
   - interval positivity checks for one-variable gap functions on
     `0 < theta < pi/10`;
   - if useful, certified critical-point or real-algebraic sign checks.
3. Start with the remaining competitive family `Q:0-2-34|P:2-4-01`.
   The current best route is:
   - formalize its endpoint contraction to `Q:0-1-3|P:0-1-3`;
   - decide whether the interior branch needs a full explicit formula or only a
     coarse lower bound above the active `2`-bounce branch.
4. After the near-minimum families are closed, return to the reduction lemma in
   `formal/sys-landscape/pentagon-rotation-formula.tex` and decide whether the
   rest of the open-interval `3`-bounce signatures can be handled by:
   - a coarse template bound, or
   - a finite descriptor loop in SageMath.

## Known Caution About Trust Boundaries

- The current witness script checks exact identities after the geometric/setup
  reduction. It does not by itself certify every interval inequality unless the
  gap expression has already been reduced to an obviously positive form.
- The later SageMath migration is the right place to make the inequality layer
  more explicit and more trustworthy.

## Regeneration Commands

- `cargo run -p exp-sys-landscape --release --bin sys-pentagon-rotation-formula`
- `cargo run -p exp-sys-landscape --release --bin sys-pentagon-rotation-formula -- --three-bounce-branches`
- `cd experiments/sys-landscape/pentagon-rotation-formula && uv run analyze.py`
- `uv run experiments/sys-landscape/pentagon-rotation-formula/cas_witnesses.py`
- `cd formal && latexmk -pdf -interaction=nonstopmode main.tex`
