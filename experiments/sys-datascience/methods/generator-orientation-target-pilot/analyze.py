#!/usr/bin/env python3
"""Validate and analyze the frozen 8-base x 3-variant orientation pilot."""
from __future__ import annotations
import argparse, hashlib, json, math, subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parent
SOURCE = ROOT.parent / "generator-orientation-smoke/artifacts/panel-2-per-bucket/rows.jsonl"
SOURCE_REPORT = SOURCE.with_name("report.json")
FEATURES = ROOT.parent / "generator-exact-feature-augmenter/artifacts/full-panels/features.jsonl"
FEATURE_REPORT = FEATURES.with_name("report.json")
DESIGN = ROOT / "design.json"
SOURCE_SHA = "b5ded0a5e83d41f35ca035660d222326a161ce5001fd18c12f74f0ed9f3bc367"
SOURCE_REPORT_SHA = "02b7084141c0f2422aaabf1516fa62af501963ce638b9df3ef756c762722d61c"
FEATURE_SHA = "e7cc585b2e774bc6ee5dcd658e49b02cefd7cdd914fb1ffaba759ccb64d6b624"
FEATURE_REPORT_SHA = "4982846e2a8828ba2e217b7b017605180927b2e040f96818d9eac9a405477e43"
VARIANTS = ("identity", "u2-haar", "so4-haar")
BUCKETS = ("3x3", "4x4", "4x6", "6x6")

class AnalysisError(Exception): pass

def digest(path: Path) -> str: return hashlib.sha256(path.read_bytes()).hexdigest()
def jsonl(path: Path): return [json.loads(line) for line in path.read_text().splitlines() if line.strip()]
def finite(x, label):
    if isinstance(x, bool) or not isinstance(x, (int, float)) or not math.isfinite(float(x)): raise AnalysisError(f"{label} nonfinite")

def rank_average(xs):
    order = sorted(range(len(xs)), key=lambda i: xs[i]); ranks = [0.0] * len(xs); i = 0
    while i < len(xs):
        j = i + 1
        while j < len(xs) and xs[order[j]] == xs[order[i]]: j += 1
        rank = (i + 1 + j) / 2.0
        for k in range(i, j): ranks[order[k]] = rank
        i = j
    return ranks

def spearman(xs, ys):
    if len(xs) != len(ys) or not xs: return None
    rx, ry = rank_average(xs), rank_average(ys)
    mx, my = sum(rx)/len(rx), sum(ry)/len(ry)
    a = sum((x-mx)*(y-my) for x,y in zip(rx,ry)); bx = math.sqrt(sum((x-mx)**2 for x in rx)); by = math.sqrt(sum((y-my)**2 for y in ry))
    return a/(bx*by) if bx and by else None

def validate_source(rows):
    if len(rows) != 40: raise AnalysisError("source row count is not 40")
    selected = {}
    bases = {}
    ids = set()
    for r in rows:
        if r.get("schema") != "generator-orientation-smoke-row-v2": raise AnalysisError("source schema")
        for key in ("capacity", "sys", "target", "iterations", "bounce_label"):
            if r.get(key) is not None: raise AnalysisError(f"source target field {key}")
        if r.get("base_accepted") is not True or r.get("semantic_invariants_passed") is not True or r.get("reconstruction_status") != "reconstructed" or r.get("map_status") != "generated" or r.get("invariant_failures") != []: raise AnalysisError("source status")
        if not isinstance(r.get("sample_id"), str) or not isinstance(r.get("transformed_id"), str): raise AnalysisError("source ID type")
        if r["transformed_id"] in ids: raise AnalysisError("source duplicate transformed ID")
        ids.add(r["transformed_id"])
        if r.get("bucket") not in BUCKETS: raise AnalysisError("source bucket")
        base, variant = r.get("base_id"), r.get("map_variant")
        bases.setdefault(base, set()).add(variant)
        if variant in VARIANTS: selected[(base, variant)] = r
    if len(bases) != 8 or set(bases) != {r["base_id"] for r in rows if r["bucket"] in BUCKETS}: raise AnalysisError("source bases")
    if any(v != set(("identity", "u2-haar", "so4-haar", "u2-deterministic", "so4-deterministic")) for v in bases.values()): raise AnalysisError("source variant grid")
    if len(selected) != 24: raise AnalysisError("selected grid")
    if {r["bucket"] for r in selected.values()} != set(BUCKETS): raise AnalysisError("selected buckets")
    if {b: len({r["base_id"] for r in selected.values() if r["bucket"] == b}) for b in BUCKETS} != {b: 2 for b in BUCKETS}: raise AnalysisError("two bases per bucket")
    return selected

