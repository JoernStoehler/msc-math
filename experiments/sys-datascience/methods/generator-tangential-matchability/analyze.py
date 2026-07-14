#!/usr/bin/env python3
"""Audit a target-free paired factorial generator panel."""

from __future__ import annotations

import argparse
import hashlib
import json
import math
import shlex
from collections import Counter, defaultdict
from pathlib import Path
from typing import Any

ROW_SCHEMA = "alternative-generator-smoke-row-v2"
MANIFEST_SCHEMA = "alternative-generator-smoke-report-v2"
REPORT_SCHEMA = "generator-tangential-matchability-report-v2"
LAW_VERSION = "wishlist-2026-07-14-v2"
ARMS = {
    "factorial-baseline": "current",
    "factorial-q": "q=tangential",
    "factorial-p": "p=tangential",
    "factorial-both": "q,p=tangential",
}
BUCKETS = ("3x3", "4x6", "6x6")
FACTOR_METRICS = ("support_cv", "gap_cv", "isoperimetric_ratio")
AGREEMENT_TOL = 1.0e-9


class AnalysisError(ValueError):
    pass


def load_rows(path: Path) -> tuple[list[dict[str, Any]], str]:
    payload = path.read_bytes()
    if not payload.endswith(b"\n"):
        raise AnalysisError("JSONL input has no final newline; possible truncated producer output")
    rows = []
    seen = set()
    for line_number, line in enumerate(payload.splitlines(), 1):
        try:
            row = json.loads(line)
        except json.JSONDecodeError as exc:
            raise AnalysisError(f"line {line_number}: invalid JSON") from exc
        if not isinstance(row, dict) or row.get("schema") != ROW_SCHEMA:
            raise AnalysisError(f"line {line_number}: unexpected row schema")
        sample_id = row.get("sample_id")
        if not isinstance(sample_id, str) or sample_id in seen:
            raise AnalysisError(f"line {line_number}: missing or duplicate sample_id")
        seen.add(sample_id)
        rows.append(row)
    if not rows:
        raise AnalysisError("empty JSONL input")
    return rows, hashlib.sha256(payload).hexdigest()


def load_manifest(path: Path) -> tuple[dict[str, Any], str]:
    payload = path.read_bytes()
    try:
        manifest = json.loads(payload)
    except json.JSONDecodeError as exc:
        raise AnalysisError("manifest is invalid JSON") from exc
    if not isinstance(manifest, dict):
        raise AnalysisError("manifest must be a JSON object")
    return manifest, hashlib.sha256(payload).hexdigest()


def _finite_number(value: Any, label: str) -> float:
    if isinstance(value, bool) or not isinstance(value, (int, float)) or not math.isfinite(value):
        raise AnalysisError(f"{label} must be finite")
    return float(value)


def _parse_command(command: Any) -> dict[str, str]:
    if not isinstance(command, str):
        raise AnalysisError("manifest command must be a string")
    try:
        tokens = shlex.split(command)
    except ValueError as exc:
        raise AnalysisError("manifest command is not valid shell syntax") from exc
    if not tokens or Path(tokens[0]).name != "sys-datascience-alternative-generator-smoke":
        raise AnalysisError("manifest command names the wrong producer")
    if "--target" in tokens:
        raise AnalysisError("target-free packet cannot contain --target")
    options: dict[str, str] = {}
    index = 1
    while index < len(tokens):
        option = tokens[index]
        if not option.startswith("--") or index + 1 >= len(tokens):
            raise AnalysisError(f"malformed producer command near {option}")
        if option in options:
            raise AnalysisError(f"duplicate producer command option {option}")
        options[option] = tokens[index + 1]
        index += 2
    allowed = {
        "--out-dir",
        "--seed",
        "--attempts",
        "--runtime-cap-ms",
        "--rows-per-law",
        "--only-family",
        "--identity-scope",
    }
    if set(options) != allowed:
        raise AnalysisError("producer command must contain exactly the reviewed family-panel options")
    return options


