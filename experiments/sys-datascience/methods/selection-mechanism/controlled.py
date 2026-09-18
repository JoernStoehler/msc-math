#!/usr/bin/env python3
"""Reconstruct R on retained paired deformations; never evaluate capacities."""

import hashlib
import json
import math
from pathlib import Path

import numpy as np


HERE = Path(__file__).resolve().parent
METHODS = HERE.parent
OUT = HERE / "controlled-artifacts"


def read_rows(path):
    return [json.loads(line) for line in path.read_text().splitlines()]


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def area(vertices):
    return float(np.sum(vertices[:, 0] * np.roll(vertices[:, 1], -1)
                        - vertices[:, 1] * np.roll(vertices[:, 0], -1)) / 2)


def polygon(factor):
    normals = np.array(factor["normals"])
    heights = np.array(factor["heights"])
    count = len(normals)
    vertices = np.array([
        np.linalg.solve(normals[[i, (i + 1) % count]], heights[[i, (i + 1) % count]])
        for i in range(count)
    ])
    assert area(vertices) > 0
    assert np.max(normals @ vertices.T - heights[:, None]) < 1e-12
    assert np.min(np.linalg.norm(np.roll(vertices, -1, axis=0) - vertices, axis=1)) > 1e-12
    return vertices


def ridge_r(p, q):
    edges_p = np.roll(p, -1, axis=0) - p
    edges_q = np.roll(q, -1, axis=0) - q
    edge_sum = float(np.abs(edges_p @ edges_q.T).sum())
    # Total variation of a linear functional around a convex polygon is
    # twice its width. This independently checks the edge-sum computation.
    projections = edges_p @ q.T
    width_sum = float(2 * (projections.max(axis=1) - projections.min(axis=1)).sum())
    assert math.isclose(edge_sum, width_sum, rel_tol=1e-12, abs_tol=1e-12)
    result = edge_sum / math.sqrt(area(p) * area(q))
    assert result >= 8 - 1e-12
    return result


def average_ranks(values):
    values = np.asarray(values)
    ranks = np.empty(len(values), dtype=float)
    for value in np.unique(values):
        ranks[values == value] = (np.sum(values < value) + (np.sum(values == value) - 1) / 2)
    return ranks


def spearman(x, y):
    return float(np.corrcoef(average_ranks(x), average_ranks(y))[0, 1])


def summary(contrasts):
    x = [r["delta_R"] for r in contrasts]
    y = [r["delta_sys"] for r in contrasts]
    assert all(abs(v) > 1e-10 for v in x + y)
    return {
        "count": len(contrasts),
        "opposite_signs": sum(a * b < 0 for a, b in zip(x, y)),
        "same_signs": sum(a * b > 0 for a, b in zip(x, y)),
        "delta_R_delta_sys_spearman": spearman(x, y),
        "mean_delta_R": float(np.mean(x)),
        "mean_delta_sys": float(np.mean(y)),
        "smallest_absolute_delta_R": min(abs(v) for v in x),
        "smallest_absolute_delta_sys": min(abs(v) for v in y),
    }


