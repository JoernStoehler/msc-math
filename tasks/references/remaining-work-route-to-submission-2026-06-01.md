Status: dated planning report for remaining thesis-success work as assessed on
2026-06-01. Not source truth. Not the live planning layer.
Purpose: help a later agent explain, assign, or revise a route from the
2026-06-01 repo state to a defensible submitted thesis without redoing the
source scan.
Overruled by: active thesis files, research notes, experiment artifacts, formal
proof files, official sources, accepted Jörn/Kai decisions, and later task
state.

# Remaining Work Route To Submission

## 0. How To Use This File

This file is for agents, not primarily for Jörn to read end-to-end.

Before executing or assigning a route packet from this file, refresh the
relevant source surfaces and the compact live planning layer:
`tasks/definition-of-success.md`, `tasks/current-state.md`, and
`tasks/planning-notes.md`.

Use it to:

- explain the project state progressively in chat;
- choose the next work packet;
- check whether a proposed packet changes thesis success;
- avoid reopening already-settled scope questions.

Do not use it to:

- prove a thesis claim;
- declare the thesis ready;
- replace `tasks/current-state.md`, `tasks/planning-notes.md`, or source files;
- treat the 2026-06-01 packet order as current without refresh;
- ask Jörn to reclassify the 11 must-have content areas recorded in
  `tasks/current-state.md`.

Cost labels in this file are rough decision labels, not estimates:

- `Jörn 0`: no expected Jörn time during the packet.
- `Jörn low`: one focused review or expert question.
- `Jörn medium`: a review that likely needs careful reading or discussion.
- `Jörn high`: avoid unless it is clearly thesis-critical.

Ranges mean the packet can usually start at the lower cost, but the listed
acceptor gates still apply before thesis-facing claims are treated as accepted.

## 1. Source Surfaces Read

Required sources read:

- `tasks/README.md`
- `tasks/definition-of-success.md`
- `tasks/current-state.md`
- `tasks/planning-notes.md`
- `tasks/references/planning-agent-memory-2026-06-01.md`
- `tasks/references/recurring-agent-feedback-2026-06-01.md`
- `tasks/references/central-claim-control-packet-2026-06-01.md`
- `thesis/main.tex`
- `thesis/MAP.md`
- `research/INDEX.md`
- `experiments/MAP.md`
- `crates/MAP.md`
- `CAPABILITY_CLAIM_MAP.md`
- `tasks/submit-thesis/README.md`

Targeted high-risk sources read:

- HKO: `research/hko-local-maximum-status.md`,
  `research/hko-local-maximum.md`,
  `research/hko-local-maximum-exact-clarke.md`,
  `thesis/hko-local-maximum.tex`,
  `experiments/hko-local-maximum/README.md`
- Hostile landscape and pentagon product:
  `research/sys-landscape.md`,
  `research/sys-landscape-toolbox-audit.md`,
  `research/sys-landscape-datascience/idea-ledger.md`,
  `thesis/black-box-datascience.tex`,
  `thesis/rotated-regular-polygons.tex`,
  `experiments/sys-landscape/README.md`
- First-order, numerics, verification, code/data:
  `research/sys-first-order-local-behavior.md`,
  `thesis/first-order-perturbations.tex`,
  `research/numerics.md`,
  `thesis/numerics.tex`,
  `research/verification.md`,
  `thesis/published-code-data.tex`
- Tube, visualization, AI, foundations:
  `research/tube-algorithm.md`,
  `thesis/flow-graph-algorithm-ch2021.tex`,
  `research/visualization.md`,
  `thesis/visualization-3d.tex`,
  `thesis/use-of-ai.tex`,
  `thesis/generalized-reeb-orbits-polytopes.tex`,
  `thesis/quadratic-program-algorithm-hk2019.tex`,
  `thesis/preliminaries.tex`

Additional checks:

- `rg` scan of active thesis TODO/context markers.
- `rg` scan of formal/research/thesis/task status and unverified markers.
- `scripts/repo-status-summary.sh`.
- `git status --short`.

## 2. Current Thesis-Success State

### 2.1 Source-Backed Facts

