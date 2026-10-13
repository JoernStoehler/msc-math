# Steer event timeline (bounded slice)

**Window:** 2026-10-13 19:22:20–19:24:02 UTC, thread `01a136b0-aa22-7172-bdb4-2144008a0fba`. Sources: this thread’s rollout and indexed local `logs_2.sqlite`; only event/mode/lifecycle metadata was inspected or retained. This is a narrow event trace, not a full transport or client-delivery trace.

| UTC | Durable rollout observation | Indexed log observation |
|---|---|---|
| 19:22:21.983 | Prior assistant task completed. | Stream/session trace activity around 19:22:21.5–21.98. |
| 19:22:27.109–27.145 | New task started; user message is durable at 27.144. | Handler/world-state and configuration/feedback-tag records follow within ~0.01–0.03 s. |
| 19:22:49.566 | Reasoning item completed; user steer recorded at 19:22:50.128. | Stream traces at :49.56–:49.71. Two SSE records at :50.028 and :50.115; one contains an `interrupted` marker. Session-turn/world-state trace follows. |
| 19:23:02.341, :10.170, :23.297, :38.616, :40.988, :47.910, :51.754 | Each is a durable user message (the interval from prior input ranges ~2–15 s). No assistant response item precedes them in this slice. | Each input is followed by handler/SSE/turn/world-state records within roughly 0.1–0.3 s. No `interrupted` marker was found on these later SSE rows. |
| 19:23:54.305–56.587 | Assistant commentary item completed at :56.484 and is durable as an assistant response at :56.486. | Stream event utility records at :54.305, :56.484, :56.587. First observed assistant response is ~4.7 s after the last steer. |
| 19:23:58.938 | — | Stream event utility info record. |
| 19:24:01.839–01.877 | Next assistant/tool activity begins just beyond the requested bound; excluded from primary timeline. | Tool/parallel and session lifecycle log rows begin. |

## What this supports

- The user inputs were durably recorded; the local rollout does not show an absent-input/durability gap.
- The local trace is consistent with fast intake/handler activity after each steer, with no assistant-facing text until the commentary at 19:23:56. The visible gap is therefore downstream of durable user-message recording and upstream of the first durable assistant response, but this trace does not isolate model time, orchestration, buffering, or UI delivery.
- There is one explicit `interrupted` marker in an SSE log row near 19:22:50. This is evidence of an interrupted stream in the trace, not sufficient evidence that every steer cancels and restarts sampling. Later input clusters do not show the same marker in the bounded SSE rows.
- No explicit disconnect, timeout, or failed marker was observed in the sampled thread rows. Absence in this narrow application log slice does not rule out a client/UI delivery issue or unlogged backend stall.
- Repeated feedback/config log records are auxiliary activity and are not counted as independent sampling restarts. The rollout alone establishes user-message timing and assistant response timing, not whether each user message was injected into an already running model call or launched a new one.

## Evidence boundary / follow-up

The available local logs establish the InstantInterrupt feature flag was active, but do not establish token-generation start/stop boundaries per steer, client rendering/acknowledgment, or backend request correlation with the rollout items. Treat “one interrupted SSE event, then repeated intake traces, then first response after the last steer” as the measured result; causation and user-visible delivery remain open. This report intentionally omits request IDs, message payloads, model identifiers, and unrelated log data.
