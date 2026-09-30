# Workstation OpenTelemetry setup

Prepared 2026-09-30; updated after Jörn explicitly authorized installation in
`~/.dotfiles`. Status: **backend installed and verified; shared Codex daemon
restarted by Jörn and all three telemetry signals verified live**.
Originating thread: `01a0f18c-14b4-78e2-a7ef-91e0d7b23f0a`.
Component wrapped up: host implementation committed in dotfiles at `c2e8e54`.
No implementation work remains. The originating coordinator owns integration
of this completion into the shared registry and environment summary.

## Installed outcome (2026-09-30)

The host implementation and operations are owned by
[`~/.dotfiles/INSTALL.md`](/home/joern/.dotfiles/INSTALL.md#workstation-codex-opentelemetry),
`scripts/setup-openobserve.sh`, the `openobserve` package, the user service
`openobserve.service`, and the host Codex configuration. OSS v1.0.4 is enabled
as a user service. The private UI is
<https://joern-pc.tailc5e761.ts.net:8443/>; existing Tailscale routes were preserved.
Runtime data and authentication stay outside Git. No sandboxes or cloud VMs
were configured. Jörn's explicit workstation setup request supersedes the
proposal's installation gate below; that text remains the earlier design record.

Two isolated idle Codex app-server sessions verified native log and trace export;
explicit metrics enablement produced 90 native metric streams. A PromQL range
query returned four series and 24 finite samples. A clean backend restart
preserved 27 log records and 1,033 spans at the observation time. No model prompt
was sent. Service memory was about 300 MiB and storage about 81 MiB around
10:08 UTC, with a 2 GiB memory cap, one CPU-equivalent cap, zero swap allowance,
and seven-day global retention. These are startup observations, not sustained
load or retention-expiry validation. Private HTTPS served the app HTML;
interactive browser/device checks were unavailable in this TUI.

Jörn restarted the shared daemon on 2026-09-30 at 10:32:33 UTC. The replacement
process has all three authentication environment variables. Queries restricted
to the period after that restart returned 88 log records, 7,451 spans and two
metric series with two samples at verification. Authentication cutover is
complete; these counts are observations, not completeness guarantees.

Maintenance entry point: the linked dotfiles `INSTALL.md` owns status, storage
review, updates, recovery and disabling export. Stop/start the backend with
`systemctl --user stop/start openobserve.service`; review disk use against the
documented 10 GiB threshold. Interactive browser login was not verified here;
API ingestion, queries and private HTTPS were verified. No additional services,
dashboards, alerts or external-environment exporters are part of this component.

## Earlier proposal

## Scope and recommendation

Jörn's confirmed topology and priorities are in
[development-environments.md](../development-environments.md): Ubuntu hosts
generic OTel storage, local Codex TUI processes export, and Claude microVM and
Astra Pro need not export. Setup and maintenance effort are the deciding factors.

Recommend **OpenObserve OSS, single node, local disk**: verify logs first, then
try metrics and traces within the same bounded pilot. Jörn's follow-up questions
express interest in these signals and alternatives; installation approval is
still pending.
Compare it with **SigNoz Community using its supported Foundry/Compose setup**.
The recommendation is an engineering judgment from component count and the
needed records, not a measured comparison. No backend has been installed or
tested by this task. No service, configuration, coordination registry, or global
guidance has been changed; this proposal is the only owned file.

## Two off-the-shelf options

| Consideration | OpenObserve OSS | SigNoz Community |
| --- | --- | --- |
| Deployment | One binary or container; choose a pinned Linux binary plus a user service for this workstation | Foundry generates and manages Docker Compose; current example includes SigNoz, collector, ClickHouse, Keeper and PostgreSQL |
| Records and UI | OTLP ingestion; searchable logs and SQL; backend also supports metrics/traces | OTLP ingestion; log explorer with live, table and time-series views; broader observability platform |
| Durable storage | Local Parquet stream data and SQLite metadata | ClickHouse telemetry store and separate metadata store; persisted deployment volumes |
| Setup estimate | 60–120 minutes including Codex integration, private access and verification | 2–4 hours including Foundry, stack, access and verification |
| Resource expectation | No verified footprint; propose a 2 GiB trial memory ceiling and explicit cache/memtable settings | Official prerequisite is at least 4 GB allocated to Docker; actual use still needs measurement |
| Maintenance estimate | 15–30 minutes/month for a small installation, plus occasional upgrade issues | 30–60 minutes/month, plus database/stack upgrade issues |
| Main gap | Generic event explorer needs a few useful queries; no automatic project/task interpretation | More components to maintain; richer UI still cannot supply missing Codex fields or external activity |

Estimates are planning allowances, not vendor guarantees. OpenObserve's
[quickstart](https://openobserve.ai/docs/getting-started/),
[OTLP ingestion](https://openobserve.ai/docs/ingestion/logs/otlp/) and
[storage documentation](https://openobserve.ai/docs/administration/maintenance/storage-management/storage/)
support its deployment and storage description. SigNoz's
[current Docker installation guide](https://signoz.io/docs/install/docker/)
and [log explorer guide](https://signoz.io/docs/userguide/logs_query_builder/)
support the alternative. Its old `deploy/` Compose files and install script
are now deprecated; do not plan against an older tutorial.

### Grafana comparison and minimum scope

Grafana's [all-in-one OTLP image](https://grafana.com/docs/opentelemetry/docker-lgtm/)
is also easy to start and can persist its data. Its stated target is development,
demos and testing. It bundles several systems behind one container; that is a
credible pilot alternative, rather than an inherently difficult setup.
A separately maintained Grafana/Loki/Tempo/metrics-backend stack creates more
configuration and upgrade surfaces. Even Loki can
[run as one process](https://grafana.com/docs/loki/latest/get-started/deployment-modes/),
so the comparison should not assume a Kubernetes deployment. For one workstation
without an existing Grafana investment, OpenObserve's integrated single process
remains the proposed simpler ongoing arrangement. This is a judgment about
operational scope, not a demonstrated superiority in performance or reliability.

Required: authenticated private UI, local persistent data, Codex OTLP export,
searchable session/error events, short explicit retention, resource limits and
verification, and a basic start/stop/update/recovery procedure. Try native
metrics/traces without adding services, but keep only useful, verified signals.
Codex documents request/tool counters and duration histograms; its trace exporter
exists, while exact span coverage still needs observation. Allow approximately
30–60 minutes for those extra checks once logs work, within the existing two-hour
attempt limit. These are unmeasured estimates.

Defer collectors unless direct export requires one; also defer Kubernetes,
clustering, object storage, external-agent exporters, host/container scraping,
custom dashboards, alerting, sampling policies and long-term telemetry backups.
Telemetry remains separate from project assignments and subscription accounting.

## What Jörn would get

The [official Codex telemetry documentation](https://learn.chatgpt.com/docs/config-file/config-advanced#observability-and-telemetry)
describes conversation IDs/model metadata, requests and streaming events,
prompt metadata, tool decisions/results and token counts on some completed
responses. Logs export is opt-in; batching and shutdown flush introduce delay.
Keep `log_user_prompt = false`. Tool output snippets can still contain project
content, so prompt redaction does not mean content-free records.

The [configuration reference](https://learn.chatgpt.com/docs/config-file/config-reference)
exposes separate log, trace and metrics exporters and places telemetry routing
outside project-local configuration. The first pilot would use per-invocation
configuration for one fresh TUI process. A permanent workstation-wide exporter
would be a later, explicit implementation choice.

Proposed UI views, subject to checking the actual ingested schema:

- Recent events for a selected conversation, ordered by timestamp.
- Failed requests/tools and their observed duration, grouped by model/tool.
- Token counts from supported completion events, with missing fields visible.

This provides a searchable local activity record. It does **not** establish
which thesis obligations are neglected, mathematical validity, completed
assignments, billed subscription spend, remaining quota, human time, or a
complete command/transcript archive. Conversation IDs do not guarantee parent
relationships, project/worktree labels, titles or task assignments; inspect
those fields before promising joins. No historic backfill is planned.

Claude and Pro remain outside telemetry coverage. Coordination records can
represent their assignments and returned outcomes; absent usage stays unknown.
The existing [subtree usage reader](../../scripts/subtree-usage.md) remains a
separate local-log accounting surface with its stated limitations and shadow
prices. Do not replace it with a naive sum of OTel events: retries, replay and
SSE/WebSocket coverage must first be understood.

## Proposed implementation, only after review

1. Check the selected OSS release, checksums, existing ports and private browser
   route. Use a pinned binary, an explicit data directory outside worktrees,
   and credentials outside Git. Keep SQLite, WAL and stream files together for
   recovery; copying only Parquet files is insufficient.
2. Initially run the backend as a bounded foreground pilot, bound to loopback.
   Send one fresh local TUI's OTLP/HTTP binary logs directly to
   `http://127.0.0.1:5080/api/default/v1/logs`, with backend authentication and
   a `codex` stream header. This combines the two products' documented protocol
   support; direct interoperability has not been tested. Do not add a collector
   unless this fails or filtering/enrichment becomes necessary.
3. Set seven-day retention explicitly. Limit memory/cache settings rather than
   relying on host-relative defaults. Observe process memory, CPU and all data
   directories. Start with a 2 GiB memory ceiling, one CPU-equivalent ceiling,
   and a 10 GiB disk review threshold; these are proposed limits, not a measured
   minimum. A directory-size threshold is not a hard filesystem quota.
4. Provide the UI through the workstation's existing Tailscale private access,
   preferably its private HTTPS reverse-proxy facility while ingestion stays
   loopback. Resolve any existing route conflicts first. Test from Chromebook
   and phone; a localhost link alone is insufficient. No public sharing.
5. Compare one ordinary short task with its received records. Check conversation
   ID, model, ordering, prompt redaction, tool results and any usage fields.
   Test clean shutdown flush and backend restart persistence. Briefly stop the
   backend to establish whether export failure interferes with TUI work and
   what records are lost; durable offline replay is not promised.
6. Return the measured outcome and a real browser URL to Jörn. Keep the pilot
   temporary until usefulness and resource use are accepted. Only then arrange
   a restartable user service, document start/stop/update/recovery, and decide
   whether other workstation TUI processes should export.

The endpoint and header shape come from the linked OTLP ingestion page. The
exact release, service unit and Codex override syntax belong to implementation;
this document is not an executable installation script.

## Retention, resources and cost

The [OpenObserve settings reference](https://openobserve.ai/docs/administration/configuration/environment-variables/)
documents global/stream retention, a ten-year global default, host-relative
memtable sizing, and optional vendor telemetry. Proposed configuration uses
seven days and disables OpenObserve's vendor telemetry. Actual deletion can
lag expiry; retain headroom for WAL, indexes and compaction. A single local
disk is a single failure domain. Treat the rolling records as disposable
diagnostics initially; retain any important findings separately before expiry.

SigNoz documents configurable per-signal retention, but its
[installation guide](https://signoz.io/docs/install/docker/) says seven-day
logs/traces defaults while its
[retention guide](https://signoz.io/docs/userguide/retention-period/) says
fifteen. Explicitly set and verify seven days if that alternative is chosen.

Read-only workstation observations at approximately **2026-09-30T09:33Z**:
Codex CLI 0.159.2; user configuration has no `[otel]` table; Docker and
Tailscale executables resolve. The host reports 15 GiB RAM total, approximately
9.1 GiB available, and 110 GiB disk available. These are transient readings,
not reserved capacity or evidence that services/browser access are healthy.
The current installed package contains no source used to verify exporter
behavior; current official docs supply the contract pending a live pilot.

No ingestion-volume estimate is measured. Illustratively, 10,000 events/day at
2 KiB each gives about 137 MiB raw payload over seven days; 100,000 at 10 KiB
gives about 6.7 GiB. Compression, indexes, WAL and transient files change actual
disk use. Do not apply a vendor marketing compression ratio to these records.

Proposed software/service expenditure: **€0 additional**, using self-hosted OSS
on existing hardware; electricity and disk wear are unmeasured. No new paid
model benchmark is needed. The ordinary verification turn consumes whatever
the current account normally charges; actual quota/billing is unknown.
Allow Jörn 5–10 minutes for review and 5–10 minutes for device checks.

Cap the first implementation attempt at **two hours**, including a 15–30 minute
bounded ingestion observation. Stop and report if direct export fails, the
resource ceilings prove insufficient, or the expected UI cannot be obtained
without substantial custom work. Do not silently switch to SigNoz or start
sustained collection. Longer retention, backups, collectors, alerting, and
project-dashboard integration are later decisions.

## Review decision

Given the priority of low setup and maintenance, does Jörn accept this
OpenObserve OSS pilot, checking logs then metrics/traces, with seven-day
retention, private Tailscale UI access and a two-hour implementation limit,
or prefer the SigNoz or Grafana alternative? Installation and sustained execution
await that review, as
explicitly required by the delegated task.
