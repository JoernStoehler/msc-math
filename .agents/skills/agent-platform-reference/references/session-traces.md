# Classify where a traced agent run spends time

Consult after a slow or poorly supervised research run, before choosing concurrency/model/context changes. Use the actual measured run, including worker traces—not a plausible reconstruction from the final answer.

## What to collect

- Preserve client/provider versions, effective model and effort, run/agent/thread IDs, start/end times, context/compaction events, tool and messaging events, token/usage records, explicit user waits and queue waits.
- Join concurrent workers to their parent task and preserve the scheduling/assumption ledger. Track review/check artifacts and their outcomes separately from elapsed time.
- Record the clock domain and instrumentation coverage. Event receipt/write times may differ from execution boundaries; rollout transcripts are not automatically full execution traces.

## Documented observability surfaces

**Codex:** `codex exec --json` exposes machine events and `turn.completed` usage; app-server exposes account rate-limit read/update methods. Session JSONL and TUI logs depend on effective configuration. Do not assume per-step execution durations exist merely because a transcript line has a timestamp. Sources: `codex-skill-evals`, `chatgpt-work-docs-app-server`, `codex-agents-md`, and tagged protocol/rollout source where relevant.

**OpenAI Agents SDK:** tracing includes agent, generation, function/tool, guardrail and handoff spans; custom processors can export them. Parent task/turn spans and nested agent/tool spans must not be summed as disjoint work. Source: `openai-agents-sdk-openai-agents-python-tracing`.

**Claude Code / Agent SDK:** OTel exports metrics, events and optional traces. The monitoring docs distinguish turn wall duration, API request duration including retries, tool result duration including permission wait/execution, explicit permission decision duration and tool-body execution duration. Session/trace IDs support joins. Export configuration and attributes depend on client/version; a transcript alone may lack these measurements. Source: `claude-code-docs-en-monitoring-usage` (see event/trace tables); `/context` and compaction provide separate context diagnostics, not runtime attribution.

**Ordinary ChatGPT/Claude chat:** these CLI/SDK observability contracts do not automatically exist. Use exported artifacts and whatever telemetry the actual surface provides; state missing measurements instead of inventing them.

## Attribution rules

Report elapsed wall time, summed work and union time separately. If two workers each spend 10 seconds simultaneously, elapsed time may be 10 seconds while summed work is 20. Parent spans overlap child spans; aggregating both double-counts. Classify generation, tool execution, permission/user wait, queue wait, compaction and review only when the measured event/span supports that label.

Concurrent categories can overlap: report category unions and overlap rather than force a partition. Uncovered wall-time intervals are **unobserved**, not “the model was thinking.” Token counts are usage evidence, not duration measurements. Fully traced wall time can still omit provider-internal compute phases or human evaluation of mathematical correctness.

## Reproducible local helper

`scripts/trace_time.py` analyzes an explicitly normalized measured-span JSON format. It reports wall elapsed, sum of span durations, per-category union coverage, any overlap and unobserved time. It rejects absent/reversed span endpoints. Native session/OTel exports require an explicit adapter that preserves provenance and timing semantics; this helper does not pretend to parse every vendor log schema.

`examples/traces/overlap.json` is a **synthetic attribution example**, not a performance measurement. Running the helper demonstrates why concurrent tool work and nested waits cannot be summed as a wall-time breakdown. For real work, retain the raw log, adapter/configuration and normalized trace together before drawing intervention conclusions.
