"""Generate a compact, derived report without replacing raw exact evidence."""
from __future__ import annotations

import json
import sys
from pathlib import Path


def main() -> None:
    out = Path(sys.argv[1]) if len(sys.argv) > 1 else Path("artifacts/retained-exact")
    rows = [json.loads(line) for line in (out / "raw_rows.jsonl").read_text().splitlines() if line]
    summary = []
    for row in rows:
        statuses = {}
        for candidate in row["candidates"]:
            key = (candidate["f64_status"], candidate["retained_exact_status"])
            statuses["|".join(key)] = statuses.get("|".join(key), 0) + 1
        generation = row["candidate_generation_ms"]
        summary.append(
            {
                "case_id": row["case_id"],
                "stream_candidates_rejected": row["f64_rejected_count"],
                "retained_count": len(row["candidates"]),
                "retained_exact_accept_count": row["retained_exact_accept_count"],
                "retained_exact_reject_count": row["retained_exact_reject_count"],
                "f64_to_exact_strata": statuses,
                "scalar_agreement": row["scalar_agreement_retained_vs_all"],
                "minimizer_agreement": row["minimizer_agreement_retained_vs_all"],
                "window_agreement": row["window_agreement_retained_vs_all"],
                "current_vs_retained": {
                    "scalar": row["scalar_agreement_current_vs_retained"],
                    "minimizer": row["minimizer_agreement_current_vs_retained"],
                    "window": row["window_agreement_current_vs_retained"],
                },
                "candidate_generation_ms": generation,
                "current_minimasafe_ms": row["current_minimasafe_ms"],
                "retained_exact_ms": row["retained_exact_ms"],
                "exact_all_reference_ms": row["exact_all_reference_ms"],
                "retained_exact_over_generation": row["retained_exact_ms"] / generation,
                "exact_all_over_retained_exact": (
                    None
                    if row["exact_all_reference_ms"] is None
                    else row["exact_all_reference_ms"] / row["retained_exact_ms"]
                ),
            }
        )
    (out / "analysis.json").write_text(json.dumps({"schema_version": "qp-retained-exact-analysis-v1", "cases": summary}, indent=2) + "\n")
    lines = [
        "# Retained-exact route summary",
        "",
        "This report is derived from `raw_rows.jsonl`; exact rational values and per-sigma decisions remain only in that raw artifact.",
        "",
        "| case | stream | f64 T/I/rej | retained exact A/R | current→retained S/M/W | retained→exact-all S/M/W | exact ms / retained ms |",
        "|---|---:|---:|---:|---|---|---:|",
    ]
    for row in rows:
        current_agree = "/".join("yes" if row[key] else "no" for key in ("scalar_agreement_current_vs_retained", "minimizer_agreement_current_vs_retained", "window_agreement_current_vs_retained"))
        agree = "/".join("yes" if row[key] else "no" for key in ("scalar_agreement_retained_vs_all", "minimizer_agreement_retained_vs_all", "window_agreement_retained_vs_all"))
        lines.append(
            f"| {row['case_id']} | {row['sigma_stream_count']} | {row['f64_true_count']}/{row['f64_indeterminate_count']}/{row['f64_rejected_count']} | {row['retained_exact_accept_count']}/{row['retained_exact_reject_count']} | {current_agree} | {agree} | {row['exact_all_reference_ms']:.1f} / {row['retained_exact_ms']:.1f} |"
        )
    lines += [
        "",
        "The timings are wall-clock observations for the named scopes: candidate generation, ordinary `MinimaSafe`, exact recheck of every retained candidate, and exact-all reference over the supplied stream. They do not include compilation.",
    ]
    (out / "summary.md").write_text("\n".join(lines) + "\n")


if __name__ == "__main__":
    main()
