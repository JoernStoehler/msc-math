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
   and linear symplectic maps, conditional on final theorem wording/Kai review.
   Jörn quick-reviewed the agent-line-checked feasible-section implication from
   the exact certificate on 2026-06-05 and spotted no gaps. The 2026-06-05
   clarity repairs made the local chart hypotheses, the volume-row arithmetic,
   and the local symmetry group/dimension explicit.
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
   Kai theorem review, update the status markers and write
   `thesis/hko-local-maximum.tex` from this file.

Do not treat this file as complete enough for final theorem-strength HKO
writing until Kai review accepts the theorem wording.

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
- Jörn quick-reviewed the rebuilt PDF on 2026-06-05 and spotted no gaps;
- Jörn judged remaining mistakes, if any, likely closeable before thesis
  release;
- the 2026-06-05 clarity repairs made the local chart hypotheses, the
  volume-row arithmetic, and the local symmetry group/dimension explicit;
- remaining: decide with Kai whether the theorem wording may say "locally
  maximal in the ten-facet model modulo translations, scaling, and linear
  symplectic maps".

Fallback: if review finds a real gap, repair the implication or weaken the HKO
section to exact-certificate status plus the named blocker.

## Thesis Material Beyond The Sage Script

Purpose: prevent the HKO thesis section from becoming only a walkthrough of
`verify_feasible_section_witness.sage.py`. The script verifies the finite
predicate. The thesis still has to supply the mathematical setup, the
implication from the finite predicate to local maximality, and the scope
control around the result.

Current status: this is the working checklist for
`thesis/hko-local-maximum.tex`. It should be refreshed if the theorem wording,
row certificate, or Kai review outcome changes.

Classification rule:
- core thesis content is prose or theorem/proof text that the HKO section must
  contain;
- an auxiliary asset is a table, display, figure, code excerpt, or data excerpt
  that supports core thesis content;
- do not create an auxiliary asset merely because a source artifact exists in
  `experiments/`;
- an auxiliary asset is worth creating only if it solves a specific reader
  problem better than prose alone.

### 1. The theorem setting

What the thesis must explain:
- the HKO2024 input is the Lagrangian product of two regular pentagons, with
  the capacity value taken from HKO2024;
- the local model is the ten-facet dual-vertex chart
  `K(a) = {x : <a_i,x> <= 1}` near the HKO point `a0`;
- the coordinate order is facet-major in `(q1,q2,p1,p2)`;
- the statement is quotient-local, not raw ambient strict local maximality.

Why this is not just script explanation:
- the Sage verifier can reconstruct `a0` and check exact algebra, but it does
  not decide which local moduli problem the thesis claims.

Source pointers:
- `formal/hko-feasible-section-upper-branches.tex`,
  `lem:hko-capacity-normalization` and
  `lem:hko-full-chart-volume-row`;
- `research/hko-local-maximum-status.md`, "Current Repo Claim";
- `research/hko-local-maximum-proof-control-packet.md`, "Theorem Target For
  Review".

### 2. The quotient and slice

What the thesis must explain:
- translations, scaling, and the identity component of `Sp(4)` preserve the
  systolic ratio in the local model;
- their tangent directions form a `15`-dimensional symmetry tangent space at
  HKO;
- a `25`-dimensional transverse slice is used because the ambient chart has
  dimension `40`;
- rows that annihilate the symmetry tangent space descend to covectors on the
  quotient slice.

Why this is not just script explanation:
- Sage checks rank `15`, row annihilation, and row rank `25`; the thesis must
  say why these are the right linear objects for quotient-local maximality.

Source pointers:
- `formal/hko-feasible-section-upper-branches.tex`,
  `lem:hko-ambient-rows-descend-to-slice`,
  `lem:hko-transverse-symmetry-slice`, and
  `cor:hko-feasible-branch-quotient-template`;
