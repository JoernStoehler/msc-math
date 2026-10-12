#!/usr/bin/env python3
"""Summarize dev-QP KKT error-audit JSONL.

This is intentionally smaller than the exploratory scratch summarizer.  It
keeps the live questions separate: ternary predicate soundness, Q-bound
coverage, and audited capacity isolation.
"""

from __future__ import annotations

import argparse
import csv
import json
import math
from collections import defaultdict
from pathlib import Path
from typing import Any, Callable

PredicateFn = Callable[[dict[str, Any]], str | None]

PREDICATES: dict[str, PredicateFn] = {
    "current_f64_verdict": lambda row: row.get("current_f64_verdict"),
    "verified_inverse_beta_radius_verdict": lambda row: row.get(
        "verified_inverse_beta_radius_verdict"
    ),
}

Q_BOUNDS = {
    "current_q_error_bound": "current_q_bound_covers_exact",
    "verified_inverse_beta_radius_q_bound": (
        "verified_inverse_beta_radius_q_bound_covers_exact"
    ),
}


def main() -> None:
    args = parse_args()
    rows = [row for row in read_rows(args.inputs) if row.get("status") == "ok"]
    args.out_dir.mkdir(parents=True, exist_ok=True)
    write_csv(args.out_dir / "predicate_summary.csv", predicate_summary(rows))
    write_csv(args.out_dir / "q_bound_summary.csv", q_bound_summary(rows))
    capacity_rows = capacity_impact_summary(rows)
    write_csv(args.out_dir / "capacity_impact_summary.csv", capacity_rows)
    write_csv(
        args.out_dir / "capacity_impact_by_family.csv",
        capacity_impact_by_family(capacity_rows),
    )
    write_csv(args.out_dir / "family_summary.csv", family_summary(rows))
    (args.out_dir / "report.md").write_text(render_report(rows), encoding="utf-8")
    print(args.out_dir)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("inputs", nargs="+", type=Path)
    parser.add_argument("--out-dir", type=Path, default=Path("/tmp/qp-kkt-error-audit-summary"))
    return parser.parse_args()


def read_rows(paths: list[Path]) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    for path in paths:
        with path.open() as handle:
            for line_number, line in enumerate(handle, 1):
                line = line.strip()
                if not line:
                    continue
                row = json.loads(line)
                row["_input_file"] = str(path)
                row["_line_number"] = line_number
                rows.append(row)
    return rows


def predicate_summary(rows: list[dict[str, Any]]) -> list[dict[str, Any]]:
    output = []
    for key, group in grouped(rows, "input_source", "family").items():
        input_source, family = key
        for policy, verdict_fn in PREDICATES.items():
            counts = truth_counts(group, verdict_fn)
            decided = (
                counts["true_true"]
                + counts["true_false"]
                + counts["false_true"]
                + counts["false_false"]
            )
            unsound = counts["true_false"] + counts["false_true"]
            output.append(
                {
                    "input_source": input_source,
                    "family": family,
                    "policy": policy,
                    "rows": len(group),
                    **counts,
                    "decided_count": decided,
                    "decided_fraction": fraction(decided, len(group)),
                    "unsound_count": unsound,
                    "classification": "falsified" if unsound else "not_falsified",
                }
            )
    return output


def q_bound_summary(rows: list[dict[str, Any]]) -> list[dict[str, Any]]:
    output = []
    q_rows = [row for row in rows if row.get("exact_q") is not None]
    for key, group in grouped(q_rows, "input_source", "family").items():
        input_source, family = key
        for bound_field, covers_field in Q_BOUNDS.items():
            applicable = [row for row in group if row.get(bound_field) is not None]
            failures = [row for row in applicable if row.get(covers_field) is False]
            output.append(
                {
                    "input_source": input_source,
                    "family": family,
                    "policy": bound_field,
                    "rows": len(group),
                    "applicable_rows": len(applicable),
                    "inapplicable_rows": len(group) - len(applicable),
                    "failure_rows": len(failures),
                    "max_q_abs_error": max_metric(group, lambda row: abs(row["f64_q"] - row["exact_q"])),
                    "max_bound": max_metric(applicable, lambda row, field=bound_field: row[field]),
                    "classification": "falsified" if failures else "not_falsified_on_applicable_rows",
                }
            )
    return output


def capacity_impact_summary(rows: list[dict[str, Any]]) -> list[dict[str, Any]]:
    output = []
    for key, group in grouped(rows, "input_source", "family", "source_id", "enumeration").items():
        input_source, family, source_id, enumeration = key
        exact_positive = [row for row in group if row.get("exact_positive") is True]
        exact_capacity = min_metric(exact_positive, "exact_action")
        for policy, verdict_fn in PREDICATES.items():
            bound_field = (
                "verified_inverse_beta_radius_q_bound"
                if policy == "verified_inverse_beta_radius_verdict"
                else "current_q_error_bound"
            )
            output.append(
                capacity_impact_row(
                    input_source,
                    family,
                    source_id,
                    enumeration,
                    policy,
                    bound_field,
                    group,
                    exact_capacity,
                    verdict_fn,
                )
            )
    return output


