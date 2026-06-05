# HKO Local Maximum Content Notes

Status: thesis-local content-gathering notes, not source truth.

Purpose: gather the HKO result packet, source pointers, evidence status, and
remaining review gates needed to draft `thesis/hko-local-maximum.tex`.

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
   and linear symplectic maps, conditional on Jörn/Kai review of the
   agent-line-checked feasible-section implication from the exact certificate.
   Source pointers: `research/hko-local-maximum-status.md`;
   `research/hko-local-maximum.md`;
   `tasks/references/central-claim-control-packet-2026-06-01.md`;
   `thesis/hko-local-maximum.tex`.

3. Exact certificate:
   Code generates a witness, Sage verifies it, and the mathematical writeup
   proves that the verified witness implies the theorem target.
   Source pointers: `FACTSHEET.md` items 20--22;
   `research/hko-local-maximum-exact-witness.md`;
   `experiments/hko-local-maximum/theorem/exact-witness/README.md`;
   `experiments/hko-local-maximum/theorem/exact-witness/widened-representative-witness.json`;
   `experiments/hko-local-maximum/theorem/exact-witness/widened-representative-witness-verification.json`;
   `experiments/hko-local-maximum/theorem/feasible-section-certificate/README.md`;
   `experiments/hko-local-maximum/theorem/feasible-section-certificate/verification-summary.json`.

4. Empirical confirmation:
   Numerical and perturbative checks support the local-maximum picture.
   Source pointers: `research/hko-local-maximum.md`;
   `experiments/hko-local-maximum/README.md`;
   `experiments/hko-local-maximum/empirical/first-order/`;
   `experiments/hko-local-maximum/empirical/second-order/`;
   `experiments/hko-local-maximum/empirical/neighborhood-sampling/m10/`;
   `experiments/hko-local-maximum/empirical/neighborhood-sampling/m11/`;
   `experiments/hko-local-maximum/empirical/m11-ascent/`;
   `experiments/hko-local-maximum/empirical/neighborhood-sampling/m10-lagrangian-product/`.

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

## Current HKO Work Packets

The HKO non-writing work is currently split into four packets.

1. Proof-control packet:
   `research/hko-local-maximum-proof-control-packet.md`.
   This packet is current and records what the next proof work must prove or
   check. It is not final thesis-writing material.

2. Feasible-section packet:
   implemented by `theorem/feasible-section-certificate/`.
   The exact verifier checks 26 rows, exact row rank `25`, exact symmetry rank
   `15`, positive lambdas, and exact lambda-weighted row sum `0`.

3. Sage construction and verification packet:
   implemented by `construct_exact_witness.sage.py` and
   `verify_feasible_section_witness.sage.py`.

4. Thesis-writing companion finalization:
   this file now records the intended writing structure and review gate. After
   Jörn/Kai theorem review, update the status markers and write
   `thesis/hko-local-maximum.tex` from this file.

Do not treat this file as complete enough for final theorem-strength HKO
writing until Jörn/Kai review accepts the agent-line-checked formal implication
and theorem wording.

## Review Gate

Purpose: decide whether the exact Sage certificate plus
`formal/hko-feasible-section-upper-branches.tex` support theorem-strength HKO
local maximality in the ten-facet quotient model.

Source pointers:
- `formal/main.pdf`, section "HKO Feasible-Section Upper Branches".
- `formal/hko-feasible-section-upper-branches.tex`.
- `experiments/hko-local-maximum/theorem/feasible-section-certificate/verification-summary.json`.

Review labels:

1. `lem:hko-full-chart-volume-row`;
2. `lem:hko-feasible-section-upper-branch`;
3. `prop:hko-feasible-branch-slice-local-max`;
4. `lem:hko-transverse-symmetry-slice`;
5. `cor:hko-feasible-branch-quotient-template`.

Outcome needed:
- accept theorem route as thesis-ready modulo prose;
- or name the first proof label that needs repair;
- and decide whether the theorem wording may say "locally maximal in the
  ten-facet model modulo translations, scaling, and linear symplectic maps".

