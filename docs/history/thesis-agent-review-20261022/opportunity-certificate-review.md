# Variation and HKO certificate navigation review

[review: /root/message_reviewer]

Review date: 22 October 2026. Checkout: `/workspaces/msc-math`, HEAD `9667e351514faf1f00e2d86796c20e9d9b6f2f10`. Task: `opportunity-certificate-review`; read-only source review within the requested 40-minute limit.

## Verdict and scope

**No surviving defect found against backlog item 2.** Its four navigation obligations are implemented in the selected sources. The finite certificate and the slope-to-neighborhood proof are both present. Their different responsibilities are stated explicitly; the certificate is not presented as a standalone proof of the geometric theorem.

I read all of `thesis/chapters/06-variation.tex`, `thesis/chapters/07-hko.tex`, its four HKO input fragments, `thesis/appendices/hko-certificate.tex`, and the complete 92-line `thesis/appendices/hko_core.py`. I traced the relevant formulas and acceptance checks in `topics/variation-and-local-maxima/hko-local-maximum/theorem/verify.sage.py`, read its packet README and retained verification summary, and checked the availability bridge in Chapter 14. The selected inputs are confirmed at `thesis/main.tex:71`, `thesis/main.tex:72`, and `thesis/main.tex:89`. The review starts from `meta-topics/project-management/thesis-work.md:61`; historical concerns were compared with `docs/history/coordination/whole-exposition-assessment.md:184` and `docs/history/coordination/whole-exposition-assessment.md:266`.

This is a source/navigation and bounded implication review, not a new certificate run, independent rederivation of all 26 derivative rows, external-paper verification, PDF layout acceptance, or whole-thesis approval. No build, experiment, or certificate producer was run. No repository files or Git state were deliberately changed. The four pre-existing modified paths were outside the reviewed manuscript sources: `docs/file-viewer.md`, `meta-topics/project-management/current.json`, `scripts/serve-files.py`, and `scripts/test_serve_files.py`. Only the requested scratchpad report and temporary review packet were written.

## Findings and dispositions

Severity describes the historical concern where one exists; **solved** means the repair is already present, not implemented during this review. **Withdrawn** means the alleged absence is contradicted by the inspected source. There are no surviving findings requiring a manuscript change.

### 1. KKT matrix and implicit-function hypotheses — solved

- **Severity:** Low/medium, closed.
- **Exact locations:** `thesis/chapters/06-variation.tex:170` defines the rank, positivity, stationarity and restricted negative-definiteness hypotheses; `thesis/chapters/06-variation.tex:218` begins the proof; `thesis/chapters/06-variation.tex:227` identifies the block matrix explicitly; `thesis/chapters/06-variation.tex:235` proves its nonsingularity.
- **Assessment:** The proof supplies both halves of the kernel argument: negative definiteness forces the weight perturbation to vanish, then full row rank forces the multiplier perturbation to vanish. The implicit-function conclusion no longer depends on an unnamed matrix.
- **Concrete proposed fix:** Already implemented: retain the displayed block matrix and the two-step kernel argument. No further edit is needed for backlog item 2.1.

### 2. The search slice is connected to the HKO symmetry complement — solved

- **Severity:** Low/medium, closed.
- **Exact locations:** `thesis/chapters/06-variation.tex:308` defines the slice by the systolic-ratio symmetries and names the HKO instance; `thesis/chapters/hko-lemma-conclusion.tex:7` defines `T ⊕ S` with dimensions 15 and 25; `thesis/chapters/hko-lemma-conclusion.tex:81` gives the actual symmetry action on rows.
- **Assessment:** The reader can identify the slice used for the search model with the later transverse complement, without inferring a new quotient construction or a 40-dimensional strict maximum.
- **Concrete proposed fix:** Already implemented: retain the cross-reference and the 40/15/25 dimension explanation. No additional definition or relocation is needed.

### 3. Singular seven-facet sections and the worked derivative — solved

