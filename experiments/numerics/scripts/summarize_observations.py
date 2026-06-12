#!/usr/bin/env python3
"""Summarize numerics audit JSONL output."""

from __future__ import annotations

import csv
import json
import statistics
import sys
from collections import defaultdict
from pathlib import Path
from typing import Any


NUMERIC_COLUMNS = [
    "mode",
    "algorithm",
    "variable",
    "context_kind",
    "input_pair_kind",
    "oracle_kind",
    "sample_policy",
    "count",
    "with_oracle",
    "max_abs_error",
    "mean_abs_error",
    "p50_abs_error",
    "p95_abs_error",
    "max_rel_error",
]

PREDICATE_COLUMNS = [
    "mode",
    "algorithm",
    "predicate",
    "context_kind",
    "input_pair_kind",
    "oracle_kind",
    "sample_policy",
    "count",
    "with_oracle",
    "disagreements",
    "indeterminate_or_nonbinary",
]


def main(argv: list[str]) -> int:
    if len(argv) not in {2, 3}:
        print(
            "Usage: summarize_observations.py <run-dir-or-events.jsonl> [out-dir]",
            file=sys.stderr,
        )
        return 2

    source = Path(argv[1])
    events_path = source if source.is_file() else source / "events.jsonl"
    out_dir = Path(argv[2]) if len(argv) == 3 else events_path.parent
    rows = read_jsonl(events_path)
    write_outputs(rows, events_path, out_dir)
    return 0


def read_jsonl(path: Path) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    with path.open(encoding="utf-8") as handle:
        for line_number, line in enumerate(handle, start=1):
            stripped = line.strip()
            if stripped:
                try:
                    rows.append(json.loads(stripped))
                except json.JSONDecodeError as error:
                    raise ValueError(f"{path}:{line_number}: invalid JSON: {error}") from error
    return rows


def write_outputs(rows: list[dict[str, Any]], events_path: Path, out_dir: Path) -> None:
    processed = out_dir / "processed"
    processed.mkdir(parents=True, exist_ok=True)
    numeric = summarize_numeric(rows)
    predicates = summarize_predicates(rows)

    write_csv(processed / "numeric_summary.csv", NUMERIC_COLUMNS, numeric)
    write_csv(processed / "predicate_summary.csv", PREDICATE_COLUMNS, predicates)
    (out_dir / "report.md").write_text(
        render_report(rows, events_path, processed, numeric, predicates),
        encoding="utf-8",
    )


def summarize_numeric(rows: list[dict[str, Any]]) -> list[dict[str, Any]]:
    groups: dict[tuple[Any, ...], list[dict[str, Any]]] = defaultdict(list)
    for row in rows:
        if row.get("event") != "observation":
            continue
        groups[numeric_key(row)].append(row)

    output = []
    for key in sorted(groups):
        group = groups[key]
        errors = [float(row["abs_error"]) for row in group if "abs_error" in row]
        rel_errors = [float(row["rel_error"]) for row in group if "rel_error" in row]
        output.append(
            dict(
                zip(NUMERIC_COLUMNS[:7], key),
                count=len(group),
                with_oracle=sum(1 for row in group if "oracle_kind" in row),
                max_abs_error=max(errors) if errors else "",
                mean_abs_error=statistics.fmean(errors) if errors else "",
                p50_abs_error=percentile(errors, 50) if errors else "",
                p95_abs_error=percentile(errors, 95) if errors else "",
                max_rel_error=max(rel_errors) if rel_errors else "",
            )
        )
    return output


def summarize_predicates(rows: list[dict[str, Any]]) -> list[dict[str, Any]]:
    groups: dict[tuple[Any, ...], list[dict[str, Any]]] = defaultdict(list)
    for row in rows:
        if row.get("event") != "predicate_observation":
            continue
        groups[predicate_key(row)].append(row)

    output = []
    for key in sorted(groups):
        group = groups[key]
        output.append(
            dict(
                zip(PREDICATE_COLUMNS[:7], key),
                count=len(group),
                with_oracle=sum(1 for row in group if "oracle_binary" in row),
                disagreements=sum(1 for row in group if row.get("disagrees_with_oracle") is True),
                indeterminate_or_nonbinary=sum(
                    1
                    for row in group
                    if row.get("f64_trinary") not in {None, "true", "false"}
                ),
            )
        )
    return output


def numeric_key(row: dict[str, Any]) -> tuple[Any, ...]:
    return (
        row.get("mode", ""),
        row.get("algorithm", ""),
        row.get("variable", ""),
        row.get("context_kind", ""),
        row.get("input_pair_kind", ""),
        row.get("oracle_kind", "none"),
        row.get("sample_policy", ""),
    )


def predicate_key(row: dict[str, Any]) -> tuple[Any, ...]:
    return (
        row.get("mode", ""),
        row.get("algorithm", ""),
        row.get("predicate", ""),
        row.get("context_kind", ""),
        row.get("input_pair_kind", ""),
        row.get("oracle_kind", "none"),
        row.get("sample_policy", ""),
    )


