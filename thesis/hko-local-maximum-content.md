# HKO Local Maximum Content Notes

Status: thesis-local content-gathering notes, not source truth.

Purpose: gather the HKO result packet, source pointers, evidence status, and
missing non-writing work needed to draft `thesis/hko-local-maximum.tex`.

Overruled by: `FACTSHEET.md`, `research/hko-local-maximum*.md`, exact
artifacts in `experiments/hko-local-maximum/`, formal proof files, task files,
and Jörn/Kai review.

Lifecycle: keep while the HKO thesis section is being assembled. After the
section is stable, either delete this file or reduce it to a short maintenance
index. Do not cite this file as evidence in thesis prose.

Update rule: add or change a claim only with a source pointer or an explicit
`needs source` marker.

## Result Packet

The HKO thesis-facing result is a packet, not only one theorem statement.

1. Known result:
   HKO2024 is a counterexample with `sys > 1`.
   Source pointers: `FACTSHEET.md` item 8.1; `thesis/bibliography.bib`;
   `research/hko-local-maximum-status.md`.

2. Main theorem target:
   HKO is locally maximal in the ten-facet model modulo translations, scaling,
   and linear symplectic maps, if the exact certificate closes.
   Source pointers: `research/hko-local-maximum-status.md`;
   `research/hko-local-maximum.md`;
   `tasks/references/central-claim-control-packet-2026-06-01.md`;
   `thesis/hko-local-maximum.tex`.

3. Exact certificate:
   Code generates a witness, Sage verifies it, and the mathematical writeup
   proves that the verified witness implies the theorem target.
   Source pointers: `FACTSHEET.md` items 20--22;
   `research/hko-local-maximum-exact-clarke.md`;
   `experiments/hko-local-maximum/exact-clarke/README.md`;
   `experiments/hko-local-maximum/exact-clarke/widened-seed-witness.json`;
   `experiments/hko-local-maximum/exact-clarke/widened-seed-witness-verification.json`.

4. Empirical confirmation:
   Numerical and perturbative checks support the local-maximum picture.
   Source pointers: `research/hko-local-maximum.md`;
   `experiments/hko-local-maximum/README.md`;
   `experiments/hko-local-maximum/gradient-analysis/`;
   `experiments/hko-local-maximum/second-order/`;
   `experiments/hko-local-maximum/perturbation-neighborhood/`;
   `experiments/hko-local-maximum/facet-splitting/`;
   `experiments/hko-local-maximum/cut-and-ascent/`;
   `experiments/hko-local-maximum/lagrangian-boundary/`.

5. Broader conjectural interpretation:
   HKO appears locally stable beyond the exact theorem model, but this broader
   interpretation is not proved by the current theorem target.
   Source pointers: `research/hko-local-maximum-status.md`;
   `research/hko-local-maximum.md`;
   `CAPABILITY_CLAIM_MAP.md` HKO local-maximum evidence-route row.

6. Search/context result:
   Broader search work did not find a better local escape or general
   explanation.
   Source pointers: `research/sys-landscape.md`;
   `research/sys-landscape-toolbox-audit.md`;
   `experiments/sys-landscape/`;
   `tasks/references/central-claim-control-packet-2026-06-01.md`.

Guard: do not collapse items 3--6 into item 2. They are thesis-facing result
components with different claim strengths.

## Evidence By Packet Component

### 1. Known Result

Status:
- Retained thesis content area: `FACTSHEET.md` item 8.1.
- HKO paper bibliography entry: `thesis/bibliography.bib`.

Missing:
- Thesis prose still needs final exact wording and citation placement.

### 2. Main Theorem Target

Status:
- Target theorem wording is local maximality on `M_10` modulo translations,
  scaling, and linear symplectic maps.
- Current repo classification: broad HKO local maximality remains conjectural;
  exact first-order certificate is not yet closed.

Source pointers:
- `research/hko-local-maximum-status.md`, sections "Current Repo Claim" and
  "Current Proof Route".
- `research/hko-local-maximum.md`, sections "Current State", "Decisions", and
  "Next Steps".
- `research/hko-local-maximum-proof-route-note.md` for the 2026-06-03
  selected-branch proof checkpoint and singular-KKT obstruction.
- `tasks/current-state.md`, HKO local maximality row.

Missing:
- Final exact active-gradient certificate.
- Jörn/Kai acceptance of theorem-strength wording.

### 3. Exact Certificate

Status:
- Packet 1 is closed in the current research notes: exact dual coordinates,
  coefficient field, and rank-15 symmetry tangent setup.
- Packet 2 is partially closed with exact prototypes.
- Packet 3 remains the main blocker.
- Current widened representative-row witness passes present Sage checks but is
  partial.

Current checked partial witness facts:
- Exact field: `Q(tan(pi/5))`, minimal polynomial `t^4 - 10 t^2 + 5`.
- Endpoint representative rows: 5 rows, rank 5.
- Midpoint representative rows: 6 rows, rank 6.
- Current widened representative-row union: 11 rows, rank 11.
- Current right kernel dimension: 29.
- Current rows annihilate the 15 symmetry tangent directions.

Source pointers:
- `research/hko-local-maximum-exact-clarke.md`, sections "Status",
  "Scope Boundary", "Sage Note", and "Packet 2 Note".