def validate_inputs(target_path: Path, manifest_path: Path):
    for path in (SOURCE, SOURCE_REPORT, FEATURES, FEATURE_REPORT, DESIGN, target_path, manifest_path):
        if not path.is_file(): raise AnalysisError(f"missing input {path}")
    if digest(SOURCE) != SOURCE_SHA or digest(SOURCE_REPORT) != SOURCE_REPORT_SHA: raise AnalysisError("source hash mismatch")
    if digest(FEATURES) != FEATURE_SHA or digest(FEATURE_REPORT) != FEATURE_REPORT_SHA: raise AnalysisError("feature hash mismatch")
    design = json.loads(DESIGN.read_text()); design_hash = digest(DESIGN)
    if design.get("source_sha256") != SOURCE_SHA or design.get("source_report_sha256") != SOURCE_REPORT_SHA: raise AnalysisError("design source binding")
    if design.get("evaluator", {}).get("source_sha256") != digest(ROOT / "main.rs"): raise AnalysisError("design evaluator source binding")
    for item in design.get("evaluator", {}).get("implementation_files", []):
        path = Path(__file__).resolve().parents[3] / item["path"]
        if not path.is_file() or digest(path) != item["sha256"]: raise AnalysisError(f"design implementation closure mismatch: {item['path']}")
    selection_manifest = ROOT / "selection-manifest.json"
    if digest(selection_manifest) != design.get("selection", {}).get("manifest_sha256"): raise AnalysisError("selection manifest binding")
    target = jsonl(target_path); manifest = json.loads(manifest_path.read_text())
    if manifest.get("status") != "complete" or manifest.get("completed_rows") != 24 or manifest.get("expected_rows") != 24: raise AnalysisError("incomplete target manifest")
    if manifest.get("target_path") and Path(manifest["target_path"]).resolve() != target_path.resolve(): raise AnalysisError("manifest target path")
    if manifest.get("source_sha256") != SOURCE_SHA or manifest.get("source_report_sha256") != SOURCE_REPORT_SHA or manifest.get("design_sha256") != design_hash: raise AnalysisError("manifest provenance")
    if len(target) != 24 or any(r.get("schema") != "generator-orientation-target-pilot-row-v1" or r.get("target_status") != "complete" for r in target): raise AnalysisError("target schema/status")
    source = validate_source(jsonl(SOURCE))
    tmap = {r.get("source_id"): r for r in target}
    if len(tmap) != 24 or set(tmap) != {r["transformed_id"] for r in source.values()}: raise AnalysisError("target IDs/grid")
    for r in target:
        finite(r.get("sys"), "sys"); finite(r.get("capacity"), "capacity"); finite(r.get("volume"), "volume")
        if r.get("backend") != "auto" or r.get("source_sha256") != SOURCE_SHA or r.get("source_report_sha256") != SOURCE_REPORT_SHA or r.get("design_sha256") != design_hash: raise AnalysisError("target provenance/backend")
        if r.get("coordinate_order") != "q1,q2,p1,p2": raise AnalysisError("target coordinate order")
        src = next(x for x in source.values() if x["transformed_id"] == r["source_id"])
        for key in ("base_id", "bucket", "map_variant", "sample_id", "transformed_id"):
            if r.get(key) != src.get(key): raise AnalysisError(f"target/source mismatch {key}")
    features = {r.get("source_sample_id"): r for r in jsonl(FEATURES) if r.get("source_kind") == "orientation" and r.get("map_variant") in VARIANTS}
    if len(features) < 24: raise AnalysisError("feature grid absent")
    return source, tmap, features, design_hash

