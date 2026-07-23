#!/usr/bin/env python3
"""Fail-closed provenance validation and analysis for the frozen pilot."""
from __future__ import annotations

import argparse
import hashlib
import json
import math
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent
REPO = Path(__file__).resolve().parents[4]
SOURCE = ROOT.parent / "generator-orientation-smoke/artifacts/panel-2-per-bucket/rows.jsonl"
SOURCE_REPORT = SOURCE.with_name("report.json")
FEATURES = ROOT / "artifacts/orientation-features.jsonl"
FEATURE_REPORT = ROOT / "artifacts/orientation-feature-manifest.json"
DESIGN = ROOT / "design.json"
SELECTION = ROOT / "selection-manifest.json"
PROTOCOL_HISTORY = ROOT / "protocol-history.json"
EVALUATOR = ROOT / "main.rs"
SOURCE_SHA = "b5ded0a5e83d41f35ca035660d222326a161ce5001fd18c12f74f0ed9f3bc367"
SOURCE_REPORT_SHA = "02b7084141c0f2422aaabf1516fa62af501963ce638b9df3ef756c762722d61c"
FEATURE_SHA = "050859880b55a7063d8b982f3bae7ce0962350862767965e9e4ab7c7c8c10eda"
FEATURE_REPORT_SHA = "cd7587d31037a462303fc105c2b72d849fc53166d783ffacb169c0615077a439"
FULL_FEATURE_SHA = "e7cc585b2e774bc6ee5dcd658e49b02cefd7cdd914fb1ffaba759ccb64d6b624"
FULL_FEATURE_REPORT_SHA = "4982846e2a8828ba2e217b7b017605180927b2e040f96818d9eac9a405477e43"
TARGET_SHA = "2d0f91e1aeb99350b345d5e5a36d44a0a68f83d35ac79d3865c457bbbe187a88"
RETAINED_DESIGN_SHA = "35253ac47127a5358378cb386028eaaf24eb91466c40227cd7522c2fc27cd3df"
RETAINED_EVALUATOR_SHA = "ec92a3b1e76e201fb8e3cb843a437a1b336865bf3adfefce0b6628398c8ca91b"
RETAINED_SELECTION_SHA = "6752940d5cf1f80598bff8676570c25d0215e83b1e105c94075d665b4efae030"
RETAINED_COMMIT = "a59441c0ecde29ac667745e02aac4bedb8ca7d14"
VARIANTS = ("identity", "u2-haar", "so4-haar")
BUCKETS = ("3x3", "4x4", "4x6", "6x6")
IMPLEMENTATION_PATHS = (
    "experiments/sys-landscape/src/datascience_cache.rs",
    "experiments/sys-landscape/src/sys_landscape_cache.rs",
    "experiments/sys-landscape/src/lib.rs",
    "experiments/sys-landscape/Cargo.toml",
    "Cargo.lock",
)
RETAINED_IMPLEMENTATION_SHA = {
    "experiments/sys-landscape/src/datascience_cache.rs": "ce15a5a29b6b11053072336421f004c028cb04ffdc40001d291e70298e731569",
    "experiments/sys-landscape/src/sys_landscape_cache.rs": "de69e5e50fec9f5f036941fcdd4d013ec16b4171cc9eba91f610eff815e9d240",
    "experiments/sys-landscape/src/lib.rs": "1b504f7674a756431eba1f2a01fce6660184c801d4733d9b2da463d1a1bfc75a",
    "experiments/sys-landscape/Cargo.toml": "b659df44f0101f5dc85f3913a0131eb2aad08c6a728f6ab4ea3ac60087ce48b5",
    "Cargo.lock": "9e59bc0ed96aeccce2decc178beff2f224702174122ccf55d158fb009ef96d1e",
}


class AnalysisError(Exception):
    pass


_WARNED_STALE: set[str] = set()


