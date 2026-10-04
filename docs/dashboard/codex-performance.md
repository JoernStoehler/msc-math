# Codex performance diagnostics

[OpenObserve dashboard](https://joern-pc.tailc5e761.ts.net:8443/web/dashboards/view?org_identifier=default&dashboard=7511125879225843712&folder=default&tab=throughput).
[Sanitized saved definition](codex-performance.json). Created 30 September 2026;
API save/read and each graph query verified. Browser rendering remains unverified.

Estimated TPS divides completion counters by elapsed time since the latest preceding
websocket request **in the same conversation**. It includes latency and lacks a
response-ID join; it is not an independently measured streaming decode rate.
Reasoning is included in output. Nonreasoning includes tool/formatting tokens.
The saved query scope is a fixed set of root/descendant IDs, not automatic coverage
of future sessions, independent roots, cloud or Pro work.

In Codex 0.159.2, native `ttft_ms` records the first OutputItemAdded after wrapping
the available stream: [pinned source](https://github.com/openai/codex/blob/rust-v0.159.2/codex-rs/core/src/client.rs#L2392).
It does not measure first reasoning token or isolate queue/network/prefill.
Backend overhead, inference and engine TTFT/TBT histograms also exist, but their
labels lack conversation/request IDs; per-request token joins are not established.
Inference-cache lookup duration, isolated prefill duration/TPS and first-reasoning
latency are not available in the observed exports. Cache-share scatter is correlation.

An [official API-latency incident](https://status.openai.com/incidents/01M3SMF1Q0TDCQVNYSXYG37QMR)
overlaps our observations. It supports a shared-service hypothesis, not a proven
cause or a measured factor relative to normal speed. No billing or model settings
were changed by this diagnostic work.
