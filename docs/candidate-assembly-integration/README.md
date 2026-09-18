# Proposed candidate assembly — 18 September 2026

**An 81-page integration draft is built and reviewable. It is not a prose PASS,
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

- **DS:** complete recovered chapter and appendix remain selected. The reviewed
  subsection revision is not a whole-chapter replacement. In particular the
  closing statement that a restricted relation remains to be formulated is
  stale relative to the known pentagon identity. Introduction/conclusion's DS
  interpretation also awaits selected contemporary content. Do not call this
  whole-document scientific coherence closed.
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
   integration notes (repaired) and the deferred DS interpretation (recorded).
5. Build manifest records actual local TeX/figure/bibliography inputs. No other
   checkout is required to render this candidate.

## Follow-through

Root/Jörn review this candidate and the separate coordination map before
choosing a permanent integration surface. A subsequent integrator can apply
the scoped commits rather than reconstructing the interrupted session. This
active worktree retains unique reviewable work and is not disposable scratch.
