# Allocate Model Effort For Empirical Research

Read this only when model/effort, context mode, decomposition, or review design
is a material recurring choice. This is current task-conditioned evidence, not
a leaderboard or a default that every experiment must discuss.

Contents: [current evidence](#current-evidence) and
[current reversible choices](#current-reversible-choices).

Infer performance for a configured system:

```text
(labor, decomposition, context, prompt, model/effort, tools, oversight)
    -> (observable behavior, product quality, repair, downstream usability)
```

Do not infer an internal cause from a read, omission, or self-summary. Distinguish
`can do when prompted` from `spontaneously initiates`, and a successful stronger
configuration from evidence that a cheaper one cannot do the work.

## Current Evidence

### Bounded research screening

Six fresh agents received the same read-only research-screening prompt with
cheap empirical checks. One sample per configuration produced non-nested ideas.

| Configuration | Wall | Shadow cost | Observed product |
| --- | ---: | ---: | --- |
| Luna medium | 138 s | $0.31 | adequate summary, shallow frontier selection |
| Luna high | 450 s | $1.12 | strong evidence-heavy audit and exact-rho checks |
| Luna xhigh | 532 s | $0.85 | contaminated by another agent's scratch; inconclusive |
| Sol low | 90 s | $0.49 | useful fast audit and duplicate/confusion detection |
| Sol medium | 171 s | $0.94 | useful new class-degeneration hypothesis |
| Sol high | 466 s | $2.70 | most valuable abstract scientific ideas in this sample |

The runs do not establish adjacent-level success probabilities or a general
frontier. Their additive non-duplicate findings support using independent idea
search when option diversity is valuable, but not rerunning all levels without
a live decision. Source rollouts are the 2026-07-13 threads
`019f5ce2-4ca6-75c0-8c62-4897c8374247`,
`019f5ce2-7269-7842-b081-7af15bd10f6f`,
`019f5ce2-9270-71f3-8905-c794480fee15`,
`019f5ce2-f033-7403-9166-9c4b421e60bb`,
`019f5ce3-0b65-7792-9495-9e5f8db132d5`, and
`019f5ce3-2a8b-7180-b677-df7d4c264025`.

### One structured research-line chain

A Sol-high lead improved a malformed mathematical seed using cheap checks; a
Luna-high executor built a substantial exact packet but its self-review missed
several claim-bearing semantic, control, and provenance defects; a fresh
Sol-low reviewer found them quickly; repair plus targeted verification recovered
the packet. Returning to the same lead produced bounded interpretation, but the
claimed context-reuse saving was not measured.

This episode supports fresh review for similarly complex claim-bearing
executor output and supports cheap investigation before specification. It does
not establish the efficiency of the exact role sequence, that Luna-high caused
the defects, that Sol-high was required, or that persistent-lead reuse had
positive net value. The known avoidable producer cost was one stale-binary full
run; live parent monitoring had near-zero demonstrated benefit. Source threads:
lead `019f5d06-80e0-73e2-9c11-d350ce3275b6`, executor
`019f5d14-05ee-7582-bd9e-c7691c457ae8`, reviewer
`019f5d3b-dce9-7952-94a4-18aeb15219a1`.

### Blind technical-review comparison

Four fresh reviewers inspected the same deliberately defective packet snapshot.
Weighted defect recall was 8/21 for Luna-xhigh/minimal, 15/21 for
Luna-xhigh/structured, 14/21 for Sol-low/minimal, and 15/21 for
Sol-low/structured. Under the structured prompt, Sol-low took 1m39s and about
$0.45 shadow cost versus Luna-xhigh's 7m13s and about $0.49. No cell caught all
known defects. One sample per cell cannot establish a prompt or model effect.

Use Sol-low with a structured, named transition gate as the next prospective
technical-review candidate for similar packets because it tied observed recall
at much lower wall time, not because general superiority was shown. Add live
domain-specific contradictions and negative controls; the generic structure
did not recover every mathematical defect. Source threads:
`019f5fdc-9521-76f1-b8f6-495b37491625`,
`019f5fdc-b634-7a72-9d36-9ff4c53b2239`,
`019f5fdc-d86a-7013-b0bd-53d216f1380b`, and
`019f5fdc-fa7f-75b3-90ff-69a5742b180c`.

## Current Reversible Choices

- Reserve expensive abstract-reasoning configurations for scientific design,
  mathematical cruxes, interpretation, or option search whose possible value
  justifies them. The current evidence does not locate a stable cutoff.
- Use cheaper executors when the measured object, source contract, output, and
  self-checks are explicit. Treat complex executor self-review as evidence, not
  a promotion gate.
- For a similar bounded technical gate, try structured Sol-low plus named
  domain risks before paying for a stronger reviewer; update from downstream
  defects and repair burden.
- Preserve lead context when a line spans dependent packets, but do not claim a
  saving without comparison. A fresh interpreter can be preferable when
  anchoring risk or independence matters more than reconstruction cost.
- Isolate scratch directories when independent runs are meant to measure idea
  diversity or model/prompt differences.

Update this file by pruning or replacing a current choice when source-linked
episodes change a live recurring decision. Keep exact prompts, raw outputs, and
parent scoring outside the skill during evaluation; use
`improve-empirical-research-workflow.md` for the change gate.