- `experiments/hko-local-maximum/theorem/feasible-section-certificate/README.md`,
  "Current verifier summary".

### 3. Why selected upper branches suffice

What the thesis must explain:
- no completeness theorem for all HK2017 branches is needed;
- for each selected feasible branch `U_sigma`, one has
  `sys(a) <= U_sigma(a)` near `a0` and equality at `a0`;
- if every nonzero slice direction has some selected upper branch with
  negative first derivative, then `sys` decreases in that direction;
- the positive convex-hull certificate gives a uniform negative first-order
  margin on the slice unit sphere.

Why this is not just script explanation:
- the verifier checks the finite row certificate, but the thesis must prove the
  logical implication from selected upper branches to local maximality.

Source pointers:
- `formal/hko-feasible-section-upper-branches.tex`,
  `lem:hko-feasible-section-upper-branch`,
  `lem:hko-positive-convex-certificate-margin`, and
  `prop:hko-feasible-branch-slice-local-max`;
- `research/hko-local-maximum-proof-route-note.md`, "One-Sided
  Branch-Certificate Principle" and "Convex-Hull Criterion On The Slice";
- `research/hko-local-maximum-proof-control-packet.md`, "Proof Tree".

### 4. Why singular rows appear, and why they are legitimate

What the thesis must explain:
- nonsingular positive-beta active rows are not enough for the current
  certificate: they span only rank `23` in the `25`-dimensional quotient slice;
- the padded-once diagnostic did not find a nonsingular minimum-action
  replacement route;
- this is why the final witness uses singular positive-beta seven-facet rows;
- these rows are not justified as nearby optimizing KKT branches;
- they are justified by explicit feasible beta sections, which give smooth
  HK2017 upper branches.

Why this is not just script explanation:
- Kai may reasonably ask why singular data occur although HKO has a minimizing
  family, and why the proof is still allowed to differentiate something. The
  answer is a mathematical construction of feasible sections, not a script
  implementation detail.

Source pointers:
- `formal/hko-feasible-section-upper-branches.tex`,
  `lem:hko-feasible-section-upper-branch` and
  `rem:hko-feasible-section-derivative-data`;
- `research/hko-local-maximum-proof-route-note.md`, "Singular-KKT
  Obstruction", "Endpoint Rows And The Minimizing Family", and "Padded-Endpoint
  Alternative";
- `research/hko-local-maximum-proof-control-packet.md`, "Feasible-Section
  Packet".

### 5. What the exact certificate statement should say

What the thesis must explain:
- Rust is only a generator of finite candidate choices;
- Sage is trusted for exact construction and exact verification;
- the proof-facing certificate is the exception-based verifier result, not the
  search procedure;
- the final finite predicate is: 26 selected feasible-section rows, exact
  action equality at HKO, exact feasible-section derivative rows, exact
  symmetry annihilation, exact row rank `25`, positive exact lambdas summing to
  `1`, and exact lambda-weighted row sum `0`.

Why this is not just script explanation:
- the thesis should state the checked mathematical predicate before or while
  showing any code snippets. A reader should be able to understand what the
  computation proves without reconstructing it from source code order.

Source pointers:
- `experiments/hko-local-maximum/theorem/feasible-section-certificate/README.md`,
  "Trust Boundary" and "Current verifier summary";
- `experiments/hko-local-maximum/theorem/feasible-section-certificate/verification-summary.json`;
- `research/hko-local-maximum-proof-control-packet.md`, "Sage Construction
  And Verification Packet".

### 6. What to include from the verifier script

What the thesis should use from the script:
- selected short code listings only where they make a mathematical check more
  inspectable;
- current anchors to consider: field setup, HKO geometry reconstruction,
  volume row, symmetry tangent generators, row-level feasible-section checks,
  and convex certificate checks;
- printed summaries or small snippets if they help a reader sanity-check the
  long finite list.

What the thesis should not do:
- do not ask the reader to trust a long code listing instead of a stated
  mathematical predicate;