- **Severity:** Medium, closed.
- **Exact locations:** `thesis/chapters/06-variation.tex:121` states the feasible-section upper-bound lemma; `thesis/chapters/06-variation.tex:157` gives the inverse-minor continuation independent of KKT regularity. The worked HKO word and position sets start at `thesis/chapters/hko-lemma-bounds.tex:61`; the minor and local section are at `thesis/chapters/hko-lemma-bounds.tex:78`; the reduced quadratic and its derivatives are at `thesis/chapters/hko-lemma-bounds.tex:101`; singularity is explained at `thesis/chapters/hko-lemma-bounds.tex:114`; the nonproduct coordinate derivative is at `thesis/chapters/hko-lemma-bounds.tex:134`.
- **Assessment:** The displayed reduced quadratic is independent of the free coordinate `u` and has second derivative `-8t` in `v`. The text therefore identifies the singular restricted Hessian while preserving the invertible feasibility minor. It also shows differentiated closure, `DQ`, vanishing volume derivative in the chosen direction, and the resulting `DA` and `DU`. A nearby optimizing branch is not needed: feasibility gives an upper bound and the imported base capacity supplies touching.
- **Concrete proposed fix:** Already implemented: preserve the distinction between a feasible section and an optimizing KKT branch, together with the displayed partial derivatives. Do not replace it by a claim of regular KKT continuation.

### 4. Why 26 bounds occur in a 25-dimensional quotient — solved

- **Severity:** Medium, closed.
- **Exact locations:** `thesis/chapters/hko-lemma-bounds.tex:22` states the exact-bounds lemma; `thesis/chapters/hko-lemma-bounds.tex:36` explains the count and the simplex containing zero.
- **Assessment:** Rank 25 and a nonzero positive relation require at least 26 rows. With exactly 26 and rank 25, the positive normalized relation gives the stated simplex geometry on the transverse dual space. The count is explained by the proof's needs, not by an asserted exhaustive list of capacity branches.
- **Concrete proposed fix:** Already implemented: retain the rank/relation count and simplex explanation. No larger branch catalogue or certificate recomputation is required.

### 5. Four finite predicate groups map to the lemma and executable — solved

- **Severity:** Medium, closed.
- **Exact locations:** `thesis/chapters/hko-lemma-bounds.tex:182` introduces the four-role predicate list; `thesis/chapters/hko-lemma-bounds.tex:199` gives the symmetry generators; `thesis/appendices/hko-certificate.tex:72` maps each group to executable line numbers.
- **Assessment:** The actual code locations match that map: `thesis/appendices/hko_core.py:40` constructs positive feasible sections, `thesis/appendices/hko_core.py:84` checks the base value, `thesis/appendices/hko_core.py:85` checks ranks 25 and 15, `thesis/appendices/hko_core.py:86` checks annihilation, and `thesis/appendices/hko_core.py:88` constructs and checks the strictly positive relation. The derivative formula at `thesis/chapters/hko-lemma-bounds.tex:170` matches the product-rule structure at `thesis/appendices/hko_core.py:58`. These observations concern source correspondence; the checks were not rerun.
- **Concrete proposed fix:** Already implemented: retain the four named groups and the executable-line map. Keep smooth continuation and the geometric implication attributed to the written constructions and lemmas.

### 6. Assignment-table and appendix-listing navigation — solved

- **Severity:** Low for the table caption; medium for the former bare continuation, closed.
- **Exact locations:** `thesis/chapters/hko-lemma-bounds.tex:160` references the numbered table and singles out entry 22's different minor; `thesis/chapters/07-hko.tex:29` fixes indexing and exact-decimal conventions; `thesis/chapters/hko-sections.tex:3` supplies the caption and label; `thesis/chapters/hko-sections.tex:9` supplies the continuation heading. Appendix listing ranges start at `thesis/appendices/hko-certificate.tex:37`, `thesis/appendices/hko-certificate.tex:45`, `thesis/appendices/hko-certificate.tex:57`, and `thesis/appendices/hko-certificate.tex:70`.
- **Assessment:** The table names solved positions as the complement of `J`, and the main text specifies the exceptional `I=(1,2,3,4,6)`. The appendix prints contiguous source ranges 1–11, 12–39, 40–68, and 69 onward with explicit starting line numbers. Both one-based facet/position labels and zero-based entry numbers are explained.
- **Concrete proposed fix:** Already implemented: retain the caption, continuation heading, exceptional-minor sentence and numbered listing ranges. Table relocation is unnecessary for this obligation. Rendered pagination was not newly checked.

### 7. Program trust boundary and public-verifier navigation — solved

