# MSc/Codex session postmortem, 3–4 October 2026

Read the [rendered report](report.html) or [Markdown source](REPORT.md). The report leads with outcomes and causal analysis; its expandable appendix supplies selected exact evidence. The shorter [reusable knowledge route](../../knowledge/session-failures-2026-10-03.md) explains applicability, distinctions from earlier knowledge, and what would need rechecking.

The [reusable output map](OUTPUTS.md) locates working tools, retained mathematical candidates and inactive prototypes. It preserves the distinction between the two worktrees and between accepted, tested, unapproved and withdrawn work.

The eight root sessions were active between **18:24:26.651 and 23:24:26.651 UTC on 3 October 2026**. Earlier context interprets their assignments. Three children supply supporting review/supervision evidence and are not additional root outcomes. Household power monitoring and host backups are excluded. Outcomes after the freeze, including the scheduled quota reset, remain outside this assessment.

This package preserves explanations while the human communication window and included compute were available. It does not turn proposed interventions into validated instructions, reopen completed assignments, or authorize new spending. The investigation map in [STRUCTURE.md](STRUCTURE.md) records the coordinating owner's process; the report owns the historical synthesis.

## Evidence and privacy

[evidence.json](evidence.json) records the source key, thread ID, exact frozen source line, timestamp, line hash and selected excerpt. The source inventory records SHA-256 hashes, byte counts, native rollout locations and frozen archival locations. Excerpts retain original spelling and are clearly marked when surrounding text is omitted. Agent statements and confessions are observations of what was said, not independent proof of their causal claims.

Small sanitized code/action/receipt fragments retain the decisive accounting probes, shared fallback command, retrieved rationale and actual goal transitions. Historical resource aggregates retain the ownership/deduplication method and its limitations separately; live quota and current spending authorization are not mixed into those frozen measurements.

Full raw logs and investigator working notes remain in the private archival bundle at `/home/joern/.codex/artifacts/session-postmortem-20261004/`. They are not copied into this repository package. No private reasoning, credentials, OAuth URLs or full tool-call bodies are included. The HTML has no JavaScript, external fonts, network-loaded assets, or links that publish the raw corpus. It uses only local report/evidence files and normal source citations.

The selected excerpts make many findings inspectable without reopening complete logs. Some technical conclusions depend on the raw receipts explicitly cited in the report; their private hashes and locations preserve a route for authorized later checking. The appendix is not a normalized incident count or an assertion of exhaustive causal coverage.

## Rebuild and verification

From the repository root:

```bash
python3 docs/history/session-postmortem-2026-10-04/build_report.py
python3 docs/history/session-postmortem-2026-10-04/build_report.py --check
```

Python 3 and Pandoc are required. Rendering uses the retained Markdown and selected evidence only; it does not access the private corpus or the network. The check detects stale rendered content, missing/duplicate local anchors, accidental scripts and a changed root count. It does not validate the analysis.

Where the private frozen corpus is present, independently verify retained source hashes and visible-message excerpt offsets:

```bash
python3 docs/history/session-postmortem-2026-10-04/build_report.py --check --verify-private
```

The current workstation's existing file viewer can display this package by prefixing its absolute path with `https://joern-pc.tailc5e761.ts.net/files`. That hostname and service are delivery conveniences, not portable archival dependencies. Use relative package links after moving the repository.

Browser visual inspection was unavailable during initial assembly because no connected browser was reported. Structural rendering, excerpt/hash verification and live delivery are checked separately; any later visual or editorial review belongs in the coordinating owner's completion record.