def validate_packet(
    rows: list[dict[str, Any]],
    manifest: dict[str, Any],
    expected_rows_per_bucket: int,
    identity_scope: str,
) -> dict[str, Any]:
    if expected_rows_per_bucket <= 0:
        raise AnalysisError("expected rows per bucket must be positive")
    if manifest.get("schema") != MANIFEST_SCHEMA:
        raise AnalysisError("unexpected manifest schema")
    if manifest.get("law_version") != LAW_VERSION:
        raise AnalysisError("unexpected manifest law_version")
    if manifest.get("identity_scope") != identity_scope:
        raise AnalysisError("manifest identity_scope mismatch")
    if manifest.get("pairs") != list(BUCKETS):
        raise AnalysisError("manifest pair buckets do not match the reviewed panel")
    if manifest.get("rows") != len(rows):
        raise AnalysisError("manifest row count does not match JSONL")
    source_revision = manifest.get("source_revision")
    if (
        not isinstance(source_revision, str)
        or len(source_revision) != 40
        or any(character not in "0123456789abcdef" for character in source_revision)
    ):
        raise AnalysisError("manifest source_revision is not a full lowercase Git object ID")

    options = _parse_command(manifest.get("command"))
    if options["--only-family"] != "factorial":
        raise AnalysisError("producer command did not use the reviewed factorial family filter")
    if options["--identity-scope"] != identity_scope:
        raise AnalysisError("producer command identity scope mismatch")
    try:
        command_rows = int(options["--rows-per-law"])
        command_seed = int(options["--seed"])
        command_attempts = int(options["--attempts"])
    except ValueError as exc:
        raise AnalysisError("producer command has a noninteger seed/attempt/row option") from exc
    if command_rows != expected_rows_per_bucket:
        raise AnalysisError("producer command rows-per-law does not match requested panel")
    if command_seed != manifest.get("seed"):
        raise AnalysisError("producer command seed disagrees with manifest")
    if command_attempts != manifest.get("max_attempts_per_row") or command_attempts <= 0:
        raise AnalysisError("producer command attempt cap disagrees with manifest")

    expected_row_count = len(ARMS) * len(BUCKETS) * expected_rows_per_bucket
    if len(rows) != expected_row_count:
        raise AnalysisError(f"expected {expected_row_count} factorial rows, found {len(rows)}")
    statuses = dict(sorted(Counter(row.get("validation_status") for row in rows).items()))
    if manifest.get("status_counts") != statuses:
        raise AnalysisError("manifest status counts do not match JSONL")

    actual_cells: dict[tuple[str, str, int], dict[str, Any]] = {}
    groups: dict[tuple[str, int], list[dict[str, Any]]] = defaultdict(list)
    for row in rows:
        law = row.get("law")
        bucket = row.get("pair_bucket")
        row_index = row.get("row_index")
        if law not in ARMS or row.get("parameter") != ARMS.get(law):
            raise AnalysisError("JSONL contains a nonfactorial or wrong-parameter row")
        if bucket not in BUCKETS or not isinstance(row_index, int):
            raise AnalysisError("JSONL contains an unexpected bucket or row index")
        if not 0 <= row_index < expected_rows_per_bucket:
            raise AnalysisError("row index lies outside the requested panel")
        cell = (law, bucket, row_index)
        if cell in actual_cells:
            raise AnalysisError("duplicate arm/bucket/row-index cell")
        actual_cells[cell] = row
        groups[(bucket, row_index)].append(row)

        if row.get("identity_scope") != identity_scope:
            raise AnalysisError("row identity_scope mismatch")
        if row.get("law_version") != LAW_VERSION or row.get("seed") != command_seed:
            raise AnalysisError("row seed/law_version mismatch")
        attempt = row.get("attempt")
        attempts = row.get("attempts")
        if not isinstance(attempt, int) or not isinstance(attempts, int) or attempts != attempt + 1:
            raise AnalysisError("row attempt accounting is inconsistent")
        if not 1 <= attempts <= command_attempts:
            raise AnalysisError("row exceeds the bounded attempt cap")
        if row.get("rejections") != (attempt if row.get("accepted") else attempts):
            raise AnalysisError("row rejection accounting is inconsistent")
        if row.get("capacity") is not None or row.get("sys") is not None:
            raise AnalysisError("target-free packet contains capacity or sys")
        if row.get("iterations") is not None or row.get("target_ms") != 0.0:
            raise AnalysisError("target-free packet contains target execution evidence")
        prefix = f"altgen-v2/scope={identity_scope}/"
        if row.get("accepted") is True:
            expected_sample_id = (
                f"{prefix}{law}/param={row['parameter']}/seed={command_seed}/"
                f"row={row_index}/attempt={attempt}/{bucket}"
            )
        else:
            expected_sample_id = (
                f"{prefix}{law}/param={row['parameter']}/seed={command_seed}/"
                f"row={row_index}/outcome=exhausted/{bucket}"
            )
        if row.get("sample_id") != expected_sample_id:
            raise AnalysisError("sample identity is incomplete or noncanonical")

        expected_pairing_id = (
            f"{prefix}factorial/seed={command_seed}/row={row_index}/attempt={attempt}/{bucket}"
        )
        if row.get("pairing_id") != expected_pairing_id:
            raise AnalysisError("pairing identity is incomplete or noncanonical")

        if row.get("accepted") is True:
            if row.get("validation_status") != "survived":
                raise AnalysisError("accepted geometry-only row is not marked survived")
            q_area = _finite_number(row.get("factor_q_area"), "factor_q_area")
            p_area = _finite_number(row.get("factor_p_area"), "factor_p_area")
            volume = _finite_number(row.get("volume"), "volume")
            if max(abs(q_area - 1.0), abs(p_area - 1.0), abs(volume - q_area * p_area)) > AGREEMENT_TOL:
                raise AnalysisError("factor-area/product-volume agreement exceeds tolerance")
            for factor in ("q", "p"):
                for metric in FACTOR_METRICS:
                    value = _finite_number(row.get(f"factor_{factor}_{metric}"), metric)
                    if value < -AGREEMENT_TOL:
                        raise AnalysisError("scale-free factor metric is negative")
                    if metric == "isoperimetric_ratio" and value > 1.0 + AGREEMENT_TOL:
                        raise AnalysisError("isoperimetric ratio exceeds one")
        elif row.get("validation_status") != "invalid_or_low_acceptance":
            raise AnalysisError("unaccepted terminal row has an unexpected status")

    expected_cells = {
        (law, bucket, row_index)
        for law in ARMS
        for bucket in BUCKETS
        for row_index in range(expected_rows_per_bucket)
    }
    if set(actual_cells) != expected_cells:
        raise AnalysisError("factorial arm/bucket/row grid is incomplete")

    for key, group in groups.items():
        if {row["law"] for row in group} != set(ARMS) or len(group) != len(ARMS):
            raise AnalysisError(f"paired factorial group {key} is incomplete")
        if len({row["pairing_id"] for row in group}) != 1:
            raise AnalysisError(f"paired factorial group {key} has split identities")
        if len({(row["attempt"], row["attempts"]) for row in group}) != 1:
            raise AnalysisError(f"paired factorial group {key} accepted different latent attempts")

    per_arm_entries = manifest.get("per_arm")
    if (
        not isinstance(per_arm_entries, list)
        or len(per_arm_entries) != len(ARMS)
        or any(not isinstance(entry, dict) for entry in per_arm_entries)
    ):
        raise AnalysisError("manifest per_arm must be exactly four objects")
    arm_keys = [(entry.get("law"), entry.get("parameter")) for entry in per_arm_entries]
    if len(set(arm_keys)) != len(arm_keys):
        raise AnalysisError("manifest per_arm arm keys must be unique")
    manifest_arms = dict(zip(arm_keys, per_arm_entries, strict=True))
    if set(manifest_arms) != {(law, parameter) for law, parameter in ARMS.items()}:
        raise AnalysisError("manifest per_arm grid is not exactly the factorial family")
    for law, parameter in ARMS.items():
        group = [row for row in rows if row["law"] == law]
        entry = manifest_arms[(law, parameter)]
        expected_counts = {
            "rows": len(group),
            "accepted_rows": sum(row["accepted"] is True for row in group),
            "survived_rows": sum(row["validation_status"] == "survived" for row in group),
            "max_attempts_observed": max(row["attempts"] for row in group),
            "factor_metric_count": 2 * sum(row["accepted"] is True for row in group),
        }
        if any(entry.get(field) != value for field, value in expected_counts.items()):
            raise AnalysisError(f"manifest per_arm counts disagree for {law}")

    return {"seed": command_seed, "attempt_cap": command_attempts, "groups": groups}


