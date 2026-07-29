# Local-Maxima Control Calibration

This companion calibrates cheap fixed-facet falsifiers before applying them to
new candidate bodies. It reuses the quotient geometry and full scalar evaluator
from the parent packet; it does not create another local-maxima experiment
owner.

The selected-body audit, method comparison, retained results, and next-route
assessment are in [`PILOT-REPORT.md`](PILOT-REPORT.md).

## Question and downstream use

Can a small derivative-free suite distinguish:

- the proved fixed-`F=10` HKO local maximum;
- controlled off-HKO states;
- the exact rotated-pentagon nonmaximum and its improving side; and
- an ordinary random fixed-`F=6` state?

A successful calibration licenses the same finite suite as an inexpensive
first falsifier for selected candidates. It does not license a
local-maximality claim from a miss.

## Frozen pilot contract

The producer evaluates:

1. both signs of every axis in the deterministic Euclidean quotient basis at
   relative row radius `1e-4` for HKO and one deterministic ordinary random
   `F=6` polytope;
2. return moves of `0.1`, `0.01`, and `0.001` times the HKO distance from two
   controlled quotient perturbations at relative distance `1e-3`;
3. both relative-rotation directions from the rotated-pentagon equality
   crossing at angular radii `1e-2`, `1e-3`, and `1e-4`; and
4. forward continuation from a point `1e-2` radians onto the improving side at
   angular steps `1e-3`, `1e-4`, and `1e-5`.

Every point is recomputed with the current `MinimaSafe` scalar route. Raw rows
retain the direction, step norm, dual vertices, incidence comparison, scalar
interval, best returned word, returned-orbit count, and cost. The producer
does not expose beta/domain diagnostics and therefore records them as
unavailable rather than implying numerical active-set completeness.

Expected outcomes were fixed before the retained run:

- the exact pentagon directions recover the profile's sign;
- HKO has no nominal improving signed-basis direction;
- controlled HKO perturbations have nominally improving return moves;
- the ordinary random state has at least one nominally improving signed-basis
  direction.

Failure to recover either pentagon or random negative control blocks use of
target misses. HKO disagreement is first an evaluator/control problem because
the exact theorem packet is authoritative and its f64 capacity intervals can
be broad.

## Evidence and claim boundaries

The rotated-pentagon theorem proves that the equality crossing and every
interior point on the named improving side are not local maxima in that
structured family. The generated rows are only detector calibration for that
known result.

For HKO, the exact feasible-section packet proves fixed-ten-facet local
maximality. A finite signed-basis miss is operational consistency only.

For a controlled HKO perturbation or random body, a found finite improving
point rejects only the corresponding finite stationarity condition. Three
shrinking positive steps are empirical evidence for an improving germ, not a
proof of arbitrarily small improvement. A miss remains basis-, radius-,
chart-, and evaluator-dependent.

## Run

From the repository root:

```bash
cargo run --release -p exp-local-maxima-check \
  --bin local-maxima-control-calibration -- --smoke

cargo run --release -p exp-local-maxima-check \
  --bin local-maxima-control-calibration -- --canonical
```

Smoke output is temporary and tests only one quotient pair, one HKO return
move, one crossing radius, one improving-side step, and one random quotient
pair. The bounded canonical pilot writes:

- `artifacts/rows.jsonl`: raw finite-point evidence;
- `artifacts/summary.json`: generated case classifications at their declared
  evidence strength;
- `artifacts/run-provenance.json`: command, constants, source paths, Git state,
  random identity, and measured wall time.

## Plot contract for a larger panel

No plot is needed to interpret this small calibration. A larger candidate
panel should plot best signed change per unit step against radius, with
separate facets for body and probe family. It should show all directions,
highlight incidence changes, distinguish nominal from interval-separated
signs, and overlay the exact pentagon slope only as a control. Bodies, not
directions within one body, are the independent units.