1. `tasks/definition-of-success.md` defines success as a defensible thesis PDF,
   supportable claims, true or caveated repo promises, submission readiness, and
   Jörn final acceptance.
2. `thesis/main.tex` inputs an active thesis scaffold. It explicitly says most
   input files may contain only headings, labels, and TODO/context comments.
3. The active scaffold covers all 11 must-have content areas recorded in
   `tasks/current-state.md`.
4. `tasks/current-state.md` says current closeout is writeup-first.
5. `tasks/current-state.md` says HKO local maximality and hostile `sys` search
   are the main thesis story blocks currently treated as sufficient.
6. HKO theorem-strength wording is not closed. HKO exact-certificate Packet 3
   remains the main blocker for the exact first-order certificate.
7. HKO thesis-safe wording today is support/certificate-in-progress wording,
   unless HKO exact-certificate Packet 3 closes and the theorem wording passes
   Jörn/Kai review.
8. Hostile-landscape evidence is bounded empirical and method-ledger evidence.
   It must not be written as density, impossibility, or exhaustive-search
   wording.
9. The hostile-landscape method table is substantially populated, but
   `endpoint-residualized-regression` is not thesis-bearing as-is and
   `stat-sanity` is only provisional/source-truth-repair evidence.
10. The first-order arbitrary-polytope route is classified as `ONLY-HEAVY`.
    The readable thesis route is generic smooth row-chart exposition plus
    explicit caveats.
11. Numerics supports retained thesis text through exact/f64/indeterminate
    boundaries, generic-case-first reasoning, and selected exact/empirical
    diagnostics. It is not a public certified solver story by default.
12. Verification evidence exists for selected capacity/orbit-recovery checks:
    `28` selected polytopes, `469` trusted minima, and full reconstruction
    success for all `469` minima.
13. As observed on 2026-06-01, the 2026-05-31 repo-status reference records
    core smoke and selected checks, including formal and thesis builds. On
    current `HEAD` `4af221ff`, `scripts/repo-status-summary.sh` reported no
    uncommitted check-affecting paths; task/reference files were dirty or
    untracked and are outside that check-affecting claim.
14. Submission forms in `tasks/submit-thesis/` were downloaded on 2026-04-24.
    Final handin facts must be rechecked against official sources near handin.
15. The registration form was recorded as signed by Kai. Jörn confirmed on
    2026-06-03 that Elizabeth approved it; the pending action is to hand in the
    note to the `Prüfungsamt`.

### 2.2 Inferences

1. The main current bottleneck is not final PDF mechanics. It is turning
   scaffold plus scattered source knowledge into supportable retained thesis
   wording.
2. The next route should protect writing from hidden overclaim. That means
   settling or caveating central claims while drafting, instead of polishing
   scaffold text first.
3. HKO exact-certificate Packet 3 and hostile method-table closeout should be
   prepared as bounded claim-settlement packets, not open-ended research
   programs.
4. A direct whole-thesis prose sprint is risky unless the worker starts from a
   source-linked section-control surface or a narrowly scoped section.
5. Final claim-support, repo-promise, provenance, build, and readability checks
   should happen after enough retained TeX exists to audit.

### 2.3 Stale-Check-Needed Claims

1. Active thesis files are scaffold-heavy. This is source-backed now, but any
   later prose-writing session can change it and should refresh `thesis/*.tex`.
2. Official submission requirements can change. Recheck the official MNTF page
   before final handin.
3. Tracked experiment datasets, figures, and generated reports were not
   refreshed by the 2026-05-31 smoke/reference checks.
4. Formal notes contain many `unverified` and `TODO: JÖRN` markers. Use them as
   proof-route material only after local status checks.
5. `tasks/planning-notes.md` and this file are planning surfaces. They must be
   refreshed from source truth before supporting stronger claims.

### 2.4 Jörn/Kai/External-Context Items

1. Jörn has already set the 11 listed content areas as must-have. Do not ask him
   to classify them again in the next weeks unless he changes the scope.
2. Jörn final acceptance is required before submission.
3. Jörn/Kai mathematical acceptance is required for theorem-strength HKO wording.
4. Kai/Elizabeth/admin external facts can affect submission mechanics.
5. Jörn's agent-project experience is relevant for packet shape and review
   burden. This report does not require a Jörn answer now, but future route
   changes that rely on strong assumptions about agent reliability should ask a
   focused expert-review question.