def percentile(values: list[float], pct: float) -> float:
    if len(values) == 1:
        return values[0]
    ordered = sorted(values)
    rank = (len(ordered) - 1) * pct / 100
    low = int(rank)
    high = min(low + 1, len(ordered) - 1)
    weight = rank - low
    return ordered[low] * (1 - weight) + ordered[high] * weight


def write_csv(path: Path, columns: list[str], rows: list[dict[str, Any]]) -> None:
    with path.open("w", newline="", encoding="utf-8") as handle:
        writer = csv.DictWriter(handle, fieldnames=columns)
        writer.writeheader()
        writer.writerows(rows)


def render_report(
    rows: list[dict[str, Any]],
    events_path: Path,
    processed: Path,
    numeric: list[dict[str, Any]],
    predicates: list[dict[str, Any]],
) -> str:
    observations = [row for row in rows if row.get("event") == "observation"]
    predicate_rows = [row for row in rows if row.get("event") == "predicate_observation"]
    contexts = sorted(
        (row for row in rows if row.get("event") == "context_started"),
        key=lambda row: (
            row.get("object_id", ""),
            str(row.get("sigma", "")),
            row.get("input_pair_kind", ""),
            row.get("sample_policy", ""),
        ),
    )
    oracle_numeric_rows = [row for row in observations if "oracle_kind" in row]
    worst_numeric_rows = sorted(
        (row for row in oracle_numeric_rows if "abs_error" in row),
        key=lambda row: float(row["abs_error"]),
        reverse=True,
    )[:10]
    disagreements = [row for row in predicates if row["disagreements"]]
    exact_rational_disagreements = [
        row
        for row in disagreements
        if row["oracle_kind"] == "exact_rational"
    ]
    exact_binary64_disagreements = [
        row
        for row in disagreements
        if row["oracle_kind"] == "exact_binary64_input"
    ]

    lines = [
        "# Numerics Audit Report",
        "",
        f"Raw events: `{events_path}`",
        f"Processed summaries: `{processed}`",
        "",
        "## Coverage",
        "",
        f"- Contexts: {len(contexts)}",
        f"- Numeric observations: {len(observations)}",
        f"- Predicate observations: {len(predicate_rows)}",
        f"- Oracle-backed numeric observations: {len(oracle_numeric_rows)}",
        "",
        "## Support Status",
        "",
        f"- Exact-rational predicate disagreements: {sum(int(row['disagreements']) for row in exact_rational_disagreements)}",
        f"- Exact-binary64-input predicate disagreements: {sum(int(row['disagreements']) for row in exact_binary64_disagreements)}",
        "- Exact-rational numeric errors are empirical f64-vs-exact measurements for",
        "  the emitted rational fixture contexts.",
        "- Exact-binary64-input rows compare f64 solver output with exact arithmetic",
        "  on the rational values represented by the stored binary64 input.",
        "- Exact-binary64-input rows are not algebraic-source oracle evidence.",
        "",
        "## Emitted Context Bank",
        "",
    ]
    lines.extend(
        markdown_table(
            ["object", "sigma", "input_pair_kind", "sample_policy"],
            [
                [
                    str(row.get("object_id", "")),
                    json.dumps(row.get("sigma", "")),
                    str(row.get("input_pair_kind", "")),
                    str(row.get("sample_policy", "")),
                ]
                for row in contexts
            ],
        )
    )
    lines.extend(["", "## Oracle-Backed f64 Measurements", ""])
    lines.extend(render_oracle_coverage_table(numeric))
    lines.extend(["", "### Largest Row-Level Absolute Errors", ""])
    if worst_numeric_rows:
        lines.extend(render_worst_numeric_rows(worst_numeric_rows))
    else:
        lines.append("- No oracle-backed numeric errors were emitted.")
    lines.extend(["", "## Predicate Agreement Diagnostics", ""])
    lines.extend(render_predicate_summary(predicates))
    lines.extend(["", "### Predicate Disagreements", ""])
    if disagreements:
        lines.extend(
            "- {algorithm}/{predicate} on {input_pair_kind} policy={sample_policy} "
            "oracle={oracle_kind}: disagreements={disagreements}/{count}".format(**row)
            for row in disagreements
        )
    else:
        lines.append("- No predicate disagreements were observed in this run.")
    lines.extend(["", "## Conditioning And Solver Diagnostics Without Oracle", ""])
    lines.extend(render_diagnostic_table(observations))
    lines.extend(
        [
            "",
            "## Interpretation Boundary",
            "",
            "This report supports empirical error-audit claims for the emitted context bank.",
            "Rows labeled `exact_rational` use exact rational source data for the same",
            "polytope that the f64 solver receives after conversion to f64. Rows",
            "labeled `exact_binary64_input` use exact rational arithmetic on the",
            "values represented by the stored f64 inputs.",
            "Diagnostic rows without an oracle are retained as context for conditioning",
            "and solver behavior, not as certified error measurements.",
            "",
        ]
    )
    return "\n".join(lines)


