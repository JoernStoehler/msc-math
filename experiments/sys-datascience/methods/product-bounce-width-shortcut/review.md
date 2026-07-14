# Independent packet review

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

Strongest supported statement: conditional on the repository's cited
shortest-billiard/non-translatable-polygon correspondence, the two-bounce class
action is exactly the difference-body inradius in `DERIVATION.md`.  The proof
is exact and the implementation has 20/20 exact retained checks; Jörn has not
reviewed or accepted the theorem.  Route the interface through source-level or
Jörn review before thesis use.  The full-table decomposition is descriptive
post-target bookkeeping and does not independently predict the bounce label.