def main():
    paired = METHODS / "paired-tangentialization" / "artifacts"
    endpoint = METHODS / "ridge-endpoint-path" / "artifacts"
    inputs_path = paired / "inputs.jsonl"
    results_path = paired / "results.jsonl"
    inputs = read_rows(inputs_path)
    results = read_rows(results_path)
    by_id = {r["id"]: r for r in results}
    expected = {(b, i, a) for b in ("4x4", "4x6") for i in range(8)
                for a in ("00", "10", "01", "11")}
    assert len(inputs) == len(results) == len(by_id) == 64
    assert {(r["bucket"], r["row"], r["arm"]) for r in inputs} == expected
    assert set(by_id) == {r["id"] for r in inputs}
    assert len({json.dumps(r["evaluator"], sort_keys=True) for r in results}) == 1
    assert results[0]["evaluator"]["input_file_sha256"] == digest(inputs_path)
    latent_factors = {}
    for row in inputs:
        factors = row["factors_post"]
        assert factors[0]["normals"] == factors[2]["normals"]
        assert factors[1]["normals"] == factors[3]["normals"]
        latent_key = (row["bucket"], row["row"])
        if latent_key in latent_factors:
            assert factors == latent_factors[latent_key]
        else:
            latent_factors[latent_key] = factors
    geometries = []
    max_area_error = 0.0
    max_volume_error = 0.0
    for row in inputs:
        result = by_id[row["id"]]
        assert result["status"] == "ok" and result["input"] == row
        assert result["capacity_route"] == "product"
        assert result["capacity_lower"] <= result["capacity"] <= result["capacity_upper"]
        assert math.isclose(result["sys"], result["capacity"] ** 2 / (2 * result["volume"]), rel_tol=1e-12)
        q_index = 2 if row["arm"][0] == "1" else 0
        p_index = 3 if row["arm"][1] == "1" else 1
        factors = [row["factors_post"][i] for i in (q_index, p_index)]
        q, p = [polygon(f) for f in factors]
        assert (len(q), len(p)) == tuple(map(int, row["bucket"].split("x")))
        dual = np.zeros((len(q) + len(p), 4))
        dual[:len(q), :2] = np.array(factors[0]["normals"]) / np.array(factors[0]["heights"])[:, None]
        dual[len(q):, 2:] = np.array(factors[1]["normals"]) / np.array(factors[1]["heights"])[:, None]
        assert np.max(np.abs(dual - row["dual_vertices"])) < 1e-12
        max_area_error = max(max_area_error, abs(area(q) - 1), abs(area(p) - 1))
        max_volume_error = max(max_volume_error, abs(area(q) * area(p) - result["volume"]))
        assert abs(area(q) * area(p) - result["volume"]) < 1e-12
        geometries.append({"id": row["id"], "bucket": row["bucket"], "row": row["row"],
                           "arm": row["arm"], "R": ridge_r(q, p), "sys": result["sys"]})
    by_grid = {(r["bucket"], r["row"], r["arm"]): r for r in geometries}
    contrasts = []
    for bucket in ("4x4", "4x6"):
        for index in range(8):
            base = by_grid[bucket, index, "00"]
            for arm in ("10", "01", "11"):
                target = by_grid[bucket, index, arm]
                contrasts.append({"bucket": bucket, "row": index, "arm": arm,
                                  "baseline_id": base["id"], "target_id": target["id"],
                                  "R_baseline": base["R"], "R_target": target["R"],
                                  "sys_baseline": base["sys"], "sys_target": target["sys"],
                                  "delta_R": target["R"] - base["R"],
                                  "delta_sys": target["sys"] - base["sys"]})
    # Independent retained geometries already have an API-computed 4D ridge
    # feature, providing a numerical cross-check of this reconstruction.
    candidates_path = endpoint / "candidates.jsonl"
    api_path = endpoint / "api-verification.jsonl"
    target_path = endpoint / "target-evaluation.jsonl"
    candidates = read_rows(candidates_path)
    api = {r["candidate_id"]: r for r in read_rows(api_path)}
    targets = {r["candidate_id"]: r for r in read_rows(target_path)}
    endpoint_rows = []
    for candidate in candidates:
        key = candidate["candidate_id"]
        r = ridge_r(np.array(candidate["q_vertices_ccw"]), np.array(candidate["p_vertices_ccw"]))
        assert math.isclose(r, api[key]["feature_r"], rel_tol=1e-12)
        assert api[key]["passed"] and targets[key]["result_is_finite"]
        assert targets[key]["candidates_sha256"] == digest(candidates_path)
        assert targets[key]["api_verification_sha256"] == digest(api_path)
        endpoint_rows.append({"id": key, "bucket": candidate["bucket"],
                              "path_label": candidate["path_label"], "R": r, "sys": targets[key]["sys"]})
    sources = [inputs_path, results_path, candidates_path, api_path, target_path]
    output = {
        "purpose": "Exploratory within-body ridge/sys check on retained paired tangentialization; no new capacity calls",
        "sources": {str(p.relative_to(METHODS)): digest(p) for p in sources},
        "numerical_checks": {"max_unit_area_error": max_area_error, "max_volume_reconstruction_error": max_volume_error},
        "primary_both_factors": summary([r for r in contrasts if r["arm"] == "11"]),
        "by_arm": {a: summary([r for r in contrasts if r["arm"] == a]) for a in ("10", "01", "11")},
        "by_bucket_and_arm": {b: {a: summary([r for r in contrasts if r["bucket"] == b and r["arm"] == a])
                                    for a in ("10", "01", "11")} for b in ("4x4", "4x6")},
        "same_sign_primary_pairs": [r for r in contrasts if r["arm"] == "11" and r["delta_R"] * r["delta_sys"] > 0],
        "endpoint_paths": endpoint_rows,
    }
    OUT.mkdir(exist_ok=True)
    (OUT / "summary.json").write_text(json.dumps(output, indent=2, allow_nan=False) + "\n")
    (OUT / "geometries.jsonl").write_text("".join(json.dumps(r, sort_keys=True) + "\n" for r in geometries))
    (OUT / "contrasts.jsonl").write_text("".join(json.dumps(r, sort_keys=True) + "\n" for r in contrasts))
    print(json.dumps({k: output[k] for k in ("numerical_checks", "primary_both_factors", "same_sign_primary_pairs")}, indent=2))


if __name__ == "__main__":
    main()
