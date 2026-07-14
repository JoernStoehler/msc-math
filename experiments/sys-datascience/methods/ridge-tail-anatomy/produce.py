#!/usr/bin/env python3
"""Snapshot the retained rows needed by the ridge-tail anatomy analyzer.

The product covariance validation keeps large geometry/feature caches outside
the repository.  This producer extracts only the frozen 5x5 rows used by this
packet, while preserving source hashes and the frozen arm memberships.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import shutil
from pathlib import Path


def sha256(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as f:
        for chunk in iter(lambda: f.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


def read_jsonl(path: Path) -> list[dict]:
    return [json.loads(line) for line in path.read_text().splitlines() if line.strip()]


def write_jsonl(path: Path, rows: list[dict]) -> None:
    path.write_text("".join(json.dumps(row, sort_keys=True, separators=(",", ":")) + "\n" for row in rows))


def snapshot_generic(geometry_path: Path, target_path: Path) -> list[dict]:
    geometry = {row["candidate_id"]: row for row in read_jsonl(geometry_path)}
    targets = {row["candidate_id"]: row for row in read_jsonl(target_path)}
    if set(geometry) != set(targets) or len(geometry) != 200:
        raise ValueError("generic geometry/target join is not exactly 200 rows")
    out = []
    for cid in sorted(geometry):
        g, t = geometry[cid], targets[cid]
        if g["poly_id"] != t["poly_id"]:
            raise ValueError(f"generic poly_id mismatch for {cid}")
        out.append({
            "candidate_id": cid,
            "poly_id": g["poly_id"],
            "role": t["role"],
            "future_band": t["future_band"],
            "f64_rank": t["f64_rank"],
            "facet_count": g["facet_count"],
            "f64_volume": t["f64_volume"],
            "sys": t["sys"],
            "proxy": t["proxy"],
            "ridge_count_source": t["ridge_count"],
            "dual_vertices_rational": g["dual_vertices_rational"],
            "vertices_rational": g["vertices_rational"],
        })
    return out


def snapshot_product(
    pre_paths: list[Path], eval_paths: list[Path], geometry_paths: list[Path], feature_paths: list[Path]
) -> list[dict]:
    pre = {}
    for path in pre_paths:
        for row in read_jsonl(path):
            if row["bucket_id"] == "random-product:5x5:h0p8_1p2":
                pre[row["candidate_id"]] = row
    evaluations = {}
    for path in eval_paths:
        for row in read_jsonl(path):
            if row["bucket_id"] == "random-product:5x5:h0p8_1p2":
                if row["candidate_id"] in evaluations:
                    raise ValueError(f"duplicate product evaluation {row['candidate_id']}")
                evaluations[row["candidate_id"]] = row
    geometry = {}
    for path in geometry_paths:
        for row in read_jsonl(path):
            if row["bucket_id"] == "random-product:5x5:h0p8_1p2":
                geometry[row["candidate_id"]] = row
    features = {}
    for path in feature_paths:
        for row in read_jsonl(path):
            if row["bucket_id"] == "random-product:5x5:h0p8_1p2":
                features[row["candidate_id"]] = row
    ids = set(pre) & set(evaluations)
    if len(ids) != len(pre) or not ids <= set(geometry) or not ids <= set(features):
        raise ValueError(f"product 5x5 join mismatch: pre={len(pre)} eval={len(evaluations)} geometry={len(geometry)} features={len(features)} joined={len(ids)}")
    if len(ids) != 142:
        raise ValueError(f"expected 142 frozen 5x5 rows, got {len(ids)}")
    out = []
    for cid in sorted(ids):
        p, e, g, f = pre[cid], evaluations[cid], geometry[cid], features[cid]
        for field in ("poly_id", "bucket_id"):
            if p.get(field) != e.get(field) or p.get(field) != g.get(field):
                raise ValueError(f"product {field} mismatch for {cid}")
        if p.get("selection_ids") != e.get("selection_ids"):
            raise ValueError(f"product selection_ids mismatch for {cid}")
        out.append({
            "candidate_id": cid,
            "poly_id": p["poly_id"],
            "seed": p["source"]["seed"],
            "sample_index": p["source"]["sample_index"],
            "arm_memberships": sorted(p["selection_ids"]),
            "sys": e["sys"],
            "capacity": e["capacity"],
            "f64_volume": g["volume"],
            "facet_count": g["facet_count"],
            "product_k": g["product_k"],
            "product_m": g["product_m"],
            "dual_vertices": g["dual_vertices"],
            "vertices": g["vertices"],
            "vertex_facet_incidence": g["vertex_facet_incidence"],
            "vertex_count": g["vertex_count"],
            "ridge_count_source": g["ridge_count"],
            "rho": f["vertex_covariance_rho"],
            "ridge_mean_feature": f["ridge_symp_area_mean_over_volume_sqrt"],
            "ridge_sum_feature": f["ridge_symp_area_sum_over_volume_sqrt"],
        })
    return out


def main() -> None:
    p = argparse.ArgumentParser()
    p.add_argument("--generic-geometry", type=Path, required=True)
    p.add_argument("--generic-target", type=Path, required=True)
    p.add_argument("--product-pre", type=Path, action="append", required=True)
    p.add_argument("--product-eval", type=Path, action="append", required=True)
    p.add_argument("--product-geometry", type=Path, action="append", required=True)
    p.add_argument("--product-features", type=Path, action="append", required=True)
    p.add_argument("--out-dir", type=Path, required=True)
    args = p.parse_args()
    out = args.out_dir
    out.mkdir(parents=True, exist_ok=True)
    if any(out.iterdir()):
        raise SystemExit(f"refusing to overwrite nonempty output directory: {out}")
    generic = snapshot_generic(args.generic_geometry, args.generic_target)
    product = snapshot_product(args.product_pre, args.product_eval, args.product_geometry, args.product_features)
    write_jsonl(out / "generic-input.jsonl", generic)
    write_jsonl(out / "product-5x5-input.jsonl", product)
    sources = [args.generic_geometry, args.generic_target, *args.product_pre, *args.product_eval, *args.product_geometry, *args.product_features]
    provenance = {
        "schema": "sys-datascience.ridge-tail-anatomy.input-provenance.v1",
        "producer": "produce.py",
        "command": " ".join(__import__("sys").argv),
        "inputs": [{"path": str(path), "sha256": sha256(path), "bytes": path.stat().st_size} for path in sources],
        "rows": {"generic": len(generic), "product_5x5": len(product)},
        "product_selection_contract": "two frozen covariance validation seeds; 25 rho, 25 ridge, and 25 disjoint-control memberships per bucket; 5x5 rows only",
        "target_timing": "generic and product sys values were evaluated before this retained-data anatomy analysis; no new target call",
    }
    (out / "provenance.json").write_text(json.dumps(provenance, indent=2, sort_keys=True) + "\n")
    print(json.dumps(provenance["rows"], sort_keys=True))


if __name__ == "__main__":
    main()
