# Chapter 7 claims and source/PDF identity

Read-only review, 22 October 2026. Assigned checkout: `/workspaces/msc-math`; HEAD `9667e351514faf1f00e2d86796c20e9d9b6f2f10`. No repository edits, builds, verifier runs or Git writes were performed. This report and supporting inspection files are outside the repository. The 40-minute limit was not exhausted.

**Result:** a matching current PDF is available. No surviving claim/caveat mismatch was found in the requested L10-N/L10-N+ scope. Section 7.7 / Theorem 7.8 is on physical PDF pages **52–54** of the 105-page PDF. These pages were extracted and visually inspected in full. This is a bounded claim-to-evidence review, not whole-PDF PASS, human acceptance, a fresh algebra replay, or exhaustive proof verification.

## Identity established

- PDF: `docs/history/selected-partner-navigation-20261022/thesis.pdf`, 1,783,112 bytes, 105 pages, SHA-256 `00aabd301cd9dcaae0c0a67b81dc0545533789f4d1ca24007831109c32384a06`.
- Recorded source revision: **`6b02c04f73b3705c37cbca097ac3a43d535db384`**. The manifest's source-revision field is `docs/history/selected-partner-navigation-20261022/manifest.json:2`; PDF hash/page count are at lines 3–4. All **52** recorded input hashes match both that commit and the inspected working files. `git diff --name-only 6b02c04f HEAD -- thesis` is empty; `thesis/` has no working-tree changes. The manifest explicitly limits coverage to recorded local TeX inputs and bibliography, excluding external TeX packages (`manifest.json:65`).
- `thesis/build/main.pdf` has identical bytes by SHA-256. The retained publication was added at HEAD; HEAD is not claimed as the original build revision. The selected entry includes Chapter 7 at `thesis/main.tex:72`.
- `docs/resume/thesis-resume.pdf` has the different SHA-256 `5b487580ee5bed7473e7de3233ea3ccaacbb3c7400d685883b99a081aa582fea`. It was **not** used as the current artifact. `thesis/README.md:20` identifies it as diagnostic; the newer retained PDF is routed at `meta-topics/project-management/selected-partner-navigation-20261022.md:46`.

## Findings and disposition

Here **solved** means the requested review obligation is satisfied by existing evidence, without a patch in this review; **withdrawn** means the potential objection is unsupported; **surviving** would identify a present defect. There are no surviving findings. Severity “none” means no present defect; no former defect is inferred merely from the review question.

### F1 — Current PDF prerequisite is met

- **Status:** solved. **Severity:** none.
- **Exact locations:** `docs/history/selected-partner-navigation-20261022/manifest.json:2`, `:3`, `:4`, `:30`; `meta-topics/project-management/thesis-work.md:261`; `thesis/README.md:20`.
- The 4 October entry explicitly describes a historical 100-page candidate and pages 51–53. It does not identify the current 105-page artifact. The independently checked hashes above establish the available current source/PDF pair; the applicable pages are now 52–54.
- **Proposed fix:** no manuscript fix or new build is required. In a later authorized coordination update, append this exact source revision, PDF hash and current page range to `thesis-work.md` §7, preserving the dated 4 October check as historical. Do not relabel the frozen resumption PDF or its acceptance record as current.

### F2 — L10-N is rendered with the promoted fixed-ten-facet scope

- **Status:** solved. **Severity:** none.
- **Exact locations:** `thesis/chapters/07-hko.tex:65` (Theorem 7.8), `:67` (geometry and invariants), `:73` (Hausdorff neighborhood), `:215` (scope limits); `docs/consolidation/2026-09-24/claim-disposition.md:6`; `topics/variation-and-local-maxima/hko-local-maximum/non-hko/PROOF.md:32`.
- PDF p. 52 states simplicity, ten genuine facets, 24 vertices, volume 1/2, capacity 1, ratio 1, and a local ratio bound among convex four-polytopes with exactly ten genuine facets. PDF p. 54 excludes added facets, global uniqueness and a new violation of Viterbo's inequality; its affine nonequivalence argument uses 24 versus 25 vertices. This matches the September promotion and the packet's scope (`non-hko/README.md:26`; `non-hko/PROOF.md:543`).
- **Proposed fix:** none required; retain the fixed-ten-facet hypothesis and the concluding exclusions when revising this theorem.

### F3 — L10-N+ equality family and its local exhaustiveness are supported

