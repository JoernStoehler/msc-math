# Sys-Datascience Sprint: Observed Cost And Result

Status: selected naturalistic case for the AI-research discussion. This is an
observed episode, not a human-only counterfactual, a representative sample, or
a causal estimate of delegation overhead.

## Case Boundary

- Root Codex session: `019f706b-45c4-70b0-aada-7edc8d45c292`.
- Start checkpoint: Jörn's budget authorization at
  `2026-07-17T17:18:45.314Z`.
- End checkpoint: Jörn's request for the completed-sprint recap at
  `2026-07-18T06:46:54.313Z`.
- Authorized budgets: USD 200 shadow API cost for experiments, USD 100 for new
  experiment ideas, and six hours wall time. Jörn asked for a hard stop when
  any budget was exceeded.

The cost audit follows metadata-linked descendants recursively and charges
each rollout by its recorded model. It converts cumulative counters to deltas
after the start checkpoint and sums interaction checkpoints no later than the
end checkpoint. The rates are the project priority-tier rates recorded in
`AGENTS.md` on 2026-07-16:

```text
cost = ((input - cached_input) * I
        + cached_input * C
        + output * O) / 1,000,000
```

The structural parser does not need message bodies to compute the counters.
The current producer is proposed separately at
`.agents/skills/codex-session-log-parsing/scripts/session_cost.py`; until that
harness change is accepted, the raw rollout and this recorded checkpoint
contract remain the source for an independent recomputation.

## Observed Resource Use

| Work group | Uncached input | Cached input | Output | Shadow cost (USD) |
| --- | ---: | ---: | ---: | ---: |
| Luna repository/evidence scouts | 1,517,807 | 30,721,792 | 109,321 | 10.49 |
| Chaidez--Hutchings exact fixture | 842,939 | 17,897,728 | 72,613 | 30.68 |
| HKO transverse-ray pilot | 1,725,328 | 45,352,704 | 259,774 | 78.19 |
| Regular 3-by-6 orientation pilot | 3,197,526 | 110,485,504 | 478,597 | 171.18 |
| Main-session planning, interpretation, and synthesis | 1,027,743 | 64,188,928 | 104,054 | 80.71 |
| **Total to recap request** | **8,311,343** | **268,646,656** | **1,024,359** | **371.25** |

The three experiment lineages cost USD 280.05, exceeding the USD 200
experiment-agent allocation before charging scouts or main-session work.
Cached input was charged at its separate lower rate; it was not treated as
free. The session continued after this checkpoint, so whole-session counters
must not be substituted for this sprint total.

The three scientific runs themselves took minutes to tens of minutes. The HKO
producer, the longest retained run, recorded 1,419 seconds for 1,171 capacity
evaluations. Thus this episode's shadow cost was dominated by agent generation,
context, repair, and review rather than target computation.

## Result Ladder

### Chaidez--Hutchings fixture

- Produced and retained an exact reproducible packet.
- Established the displayed body's exact geometry, capacity, and `sys=1`.
- Found that nine actual two-faces are Lagrangian, changing the interpretation
  of the ordinary combinatorial-flow route while leaving HK applicability.
- Integrated on Main and usable as mathematical/verification evidence.

### HKO transverse rays

- Produced a frozen 32-direction panel with controls, manifests, full
  evaluations, basis, and bounded interpretation.
- Found an above-to-below nominal transition on every sampled ray in one chosen
  transverse affine slice.
- Did not establish an invariant radius, star-shapedness, component trapping,
  or absence of thin connections.
- The July 22 wrap-up candidate promotes the reproduction packet while leaving
  its possible thesis presentation as a separate decision.

### Regular 3-by-6 orientations

- Produced a target-blind four-point quotient-orientation panel; all four exact
  values were below one.
- Did not test a narrow local equality cone or other bodies and supplied little
  standalone thesis value.
- Exposed a correctness defect in relying on floating-point candidate retention
  for an exact interval. The repaired run exact-solved all candidates in the
  supplied theorem stream.
- Main already has an experiment-local exact-all-visited-sigma route; the
  remaining crate-API extraction is a reuse decision, not the only available
  correctness route.

### Portfolio effect

- The observations changed the scientific map and supplied one exact integrated
  result, one potentially useful bounded HKO panel, one narrow negative, and a
  reusable correctness lesson.
- They did not produce a new `sys > 1` example away from the known family.
- The overrun was noticed retrospectively rather than stopping execution at the
  authorized boundary. This is a workflow failure independent of whether the
  retained scientific results are valuable.

## What The Case Supports

The episode supports separating four costs that were conflated during
execution:

1. obtaining a scientifically valid observation;
2. interpreting the observation and updating the research portfolio;
3. preserving a reproducible evidence packet; and
4. integrating reusable code or publication-facing prose.

Not every observation repays all four costs. Conversely, calling code
throwaway does not justify incomplete candidate families, missing controls, or
unrecoverable evidence. A consumer gate between observation and promotion
would have allowed the exact fixture to be retained without automatically
charging both pilots for extensive reusable infrastructure.

The case also shows why deferral is not inherently efficient. Deferring a
promotion is favorable only when the avoided current review/maintenance cost
exceeds later reconstruction and context-loss cost. For the HKO packet, a bare
branch hash would not preserve enough evidence for likely future use; the July
22 candidate therefore promotes the reproducible packet instead of indefinitely
deferring the decision.

## Claims Not Supported

- The USD 371.25 total is not a paid invoice or the cost of a human alternative.
- The grouping does not isolate a causal effect of model choice, delegation,
  review count, cached context, or prompt quality.
- Wall-clock rollout duration is not human active time.
- The episode does not show that subagents, review, or durable infrastructure
  are generally wasteful. It shows that their measured cost exceeded the
  declared budget in this configured case.
- Code size, commit count, and session count are not value measures.