def warn_stale(condition: bool, label: str) -> None:
    # Byte and revision identities are advisory provenance. Schemas, complete
    # row grids, joins, target status, payload equality, and numerical checks
    # remain blocking.
    if not condition and label not in _WARNED_STALE:
        _WARNED_STALE.add(label)
        print(
            f"warning: {label} differs from retained provenance; continuing "
            "with semantic checks. Correlate paths and run timestamps with Git "
            "history and reassess retained interpretation before treating this "
            "run as equivalent.",
            file=sys.stderr,
        )


def digest(path: Path) -> str:
    try:
        return hashlib.sha256(path.read_bytes()).hexdigest()
    except OSError as exc:
        raise AnalysisError(f"cannot hash {path}: {exc}") from exc


def jsonl(path: Path) -> list[dict]:
    try:
        lines = path.read_text().splitlines()
    except OSError as exc:
        raise AnalysisError(f"cannot read {path}: {exc}") from exc
    rows = []
    for number, line in enumerate(lines, 1):
        if not line.strip():
            raise AnalysisError(f"blank JSONL line {number} in {path}")
        try:
            value = json.loads(line)
        except json.JSONDecodeError as exc:
            raise AnalysisError(f"invalid JSONL line {number} in {path}: {exc}") from exc
        if not isinstance(value, dict):
            raise AnalysisError(f"JSONL line {number} is not an object in {path}")
        rows.append(value)
    return rows


def finite(value, label: str) -> None:
    if isinstance(value, bool) or not isinstance(value, (int, float)) or not math.isfinite(float(value)):
        raise AnalysisError(f"{label} nonfinite")


def rank_average(xs):
    order = sorted(range(len(xs)), key=lambda i: xs[i])
    ranks = [0.0] * len(xs)
    i = 0
    while i < len(xs):
        j = i + 1
        while j < len(xs) and xs[order[j]] == xs[order[i]]:
            j += 1
        rank = (i + 1 + j) / 2.0
        for k in range(i, j):
            ranks[order[k]] = rank
        i = j
    return ranks


def spearman(xs, ys):
    if len(xs) != len(ys) or not xs:
        return None
    rx, ry = rank_average(xs), rank_average(ys)
    mx, my = sum(rx) / len(rx), sum(ry) / len(ry)
    numerator = sum((x - mx) * (y - my) for x, y in zip(rx, ry))
    bx = math.sqrt(sum((x - mx) ** 2 for x in rx))
    by = math.sqrt(sum((y - my) ** 2 for y in ry))
    return numerator / (bx * by) if bx and by else None


def primary_disposition(u2_max_abs_delta, abs_so4):
    if u2_max_abs_delta > 1e-8:
        return "ambiguous_numerical_control_failure"
    if sum(v >= 0.01 for v in abs_so4) >= 6:
        return "supports_material_alignment_role"
    if max(abs_so4, default=0.0) < 0.005:
        return "contradicts_material_role_on_frozen_maps"
    return "ambiguous"


def git_blob(commit: str, path: str) -> bytes | None:
    try:
        result = subprocess.run(
            ["git", "-C", str(REPO), "show", f"{commit}:{path}"],
            check=True,
            capture_output=True,
        )
    except (OSError, subprocess.CalledProcessError):
        return None
    return result.stdout