def capacity_impact_row(
    input_source: str,
    family: str,
    source_id: str,
    enumeration: str,
    policy: str,
    bound_field: str,
    rows: list[dict[str, Any]],
    exact_capacity: Any,
    verdict_fn: PredicateFn,
) -> dict[str, Any]:
    true_rows = [row for row in rows if verdict_fn(row) == "true"]
    false_rows = [row for row in rows if verdict_fn(row) == "false"]
    indet_rows = [row for row in rows if verdict_fn(row) == "indet"]
    none_rows = [row for row in rows if verdict_fn(row) not in {"true", "false", "indet"}]
    not_false_rows = true_rows + indet_rows + none_rows

    true_capacity = min_metric(true_rows, "f64_action")
    not_false_capacity = min_metric(not_false_rows, "f64_action")
    not_false_lower = min_action_lower(not_false_rows, bound_field)
    true_upper = min_action_upper(true_rows, bound_field)
    isolation = winner_isolation(true_rows, not_false_rows, bound_field)

    return {
        "input_source": input_source,
        "family": family,
        "source_id": source_id,
        "enumeration": enumeration,
        "policy": policy,
        "rows": len(rows),
        "true_rows": len(true_rows),
        "indet_rows": len(indet_rows),
        "none_rows": len(none_rows),
        "false_rows": len(false_rows),
        "exact_capacity_audited": exact_capacity,
        "true_nominal_capacity": true_capacity,
        "not_false_nominal_capacity": not_false_capacity,
        "not_false_changes_nominal_capacity": capacities_differ(true_capacity, not_false_capacity),
        "not_false_capacity_lower_bound": not_false_lower,
        "true_capacity_upper_bound": true_upper,
        "capacity_interval_width": capacity_gap(true_upper, not_false_lower),
        "best_witness_isolated": isolation,
    }


def capacity_impact_by_family(rows: list[dict[str, Any]]) -> list[dict[str, Any]]:
    output = []
    for key, group in grouped(rows, "input_source", "family", "policy").items():
        input_source, family, policy = key
        output.append(
            {
                "input_source": input_source,
                "family": family,
                "policy": policy,
                "cases": len(group),
                "winner_isolated_cases": sum(
                    1 for row in group if row["best_witness_isolated"] == "yes"
                ),
                "winner_not_isolated_cases": sum(
                    1 for row in group if row["best_witness_isolated"] == "no_gap"
                ),
                "missing_bound_blocked_cases": sum(
                    1 for row in group if row["best_witness_isolated"] == "no_missing_q_bounds"
                ),
                "not_false_changes_nominal_cases": sum(
                    1 for row in group if row["not_false_changes_nominal_capacity"] == "yes"
                ),
                "max_capacity_interval_width": max_metric(
                    group, lambda row: numeric_or_none(row["capacity_interval_width"])
                ),
            }
        )
    return output


def family_summary(rows: list[dict[str, Any]]) -> list[dict[str, Any]]:
    output = []
    for key, group in grouped(rows, "input_source", "family").items():
        input_source, family = key
        output.append(
            {
                "input_source": input_source,
                "family": family,
                "rows": len(group),
                "exact_positive_rows": sum(1 for row in group if row.get("exact_positive") is True),
                "current_false_positives": sum(
                    1
                    for row in group
                    if row.get("current_f64_verdict") == "true"
                    and row.get("exact_positive") is False
                ),
                "verified_inverse_false_positives": sum(
                    1
                    for row in group
                    if row.get("verified_inverse_beta_radius_verdict") == "true"
                    and row.get("exact_positive") is False
                ),
                "verified_inverse_missing_rows": sum(
                    1 for row in group if row.get("verified_inverse_beta_radius_verdict") is None
                ),
            }
        )
    return output


def truth_counts(rows: list[dict[str, Any]], verdict_fn: PredicateFn) -> dict[str, int]:
    counts = {
        "true_true": 0,
        "true_false": 0,
        "false_true": 0,
        "false_false": 0,
        "indet_true": 0,
        "indet_false": 0,
        "none_true": 0,
        "none_false": 0,
    }
    for row in rows:
        verdict = verdict_fn(row)
        if verdict not in {"true", "false", "indet"}:
            verdict = "none"
        truth = "true" if row.get("exact_positive") is True else "false"
        counts[f"{verdict}_{truth}"] += 1
    return counts