def _summary(values: list[float | int]) -> dict[str, float | int | None]:
    return {
        "n": len(values),
        "min": min(values) if values else None,
        "mean": sum(values) / len(values) if values else None,
        "max": max(values) if values else None,
    }


def _range_overlap(arm_values: dict[str, list[float]]) -> float | None:
    if not all(arm_values.values()):
        return None
    lower = max(min(values) for values in arm_values.values())
    upper = min(max(values) for values in arm_values.values())
    union_lower = min(min(values) for values in arm_values.values())
    union_upper = max(max(values) for values in arm_values.values())
    width = union_upper - union_lower
    if width <= 1.0e-15:
        return 1.0
    return max(0.0, upper - lower) / width


def analyze(
    rows: list[dict[str, Any]],
    manifest: dict[str, Any],
    input_sha256: str,
    manifest_sha256: str,
    expected_rows_per_bucket: int,
    identity_scope: str,
) -> dict[str, Any]:
    accepted = [row for row in rows if row["accepted"] is True]
    structural_groups: dict[tuple[str, int], list[dict[str, Any]]] = defaultdict(list)
    for row in rows:
        structural_groups[(row["pair_bucket"], row["row_index"])].append(row)
    structurally_complete_pair_count = len(structural_groups)
    accepted_complete_pair_count = sum(
        all(row["accepted"] is True for row in group) for group in structural_groups.values()
    )
    pairs_with_rejected_rows = structurally_complete_pair_count - accepted_complete_pair_count
    pair_acceptance_by_bucket = []
    for bucket in BUCKETS:
        bucket_groups = [
            group for (group_bucket, _), group in structural_groups.items() if group_bucket == bucket
        ]
        accepted_complete = sum(
            all(row["accepted"] is True for row in group) for group in bucket_groups
        )
        pair_acceptance_by_bucket.append(
            {
                "pair_bucket": bucket,
                "structurally_complete_pair_count": len(bucket_groups),
                "accepted_complete_pair_count": accepted_complete,
                "pairs_with_rejected_rows": len(bucket_groups) - accepted_complete,
            }
        )
    by_bucket_arm = []
    for bucket in BUCKETS:
        for law in ARMS:
            group = [row for row in rows if row["pair_bucket"] == bucket and row["law"] == law]
            by_bucket_arm.append(
                {
                    "pair_bucket": bucket,
                    "law": law,
                    "rows": len(group),
                    "accepted_rows": sum(row["accepted"] is True for row in group),
                    "attempts": _summary([row["attempts"] for row in group]),
                    "generation_ms": _summary([float(row["generation_ms"]) for row in group]),
                    "validation_ms": _summary([float(row["validation_ms"]) for row in group]),
                    "factor_geometry": {
                        factor: {
                            metric: _summary(
                                [
                                    float(row[f"factor_{factor}_{metric}"])
                                    for row in group
                                    if row["accepted"] is True
                                ]
                            )
                            for metric in FACTOR_METRICS
                        }
                        for factor in ("q", "p")
                    },
                }
            )

    overlaps = []
    for bucket in BUCKETS:
        for factor in ("q", "p"):
            for metric in FACTOR_METRICS:
                arm_values = {
                    law: [
                        float(row[f"factor_{factor}_{metric}"])
                        for row in accepted
                        if row["pair_bucket"] == bucket and row["law"] == law
                    ]
                    for law in ARMS
                }
                overlaps.append(
                    {
                        "pair_bucket": bucket,
                        "factor": factor,
                        "metric": metric,
                        "all_arm_range_intersection_over_union": _range_overlap(arm_values),
                    }
                )

    equality_pairs = {
        "q": (("factorial-baseline", "factorial-p"), ("factorial-q", "factorial-both")),
        "p": (("factorial-baseline", "factorial-q"), ("factorial-p", "factorial-both")),
    }
    paired_deltas = []
    indexed = {(row["pair_bucket"], row["row_index"], row["law"]): row for row in accepted}
    for bucket in BUCKETS:
        for factor, pairs in equality_pairs.items():
            for metric in FACTOR_METRICS:
                for left, right in pairs:
                    deltas = []
                    for row_index in range(expected_rows_per_bucket):
                        left_row = indexed.get((bucket, row_index, left))
                        right_row = indexed.get((bucket, row_index, right))
                        if left_row is not None and right_row is not None:
                            deltas.append(
                                float(right_row[f"factor_{factor}_{metric}"])
                                - float(left_row[f"factor_{factor}_{metric}"])
                            )
                    paired_deltas.append(
                        {
                            "pair_bucket": bucket,
                            "factor": factor,
                            "metric": metric,
                            "left": left,
                            "right": right,
                            "n": len(deltas),
                            "max_abs_delta": max(map(abs, deltas)) if deltas else None,
                        }
                    )

    area_errors = []
    volume_errors = []
    for row in accepted:
        q_area = float(row["factor_q_area"])
        p_area = float(row["factor_p_area"])
        volume = float(row["volume"])
        area_errors.extend((abs(q_area - 1.0), abs(p_area - 1.0)))
        volume_errors.append(abs(volume - q_area * p_area))

    return {
        "schema": REPORT_SCHEMA,
        "input_sha256": input_sha256,
        "manifest_sha256": manifest_sha256,
        "source_revision": manifest.get("source_revision"),
        "law_version": LAW_VERSION,
        "identity_scope": identity_scope,
        "seed": manifest["seed"],
        "expected_rows_per_bucket": expected_rows_per_bucket,
        "row_count": len(rows),
        "structurally_complete_pair_count": structurally_complete_pair_count,
        "accepted_complete_pair_count": accepted_complete_pair_count,
        "pairs_with_rejected_rows": pairs_with_rejected_rows,
        "pair_acceptance_by_bucket": pair_acceptance_by_bucket,
        "accepted_rows": len(accepted),
        "rejected_rows": len(rows) - len(accepted),
        "status_counts": dict(sorted(Counter(row["validation_status"] for row in rows).items())),
        "normalization_agreement": {
            "tolerance": AGREEMENT_TOL,
            "max_factor_area_abs_error_from_one": max(area_errors) if area_errors else None,
            "max_product_volume_abs_error": max(volume_errors) if volume_errors else None,
        },
        "by_bucket_arm": by_bucket_arm,
        "scale_free_range_overlap": overlaps,
        "paired_expected_geometry_equalities": paired_deltas,
        "ridge_covariance_eligibility": {
            "status": "not_computed",
            "blocked_payload_api": [
                "normalized ridge fields require prepare-private features_face_symplectic plus two-face payload assembly; SysLandscapePolytopeCache exposes the parts but no shared feature API",
                "vertex covariance eligibility is private vertex_covariance_diagnostics in extreme-scalar-rejection-proposer and has no exported cache-to-diagnostics API",
            ],
        },
        "interpretation_boundary": (
            "Target-free generator matchability evidence only: audits paired construction, acceptance, "
            "normalization, exact product volume, and coarse factor-geometry overlap. It contains no "
            "capacity or sys and cannot support transfer, target-effect, population, or mechanism claims."
        ),
    }


