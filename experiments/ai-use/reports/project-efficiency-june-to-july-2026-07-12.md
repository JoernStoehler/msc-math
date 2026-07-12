# June 1 to July 1 project-efficiency comparison

Status: first interval-level value assessment. It compares the last visible
`main` snapshot on 2026-06-01 with the last visible snapshot on 2026-07-01 and
attributes the intervening work at the thesis-surface level. It is more useful
for the requested question than a daily progress curve, but its value units are
an explicit qualitative accounting, not an objective percentage.

## Endpoint and interval

| Endpoint | `main` snapshot | State of the project ledger |
|---|---|---|
| 2026-06-01 | `fc7f1b99` — `Clarify goal review completion` | The later `PROJECT_COMPLETION.md` ledger did not exist yet; baseline reconstructed from `tasks/definition-of-success.md`, active thesis sources, and owner-local experiment sources. |
| 2026-07-01 | `fcd8545a` — `Clarify flow-graph theorem scope` | The later completion ledger still did not exist; endpoint reconstructed from the same source classes and the July 1 thesis/experiment state. |

For resource accounting, “between” means 2026-06-02 through 2026-07-01,
between the two end-of-day snapshots. The full June calendar month is shown as
context where useful.

## Resource cost

| Interval | Recorded tokens | Cached input | Uncached input | Output | Model mixture | Shadow API-equivalent cost |
|---|---:|---:|---:|---:|---|---:|
| Jun 2–Jul 1 | 11.858B | 11.324B | 491.9M | 41.6M | GPT-5.5 99.966%, Spark 0.034% | $9,365.79 |
| June calendar month | 11.941B | 11.409B | 490.0M | 41.7M | GPT-5.5 99.966%, Spark 0.034% | $9,399.77 |
| July 1 only | 211.2M | 198.7M | 11.6M | 1.0M | GPT-5.5 100% | $186.25 |

The shadow cost uses the mapped public API rates and cached-input discount; it
is not subscription expenditure. The model mixture was effectively constant
over this interval, so the cost comparison is mostly a workload comparison,
not a model-price-mixture artifact. The interval contained 975 rollouts and
92,242 usable token events. Jörn hours and LICCA core-hours are not recoverable
from the available records at this precision.

## Value delta

I used one conservative value unit for a thesis-relevant surface that gained a
material, source-backed support or control transition during the interval. A
unit is not awarded for line count, commit count, or repository cleanup alone.
It requires a named downstream surface and an observable artifact or accepted
scope decision. This gives the interval six defensible support units:

| Surface | Evidence in the interval | What improved | Unit status |
|---|---|---|---|
| HKO local maximum | `eaf31f0d`, `4aa37ff0`, `2e2f1579`, `4b062c09`, `8db8dcd2` | Certificate/verifier route, trust boundary, and thesis explanation were substantially developed and reviewed. | +1 support unit; final theorem/advisor gate remains open. |
| Flow graph / CH2021 | `70fedca8`, `bfea2eab`, `406381cb`, `e87ea869`, `fcd8545a` | Algorithm surface, formal scaffolds, semantic tests, and theorem-scope exposition were strengthened. | +1 support unit; final algorithm role remains conditional. |
| QP and numerics | `0ab23599`, `682ba32d`, `31cefd7e`, `e18c38f8`, `9a4148cf` | Finite-computation wording, f64-route boundaries, failure demonstrations, and numerical provenance were clarified. | +1 support unit; numerical thesis surface remains incomplete. |
| Data-science search | `b0033904`, `c7e7a74d`, `8a8cb06d`, `94d6b2e6`, `d19821c9`, `b1bf6db8` | Baseline closure, LICCA producer/provenance infrastructure, run statistics, and retained evidence were consolidated. | +1 support unit; demonstration and final source-backed rewrite remain open. |
| Rotated products | `c25aa2ae`, `bd7a9317`, `d8f4e9a9` | Thesis draft and exact/proof-status framing were advanced. | +1 support unit; finite-enumeration and exposition gates remain open. |
| Reproducibility and AI-use | `5ccde836`, `e8321161`, `f05d1e75`, `d1f1771d` | Reproduction promises, provenance inventory, and AI-use process evidence became explicit repository artifacts. | +1 support unit; final thesis integration and archive acceptance remain open. |

Thus the defensible summary is: **approximately six material thesis-surface
support units were gained for about $9,366 API-equivalent cost**, with no claim
that this means 6% or that all units are equal. No full completion gate can be
claimed from this interval; most gains strengthened routes that still remained
draft, conditional, or awaiting review.

## Efficiency interpretation

The interval was productive in the sense that it produced broad, durable
support across six thesis-relevant surfaces. It was not efficient by a simple
“units per dollar” measure: the result is roughly 0.00064 conservative support
units per API-equivalent dollar, and that denominator is only an opportunity
cost. The more useful finding is where the cost went:

- the mixture was almost entirely GPT-5.5, so model substitution does not
  explain the amount;
- the work included extensive parallel data-science and numerical exploration,
  not only thesis prose;
- several support units are enabling or risk-reducing work whose downstream
  payoff has not yet been realized;
- the interval did not close the final thesis, advisor, or submission gates.

This is a better first answer to “what did the month buy?” than a percentage
progress claim. Future intervals can reuse the same surface ledger and record
whether a previous support unit later becomes a closed gate, remains dormant,
or is cut from the retained thesis.

## Reproduction

First produce the full token packet as documented in
`token-usage-lifetime-analysis-2026-07-12.md`, then run:

```bash
uv run --script experiments/ai-use/scripts/analyze_project_efficiency.py \
  --date 2026-06-01 --date 2026-07-01 \
  --token-dir /tmp/codex-token-usage-lifetime2 \
  --git-ref main \
  --out-dir /tmp/codex-project-efficiency-june-july
```

The generated packet supplies the endpoint commits, token/cost rows, changed
file counts, and raw session-log proxies. The six-unit interpretation above is
the reviewed semantic layer and must be revisited if the thesis scope or
source status changes.

This report is experiment-support material, not thesis-ready quantitative
evidence. Its possible thesis use is as a carefully caveated AI-use process
reflection, not as a claim that research value has been objectively measured.
