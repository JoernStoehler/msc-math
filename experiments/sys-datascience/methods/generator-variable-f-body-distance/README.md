# Variable-facet body distances

This target-free packet supplies a body-level comparison for irredundant four-dimensional polytopes whose facet counts may differ. It complements, rather than replaces, the exact fixed-facet symplectic-Gram view: a sampled support comparison can see a facet birth/death, while it neither detects symplectic equivalence nor retains combinatorial information.

## Contract

The executable accepts only normalized dual presentations `a_i . x <= 1` in `(q1,q2,p1,p2)` order. It reconstructs all vertices by exact rational four-facet joins, checks every join against every inequality, and rejects a presentation with a duplicate or any inequality that does not span a three-dimensional reconstructed facet. The redundant-cube calibration includes both a duplicate and a supporting plane that touches only one vertex, each with an explicit `fail_closed` disposition; a merely permuted irredundant cube is invariant.

The convention subtracts the arithmetic mean of reconstructed vertices and divides by RMS vertex radius. It quotients translation and positive global scale only. Support is evaluated on an antipodally complete finite `S^3` design: normalized primitive integer directions with maximum coordinate 3. `direct_sampled` reports support `L_infinity` (a sampled Hausdorff surrogate, not exact Hausdorff) and `L2`.

`u2_finite_bank` minimizes over 32 monomial unitary maps, and `so4_finite_bank` over 192 orientation-preserving signed permutations. These are finite-bank approximations, not U(2)/SO(4) infima. Coarse-to-fine direction residuals and coarse-to-fine finite-bank search residuals are reported separately. O(4) reflection is omitted because this packet does not declare reflection semantics. Sp(4) is noncompact, so no fictional Haar sampling is attempted.

The calibration includes retained exact orientation rows, target-free exact-feature geometry, a cube versus a truncated cube, coarse ball/ellipsoid polytope approximants, and a narrow corner cut. The narrow cut's axis-only sampled `L_infinity` is at most 51% of the primitive-panel value, illustrating direction-grid miss risk without falsely calling the coarse result zero after RMS normalization. The local bounded fixed-F direct view reproduces the six-facet control: deterministic U(2) is zero for the symplectic-Gram quotient while deterministic SO(4) is positive.

## Run and evidence

```bash
uv run --script experiments/sys-datascience/methods/generator-variable-f-body-distance/test_body_distance.py

# After committing source changes, to make the provenance clean-tree guard meaningful:
uv run --script experiments/sys-datascience/methods/generator-variable-f-body-distance/body_distance.py \
  --require-tracked-clean \
  --out-dir experiments/sys-datascience/methods/generator-variable-f-body-distance/artifacts
```

`artifacts/report.json` binds exact input hashes, producer hash, revision, pre-generation tracked-clean predicate, reconstruction counts, finite designs, and target-free status. It also records observed exact-reconstruction and finite-bank-search costs. `artifacts/calibration.tsv` is the compact readable table. Wall seconds are retained only as observations; the replay check strips them before requiring equal scientific values.

The method can answer small-panel cross-facet Euclidean body-shape questions under its declared quotient. It cannot prove a population effect, establish a continuous compact-group quotient, recover exact Hausdorff distance, identify symplectic equivalence, or compare `sys` (which it never reads or evaluates). Normalized surface-area-measure transport and a directed symplectic containment gauge are explicitly deferred; the latter would remain only a dissimilarity pending metric and computation facts.
