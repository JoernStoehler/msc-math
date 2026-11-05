# Independent review: affine regular-heptagon theorem

Reviewer: `/root/regular_review`, 17 October 2026. Receiving owner: `/root`; proof/certificate owner: `/root/regular`.

**Verdict: approved as an independently agent-reviewed computational proof, conditional on the stated imported capacity/sparse reduction.** No consequential gap found in the reviewed report, finite containment implication, or standard-library verifier. This is not human proof acceptance and not an assessment of novelty outside this repository. The gallery, proposed chapter text and figure are outside this review.

## Inspected sources and access

Read this worktree's `AGENTS.md`, `AGENTS.md.commentary.md`, `README.md`, `docs/research-guide.md` and collaboration rationale. Mathematical sources inspected: `patterns/regular/REPORT.md`, `certificate.py`, `heptagon-certificate.json`; the sparse reduction and width/tensor decomposition in `topics/products/pentagon-affine-products/research.tex` Sections 2–3; `topics/products/product-qp-six-facet-reduction.tex`; and `topics/products/pentagon-affine-products/source-contract.md`. Reviewed `scan.py`, `tensor.json` and the numerical discovery context for scope only. Did not inspect every exploratory sweep or re-fetch external literature; source-contract attribution remains an imported source check.

## Mathematical audit

- The sparse reduction is an existence result: finite compact maximization, linear optimization in each separate factor's normalized closure weights, and positive bilinear objective justify choosing closure vertices and factor mass one half. At most three support rows per factor then suffice. Zero-weight insertion/deletion and cyclic invariance preserve the original full-permutation HK maximum.
- For a regular heptagon there are no opposite pairs or degenerate zero-weight closure triples. Positive cyclic gaps are at most three, sum to seven, and have unordered types `(1,3,3)` or `(2,2,3)`. Each type has seven rotations. An independent exact integer-gap enumeration of all 35 triples gave exactly the fourteen rotations of `(0,3,4)` and `(0,2,4)`.
- For each closure pair, cyclic cutting at its distinguished q-step leaves exactly two q-orders times six p-orders. No reflection quotient is needed. Independent rotations preserve the signed rank-one atom set; consequently the 48 cases cover all fourteen-by-fourteen closure pairs and their twelve interleavings.
- A four-block objective is `x·y/2`; a six-block objective is `(x0·y0-x2·y1)/2`. The width body is `D/(1+h)`. The verifier's scaled steps `gamma*n*(1+h)/h` are exactly the true normalized steps divided by width radius `s=1/(1+h)`. Its target is therefore exactly the report's `w²(x0*y0^T-x2*y1^T)`. There is no lost factor of two.
- Each four-atom exact solution has coefficient absolute mass less than `24/25`. The centrally symmetric atom hull contains zero, so the triangle inequality for its gauge proves strict tensor containment. Together with the width-step lemma, which realizes every four-block generator by a feasible distinct-facet word, this supplies both sides of the capacity equality; finite upper witnesses alone would not.
- Under independent nonsingular maps, both tensor bodies transform by `T -> G^{-T} T H^{-1}`. Pairing with identity gives `n_i^T G^{-1}H^{-T} n_j`, confirming the reported order of the matrix factors. Negative determinants cause no change to containment.
- The ratio is `(w^4/(2 S7²))*|det M|/m(M)²`. `L D` is contained in `D°` for `L=M/m(M)`, and `D°=sec(pi/14) R_(pi/14) D`. Area comparison proves the sharp bound. Equality of nested convex-body areas forces equality of bodies. Their resulting linear automorphism permutes the polygon vertices and cyclic adjacency, hence is one of its orthogonal dihedral symmetries. Conversely `M=t R_(pi/14)U` attains equality. The displayed sharp value around `0.9174081964` is below one with a substantial exact interval margin.

## Verifier audit and execution

Reviewed directed rounding on signed rational inputs, interval multiplication/division, Machin's formula with alternating arctangent bounds, Taylor recurrences of degrees 99/100 and the common order-101 Lagrange remainder. Every evaluated angle is asserted to lie below seven in absolute value. The sine degree-99 polynomial is also the degree-100 Taylor polynomial because its even coefficient vanishes, so the stated order-101 remainder is valid.

The cross-product barycentric formula gives the exact closure weights; interval positivity is checked. Gaussian elimination uses only pivots whose intervals exclude zero; interval operations enclose the corresponding exact elimination for the same selected pivots, proving invertibility and enclosing the exact solution. It does not trust LP coefficients or floating hull facets. The final set equality plus list length 48 establishes complete, nonduplicated case coverage.

Executed the updated standard-library verifier independently:

`python patterns/regular/certificate.py verify patterns/regular/heptagon-certificate.json`

Result: PASS, all 48 cases; maximal coefficient-mass upper numerator `1399807903582040072072266258464949268200001493163` over `2^160`, below `24/25`. Sharp systolic interval upper endpoint `670396790548552512047309409545618673456723681395 / 730750818665451459101842416358141509827966271488`, strictly below one. A separate 70-case exact-rational sign/zero arithmetic sanity check of interval addition, multiplication and division passed; the proof of interval validity rests on the code audit rather than these finite tests.

## Limits and wording

The theorem concerns products of independently affinely regular heptagons. It does not prove the general odd-regular containment conjecture or classify arbitrary heptagons. Its optimizer conclusion should be that six-block competitors do not improve the four-block capacity value; the sparse reduction does not classify every nonsparse optimizing orbit. The report already states the relevant existence distinction. No expensive numerical reruns, installation, network or project-source edits were performed in this review.

## Exact prepared report review

Read `.agents/skills/message-review/SKILL.md` and the complete current `patterns/regular/REPORT.md`, including final figure-parameter paragraph. **APPROVE regular-report-v1**, defined as the report with trailing whitespace normalized to one blank line followed by `[review: /root/regular_review]` and a final newline. SHA-256 of those complete approved bytes: `154c01b44b19fe0ae0592cd7134d70aeae24c7533f72ff931d59084391d6d9e3`. No unresolved consequential objection. The report clearly distinguishes the finite predicate, imported reduction, full affine consequence, limited family, human acceptance and external novelty. Root owns adding the marker, integration and commit; this reviewer preserves the skill's read-only project boundary. No substantive edit or test rerun was needed for this wording review.


APPROVE regular-report-v2. Native reviewer /root/regular_review reviewed the complete current report, including its marker; display-delimiter changes leave mathematics and wording intact.
