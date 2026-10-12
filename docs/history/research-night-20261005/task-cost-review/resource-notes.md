# Research-night resource accounting

Scope: 5 October **00:49:45–04:56:25 Berlin** (4 October 22:49:45–5 October 02:56:25 UTC), exactly the approved active-goal interval provided by root. Elapsed walltime: **4 h 6 min 40 s**. This excludes planning/reset setup before goal creation and the new retrospective after cutoff. It includes final checkpoint supervision after science stopped.

The local inventory found 29 threads existing by cutoff; 16 incurred owned usage inside this interval. Thirteen earlier preparatory/released descendants incurred none inside it. Two new retrospective descendants were excluded. Counts use scripts/subtree-usage.py inventory and usage_events; native thread ownership and response IDs reject copied fork counters, mirrors and duplicate owned responses. Each retained thread source and captured SHA256 is in resources.json. No local accounting warnings occurred.

**Observed tokens:** 358,102,403 input, comprising 350,164,096 cached and 7,938,307 noncached; 1,381,978 output, including 462,847 reasoning output. There were 3,380 owned responses. Every counted response supplied reasoning and cache-write fields; cache-write was zero. Reasoning is already inside output. All observed responses used gpt-6.1-sol. Cached context is repeatedly processed across requests, so the 358M figure is not unique text written/read, human work, or quota units.

| Stable lane | Noncached input | Cached input | Output (reasoning included) | Reasoning subtype | Diagnostic token share |
|---|---:|---:|---:|---:|---:|
| HKO F11 acceptance | 317,124 | 8,035,456 | 43,130 | 10,358 | 3.87% |
| P* facet-birth local geometry | 2,317,506 | 90,966,912 | 441,683 | 162,818 | 29.60% |
| geometry mechanisms and source integration | 1,121,169 | 25,086,208 | 147,831 | 42,112 | 13.62% |
| pentagon global bound / matrix extension | 1,543,740 | 61,758,464 | 342,158 | 106,981 | 20.23% |
| root coordination | 796,994 | 37,285,760 | 156,004 | 64,699 | 10.22% |
| scientific comparison and integration | 1,233,093 | 53,547,008 | 205,119 | 74,562 | 15.43% |
| supervision | 608,681 | 73,484,288 | 46,053 | 1,317 | 7.02% |

Share denominator is 9,320,285 **noncached-input-plus-output tokens**, unweighted by model/billing. It is a transparent token-volume diagnostic only. The root, scientific comparator and supervisor consumed 32.68% of that denominator together; this is neither proof that all that work was avoidable nor a measured quota share. Scientific threads also integrated/reviewed/coordinated. The semantic lanes follow thread ownership: direct_cover_lead and its children are charged to pentagon work, and f11_cross_review to HKO work. Geometry lead includes both science and later source integration. A finer per-task join must come from the independent timeline owner.

**Timing:** resources.json provides contiguous 5- and 15-minute bins plus each event with lane, timestamp and counters. Event time is accounting-envelope completion time, not a trace of generation start/end; bins cannot measure exact active reasoning walltime or distinguish computation and inference overlap. Last two 15-minute bins (04:34:45–04:56:25 Berlin, the last clipped at cutoff) contain only root and supervisor usage. Earlier final bin also includes final science/source packaging. Root timeline records are required to assess what could have been started, overlapped or omitted.

**Quota ledger from native rollout snapshots:** account-wide primary weekly window (10,080 minutes), secondary absent; reset target remained around 11 October 10:35:01 Berlin with observed 1–2-second variants. The account observation went from 1% used at goal start to 8% by 04:18 Berlin and remained at 8% through cutoff. That is **7 rounded percentage points during this interval**, not a measured 8–9% share attributable to this task. Different sessions/accounts surfaces can contribute; conversion from these tokens to subscription quota is unestablished. Snapshots and precise sources are in resources.json. The separate reset and later 9% observation are outside this accounting interval.

| First observed Berlin | Weekly used |
|---|---:|
| 05 Oct 00:49 | 1% |
| 05 Oct 00:59 | 2% |
| 05 Oct 01:25 | 3% |
| 05 Oct 01:57 | 4% |
| 05 Oct 02:29 | 5% |
| 05 Oct 03:04 | 6% |
| 05 Oct 03:41 | 7% |
| 05 Oct 04:18 | 8% |

**Native corroboration:** 17 bounded OpenObserve queries covered the whole interval using safe metadata projection only. Every one of the 3,380 local per-thread counter tuples had a native match. Queries did not report partial results or hit the 10,000-row limit. Native additionally emitted 20 identical-shape initial rows (12,872 input, zero cache/output) across spawned threads and one extra comparator row (94,349 input, 93,696 cached, 676 output); three native completions lacked counters. Native response IDs were not projected, so these extras cannot be proven independently incurred/deduplicated. They are retained separately, not silently added. Thus 3,380 is a well-corroborated owned-record subtotal, not a claim of complete subscription billing or complete native exporter behavior.

**Unavailable measurements:** no actual monetary subscription spending is attributed here. No paid API dollar amount is inferred, and shadow prices were not computed. File-edit/test/read proportions by walltime or tokens are unknown: tool invocation counts cannot supply those shares; argument content/output/reasoning classification would require its own validated coding and resource join. Recorded reasoning output is 33.49% of output tokens, but that is not 33.49% of work/time and does not measure unrecorded internal activity. Cache rates likewise are not allocation or mathematical productivity measures.

**Data outputs:** resources.json is machine-readable totals, lanes, threads, event-level counters, 5/15-minute bins, quota snapshots, metadata-only native crosschecks and caveats. Scratch scripts in ownscratch reproduce the accounting; they read logs/telemetry and write only this assigned scratch package. No research, reset, config, manuscript or coordination state was modified.

After the reported scientific/source closure at **04:28 Berlin**, through goal pause at **04:56:25**, recorded usage was:

| Lane | Responses | Noncached input | Cached input | Output | Reasoning subtype |
|---|---:|---:|---:|---:|---:|
| root coordination | 38 | 25,736 | 3,036,416 | 9,581 | 4,205 |
| supervision | 59 | 57,936 | 9,414,144 | 6,138 | 461 |

Other semantic lanes had 0 responses after this closure timestamp. This window consumed 99,391 noncached-plus-output tokens; 28 min 25 s of walltime elapsed. Those tokens measure checkpoint/wait/supervision activity; they do not establish that research was impossible during that time.

Reproduction: from the repository root, run `python3 /tmp/msc-math/night-cost-review/ownscratch/account.py`, then source `~/.config/openobserve/codex-env.sh` and run `native.py` followed by `native-full.py` in that same scratch directory. The first script regenerates the base JSON; subsequent scripts add safe native metadata crosschecks and redact credit-balance metadata. Final prose and shares are a derived report and not generated by those three scripts. Source SHA256 values capture the logs as read; the root rollout continues growing after this snapshot.