Fallback: if review finds a real gap, repair the implication or weaken the HKO
section to exact-certificate status plus the named blocker.

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
  the ten-facet feasible-section certificate now verifies exactly in Sage.

Source pointers:
- `research/hko-local-maximum-status.md`, sections "Current Repo Claim" and
  "Current Proof Route".
- `research/hko-local-maximum.md`, sections "Current State", "Decisions", and
  "Next Steps".
- `research/hko-local-maximum-proof-control-packet.md` for the current
  theorem target, proof tree, sufficient Sage decision problem,
  feasible-section route, and stop conditions.
- `formal/hko-feasible-section-upper-branches.tex` for the current PDF-review
  draft of the feasible-section upper-branch lemmas.
- `research/hko-local-maximum-proof-route-note.md` for the 2026-06-03
  selected-branch proof checkpoint and singular-KKT obstruction.
- `tasks/current-state.md`, HKO local maximality row.

Missing:
- Jörn/Kai acceptance of the agent-line-checked formal implication and
  theorem-strength wording.

### 3. Exact Certificate

Status:
- Exact dual coordinates, coefficient field, and rank-15 symmetry tangent setup
  are in place.
- The older widened representative-row witness passes present Sage checks but
  is partial.
- The current theorem-facing certificate is the 26-row feasible-section
  certificate.

Current checked partial witness facts:
- Exact field: `Q(tan(pi/5))`, minimal polynomial `t^4 - 10 t^2 + 5`.
- Endpoint representative rows: 5 rows, rank 5.
- Midpoint representative rows: 6 rows, rank 6.
- Current widened representative-row union: 11 rows, rank 11.
- Current right kernel dimension: 29.
- Current rows annihilate the 15 symmetry tangent directions.

Source pointers:
- `research/hko-local-maximum-exact-witness.md`, sections "Status",
  "Scope Boundary", "Sage Note", and "Packet 2 Note".
- `research/hko-local-maximum-proof-route-note.md`, especially the one-sided
  branch-certificate principle and candidate Sage decision problem.
- `research/hko-local-maximum-proof-route-note.md`, sections
  "Singular-KKT Obstruction" and "Padded-Endpoint Alternative", for the
  thesis-facing reason why the witness has to use singular positive-beta
  seven-facet branches.
- `experiments/hko-local-maximum/theorem/exact-witness/widened-representative-witness.json`.
- `experiments/hko-local-maximum/theorem/exact-witness/widened-representative-witness-verification.json`.

Current checked theorem-facing facts:
- 26 exact feasible-section rows.
- Exact row rank `25`.
- Exact symmetry tangent rank `15`.
- Every row annihilates the symmetry tangent space.
- Positive exact lambdas sum to `1`.
- The lambda-weighted exact row sum is `0`.

Missing:
- Jörn/Kai proof review against these verifier propositions and the formal
  implication.

Thesis-use note:
- Spend at least one sentence explaining why singular cases appear in the
  witness: the nonsingular active branches cover only rank `23` of the
  `25`-dimensional quotient slice, while the smooth padded-once diagnostic
  found no nonsingular minimum-action replacement rows.
- Clean explanation: the KKT matrix can be singular because there is a family
  of minimizing beta data at `a0`, but selected positive betas still admit
  local feasible sections through full-rank closure/normalization minors. The
  proof computes the derivative of those explicit feasible upper branches; it
  does not require nearby optimizing beta data or nearby KKT criticality.

### 4. Empirical Confirmation

Status:
- `empirical/first-order/` gives first-order numerical support and active-gradient
  bookkeeping.
- `empirical/second-order/` gives fixed-`F=10` support, including current notes reporting
  `rank(G)=25`, `dim ker(G)=15`, and negative curvature evidence along sampled
  flat directions.
- `empirical/neighborhood-sampling/` and `empirical/m11-ascent/` are supporting
  falsification/neighborhood checks.

Source pointers:
- `research/hko-local-maximum.md`, "Current State" and "Supporting Evidence".
- `experiments/hko-local-maximum/README.md`, "Result Strands".
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

