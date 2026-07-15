# Historical independent packet review

This review records the packet's pre-merge assessment and is retained as an
independent review record.  Its derivation verdict has been superseded as
current mathematical authority by `formal/product-two-bounce-class.tex`, which
contains the stronger merged proof at agent-reviewed, not Jörn-reviewed,
status.  The review remains relevant to the method-local exact implementation,
its artifacts, and its bounded 20-row cross-bucket validation.

Verdict: pass after provenance/status repairs; ready for research reuse as a
conditional theorem target and bounded implementation smoke.

A fresh read-only reviewer checked the mathematical derivation, exact polygon
implementation, artifacts, source interfaces, and interpretation boundary.  It
found no material mathematical or implementation error.  In particular, it
accepted the two-point translatability equivalence, boundary reduction,
support-function factor, containment reformulation, exact max-of-affine
enumeration, `(q1,q2,p1,p2)` factor extraction, and independence from the
word/KKT target search under the stated interface.

The reviewer found two documentation/provenance defects:

1. The README did not distinguish the 20-row independent geometry smoke from
   the 10,240-row target-derived association bookkeeping and omitted the
   stopped all-row attempts.  The README now names both artifacts, commands,
   numeric sources, denominators, compute boundary, and prohibited readings.
2. The smoke artifact predated the final analyzer's association-only metadata,
   while the association artifact recorded ephemeral prepared-table paths.
   `artifacts/provenance.json` now identifies all hashes, explains the exact
   source compatibility and non-byte-identical regeneration boundary, and
   makes fixed hashes/rebuild contracts authoritative over `/tmp` paths.

The requested smoke regeneration was intentionally not run: aggregate compute
had approached the packet cap.  This leaves a transparent provenance
limitation, not a mathematical or metric ambiguity.  Future refreshes should
regenerate the smoke from the then-current analyzer only when another consumer
already justifies execution; completion of all 10,240 independent exact checks
is not required to establish the formula.

Current strongest statement: `formal/product-two-bounce-class.tex` proves the
two-bounce class formula at agent-reviewed, not Jörn-reviewed, status.  This
packet's exact implementation has 20/20 retained checks across its bounded
two-rows-per-bucket surface; that smoke is not exhaustive.  The full-table
decomposition is descriptive post-target bookkeeping and does not independently
predict the bounce label.
