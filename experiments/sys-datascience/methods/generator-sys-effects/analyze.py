#!/usr/bin/env -S uv run --script
"""Summarize the complete one-row alternative-generator target pilot."""

from __future__ import annotations

import argparse
import hashlib
import json
import math
import shlex
from collections import Counter, defaultdict
from pathlib import Path
from typing import Any

EXPECTED_SCHEMA = "alternative-generator-smoke-row-v2"
EXPECTED_MANIFEST_SCHEMA = "alternative-generator-smoke-report-v2"
EXPECTED_BINARY = "sys-datascience-alternative-generator-smoke"
EXPECTED_STATUSES = {"invalid_or_low_acceptance", "runtime_cap", "survived"}
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


def load_manifest(path: Path) -> tuple[dict[str, Any], str]:
    payload = path.read_bytes()
    try:
        manifest = json.loads(payload)
    except json.JSONDecodeError as exc:
        raise AnalysisError("manifest: invalid JSON") from exc
    if not isinstance(manifest, dict):
        raise AnalysisError("manifest: expected a JSON object")
    return manifest, hashlib.sha256(payload).hexdigest()


def _require_int(value: Any, label: str, *, minimum: int = 0) -> int:
    if isinstance(value, bool) or not isinstance(value, int) or value < minimum:
        raise AnalysisError(f"manifest: {label} must be an integer >= {minimum}")
    return value


def _command_options(command: Any) -> dict[str, str | bool]:
    if not isinstance(command, str):
        raise AnalysisError("manifest: command must be a string")
    try:
        tokens = shlex.split(command)
    except ValueError as exc:
        raise AnalysisError("manifest: command is not valid shell syntax") from exc
    if not tokens or Path(tokens[0]).name != EXPECTED_BINARY:
        raise AnalysisError(f"manifest: command must invoke {EXPECTED_BINARY}")

    value_options = {"--out-dir", "--seed", "--attempts", "--runtime-cap-ms", "--rows-per-law"}
    flag_options = {"--target"}
    options: dict[str, str | bool] = {}
    index = 1
    while index < len(tokens):
        option = tokens[index]
        if option in options:
            raise AnalysisError(f"manifest: duplicate command option {option}")
        if option in flag_options:
            options[option] = True
            index += 1
        elif option in value_options:
            if index + 1 >= len(tokens) or tokens[index + 1].startswith("--"):
                raise AnalysisError(f"manifest: command option {option} needs a value")
            options[option] = tokens[index + 1]
            index += 2
        else:
            raise AnalysisError(f"manifest: unexpected command token {option}")
    expected_options = value_options | flag_options
    if set(options) != expected_options:
        missing = ", ".join(sorted(expected_options - set(options)))
        raise AnalysisError(f"manifest: command is missing {missing}")
    return options


