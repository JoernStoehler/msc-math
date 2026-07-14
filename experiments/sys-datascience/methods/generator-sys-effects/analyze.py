#!/usr/bin/env -S uv run --script
"""Summarize the complete one-row alternative-generator target pilot."""

from __future__ import annotations

import argparse
import hashlib
import json
import math
from collections import Counter, defaultdict
from pathlib import Path
from typing import Any

EXPECTED_SCHEMA = "alternative-generator-smoke-row-v2"
FACTORIAL = {
    "factorial-baseline": "baseline",
    "factorial-q": "q_tangential",
    "factorial-p": "p_tangential",
    "factorial-both": "both_tangential",
}
ANTIPODAL = {
    "broken-symmetric-control": "paired_support_control",
    "broken-antipodal": "independent_supports",
}


class AnalysisError(ValueError):
    pass


def load_rows(path: Path) -> tuple[list[dict[str, Any]], str]:
    payload = path.read_bytes()
    rows: list[dict[str, Any]] = []
    seen: set[str] = set()
    for line_number, line in enumerate(payload.splitlines(), 1):
        try:
            row = json.loads(line)
        except json.JSONDecodeError as exc:
            raise AnalysisError(f"line {line_number}: invalid JSON") from exc
        if row.get("schema") != EXPECTED_SCHEMA:
            raise AnalysisError(f"line {line_number}: unexpected schema")
        sample_id = row.get("sample_id")
        if not isinstance(sample_id, str) or sample_id in seen:
            raise AnalysisError(f"line {line_number}: missing or duplicate sample_id")
        seen.add(sample_id)
        value = row.get("sys")
        if value is not None and (not isinstance(value, (int, float)) or not math.isfinite(value)):
            raise AnalysisError(f"line {line_number}: non-finite sys")
        rows.append(row)
    if not rows:
        raise AnalysisError("empty input")
    return rows, hashlib.sha256(payload).hexdigest()


def paired_effects(rows: list[dict[str, Any]], law_map: dict[str, str]) -> list[dict[str, Any]]:
    groups: dict[str, dict[str, float]] = defaultdict(dict)
    buckets: dict[str, str] = {}
    for row in rows:
        label = law_map.get(row["law"])
        pairing_id = row.get("pairing_id")
        if label is None or pairing_id is None or row.get("sys") is None:
            continue
        groups[pairing_id][label] = float(row["sys"])
        buckets[pairing_id] = row["pair_bucket"]
    output = []
    for pairing_id, values in sorted(groups.items()):
        record: dict[str, Any] = {
            "pairing_id": pairing_id,
            "pair_bucket": buckets[pairing_id],
            "values": values,
            "complete": set(values) == set(law_map.values()),
        }
        if set(values) == set(FACTORIAL.values()):
            b = values["baseline"]
            q = values["q_tangential"]
            p = values["p_tangential"]
            both = values["both_tangential"]
            record["contrasts"] = {
                "q_minus_baseline": q - b,
                "p_minus_baseline": p - b,
                "both_minus_baseline": both - b,
                "factorial_interaction": both - q - p + b,
            }
        elif set(values) == set(ANTIPODAL.values()):
            record["contrasts"] = {
                "independent_minus_paired": values["independent_supports"]
                - values["paired_support_control"]
            }
        output.append(record)
    return output


def analyze(rows: list[dict[str, Any]], input_sha256: str) -> tuple[dict[str, Any], list[dict[str, Any]]]:
    evaluated = [row for row in rows if row.get("sys") is not None]
    by_arm: dict[tuple[str, str, str], list[dict[str, Any]]] = defaultdict(list)
    for row in rows:
        by_arm[(row["law"], row["parameter"], row["pair_bucket"])].append(row)
    arms = []
    for (law, parameter, bucket), group in sorted(by_arm.items()):
        values = [float(row["sys"]) for row in group if row.get("sys") is not None]
        arms.append(
            {
                "law": law,
                "parameter": parameter,
                "pair_bucket": bucket,
                "rows": len(group),
                "evaluated": len(values),
                "mean_sys": sum(values) / len(values) if values else None,
                "min_sys": min(values) if values else None,
                "max_sys": max(values) if values else None,
                "mean_target_ms": (
                    sum(float(row["target_ms"]) for row in group if row.get("sys") is not None)
                    / len(values)
                    if values
                    else None
                ),
            }
        )
    witnesses = sorted(
        (
            {
                "law": row["law"],
                "parameter": row["parameter"],
                "pair_bucket": row["pair_bucket"],
                "sample_id": row["sample_id"],
                "sys": row["sys"],
            }
            for row in evaluated
        ),
        key=lambda record: (record["pair_bucket"], -record["sys"], record["law"], record["parameter"]),
    )
    factorial = paired_effects(rows, FACTORIAL)
    antipodal = paired_effects(rows, ANTIPODAL)
    factorial_3x3 = [
        effect for effect in factorial if effect["pair_bucket"] == "3x3" and effect["complete"]
    ]
    negative_control_spread = None
    if factorial_3x3:
        values = list(factorial_3x3[0]["values"].values())
        negative_control_spread = max(values) - min(values)
    report = {
        "schema": "generator-sys-effects-report-v1",
        "input_sha256": input_sha256,
        "row_count": len(rows),
        "unique_sample_ids": len({row["sample_id"] for row in rows}),
        "accepted_rows": sum(bool(row.get("accepted")) for row in rows),
        "evaluated_sys_rows": len(evaluated),
        "validation_status_counts": dict(sorted(Counter(row["validation_status"] for row in rows).items())),
        "target_ms": {
            "sum": sum(float(row["target_ms"]) for row in evaluated),
            "max": max(float(row["target_ms"]) for row in evaluated),
        },
        "arms": arms,
        "paired_factorial": factorial,
        "paired_antipodal": antipodal,
        "factorial_3x3_negative_control_max_minus_min": negative_control_spread,
        "interpretation_boundary": (
            "One deterministic row per arm/bucket: semantic and hypothesis-generating evidence only. "
            "It cannot estimate generator populations, transfer, tail rates, or stable rankings."
        ),
    }
    return report, witnesses


def write_outputs(out_dir: Path, report: dict[str, Any], witnesses: list[dict[str, Any]]) -> None:
    out_dir.mkdir(parents=True, exist_ok=True)
    (out_dir / "report.json").write_text(json.dumps(report, indent=2, sort_keys=True) + "\n")
    lines = ["pair_bucket\trank\tlaw\tparameter\tsys\tsample_id"]
    rank_by_bucket: Counter[str] = Counter()
    for row in witnesses:
        bucket = row["pair_bucket"]
        rank_by_bucket[bucket] += 1
        lines.append(
            f"{bucket}\t{rank_by_bucket[bucket]}\t{row['law']}\t{row['parameter']}\t{row['sys']:.17g}\t{row['sample_id']}"
        )
    (out_dir / "witnesses.tsv").write_text("\n".join(lines) + "\n")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--input", required=True, type=Path)
    parser.add_argument("--out-dir", required=True, type=Path)
    args = parser.parse_args()
    rows, digest = load_rows(args.input)
    report, witnesses = analyze(rows, digest)
    write_outputs(args.out_dir, report, witnesses)
    print(f"wrote {len(witnesses)} evaluated witnesses from {len(rows)} rows")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