def winner_isolation(rows_true: list[dict[str, Any]], rows_possible: list[dict[str, Any]], bound_field: str) -> str:
    if not rows_true:
        return "no_true_witness"
    best_true = min(rows_true, key=lambda row: row["f64_action"])
    best_upper = action_upper(best_true, bound_field)
    competitor_rows = [row for row in rows_possible if row is not best_true]
    competitor_lowers = [action_lower(row, bound_field) for row in competitor_rows]
    if best_upper is None or any(value is None for value in competitor_lowers):
        return "no_missing_q_bounds"
    return "yes" if all(best_upper <= value for value in competitor_lowers) else "no_gap"


def min_action_lower(rows: list[dict[str, Any]], bound_field: str) -> float | str:
    values = [action_lower(row, bound_field) for row in rows]
    values = [value for value in values if value is not None]
    return min(values) if values else ""


def min_action_upper(rows: list[dict[str, Any]], bound_field: str) -> float | str:
    values = [action_upper(row, bound_field) for row in rows]
    values = [value for value in values if value is not None]
    return min(values) if values else ""


def action_lower(row: dict[str, Any], bound_field: str) -> float | None:
    q = row.get("f64_q")
    bound = row.get(bound_field)
    if q is None or bound is None:
        return None
    return 0.5 / (q + bound)


def action_upper(row: dict[str, Any], bound_field: str) -> float | None:
    q = row.get("f64_q")
    bound = row.get(bound_field)
    if q is None or bound is None:
        return None
    q_lower = q - bound
    if q_lower <= 1e-15:
        return math.inf
    return 0.5 / q_lower


def capacities_differ(left: Any, right: Any) -> str:
    if not isinstance(left, (int, float)) or not isinstance(right, (int, float)):
        return "unknown"
    return "yes" if abs(left - right) > 1e-12 else "no"


def capacity_gap(upper: Any, lower: Any) -> float | str:
    if not isinstance(upper, (int, float)) or not isinstance(lower, (int, float)):
        return ""
    return upper - lower


def grouped(rows: list[dict[str, Any]], *keys: str) -> dict[tuple[Any, ...], list[dict[str, Any]]]:
    groups: dict[tuple[Any, ...], list[dict[str, Any]]] = defaultdict(list)
    for row in rows:
        groups[tuple(row.get(key, "") for key in keys)].append(row)
    return dict(sorted(groups.items()))


def min_metric(rows: list[dict[str, Any]], key: str) -> float | str:
    values = [row.get(key) for row in rows if isinstance(row.get(key), (int, float))]
    return min(values) if values else ""


def max_metric(rows: list[dict[str, Any]], fn: Callable[[dict[str, Any]], Any]) -> float | str:
    values = [numeric_or_none(fn(row)) for row in rows]
    values = [value for value in values if value is not None]
    return max(values) if values else ""


def numeric_or_none(value: Any) -> float | None:
    return value if isinstance(value, (int, float)) and math.isfinite(value) else None


def fraction(numerator: int, denominator: int) -> float | str:
    return numerator / denominator if denominator else ""


def write_csv(path: Path, rows: list[dict[str, Any]]) -> None:
    if not rows:
        path.write_text("", encoding="utf-8")
        return
    with path.open("w", newline="") as handle:
        writer = csv.DictWriter(handle, fieldnames=list(rows[0].keys()))
        writer.writeheader()
        writer.writerows(rows)


def render_report(rows: list[dict[str, Any]]) -> str:
    lines = [
        "# QP KKT Error Audit Summary",
        "",
        f"- rows: {len(rows)}",
        f"- current f64 false positives: {sum(1 for row in rows if row.get('current_f64_verdict') == 'true' and row.get('exact_positive') is False)}",
        f"- verified-inverse false positives: {sum(1 for row in rows if row.get('verified_inverse_beta_radius_verdict') == 'true' and row.get('exact_positive') is False)}",
        "",
        "## Predicate Summary",
        "",
        "| input_source | family | policy | rows | true/true | true/false | false/true | false/false | indet/true | indet/false | none/true | none/false |",
        "| --- | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |",
    ]
    for row in predicate_summary(rows):
        lines.append(
            "| {input_source} | {family} | {policy} | {rows} | {true_true} | {true_false} | {false_true} | {false_false} | {indet_true} | {indet_false} | {none_true} | {none_false} |".format(
                **row
            )
        )
    lines.extend(
        [
            "",
            "## Capacity Impact",
            "",
            "| input_source | family | policy | cases | winner isolated | missing-bound blocked | not-false changes nominal | max interval width |",
            "| --- | --- | --- | ---: | ---: | ---: | ---: | ---: |",
        ]
    )
    for row in capacity_impact_by_family(capacity_impact_summary(rows)):
        lines.append(
            "| {input_source} | {family} | {policy} | {cases} | {winner_isolated_cases} | {missing_bound_blocked_cases} | {not_false_changes_nominal_cases} | {max_capacity_interval_width} |".format(
                **row
            )
        )
    lines.append("")
    return "\n".join(lines)


if __name__ == "__main__":
    main()
