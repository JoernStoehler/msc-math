#!/usr/bin/env python3
"""Deterministic retained-data anatomy analysis for ridge-tail step 2."""

from __future__ import annotations

import argparse
import hashlib
import json
import math
from collections import defaultdict
from fractions import Fraction
from pathlib import Path

OMEGA_EPS = 1e-12
AREA_EPS = 1e-12
IDENTITY_TOL = 2e-10
RHO_ID = "frozen_low_vertex_covariance_rho_bottom_0p005"
RIDGE_ID = "frozen_ridge_bottom_0p01_then_bottom_0p5"
CONTROL_ID = "frozen_shared_disjoint_control_25_per_bucket"


def read_jsonl(path: Path) -> list[dict]:
    return [json.loads(line) for line in path.read_text().splitlines() if line.strip()]


def sha256(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as f:
        for chunk in iter(lambda: f.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


def dot(a, b):
    return sum(x * y for x, y in zip(a, b))


def sub(a, b):
    return [x - y for x, y in zip(a, b)]


def norm(a):
    return math.sqrt(dot(a, a))


def omega(a, b):
    return a[0] * b[2] - a[2] * b[0] + a[1] * b[3] - a[3] * b[1]


def order_face(incidence: list[list[bool]], facets: tuple[int, int], vertices: list[int]) -> list[int] | None:
    if len(vertices) < 3:
        return None
    neigh = [[] for _ in vertices]
    for i, left in enumerate(vertices):
        for j in range(i + 1, len(vertices)):
            right = vertices[j]
            shared = any(
                k not in facets and incidence[left][k] and incidence[right][k]
                for k in range(len(incidence[0]))
            )
            if shared:
                neigh[i].append(j)
                neigh[j].append(i)
    if any(len(x) != 2 for x in neigh):
        return None
    order = [0]
    previous, current = 0, neigh[0][0]
    while current != 0:
        if current in order:
            return None
        order.append(current)
        choices = neigh[current]
        nxt = choices[1] if choices[0] == previous else choices[0]
        previous, current = current, nxt
    return [vertices[i] for i in order] if len(order) == len(vertices) else None


def face_pairs(incidence: list[list[bool]]) -> list[tuple[tuple[int, int], list[int]]]:
    f = len(incidence[0])
    out = []
    for i in range(f):
        for j in range(i + 1, f):
            vertices = [v for v, row in enumerate(incidence) if row[i] and row[j]]
            if len(vertices) >= 3:
                out.append(((i, j), vertices))
    return out


def euclidean_area(points: list[list[float]]) -> float:
    if len(points) < 3:
        return 0.0
    origin = points[0]
    u = None
    for point in points[1:]:
        e = sub(point, origin)
        if norm(e) > AREA_EPS:
            u = [x / norm(e) for x in e]
            break
    if u is None:
        return 0.0
    v = None
    for point in points[1:]:
        e = sub(point, origin)
        proj = dot(e, u)
        w = [e_i - proj * u_i for e_i, u_i in zip(e, u)]
        if norm(w) > AREA_EPS:
            v = [x / norm(w) for x in w]
            break
    if v is None:
        return 0.0
    coords = [(dot(sub(point, origin), u), dot(sub(point, origin), v)) for point in points]
    return 0.5 * abs(sum(x * coords[(i + 1) % len(coords)][1] - coords[(i + 1) % len(coords)][0] * y for i, (x, y) in enumerate(coords)))


def quantile(values: list[float], p: float) -> float:
    if not values:
        return 0.0
    x = p * (len(values) - 1)
    lo, hi = int(x), min(int(x) + 1, len(values) - 1)
    return values[lo] + (values[hi] - values[lo]) * (x - lo)


def concentration(values: list[float]) -> tuple[float, float]:
    total = sum(values)
    if total <= 0:
        return 0.0, 0.0
    probs = [x / total for x in values if x > 0]
    entropy = -sum(p * math.log(p) for p in probs)
    return entropy, math.exp(entropy)


def classify_factor(normal: list[float]) -> str:
    q = norm(normal[:2])
    p = norm(normal[2:])
    if q > 1e-10 and p <= 1e-10:
        return "q_factor"
    if p > 1e-10 and q <= 1e-10:
        return "p_factor"
    return "unknown"


def geometry_summary(row: dict, product: bool) -> tuple[dict, list[dict]]:
    if product:
        vertices = [[float(x) for x in v] for v in row["vertices"]]
        incidence = [[bool(x) for x in v] for v in row["vertex_facet_incidence"]]
        dual = [[float(x) for x in v] for v in row["dual_vertices"]]
        volume = float(row["f64_volume"])
    else:
        cv = lambda x: float(Fraction(x))
        vertices = [[cv(x) for x in v] for v in row["vertices_rational"]]
        dual = [[cv(x) for x in v] for v in row["dual_vertices_rational"]]
        volume = float(row["f64_volume"])
        incidence = [[abs(dot(v, n) - 1.0) <= 2e-10 for n in dual] for v in vertices]
    faces = []
    for facets, raw_vertices in face_pairs(incidence):
        order = order_face(incidence, facets, raw_vertices)
        if order is None:
            faces.append({"facets": list(facets), "ordered": False, "degenerate": True, "euclidean_area": 0.0, "symplectic_area": 0.0, "kappa": 0.0, "kind": "unclassified"})
            continue
        points = [vertices[i] for i in order]
        euclidean = euclidean_area(points)
        symp = 0.5 * abs(sum(omega(points[i], points[(i + 1) % len(points)]) for i in range(len(points))))
        degenerate = euclidean <= AREA_EPS or not math.isfinite(euclidean) or not math.isfinite(symp)
        kappa = 0.0 if degenerate else symp / euclidean
        translated = [[x + t for x, t in zip(point, (0.37, -0.19, 0.23, -0.41))] for point in points]
        scaled = [[1.7 * x for x in point] for point in points]
        translated_e = euclidean_area(translated)
        translated_a = 0.5 * abs(sum(omega(translated[i], translated[(i + 1) % len(translated)]) for i in range(len(translated))))
        scaled_e = euclidean_area(scaled)
        scaled_a = 0.5 * abs(sum(omega(scaled[i], scaled[(i + 1) % len(scaled)]) for i in range(len(scaled))))
        translation_error = max(abs(translated_e - euclidean), abs(translated_a - symp))
        scale_error = max(abs(scaled_e / (1.7**2) - euclidean), abs(scaled_a / (1.7**2) - symp))
        kind = "unknown"
        if product:
            fi, fj = classify_factor(dual[facets[0]]), classify_factor(dual[facets[1]])
            kind = "structural_zero" if fi == fj and fi != "unknown" else "mixed" if {fi, fj} == {"q_factor", "p_factor"} else "unknown"
        faces.append({"facets": list(facets), "vertices": order, "ordered": True, "degenerate": degenerate, "euclidean_area": euclidean, "symplectic_area": symp, "kappa": kappa, "kind": kind, "translation_invariance_abs_error": translation_error, "positive_scaling_invariance_abs_error": scale_error})
    eligible = [f for f in faces if f["ordered"] and not f["degenerate"]]
    e = [f["euclidean_area"] for f in eligible]
    a = [f["symplectic_area"] for f in eligible]
    k = [f["kappa"] for f in eligible]
    e_sum, a_sum = sum(e), sum(a)
    mean_e = e_sum / len(e) if e else 0.0
    mean_a = a_sum / len(a) if a else 0.0
    weighted_kappa = sum(ei * ki for ei, ki in zip(e, k)) / e_sum if e_sum > 0 else 0.0
    cov = (sum((ei - mean_e) * (ki - sum(k) / len(k)) for ei, ki in zip(e, k)) / len(e)) if e else 0.0
    entropy, effective = concentration(a)
    k_sorted = sorted(k)
    k_top = sorted(k, reverse=True)
    k_total = sum(k)
    a_sorted = sorted(a, reverse=True)
    a_total = sum(a)
    summary = {
        "candidate_id": row["candidate_id"], "poly_id": row["poly_id"], "product": product,
        "seed": row.get("seed"), "role": row.get("role"), "future_band": row.get("future_band"),
        "f64_rank": row.get("f64_rank"), "arm_memberships": row.get("arm_memberships", []),
        "sys": float(row["sys"]), "f64_volume": volume, "facet_count": len(dual), "vertex_count": len(vertices),
        "ridge_count_source": row.get("ridge_count_source"), "face_count": len(faces), "eligible_face_count": len(eligible),
        "degenerate_face_count": sum(f["degenerate"] for f in faces), "ordered_face_count": sum(f["ordered"] for f in faces),
        "mixed_face_count": sum(f["kind"] == "mixed" for f in eligible), "structural_zero_face_count": sum(f["kind"] == "structural_zero" for f in eligible),
        "unknown_kind_face_count": sum(f["kind"] == "unknown" for f in eligible),
        "euclidean_area_sum": e_sum, "euclidean_area_mean": mean_e, "symplectic_area_sum": a_sum, "symplectic_area_mean": mean_a,
        "kappa_mean": sum(k) / len(k) if k else 0.0, "kappa_median": quantile(k_sorted, 0.5), "kappa_q90": quantile(k_sorted, 0.9), "kappa_q95": quantile(k_sorted, 0.95),
        "kappa_max": max(k, default=0.0), "kappa_max_share": max(k, default=0.0) / k_total if k_total > 0 else 0.0,
        "kappa_top3_share": sum(k_top[:3]) / k_total if k_total > 0 else 0.0, "euclidean_weighted_kappa": weighted_kappa, "euclidean_kappa_covariance": cov,
        "symplectic_area_max_share": max(a, default=0.0) / a_total if a_total > 0 else 0.0, "symplectic_area_top3_share": sum(a_sorted[:3]) / a_total if a_total > 0 else 0.0,
        "symplectic_area_entropy": entropy, "symplectic_area_effective_face_count": effective,
        "decomposition_identity_abs_error": abs(sum(ei * ki for ei, ki in zip(e, k)) - a_sum),
        "decomposition_identity_rel_error": abs(sum(ei * ki for ei, ki in zip(e, k)) - a_sum) / max(1.0, a_sum),
        "translation_invariance_max_abs_error": max((f.get("translation_invariance_abs_error", 0.0) for f in faces), default=0.0),
        "positive_scaling_invariance_max_abs_error": max((f.get("positive_scaling_invariance_abs_error", 0.0) for f in faces), default=0.0),
        "ridge_count_matches_source": row.get("ridge_count_source") is None or int(row["ridge_count_source"]) == len(faces),
    }
    return summary, faces


def aggregate(name: str, rows: list[dict]) -> dict:
    if not rows:
        return {"name": name, "n": 0}
    keys = ["sys", "euclidean_area_sum", "symplectic_area_sum", "euclidean_weighted_kappa", "euclidean_kappa_covariance", "kappa_mean", "kappa_q90", "kappa_max", "symplectic_area_max_share", "symplectic_area_effective_face_count", "mixed_face_count", "structural_zero_face_count", "decomposition_identity_rel_error"]
    out = {"name": name, "n": len(rows), "sys_mean": sum(r["sys"] for r in rows) / len(rows)}
    for key in keys[1:]:
        out[key + "_mean"] = sum(r[key] for r in rows) / len(rows)
    out["sys_median"] = quantile(sorted(r["sys"] for r in rows), 0.5)
    out["sys_min"] = min(r["sys"] for r in rows)
    out["sys_max"] = max(r["sys"] for r in rows)
    out["vertex_count_distribution"] = {str(n): sum(r["vertex_count"] == n for r in rows) for n in sorted({r["vertex_count"] for r in rows})}
    return out


def main() -> None:
    p = argparse.ArgumentParser()
    p.add_argument("--input-dir", type=Path, required=True)
    p.add_argument("--out-dir", type=Path, required=True)
    args = p.parse_args()
    out = args.out_dir
    out.mkdir(parents=True, exist_ok=True)
    generic_rows = read_jsonl(args.input_dir / "generic-input.jsonl")
    product_rows = read_jsonl(args.input_dir / "product-5x5-input.jsonl")
    summaries, face_rows = [], []
    for row in generic_rows:
        summary, faces = geometry_summary(row, product=False)
        summary["feature_proxy"] = float(row["proxy"])
        summary["computed_proxy"] = summary["symplectic_area_mean"] / math.sqrt(summary["f64_volume"])
        summaries.append(summary)
        face_rows.append({"candidate_id": row["candidate_id"], "faces": faces})
    for row in product_rows:
        summary, faces = geometry_summary(row, product=True)
        summary["rho"] = row["rho"]
        summary["ridge_mean_feature"] = row["ridge_mean_feature"]
        summary["ridge_sum_feature"] = row["ridge_sum_feature"]
        summary["computed_mean_over_volume_sqrt"] = summary["symplectic_area_mean"] / math.sqrt(summary["f64_volume"])
        summary["computed_sum_over_volume_sqrt"] = summary["symplectic_area_sum"] / math.sqrt(summary["f64_volume"])
        summaries.append(summary)
        face_rows.append({"candidate_id": row["candidate_id"], "faces": faces})
    (out / "per_polytope.jsonl").write_text("".join(json.dumps(x, sort_keys=True, separators=(",", ":")) + "\n" for x in summaries))
    (out / "per_face.jsonl").write_text("".join(json.dumps(x, sort_keys=True, separators=(",", ":")) + "\n" for x in face_rows))
    generic = [r for r in summaries if not r["product"]]
    product = [r for r in summaries if r["product"]]
    groups = [aggregate("generic_selected", [r for r in generic if r["role"] == "selected"]), aggregate("generic_baseline", [r for r in generic if r["role"] == "baseline"]), aggregate("generic_ranks_1_10", [r for r in generic if r["role"] == "selected" and r["f64_rank"] <= 10]), aggregate("generic_ranks_11_100", [r for r in generic if r["role"] == "selected" and 11 <= r["f64_rank"] <= 100])]
    arm_groups = {"rho_only": [], "ridge_only": [], "overlap": [], "matched_control": []}
    for r in product:
        arms = set(r["arm_memberships"])
        if CONTROL_ID in arms:
            arm_groups["matched_control"].append(r)
        elif RHO_ID in arms and RIDGE_ID in arms:
            arm_groups["overlap"].append(r)
        elif RHO_ID in arms:
            arm_groups["rho_only"].append(r)
        elif RIDGE_ID in arms:
            arm_groups["ridge_only"].append(r)
        else:
            raise ValueError(f"product row without frozen arm: {r['candidate_id']}")
    groups.extend(aggregate("product_5x5_" + k, v) for k, v in arm_groups.items())
    contrasts = {
        "generic_selected_minus_baseline_sys": groups[0]["sys_mean"] - groups[1]["sys_mean"],
        "generic_rank_1_10_minus_11_100_sys": groups[2]["sys_mean"] - groups[3]["sys_mean"],
        "product_rho_only_minus_control_sys": aggregate("x", arm_groups["rho_only"])["sys_mean"] - aggregate("x", arm_groups["matched_control"])["sys_mean"],
        "product_ridge_only_minus_control_sys": aggregate("x", arm_groups["ridge_only"])["sys_mean"] - aggregate("x", arm_groups["matched_control"])["sys_mean"],
        "product_overlap_minus_control_sys": aggregate("x", arm_groups["overlap"])["sys_mean"] - aggregate("x", arm_groups["matched_control"])["sys_mean"],
        "product_rho_only_minus_ridge_only_sys": aggregate("x", arm_groups["rho_only"])["sys_mean"] - aggregate("x", arm_groups["ridge_only"])["sys_mean"],
    }
    validation = {
        "schema": "sys-datascience.ridge-tail-anatomy.validation.v1",
        "generic_rows": len(generic), "product_5x5_rows": len(product),
        "generic_unique_ids": len({r["candidate_id"] for r in generic}) == 200,
        "product_unique_ids": len({r["candidate_id"] for r in product}) == 142,
        "generic_group_sizes": {g["name"]: g["n"] for g in groups[:4]},
        "product_arm_sizes": {k: len(v) for k, v in arm_groups.items()},
        "product_arm_memberships": {k: sum(len(set(r["arm_memberships"])) for r in v) for k, v in arm_groups.items()},
        "generic_identity_max_rel_error": max(r["decomposition_identity_rel_error"] for r in generic),
        "product_identity_max_rel_error": max(r["decomposition_identity_rel_error"] for r in product),
        "generic_proxy_max_abs_error": max(abs(r["computed_proxy"] - r["feature_proxy"]) for r in summaries if not r["product"]),
        "product_feature_mean_max_abs_error": max(abs(r["computed_mean_over_volume_sqrt"] - r["ridge_mean_feature"]) for r in product),
        "product_feature_sum_max_abs_error": max(abs(r["computed_sum_over_volume_sqrt"] - r["ridge_sum_feature"]) for r in product),
        "product_5x5_face_composition": {"mixed": sorted({r["mixed_face_count"] for r in product}), "structural_zero": sorted({r["structural_zero_face_count"] for r in product}), "unknown": sorted({r["unknown_kind_face_count"] for r in product})},
        "ridge_count_source_matches": all(r["ridge_count_matches_source"] for r in summaries),
        "translation_invariance_max_abs_error": max(r["translation_invariance_max_abs_error"] for r in summaries),
        "positive_scaling_invariance_max_abs_error": max(r["positive_scaling_invariance_max_abs_error"] for r in summaries),
        "all_finite": all(math.isfinite(r[k]) for r in summaries for k in ("sys", "euclidean_area_sum", "symplectic_area_sum", "euclidean_weighted_kappa", "decomposition_identity_rel_error")),
        "identity_tolerance": IDENTITY_TOL,
        "proxy_tolerance": 2e-9,
    }
    validation["product_per_seed_arm_sizes"] = {str(seed): {name: sum(r["seed"] == seed for r in rows) for name, rows in arm_groups.items()} for seed in sorted({r["seed"] for r in product})}
    validation["control_disjoint_from_screen_arms"] = all(not (CONTROL_ID in set(r["arm_memberships"]) and (RHO_ID in set(r["arm_memberships"]) or RIDGE_ID in set(r["arm_memberships"]))) for r in product)
    validation["valid"] = all([validation["generic_unique_ids"], validation["product_unique_ids"], validation["generic_group_sizes"] == {"generic_selected": 100, "generic_baseline": 100, "generic_ranks_1_10": 10, "generic_ranks_11_100": 90}, validation["product_arm_sizes"] == {"rho_only": 42, "ridge_only": 42, "overlap": 8, "matched_control": 50}, validation["product_per_seed_arm_sizes"] == {"2026071201": {"rho_only": 23, "ridge_only": 23, "overlap": 2, "matched_control": 25}, "2026071202": {"rho_only": 19, "ridge_only": 19, "overlap": 6, "matched_control": 25}}, validation["control_disjoint_from_screen_arms"], validation["generic_identity_max_rel_error"] <= IDENTITY_TOL, validation["product_identity_max_rel_error"] <= IDENTITY_TOL, validation["generic_proxy_max_abs_error"] <= 2e-9, validation["product_feature_mean_max_abs_error"] <= 2e-9, validation["product_feature_sum_max_abs_error"] <= 2e-9, validation["product_5x5_face_composition"] == {"mixed": [25], "structural_zero": [10], "unknown": [0]}, validation["ridge_count_source_matches"], validation["translation_invariance_max_abs_error"] <= 1e-9, validation["positive_scaling_invariance_max_abs_error"] <= 1e-9, validation["all_finite"]])
    (out / "group-summary.json").write_text(json.dumps({"schema": "sys-datascience.ridge-tail-anatomy.group-summary.v1", "groups": groups, "contrasts": contrasts}, indent=2, sort_keys=True) + "\n")
    (out / "validation.json").write_text(json.dumps(validation, indent=2, sort_keys=True) + "\n")
    if not validation["valid"]:
        raise SystemExit("validation failed; inspect validation.json")
    output_paths = [out / name for name in ("per_face.jsonl", "per_polytope.jsonl", "group-summary.json", "validation.json")]
    manifest = {
        "schema": "sys-datascience.ridge-tail-anatomy.analysis-manifest.v1",
        "input_files": {name: sha256(args.input_dir / name) for name in ("generic-input.jsonl", "product-5x5-input.jsonl")},
        "output_files": {path.name: sha256(path) for path in output_paths},
        "rows": {"generic": len(generic), "product_5x5": len(product)},
        "valid": validation["valid"],
    }
    (out / "analysis-manifest.json").write_text(json.dumps(manifest, indent=2, sort_keys=True) + "\n")
    print(json.dumps({"valid": True, "contrasts": contrasts}, sort_keys=True))


if __name__ == "__main__":
    main()