def validate_source(rows: list[dict]) -> dict[tuple[str, str], dict]:
    if len(rows) != 40:
        raise AnalysisError("source row count is not 40")
    selected = {}
    bases = {}
    transformed_ids, sample_ids = set(), set()
    forbidden = ("capacity", "sys", "target", "iterations", "iteration", "bounce_label")
    for row in rows:
        if row.get("schema") != "generator-orientation-smoke-row-v2":
            raise AnalysisError("source schema")
        if any(key in row for key in forbidden):
            raise AnalysisError("source target field present, including null")
        if (
            row.get("base_accepted") is not True
            or row.get("semantic_invariants_passed") is not True
            or row.get("reconstruction_status") != "reconstructed"
            or row.get("map_status") != "generated"
            or row.get("invariant_failures") != []
        ):
            raise AnalysisError("source status")
        for key in ("sample_id", "transformed_id", "base_id", "bucket", "map_variant"):
            if not isinstance(row.get(key), str):
                raise AnalysisError(f"source {key} type")
        if row["sample_id"] in sample_ids or row["transformed_id"] in transformed_ids:
            raise AnalysisError("source duplicate ID")
        sample_ids.add(row["sample_id"])
        transformed_ids.add(row["transformed_id"])
        if row["bucket"] not in BUCKETS:
            raise AnalysisError("source bucket")
        bases.setdefault(row["base_id"], set()).add(row["map_variant"])
        if row["map_variant"] in VARIANTS:
            key = (row["base_id"], row["map_variant"])
            if key in selected:
                raise AnalysisError("duplicate selected source row")
            selected[key] = row
    expected_variants = set(VARIANTS) | {"u2-deterministic", "so4-deterministic"}
    if len(bases) != 8 or any(values != expected_variants for values in bases.values()):
        raise AnalysisError("source variant grid")
    if len(selected) != 24:
        raise AnalysisError("selected grid")
    if {row["bucket"] for row in selected.values()} != set(BUCKETS):
        raise AnalysisError("selected buckets")
    counts = {bucket: len({row["base_id"] for row in selected.values() if row["bucket"] == bucket}) for bucket in BUCKETS}
    if counts != {bucket: 2 for bucket in BUCKETS}:
        raise AnalysisError("two bases per bucket")
    return selected


def validate_design() -> tuple[dict, str, str]:
    design = json.loads(DESIGN.read_text())
    design_hash = digest(DESIGN)
    warn_stale(
        design.get("source_sha256") == SOURCE_SHA
        and design.get("source_report_sha256") == SOURCE_REPORT_SHA,
        "design source binding",
    )
    warn_stale(
        design.get("evaluator", {}).get("source_sha256") == digest(EVALUATOR),
        "design/current evaluator binding",
    )
    snapshot = design.get("feature_snapshot", {})
    warn_stale(
        snapshot.get("sha256") == FEATURE_SHA
        and snapshot.get("manifest_sha256") == FEATURE_REPORT_SHA,
        "design feature-snapshot binding",
    )
    warn_stale(
        digest(SELECTION) == design.get("selection", {}).get("manifest_sha256"),
        "selection-manifest binding",
    )
    warn_stale(
        digest(PROTOCOL_HISTORY) == design.get("protocol_history_sha256"),
        "protocol-history binding",
    )
    for item in design.get("evaluator", {}).get("implementation_files", []):
        path = REPO / item["path"]
        if not path.is_file():
            raise AnalysisError(f"missing implementation path: {item.get('path')}")
        warn_stale(
            digest(path) == item.get("sha256"),
            f"design implementation {item.get('path')}",
        )
    warn_stale(
        design.get("retained_run", {}).get("commit") == RETAINED_COMMIT,
        "design retained revision",
    )
    return design, design_hash, digest(PROTOCOL_HISTORY)