- do not present the verifier as an independent derivation of the formulas.
  The formulas belong in the proof; Sage mirrors them and checks exact witness
  equations.

Source pointers:
- `experiments/hko-local-maximum/theorem/feasible-section-certificate/verify_feasible_section_witness.sage.py`;
- `formal/hko-feasible-section-upper-branches.tex`,
  `rem:hko-feasible-section-derivative-data`.

### 7. Empirical and sanity-check context

What the thesis may include after the theorem proof:
- first-order numerical bookkeeping;
- second-order support;
- perturbation/neighborhood checks in `M_10`, `M_11`, and Lagrangian-product
  neighborhoods, if refreshed at writing time;
- the formula finite-difference sanity checks only as development assurance,
  if included at all.

What the thesis must not claim:
- finite-difference sanity checks do not prove the theorem;
- empirical neighborhood checks do not prove broad local maximality;
- `M_11` and ascent evidence are context unless a separate theorem uses them.

Source pointers:
- `experiments/hko-local-maximum/README.md`, "Result Strands";
- `experiments/hko-local-maximum/theorem/feasible-section-certificate/README.md`,
  "Ancillary Formula Sanity Checks";
- `research/hko-local-maximum.md`, "Supporting Evidence";
- `research/sys-landscape-toolbox-audit.md`, HKO-local perturbation row.

### 8. Core content and auxiliary asset decisions

Status:
- the core content below is not yet written in thesis-ready form;
- the auxiliary assets below are not yet created;
- the next work should update this same companion or `thesis/hko-local-maximum.tex`,
  not create another planning surface.

Core thesis content, not auxiliary assets:

1. Finite certificate proposition.
   Purpose: state the exact finite predicate verified by Sage before discussing
   code order. This should include the exact field, HKO geometry/capacity
   normalization, 26 feasible-section rows, equality at HKO, symmetry
   annihilation, row rank `25`, and the positive convex relation.
   Source pointers:
   `experiments/hko-local-maximum/theorem/feasible-section-certificate/verification-summary.json`;
   `formal/hko-feasible-section-upper-branches.tex`,
   `cor:hko-feasible-branch-quotient-template`.

2. Proof-implication text.
   Purpose: explain why the verified predicate implies local maximality:
   feasible upper branches give upper bounds for `sys`, the positive convex
   relation gives a uniform negative first-order margin on the quotient slice,
   and the transverse-slice lemma turns the slice statement into the quotient
   statement.
   Source pointers:
   `formal/hko-feasible-section-upper-branches.tex`,
   `lem:hko-feasible-section-upper-branch`,
   `lem:hko-positive-convex-certificate-margin`,
   `prop:hko-feasible-branch-slice-local-max`,
   `lem:hko-ambient-rows-descend-to-slice`,
   `lem:hko-transverse-symmetry-slice`.

3. Trust-boundary text.
   Purpose: say that Rust proposes finite candidate data, Sage verifies exact
   equations/ranks/positivity in the chosen number field, and LaTeX proves the
   implication. The theorem should not rely on Rust correctness or numerical
   search correctness.
   Source pointers:
   `FACTSHEET.md` items 20--22;
   `experiments/hko-local-maximum/theorem/feasible-section-certificate/README.md`,
   "Trust Boundary".

4. Singular-row explanation.
   Purpose: explain why the witness uses singular positive-beta seven-facet
   rows and why this is legitimate. The proof differentiates explicit feasible
   beta sections, not nearby optimizing KKT branches.
   Source pointers:
   `research/hko-local-maximum-proof-route-note.md`, "Singular-KKT
   Obstruction" and "Endpoint Rows And The Minimizing Family";
   `research/hko-local-maximum-proof-control-packet.md`, "Feasible-Section
   Packet"; `formal/hko-feasible-section-upper-branches.tex`,
   `lem:hko-feasible-section-upper-branch`.

