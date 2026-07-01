# AI-Use Provenance

This folder owns the evidence packet for thesis AI-use provenance. It supports
the thesis disclosure and PaperOrchestra inputs; it does not prove mathematical
correctness of any thesis result.

The current committed report is
[`reports/ai-provenance-log-backed-summary.md`](reports/ai-provenance-log-backed-summary.md).
It is a log-backed interaction-provenance model over available Codex and Claude
session archives. Treat it as evidence for who steered, drafted, implemented,
reviewed, accepted, or rejected work in the inspected sessions. Do not use it
as source truth for theorem correctness.

The repo-facing report intentionally keeps the aggregate model and per-area
classification, not the detailed per-session summaries. Detailed transcript
tables are scratch investigation artifacts; raw logs remain the source evidence
when a later refresh needs to re-check them.

## Trust Boundary

- Raw session logs under `/home/vscode/.codex` and `/home/vscode/.claude` are
  the source evidence.
- The report is an LLM-authored synthesis from those logs, not a deterministic
  derivation.
- The scripts in `scripts/` inventory available logs and check that cited
  evidence paths exist. They do not replace the LLM/human synthesis step.
- Git history is not evidence for idea provenance.
- User messages are evidence for what Jörn provided in chat, but not a complete
  record of offline thinking or meetings.
- Assistant messages are evidence for what AI produced or attempted, but not by
  themselves evidence that Jörn accepted the output.

## Files

| Path | Role |
| --- | --- |
| `reports/ai-provenance-log-backed-summary.md` | Current provenance synthesis for thesis/PaperOrchestra use. |
| `reports/session-log-import-report-2026-07-01.md` | Import/coverage report referenced by the synthesis. |
| `prompts/ai-provenance-investigation-prompt.md` | Prompt used to rerun the provenance investigation in a fresh session. |
| `scripts/collect_log_inventory.py` | Deterministically inventories visible Codex/Claude session logs. |
| `scripts/check_report_evidence.py` | Checks absolute evidence paths cited by a report and writes a JSON check artifact. |

Generated check artifacts go under `artifacts/`; this directory is ignored
because it can contain local absolute session-log paths.

## Reproduce Or Refresh

Inventory visible logs:

```bash
python3 experiments/ai-use/scripts/collect_log_inventory.py \
  --out experiments/ai-use/artifacts/log-inventory.json
```

Check the current report's cited evidence paths:

```bash
python3 experiments/ai-use/scripts/check_report_evidence.py \
  experiments/ai-use/reports/ai-provenance-log-backed-summary.md \
  --out experiments/ai-use/artifacts/report-evidence-check.json
```

To refresh the prose report, start a new Codex session with
[`prompts/ai-provenance-investigation-prompt.md`](prompts/ai-provenance-investigation-prompt.md),
then replace `reports/ai-provenance-log-backed-summary.md` with the reviewed
output and rerun the evidence check.

## Current Use

For the PaperOrchestra trial, use this report as the AI-use provenance source.
If origin is uncertain in the report, keep it uncertain in generated inputs or
thesis prose. Do not let the paper-writing pipeline infer authorship from
generic assumptions, style, or git history.
