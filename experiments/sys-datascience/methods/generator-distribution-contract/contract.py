#!/usr/bin/env python3
"""Copy-local linked-record contract and target-free discrete audit.

The packet deliberately keeps law/run, sampling, geometry/view, and
metric/preprocessing records separate.  It is a small standard-library-only
consumer so a future distribution packet can copy it without importing this
directory.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import math
from collections import Counter, defaultdict
from pathlib import Path
from typing import Any


SCHEMAS = {
    "law_runs": "generator-distribution-law-run-v1",
    "sampling_events": "generator-distribution-sampling-event-v1",
    "geometry_views": "generator-distribution-geometry-view-v1",
    "metrics": "generator-distribution-metric-preprocessing-v1",
}
FILES = {
    "law_runs": "law_runs.jsonl",
    "sampling_events": "sampling_events.jsonl",
    "geometry_views": "geometry_views.jsonl",
    "metrics": "metric_preprocessing.jsonl",
}
FORBIDDEN_KEYS = {
    "sys", "capacity", "iterations", "iteration", "bounce_label", "target",
    "target_ms", "target_derived_selection", "selected_by_target", "target_value",
}


class ContractError(ValueError):
    """A linked-record contract violation."""


def _forbidden(value: Any, path: str = "row") -> None:
    if isinstance(value, dict):
        for key, nested in value.items():
            if key in FORBIDDEN_KEYS:
                raise ContractError(f"forbidden target field {path}.{key}")
            _forbidden(nested, f"{path}.{key}")
    elif isinstance(value, list):
        for index, nested in enumerate(value):
            _forbidden(nested, f"{path}[{index}]")


def _read(path: Path, kind: str) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    if not path.exists():
        raise ContractError(f"missing required record file: {path}")
    with path.open(encoding="utf-8") as handle:
        for line_no, line in enumerate(handle, 1):
            if not line.strip():
                continue
            try:
                row = json.loads(line)
            except json.JSONDecodeError as exc:
                raise ContractError(f"{path}:{line_no}: invalid JSON: {exc}") from exc
            if not isinstance(row, dict):
                raise ContractError(f"{path}:{line_no}: record must be an object")
            _forbidden(row, f"{path}:{line_no}")
            if row.get("schema") != SCHEMAS[kind]:
                raise ContractError(
                    f"{path}:{line_no}: expected schema {SCHEMAS[kind]!r}, got {row.get('schema')!r}"
                )
            rows.append(row)
    return rows


def _nonempty(row: dict[str, Any], key: str, context: str) -> str:
    value = row.get(key)
    if not isinstance(value, str) or not value:
        raise ContractError(f"{context}: {key} must be a non-empty string")
    return value


def _finite_optional(value: Any, key: str, context: str) -> None:
    if value is None:
        return
    if isinstance(value, bool) or not isinstance(value, (int, float)) or not math.isfinite(float(value)):
        raise ContractError(f"{context}: {key} must be finite numeric or null")


def _validate_law(row: dict[str, Any], index: int) -> None:
    context = f"law_runs[{index}]"
    for key in ("law_run_id", "law_id", "law_version", "source_sha256", "proposed_law", "accepted_law", "normalization", "attempt_log_status", "target_exposure_state"):
        _nonempty(row, key, context)
    if not isinstance(row.get("configuration"), dict):
        raise ContractError(f"{context}: configuration must be an object")
    if row["attempt_log_status"] not in {"complete", "accepted_only", "absent"}:
        raise ContractError(f"{context}: invalid attempt_log_status")
    if row["target_exposure_state"] not in {"absent", "not_requested"}:
        raise ContractError(f"{context}: target_exposure_state must be absent/not_requested")
    if row.get("stratum") is not None and not isinstance(row["stratum"], str):
        raise ContractError(f"{context}: stratum must be a string or null")


def _validate_event(row: dict[str, Any], index: int) -> None:
    context = f"sampling_events[{index}]"
    for key in ("event_id", "law_run_id", "attempt_id", "independent_unit_id", "independent_unit_kind"):
        _nonempty(row, key, context)
    if row["independent_unit_kind"] not in {"iid_draw", "block", "paired_block"}:
        raise ContractError(f"{context}: invalid independent_unit_kind")
    if not isinstance(row.get("accepted"), (bool, type(None))):
        raise ContractError(f"{context}: accepted must be true, false, or null (unknown censor)")
    if row.get("rejection_reason") is not None and not isinstance(row["rejection_reason"], str):
        raise ContractError(f"{context}: rejection_reason must be string or null")
    _finite_optional(row.get("cost_ms"), "cost_ms", context)
    _finite_optional(row.get("attempt_ordinal"), "attempt_ordinal", context)
    if row.get("seed") is not None and (isinstance(row["seed"], bool) or not isinstance(row["seed"], int)):
        raise ContractError(f"{context}: seed must be integer or null")
    if row["accepted"] is False and row.get("geometry_id") is not None:
        raise ContractError(f"{context}: rejected event cannot reference geometry")


def _validate_geometry(row: dict[str, Any], index: int) -> None:
    context = f"geometry_views[{index}]"
    for key in ("geometry_id", "view_id", "view_version", "coordinate_order", "representation", "canonicalization"):
        _nonempty(row, key, context)
    if row["coordinate_order"] != "q1,q2,p1,p2":
        raise ContractError(f"{context}: coordinate_order must be q1,q2,p1,p2")
    if not isinstance(row.get("injective"), bool) or not isinstance(row.get("lossy"), bool):
        raise ContractError(f"{context}: injective and lossy must be boolean")
    if not isinstance(row.get("invariant_under"), list) or any(
        not isinstance(action, str) or not action for action in row["invariant_under"]
    ):
        raise ContractError(f"{context}: invariant_under must be a list of action names")
    if not isinstance(row.get("facet_count"), int) or row["facet_count"] <= 0:
        raise ContractError(f"{context}: facet_count must be positive integer")
    exact_hash = row.get("exact_geometry_hash")
    if exact_hash is not None:
        _nonempty(row, "exact_geometry_hash", context)
    incidence = row.get("incidence_signature")
    if incidence is not None:
        if not isinstance(incidence, list) or not incidence or any(
            not isinstance(face, list) or not face
            or any(not isinstance(i, int) or i < 0 or i >= row["facet_count"] for i in face)
            for face in incidence
        ):
            raise ContractError(f"{context}: incidence_signature must be lists of facet indices")


def _validate_metric(row: dict[str, Any], index: int) -> None:
    context = f"metric_preprocessing[{index}]"
    for key in ("metric_view_id", "metric_version", "preprocessing_id", "fit_reference", "fit_split"):
        _nonempty(row, key, context)


def load_bundle(directory: Path) -> dict[str, list[dict[str, Any]]]:
    """Load, validate, and join the four linked record files."""
    records = {kind: _read(directory / filename, kind) for kind, filename in FILES.items()}
    validators = {
        "law_runs": _validate_law,
        "sampling_events": _validate_event,
        "geometry_views": _validate_geometry,
        "metrics": _validate_metric,
    }
    key_names = {"law_runs": "law_run_id", "sampling_events": "event_id", "geometry_views": "geometry_id", "metrics": "metric_view_id"}
    for kind, rows in records.items():
        seen: set[str] = set()
        key = key_names[kind]
        for index, row in enumerate(rows):
            validators[kind](row, index)
            value = _nonempty(row, key, f"{kind}[{index}]")
            if value in seen:
                raise ContractError(f"duplicate {kind} key {value!r}")
            seen.add(value)
    laws = {row["law_run_id"]: row for row in records["law_runs"]}
    geometries = {row["geometry_id"]: row for row in records["geometry_views"]}
    metrics = {row["metric_view_id"]: row for row in records["metrics"]}
    geometry_views = {row["geometry_id"]: row["view_id"] for row in records["geometry_views"]}
    for geometry in records["geometry_views"]:
        if geometry["view_id"] not in metrics:
            raise ContractError(
                f"geometry {geometry['geometry_id']!r} references unknown metric/preprocessing view "
                f"{geometry['view_id']!r}"
            )
    seen_attempts: dict[str, set[str]] = defaultdict(set)
    seen_iid_units: dict[str, set[str]] = defaultdict(set)
    for event in records["sampling_events"]:
        if event["law_run_id"] not in laws:
            raise ContractError(f"event {event['event_id']!r} references unknown law_run_id")
        run_id = event["law_run_id"]
        if event["attempt_id"] in seen_attempts[run_id]:
            raise ContractError(f"law run {run_id!r} repeats attempt_id {event['attempt_id']!r}")
        seen_attempts[run_id].add(event["attempt_id"])
        if event["independent_unit_kind"] == "iid_draw":
            if event["independent_unit_id"] in seen_iid_units[run_id]:
                raise ContractError(
                    f"law run {run_id!r}: iid_draw independent unit {event['independent_unit_id']!r} "
                    "has more than one sampling event"
                )
            seen_iid_units[run_id].add(event["independent_unit_id"])
        metric_id = event.get("metric_view_id")
        if metric_id is not None and metric_id not in metrics:
            raise ContractError(f"event {event['event_id']!r} references unknown metric_view_id")
        geometry_id = event.get("geometry_id")
        if event["accepted"] is True and geometry_id is None:
            # Accepted-but-no-geometry is allowed: this is a visible missingness
            # disposition, not a silently fabricated geometry.
            continue
        if geometry_id is not None and geometry_id not in geometries:
            raise ContractError(f"event {event['event_id']!r} references unknown geometry_id")
        if geometry_id is not None:
            geometry_view_id = geometry_views[geometry_id]
            if metric_id != geometry_view_id:
                raise ContractError(
                    f"event {event['event_id']!r}: metric_view_id {metric_id!r} disagrees with "
                    f"geometry view_id {geometry_view_id!r}"
                )
    return records


def _unit_sort_key(unit_id: str) -> tuple[bytes, str]:
    return hashlib.sha256(unit_id.encode("utf-8")).digest(), unit_id


def _signature(row: dict[str, Any]) -> str | None:
    value = row.get("incidence_signature")
    if value is None:
        return None
    return json.dumps(value, separators=(",", ":"), sort_keys=True)


def audit(records: dict[str, list[dict[str, Any]]]) -> dict[str, Any]:
    """Compute the audit, with one primary report per declared law/run."""
    laws = {row["law_run_id"]: row for row in records["law_runs"]}
    geometries = {row["geometry_id"]: row for row in records["geometry_views"]}
    by_run: dict[str, list[dict[str, Any]]] = defaultdict(list)
    for event in records["sampling_events"]:
        by_run[event["law_run_id"]].append(event)
    reports: list[dict[str, Any]] = []
    for law_run_id in sorted(by_run):
        events = by_run[law_run_id]
        run_row = laws[law_run_id]
        law_id = run_row["law_id"]
        run_rows = [run_row]
        statuses = Counter("accepted" if e["accepted"] is True else "rejected" if e["accepted"] is False else "unknown_censor" for e in events)
        known = statuses["accepted"] + statuses["rejected"]
        costs = [float(e["cost_ms"]) for e in events if e.get("cost_ms") is not None]
        exact_hashes: list[str] = []
        signatures_by_unit: dict[str, set[str]] = defaultdict(set)
        unit_kind: dict[str, str] = {}
        for event in events:
            unit = event["independent_unit_id"]
            prior_kind = unit_kind.get(unit)
            if prior_kind is not None and prior_kind != event["independent_unit_kind"]:
                raise ContractError(
                    f"law run {law_run_id!r}: independent unit {unit!r} has inconsistent kinds "
                    f"{prior_kind!r} and {event['independent_unit_kind']!r}"
                )
            unit_kind[unit] = event["independent_unit_kind"]
            if event["accepted"] is not True:
                continue
            geometry = geometries.get(event.get("geometry_id"))
            if geometry is None:
                continue
            if geometry.get("exact_geometry_hash") is not None:
                exact_hashes.append(geometry["exact_geometry_hash"])
            signature = _signature(geometry)
            if signature is not None:
                signatures_by_unit[unit].add(signature)
        multiplicities = Counter(exact_hashes)
        unit_ids = sorted(unit_kind, key=_unit_sort_key)
        discovery: list[dict[str, int]] = []
        seen_signatures: set[str] = set()
        for count, unit in enumerate(unit_ids, 1):
            seen_signatures.update(signatures_by_unit.get(unit, set()))
            discovery.append({"independent_units": count, "unique_incidence_signatures": len(seen_signatures)})
        signature_occurrences = Counter(signature for values in signatures_by_unit.values() for signature in values)
        all_one_signature = all(len(signatures_by_unit.get(unit, set())) == 1 for unit in unit_ids)
        iid_only = all(unit_kind[unit] == "iid_draw" for unit in unit_ids)
        unknown = statuses["unknown_censor"] > 0
        complete_log = run_row["attempt_log_status"] == "complete"
        iid_event_log = all(kind == "iid_draw" for kind in unit_kind.values())
        good_turing: float | None = None
        if unit_ids and complete_log and iid_only and not unknown and all_one_signature:
            good_turing = sum(1 for count in signature_occurrences.values() if count == 1) / len(unit_ids)
        split = len(unit_ids) // 2
        train_units, heldout_units = unit_ids[:split], unit_ids[split:]
        train_signatures = set().union(*(signatures_by_unit.get(unit, set()) for unit in train_units)) if train_units else set()
        heldout_with_signature = [unit for unit in heldout_units if signatures_by_unit.get(unit)]
        heldout_new = [unit for unit in heldout_with_signature if not signatures_by_unit[unit] <= train_signatures]
        diagnostics: list[str] = []
        if any(row["attempt_log_status"] != "complete" for row in run_rows):
            diagnostics.append("attempt log incomplete or absent: acceptance rate may be unidentifiable")
        if statuses["unknown_censor"]:
            diagnostics.append("unknown/censored sampling events prevent an unconditional acceptance rate")
        if len(costs) < len(events):
            diagnostics.append("cost missing for some sampling events")
        if len(exact_hashes) < statuses["accepted"]:
            diagnostics.append("exact geometry identity missing for some accepted events")
        if sum(bool(values) for values in signatures_by_unit.values()) < len(unit_ids):
            diagnostics.append("incidence signatures missing for some independent units")
        if any(kind != "iid_draw" for kind in unit_kind.values()):
            diagnostics.append("dependent block/paired rows counted as one declared independent unit")
            if complete_log:
                diagnostics.append("complete event log still cannot identify attempted-draw counts across dependent rows")
        if not unit_ids or not train_units or not heldout_units:
            diagnostics.append("held-out split unavailable at this unit count")
        cost_complete = len(costs) == len(events)
        accepted_exact_events = [
            e for e in events
            if e["accepted"] is True
            and geometries.get(e.get("geometry_id"), {}).get("exact_geometry_hash") is not None
        ]
        accepted_exact_costs = [e.get("cost_ms") for e in accepted_exact_events]
        accepted_exact_cost_complete = bool(accepted_exact_events) and all(cost is not None for cost in accepted_exact_costs)
        total_recorded_sampling_cost = sum(costs) if costs else None
        total_cost_identifiable = complete_log and iid_event_log and not unknown and cost_complete and bool(accepted_exact_events)
        if good_turing is not None:
            good_turing_status = "available_complete_iid_one-signature-per-unit"
        elif not complete_log:
            good_turing_status = "not-identifiable-attempt-provenance"
        elif unknown:
            good_turing_status = "not-identifiable-unknown-censor"
        elif not iid_only:
            good_turing_status = "not-identifiable-dependent-units"
        elif not all_one_signature:
            good_turing_status = "not-identifiable-missing-or-multiple-signatures"
        else:
            good_turing_status = "not-identifiable-assumptions-fail"
        report = {
            "law_run_id": law_run_id,
            "law_id": law_id,
            "stratum": run_row.get("stratum"),
            "recorded_event_rows": len(events),
            "attempt_count": len(events) if complete_log and iid_event_log else None,
            "attempt_count_status": "identifiable_complete_iid_log" if complete_log and iid_event_log else "unidentifiable_dependent_rows_or_incomplete_log",
            "accepted": statuses["accepted"],
            "rejected": statuses["rejected"],
            "unknown_censor": statuses["unknown_censor"],
            "acceptance_rate": statuses["accepted"] / known if known and not unknown and complete_log and iid_event_log else None,
            "cost_ms_per_recorded_event": sum(costs) / len(costs) if costs else None,
            "cost_ms_per_recorded_event_status": "complete" if cost_complete else "incomplete_missing_cost",
            "cost_ms_per_attempt": sum(costs) / len(costs) if complete_log and iid_event_log and cost_complete and costs else None,
            "cost_ms_per_attempt_status": "identifiable_complete_iid_log_and_cost" if complete_log and iid_event_log and cost_complete else "unidentifiable_dependent_rows_incomplete_log_or_cost",
            "mean_processing_cost_ms_per_accepted_exact_event": sum(float(cost) for cost in accepted_exact_costs) / len(accepted_exact_costs) if accepted_exact_cost_complete else None,
            "mean_processing_cost_ms_per_accepted_exact_event_status": "complete" if accepted_exact_cost_complete else "unidentifiable_missing_cost_or_exact_geometry",
            "total_recorded_sampling_cost_ms_per_accepted_exact_result": total_recorded_sampling_cost / len(accepted_exact_events) if total_cost_identifiable else None,
            "total_recorded_sampling_cost_ms_per_accepted_exact_result_status": "identifiable_complete_uncensored_iid_log_and_cost" if total_cost_identifiable else "unidentifiable_incomplete_or_dependent_log_censor_or_cost",
            "exact_duplicate_multiplicities": sorted(multiplicities.values(), reverse=True),
            "exact_duplicate_groups": sum(1 for count in multiplicities.values() if count > 1),
            "independent_units": len(unit_ids),
            "independent_unit_kinds": dict(sorted(Counter(unit_kind.values()).items())),
            "incidence_discovery_curve": discovery,
            "incidence_rarefaction_curve": discovery,
            "discovery_split_semantics": "cumulative deterministic SHA-256-ranked independent units; dependent rows grouped",
            "singleton_signature_count": sum(count == 1 for count in signature_occurrences.values()),
            "doubleton_signature_count": sum(count == 2 for count in signature_occurrences.values()),
            "unseen_signature_mass_good_turing": good_turing,
            "unseen_signature_mass_status": good_turing_status,
            "heldout_new_signature_rate": len(heldout_new) / len(heldout_with_signature) if heldout_with_signature else None,
            "heldout_units": len(heldout_units),
            "heldout_units_with_signature": len(heldout_with_signature),
            "diagnostics": diagnostics,
        }
        reports.append(report)
    return {
        "schema": "generator-distribution-audit-report-v1",
        "statistical_unit": "primary report is one law_run_id/stratum; within it declared independent_unit_id is the statistical unit, with paired/block rows grouped and never treated as IID events",
        "laws": reports,
    }


def write_report(records_dir: Path, out: Path) -> dict[str, Any]:
    records = load_bundle(records_dir)
    report = audit(records)
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    return report


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--input-dir", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    try:
        write_report(args.input_dir, args.output)
    except ContractError as exc:
        parser.error(str(exc))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