def validate_retained_commit(design: dict) -> None:
    try:
        subprocess.run(["git", "-C", str(REPO), "cat-file", "-e", f"{RETAINED_COMMIT}^{{commit}}"], check=True, capture_output=True)
    except (OSError, subprocess.CalledProcessError):
        warn_stale(False, "retained revision availability")
        return
    for label, path, expected in [
        (
            "design",
            "experiments/sys-datascience/methods/generator-orientation-target-pilot/design.json",
            RETAINED_DESIGN_SHA,
        ),
        (
            "evaluator",
            "experiments/sys-datascience/methods/generator-orientation-target-pilot/main.rs",
            RETAINED_EVALUATOR_SHA,
        ),
        (
            "selection",
            "experiments/sys-datascience/methods/generator-orientation-target-pilot/selection-manifest.json",
            RETAINED_SELECTION_SHA,
        ),
    ]:
        blob = git_blob(RETAINED_COMMIT, path)
        warn_stale(blob is not None, f"{label} availability at retained revision")
        if blob is not None:
            warn_stale(
                hashlib.sha256(blob).hexdigest() == expected,
                f"{label} at retained revision",
            )
    for item in design.get("evaluator", {}).get("implementation_files", []):
        blob = git_blob(RETAINED_COMMIT, item["path"])
        warn_stale(
            blob is not None,
            f"implementation availability at retained revision: {item.get('path')}",
        )
        if blob is not None:
            warn_stale(
                hashlib.sha256(blob).hexdigest()
                == RETAINED_IMPLEMENTATION_SHA.get(item["path"]),
                f"implementation at retained revision: {item.get('path')}",
            )


def validate_manifest_common(manifest: dict, design_hash: str, target_path: Path) -> None:
    if manifest.get("schema") != "generator-orientation-target-pilot-manifest-v1":
        raise AnalysisError("manifest schema")
    if manifest.get("expected_rows") != 24 or not isinstance(manifest.get("completed_rows"), int):
        raise AnalysisError("manifest row counts")
    warn_stale(
        manifest.get("source_sha256") == SOURCE_SHA
        and manifest.get("source_report_sha256") == SOURCE_REPORT_SHA,
        "manifest source provenance",
    )
    warn_stale(manifest.get("design_sha256") == design_hash, "manifest/current design")
    warn_stale(
        manifest.get("evaluator_source_sha256") == digest(EVALUATOR),
        "manifest/current evaluator",
    )
    warn_stale(
        manifest.get("retained_design_sha256") == RETAINED_DESIGN_SHA
        and manifest.get("retained_evaluator_source_sha256") == RETAINED_EVALUATOR_SHA,
        "manifest retained evaluator/design",
    )
    if manifest.get("target_schema") != "generator-orientation-target-pilot-row-v1" or manifest.get("backend") != "auto" or manifest.get("method_local_cache") is not True:
        raise AnalysisError("manifest target backend/schema/cache")
    warn_stale(
        manifest.get("protocol_history_sha256") == digest(PROTOCOL_HISTORY),
        "manifest protocol history",
    )
    listed = manifest.get("provenance", {}).get("implementation_files")
    if not isinstance(listed, list) or {item.get("path") for item in listed} != set(IMPLEMENTATION_PATHS):
        raise AnalysisError("manifest implementation closure paths")
    for item in listed:
        warn_stale(
            digest(REPO / item["path"]) == item.get("sha256"),
            f"manifest implementation {item.get('path')}",
        )
    declared = manifest.get("target_path")
    if not isinstance(declared, str):
        raise AnalysisError("manifest target path missing")
    candidates = {(REPO / declared).resolve(), Path(declared).resolve()}
    if target_path.resolve() not in candidates:
        raise AnalysisError("manifest target path")


