# Linked generator distribution contract

## Question and boundary

This target-free packet asks what a small generator sample actually says about
attempted draws, accepted exact geometries, duplicates, and discrete
incidence-signature discovery.  It is the copy-local contract for future
distribution packets, not a shared schema registry and not a generator-quality
score.  The primary report unit is one declared `law_run_id` (and therefore its
stratum/configuration); within that report, the statistical unit is the declared
`independent_unit_id`. Event rows within one `block` or `paired_block` are
retained but never counted as IID draws. Distinct runs/strata sharing a
`law_id` are not silently pooled.

The packet rejects target fields (`sys`, capacity, iterations, bounce/target
labels and target-derived selections) even when null.  Consequently the audit
cannot establish target utility, transfer, natural population frequencies, or
support in continuous geometry.  Its unseen-signature estimate is a
Good--Turing-style `singletons / units` diagnostic only when every independent
unit is IID, the attempt log is complete, no censor is present, and each unit
has exactly one accepted observed incidence signature. Accepted-only logs make
this estimate unavailable even when their visible signatures look complete.

## Linked records

The four JSONL files are intentionally narrow and copy-editable:

| file | key and relationship |
| --- | --- |
| `law_runs.jsonl` | `law_run_id`; generator law/version, source SHA/path, exact configuration, mixture component and stratum, proposed/accepted law, normalization, attempt-log status, and target-exposure state. |
| `sampling_events.jsonl` | `event_id`; references `law_run_id`, declares attempt/draw identity, `accepted` (`true`, `false`, or unknown `null`), reason, independent unit kind/id, seed/lineage, cost, pairing, and optional `geometry_id`/metric view. |
| `geometry_views.jsonl` | `geometry_id`; exact payload hash/pointer (nullable when unavailable), facet count, `(q1,q2,p1,p2)` coordinate order, incidence signature, representation/canonicalization, view ID/version, explicit `injective` and `lossy` booleans, and an `invariant_under` action list. |
| `metric_preprocessing.jsonl` | `metric_view_id`; metric/version plus fitted preprocessing identity, reference, and split. No fitted target representation is allowed. |

One law/run may have many sampling events; an accepted event may join one
geometry view; many events may alias one exact geometry hash.  Rejected events
have no geometry.  Unknown-censor events are retained and make an unconditional
acceptance rate unavailable. Missing exact hashes, incidence signatures, costs,
and attempt logs remain explicit rather than being imputed. `recorded_event_rows`
is only the number of event records; `attempt_count` is null unless the run's
attempt log is complete and the rows are IID one-event/one-attempt units. Cost
per recorded event is descriptive. The mean processing cost on accepted exact
events is `mean_processing_cost_ms_per_accepted_exact_event`; total recorded
sampling cost per accepted exact result additionally includes rejected attempts
and is emitted only for complete uncensored IID logs with complete costs. Cost
per attempt and both accepted-result fields carry explicit availability status.
Attempt IDs are unique within each law/run, and IID rows cannot repeat an
independent unit. Geometry view IDs must resolve to metric/preprocessing records;
an event's metric ID must agree with its geometry view ID. An empty incidence
list or empty face is corrupt/missing; use JSON `null` for an unavailable view.

## Consumer and outputs

Run the deterministic standard-library consumer:

```bash
python3 contract.py --input-dir fixtures/synthetic \
  --output /tmp/generator-distribution-audit.json
```

It reports, by law/run and declared independent unit: recorded event rows,
accepted/rejected/unknown-censor counts, an attempt count only with a complete
attempt log, and an acceptance rate only with a complete log and no censoring;
costs with explicit completeness statuses; exact duplicate multiplicities;
unit-level incidence discovery/rarefaction curves; singleton and doubleton
signatures; the guarded unseen-signature estimate; and a deterministic
half-split held-out new-signature rate.  Diagnostics identify each statistic
made unidentifiable by missing provenance. Unit ordering uses a SHA-256 rank,
so JSONL input order does not affect the report. A regression fixture uses one
`law_id` across two runs/strata and verifies that they remain separate.

## Calibrations

`make_fixtures.py` regenerates `fixtures/synthetic/`, which contains:

- `null-law`: same-law IID split with no held-out novelty (negative baseline);
- `collapsed-law`: eight exact aliases;
- `rare-mixture` and `rare-deleted`: a two-unit rare incidence type versus its
  deletion control;
- `censored-law`: rejection, unknown truncation, and missing cost;
- `rejection-cost-law`: a complete IID log with a deliberately high-cost
  rejection, checking rejection-inclusive cost per accepted exact result;
