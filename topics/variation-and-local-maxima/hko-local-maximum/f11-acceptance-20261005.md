# Accepted HKO local maximality for at most eleven facets

Research acceptance: 4 October 2026, 23:00–23:02 UTC; source retention:
5 October in Europe/Berlin. This is an accepted computer-assisted research
theorem. It is not yet a theorem in the selected manuscript or its PDF.

For the standard HKO pentagon product H, there exists ε>0 such that every
convex four-polytope K with at most eleven genuine facets and d_H(K,H)<ε has

`c_EHZ(K)^2 / (2 Vol_4(K)) ≤ (3+sqrt(5))/5`.

Old normals may move arbitrarily. Nonlinear approach sequences and coupled
relative scales are included; product structure and fixed-normal cuts are not
assumed. The proof takes the previously accepted sharp ten-facet theorem and
local symmetry slice as inputs, together with the primary polar-loop capacity
principle and HKO benchmark. The ten-facet formal proof supplies the needed
linear deficit, stronger than strictness alone.

## Proof and acceptance evidence

The immutable [original ZIP](f11-return-20261004/hko_f11_complete_proof.zip)
is SHA-256 `aea22ffd97fd4728eeedb4de4ab272626cf58501aca813c6de131a371fc39f6e`.
The [original proof text](f11-return-20261004/hko_f11_complete_proof.md)
is also retained. ZIP entries `PROOF.md`, `math/CORE_THEOREM.md`,
`math/COLLAR_THEOREM.md`, `inherited/math/ENDPOINTS.md`, and
`inherited/previous_math/{RIDGE_NEIGHBORHOODS,LOCAL_CUT_LIFTING,SETUP_AND_AUDIT}.md`
own the proof and its finite/analytic interfaces. Historical inherited texts
that call the interior case open are superseded by the closing `PROOF.md`.

- [Integrated acceptance](../../empirical-studies/discovery-design/night-20261005/f11-acceptance/report.md)
  checks sharp F10, duplicate polar-loop witnesses, closure correction,
  upper-collar interchange, exact arithmetic signs and arbitrary-sequence
  Hausdorff exhaustion.
- [Finite replay and source audit](../../empirical-studies/discovery-design/night-20261005/f11-acceptance/finite-replay.md)
  and its [complete fresh evidence](../../empirical-studies/discovery-design/night-20261005/f11-acceptance/finite-replay/)
  accept all seven stages in 387.039 seconds on Python 3.12.3: four positive
  stages and three corruptions rejected at their intended predicates.
  Core checks cover 3,258 rows, 824 relations and 1,968 tetrahedra with exact
  margin ≥10^-6. Collar checks cover 257 paths, twenty policies, 160 sign cells,
  887 leaves and 195,140 nonnegative coefficients, proving margin μ²/1000.
  All 140 original/copied source files stayed unchanged; mathematical outputs
  match the author's replay after excluding timing/environment fields.
- [Independent endpoint/other-face audit](../../empirical-studies/discovery-design/night-20261005/f11-acceptance/endpoint-audit.md)
  accepts moving two-slack charts, relative shrinking-cap remainders,
  duplicate interchange and exhaustion of the other polar strata. Finite PASS
  alone would not establish these implications.
- [Crossed implication review](../../empirical-studies/discovery-design/night-20261005/comparison/f11-cross-review.md)
  independently accepts the finite-plus-analytic chain to the full theorem.
  It inherits the finite replay rather than claiming another execution.
- The prior [interior bridge review](f11-return-20261004/independent-bridge-review.md)
  derives the literal section derivative, polygon moment identity and cleared
  polynomial identity; point comparisons alone do not prove these identities.

The accepted F10 input remains owned by [theorem/README.md](theorem/README.md)
and [the formal feasible-section argument](../../../formal/hko-feasible-section-upper-branches.tex).
No new Sage replay of F10 was performed by this F11 acceptance. The older
manuscript continuation's separately scoped fresh Sage checks remain attributed
to [their receipt](../../../docs/history/thesis-continuation-2026-10-04/hko-execution/receipt.json).

## Reproduce the finite predicates

From the repository root, extract the retained ZIP into fresh scratch and run
its standard-library checker. The following is the same entry point used in
the accepted fresh replay; it does not rerun discovery or replace retained data.
Python 3.11 or newer is the packet's requirement; the fresh project run used
3.12.3 with assertions enabled. Use a new scratch directory for each run.

```sh
f11_replay=$(mktemp -d /tmp/hko-f11-replay.XXXXXX)
python3 - "$f11_replay" <<'PY'
from pathlib import Path
from zipfile import ZipFile
import sys
target = Path(sys.argv[1]) / "packet"
with ZipFile("topics/variation-and-local-maxima/hko-local-maximum/f11-return-20261004/hko_f11_complete_proof.zip") as archive:
    for name in archive.namelist():
        path = Path(name)
        if path.is_absolute() or ".." in path.parts:
            raise ValueError("unsafe archive path")
    archive.extractall(target)
PY
env -u PYTHONOPTIMIZE python3 "$f11_replay/packet/run_checks.py" \
  --out "$f11_replay/results" --negative-controls
```

The master rejects an existing output directory. Outputs and incidental
bytecode stay in scratch. `results/SUMMARY.json` binds stage commands, results,
interpreter and timings; each stage retains complete logs and positive JSON.
Runtime is environment dependent: the packet's retained measurement was about
217 seconds, while the complete fresh run here was 387 seconds. Reproducing
finite predicates does not itself recheck the analytic proof or F10 input.

## Manuscript and limits

The selected [Chapter 7](../../../thesis/chapters/07-hko.tex), theorem
`thm:hko-ten-facet-local-maximum`, still states the fixed-ten-facet result.
There is **no selected thesis location for the F≤11 theorem**. The retained
[101-page candidate](../../../docs/history/thesis-continuation-2026-10-04/thesis-candidate.pdf)
and its [acceptance owner](../../../docs/history/thesis-continuation-2026-10-04/acceptance-review.md)
remain unchanged. On 4 October at 23:08 UTC, all 61 selected source hashes and
the retained PDF hash matched their [candidate manifest](../../../docs/history/thesis-continuation-2026-10-04/candidate-manifest.json).
This identity check carries forward the earlier PDF review; it is not a new
PDF review of an F11 writeup. Manuscript inclusion, exposition and affected-PDF
review are separate future integration work.

The neighborhood radius is existential. No eleven-facet equality
classification, global HKO extremality, result for twelve or more facets, or
unrestricted-body theorem is claimed. Project acceptance is not proof-assistant
formalization, external refereeing, Jörn's prose acceptance or Kai's PASS.