## 3. Remaining Work By Success Condition

### 3.1 Thesis Prose And PDF

Source surfaces:
`thesis/main.tex`, `thesis/MAP.md`, active `thesis/*.tex`,
`thesis/DEVELOPMENT.md`, `tasks/planning-notes.md`.

Current evidence:
Strong that the active thesis is still scaffold-heavy. `rg` finds TODO/context
markers across the active thesis files, and `thesis/main.tex` says the active
thesis is a scaffold.

Done means:

- every retained section has reader-facing prose, not only comments;
- claim/support/caveat/pointer are visible where needed;
- bibliography, cross-references, figures, tables, and appendix pointers resolve;
- `cd thesis/ && latexmk && ./check-build.sh` passes close to final submission;
- Jörn accepts the PDF as ready.

Can be weakened or cut:
Section-level detail can be moved to appendix or future work only if retained
main-text claims no longer depend on it, or Jörn explicitly accepts the caveat.
The 11 content areas themselves are not to be reclassified here.

Acceptor:
Agent can draft and run build checks. Jörn accepts final prose. Jörn/Kai accept
theorem-strength math.

Cost/risk:
High calendar risk, Jörn medium-to-high near final review. Risk is hidden
overclaim if prose is drafted before claim strength is settled.

Dependencies/parallelism:
Central prose depends on HKO and hostile claim strength enough to avoid lying.
Supporting sections can be drafted in parallel if each packet names its claim
strength and caveats.

### 3.2 Central Claim Support And Caveats

Source surfaces:
`tasks/references/central-claim-control-packet-2026-06-01.md`,
`thesis/abstract.tex`, `thesis/introduction.tex`,
`thesis/hko-local-maximum.tex`, `thesis/black-box-datascience.tex`,
`thesis/conclusion.tex`, HKO and hostile research notes.

Current evidence:
Medium-to-strong. A reviewed central control packet exists, but it is not source
truth and central TeX is still scaffold.

Done means:

- abstract/introduction/conclusion state the HKO, pentagon, hostile-search, and
  method/support contributions at the actually supported strength;
- fallback wording is written for HKO if HKO exact-certificate Packet 3 does
  not close;
- hostile wording avoids "no `sys>1` examples" and avoids exhaustive claims;
- all central theorem-strength claims have explicit review gates.

Can be weakened or cut:
Claim strength can be weakened from theorem to support/certificate-in-progress
or bounded empirical evidence. Must-have topics cannot be silently moved to
future work while retained wording depends on them.

Acceptor:
Agent for caveated/status wording. Jörn/Kai for HKO theorem framing.

Cost/risk:
Jörn low-to-medium. High thesis-risk if skipped because central prose controls
reader expectations.

Dependencies/parallelism:
Runs before or alongside central TeX drafting. Uses HKO and hostile packet
outputs.

### 3.3 HKO

Source surfaces:
`research/hko-local-maximum-status.md`,
`research/hko-local-maximum.md`,
`research/hko-local-maximum-exact-clarke.md`,
`experiments/hko-local-maximum/exact-clarke/`,
`experiments/hko-local-maximum/README.md`,
`thesis/hko-local-maximum.tex`.

Current evidence:
Strong support, not a closed theorem certificate. HKO exact-certificate Packet
1 closed, HKO exact-certificate Packet 2 partially closed, and HKO
exact-certificate Packet 3 remains the main blocker. Current exact route uses
quartic `Q(tan(pi/5))`. Current widened representative rows verify rank `11`,
not final active-gradient rank `25`.

Done means one of:

- theorem route closes: active-gradient rank `25`, kernel dimension `15`, and
  kernel equals the symmetry tangent space; then HKO wording passes Jörn/Kai
  review;
- theorem route does not close within a bounded packet, and thesis wording is
  weakened honestly to support/certificate-in-progress wording.

Can be weakened or cut:
Do not claim strict raw `R^40` local maximality. If that exact-certificate
packet fails or is too costly, weaken theorem-strength wording while
preserving the HKO section as current result/status.