def write_outputs(out_dir: Path, report: dict[str, Any]) -> None:
    out_dir.mkdir(parents=True, exist_ok=True)
    (out_dir / "report.json").write_text(json.dumps(report, indent=2, sort_keys=True) + "\n")
    normal = report["normalization_agreement"]
    lines = [
        "# Generator tangential matchability summary",
        "",
        f"- Rows: {report['row_count']}; structurally complete four-arm grids: {report['structurally_complete_pair_count']}.",
        f"- Accepted-complete geometry pairs: {report['accepted_complete_pair_count']}.",
        f"- Structural pairs containing rejected rows: {report['pairs_with_rejected_rows']}.",
        f"- Accepted rows: {report['accepted_rows']}; rejected rows: {report['rejected_rows']}.",
        f"- Maximum factor-area error from one: {normal['max_factor_area_abs_error_from_one']!r}.",
        f"- Maximum product-volume agreement error: {normal['max_product_volume_abs_error']!r}.",
        "- Ridge/covariance eligibility: not computed; see `report.json` for the blocked APIs.",
        "",
        report["interpretation_boundary"],
    ]
    (out_dir / "summary.md").write_text("\n".join(lines) + "\n")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--input", required=True, type=Path)
    parser.add_argument("--manifest", required=True, type=Path)
    parser.add_argument("--expected-rows-per-bucket", required=True, type=int)
    parser.add_argument("--identity-scope", required=True)
    parser.add_argument("--out-dir", required=True, type=Path)
    args = parser.parse_args()
    rows, input_sha256 = load_rows(args.input)
    manifest, manifest_sha256 = load_manifest(args.manifest)
    validate_packet(rows, manifest, args.expected_rows_per_bucket, args.identity_scope)
    report = analyze(
        rows,
        manifest,
        input_sha256,
        manifest_sha256,
        args.expected_rows_per_bucket,
        args.identity_scope,
    )
    write_outputs(args.out_dir, report)
    print(
        f"audited {report['row_count']} rows, "
        f"{report['structurally_complete_pair_count']} structural pairs, and "
        f"{report['accepted_complete_pair_count']} accepted-complete pairs"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