def validate_manifest(rows: list[dict[str, Any]], manifest: dict[str, Any]) -> None:
    if manifest.get("schema") != EXPECTED_MANIFEST_SCHEMA:
        raise AnalysisError("manifest: unexpected schema")
    source_revision = manifest.get("source_revision")
    if (
        not isinstance(source_revision, str)
        or len(source_revision) != 40
        or any(character not in "0123456789abcdef" for character in source_revision)
    ):
        raise AnalysisError("manifest: source_revision must be a full lowercase Git object ID")
    if _require_int(manifest.get("rows"), "rows") != len(rows):
        raise AnalysisError(
            f"manifest: rows={manifest.get('rows')} does not match JSONL length {len(rows)}"
        )

    row_statuses = [row.get("validation_status") for row in rows]
    if any(status not in EXPECTED_STATUSES for status in row_statuses):
        raise AnalysisError("manifest: JSONL row has an unexpected validation_status")
    actual_statuses = dict(sorted(Counter(row_statuses).items()))
    if manifest.get("status_counts") != actual_statuses:
        raise AnalysisError("manifest: status_counts do not match JSONL rows")

    seed = _require_int(manifest.get("seed"), "seed")
    if {row.get("seed") for row in rows} != {seed}:
        raise AnalysisError("manifest: seed is not consistent across JSONL rows")
    law_version = manifest.get("law_version")
    if not isinstance(law_version, str) or not law_version:
        raise AnalysisError("manifest: law_version must be a nonempty string")
    if {row.get("law_version") for row in rows} != {law_version}:
        raise AnalysisError("manifest: law_version is not consistent across JSONL rows")

    max_attempts = _require_int(
        manifest.get("max_attempts_per_row"), "max_attempts_per_row", minimum=1
    )
    for row in rows:
        attempts = row.get("attempts")
        attempt = row.get("attempt")
        if (
            isinstance(attempts, bool)
            or not isinstance(attempts, int)
            or not 1 <= attempts <= max_attempts
            or isinstance(attempt, bool)
            or not isinstance(attempt, int)
            or attempt != attempts - 1
        ):
            raise AnalysisError(f"manifest: invalid attempt accounting for {row.get('sample_id')}")

    pairs = manifest.get("pairs")
    row_pairs = [row.get("pair_bucket") for row in rows]
    if any(not isinstance(pair, str) or not pair for pair in row_pairs):
        raise AnalysisError("manifest: every JSONL row must name a pair_bucket")
    actual_pairs = sorted(set(row_pairs))
    if not isinstance(pairs, list) or pairs != actual_pairs:
        raise AnalysisError("manifest: pairs do not match the JSONL pair-bucket grid")

    command_options = _command_options(manifest.get("command"))
    try:
        command_seed = int(str(command_options["--seed"]))
        command_attempts = int(str(command_options["--attempts"]))
        command_runtime_cap = float(str(command_options["--runtime-cap-ms"]))
        rows_per_law = int(str(command_options["--rows-per-law"]))
    except ValueError as exc:
        raise AnalysisError("manifest: command has a nonnumeric numeric option") from exc
    if command_seed != seed or command_attempts != max_attempts:
        raise AnalysisError("manifest: command seed/attempts disagree with manifest fields")
    if rows_per_law != 1 or command_options.get("--target") is not True:
        raise AnalysisError("manifest: command must include --rows-per-law 1 --target")
    if not isinstance(command_options.get("--out-dir"), str):
        raise AnalysisError("manifest: command must name an output directory")

    runtime_cap = manifest.get("runtime_cap_ms")
    if (
        isinstance(runtime_cap, bool)
        or not isinstance(runtime_cap, (int, float))
        or not math.isfinite(runtime_cap)
        or runtime_cap <= 0
    ):
        raise AnalysisError("manifest: runtime_cap_ms must be finite and positive")
    runtime_cap = float(runtime_cap)
    if not math.isfinite(command_runtime_cap) or command_runtime_cap != runtime_cap:
        raise AnalysisError("manifest: command runtime cap disagrees with manifest field")

    actual_arms: dict[tuple[str, str], list[dict[str, Any]]] = defaultdict(list)
    seen_grid: set[tuple[str, str, int, str]] = set()
    for row in rows:
        law = row.get("law")
        parameter = row.get("parameter")
        row_index = row.get("row_index")
        bucket = row.get("pair_bucket")
        if not isinstance(law, str) or not isinstance(parameter, str):
            raise AnalysisError("manifest: every row must name a law and parameter")
        if row_index != 0:
            raise AnalysisError("manifest: --rows-per-law 1 requires row_index 0")
        grid_key = (law, parameter, row_index, bucket)
        if grid_key in seen_grid:
            raise AnalysisError("manifest: duplicate law/parameter/row/bucket grid cell")
        seen_grid.add(grid_key)
        actual_arms[(law, parameter)].append(row)

        status = row.get("validation_status")
        target_ms = row.get("target_ms")
        facet_count = row.get("facet_count")
        if not isinstance(target_ms, (int, float)) or not math.isfinite(target_ms) or target_ms < 0:
            raise AnalysisError(f"manifest: invalid target_ms for {row.get('sample_id')}")
        censored = isinstance(facet_count, int) and facet_count > 10
        over_runtime = target_ms > runtime_cap
        if censored and status != "runtime_cap":
            raise AnalysisError("manifest: a facet-count skip is not marked runtime_cap")
        if status == "runtime_cap" and not (censored or over_runtime):
            raise AnalysisError("manifest: runtime_cap row has no cap condition")
        if over_runtime and status != "runtime_cap":
            raise AnalysisError("manifest: over-cap target row is not marked runtime_cap")
        if status == "survived" and row.get("sys") is None:
            raise AnalysisError("manifest: survived target row has no sys")

    manifest_arms = manifest.get("per_arm")
    if not isinstance(manifest_arms, list):
        raise AnalysisError("manifest: per_arm must be a list")
    seen_arms: set[tuple[str, str]] = set()
    for arm in manifest_arms:
        if not isinstance(arm, dict):
            raise AnalysisError("manifest: per_arm entry must be an object")
        key = (arm.get("law"), arm.get("parameter"))
        if key in seen_arms:
            raise AnalysisError("manifest: duplicate per_arm law/parameter")
        seen_arms.add(key)
        group = actual_arms.get(key)
        if group is None:
            raise AnalysisError("manifest: per_arm contains an absent law/parameter")
        expected_counts = {
            "rows": len(group),
            "accepted_rows": sum(row.get("accepted") is True for row in group),
            "survived_rows": sum(row.get("validation_status") == "survived" for row in group),
            "max_attempts_observed": max(int(row["attempts"]) for row in group),
            "factor_metric_count": sum(
                all(
                    row.get(f"factor_{factor}_{metric}") is not None
                    for metric in ("support_cv", "gap_cv", "isoperimetric_ratio")
                )
                for row in group
                for factor in ("q", "p")
            ),
        }
        for field, expected in expected_counts.items():
            if arm.get(field) != expected:
                raise AnalysisError(f"manifest: per_arm {key} has incorrect {field}")
    if seen_arms != set(actual_arms):
        raise AnalysisError("manifest: per_arm grid does not match JSONL law/parameter grid")


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


