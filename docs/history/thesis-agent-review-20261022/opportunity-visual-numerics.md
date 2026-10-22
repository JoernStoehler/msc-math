# opportunity-visual-numerics — bounded source review

Review date: 22 October 2026. Checkout: `/workspaces/msc-math`, HEAD `9667e351514faf1f00e2d86796c20e9d9b6f2f10`.

**Disposition: no surviving manuscript defect found in this scope.** The requested panel explanations and certified example already exist. Proposal: close this opportunity without a manuscript patch or new numerical/figure production; preserve the specific safeguards below. This is a source/evidence review, not renewed human acceptance or a full numerical implementation audit.

## Scope and access

Started with `README.md` and `meta-topics/project-management/thesis-work.md:138`. Read all three requested source files, selected implementation/evidence anchors, the visualization runbook, and the appendix's public-verifier mapping. `thesis/main.tex:78`, `thesis/main.tex:79`, and `thesis/main.tex:80` select these files. The intervening selected chapter at `thesis/main.tex:76` means the requested 12–14 identifiers are treated here as source filenames, not a fresh assertion about displayed section numbers.

Inspected both existing thesis PNGs directly. Did not build or inspect the current PDF, regenerate figures, run Rust/Sage/tests, fetch the public repository, materialize external datasets, or launch producers. Reported historical test results remain attributed to their retained records. Four pre-existing modified repository files were observed and left untouched. This review makes no repository or git writes; only the requested scratchpad and disposable review packet are written.

“Severity: none” below means the former concern is solved, not that its underlying inference boundary is unimportant. Status applies to the selected source, not to human acceptance.

## Findings

### V1 — Cube panel has an explicit purpose and metric limitation

- **Status: solved. Severity: none.**
- **Exact locations:** `thesis/chapters/12-visualization.tex:14`, `thesis/chapters/12-visualization.tex:22`, `thesis/chapters/12-visualization.tex:51`.
- **Evidence:** The text identifies the coloured cube meshes as aids for distinguishing ridges and following shared edges, explicitly forbids comparing apparent areas with symplectic ridge areas, and explains interpolation, stereographic distortion and pole clipping. The existing PNG visibly contains the described meshes and edge structure. Producer selection agrees at `experiments/visualization/viewer/screenshot-figures.mjs:40`.
- **Concrete proposed fix:** No patch needed. Retain the area warning beside the panel discussion and the clipping/qualitative-view qualification in its caption when editing surrounding prose.

### V2 — HKO panel illustrates placement and dynamics without supplying a capacity certificate

- **Status: solved. Severity: none.**
- **Exact locations:** `thesis/chapters/12-visualization.tex:25`, `thesis/chapters/12-visualization.tex:29`, `thesis/chapters/12-visualization.tex:55`, `thesis/chapters/12-visualization.tex:61`.
- **Evidence:** The orange curve explains where the six-segment orbit lies relative to the boundary; displayed arc lengths are explicitly unrelated to dwell times or action. Recovery uses numerical closure/containment checks and is explicitly called a computed example rather than capacity proof. The PNG visibly distinguishes the orange outlined trajectory from the grey skeleton. The screenshot selects trajectory zero at `experiments/visualization/viewer/screenshot-figures.mjs:61`; recovery guards are `experiments/visualization/src/trajectories.rs:36` and `experiments/visualization/src/trajectories.rs:43` (closure `1e-6`, half-space violation `1e-4`). The chapter also records the failure to obtain a reliable visual search rule.
- **Concrete proposed fix:** No chapter patch needed. Preserve “numerically recovered” and the explicit non-proof qualification. If reusing this image elsewhere, carry those qualifications with it: its exporter uses the older threshold-based solver (`experiments/visualization/src/orbit_collection.rs:8`, `experiments/visualization/src/orbit_collection.rs:32`), so the certified production capacity contract must not be silently transferred to the displayed orbit.

### N1 — The requested compact certified example is already complete

- **Status: solved. Severity: none.**
- **Exact locations:** `thesis/chapters/13-numerics.tex:156`, `thesis/chapters/13-numerics.tex:168`, `thesis/chapters/13-numerics.tex:175`, `thesis/chapters/13-numerics.tex:179`, `thesis/chapters/13-numerics.tex:184`.
- **Evidence:** The cube example supplies all eight exact input rows, a feasible word `(0,4,2,6)` with weights `1/4`, `Q=1/8`, action `4`, the complete 24-order comparison, two tied sparse winners, capacity `4`, output interval `[4,4]`, and the below-four versus closed-at-four cutoff decision. The displayed pairings give `(1-1+1+1)/16=1/8`. The closure enumeration consists of four support-pair choices with six cyclic orders each; the two same-axis choices each contribute one positive and one negative alternating order, leaving twenty zero objectives.
- **Evidence chain:** Input rows are fixed by `topics/capacity-and-qp/development/tools/product_closure_route/main.rs:270`. The retained audit row is `topics/capacity-and-qp/development/tools/product_closure_route/sample5.jsonl:4`: `q_max_exact="1/8"`, `capacity_exact="4"`, 24 orders, two winners. Production assigns its endpoints using `rational_bounds` at `crates/symplectic/src/algorithms/capacity_4d/mod.rs:497`; exact representability yields equal endpoints at `crates/symplectic/src/algorithms/capacity_4d/mod.rs:709`.
- **Concrete proposed fix:** No additional example or execution needed. Keep the existing example and its footnote. Preserve the evidence distinction: the retained development row records the exact capacity and winner count; `[4,4]` follows from the inspected production conversion, not from a newly observed API execution or a serialized endpoint pair in that row.