def analyze(target_path: Path, manifest_path: Path):
    source, target, features, design_hash = validate_inputs(target_path, manifest_path)
    groups = {}
    pairs = []
    for (base, variant), src in source.items():
        t = target[src["transformed_id"]]; groups.setdefault(base, {})[variant] = t
    for base in sorted(groups):
        g = groups[base]; ident = g["identity"]; u2, so4 = g["u2-haar"], g["so4-haar"]
        ridge_i = features[ident["transformed_id"]]["symplectic_ridge_area_mean"]
        ridge_s = features[so4["transformed_id"]]["symplectic_ridge_area_mean"]
        dr = ridge_s - ridge_i; du = u2["sys"] - ident["sys"]; ds = so4["sys"] - ident["sys"]
        pairs.append({"base_id": base, "bucket": ident["bucket"], "sys_identity": ident["sys"], "sys_u2": u2["sys"], "sys_so4": so4["sys"], "delta_u2": du, "delta_so4": ds, "ridge_identity": ridge_i, "ridge_so4": ridge_s, "delta_ridge": dr})
    u2_max = max(abs(x["delta_u2"]) for x in pairs); abs_so4 = [abs(x["delta_so4"]) for x in pairs]; signed = [x["delta_so4"] for x in pairs]
    nonzero = [(x["delta_so4"], x["delta_ridge"]) for x in pairs if x["delta_so4"] != 0 and x["delta_ridge"] != 0]
    opposite = sum(a*b < 0 for a,b in nonzero); rho = spearman([a for a,b in nonzero], [b for a,b in nonzero])
    material = sum(v >= 0.01 for v in abs_so4); maximum = max(abs_so4); median = sorted(abs_so4)[len(abs_so4)//2-1:len(abs_so4)//2+1]; median = sum(median)/2
    if u2_max > 1e-8: disposition = "ambiguous_numerical_control_failure"
    elif material >= 6: disposition = "supports_material_alignment_role"
    elif maximum < 0.005: disposition = "contradicts_material_role_on_frozen_maps"
    else: disposition = "ambiguous"
    by_bucket = {}
    for b in BUCKETS:
        xs = [x["delta_so4"] for x in pairs if x["bucket"] == b]
        by_bucket[b] = {"count": len(xs), "signed_mean": sum(xs)/len(xs), "median_abs_delta": sum(sorted(abs(v) for v in xs))/len(xs)}
    loo = []
    for b in BUCKETS:
        xs = [x["delta_so4"] for x in pairs if x["bucket"] != b]; loo.append({"excluded_bucket": b, "signed_mean": sum(xs)/len(xs), "median_abs_delta": sum(sorted(abs(v) for v in xs)[len(xs)//2-1:len(xs)//2+1])/2})
    bucket_abs = {b: sum(abs(x["delta_so4"]) for x in pairs if x["bucket"] == b) for b in BUCKETS}; total = sum(bucket_abs.values()); largest = max(bucket_abs.values())/total if total else 0.0
    signs = sum(v > 0 for v in signed), sum(v < 0 for v in signed), sum(v == 0 for v in signed)
    result = {"schema": "generator-orientation-target-pilot-report-v1", "source_sha256": SOURCE_SHA, "source_report_sha256": SOURCE_REPORT_SHA, "feature_sha256": FEATURE_SHA, "feature_report_sha256": FEATURE_REPORT_SHA, "design_sha256": design_hash, "pair_rows": pairs, "u2_max_abs_delta": u2_max, "u2_control_pass": u2_max <= 1e-8, "primary": {"median_abs_delta": median, "max_abs_delta": maximum, "count_abs_delta_ge_0_01": material, "signed_mean": sum(signed)/len(signed), "signed_median": sum(sorted(signed)[3:5])/2, "sign_counts": {"positive": signs[0], "negative": signs[1], "zero": signs[2]}}, "disposition": disposition, "ridge_linked": {"nonzero_pairs": len(nonzero), "opposite_sign_count": opposite, "spearman_rho": rho, "pattern": "directionally_consistent" if opposite >= 6 and rho is not None and rho <= -0.5 else "not_directionally_consistent_or_ambiguous"}, "heterogeneity": {"by_bucket": by_bucket, "leave_one_bucket_out": loo, "common_nonzero_sign": max(signs[:2]) >= 7, "sign_heterogeneous": max(signs[:2]) < 7, "largest_bucket_absolute_share": largest, "bucket_concentrated": largest > 0.5, "prohibit_common_signed_effect_claim": max(signs[:2]) < 7 or largest > 0.5}, "n": 8, "interpretation_boundary": "frozen witnesses only; no population, p-value, bootstrap, causal, or law-ranking claim"}
    return result

def self_test():
    assert rank_average([1, 1, 2]) == [1.5, 1.5, 3.0]
    assert abs(spearman([1, 2, 3], [3, 2, 1]) + 1.0) < 1e-12
    assert spearman([1, 1, 2], [3, 2, 1]) < -0.8
    print("synthetic positive/negative/tie calibrations pass")

def main():
    p = argparse.ArgumentParser(); p.add_argument("--target", type=Path); p.add_argument("--manifest", type=Path); p.add_argument("--out", type=Path); p.add_argument("--self-test", action="store_true"); a = p.parse_args()
    if a.self_test: self_test(); return
    if not a.target or not a.manifest or not a.out: p.error("--target, --manifest, and --out are required")
    result = analyze(a.target, a.manifest); a.out.write_text(json.dumps(result, indent=2) + "\n"); print(result["disposition"])

if __name__ == "__main__": main()