1. Jörn review of the agent-line-checked formal implication from the Sage
   verifier propositions to the local-maximum theorem.
   Source pointers: `research/hko-local-maximum-status.md`;
   `research/hko-local-maximum.md`;
   `formal/hko-feasible-section-upper-branches.tex`;
   `experiments/hko-local-maximum/theorem/feasible-section-certificate/verification-summary.json`.

2. Decide final theorem wording.
   The intended wording is local maximality in the ten-facet model modulo
   translations, scaling, and linear symplectic maps. Broad HKO local
   maximality should remain conjectural unless separately proved.
   Source pointers: `research/hko-local-maximum-status.md`;
   `research/hko-local-maximum-proof-control-packet.md`.

3. Refresh this thesis companion into a final writing plan after Jörn/Kai
   theorem review.
   Source pointers: this file; `formal/hko-feasible-section-upper-branches.tex`;
   `experiments/hko-local-maximum/theorem/feasible-section-certificate/README.md`.

4. Decide empirical-support inclusion.
   Source pointers: `research/hko-local-maximum.md`;
   `experiments/hko-local-maximum/README.md`.

5. Get required Jörn/Kai review for theorem-strength wording.
   Source pointers: `tasks/definition-of-success.md`;
   `FACTSHEET.md` items 20--22.

## Fallback Branches

If the formal implication is accepted:
- State theorem-strength local maximality in the ten-facet quotient model.
- Present the exact generator/verifier/proof chain as the main support.
- Present empirical checks as confirmation and broader context.
- Keep broader local-stability language conjectural unless separately proved.

If the formal implication has a real gap:
- Do not state theorem-strength local maximality.
- State exact feasible-section certificate status and the remaining formal
  blocker.
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

## Thesis Section Writing Plan

Use this plan after the review gate above is accepted, or use the fallback
branch if it is not accepted.

1. Start with the theorem target.
   Claim: in the ten-facet dual-vertex model, HKO is locally maximal modulo
   translations, scaling, and linear symplectic maps.
   Caveat: this is not a theorem about all nearby facet counts or broad HKO
   local stability.

2. Recall the HKO2024 input.
   State the Lagrangian product of the two circumradius-1 regular pentagons
   and cite HKO2024 for the capacity value. Use the exact field
   `t = tan(pi/5)`, `t^4 - 10t^2 + 5 = 0`, only where it helps explain the
   verifier.

3. State the certificate theorem in human terms.
   Rust produces a finite witness. Sage reconstructs the exact HKO geometry,
   verifies the witness equations exactly, and checks 26 feasible-section rows,
   row rank `25`, symmetry rank `15`, positive lambdas, and row sum `0`.

4. Explain why feasible sections are used.
   The witness needs singular positive-beta rows because nonsingular active
   branches span only rank `23` on the 25-dimensional quotient slice, and the
   padded-once diagnostic found no nonsingular minimum-action replacements.
   The proof avoids nearby KKT criticality: it fixes enough beta coordinates,
   solves the closure/normalization equations locally, and uses the resulting
   feasible HK2017 branches only as upper bounds.

5. Give the proof tree.
   Each selected feasible section gives a smooth upper branch `U_sigma` with
   `sys <= U_sigma` and equality at HKO. The exact row certificate says the
   first-order rows of these upper branches span the quotient cotangent space
   and contain `0` in their positive convex hull. Therefore every nonzero slice
   direction has some upper branch with strictly negative first-order change.
   A uniform remainder argument gives strict local maximality on the slice.
   The transverse-slice lemma turns this into quotient-local maximality.

6. State empirical confirmation separately.
   Use only short supporting language unless a retained experiment is refreshed
   at writing time. Keep `M_11`, neighborhood sampling, and second-order checks
   as context, not as the proof of the theorem.

7. End with scope and fallback wording.
   If the formal review is accepted, the section may state the theorem-strength
   ten-facet quotient result. If the formal review finds a gap, state the exact
   certificate and the named remaining implication blocker instead.