def render_oracle_coverage_table(numeric: list[dict[str, Any]]) -> list[str]:
    oracle_rows = [row for row in numeric if row["oracle_kind"] != "none"]
    if not oracle_rows:
        return ["- No oracle-backed numeric rows were emitted."]
    return markdown_table(
        [
            "input_pair_kind",
            "oracle_kind",
            "algorithm",
            "variable",
            "sample_policy",
            "count",
            "max_abs_error",
            "p95_abs_error",
        ],
        [
            [
                row["input_pair_kind"],
                row["oracle_kind"],
                row["algorithm"],
                row["variable"],
                row["sample_policy"],
                row["count"],
                format_cell(row["max_abs_error"]),
                format_cell(row["p95_abs_error"]),
            ]
            for row in oracle_rows
        ],
    )


def render_worst_numeric_rows(rows: list[dict[str, Any]]) -> list[str]:
    return markdown_table(
        [
            "object",
            "sigma",
            "input_pair_kind",
            "oracle_kind",
            "algorithm",
            "variable",
            "component",
            "f64",
            "oracle_f64",
            "abs_error",
        ],
        [
            [
                row.get("object_id", ""),
                json.dumps(row.get("sigma", "")),
                row.get("input_pair_kind", ""),
                row.get("oracle_kind", ""),
                row.get("algorithm", ""),
                row.get("variable", ""),
                row.get("component", ""),
                format_cell(row.get("f64", "")),
                format_cell(row.get("oracle_f64", "")),
                format_cell(row.get("abs_error", "")),
            ]
            for row in rows
        ],
    )


def render_predicate_summary(predicates: list[dict[str, Any]]) -> list[str]:
    if not predicates:
        return ["- No predicate observations were emitted."]
    return markdown_table(
        [
            "input_pair_kind",
            "oracle_kind",
            "algorithm",
            "predicate",
            "sample_policy",
            "count",
            "with_oracle",
            "disagreements",
            "indeterminate_or_nonbinary",
        ],
        [
            [
                row["input_pair_kind"],
                row["oracle_kind"],
                row["algorithm"],
                row["predicate"],
                row["sample_policy"],
                row["count"],
                row["with_oracle"],
                row["disagreements"],
                row["indeterminate_or_nonbinary"],
            ]
            for row in predicates
        ],
    )


DIAGNOSTIC_VARIABLES = {
    "sigma_min_c",
    "h_eigenvalue",
    "constraint_residual_norm",
    "beta_margin",
    "q_error_bound",
    "positive_eigenvalues",
    "zero_eigenvalues",
    "negative_eigenvalues",
}


def render_diagnostic_table(observations: list[dict[str, Any]]) -> list[str]:
    diagnostics = [
        row
        for row in observations
        if "oracle_kind" not in row and row.get("variable") in DIAGNOSTIC_VARIABLES
    ]
    if not diagnostics:
        return ["- No no-oracle conditioning or solver diagnostics were emitted."]

    groups: dict[tuple[Any, ...], list[dict[str, Any]]] = defaultdict(list)
    for row in diagnostics:
        groups[
            (
                row.get("input_pair_kind", ""),
                row.get("algorithm", ""),
                row.get("variable", ""),
                row.get("sample_policy", ""),
            )
        ].append(row)

    table_rows = []
    for key in sorted(groups):
        group = groups[key]
        values = [float(row["f64"]) for row in group if "f64" in row]
        table_rows.append(
            [
                key[0],
                key[1],
                key[2],
                key[3],
                len(group),
                format_cell(min(values) if values else ""),
                format_cell(max(values) if values else ""),
            ]
        )
    return markdown_table(
        [
            "input_pair_kind",
            "algorithm",
            "diagnostic",
            "sample_policy",
            "count",
            "min_f64",
            "max_f64",
        ],
        table_rows,
    )


def markdown_table(headers: list[str], rows: list[list[Any]]) -> list[str]:
    if not rows:
        return ["- No rows."]
    lines = [
        "| " + " | ".join(headers) + " |",
        "| " + " | ".join("---" for _ in headers) + " |",
    ]
    for row in rows:
        cells = [escape_markdown_cell(format_cell(cell)) for cell in row]
        lines.append("| " + " | ".join(cells) + " |")
    return lines


def format_cell(value: Any) -> str:
    if value is None:
        return ""
    if value == "":
        return ""
    if isinstance(value, float):
        return f"{value:.6g}"
    return str(value)


def escape_markdown_cell(value: str) -> str:
    return value.replace("|", "\\|").replace("\n", " ")


if __name__ == "__main__":
    raise SystemExit(main(sys.argv))
