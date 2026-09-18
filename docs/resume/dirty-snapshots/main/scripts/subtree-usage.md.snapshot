# Subtree usage

```sh
python3 scripts/subtree-usage.py
python3 scripts/subtree-usage.py ROOT_THREAD_ID
```

Without an ID, uses `CODEX_THREAD_ID` and follows available parent metadata to
the top-level root. An explicit ID selects precisely that subtree. Run from
any directory using the script's absolute path. Requires only Python's standard
library; reads local logs, makes no network requests and changes no files.

Reports root start/current UTC time, elapsed time, latest recorded usage,
total/last-30-minute/last-5-minute consumption, model breakdown and root versus
all descendants. It does not stop agents or measure Jörn-time. Start a fresh root for a
fresh sprint total; an old root includes its previous work and idle wall time.

## Accounting convention

The checked-in rates are **Standard API-equivalent shadow prices as of
2026-09-16**, not actual subscription quota or billed spend, and not the older
project price table. Sources:

- https://developers.openai.com/api/docs/models/gpt-6-astra
- https://developers.openai.com/api/docs/models/gpt-5.6-sol
- https://developers.openai.com/api/docs/models/gpt-5.6-luna

The Sol/Luna pages also list the comparison rates used for Terra, GPT-5.5 and
GPT-5.4. All usage, including historical usage, is valued at this fixed rate
card. No service-tier/fast-mode surcharge is inferred. API long-context
multipliers apply per recorded request above 272,000 input tokens. Cache-write
tokens, when present, use 1.25 times the uncached-input rate. Tool fees are
excluded. Reasoning output is already included in output, not charged twice.

Counters are `last_token_usage`, deduplicated by unchanged cumulative usage
within each thread. Counter-delta discrepancies raise visible warnings rather
than inventing missing requests or assigning them a guessed model. The first
record uses its last-request count, not a potentially inherited cumulative
counter. Replayed events before thread creation are excluded.

Discovers descendants recursively via parent metadata, counts duplicate
native/archive thread copies once (largest file), and does not treat separately
launched root threads as children. Only locally available logs can be counted.
`--codex-home PATH` selects an imported Codex home containing `sessions/` and/or
`archived_sessions/`. `--now ISO_TIMESTAMP` freezes the reporting cutoff.

Windows are based on usage-event timestamps, not token generation times.
In-flight/unflushed calls are not yet counted. Missing prices or malformed,
inconsistent, or absent usage telemetry print warnings and return exit code 2;
unpriced rows show `+?`, never a fabricated complete dollar total. Unknown or
unavailable descendants cannot be discovered from absent metadata.

Tests: `python3 -m unittest discover -s scripts -p test_subtree_usage.py`.