Acceptor:
Agent can map artifacts and run scripts. Jörn/Kai accept theorem-strength math
or weakening.

Cost/risk:
Technical risk high. Jörn low for artifact-mapping/fallback packet; Jörn medium
for theorem wording review. Main risk is an open-ended proof sprint.

Dependencies/parallelism:
HKO blocker packet should precede broad compute. It can run in parallel with
hostile table closeout and supporting-section status work.

### 3.4 Hostile Landscape / Search Data Science

Source surfaces:
`research/sys-landscape.md`,
`research/sys-landscape-toolbox-audit.md`,
`research/sys-landscape-datascience/idea-ledger.md`,
`research/sys-landscape-datascience/method-ledger.md`,
`experiments/sys-landscape/datascience/`,
`thesis/black-box-datascience.tex`,
`thesis/appendix-datascience-results.tex`.

Current evidence:
Medium-to-strong for bounded negative/search-usefulness claims. The method
ledger is now populated enough to support a table-shaped thesis section, but
some rows remain repair/cut decisions.

Done means:

- every thesis-used method row has a terminal state and source-owned evidence;
- `endpoint-residualized-regression` is repaired with endpoint-only loading and
  a durable report, or explicitly cut or moved to future work;
- `stat-sanity` is repaired from source truth or downgraded to non-load-bearing
  caveat evidence;
- any positive/conjectured-positive lead is escalated before unrelated method
  churn continues;
- thesis prose states bounded empirical/caveated wording only.

Can be weakened or cut:
Specific method rows can be omitted, downgraded, or moved to future work if the
thesis does not rely on them as failed attempts. The overall hostile-landscape
story still needs enough method coverage and caveats to be defensible.

Acceptor:
Agent can close row states with review. Jörn decides if an omitted standard
method family is thesis-acceptable when that is a taste/reader-expectation
judgment.

Cost/risk:
Jörn 0 to low unless a positive lead or omitted-method judgment appears.
Technical risk medium. Main risk is treating implementation failures as
scientific negatives without saying so.

Dependencies/parallelism:
Can run in parallel with HKO. Endpoint/stat-sanity rows should be serial before
optional SVM/interpretable-tail probes.

### 3.5 Supporting Must-Have Sections

Source surfaces:
`thesis/preliminaries.tex`,
`thesis/generalized-reeb-orbits-polytopes.tex`,
`thesis/quadratic-program-algorithm-hk2019.tex`,
`thesis/first-order-perturbations.tex`,
`thesis/flow-graph-algorithm-ch2021.tex`,
`thesis/rotated-regular-polygons.tex`,
`thesis/visualization-3d.tex`,
`thesis/numerics.tex`,
`thesis/published-code-data.tex`,
`thesis/use-of-ai.tex`,
topic research/formal/experiment notes.

Current evidence:
Mixed. All sections exist as scaffold. Several have clear source notes or
experiment surfaces. Formal proof notes contain unverified markers, so they
cannot be used blindly as final theorem support.

Done means:

- preliminaries define the objects used later and include only the necessary
  proof/citation level;
- generalized Reeb orbit and HK2019 sections justify the finite computation
  story at the strength used by later claims;
- first-order section gives the generic readable route and explicit
  non-generic/HKO caveats;
- CH2021/flow-graph/tube section is either a correct status/theory section or a
  clearly caveated algorithm section; it does not promise unsupported
  implementation/empirics;
- pentagon-product section gives supportable exact/Sage wording for the
  structured positive result;
- visualization section states what was visualized, what was not found, and
  which figures support exposition;
- numerics section states exact/f64/indeterminate boundaries without promising
  a public certified solver;
- code/data section states true reproducibility and archive promises;
- AI-use section is factual and at the chosen level of detail.

Can be weakened or cut:
Depth and theorem strength can be reduced when retained claims no longer depend
on stronger support. Do not silently drop a must-have content area.

Acceptor:
Agent for source transfer and caveated drafting. Jörn for taste/scope and any
math-sensitive cuts. Jörn/Kai for theorem-strength proof claims.

Cost/risk:
Jörn 0 to medium by section. Main risk is old legacy prose or unverified formal
material being copied into active thesis without revalidation.

