#!/usr/bin/env python3
"""Extract the reviewed orientation rows from the full exact-feature artifact.

The full artifact is a stopped breadth-wave input and is intentionally not
retained by this packet.  This route copies the original JSONL lines without
re-serializing them, after checking the full artifact, source panel, and report
identities.  It never reads target fields or computes a feature.
"""
from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path

FULL_FEATURE_SHA = "e7cc585b2e774bc6ee5dcd658e49b02cefd7cdd914fb1ffaba759ccb64d6b624"
FULL_REPORT_SHA = "4982846e2a8828ba2e217b7b017605180927b2e040f96818d9eac9a405477e43"
SOURCE_SHA = "b5ded0a5e83d41f35ca035660d222326a161ce5001fd18c12f74f0ed9f3bc367"
SOURCE_REPORT_SHA = "02b7084141c0f2422aaabf1516fa62af501963ce638b9df3ef756c762722d61c"
SOURCE_REVISION = "875c5f6f8aff45013140e109d016ee34a61ff7cd"
FEATURE_SCHEMA = "generator-exact-feature-augmenter-row-v2"
FORBIDDEN = {"capacity", "sys", "iterations", "iteration", "bounce_label", "target", "target_ms"}


def digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def read_jsonl(path: Path) -> list[dict]:
    rows = []
    for line_number, line in enumerate(path.read_text().splitlines(), 1):
        if not line.strip():
            raise ValueError(f"blank JSONL line {line_number}: {path}")
        row = json.loads(line)
        if not isinstance(row, dict):
            raise ValueError(f"non-object JSONL line {line_number}: {path}")
        rows.append(row)
    return rows


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--full-features", type=Path, required=True)
    parser.add_argument("--full-report", type=Path, required=True)
    parser.add_argument("--source", type=Path, default=Path("../generator-orientation-smoke/artifacts/panel-2-per-bucket/rows.jsonl"))
    parser.add_argument("--source-report", type=Path, default=Path("../generator-orientation-smoke/artifacts/panel-2-per-bucket/report.json"))
    parser.add_argument("--out-dir", type=Path, default=Path("artifacts"))
    args = parser.parse_args()

    if digest(args.full_features) != FULL_FEATURE_SHA:
        raise SystemExit("full feature artifact hash mismatch")
    if digest(args.full_report) != FULL_REPORT_SHA:
        raise SystemExit("full feature report hash mismatch")
    if digest(args.source) != SOURCE_SHA or digest(args.source_report) != SOURCE_REPORT_SHA:
        raise SystemExit("orientation source/report hash mismatch")

    source_rows = read_jsonl(args.source)
    if len(source_rows) != 40 or any(row.get("schema") != "generator-orientation-smoke-row-v2" for row in source_rows):
        raise SystemExit("orientation source must contain exactly 40 rows")
    source_ids = {row.get("transformed_id") for row in source_rows}
    if len(source_ids) != 40 or None in source_ids:
        raise SystemExit("orientation source transformed IDs are not unique")

    full_lines = args.full_features.read_text().splitlines(keepends=True)
    full_rows = read_jsonl(args.full_features)
    if len(full_rows) != 808 or any(row.get("schema") != FEATURE_SCHEMA for row in full_rows):
        raise SystemExit("full feature artifact must contain 808 exact-feature rows")
    selected_lines = []
    selected_rows = []
    for line, row in zip(full_lines, full_rows):
        if row.get("source_kind") == "orientation":
            if any(key in row for key in FORBIDDEN):
                raise SystemExit("target field in orientation feature row")
            selected_lines.append(line)
            selected_rows.append(row)
    selected_ids = {row.get("source_id") for row in selected_rows}
    if len(selected_rows) != 40 or selected_ids != source_ids:
        raise SystemExit("orientation feature/source ID grid mismatch")
    if any(row.get("source_sample_id") != row.get("source_id") for row in selected_rows):
        raise SystemExit("orientation feature source_sample_id mismatch")

    args.out_dir.mkdir(parents=True, exist_ok=True)
    snapshot = args.out_dir / "orientation-features.jsonl"
    manifest_path = args.out_dir / "orientation-feature-manifest.json"
    snapshot.write_text("".join(selected_lines))
    snapshot_sha = digest(snapshot)
    id_bytes = "\n".join(sorted(selected_ids)).encode()
    manifest = {
        "schema": "generator-orientation-feature-snapshot-v1",
        "snapshot_path": str(snapshot),
        "snapshot_sha256": snapshot_sha,
        "snapshot_rows": 40,
        "snapshot_schema": FEATURE_SCHEMA,
        "source_kind": "orientation",
        "source_ids_sha256": hashlib.sha256(id_bytes).hexdigest(),
        "source_ids_count": 40,
        "orientation_source_sha256": SOURCE_SHA,
        "orientation_source_report_sha256": SOURCE_REPORT_SHA,
        "full_feature_sha256": FULL_FEATURE_SHA,
        "full_feature_report_sha256": FULL_REPORT_SHA,
        "full_feature_source_revision": SOURCE_REVISION,
        "selection_rule": "retain original JSONL lines with source_kind=orientation; require exact 40-ID set from orientation source transformed_id",
        "target_fields_present": False,
        "target_calls": 0,
    }
    manifest_path.write_text(json.dumps(manifest, indent=2) + "\n")
    print(f"extracted {len(selected_rows)} orientation rows; snapshot sha256={snapshot_sha}")


if __name__ == "__main__":
    main()
