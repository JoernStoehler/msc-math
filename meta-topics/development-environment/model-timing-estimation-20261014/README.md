# One-turn model timing estimate, 14 October 2026

Jörn asked where the delegation turn's time went and pointed out that missing
direct reasoning timing does not prevent fitting an estimate. This retains a
small estimator and metadata-only evidence. It is not automatic future reporting
or a universally calibrated performance model.

The root turn was `01a13a43-69bc-70a3-9b44-2c3a80cbe2a6` in conversation
`01a13a12-2669-7f12-9c6e-ed170781c4aa`, from 11:54:18.954 to 12:03:55.338 UTC,
576.384 seconds. The local OpenObserve `codex` stream was queried on 14 October
for that conversation and time interval, projecting only event timestamps,
names/kinds, durations, TTFT and token counters. Credentials and prompts are
excluded from [model-events.json](model-events.json).

The first pending websocket request through the next `response.completed` forms
a logical cycle; reconnect requests within it remain part of that cycle. Fifteen
cycles total 574.202 seconds. Short tool execution can overlap response streaming,
so tool times are not simply added to that total.

## Fit and reproduction

```bash
uv run meta-topics/development-environment/model-timing-estimation-20261014/fit-timing.py \
  meta-topics/development-environment/model-timing-estimation-20261014/model-events.json
```

Ordinary least squares fits

`cycle seconds = base + bR * reasoning tokens + bN * other output tokens + bC * reconnect count`.

| Component | Coefficient | Estimated total |
| --- | ---: | ---: |
| Base | 4.6507 seconds/cycle | 69.76 seconds |
| Reasoning | 0.042665 seconds/token | 293.02 seconds |
| Other output | 0.029527 seconds/token | 163.37 seconds |
| Reconnect | 24.0219 seconds/event | 48.04 seconds |

The script reproduces these numbers. In-sample RMSE is 2.91 seconds/cycle;
leave-one-out prediction RMSE is 6.12 seconds. Leaving one observation out changes
the reasoning allocation to 283.49–301.87 seconds. The initial scratch fit also
performed 5,000 deterministic bootstrap resamples: 4,413 full-rank fits yielded
2.5/50/97.5 percentiles of 232.97/292.92/315.95 seconds for reasoning allocation;
[timing-fit.json](timing-fit.json) retains this extra sampling diagnostic.

These are model allocations, not direct timing of hidden reasoning. Approximately
constant token-class generation rates and additive overhead are assumptions.
Rate changes, correlated overhead, contextual effects and retry behavior can bias
the coefficients. One long nonreasoning-output cycle provides substantial leverage.
Leave-one-out and bootstrap results omit systematic model error; they are not
guaranteed bounds on true reasoning time. Fifteen cycles from one turn do not
establish portability to other models, dates or contexts.

The retained exporter does not identify first-reasoning-token arrival separately.
Codex's `ttft_ms` records the first `OutputItemAdded` after wrapping the stream:
[inspected source](https://github.com/openai/codex/blob/092d3acd6bec3e3a14bdc7e7a2810ab628ab759d/codex-rs/core/src/client.rs#L2484).
This is not evidence that OpenAI has no other timing interfaces.