- **Status:** solved. **Severity:** none.
- **Exact locations:** `thesis/chapters/07-hko.tex:78`, `:84`, `:96`, `:110`, `:203`; `docs/consolidation/2026-09-24/claim-disposition.md:7`; `topics/variation-and-local-maxima/hko-local-maximum/non-hko/PROOF.md:157`, `:219`, `:230`, `:348`, `:517`.
- PDF p. 52 displays exactly the packet's hexagon/quadrilateral product E(v,s), parameters u=1−v+v², a=1/2−s, b=1/2+s, t=v−1/2, and base (v,s)=(1/2,1/12). Its factor areas are 1 and 1/2. The complete product calculation supplies capacity 1 and ratio 1 near the base; the 624 orders, 12 tight cases and base gap 1/14 appear on pp. 52–53. Independence of two equality directions modulo the fifteen symmetry directions is the packet's rank-17 statement. Strict decrease off z=y=0 plus the full chart, printed on p. 54, supports “equality holds precisely” locally. This does not assert uniqueness outside that neighborhood.
- **Proposed fix:** none required; preserve both the separate product tightness calculation and the transverse strict-decrease argument if shortening the exposition. Upper sections alone would not prove equality.

### F4 — The −13/432 upper-section caveat survives rendering

- **Status:** solved. **Severity:** none.
- **Exact locations:** `thesis/chapters/07-hko.tex:102`, `:133`, `:140`; `docs/consolidation/2026-09-24/claim-disposition.md:7`; `topics/variation-and-local-maxima/hko-local-maximum/non-hko/PROOF.md:452`, `:460`, `:476`; `non-hko/verification/20260924-independent.json:53` (all packet-relative short paths below use the same non-HKO directory).
- PDF p. 53 prints V(A*+zW)=1/2+13z²/1728, Q_j(z)=1/2, and U_j''(0)=−13/432, immediately followed by the explicit statement that this is curvature of feasible upper sections, not an asserted second derivative of actual capacity. PDF p. 52 defines U=1/(8VQ²) as an upper bound for the actual ratio. These agree with U(z)=864/(864+13z²) in the packet. No actual-capacity Hessian is asserted.
- **Proposed fix:** none required; keep the caveat adjacent to the curvature display and keep U distinguished from capacity and the actual ratio.

### F5 — Finite algebra is not substituted for the analytic neighborhood implication

- **Status:** solved. **Severity:** none.
- **Exact locations:** `thesis/chapters/07-hko.tex:144`, `:174`, `:198`, `:207`; `topics/variation-and-local-maxima/hko-local-maximum/non-hko/README.md:16`; `non-hko/PROOF.md:385`, `:404`, `:483`, `:530`; `non-hko/verification/20260924-original-summary.json:10`; `non-hko/verification/20260924-independent.json:55`.
- PDF pp. 53–54 continue contacts and W along the equality family, establish constant rank 22 using family identities, continue a positive relation, separate the regions ||y||≥Mz² and ||y||<Mz², and add the fifteen symmetries to obtain a rank-40 local chart and the fixed-facet Hausdorff conclusion. The closing paragraph explicitly assigns finite predicates/rational identities to exact arithmetic and their geometric implication to the analytic argument. It disclaims a numerical neighborhood radius. The retained September PASS summaries agree on the finite counts/ranks and curvature; this review read those retained outputs and did not rerun them.
- **Proposed fix:** none required; preserve the family identities, both mixed-direction estimates, chart argument, and trust-boundary paragraph together.

### F6 — No unsupported stronger equality or varying-facet claim remains

- **Status:** withdrawn. **Severity:** none.
- **Exact locations:** `thesis/chapters/07-hko.tex:78`, `:191`, `:203`, `:215`; `topics/variation-and-local-maxima/hko-local-maximum/non-hko/PROOF.md:517`, `:530`, `:537`.
- Potential objections that the displayed family establishes only equality examples, or that the theorem silently covers facet addition, do not survive comparison: strict transverse decrease supplies local exhaustiveness, and facet addition is expressly excluded. Later HKO F≤11 research mentioned at `meta-topics/project-management/thesis-work.md:281` is a separate result and does not change this non-HKO theorem's selected scope.
- **Proposed fix:** no weakening of “precisely” or expansion to eleven facets is justified by this review. Retain the current local qualification.

## Inspection limits and handoff

Inspected the assigned local checkout, the four requested starting sources, selected Chapter 7 source and relevant fragments/interfaces, the current PDF identity manifest and all three relevant rendered pages, and retained rational/analytic packet evidence. No external literature or discovery producer was rerun. The existing non-HKO witness/verifier files and retained evidence are available through the maintained packet; this report does not independently certify all of their code or historical execution provenance.

The four pre-existing dirty paths remained `docs/file-viewer.md`, `meta-topics/project-management/current.json`, `scripts/serve-files.py`, and `scripts/test_serve_files.py`; none was modified by this review. No coordination/dashboard updates were made under the read-only instruction. Supporting SHA checks, extracted text and page rasters are in `/tmp/msc-math/opportunity-chapter7-claims/20261022T200002Z/`.

[review: /root/chapter7_message_review]
