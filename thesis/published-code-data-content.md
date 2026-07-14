# Published Code and Data Companion

Status: chapter-local maintenance and source map for
`thesis/12-published-code-data.tex`. Not source truth.

## Purpose and boundary

The chapter lets a mathematically capable reader understand what is published,
how the principal computation-backed claims can be checked, and what the frozen
archive does and does not promise. It follows the numerics section, which has
already distinguished exact proof checks from floating-point evidence, and it
precedes the separate reflection on AI in the research process.

The chapter is an explanation, not a repository inventory or run manual.
Detailed commands remain in `REPRODUCIBILITY.md` and owner-local READMEs. It
must preserve the distinct roles of exact certificate verification, retained
empirical data, route-local comparison rules, and smoke runs.

## Current reader path

1. GitHub is the changing continuation surface; one Zenodo record is the
   frozen, citable thesis-support plus continuation archive.
2. The archive is curated, rights-bounded, and tied to a reviewed commit by an
   embedded file manifest and hashes. Retained LFS data enter as hydrated bytes.
3. Four claim-specific routes show what checking means in practice: HKO exact
   verification, rotated-pentagon exact verification, the bounded data-science
   search, and the twelve-start finite first-order panel.
4. The conclusion states why one universal rerun command, universal byte
   identity, and smoke-as-evidence would all be misleading promises.

## Source and claim map

- Archive decision and surface: `FACTSHEET.md` items 5, 6, and 6.1;
  `submit/README.md`; `submit/archive-closure-checklist.md`.
- Rights and exclusions: `LICENSE.md` and
  `submit/archive-rights-and-exclusions.md`.
- Packager behavior: `scripts/build-release.py`. It creates the embedded
  `ARCHIVE-FILE-MANIFEST.json`; there is no tracked `archive-manifest.toml` on
  current Main. The script verifies an already curated tracked tree and does
  not replace final rights, privacy, or data-value review.
- HKO certificate: `experiments/hko-local-maximum/theorem/README.md`,
  `verify.sage.py`, and
  `thesis/07-hko-local-maximum-exact-certificate.tex`. Rust generates finite
  choices; Sage reconstructs and exactly verifies the proof-facing predicate.
- Rotated-pentagon certificate:
  `experiments/regular-products/pentagon-rotation-formula-proof/README.md`,
  `executable_proof.sage.py`, its full stdout, and
  `thesis/09-rotated-regular-polygons-exact-certificate.tex`. The transcript is
  timing-bearing and the executable must run without Python optimization.
- Bounded data-science result: `experiments/sys-datascience/README.md`, its
  `prepare/README.md`, method packet owners, `thesis/08-black-box-datascience.tex`,
  and `thesis/a-datascience-results.tex`. The retained 14,336-row result and the
  generated-candidate follow-up remain distribution- and rule-specific. The
  exact generated packet is
  `methods/extreme-scalar-rejection-proposer/artifacts/100k-promising-scalars/`;
  it retains selection/evaluation rows via LFS but deliberately omits large
  geometry/feature caches, so a fresh ranking and target-field-separation audit
  requires regeneration. The retained producer data reconstruct the current
  feature schema; not every current method artifact used the older in-place
  feature table directly.
- First-order panel:
  `experiments/sys-landscape/gradient-ascent-observed-general/README.md` and
  `thesis/06-first-order-perturbations.tex`. It supports systematic finite-step
  progress and recorded cost on twelve fixed starts, not endpoint or local
  maximality. Its analyzer structurally checks recorded statuses and aggregates
  costs; it does not independently recompute every finite-step decision.
- Cross-project routing and comparison limits: `REPRODUCIBILITY.md`.

`THIRD_PARTY_NOTICES.md` does not exist on current Main. Applicable third-party
notices remain a closure checklist item, while the current material boundary is
owned by `LICENSE.md` and the archive rights audit. Do not imply that the absent
filename is already part of the release.

## Closure placeholders

Only literal values produced by the final release remain open in the chapter:

- Zenodo DOI and URL;
- archive filename and byte size;
- full release commit and archive SHA-256;
- publication date.

The prose does not name exact environment versions, so no environment-version
placeholder is needed. Replace the visible chapter placeholder block only from
the reviewed final archive record.

## Review status

Candidate review on 2026-07-14 completed the following distinct passes:

- Archive/source audit separated accepted policy, implemented packager checks,
  and still-open closure work. It confirmed that the prompt-named static
  manifest and notice files never existed; the generated ZIP manifest and the
  unfinished path-level rights/notice audit have different roles.
- Evidence-route audit checked current code, retained artifacts, owner-local
  interpretation, and thesis claims. It produced the data-science schema/cache
  and first-order analyzer calibrations now preserved above. It also confirmed
  that the HKO summary does not authenticate later verifier-source changes, so
  release-source identity plus a verifier rerun remains necessary.
- The thesis built cleanly, and pages 70--74 were inspected with the end of
  visualization, Numerics, and the opening of the AI chapter at normal
  whole-page scale. No material page-break, path, hierarchy, or transition
  problem remained.
- An independent cold reader, given only the candidate, neighbors, and rendered
  pages, reported no material reader-understanding or presentation finding.

Reopen the chapter review if archive policy or packaging changes, a named
packet or retained artifact changes materially, closure replaces the literal
placeholders, or the neighboring Numerics/AI sections change the entering or
exiting reader state.
