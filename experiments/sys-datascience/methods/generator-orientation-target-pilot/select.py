#!/usr/bin/env python3
"""Target-free selection/manifest producer for the frozen 8x3 grid."""
from pathlib import Path
import json
from analyze import REPO, SOURCE, SOURCE_REPORT, SOURCE_SHA, SOURCE_REPORT_SHA, digest, jsonl, validate_source


def repo_relative(path):
    return str(path.relative_to(REPO))

ROOT = Path(__file__).resolve().parent
selected = validate_source(jsonl(SOURCE))
manifest = {
    "schema": "generator-orientation-target-pilot-selection-v1",
    "source_path": repo_relative(SOURCE), "source_sha256": digest(SOURCE),
    "source_report_path": repo_relative(SOURCE_REPORT), "source_report_sha256": digest(SOURCE_REPORT),
    "required_source_sha256": SOURCE_SHA, "required_source_report_sha256": SOURCE_REPORT_SHA,
    "rows": 24, "buckets": ["3x3", "4x4", "4x6", "6x6"], "bases_per_bucket": 2,
    "variants": ["identity", "u2-haar", "so4-haar"], "pair_key": "base_id",
    "selected": [{"base_id": r["base_id"], "bucket": r["bucket"], "map_variant": v, "sample_id": r["sample_id"], "transformed_id": r["transformed_id"]} for (b,v),r in sorted(selected.items())],
    "target_fields_present": False, "target_calls": 0,
}
(ROOT / "selection-manifest.json").write_text(json.dumps(manifest, indent=2) + "\n")
print(f"selected {len(manifest['selected'])} target-free rows")