Dependencies/parallelism:
Many sections can be parallelized after each packet states claim strength,
source surfaces, and review gate. First-order/numerics/HKO dependencies must
stay explicit.

### 3.6 Code, Data, And Reproducibility Promises

Source surfaces:
`crates/MAP.md`, `experiments/MAP.md`, `CAPABILITY_CLAIM_MAP.md`,
`tasks/references/repo-status-smoke-and-core-2026-05-31.md`,
`scripts/repo-status-summary.sh`, topic READMEs, final thesis code/data text.

Current evidence:
Core smoke and selected verification passed in the 2026-05-31 reference through
commit `269fb7b1`. Current `HEAD` is `4af221ff`; check-affecting changed paths
are none according to the summary script, but orientation/task files are dirty.

Done means:

- final thesis promises match actual repo state;
- referenced data/artifacts are present and owned by the correct topic;
- retained experiment claims have rerun, cached evidence, or explicit caveats at
  the strength used;
- README/run instructions are good enough for the promised reproducibility
  level;
- final checks are rerun after relevant code/data/text changes.

Can be weakened or cut:
Avoid public certified solver promises, broad reusable API promises, or fresh
artifact refresh promises unless retained thesis text needs them.

Acceptor:
Agent for mechanical checks. Jörn for final promise wording if it changes what
the thesis claims.

Cost/risk:
Jörn 0 to low until final acceptance. Technical risk medium if final prose
promises more than the repo actually supports.

Dependencies/parallelism:
Best run after retained prose says which code/data claims matter. Do not
refresh broad experiment artifacts as a default.

### 3.7 Build, Readability, Provenance

Source surfaces:
`thesis/main.tex`, `thesis/*.tex`, `thesis/bibliography.bib`,
`thesis/DEVELOPMENT.md`, final PDF, build scripts, source-linked claim map.

Current evidence:
Build passed in the 2026-05-31 reference, but active prose is still mostly
scaffold, so final readability/provenance checks are not yet meaningful.

Done means:

- thesis builds cleanly;
- no silent placeholders remain;
- references, labels, figures, tables, appendix links, bibliography, and code
  citations resolve;
- claim-support audit finds no uncaveated overclaim;
- intended readability/proofread level has happened.

Can be weakened or cut:
Only non-thesis polish can be cut. Broken references, missing support, and false
claims cannot be treated as cosmetic.

Acceptor:
Agent for checks. Jörn for final readability and submission readiness. Kai/Elizabeth
if they give blocker feedback.

Cost/risk:
Jörn medium near final. Technical risk low-to-medium if postponed until after
prose exists.

Dependencies/parallelism:
Depends on retained TeX. Can be broken into independent checks once prose exists.

### 3.8 Submission, Admin, Archive

Source surfaces:
`tasks/submit-thesis/README.md`, local form markdown/PDFs, official MNTF page,
current Jörn/Kai/Elizabeth context, archive target docs.

Current evidence:
Local forms were downloaded on 2026-04-24. Registration form status is recorded
as signed by Kai, approved by Elizabeth, and pending hand-in to the
`Prüfungsamt`. Zenodo is the leading preservation candidate because Kai named
it.

Done means:

- official submission requirements are rechecked near handin;
- required forms/uploads/printed copies are done or ready at the required stage;
- final archive target and artifact set are chosen;
- external-clock blockers are resolved or explicitly accepted.

Can be weakened or cut:
Post-thesis outreach/arXiv can stay future unless promoted. Required university
submission artifacts cannot be cut.

Acceptor:
Official sources, Jörn, Kai/Elizabeth/admin as applicable.

Cost/risk:
Jörn low-to-medium depending on external signatures/uploads. Risk is hidden
admin fact, not mathematical content.

Dependencies/parallelism:
Cheap prep can happen in parallel. Final archive/submission follows thesis done
except for external-clock preparation.

## 4. Ranked Next Work Packets

### Route Packet 0: Admin Timing Refresh

Why first:
Jörn confirmed on 2026-06-03 that 9.6.2026 is a good deadline for sending the
finished PDF to Kai. `tasks/planning-notes.md` says planners must refresh
official submission facts before final handin. This is cheap and should not
wait for final claim-support checks.

