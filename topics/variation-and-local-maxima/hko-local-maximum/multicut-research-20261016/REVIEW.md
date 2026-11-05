# Independent review receipt

16 October 2026. Reviewer: native agent `/root/hko_map`; recipient and packet
producer: `/root/facet_research`. This receipt preserves the reviewer's native
messages. It was transcribed by the producer; it is not a new producer claim
of independent review.

The first review identified the ambiguity between compact sets of good
polar-boundary points and whole closed polar faces. The producer corrected
PROOF.md §5 to say “Compact sets of polar-boundary points each exposing a
primal vertex or edge are covered by (T).” The reviewer then returned:

> Multicut proof PASS after wording fix, conditional precisely on accepted F10 + insertion source. Checked archive §4 matches identity/coverage, and exact_geometry replay under -O to independent /tmp matches retained all fields except timing. New §6 averaged-normal redundancy is correct for every radial t≥0 and explains mixed75=edge50. All packet links resolve. Minor precision suggestion: numerical 'capacity upper bounds' should be 'nominal feasible-HK upper-bound estimates', since residual diagnostics and Qhull/f64 are unvalidated, though current caveat already says floating-only/no exact certificate. Do not imply strict mathematical enclosures from f64 alone.

The producer adopted that numerical wording in README.md and probe.py and
reran the final numerical producer, writing first to
`/tmp/msc-math/hko-20261016/multicut/final-probe-results.json` before retention.
The final [probe-results.json](probe-results.json) has 12 completed cases,
all nominal ratio estimates below one, and a script hash matching
[probe.py](probe.py). Numerical values were unchanged by the wording correction.
This final numerical rerun and hash comparison were performed by the producer;
the quoted independent review covers the analytic proof, source interface,
exact geometry replay and §6, not a new rigorous enclosure of the numerical
experiment.

The exact replay uses the accepted archive's arithmetic helper, so it is not
an independent implementation of that arithmetic. The analytic proof remains
conditional on the accepted sharp F10 theorem and vertex-insertion construction.
Neither this review nor the floating experiment settles unrestricted F≥12
local maximality, supplies an explicit neighborhood radius, or establishes
literature novelty.
