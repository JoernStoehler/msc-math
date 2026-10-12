# Provenance and packet boundaries

Prepared 18 September 2026 for Jörn's manual upload to ChatGPT Pro. The task
was selected by Jörn: pursue mathematics beyond rotated regular pentagon
products. This packet is preparation, not a claim to have obtained a new
extension, and is not an instruction to publish or submit anything.

## Which proof is included

`math/rotated-pentagon-proof.tex` is an exact byte copy of the project's
`docs/pentagon-proof/fresh-proof.tex`. It is the single self-contained proof
exposition included here: explicit conventions, witness, symmetries and
normalization. It can be read directly as text; no figure or external input
is required. It is also a standalone LaTeX source.

The original short analytic argument is in
`docs/pentagon-proof/analytic-proof.tex`. The later thesis replacement in
`docs/pentagon-chapter-v2/chapter.tex` carries the same argument with citation
and convention detail. Its README records that Jörn accepted the underlying
mathematics while rejecting the earlier exposition, and does not claim human
approval of the rewritten chapter. The interpolation insight came from an
AI-assisted Pro review; the explicit feasible word already existed in the
thesis. No separate human approval of the exact fresh-proof.tex wording is
inferred. These near-duplicate expositions are not included in the upload.

The packet preparer compared fresh-proof.tex with analytic-proof.tex and the
replacement chapter for theorem, endpoint normalization, witness and symmetry
argument. This is a consistency check, not a new full audit of the imported
Haim–Kislev or HKO theorems.

`math/product-six-facet-reduction.tex` is an exact copy of
`formal/product-qp-six-facet-reduction.tex`. Its source records independent
agent review and Jörn's mathematical acceptance on 28 July 2026. Earlier
population observations motivated/checked the reduction but are not its proof.
This file is a thesis fragment, not a standalone build. CONTEXT.md supplies its
referenced active-word conventions, so understanding it does not require the
rest of the repository.

## Hashes and omissions

`source-hashes.json` identifies exact included originals and hashes the
additional local sources consulted for provenance/conventions. A null packet
path means the original was deliberately omitted, not a missing dependency.
Source copies were taken from the active repository working files, including
uncommitted proof updates, rather than assuming a historical commit contained
the current mathematics. Relative original paths are provenance identifiers,
not files that Pro needs to locate.

No repository snapshot, datasets, solver, plots, session logs or third-party
paper PDFs are included. No new numerical experiment was run for this packet.
Primary paper links in SOURCES.md require external access; their relevant
inputs to the supplied proof are stated in the included mathematics.