- **Severity:** Medium, closed.
- **Exact locations:** `thesis/appendices/hko-certificate.tex:12` distinguishes the current packet verifier and witness from the appendix exposition program; `thesis/appendices/hko-certificate.tex:27` assigns chart/volume/upper-bound geometry to the lemmas; `thesis/appendices/hko-certificate.tex:76` distinguishes finite linear algebra from continuation and neighborhood conclusions; `thesis/appendices/hko-certificate.tex:83` specifies execution with assertions enabled. The historical-public/current-working-tree bridge is explicit at `thesis/chapters/14-code-data.tex:14` and `thesis/chapters/14-code-data.tex:25`.
- **Assessment:** Paths resolve in this checkout. The appendix is not claimed to have been included in the cited historical public commit. The public verifier separates candidate recovery from exact acceptance at `topics/variation-and-local-maxima/hko-local-maximum/theorem/verify.sage.py:259`, checks differentiated feasibility at `topics/variation-and-local-maxima/hko-local-maximum/theorem/verify.sage.py:435`, and performs the rank/positive-relation checks at `topics/variation-and-local-maxima/hko-local-maximum/theorem/verify.sage.py:497`. Its retained summary is historical run evidence, not a fresh result of this review. I did not independently test equality of every recovered public-verifier row with the appendix row.
- **Concrete proposed fix:** Already implemented: preserve the named current paths, the historical-public distinction, and the explicit finite/geometric boundary. No new availability claim or verifier execution is needed.

### 8. Alleged absence of a certificate or slope-to-neighborhood proof — withdrawn

- **Severity:** None as a current finding; the alleged proof absence is unsupported.
- **Exact locations:** `thesis/chapters/hko-lemma-bounds.tex:160` specifies the derivative bank from the assignments; `thesis/appendices/hko-certificate.tex:33` begins the complete executable listing. The geometric proof is at `thesis/chapters/hko-lemma-conclusion.tex:7` (uniform negative slopes), `thesis/chapters/hko-lemma-conclusion.tex:51` (strict slice decrease), `thesis/chapters/hko-lemma-conclusion.tex:75` (symmetry decomposition), and `thesis/chapters/hko-lemma-conclusion.tex:113` (Hausdorff theorem and equality).
- **Assessment:** Rank and annihilation identify the common kernel as `T`. The positive relation forces a negative slope in every nonzero direction of `S`; compactness of its unit sphere makes the bound uniform. A common Taylor remainder for finitely many smooth upper bounds yields strict decrease on the slice. The inverse function theorem adds the 15 symmetry parameters. The polarity/vertex-labelling argument at `thesis/chapters/hko-lemma-geometry.tex:28` covers nearby exactly-ten-facet bodies. These are actual written implications, not additional CAS predicates.
- **Concrete proposed fix:** Withdraw any “missing certificate” or “missing neighborhood argument” finding rather than adding a duplicate proof. Preserve the existing proof chain and the exactly-ten-facet restriction at `thesis/chapters/07-hko.tex:16`.

## Finite predicates versus geometric implication

The certificate's role ends with positive feasible base weights and invertible minors, matching base action, correctly differentiated section rows, symmetry rank 15, derivative rank 25, symmetry annihilation, and a strictly positive normalized relation. It does not establish Hausdorff chart coverage, smooth volume, the capacity formula itself, a uniform neighborhood radius, or the equality classification by executing code.

The written proof supplies those needed implications in order: local row chart and smooth volume (`hko-lemma-geometry.tex:20`, `:83`); positive smooth feasible upper sections and base touching (`hko-lemma-bounds.tex:47`, `:119`); uniform transverse slopes (`hko-lemma-conclusion.tex:7`); common-neighborhood strict decrease (`:51`); symmetry coordinates (`:75`); and the fixed-ten-facet Hausdorff conclusion (`:113`). Paths abbreviated in this paragraph are under `thesis/chapters/`.

The separate non-HKO subsection was also read. Its 24-section/rank-22 certificate and two-parameter equality family are explicitly separate from HKO's 26-section/rank-25 certificate (`thesis/chapters/07-hko.tex:122`). The number `-13/432` is expressly upper-section curvature, not a Hessian claim about actual capacity (`thesis/chapters/07-hko.tex:140`). Constant rank along the equality family, the mixed-direction estimate, and the final chart argument are written at `thesis/chapters/07-hko.tex:144`, `:174`, and `:198`; the packet and analytic-proof route are identified at `thesis/chapters/07-hko.tex:207`. This review does not independently recertify that separate packet.

## Handoff

Leave backlog item 2 closed. No source edit or certificate recomputation is proposed. This disposition is limited to the inspected navigation and proof interface; it does not assert human acceptance or extend the selected theorem to added facets or global maximality.
