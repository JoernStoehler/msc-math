# Proposed candidate assembly — 18 September 2026

**An 87-page integration draft is built and reviewable. It is not a prose PASS,
a release, or an authoritative coordination/ownership map.** No main merge,
publication, or alteration of the original interrupted checkout occurred.

From this repository root:

```sh
sh thesis/candidate/build.sh
```

Output: `thesis/candidate/build/main.pdf`. This worktree is the temporary
proposed integration home, not a selected permanent release path. Its sources
and ordinary TeX dependencies are sufficient for this PDF build; other
worktrees are not build inputs.

## Preserved baseline and exact inputs

Base commit `d720953a`; snapshot commit `8aff3321` records the actual interrupted
candidate rather than silently using the clean 86-page source tree. All 63
recorded inputs (baseline, current, and two unwired alternatives) were SHA256
checked against the assembly audit before copying. The snapshot preserves both
unwired alternatives, but selects the same recovered preliminaries as before.

- [Interrupted source manifest](interrupted-input-manifest.json): inherited
  exact hashes, source selection and provenance from assembly audit `90f50f81`.
- [Final build manifest](build-manifest.json): exact 43 build-input hashes,
  proposed PDF hash, baseline PDF hash and build/render checks.
- Original interrupted location:
  `/tmp/msc-math-thesis-review-20260917/thesis/candidate/` (untouched).

The old 86-page PDF did not include the interrupted overlays. The new 81-page
count reflects different included material, especially the analytic rotation
proof and removal of the old pentagon appendix already selected in interrupted
`main.tex`. A smaller page count is not evidence of better writing.

## Adopted changes

| Input | Adoption and evidence |
| --- | --- |
| `c69a4ee8`, QP/flow proposal | Exact prerequisite hashes checked; patch applied cleanly. QP now explicitly derives both inequalities from duality/simple minimizer, and existing repaired flow overlay is selected. Sources match proposal bytes. Independent QP check `cd1673d2` and runtime/example check `66b0f281` are upstream evidence; not rerun or inflated into general algorithm correctness here. |
| `2b0ee807`, numerics correspondence | Attribution patch applied. Added actual four-test scope and repository test/report pointers. Development-route populations are not relabeled production audits. |
| `a30c3524`, variation | Thesis patch applied, with source paths for the 64-start/448-run comparison and separate three-case finite-distance diagnostic. Rust comment/test-comment changes are **not adopted** in this manuscript-only integration. |
| `f54cd3d9`, literature | Corrected false EHZ/cylindrical-open claim; copied its bibliography entry. This does not import the later open-literature review wholesale. |
| `201fd49a`, `14401ef4`, `9171a94f`, pentagons | Maintained formal source, source-convention contract, original return/audits, and provisional affine section retained. Affine section follows unchanged short rotation proof. Historical exhaustive CAS route is mentioned; local ten-facet HKO theorem stays separate. Optional further polygon results remain only in research source. |
| `2c57169a`, reproduction routing | Scoped build-entry patch applied. It explicitly describes candidate build and distinguishes the older release route. |
| `e6ac0f15`, historical DS recovery | Retained source-recovery receipt/scripts copied. Availability text acknowledges local original-geometry recovery without claiming public release, recovered execution receipts or current certification. |
| Mechanical dependent edits | Abstract/introduction/conclusion distinguish analytic rotation, affine extension and original CAS provenance. Introduction uses symbolic section refs. Disclosure attributes new Pro argument without claiming Jörn personally verified it. Candidate/formal READMEs reflect proposal wiring. |

## Deferred work and decisions

- **DS:** the complete chapter and appendix from `b2c5db9c` plus `b2ba0638`
  are now selected. This is feedback-guided repair of the existing full chapter,
  not an independent lean-context authoring trial. All selected scientific
  strands remain; methods and detailed provenance partly move to the appendix.
  Final prose quality, emphasis and remaining experimental-accounting density
  are unvalidated. No further empirical results are silently selected.
- **Preliminaries:** existing recovered version remains selected. New top-level
  alternative is preserved but is not assumed superior or activated.
- **New empirical deductions:** not imported automatically. Scientific selection
  and synthesis remain owned by the root and empirical desk.
- **7×7 / 5×7 and other Pro extensions:** preserved in formal material, not added
  as thesis theorems.
- **Final exposition:** every adopted source is provisional in prose quality.
  Abstract/conclusion received mechanical consistency edits, not a final
  synthesis. AI reflection, diagrams, and long-range explanatory dependencies
  still need reader-facing review.
