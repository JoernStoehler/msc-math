# MSc/Codex session postmortem, 3–4 October 2026

Read the [rendered report](report.html) or [Markdown source](REPORT.md). The report leads with outcomes and causal analysis; its expandable appendix supplies selected exact evidence. The shorter [reusable knowledge route](../../knowledge/session-failures-2026-10-03.md) explains applicability, distinctions from earlier knowledge, and what would need rechecking.

The completed packet and lean knowledge note are integrated into canonical `/workspaces/msc-math` and linked from its knowledge index. Historical external artifacts remain explicitly located in the captured worktrees; they were not copied or promoted. Git retention is completed by the integrating owner.

The [reusable output map](OUTPUTS.md) locates working tools, retained mathematical candidates and inactive prototypes. It preserves the distinction between the two worktrees and between accepted, tested, unapproved and withdrawn work.

The eight root sessions were active between **18:24:26.651 and 23:24:26.651 UTC on 3 October 2026**. Earlier context interprets their assignments. Seven children supply supporting review/supervision/probe evidence and are not additional root outcomes. A separately labeled current-postmortem source retains only selected after-freeze process observations; it is neither a child nor a ninth historical case. Household power monitoring and host backups are excluded. Outcomes after the freeze, including the scheduled quota reset, remain outside this assessment.

This package preserves explanations while the human communication window and included compute were available. It does not turn proposed interventions into validated instructions, reopen completed assignments, or authorize new spending. The investigation map in [STRUCTURE.md](STRUCTURE.md) records the coordinating owner's process; the report owns the historical synthesis.

## Evidence and privacy

[evidence.json](evidence.json) records the source key, thread ID, exact frozen source line, timestamp, line hash and selected excerpt. The source inventory records SHA-256 hashes, byte counts, native rollout locations and frozen archival locations. Excerpts retain original spelling and are clearly marked when surrounding text is omitted. Agent statements and confessions are observations of what was said, not independent proof of their causal claims.

Small sanitized code/action/receipt fragments retain the decisive accounting probes, shared fallback command, retrieved rationale, task-state changes, goal transitions and native continuation events. Structured event excerpts preserve selected field names and values exactly and clearly disclose their display reserialization. The runtime appendix retains short source excerpts at a specific release commit, with file and executable provenance and the historical-build limitation. Historical resource aggregates retain the ownership/deduplication method and its limitations separately; live quota and current spending authorization are not mixed into those frozen measurements.

The [final supervisor-remit check](SUPERVISOR-REMIT.md) distinguishes encrypted historical assignment text from prompt omission in native projections. Its two selected receipts and release-source hashes preserve the unresolved parent-brief versus child-judgment boundary.

Full raw logs and investigator working notes remain in the private archival bundle at `/home/joern/.codex/artifacts/session-postmortem-20261004/`. They are not copied into this repository package. No private reasoning, credentials, OAuth URLs or full tool-call bodies are included. The HTML has no JavaScript, external fonts, network-loaded assets, or links that publish the raw corpus. It uses only local report/evidence files and normal source citations.

The selected excerpts make the principal findings inspectable without reopening complete logs. Native hashes and locations preserve a route for authorized checks of omitted context. Historical field/line verification and matching release-source verification are separate from proving a general behavioral cause. The appendix is not a normalized incident count or an assertion of exhaustive causal coverage.

## Retained evaluation

The [six-case transfer study](tests/transfer/REPORT.md) found no robust incremental decision benefit from the earlier knowledge note. Its [protocol and publication map](tests/transfer/PUBLICATION.md) retain exact cases, input packets, anonymous outputs, initial and corrected blind scores, recorded-output checks and projected usage provenance. The publication map qualifies those checks using the later runtime finding: full stored output need not be full effective model input. `tests/transfer/verify.py` checks retained bytes and aggregation without model calls or file changes. This study differs from reconstructing historical incidents; neither is a test of sustained live-root reliability.

The [four-task archival-consumption study](tests/archival-consumption/analysis.md) found no demonstrated reconstruction-cost saving: all four packet consumers took longer and aggregate source reading increased 7.3%. Its [publication map](tests/archival-consumption/PUBLICATION.md) retains exact tested bytes, source-access limits, all answers and initial/corrected judgments. The baseline was already curated, and one pair had unequal historical information. A separate read-only verifier checks hashes and aggregation.

The first report is retained in Git commit `52b1526c`. Version two adds deeper causal evidence after the user challenged the original completion basis. The archival-consumption study uses an immutable candidate snapshot; any final revision is distinguished from that tested packet.

## Rebuild and verification

From the repository root:

```bash
python3 docs/history/session-postmortem-2026-10-04/build_report.py
python3 docs/history/session-postmortem-2026-10-04/build_report.py --check
python3 docs/history/session-postmortem-2026-10-04/tests/transfer/verify.py
python3 docs/history/session-postmortem-2026-10-04/tests/archival-consumption/verify.py
```

Python 3 and Pandoc are required. Rendering uses the retained Markdown and selected evidence only; it does not access the private corpus or the network. The check detects stale rendered content, missing/duplicate local anchors, accidental scripts and a changed root count. It does not validate the analysis.

Where the private frozen corpus is present, independently verify retained source hashes and message excerpt offsets, selected event fields and runtime source excerpts:

```bash
python3 docs/history/session-postmortem-2026-10-04/build_report.py --check --verify-private
```

The current workstation's existing file viewer can display this package by prefixing its absolute path with `https://joern-pc.tailc5e761.ts.net/files`. That hostname and service are delivery conveniences, not portable archival dependencies. Use relative package links after moving the repository.

The integrating owner inspected isolated headless desktop and mobile screenshots. This caught and corrected mobile table-of-contents visibility. A separate isolated Chrome/CDP check verified that direct navigation to a selected evidence fragment opens its containing details element and displays the target in the viewport. The checks cover representative layouts and that one evidence landing, not every scroll position, interaction state or device. Structural rendering, excerpt/hash verification and live delivery are checked separately. The existing signed-in browser profile was not used.
