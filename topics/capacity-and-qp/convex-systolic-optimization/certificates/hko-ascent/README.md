# HKO simultaneous-cut ascent: portable exact replay v2

This package checks the finite computational premise in the HKO ascent proof:
the maximal radial hinge response over **all simple maximizing words and all
their weights** is exactly `1/125`. The assembled analytic proof then uses
the characteristic-invariant repeated-word reduction and the general
simultaneous-splitting derivative theorem to conclude

```
D+ log sys(H_t) = (20+64 sqrt(5))/9375 > 0.
```

The explicit twenty-inequality family, full response reduction, and complete
catalogue argument are in `hko-ascent-main.tex`. Its referenced derivative
theorem is included in `multicut-variation.tex`. These are section modules,
not standalone LaTeX documents. The derivative proof establishes improvement for every sufficiently small
positive parameter. The added `hko-effective-step.tex` and quantitative
companion now certify the explicit parameter `t=10^-30` and relative gain
greater than `5·10^-33` for the ORIGINAL ALL-TEN-UNIT-CUT TWENTY-ROW FAMILY.
This quantitative bound is not asserted for the seventeen-facet subfamilies.
`hko-seventeen-facets.tex` gives the exact response for nonnegative cut speeds,
any seven-unit-cut ascent, and the sharp seven-new-row threshold within these
fixed profiles. No categorical novelty claim is part of this package.
The base main module's nonquantitative statements refer to its derivative
proof; the effective extension supplies the additional quantitative result.

Packet integration corrected an endpoint inequality in `quantitative-proof/effective-step.md`: the distance bound is non-strict when the deficit is zero. It explicitly limits the quantitative pivot lemma to simple twenty-row words and updates the note's context after the separate exact `t=3/10` result. No finite checker, expected output, or analytic constant changed. `MANIFEST.json` records the current companion bytes; the original frozen v2 ZIP (SHA-256 `4059b495ff7418d6b80ce77bcede6a8c348cd1d1aa783d50dd1de6dc77c5a989`) remains historical provenance.

## Dependencies and one command

- Python 3.11 or later, standard library only.
- A C++17 compiler accessible as `c++`, or supplied with `--cxx`.
- No network, Sage, NumPy, SciPy, package installation, or repository checkout
  is required for this replay.

From the extracted directory run:

```sh
python replay.py --out /tmp/hko-ascent-fresh
```

The output directory **must not already exist**. With a specific compiler:

```sh
python replay.py --cxx g++ --out /tmp/hko-ascent-fresh-gcc
```

`replay.py` copies the six producer sources into the fresh output directory,
runs the exact primitive calculation, compiles and runs the complete cyclic
order scan, and computes every exact interval response maximum. It compares
the five complete generated JSON structures with the retained expected
outputs, ignoring only runtime fields named `seconds` or `elapsed_seconds`. Success creates `PASS.json`
and `replay-log.json` in that output directory. A failed subprocess or any
certificate discrepancy exits unsuccessfully. The runner refuses to replace
an existing output directory.

All generated sources, executable, Python bytecode, JSON outputs and logs are
written inside that fresh output directory. The extracted package is read
only during replay. The command does not edit a repository or contact any
external service.

The underlying commands, run by the wrapper in the new output directory, are:

```sh
python prepare.py
c++ -O2 -std=c++17 catalogue.cpp -o catalogue
./catalogue primitives.txt catalogue.json
python optimize_exact.py
python check_radial_profile_witness.py --core exact_core.py --out radial-profile-witness-PASS.json
python check_effective_constants.py --core exact_core.py --out constants-PASS.json
```

## What is recomputed

`prepare.py` constructs both normalized factor-closure polytopes by solving
every three-normal support. It checks all 3000 triangle-pair cyclic six-orders
in the exact field `Q[tau]/(tau^4-10tau^2+5)`, with `0<tau<1`, and finds all
50 maximizing primitive words.

`catalogue.cpp` scans all `9!=362880` full cyclic ten-orders. For each it
builds the complete 5 by 5 graph of winning induced triangle pairs and
enumerates its maximal complete bipartite subgraphs. After valid support-word
deduplication, this produces 190 complete charts: 50 points and 140 intervals.
It uses integer/combinatorial operations only.

`optimize_exact.py` reconstructs every chart's KKT loop in exact arithmetic.
It checks stationarity, objective, and constancy of the radial coordinate on
each characteristic segment. It computes all hinge roots and every possible
quadratic maximum on each resulting interval. Exactly 110 chart maxima are
zero and 80 are `1/125`. It also checks the exact positive difference from
the normalized cone-volume response and records the derivative coefficients.

`check_radial_profile_witness.py` verifies the literal seven-word witness
concentrating response `1/125` on a single facet, its KKT reconstruction and
body containment, the symplectic factor-exchange map, and the exact six-
versus-seven cut-speed comparisons. The weighted upper bound additionally
uses the complete base response certificate.

`check_effective_constants.py` independently enumerates the 3000 primitive
actions to verify their minimum positive loss, then checks the rational
Taylor, rounding, pivot-preservation, and final product constants. The written
proofs in `quantitative-proof/` supply the global analytic implications; these
are not inferred from finite predicates alone. The proof sources preserved
here are the reviewed versions; the quantitative audit is also included
beside the proof as `quantitative-proof/independent-audit.md`.

The comparisons establish agreement of all detailed producer outputs, not
only the headline scalar. `MANIFEST.json` pins the packaged source, expected
output, proof-module, and review-file bytes. It is provenance information;
the replay's substantive checks recompute the arithmetic and enumeration.

## Arithmetic dependency and independent reviews

`exact_core.py` is copied without modification from member
`core/code/exact_core.py` in the existing `hko_f11_complete_proof.zip`.
That source archive has SHA-256
`aea22ffd97fd4728eeedb4de4ab272626cf58501aca813c6de131a371fc39f6e`.
The helper supplies rational field arithmetic, exact sign isolation, matrix
inversion, normals, and the literal HK action. Some unused helpers remain in
the source to preserve its exact identity. The archive itself is unnecessary
for replay.

`reviews/` preserves the independent finite-derivative, repeated-word,
catalogue-coverage, exact-response, and fresh assembled-analytic audit reports.
The catalogue review independently reconstructed the field arithmetic using
Sage and the full cyclic scan using Python, matching the full chart set.
Sage was needed for that independent audit, not for this package's replay.
The assembled analytic audit separately checked the delicate repeated-word
weight/placement argument and the implications for the actual derivative.
Historical absolute paths in these reports are provenance, not portable
dependencies. The proof modules and replay sources included here supply the
active mathematical and computational interfaces.

The independent reports do not turn the full mathematical implication into
a formally verified theorem. In particular, the repeated-word reduction and
the finite-bank sensitivity argument are written proofs, not finite tests.
The precise division between these analytic arguments and the exact finite
predicates is explained in the main section.

## Executed portable replay

An isolated replay was performed after assembling this package. The copied
`evidence/PASS.json` records the actual interpreter, compiler, elapsed time and
successful exact comparisons; `evidence/replay-log.json` records the actual
commands and their outputs. Re-running is a small local CPU calculation,
normally taking seconds to tens of seconds. Runtime is descriptive, not an
acceptance predicate.