- **Technical evidence:** finite passing tests do not prove the enumeration,
  all-input numerics, every derivative, or inherited historical target values.
  Current DS pilot/revalidation outcomes are intentionally not anticipated.
- **Publication/release:** no archive, upload, university submission, main merge,
  or permanent integration-home selection is part of this task.

## Checks performed

1. Input hashes checked before adoption; `git diff --check` passes.
2. Full `sh thesis/candidate/build.sh` succeeds: **81 pages**, final log has no
   warnings, undefined references/citations, or overfull boxes.
3. All 81 pages rendered and inspected in contact sheets; changed mathematical
   and source-attribution areas inspected more closely. No clipping/overlap
   noticed. Sparse inherited pages 40 and 48 remain; this is not a final layout
   or prose approval.
4. Independent fresh source reviewer compared main selection and patch contents:
   no accidental source omission; exact QP/flow match; affine source-manifest
   hashes verified; rotation/affine conventions align. Reviewer detected stale
   integration notes (repaired) and stale DS interpretation (subsequently repaired
   by the bounded ridge-identity addition).
5. Build manifest records actual local TeX/figure/bibliography inputs. No other
   checkout is required to render this candidate.

## Follow-through

Root/Jörn review this candidate and the separate coordination map before
choosing a permanent integration surface. A subsequent integrator can apply
the scoped commits rather than reconstructing the interrupted session. This
active worktree retains unique reviewable work and is not disposable scratch.

## Bounded DS consistency follow-up

Source: `docs/ds-evidence-closure/ridge-mathematics.md` from `0e0ccb9c`.
The new equation `eq:ds-pentagon-ridge-capacity` states
`sys(K_theta) S(K_theta)^2 = 16(3+sqrt(5))` for regular rotated pentagon
products, with the same normalized total unsigned ridge sum already defined
in the DS chapter. Its derivation counts mixed-edge pairings, evaluates the
absolute-cosine sum, and invokes the existing rotation-profile theorem.
No general product law, empirical causality or discovery chronology is asserted.
Section 8.11 and introduction/conclusion now point to the restricted result;
the AI disclosure again points to its actual included equation.

## Prior-art and availability closure

Adopted the source-checked proposals `f4b36cee` and `94d5974f` after reading
their reports. Section 4.4 now compares closure-vertex enumeration with
Krupp–Rudolf's polygonal Minkowski billiard algorithms; Section 6 compares
facet-row variation with Leipold's simplex/linear-image optimization. Neither
passage claims novelty or a speed advantage. Both bibliography entries are added.

Availability now pins the checked public commit and distinguishes public packets
from local candidate/research additions. Stale snapshot/link counts were removed.
The HKO listing names its source and states that assertions require Python
optimization disabled; this records prior execution, not a fresh certificate run.
The source-check reports and exact proposal are retained locally under
`docs/algorithm-prior-art-closure/` and `docs/availability-claim-closure/`.

## User-selected product-position addition

Adopted `5a47eb2c` after reading the complete two-page proof and supporting
convention record and rerunning `formal/product-rotation/verify.py`. The new
section follows affine pentagons and precedes visualization. It states the
universal capacity increase away from symplectic-product position, proves it
by mass splitting, and gives one triangle product with a smaller Lagrangian
endpoint. The combined interior-maximum implication remains a short remark.
No profile catalogue, lifted-orbit counterexample or novelty claim is added.
Introduction/conclusion and section routing mention this selected result.

The rational check passes: six prefix cases, 120 cyclic orders, feasible
witness and rotation-generator identities. It supplements the reviewed argument;
the code does not prove the arbitrary-convex-body approximation step.

Latest product-position build: **83 pages**, no warnings, undefined references
or overfull boxes. The new section is on pages 66–67; those and adjacent pages,
contents, introduction, availability and conclusion were rendered/inspected.
Earlier 81-page checks above remain dated integration-stage evidence.

## Completed historical DS restoration

Adopted revalidation head `6656ff7a`, including the separate interval adapter
and all tracked source/feature/current-computation receipts. The scalar adapter
snapshot matches `3022f15d`. Current capacity coverage is 14,335/14,336:
14,289 accepted scalars at requested tolerance and 46 wider intervals; one
body remains outside the ordinary size policy. Numerical volume does not
supply certified systolic-ratio intervals. Full feature restoration covers
all 14,336 bodies and preserves the common historical table cells.

