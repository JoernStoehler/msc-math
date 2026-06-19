#!/usr/bin/env python3
"""Compare f64 capacity scan rows against a handwritten verification manifest."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any


EXPERIMENT_DIR = Path(__file__).resolve().parent
MISSING = object()


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Compare f64 capacity scan JSONL rows with manifest expectations."
    )
    parser.add_argument(
        "--manifest",
        type=Path,
        default=EXPERIMENT_DIR / "manifest.json",
        help="Expectation manifest.",
    )
    parser.add_argument(
        "--scan",
        action="append",
        default=[],
        type=Path,
        help="Scan JSONL path. Repeatable.",
    )
    parser.add_argument(
        "--out-dir",
        type=Path,
        default=Path("/tmp/f64-capacity-verification"),
        help="Directory for comparison rows.",
    )
    args = parser.parse_args()

    manifest = json.loads(args.manifest.read_text(encoding="utf-8"))
    scan_rows = load_scan_rows(args.scan)
    comparison_rows = compare_cases(manifest["cases"], scan_rows)
    rows_path = write_outputs(args.out_dir, comparison_rows)
    met = sum(1 for row in comparison_rows if row["expectation_status"] == "met")
    failed = sum(1 for row in comparison_rows if row["expectation_status"] == "failed")
    print(f"wrote {rows_path}; expectations met={met} failed={failed}")
    return 1 if any(row["expectation_status"] == "failed" for row in comparison_rows) else 0


def load_scan_rows(paths: list[Path]) -> dict[str, dict[str, Any]]:
    rows: dict[str, dict[str, Any]] = {}
    for path in paths:
        with path.open(encoding="utf-8") as handle:
            for line_number, line in enumerate(handle, start=1):
                stripped = line.strip()
                if not stripped:
                    continue
                row = json.loads(stripped)
                source_id = row["source_id"]
                if source_id in rows:
                    raise ValueError(f"duplicate source_id {source_id} in {path}:{line_number}")
                rows[source_id] = row
    return rows


def compare_cases(
    cases: list[dict[str, Any]], scan_rows: dict[str, dict[str, Any]]
) -> list[dict[str, Any]]:
    rows = []
    expected_source_ids = {case["source_id"] for case in cases}
    for case in cases:
        observed = scan_rows.get(case["source_id"])
        failures = []
        if case["claim_scope"] not in {"full", "limited"}:
            failures.append(f"invalid claim_scope {case['claim_scope']!r}; expected full or limited")
        if observed is None:
            failures.append(f"missing scan row for source_id {case['source_id']}")
        else:
            failures.extend(compare_expectations(case["expect"], observed))

        expectation_status = "failed" if failures else "met"
        rows.append(
            {
                "case_id": case["case_id"],
                "source_id": case["source_id"],
                "reason": case["reason"],
                "claim_scope": case["claim_scope"],
                "expectation_status": expectation_status,
                "failures": failures,
                "observed": observed_summary(observed),
            }
        )
    for source_id in sorted(set(scan_rows) - expected_source_ids):
        observed = scan_rows[source_id]
        rows.append(
            {
                "case_id": f"unexpected:{source_id}",
                "source_id": source_id,
                "reason": "scan row was not listed in the manifest",
                "claim_scope": "unexpected",
                "expectation_status": "failed",
                "failures": ["unexpected scan row"],
                "observed": observed_summary(observed),
            }
        )
    return rows


def compare_expectations(expectations: dict[str, Any], observed: dict[str, Any]) -> list[str]:
    failures = []
    for field, expected in expectations.items():
        actual = value_at_path(observed, field)
        if actual is MISSING:
            failures.append(f"{field}: missing field")
            continue
        failures.extend(compare_value(field, actual, expected))
    return failures


def compare_value(field: str, actual: Any, expected: Any) -> list[str]:
    if isinstance(expected, dict):
        allowed_keys = {"equals", "one_of", "contains", "min", "max"}
        unknown_keys = sorted(set(expected) - allowed_keys)
        if unknown_keys:
            return [f"{field}: unknown expectation operator(s) {unknown_keys!r}"]
        failures = []
        if "equals" in expected and actual != expected["equals"]:
            failures.append(f"{field}: expected {expected['equals']!r}, got {actual!r}")
        if "one_of" in expected and actual not in expected["one_of"]:
            failures.append(f"{field}: expected one of {expected['one_of']!r}, got {actual!r}")
        if "contains" in expected:
            if not isinstance(actual, list) or expected["contains"] not in actual:
                failures.append(
                    f"{field}: expected list containing {expected['contains']!r}, got {actual!r}"
                )
        if "min" in expected and (actual is None or float(actual) < float(expected["min"])):
            failures.append(f"{field}: expected >= {expected['min']!r}, got {actual!r}")
        if "max" in expected:
            if actual is None:
                failures.append(f"{field}: expected <= {expected['max']!r}, got None")
            elif float(actual) > float(expected["max"]):
                failures.append(f"{field}: expected <= {expected['max']!r}, got {actual!r}")
        return failures
    if actual != expected:
        return [f"{field}: expected {expected!r}, got {actual!r}"]
    return []


def value_at_path(row: dict[str, Any], path: str) -> Any:
    value: Any = row
    for part in path.split("."):
        if not isinstance(value, dict) or part not in value:
            return MISSING
        value = value[part]
    return value


def observed_summary(row: dict[str, Any] | None) -> dict[str, Any]:
    if row is None:
        return {}
    epistemics = row.get("output_epistemics", {})
    return {
        "family": row.get("family"),
        "validation_status": row.get("validation_status"),
        "outcome": row.get("outcome"),
        "f64_capacity": row.get("f64_capacity"),
        "audit_capacity_label": row.get("audit_capacity_label"),
        "rel_action_error": row.get("rel_action_error"),
        "f64_sigma": row.get("f64_sigma"),
        "sigma_count": row.get("sigma_count"),
        "near_minimizing_sigma_count": row.get("near_minimizing_sigma_count"),
        "min_action_gap": row.get("min_action_gap"),
        "capacity_value_status": epistemics.get("capacity_value_status"),
        "capacity_label_status": epistemics.get("capacity_label_status"),
        "minimizing_sigma_set_status": epistemics.get("minimizing_sigma_set_status"),
        "low_action_list_completeness": epistemics.get("low_action_list_completeness"),
        "low_action_items_status": epistemics.get("low_action_items_status"),
        "fallback_recommended": epistemics.get("fallback_recommended"),
        "epistemic_reasons": epistemics.get("reasons", []),
        "trust_class": row.get("trust_class"),
        "exact_audit_status": row.get("exact_audit_status"),
        "support_kind": support_kind(row),
    }


def support_kind(row: dict[str, Any]) -> str:
    if row.get("exact_audit_status") == "exact_valid_capacity_success":
        return "exact_audit"
    if row.get("audit_capacity_label") is not None:
        return "stored_label"
    return "no_label"


def write_outputs(out_dir: Path, comparison_rows: list[dict[str, Any]]) -> Path:
    out_dir.mkdir(parents=True, exist_ok=True)
    rows_path = out_dir / "comparison.jsonl"
    with rows_path.open("w", encoding="utf-8") as handle:
        for row in comparison_rows:
            handle.write(json.dumps(row, sort_keys=True) + "\n")
    return rows_path


if __name__ == "__main__":
    raise SystemExit(main())