### N2 — Example exactness is correctly limited to its input and route

- **Status: solved. Severity: none.**
- **Exact locations:** `thesis/chapters/13-numerics.tex:15`, `thesis/chapters/13-numerics.tex:28`, `thesis/chapters/13-numerics.tex:39`, `thesis/chapters/13-numerics.tex:146`, `thesis/chapters/13-numerics.tex:185`, `thesis/chapters/14-code-data.tex:99`.
- **Evidence:** Exactness concerns the supplied dyadic rows; facet/norm/candidate limits are explicit. The product route resolves possible winners rationally and returns sparse-family winners, not every geometric minimizing branch. The cube's zero-width interval is expressly distinguished from a usual general-polytope interval. Chapter 14 separately warns that certified capacities do not certify historical ratios with floating-point volumes.
- **Concrete proposed fix:** No patch needed. Retain these qualifications; do not recast the cube as a demonstration of an inverse-defect radius, algebraic-coordinate recovery, certified volume, or all-orbit enumeration.

### E1 — Numerical counts have separate evidence owners and honest availability statements

- **Status: solved. Severity: none.**
- **Exact locations:** `thesis/chapters/13-numerics.tex:197`, `thesis/chapters/13-numerics.tex:220`, `thesis/chapters/13-numerics.tex:232`, `thesis/chapters/13-numerics.tex:244`, `thesis/chapters/14-code-data.tex:60`, `thesis/chapters/14-code-data.tex:69`.
- **Evidence:** The 249-system and 88/1271-word counts match `topics/capacity-and-qp/development/tools/general_algorithm_ablation/RESULTS.md:35` and `topics/capacity-and-qp/development/tools/general_algorithm_ablation/RESULTS.md:56`. Their missing original per-system output is explicitly disclosed. The 88-product row packet is tracked; the separate 10,240-product claim is a historical geometry/cache join described at `topics/capacity-and-qp/development/tools/product_closure_route/RESULTS.md:46`. External materialization paths are registered at `artifacts/registry.json:94`. The four-test production correspondence is separately bounded; its historical receipt is `topics/capacity-and-qp/evidence/numerics-correspondence-check/README.md:21`.
- **Concrete proposed fix:** No chapter patch needed. Preserve the separate owners and missing-output disclosure. Cite the 10,240 claim as retained-report evidence unless its external files are independently checked; do not relabel historical development audits as a fresh production audit.

### E2 — Appendix and public verifier are mapped without a false public-availability claim

- **Status: solved. Severity: none.**
- **Exact locations:** `thesis/chapters/14-code-data.tex:7`, `thesis/chapters/14-code-data.tex:25`, `thesis/appendices/hko-certificate.tex:13`, `thesis/appendices/hko-certificate.tex:76`.
- **Evidence:** The chapter pins the public check to its dated commit, names the former and current verifier paths, and explicitly says the appendix file was absent from that commit. The appendix explains its compact assignment representation and the division between finite algebraic checks and analytic implications.
- **Concrete proposed fix:** No patch needed. Keep both the public-version qualification and the appendix-to-verifier map. No online recheck or release work is necessary for this bounded proposal.

### W1 — Requiring a new nonzero-width example would exceed the stated obligation

- **Status: withdrawn. Severity: none.**
- **Exact locations:** `meta-topics/project-management/thesis-work.md:162`, `thesis/chapters/13-numerics.tex:179`, `thesis/chapters/13-numerics.tex:185`.
- **Reason:** The obligation asks for concrete input, interval/action decision and capacity output; it does not demand a general-route, nonzero-width or near-singular example. The existing cube meets it. The retained cube row's maximum objective-interval width is a diagnostic, not the endpoints of a selected fixed-word interval, and should not be repurposed as such.
- **Concrete proposed fix:** Drop the prospective demand for a replacement example. If a general-route worked certificate is later selected as a separate teaching goal, first identify a retained record containing its actual input, residual/defect bounds, radius and output endpoints; do not invent those numbers or launch a producer under this review.

## Scoped proposal and stop point

Accept the existing Chapter-12 panel explanations and Chapter-13 cube calculation as the implementation of this opportunity. Carry forward the exact evidence anchors above and preserve Chapter 14's availability/trust boundaries. No surviving finding requires source changes, dataset materialization, solver work, a new certificate, or figure regeneration. Optional deeper implementation verification and whole-PDF/human acceptance remain outside this review.

[review: /root/message_review]