The full DS appendix points to previously untracked method packets. To make
those references resolve, this candidate retains the paired-tangentialization,
orientation-allocation and diagonal-CEM evidence, plus their existing scalar
evaluator, copied without changes from main. `empirical-packet-snapshot.json`
records every copied hash. Only the old compiled CEM executable under a nested
`.git` directory was omitted (hash recorded); its source and raw results are
preserved. This is evidence preservation, not a new computation or public release.

## Complete DS integration checkpoint

Selected the full chapter and appendix, rather than the previously reviewed
ridge-only subsection, from `b2c5db9c` with source-order fix `b2ba0638`. Added
only its new bibliography resource, preserving the assembled baseline resources.
Its two readable ridge figures contain all historical observations. The existing
restricted-identity label is preserved, so introduction, conclusion and disclosure
remain resolvable. Availability reflects completed interval recovery, separate
from historical values and numerical volume.

[Independent checkpoint](ds-independent-check.md) records final source hashes
and the second review's disposition of concrete human-feedback and coverage
findings. No source/contract integration blocker remained in that bounded review.
This is not a human prose verdict or evidence of autonomous authoring success.

Final PDF is **87 pages**. Build has no warnings, undefined references/citations
or overfull boxes. All chapter/appendix evidence paths resolve locally. Complete
render inspected before a TOC-only compaction; final TOC is two pages rather than
a third page holding only two entries. Final figure render remains readable.

Remaining required work: reader-facing review and correction of unacceptable
prose, final whole-document synthesis, and final delivery selection. Known
numerical limitation: one original body lacks current capacity coverage; the
text names it accurately, so completion does not require concealing or solving
that case. Optional further empirical/Pro results remain unselected. No new
scientific decision was forced by this integration. Root retains those choices.

## Finite known-defect checkpoint after 476facfd

The exact 87-page review PDF is preserved at `review-freeze/candidate-476facfd-87pages.pdf`; its hash is in the adjacent manifest. Existing DS preview review artifacts were not modified.

Adopted from narrow-flagger inventory c7f742b7: VR1 now reports the matched-start finding (multi-branch implementations had larger median terminal values; all 448 below one), links the integrated DS subsection, and preserves historical-objective/separate-panel scope. NR3 moves two source paths into a footnote without losing fixture scope. IR2 removed the unsupported flow performance comparison; the replacement accurately describes local passage geometry and the conditional finite-search cross-check.

Independent whole-manuscript audit ea283d24: corrected the introduction's self-contained-proof promise, explicitly restricted the conclusion quantifier to ten-facet polytopes, renamed product-position matrices to avoid the global J_0 collision, and replaced the flow chapter's false availability destination with existing source/test/example paths in a footnote. The conclusion also distinguishes our flow search from literal implementation of the whole Chaidez–Hutchings construction.

Not adopted: IR1, IQ1/IQ2, NR1/NR2, NQ1/NQ2/NQ3, VQ1/VQ2 blanket removals/relocations. These are unvalidated presentation suggestions; the current framing, arithmetic assumptions, branch-coverage warning and scope limits have legitimate roles. No new science or authoring workflow test was performed.

Remaining: the scoped HKO nonsmooth cutting-literature comparison identified by the independent audit is not integrated; current smooth Zoll comparison alone is incomplete related-work coverage (see `open-thesis-literature` extrema report). Whole-thesis prose acceptance remains unvalidated; empirical accounting detail and inherited sparse pages remain. The exact reviewed DS repairs do not establish a general authoring workflow. No research or producer is running under this task.

## Independent coordination checkpoint, 18 September after 14:27 UTC

The following bounded repairs supersede the corresponding open items above:

- `5a970ca7`: primary-source-checked Haim--Kislev cutting comparison and the
  introduction's restriction to transverse directions.
- `9224c8eb`: explicit bilinear-form convention in the product-position result.
- `628b33ee`: geometric interpretation of the affine equality class, independently
  checked; development/production distinction before numerical audit counts.
- `279ad991`: verified access dates for five active bibliography entries.

The integrated build remains 87 pages and has no warnings, undefined references
or overfull boxes. Changed introduction, affine equality, product-position and
numerics pages were rendered and inspected. These are integration checks, not
whole-thesis acceptance.

The separately authored DS trial at `thesis/candidate/ds-result-trial/` is
**not selected** by `main.tex`. Its fixed review PDF and received human feedback
are in `docs/ds-result-trial/`. It has a bounded scientific review and is being
evaluated as exposition, not treated as an accepted writing workflow.

Jörn clarified the acceptance standard: ready for submission, with only minor
problems expected from Kai that Jörn could fix in about two hours by hand;
borderline cases fail. No university submission or publication is authorized.