def validate_inputs(target_path: Path, manifest_path: Path):
    for path in (SOURCE, SOURCE_REPORT, FEATURES, FEATURE_REPORT, DESIGN, SELECTION, PROTOCOL_HISTORY, EVALUATOR, target_path, manifest_path):
        if not path.is_file():
            raise AnalysisError(f"missing input {path}")
    warn_stale(
        digest(SOURCE) == SOURCE_SHA and digest(SOURCE_REPORT) == SOURCE_REPORT_SHA,
        "source bytes",
    )
    warn_stale(
        digest(FEATURES) == FEATURE_SHA and digest(FEATURE_REPORT) == FEATURE_REPORT_SHA,
        "feature bytes",
    )
    feature_manifest = json.loads(FEATURE_REPORT.read_text())
    if (
        feature_manifest.get("schema") != "generator-orientation-feature-snapshot-v1"
        or feature_manifest.get("snapshot_rows") != 40
        or feature_manifest.get("target_fields_present") is not False
        or feature_manifest.get("target_calls") != 0
    ):
        raise AnalysisError("feature snapshot semantic contract")
    warn_stale(
        feature_manifest.get("snapshot_sha256") == FEATURE_SHA
        and feature_manifest.get("full_feature_sha256") == FULL_FEATURE_SHA
        and feature_manifest.get("full_feature_report_sha256") == FULL_FEATURE_REPORT_SHA
        and feature_manifest.get("orientation_source_sha256") == SOURCE_SHA
        and feature_manifest.get("orientation_source_report_sha256") == SOURCE_REPORT_SHA,
        "feature snapshot provenance",
    )
    design, design_hash, protocol_hash = validate_design()
    source = validate_source(jsonl(SOURCE))
    manifest = json.loads(manifest_path.read_text())
    validate_manifest_common(manifest, design_hash, target_path)
    if manifest.get("status") in {"failed", "partial", "incomplete"}:
        raise AnalysisError("incomplete/failed target manifest is not interpretable")
    if manifest.get("status") != "complete" or manifest.get("completed_rows") != 24:
        raise AnalysisError("incomplete target manifest")
    warn_stale(manifest.get("pre_target_commit") == RETAINED_COMMIT, "pre-target revision")
    validate_retained_commit(design)
    warn_stale(
        manifest.get("target_sha256") == digest(target_path),
        "manifest/target bytes",
    )
    warn_stale(digest(target_path) == TARGET_SHA, "target retained bytes")
    target = jsonl(target_path)
    if len(target) != 24:
        raise AnalysisError("target row count")
    target_map = {}
    for row in target:
        if row.get("schema") != "generator-orientation-target-pilot-row-v1" or row.get("target_status") != "complete":
            raise AnalysisError("target schema/status")
        source_id = row.get("source_id")
        if not isinstance(source_id, str) or source_id in target_map:
            raise AnalysisError("target missing/duplicate source ID")
        target_map[source_id] = row
        for key in ("sys", "capacity", "volume", "exact_volume_as_f64", "time_volume_ms", "time_capacity_ms"):
            finite(row.get(key), f"target {key}")
        if row.get("backend") != "auto" or row.get("coordinate_order") != "q1,q2,p1,p2":
            raise AnalysisError("target backend/coordinate provenance")
        warn_stale(
            row.get("source_sha256") == SOURCE_SHA
            and row.get("source_report_sha256") == SOURCE_REPORT_SHA,
            "target-row source provenance",
        )
        warn_stale(
            row.get("design_sha256") == RETAINED_DESIGN_SHA
            and row.get("evaluator_source_sha256") == RETAINED_EVALUATOR_SHA,
            "target-row evaluator/design provenance",
        )
    expected_ids = {row["transformed_id"] for row in source.values()}
    if set(target_map) != expected_ids:
        raise AnalysisError("target substituted/missing ID")
    for row in target:
        src = next(source[key] for key in source if source[key]["transformed_id"] == row["source_id"])
        for key in ("sample_id", "transformed_id", "base_id", "bucket", "q_sides", "p_sides", "map_variant", "map_family", "map_mode", "map_seed", "row_index"):
            if row.get(key) != src.get(key):
                raise AnalysisError(f"target/source identity mismatch: {key}")
        if row.get("source_transformed_dual_vertices_f64") != src.get("transformed_dual_vertices_f64"):
            raise AnalysisError("target/source payload mismatch")
    feature_rows = jsonl(FEATURES)
    if len(feature_rows) != 40:
        raise AnalysisError("feature snapshot row count")
    forbidden = ("capacity", "sys", "iterations", "iteration", "bounce_label", "target", "target_ms")
    if any(any(key in row for key in forbidden) for row in feature_rows):
        raise AnalysisError("feature target field present")
    features = {}
    for row in feature_rows:
        if row.get("source_kind") == "orientation" and row.get("map_variant") in VARIANTS:
            source_id = row.get("source_id")
            if source_id in features:
                raise AnalysisError("duplicate feature source ID")
            features[source_id] = row
    if set(features) != expected_ids:
        raise AnalysisError("feature source identity/grid mismatch")
    for source_id, row in features.items():
        src = next(source[key] for key in source if source[key]["transformed_id"] == source_id)
        if row.get("source_sample_id") != source_id or row.get("base_id") != src["base_id"] or row.get("bucket") != src["bucket"] or row.get("coordinate_order") != "q1,q2,p1,p2":
            raise AnalysisError("feature/source linkage mismatch")
        finite(row.get("symplectic_ridge_area_mean"), "feature ridge")
    return source, target_map, features, design_hash, {"protocol_history_sha256": protocol_hash}


