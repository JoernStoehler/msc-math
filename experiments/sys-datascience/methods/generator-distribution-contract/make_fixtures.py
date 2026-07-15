#!/usr/bin/env python3
"""Generate the tiny deterministic calibration bundle."""

from __future__ import annotations

import argparse
import json
from pathlib import Path


SCHEMAS = {
    "law_runs": "generator-distribution-law-run-v1",
    "sampling_events": "generator-distribution-sampling-event-v1",
    "geometry_views": "generator-distribution-geometry-view-v1",
    "metrics": "generator-distribution-metric-preprocessing-v1",
}
COMMON_SIGNATURE = [[0, 1], [1, 2], [0, 2]]
RARE_SIGNATURE = [[0, 1], [1, 3], [2, 3], [0, 2]]
ALT_SIGNATURE = [[0, 1], [1, 3], [0, 2], [2, 3]]


def make(out: Path) -> None:
    laws: list[dict] = []
    events: list[dict] = []
    geometries: list[dict] = []
    metrics = [{
        "schema": SCHEMAS["metrics"],
        "metric_view_id": "incidence-signature-v1",
        "metric_version": "v1",
        "preprocessing_id": "none",
        "fit_reference": "not-fitted",
        "fit_split": "declared-independent-unit",
    }]

    law_specs = [
        ("null-law", "complete", "null", 12, "default", "facet=3"),
        ("collapsed-law", "complete", "collapsed", 8, "default", "facet=3"),
        ("rare-mixture", "complete", "rare", 20, "default", "facet=3"),
        ("rare-deleted", "complete", "deleted", 20, "default", "facet=3"),
        ("censored-law", "complete", "censoring", 6, "default", "facet=3"),
        ("paired-law", "complete", "paired", 8, "default", "facet=3"),
        ("missing-provenance", "accepted_only", "missing", 1, "default", "facet=3"),
        ("rejection-cost-law", "complete", "high-cost-rejection", 3, "default", "facet=3"),
        # One law_id with two independent runs/strata is a regression guard
        # against accidental cross-stratum pooling in the primary report.
        ("stratified-law", "complete", "stratum-a", 3, "a", "facet=3"),
        ("stratified-law", "complete", "stratum-b", 3, "b", "facet=4"),
    ]
    run_ids: dict[tuple[str, str], str] = {}
    for law_id, attempt_status, component, count, run_suffix, stratum in law_specs:
        run_id = f"run/{law_id}/{run_suffix}"
        run_ids[(law_id, run_suffix)] = run_id
        laws.append({
            "schema": SCHEMAS["law_runs"],
            "law_run_id": run_id,
            "law_id": law_id,
            "law_version": "fixture-v1",
            "source_sha256": "fixture-source-sha256",
            "source_path": "fixtures/synthetic.jsonl",
            "configuration": {"component": component, "count": count},
            "mixture_component": component,
            "stratum": stratum,
            "proposed_law": law_id,
            "accepted_law": law_id,
            "normalization": "unit-volume exact fixture",
            "attempt_log_status": attempt_status,
            "target_exposure_state": "absent",
        })

    def add_geometry(geometry_id: str, signature: list[list[int]] | None, exact: str | None = None, has_exact: bool = True) -> None:
        geometries.append({
            "schema": SCHEMAS["geometry_views"],
            "geometry_id": geometry_id,
            "view_id": "incidence-signature-v1",
            "view_version": "v1",
            "exact_geometry_hash": (exact if exact is not None else geometry_id) if has_exact else None,
            "exact_payload_pointer": f"fixture://{geometry_id}",
            "facet_count": 4 if signature == RARE_SIGNATURE or signature == ALT_SIGNATURE else 3,
            "coordinate_order": "q1,q2,p1,p2",
            "incidence_signature": signature,
            "representation": "fixture exact payload",
            "canonicalization": "fixture canonical facet labels",
            "injective": False,
            "lossy": True,
            "invariant_under": ["fixture_label_permutations"],
        })

    def add_event(event_id: str, law_id: str, unit: str, accepted: bool | None, geometry_id: str | None, cost: float | None, kind: str = "iid_draw", reason: str | None = None, attempt: int = 0, run_suffix: str = "default") -> None:
        events.append({
            "schema": SCHEMAS["sampling_events"],
            "event_id": event_id,
            "law_run_id": run_ids[(law_id, run_suffix)],
            "attempt_id": f"{unit}/attempt={attempt}",
            "independent_unit_id": unit,
            "independent_unit_kind": kind,
            "accepted": accepted,
            "rejection_reason": reason,
            "attempt_ordinal": attempt,
            "seed": 1000 + len(events),
            "lineage": unit,
            "cost_ms": cost,
            "pairing_id": unit if kind == "paired_block" else None,
            "geometry_id": geometry_id,
            "metric_view_id": "incidence-signature-v1" if geometry_id is not None else None,
        })

    # Same-law null: a fixed incidence type across an independent split.
    add_geometry("null-g", COMMON_SIGNATURE)
    for i in range(12):
        add_event(f"null-{i:02d}", "null-law", f"null-unit-{i:02d}", True, "null-g", 1.0 + i / 100)
    # Deliberately collapsed law: exact duplicates are visible.
    add_geometry("collapsed-g", COMMON_SIGNATURE)
    for i in range(8):
        add_event(f"collapsed-{i:02d}", "collapsed-law", f"collapsed-unit-{i:02d}", True, "collapsed-g", 2.0)
    # Rare type and a deletion control.
    add_geometry("common-g", COMMON_SIGNATURE)
    add_geometry("rare-g", RARE_SIGNATURE)
    add_geometry("deleted-g", COMMON_SIGNATURE)
    for i in range(20):
        add_event(f"mixture-{i:02d}", "rare-mixture", f"mixture-unit-{i:02d}", True, "rare-g" if i in {17, 19} else "common-g", 1.5)
        add_event(f"deleted-{i:02d}", "rare-deleted", f"deleted-unit-{i:02d}", True, "deleted-g", 1.5)
    # Rejections, explicit censoring, and a cost missingness witness.
    add_event("censor-a", "censored-law", "censor-a", True, "common-g", 3.0)
    add_event("censor-b", "censored-law", "censor-b", True, "common-g", None)
    add_event("censor-c", "censored-law", "censor-c", False, None, 2.0, reason="invalid geometry")
    add_event("censor-d", "censored-law", "censor-d", False, None, 2.5, reason="rejection cap")
    add_event("censor-e", "censored-law", "censor-e", None, None, None, reason="truncated stream")
    add_event("censor-f", "censored-law", "censor-f", True, "common-g", 3.0)
    # Two dependent paired rows per declared block. They must count as four units.
    add_geometry("paired-a", COMMON_SIGNATURE)
    add_geometry("paired-b", ALT_SIGNATURE)
    for i in range(4):
        unit = f"paired-unit-{i:02d}"
        add_event(f"paired-{i:02d}-a", "paired-law", unit, True, "paired-a", 4.0, "paired_block", attempt=0)
        add_event(f"paired-{i:02d}-b", "paired-law", unit, True, "paired-b", 4.0, "paired_block", attempt=1)
    add_geometry("strat-a-g", COMMON_SIGNATURE)
    add_geometry("strat-b-g", RARE_SIGNATURE)
    for i in range(3):
        add_event(f"strat-a-{i:02d}", "stratified-law", f"strat-a-unit-{i:02d}", True, "strat-a-g", 5.0, run_suffix="a")
        add_event(f"strat-b-{i:02d}", "stratified-law", f"strat-b-unit-{i:02d}", True, "strat-b-g", 5.0, run_suffix="b")
    add_geometry("missing-g", None, has_exact=False)
    add_event("missing-0", "missing-provenance", "missing-unit-0", True, "missing-g", None)
    add_event("rejection-cost-rejected", "rejection-cost-law", "rejection-cost-unit-0", False, None, 100.0, reason="invalid geometry")
    add_event("rejection-cost-accepted", "rejection-cost-law", "rejection-cost-unit-1", True, "common-g", 1.0)

    out.mkdir(parents=True, exist_ok=True)
    for name, rows in (("law_runs.jsonl", laws), ("sampling_events.jsonl", events), ("geometry_views.jsonl", geometries), ("metric_preprocessing.jsonl", metrics)):
        (out / name).write_text("".join(json.dumps(row, sort_keys=True, separators=(",", ":")) + "\n" for row in rows), encoding="utf-8")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--out-dir", type=Path, required=True)
    args = parser.parse_args()
    make(args.out_dir)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
