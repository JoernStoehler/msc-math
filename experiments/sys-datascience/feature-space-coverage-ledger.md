# Sys-Datascience Feature-Space Coverage Ledger

Purpose: track which method-facing feature families are active under the
invariant-only random/product `sys` datascience contract, and which obvious
families are deliberately excluded.

Current status: this ledger was reset after the active table moved from
volume-one representative features to invariant-only features. Historical
coverage rows for raw Euclidean geometry, all-pair omega magnitudes,
omega-matrix spectra, sign graphs, and transition graphs are stale under the
current schema and should be recovered from git history only for archaeology.

## Active Families

### Combinatorial Invariants

- source: `experiments/polytope-invariant-table/invariant_features.rs` with skeleton helpers from
  `experiments/polytope-invariant-table/features_skeleton.rs`;
- columns: facet/vertex/edge/ridge counts, simplicity, incidence summaries,
  ridge-size summaries, facet-vertex summaries, facet-neighbor summaries, and
  edge density;
- invariance status: exact face-lattice invariants, hence invariant under
  `Sp(4) x R_+ x R^4 x Perm(F)` as geometric polytope invariants;
- active consumers: all trusted random/product method packets through
  `methods/_shared/random_only.py`.

### Symplectic Two-Face Area Invariants

- source: `experiments/polytope-invariant-table/features_face_symplectic.rs` and
  `experiments/polytope-invariant-table/invariant_features.rs`;
- columns: ordered-face diagnostics plus summary statistics for cyclically
  ordered primal two-face symplectic areas divided by `volume.sqrt()`, and
  dimensionless max/top-3 shares;
- invariance status: translation-invariant by closed-polygon telescoping,
  `Sp(4)`-invariant by preservation of `omega_0`, scale-invariant after
  division by `volume.sqrt()`, and facet-order invariant after summary
  aggregation;
- caveat: ridge area columns are excluded by the shared selector when any row
  reports incomplete two-face ordering.

## Active Metadata / Controls

- `poly_id`, `sys`, and `capacity_source` remain in `polytope-table.jsonl`.
- `capacity_source`, `facet_count`, product bucket, product bounce count, and
  sample height range are used as categorical controls in method packets.
- Metadata controls are not numeric geometry covariates. They test whether
  invariant-feature signals exceed source/stratum/provenance baselines.

## Excluded Families

These are intentionally not active method-facing covariates:

- raw dual vertices and Euclidean representative features;
- `capacity` and `volume`;
- raw all-pair omega entries or magnitudes of normalized dual rows;
- omega sign/cutoff features without a tolerant zero policy and boundary-hit
  diagnostics;
- transition-graph features tied to the deleted pre-invariant omega helpers.

Reopen an excluded family only by first writing its mathematical invariance
contract and extending `experiments/polytope-invariant-table/invariant_feature_check.rs` with a stochastic
symmetry test for the new columns.