Recommended auxiliary assets:

1. Verification-interface table.
   Status: not done. Priority: high.
   Shape: `Proof obligation | Sage check | Formal proof use`.
   Purpose: make the bridge between the verifier and proof auditable. This is
   better than a raw code listing because it is organized by mathematical
   obligation rather than by source-file order.
   Candidate rows: HKO field/geometry/capacity normalization; feasible beta
   section data; derivative row formula; symmetry annihilation; row rank and
   positive convex relation; quotient-local conclusion.

2. Compact certificate facts display.
   Status: not done. Priority: medium-high, only next to the finite
   certificate proposition.
   Content: ambient dimension `40`, symmetry rank `15`, selected rows `26`,
   row rank `25`, positive lambdas, lambda sum `1`, lambda-weighted row sum
   `0`.
   Purpose: make the finite linear-algebra endpoint visible at a glance.

Deferred auxiliary assets:

1. Trust-boundary mini table.
   Defer because prose may be enough. Use only if the trust-boundary paragraph
   becomes too dense.

2. Selected Sage code snippets.
   Defer until the thesis prose exists. Candidate snippets are field/root
   pinning, row verification, and convex-certificate verification. Use snippets
   only to support a stated mathematical predicate, not to replace it.

3. Nonsingular-endpoint geometric figure.
   Defer. The geometric point should first be handled as prose in the
   singular-row explanation. Add a figure only if prose is not enough.

Rejected or non-assets for thesis purposes:
- raw 26-row table;
- full lambda list;
- full printed witness excerpts;
- finite-difference sanity-check report;
- standalone proof-dependency diagram before the proof prose exists;
- appendix code-audit map before final prose chooses snippets.

Source pointers:
- `experiments/hko-local-maximum/theorem/feasible-section-certificate/README.md`,
  "Trust Boundary", "Files", and "Current verifier summary";
- `experiments/hko-local-maximum/theorem/feasible-section-certificate/verification-summary.json`;
- `experiments/hko-local-maximum/theorem/feasible-section-certificate/verify_feasible_section_witness.sage.py`;
- `research/hko-local-maximum-proof-route-note.md`, "Endpoint Rows And The
  Minimizing Family".

### 9. Forbidden thesis claims

Do not write:
- "The branch list is complete."
- "Rust proves the certificate."
- "The finite-difference sanity script verifies the theorem."
- "The theorem proves broad HKO local maximality outside the ten-facet model."
- "The singular rows are nearby optimizing branches."
- "The verifier independently derives all formulas."
- "The quotient-local theorem is raw strict local maximality in `R^40`."

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
- Kai acceptance of theorem-strength wording.

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
- Kai acceptance of theorem-strength wording and the final thesis presentation
  of the certificate.

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

1. Decide final theorem wording.
   The intended wording is local maximality in the ten-facet model modulo
   translations, scaling, and linear symplectic maps. Broad HKO local
   maximality should remain conjectural unless separately proved.
   Source pointers: `research/hko-local-maximum-status.md`;
   `research/hko-local-maximum-proof-control-packet.md`.

2. Refresh this thesis companion into a final writing plan after Kai theorem
   review.
   Source pointers: this file; `formal/hko-feasible-section-upper-branches.tex`;
   `experiments/hko-local-maximum/theorem/feasible-section-certificate/README.md`.

3. Decide empirical-support inclusion.
   Source pointers: `research/hko-local-maximum.md`;
   `experiments/hko-local-maximum/README.md`.

4. Get required Kai review for theorem-strength wording.
   Source pointers: `tasks/definition-of-success.md`;
   `FACTSHEET.md` items 20--22.

## Fallback Branches

If final theorem wording/Kai review accepts the theorem route:
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
  quick-reviewed on 2026-06-05 with no spotted gaps; remaining mistakes, if
  any, judged likely closeable before thesis release.
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