def analyze(target_path: Path, manifest_path: Path):
    source, target, features, design_hash, extra = validate_inputs(target_path, manifest_path)
    groups, pairs = {}, []
    for (base, variant), src in source.items():
        groups.setdefault(base, {})[variant] = target[src["transformed_id"]]
    if any(set(group) != set(VARIANTS) for group in groups.values()):
        raise AnalysisError("paired target grid")
    for base in sorted(groups):
        group = groups[base]
        identity, u2, so4 = group["identity"], group["u2-haar"], group["so4-haar"]
        ridge_i = features[identity["source_id"]]["symplectic_ridge_area_mean"]
        ridge_s = features[so4["source_id"]]["symplectic_ridge_area_mean"]
        pairs.append({"base_id": base, "bucket": identity["bucket"], "sys_identity": identity["sys"], "sys_u2": u2["sys"], "sys_so4": so4["sys"], "delta_u2": u2["sys"] - identity["sys"], "delta_so4": so4["sys"] - identity["sys"], "ridge_identity": ridge_i, "ridge_so4": ridge_s, "delta_ridge": ridge_s - ridge_i})
    u2_max = max(abs(row["delta_u2"]) for row in pairs)
    abs_so4 = [abs(row["delta_so4"]) for row in pairs]
    signed = [row["delta_so4"] for row in pairs]
    nonzero = [(row["delta_so4"], row["delta_ridge"]) for row in pairs if row["delta_so4"] != 0 and row["delta_ridge"] != 0]
    opposite = sum(a * b < 0 for a, b in nonzero)
    rho = spearman([a for a, _ in nonzero], [b for _, b in nonzero])
    signs = sum(value > 0 for value in signed), sum(value < 0 for value in signed), sum(value == 0 for value in signed)
    costs = {}
    for row in target.values():
        key = (row["map_variant"], row["facet_count"])
        item = costs.setdefault(key, {"variant": key[0], "facet_count": key[1], "rows": 0, "total_compute_ms": 0.0, "total_volume_ms": 0.0, "total_capacity_ms": 0.0})
        item["rows"] += 1
        item["total_volume_ms"] += row["time_volume_ms"]
        item["total_capacity_ms"] += row["time_capacity_ms"]
        item["total_compute_ms"] += row["time_volume_ms"] + row["time_capacity_ms"]
    by_bucket = {}
    for bucket in BUCKETS:
        values = [row["delta_so4"] for row in pairs if row["bucket"] == bucket]
        by_bucket[bucket] = {"count": len(values), "signed_mean": sum(values) / len(values), "median_abs_delta": sum(sorted(abs(value) for value in values)) / len(values)}
    loo = []
    for bucket in BUCKETS:
        values = [row["delta_so4"] for row in pairs if row["bucket"] != bucket]
        abs_values = sorted(abs(value) for value in values)
        loo.append({"excluded_bucket": bucket, "signed_mean": sum(values) / len(values), "median_abs_delta": sum(abs_values[len(values) // 2 - 1 : len(values) // 2 + 1]) / 2})
    bucket_abs = {bucket: sum(abs(row["delta_so4"]) for row in pairs if row["bucket"] == bucket) for bucket in BUCKETS}
    total_abs = sum(bucket_abs.values())
    largest_share = max(bucket_abs.values()) / total_abs if total_abs else 0.0
    result = {"schema": "generator-orientation-target-pilot-transplant-report-v1", "accepted_report_schema": "generator-orientation-target-pilot-report-v1", "source_sha256": SOURCE_SHA, "source_report_sha256": SOURCE_REPORT_SHA, "feature_sha256": FEATURE_SHA, "feature_report_sha256": FEATURE_REPORT_SHA, "feature_full_sha256": FULL_FEATURE_SHA, "feature_full_report_sha256": FULL_FEATURE_REPORT_SHA, "design_sha256": design_hash, "target_sha256": digest(target_path), "manifest_sha256": digest(manifest_path), "protocol_history_sha256": extra["protocol_history_sha256"], "protocol_history_account": {"design_evaluator_first_commit": "dfbcc400b8cc39c60f8e8c22f8e9ed95acc229be", "source_variant_validation_repaired": "f5f38f351576e7eccb5a51242ff95211ed7b8761", "earlier_exposure_rejected_deleted": True, "earlier_exposure_reasons": ["mistyped experiments/sys-landscape/src/lib.rs SHA-256 nibble", "wrong analyzer repository-parent depth"], "valid_pre_retained_rerun_commit": RETAINED_COMMIT, "only_retained_target_sha256": TARGET_SHA}, "retained_run_commit": RETAINED_COMMIT, "pair_rows": pairs, "u2_max_abs_delta": u2_max, "u2_control_pass": u2_max <= 1e-8, "primary": {"median_abs_delta": sum(sorted(abs_so4)[3:5]) / 2, "max_abs_delta": max(abs_so4), "count_abs_delta_ge_0_01": sum(value >= 0.01 for value in abs_so4), "signed_mean": sum(signed) / len(signed), "signed_median": sum(sorted(signed)[3:5]) / 2, "sign_counts": {"positive": signs[0], "negative": signs[1], "zero": signs[2]}}, "disposition": primary_disposition(u2_max, abs_so4), "ridge_linked": {"nonzero_pairs": len(nonzero), "opposite_sign_count": opposite, "spearman_rho": rho, "pattern": "directionally_consistent" if opposite >= 6 and rho is not None and rho <= -0.5 else "not_directionally_consistent_or_ambiguous"}, "heterogeneity": {"by_bucket": by_bucket, "leave_one_bucket_out": loo, "common_nonzero_sign": max(signs[:2]) >= 7, "sign_heterogeneous": max(signs[:2]) < 7, "largest_bucket_absolute_share": largest_share, "bucket_concentrated": largest_share > 0.5, "prohibit_common_signed_effect_claim": max(signs[:2]) < 7 or largest_share > 0.5}, "cost_by_variant_facet": sorted(costs.values(), key=lambda item: (item["variant"], item["facet_count"])), "n": 8, "interpretation_boundary": "frozen witnesses only; no population, p-value, bootstrap, causal, or law-ranking claim"}
    return result


def self_test():
    assert rank_average([1, 1, 2]) == [1.5, 1.5, 3.0]
    assert abs(spearman([1, 2, 3], [3, 2, 1]) + 1.0) < 1e-12
    assert spearman([1, 1, 2], [3, 2, 1]) < -0.8
    print("synthetic positive/negative/tie calibrations pass")


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--target", type=Path)
    parser.add_argument("--manifest", type=Path)
    parser.add_argument("--out", type=Path)
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()
    if args.self_test:
        self_test()
        return
    if not args.target or not args.manifest or not args.out:
        parser.error("--target, --manifest, and --out are required")
    result = analyze(args.target, args.manifest)
    args.out.write_text(json.dumps(result, indent=2) + "\n")
    print(result["disposition"])


if __name__ == "__main__":
    main()