- `research/hko-local-maximum-proof-route-note.md`, especially the one-sided
  branch-certificate principle and candidate Sage decision problem.
- `experiments/hko-local-maximum/exact-clarke/widened-seed-witness.json`.
- `experiments/hko-local-maximum/exact-clarke/widened-seed-witness-verification.json`.

Missing:
- Final theorem-facing active-gradient matrix.
- Rank target `rank(G)=25`.
- Kernel target `dim ker(G)=15`.
- Equality of kernel with the symmetry tangent space.
- Cone or convex-combination certificate proving no increasing quotient
  direction.
- Completeness check for the active row set used by the theorem-facing
  decision problem.
- Exact treatment of remaining asymmetric seven-facet representatives.

### 4. Empirical Confirmation

Status:
- `gradient-analysis/` gives first-order numerical support and active-gradient
  bookkeeping.
- `second-order/` gives fixed-`F=10` support, including current notes reporting
  `rank(G)=25`, `dim ker(G)=15`, and negative curvature evidence along sampled
  flat directions.
- `perturbation-neighborhood/`, `facet-splitting/`, `cut-and-ascent/`, and
  `lagrangian-boundary/` are supporting falsification/neighborhood checks.

Source pointers:
- `research/hko-local-maximum.md`, "Current State" and "Supporting Evidence".
- `experiments/hko-local-maximum/README.md`, "Directory Roles".
- `formal/hko-local-maximality-conditions.tex` for older second-order formal
  support and caveats.

Missing:
- Decide which empirical checks are retained in the thesis section.
- For retained checks, refresh exact command/data provenance from each
  experiment directory before writing final prose.

### 5. Broader Conjectural Interpretation

Status:
- Current repo supports a strict local-maximum hypothesis in tested finite and
  smooth neighborhoods.
- Broad HKO local maximality remains conjectural in current repo
  classification.

Source pointers:
- `research/hko-local-maximum-status.md`, "Current Repo Claim".
- `research/hko-local-maximum.md`, "Evidence And Interpretation".
- `CAPABILITY_CLAIM_MAP.md`, HKO local-maximum evidence-route row.

Missing:
- Final thesis wording must keep this distinct from theorem-strength local
  maximality in the ten-facet quotient model.

### 6. Search/Context Result

Status:
- HKO-local packets are supporting/contextual for the broader hostile-landscape
  story, not generic hostile-landscape evidence by themselves.
- Current central claim-control packet keeps HKO-specific packets outside the
  principal hostile-search table unless explicitly used as controls.

Source pointers:
- `research/sys-landscape.md`.
- `research/sys-landscape-toolbox-audit.md`, HKO-local perturbation row.
- `tasks/references/central-claim-control-packet-2026-06-01.md`, HKO and
  black-box-datascience rows.

Missing:
- Decide how much search/context belongs in the HKO section versus
  `thesis/black-box-datascience.tex`.

## Non-Writing Blockers

1. Close exact Packet 3 or honestly weaken theorem wording.
   Source pointers: `research/hko-local-maximum-status.md`;
   `research/hko-local-maximum.md`;
   `research/hko-local-maximum-exact-clarke.md`.

2. Produce final Sage-readable theorem certificate.
   Required checks should include exact row validity, active matrix rank,
   kernel/symmetry equality, and cone/no-increase condition.
   Source pointers: current partial verifier in
   `experiments/hko-local-maximum/exact-clarke/verify_widened_seed_witness.sage`
   and exact-Clarke research notes.

3. Write or refresh the mathematical reduction from verified certificate to
   local maximality.
   Source pointers: `formal/hko-local-maximality-conditions.tex`;
   `formal/hko-symmetry-gradient-structure.tex`;
   `research/sys-first-order-local-behavior.md`;
   `research/hko-local-maximum-proof-route-note.md`.

4. Decide empirical-support inclusion.
   Source pointers: `research/hko-local-maximum.md`;
   `experiments/hko-local-maximum/README.md`.

5. Get required Jörn/Kai review for theorem-strength wording.
   Source pointers: `tasks/definition-of-success.md`;
   `FACTSHEET.md` items 20--22.

## Fallback Branches

If the exact certificate closes:
- State theorem-strength local maximality in the ten-facet quotient model.
- Present the exact generator/verifier/proof chain as the main support.
- Present empirical checks as confirmation and broader context.
- Keep broader local-stability language conjectural unless separately proved.

If the exact certificate remains partial:
- Do not state theorem-strength local maximality.
- State exact partial-certificate status and current blocker.
- Present empirical confirmation honestly as support.
- State broader local stability as conjectural.

If exact work changes theorem wording:
- Refresh this file from `research/hko-local-maximum-status.md`,
  `research/hko-local-maximum.md`, exact artifacts, and task files before
  drafting final thesis prose.

## Review Gates

- Jörn math correctness review:
  theorem target, certificate implication, and conjectural/broader wording.
- Kai theorem framing review:
  theorem-strength HKO claim and Sage-readable proof-by-computation framing.
- Caveated/status wording only:
  empirical confirmation and search/context paragraphs, provided they do not
  assert theorem-strength claims.