Work:

- recheck the official MNTF submission page and local form status;
- record the current concrete submission/admin timing facts or missing facts;
- state whether any admin fact changes the content-packet route.

Stop condition:
Current submission timing and external-action state are recorded, or a focused
Jörn/external question is named.

Expected Jörn time:
Jörn 0 unless an external action or unavailable conversation fact is needed.

Invalidated by:
Later official-source change or new advisor/admin instruction.

### Route Packet 1: HKO Blocker And Fallback-Wording Packet

Why next:
HKO claim strength controls the abstract, introduction, HKO section, and
conclusion. It is the highest-risk theorem-strength claim.

Work:

- map final HKO theorem target into exact subclaims;
- map artifacts to each subclaim;
- identify the missing HKO exact-certificate Packet 3 rows or certificate
  pieces;
- write theorem wording and fallback wording;
- stop before open-ended compute unless the missing compute target is explicit.

Stop condition:
Report states either a bounded exact next command/artifact route or a safe
fallback wording branch.

Expected Jörn time:
Jörn 0 during mapping. Jörn medium if theorem/fallback wording needs acceptance.

Invalidated by:
HKO exact-certificate Packet 3 closes before this starts, or Jörn/Kai changes
theorem target.

### Route Packet 2: Hostile Method-Table Closeout Packet

Why second:
The hostile-landscape section is central and has enough source structure for
bounded row closure. It can run in parallel with HKO.

Work:

- set terminal states for thesis-used method rows;
- repair, cut, or move `endpoint-residualized-regression` to future work;
- repair or downgrade `stat-sanity`;
- decide whether optional omitted families need explicit skipped rows;
- update toolbox audit/idea ledger only for changed row states.

Stop condition:
Every thesis-used row has a terminal state and no positive/conjectured-positive
lead is unresolved.

Expected Jörn time:
Jörn 0 unless a row becomes positive/conjectured-positive or an omitted-method
reader-expectation judgment is needed.

Invalidated by:
A method finds an actionable positive lead, or final thesis wording no longer
uses the hostile-landscape method table.

### Route Packet 3: Pentagon And Supporting-Section Status Packet

Why third:
Central prose depends on supporting sections not making unsupported promises.
The pentagon-product result also affects the abstract, introduction, and
conclusion, so its supportable exact/Sage wording must be explicit before
central TeX finalization. This packet is cheaper than drafting all sections and
reduces later overclaim.

Work:

- for `rotated-regular-polygons`, state the supportable pentagon-product
  wording, exact/Sage source surfaces, and caveats needed by the abstract,
  introduction, and conclusion;
- for first-order, numerics, tube/CH2021, visualization, code/data, AI,
  generalized Reeb orbit, HK2019, and preliminaries, state support strength,
  caveats, source surfaces, and the minimal thesis wording target;
- identify which sections can be drafted now and which require Jörn/Kai review.

Stop condition:
Each supporting must-have area has a status row usable by central prose and a
section worker.

Expected Jörn time:
Jörn 0 unless pentagon wording, tube scope/detail, or AI-use length requires
Jörn taste judgment.

Invalidated by:
HKO or hostile outputs change what supporting sections must support.

### Route Packet 4: Central TeX Draft Packet

Why fourth:
Once central claim strength is controlled, writing the central thesis sections
starts converting planning into the actual deliverable.

Work:

- draft `abstract`, `introduction`, `hko-local-maximum`,
  `black-box-datascience`, and `conclusion` at supportable strength;
- do not finalize central pentagon-product wording until Route Packet 3 gives
  supportable wording from `rotated-regular-polygons`;
- put caveats before details;
- add source pointers and review gates in comments where needed;
- avoid global polish.

Stop condition:
Central sections contain coherent retained prose with no known unsupported
headline claim.

Expected Jörn time:
Jörn low-to-medium for central prose review after agent review, not during first
drafting.

Invalidated by:
HKO theorem strength changes, hostile row closure produces a positive lead, or
Jörn/Kai changes central framing.

### Route Packet 5: Parallel Section Draft Packets

