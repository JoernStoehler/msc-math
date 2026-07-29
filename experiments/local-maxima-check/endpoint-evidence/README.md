# Endpoint Evidence Consumer

This G1 packet turns heterogeneous finite probes and optimizer continuations
into comparable endpoint evidence. It reports whether a recorded state is
explicitly improvable, invalid or indeterminate, or a survivor of one named
finite suite. It never reports a Boolean `is_local_maximum`.

## Why a consumer

The expensive producers already exist:

- [`control-calibration/`](../control-calibration/README.md) evaluates HKO,
  controlled HKO perturbations, pentagon controls, and a random body;
- the parent packet evaluates selected equality bodies;
- `experiments/dev-gradient-ascent/quotient-endpoint-diagnostic/` evaluates
  generic optimizer states in a local symmetry-transverse slice;
- optimizer runners can supply continuation gain, compute, path length, and
  stopping information.

G1 fixes their shared evidence contract without requiring every optimizer to
share a step shape. `adapt.py` currently supports the control-calibration and
quotient-endpoint schemas. A new optimizer experiment may instead emit the
event contract directly.

## Event contract

The input is JSONL with `schema_version: 1`, `row_type`, and `state_id`.

| Row | Required scientific content |
| --- | --- |
| `state` | source/control role; current, terminal, or best-state selection; outcome-selection flag; geometry validity; named suite and complete expected probe identifiers; material threshold; optimizer compute/stopping provenance; separate positive fixed-`F` and named facet-addition evidence |
| `probe` | unique probe and suite; direction family/index/sign; radius and step norm; raw base/perturbed `sys` and change; validity, incidence, uncertainty, and failure |
| `continuation` | extra measured compute and trusted calls; raw gain; path length and displacement; stopping reason and validity |

The finite classification has this precedence:

1. any valid probe or continuation above the declared threshold gives
   `explicit_improvement_found`;
2. invalid base geometry, incomplete expected coverage, invalid/indeterminate
   expected probes, or a required failed continuation gives
   `invalid_or_indeterminate`;
3. a complete valid named suite with no improvement gives
   `finite_suite_survivor`;
4. no declared suite gives `no_finite_suite`.

Thus one good improvement remains decisive even if another direction failed.
A finite miss remains conditional on the declared suite and resolution.
Positive fixed-facet theorem/certificate evidence and named facet-addition
evidence remain separate fields.

## Smoke and tests

The smoke fixture contains compact contract rows for HKO, an HKO perturbation,
the pentagon, a random state, an archived improvable endpoint, and three
synthetic interpretation cases. It validates plumbing and predeclared
expectations; it is not new geometric evidence.

```bash
python3 -m unittest \
  experiments/local-maxima-check/endpoint-evidence/test_endpoint_evidence.py

out="$(mktemp -d)/g1-smoke"
python3 experiments/local-maxima-check/endpoint-evidence/analyze.py \
  --input experiments/local-maxima-check/endpoint-evidence/fixtures/contract-events.ndjson \
  --out "$out"
```

The command writes `normalized-evidence.jsonl`, `summary.json`, and
`REPORT.md` into a new output directory. It refuses to overwrite an existing
path. A failed predeclared control makes the command fail.

Adapters:

```bash
python3 experiments/local-maxima-check/endpoint-evidence/adapt.py \
  control-calibration \
  --rows MATERIALIZED_CONTROL_ROWS.jsonl \
  --out /tmp/g1-control-input.jsonl

python3 experiments/local-maxima-check/endpoint-evidence/adapt.py \
  quotient-endpoint \
  --states MATERIALIZED_STATES.jsonl \
  --probes MATERIALIZED_PROBES.jsonl \
  --out /tmp/g1-endpoint-input.jsonl
```

The quotient adapter assigns structured selection provenance from the
producer's declared `control_role`; it does not infer scientific strata from
free-text selection descriptions. Its inputs are complete outputs from the
named producer packet, not an untrusted interchange format.

Git LFS pointer text is rejected with a specific error instead of being parsed
as scientific data.

## Downstream use

- G2 joins normalized rows to optimizer checkpoints and reports score,
  continuation, slope, path, and evidence distributions by method and compute.
- L1 uses the same rows to screen endpoints; one improvement rejects a state,
  while finite survivors may receive one richer direction/return follow-up.
- I2 stratifies invariant trajectories by explicit improvement, conditional
  finite survival, or separately sourced stronger evidence.

No consumer may turn `finite_suite_survivor` into local maximality, exhaustion
of ordinary defects, a basin, or stability under adding facets.
