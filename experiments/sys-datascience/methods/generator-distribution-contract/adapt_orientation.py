#!/usr/bin/env python3
"""Adapt the retained target-free orientation panel into linked records."""

from __future__ import annotations

import argparse
import hashlib
import json
import subprocess
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parent
SCHEMA = {
    "law_runs": "generator-distribution-law-run-v1",
    "sampling_events": "generator-distribution-sampling-event-v1",
    "geometry_views": "generator-distribution-geometry-view-v1",
    "metrics": "generator-distribution-metric-preprocessing-v1",
}


def _sha(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1 << 20), b""):
            digest.update(chunk)
    return digest.hexdigest()


def _write(path: Path, rows: list[dict[str, Any]]) -> None:
    path.write_text("".join(json.dumps(row, sort_keys=True, separators=(",", ":")) + "\n" for row in rows), encoding="utf-8")


def _source_provenance(source: Path, repo_root: Path, supplied_revision: str | None = None) -> tuple[str, str]:
    """Return a stable source label and revision for in- or out-of-repo input."""
    source = source.resolve()
    try:
        source_label = str(source.relative_to(repo_root))
    except ValueError:
        source_label = str(source)
        return source_label, supplied_revision or "outside-repository/not-recorded"
    revision = supplied_revision or subprocess.check_output(
        ["git", "-C", str(repo_root), "log", "-1", "--format=%H", "--", source_label],
        text=True,
    ).strip()
    if not revision:
        raise ValueError(f"no source-owned revision found for {source_label}")
    return source_label, revision


RUN_SEMANTIC_FIELDS = (
    "map_variant", "map_family", "map_mode", "bucket", "coordinate_order",
    "q_sides", "p_sides", "base_facet_count", "base_vertex_count", "facet_count", "vertex_count",
)


def _run_semantics(row: dict[str, Any]) -> dict[str, Any]:
    """Extract fields that must agree for every row in one declared run."""
    missing = [field for field in RUN_SEMANTIC_FIELDS if field not in row or row[field] is None]
    if missing:
        raise ValueError(f"source row missing run-semantic fields: {', '.join(missing)}")
    return {field: row[field] for field in RUN_SEMANTIC_FIELDS}


def _check_run_semantics(existing: dict[str, Any], current: dict[str, Any], run_id: str) -> None:
    for field in RUN_SEMANTIC_FIELDS:
        if existing[field] != current[field]:
            raise ValueError(
                f"run {run_id!r}: source rows disagree on {field}: "
                f"{existing[field]!r} versus {current[field]!r}"
            )