def analyze(
    rows: list[dict[str, Any]],
    input_sha256: str,
    manifest: dict[str, Any],
    manifest_sha256: str,
) -> tuple[dict[str, Any], list[dict[str, Any]]]:
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
        "manifest_sha256": manifest_sha256,
        "source_revision": manifest.get("source_revision"),
        "source_law_version": manifest["law_version"],
        "source_seed": manifest["seed"],
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
    lines = ["pair_bucket\tdisplay_order\tlaw\tparameter\tsys\tsample_id"]
    order_by_bucket: Counter[str] = Counter()
    for row in witnesses:
        bucket = row["pair_bucket"]
        order_by_bucket[bucket] += 1
        lines.append(
            f"{bucket}\t{order_by_bucket[bucket]}\t{row['law']}\t{row['parameter']}\t{row['sys']:.17g}\t{row['sample_id']}"
        )
    (out_dir / "witnesses.tsv").write_text("\n".join(lines) + "\n")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--input", required=True, type=Path)
    parser.add_argument("--manifest", required=True, type=Path)
    parser.add_argument("--out-dir", required=True, type=Path)
    args = parser.parse_args()
    rows, digest = load_rows(args.input)
    manifest, manifest_digest = load_manifest(args.manifest)
    validate_manifest(rows, manifest)
    report, witnesses = analyze(rows, digest, manifest, manifest_digest)
    write_outputs(args.out_dir, report, witnesses)
    print(f"wrote {len(witnesses)} evaluated witnesses from {len(rows)} rows")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