- `paired-law`: two dependent paired rows per four independent blocks; and
- `missing-provenance`: accepted-only attempt status, no cost, exact hash, or
  incidence signature.

The focused tests also mutate a record with a forbidden target field, malformed
incidence, unknown/mismatched metric joins, repeated attempt/unit IDs,
accepted-only Good--Turing provenance, empty incidence/faces, and a truncated
JSON record. Adapter-focused tests cover outside-repository provenance and
within-run semantic-field disagreement. They check deterministic reruns and
input-order invariance.

```bash
python3 make_fixtures.py --out-dir /tmp/generator-distribution-fixture
python3 -m unittest discover -s . -p 'test_*.py'
```

The retained `artifacts/synthetic-audit-report.json` is regenerated from the
fixture.  It is a calibration artifact, not evidence about a production law.

## Real-source adapter

`adapt_orientation.py` consumes the reviewed, target-free 40-row orientation
panel without changing its owner.  The source path and SHA-256 are retained in
`artifacts/orientation-panel/adapter-report.json`; the normalized linked records
and `artifacts/orientation-audit-report.json` are inspectable.  The panel has
exact transformed geometry IDs and labeled incidence signatures, and measured
generation/transform/reconstruction cost.  Its rows are accepted-only and each
five-map family shares a `base_id`, so the adapter declares `paired_block`
units. Across all four buckets there are 8 independent bases per map variant
and 40 events. Rejection
counts are absent from this accepted-only source. The primary real estimand is
**two independent bases per
map-variant/bucket run** (20 separate runs), not eight pooled bases per variant;
the 40 event rows are five paired-map observations for each base. Rejection
counts and an acceptance rate are therefore not identifiable; each run reports
`recorded_event_rows=2`, `attempt_count=null`, and a null cost-per-attempt.
Source timing remains a descriptive cost per recorded event, and the mean
processing cost on accepted exact events is available where every accepted
geometry has a recorded cost. Even a complete event log does not make
attempted-draw counts or cost-per-attempt identifiable for dependent paired
rows; those fields remain null and carry a diagnostic. Target exposure is
absent. The incidence discovery and held-out curves are descriptive within each
two-base run, not population estimates.

Reproduce the adapter and audit (the source is a retained LFS artifact; hydrate
only this named input when needed):

```bash
cd /workspaces/msc-math
git lfs checkout -- experiments/sys-datascience/methods/generator-orientation-smoke/artifacts/panel-2-per-bucket/rows.jsonl
cd experiments/sys-datascience/methods/generator-distribution-contract
python3 adapt_orientation.py \
  --source ../generator-orientation-smoke/artifacts/panel-2-per-bucket/rows.jsonl \
  --out-dir /tmp/generator-distribution-orientation-panel
python3 contract.py \
  --input-dir /tmp/generator-distribution-orientation-panel \
  --output /tmp/generator-distribution-orientation-audit.json
```

Measured local cost for the retained adapter is 40 events and 40 exact views
(about 0.02 s to adapt and audit on the devcontainer); the source timing fields
sum to roughly 212--776 ms per map-variant panel across its eight bases. The
adapter records the stable last commit touching the named source path rather
than the volatile checkout `HEAD`. This is source cost metadata, not a
production benchmark.

If a valid source is copied outside this repository, the adapter remains usable:
it labels the source by its absolute path and records
`outside-repository/not-recorded`, or a caller-supplied stable revision via
`--source-revision`. Rows sharing one `(map_variant,bucket)` run are checked for
agreement in map family/mode, coordinate order, side and facet/vertex counts,
and other normalization-relevant fields before records are emitted.

## Claims and reopen conditions

Allowed claims are that this contract distinguishes attempts from accepted
rows, aliases from distinct exact identities, and dependent blocks from IID
units, and that the synthetic/real-source plumbing produces the reported
finite-sample diagnostics.  Prohibited claims include law ranking, continuous
support or topology, natural frequencies, target (`sys`) performance, causal
mechanisms, and transfer.  Reopen before reuse if a producer cannot declare an
independent unit, mixes facet/mixture strata, changes incidence equivalence,
uses lossy features as exact geometry, or needs a target-derived selection.

The orientation adapter is the only real source retained here.  An alternative
generator smoke adapter was not added: its checked-in rows expose scalar factor
summaries but no exact geometry or incidence signature, so reverse-engineering a
lossy hash would add cost without supporting the required discovery audit.
