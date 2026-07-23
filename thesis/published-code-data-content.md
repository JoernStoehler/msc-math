# Availability of Code and Data Companion

Status: chapter-local purpose, source map, and review state for
`thesis/12-published-code-data.tex`. Not source truth.

## Reader purpose

Jörn's 2026-07-15 review reset the chapter's purpose. Kai and the other thesis
readers need to understand what mathematical research material will be
available to future researchers, what that material permits them to check or
continue, and what is not preserved or promised. They do not need a release
manual in the thesis.

The chapter follows Numerics, which already distinguishes exact verification
from floating-point evidence. It applies that distinction to availability:

- exact verifier source, finite inputs, and outputs remain available for direct
  checking of the theorem-facing finite predicates;
- retained empirical data and provenance remain available for reanalysis and
  continuation without acquiring theorem status;
- code, sources, notes, and environment definitions support later development;
- exclusions and reproduction limits prevent "available" from being read as
  "complete process archive" or "universally byte-reproducible."

## Deliberate exclusions from thesis prose

Repository paths, command sequences, packager and manifest implementation,
Python assertion modes, artifact hashes, detailed comparison procedures, and
packet-local operational caveats belong in `docs/reproducibility.md` and packet-local
READMEs. Mention an implementation distinction only when it changes the
mathematical trust boundary, as with untrusted HKO witness generation versus
exact SageMath verification.

The previous four-page candidate at commit `94db62a5` was rejected as the
chapter structure because it answered repository-audit questions more than the
reader's availability question. Its source audits remain useful evidence; its
organization and operational detail are superseded.

## Source and claim map

- Publication and archive outcome: `docs/project-facts.md` items 5, 6, and 6.1;
  `submit/README.md`; `submit/archive-closure-checklist.md`.
- General availability and data policy: `docs/reproducibility.md`.
- Rights and exclusions: `LICENSE.md` and
  `submit/archive-rights-and-exclusions.md`. Final path selection and notices
  remain closure work; do not describe rights review as complete.
- HKO exact material: `experiments/hko-local-maximum/theorem/README.md` and
  `thesis/07-hko-local-maximum-exact-certificate.tex`. Rust selects finite
  choices; Sage verifies the exact predicate used by the hand argument.
- Rotated-pentagon exact material:
  `experiments/regular-products/pentagon-rotation-formula-proof/README.md` and
  `thesis/09-rotated-regular-polygons-exact-certificate.tex`.
- Bounded data-science material: `experiments/sys-datascience/README.md`, its
  producer/prepare/method owners, `thesis/08-black-box-datascience.tex`, and
  `thesis/a-datascience-results.tex`. Preserve the finite-distribution and
  generated-rule boundaries.
- First-order panel:
  `experiments/sys-landscape/gradient-ascent-observed-general/README.md` and
  `thesis/06-first-order-perturbations.tex`. It supports finite-step progress
  and recorded cost on twelve fixed starts, not an endpoint or local maximum.
- Figure availability is governed by the thesis and experiment asset owners;
  publication of a producer does not turn an exploratory figure into proof.

## Archive placeholder

The only literal value the current prose chooses to expose is the final Zenodo
DOI and record URL. Archive filename, size, release commit, hashes, publication
date, and exact environment identities belong in archive metadata and
repository documentation unless a later reader need justifies adding them to
the thesis.

## Review status

The clean-sheet availability draft was reviewed on 2026-07-15:

- `latexmk` and `check-build.sh` completed cleanly;
- candidate PDF pages 70--72 were inspected with Numerics and the opening of
  the AI chapter at normal whole-page scale;
- a fresh mathematical reader understood the exact and empirical material
  available for checking and continuation, together with the exclusions and
  reproduction limits;
- the reviewer found no material source-strength, adjacent-chapter duplication,
  operational-clutter, reader-understanding, or rendered-presentation issue.

Reopen the purpose if operational explanation again occupies more of the
chapter than the future-researcher availability account, or if final archive
selection makes any stated availability or exclusion false.