def adapt(source: Path, out_dir: Path, supplied_revision: str | None = None) -> dict[str, Any]:
    source = source.resolve()
    repo_root = ROOT.parents[3]
    source_label, source_revision = _source_provenance(source, repo_root, supplied_revision)
    source_hash = _sha(source)
    rows = [json.loads(line) for line in source.read_text(encoding="utf-8").splitlines() if line.strip()]
    law_runs: dict[str, dict[str, Any]] = {}
    events: list[dict[str, Any]] = []
    geometries: list[dict[str, Any]] = []
    metrics: dict[str, dict[str, Any]] = {}
    run_semantics: dict[str, dict[str, Any]] = {}
    for row in rows:
        if row.get("schema") != "generator-orientation-smoke-row-v2":
            raise ValueError(f"unexpected source schema: {row.get('schema')!r}")
        variant = row["map_variant"]
        law_id = f"generator-orientation/{variant}"
        run_id = f"{law_id}/bucket={row['bucket']}"
        current_semantics = _run_semantics(row)
        if run_id in run_semantics:
            _check_run_semantics(run_semantics[run_id], current_semantics, run_id)
        else:
            run_semantics[run_id] = current_semantics
        if run_id not in law_runs:
            law_runs[run_id] = {
                "schema": SCHEMA["law_runs"],
                "law_run_id": run_id,
                "law_id": law_id,
                "law_version": row["schema"],
                "source_sha256": source_hash,
                "source_path": source_label,
                "source_revision": source_revision,
                "configuration": {
                    "map_variant": variant,
                    "map_family": row["map_family"],
                    "map_mode": row["map_mode"],
                    "bucket": row["bucket"],
                    "coordinate_order": row["coordinate_order"],
                },
                "mixture_component": variant,
                "stratum": row["bucket"],
                "proposed_law": "generator-orientation-v1",
                "accepted_law": law_id,
                "normalization": "area-normalized Lagrangian product (source exact-volume witness)",
                "attempt_log_status": "accepted_only",
                "target_exposure_state": "absent",
            }
        metric_id = "incidence-signature-v1"
        metrics.setdefault(metric_id, {
            "schema": SCHEMA["metrics"],
            "metric_view_id": metric_id,
            "metric_version": "v1",
            "preprocessing_id": "none",
            "fit_reference": "not-fitted",
            "fit_split": "independent-base-id",
        })
        geometry_id = row["transformed_id"]
        geometries.append({
            "schema": SCHEMA["geometry_views"],
            "geometry_id": geometry_id,
            "view_id": metric_id,
            "view_version": "v1",
            "exact_geometry_hash": row.get("transformed_geometry_id"),
            "exact_payload_pointer": f"{source_label}#sample_id={row['sample_id']}",
            "facet_count": row["facet_count"],
            "coordinate_order": row["coordinate_order"],
            "incidence_signature": row.get("labeled_incidence_signature"),
            "representation": "rational dual vertices + reconstructed rational primal vertices",
            "canonicalization": "source labeled facet order; no permutation quotient",
            # The labeled incidence view is invariant under the source's
            # invertible affine map action but is lossy/non-injective for exact
            # geometry.  Keep those dimensions explicit rather than forcing
            # an exclusive semantic label.
            "injective": False,
            "lossy": True,
            "invariant_under": ["invertible_affine_maps", "facet_label_preserving"],
        })
        events.append({
            "schema": SCHEMA["sampling_events"],
            "event_id": row["sample_id"],
            "law_run_id": run_id,
            "attempt_id": f"{row['base_id']}/attempt={row['accepted_attempt']}",
            "independent_unit_id": row["base_id"],
            "independent_unit_kind": "paired_block",
            "accepted": True,
            "rejection_reason": None,
            "attempt_ordinal": row.get("accepted_attempt"),
            "seed": row.get("base_seed"),
            "lineage": row["base_id"],
            "cost_ms": sum(float(row.get(key) or 0.0) for key in ("generation_ms", "transform_ms", "reconstruction_ms")),
            "pairing_id": row["base_id"],
            "geometry_id": geometry_id,
            "metric_view_id": metric_id,
        })
    # Exact source IDs make duplicate input rows a hard failure rather than a
    # silently inflated audit.
    if len({row["event_id"] for row in events}) != len(events):
        raise ValueError("duplicate source sample_id")
    if len({row["geometry_id"] for row in geometries}) != len(geometries):
        raise ValueError("duplicate source transformed_id")
    out_dir.mkdir(parents=True, exist_ok=True)
    _write(out_dir / "law_runs.jsonl", sorted(law_runs.values(), key=lambda row: row["law_run_id"]))
    _write(out_dir / "sampling_events.jsonl", sorted(events, key=lambda row: row["event_id"]))
    _write(out_dir / "geometry_views.jsonl", sorted(geometries, key=lambda row: row["geometry_id"]))
    _write(out_dir / "metric_preprocessing.jsonl", sorted(metrics.values(), key=lambda row: row["metric_view_id"]))
    manifest = {
        "schema": "generator-distribution-orientation-adapter-report-v1",
        "source_path": source_label,
        "source_sha256": source_hash,
        "source_revision": source_revision,
        "source_rows": len(rows),
        "law_runs": len(law_runs),
        "sampling_events": len(events),
        "geometry_views": len(geometries),
        "independent_units": len({row["base_id"] for row in rows}),
        "attempt_log_status": "accepted_only",
        "target_exposure_state": "absent",
        "identifiable": {
            "exact_geometry_identity": True,
            "incidence_signature": True,
            "cost": True,
            "acceptance_rate": False,
            "attempts_and_rejections": False,
        },
    }
    (out_dir / "adapter-report.json").write_text(json.dumps(manifest, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    return manifest


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--source", type=Path, required=True)
    parser.add_argument("--out-dir", type=Path, required=True)
    parser.add_argument("--source-revision", help="Stable supplied revision for an outside-repository source")
    args = parser.parse_args()
    adapt(args.source, args.out_dir, args.source_revision)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