Why fifth:
The thesis cannot become defensible without supporting sections, but many are
agent-suitable once status rows exist.

Work:

- draft one section per packet from its status row and source surfaces;
- keep theorem-strength claims behind explicit proof/review gates;
- include only the detail needed for reader understanding and retained claims.

Stop condition:
Each section has reader-facing prose and a local open-claim list.

Expected Jörn time:
Jörn 0 during drafting. Jörn low/medium per section review depending on math
sensitivity.

Invalidated by:
Source-status rows reveal a hidden blocker that changes the section's role.

### Route Packet 6: Final Claim-Support, Build, Provenance, Submission Packet

Why last:
These checks need retained text. Running them too early mainly confirms the
known scaffold state.

Work:

- audit retained claims against proof/evidence/caveats;
- rerun build and final selected checks;
- verify repo/data/archive promises;
- recheck official submission facts;
- complete or stage required forms, uploads, printed copies, and other
  university submission artifacts for the point where they are required;
- choose the archive target and final artifact set, then verify that the thesis
  archive promise matches what is actually preserved;
- prepare final Jörn acceptance request with named caveats only.

Stop condition:
No known thesis-scope blocker remains after the named audits, required
submission/archive artifacts are completed or ready at the stage where they are
required, and every known remaining caveat is explicitly accepted by Jörn as
non-blocking.

Expected Jörn time:
Jörn medium-to-high, because final acceptance cannot be delegated.

Invalidated by:
New advisor/admin feedback, broken final build, or claim-support failure.

## 5. Visible Risks

1. HKO exact certificate risk:
   HKO exact-certificate Packet 3 may remain open. The route must preserve
   fallback wording.
2. Hostile-landscape evidence risk:
   Implementation bugs or provisional rows could be mistaken for negative
   scientific evidence.
3. Prose-before-claims risk:
   Drafting without source-linked claim strength can create late rewrite work.
4. Legacy-prose risk:
   Old thesis prose and formal notes contain unverified material and must not be
   copied without revalidation.
5. Agent-planning risk:
   Agents may optimize for producing a file or prompt rather than changing
   thesis-success state.
6. Admin timing risk:
   Jörn confirmed on 2026-06-03 that 9.6.2026 is a good deadline for sending
   the finished PDF to Kai. Official submission facts still need a
   current-source refresh before final handin.
7. Final-review risk:
   Jörn/Kai acceptance cannot be replaced by tool checks for theorem-strength
   or final-readiness decisions.

## 6. Questions

No question blocks the next packet ranking.

Focused future review request:

- Only if launching multiple agents at once, ask Jörn:
  "Approve running Route Packets 1-3 in parallel under the listed source
  surfaces, stop conditions, and independent-review requirement? If not, name
  which packet must wait."

Default if unanswered:

- Run Route Packet 0 first.
- Then run Route Packet 1 first; do not launch Route Packets 2-3 in parallel on
  an assumption about agent reliability.

## 7. Review Status For This Report

Review completed on 2026-06-01.

Review passes:

1. Omissions and bad-assumptions review: returned `NEEDS AMEND`.
   Accepted fixes:
   `rotated-regular-polygons` was made explicit in Route Packet 3, and admin
   timing refresh was moved into Route Packet 0.
2. Reasoning/completeness review: returned `NEEDS AMEND`.
   Accepted fixes:
   Route Packet 3 now preserves the pentagon-product dependency, and Route
   Packet 6 now requires submission/archive artifacts to be completed or ready.
3. Clarity/epistemics review: returned `NEEDS AMEND`.
   Accepted fixes:
   HKO exact-certificate packets were distinguished from route packets; cost
   ranges, working-tree status, cut-or-future-work wording, final stop
   condition, and future Jörn review request were clarified.

No accepted review finding remains open in this file.

Post-amend review completed on 2026-06-01 after Jörn challenged the earlier
completion claim. The strict post-amend review categories required by the
charter also passed:

1. Sanity review for omissions and bad assumptions: returned `PASS`.
2. Reasoning/completeness review: returned `PASS`.
3. Clarity/epistemics review: returned `PASS`.

Post-amend result:
The report is usable under the charter. This does not make the report source
truth, and it does not make the thesis ready.
