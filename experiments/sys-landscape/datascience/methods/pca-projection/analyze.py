#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = [
#   "numpy",
# ]
# ///
"""PCA projection packet for the retained sys-landscape dataset."""

from __future__ import annotations

import argparse
import hashlib
import json
import math
import time
from pathlib import Path

import numpy as np


METHOD_DIR = Path(__file__).resolve().parent
DEFAULT_DATASET = METHOD_DIR.parents[1] / "dataset"
DEFAULT_OUTPUT = METHOD_DIR / "pca-summary.json"

EXCLUDED_INPUT_COLUMNS = {
    "poly_id",
    "dual_vertices_rational",
    "dual_vertices_f64",
    "dual_vertices_flat_f64",
    "capacity",
    "capacity_source",
    "sys",
    "sigma_gap_cutoff",
    "sigmas",
    "raw_orbit_scalars",
}
EXCLUDED_INPUT_PREFIXES = (
    "orbit_",
)
INCLUDED_PREFIXES = (
    "allpair_",
    "edge_",
    "facet_",
    "geom_",
    "ridge_",
    "transition_",
    "vertex_",
)
INCLUDED_COLUMNS = {
    "dual_vertex_count",
    "is_simple",
    "simple_vertex_fraction",
    "volume",
}


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--dataset", type=Path, default=DEFAULT_DATASET)
    parser.add_argument("--output", type=Path, default=DEFAULT_OUTPUT)
    parser.add_argument(
        "--limit",
        type=int,
        default=None,
        help="Optional smoke limit on polytope rows, after file-order loading.",
    )
    parser.add_argument("--components", type=int, default=6)
    return parser.parse_args()


def read_jsonl(path: Path, limit: int | None = None) -> list[dict]:
    rows: list[dict] = []
    with path.open() as handle:
        for index, line in enumerate(handle):
            if limit is not None and index >= limit:
                break
            rows.append(json.loads(line))
    return rows


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def is_number(value: object) -> bool:
    return isinstance(value, (bool, int, float)) and not isinstance(value, str)


def is_allowed_input_column(name: str) -> bool:
    if name in EXCLUDED_INPUT_COLUMNS:
        return False
    if any(name.startswith(prefix) for prefix in EXCLUDED_INPUT_PREFIXES):
        return False
    return name in INCLUDED_COLUMNS or any(name.startswith(prefix) for prefix in INCLUDED_PREFIXES)


def choose_features(rows: list[dict]) -> tuple[list[str], dict[str, list[str]]]:
    keys = sorted({key for row in rows for key in row})
    excluded: dict[str, list[str]] = {
        "target_capacity_identity_or_raw": [],
        "orbit_capacity_witness": [],
        "not_intrinsic_feature_policy": [],
        "non_numeric_or_non_finite": [],
        "constant": [],
    }
    candidates = []
    for key in keys:
        if key in EXCLUDED_INPUT_COLUMNS:
            excluded["target_capacity_identity_or_raw"].append(key)
            continue
        if any(key.startswith(prefix) for prefix in EXCLUDED_INPUT_PREFIXES):
            excluded["orbit_capacity_witness"].append(key)
            continue
        if not is_allowed_input_column(key):
            excluded["not_intrinsic_feature_policy"].append(key)
            continue
        values = [row.get(key) for row in rows]
        if not all(is_number(value) and math.isfinite(float(value)) for value in values):
            excluded["non_numeric_or_non_finite"].append(key)
            continue
        floats = np.array([float(value) for value in values], dtype=float)
        if float(np.std(floats)) == 0.0:
            excluded["constant"].append(key)
            continue
        candidates.append(key)
    return candidates, excluded


def standardized_matrix(rows: list[dict], features: list[str]) -> np.ndarray:
    matrix = np.array([[float(row[name]) for name in features] for row in rows], dtype=float)
    means = matrix.mean(axis=0)
    stds = matrix.std(axis=0)
    return (matrix - means) / stds


def fit_pca(z: np.ndarray, component_count: int) -> tuple[np.ndarray, np.ndarray, np.ndarray]:
    _, singular_values, vh = np.linalg.svd(z, full_matrices=False)
    count = min(component_count, vh.shape[0])
    components = vh[:count].copy()
    for row in components:
        pivot = int(np.argmax(np.abs(row)))
        if row[pivot] < 0:
            row *= -1
    scores = z @ components.T
    eigenvalues = (singular_values[:count] ** 2) / (z.shape[0] - 1)
    explained = eigenvalues / np.sum((singular_values**2) / (z.shape[0] - 1))
    return scores, components, explained


def quantile_threshold(values: np.ndarray, q: float, side: str) -> float:
    method = "higher" if side == "high" else "lower"
    return float(np.quantile(values, q, method=method))


def summarize_region(name: str, mask: np.ndarray, sys_values: np.ndarray, global_top_mask: np.ndarray) -> dict:
    region_sys = sys_values[mask]
    return {
        "name": name,
        "row_count": int(mask.sum()),
        "row_fraction": float(mask.mean()),
        "max_sys": float(region_sys.max()),
        "mean_sys": float(region_sys.mean()),
        "p90_sys": float(np.quantile(region_sys, 0.9)),
        "global_top_1_percent_rows_captured": int(np.logical_and(mask, global_top_mask).sum()),
    }


def candidate_region_audit(scores: np.ndarray, sys_values: np.ndarray) -> list[dict]:
    pc1 = scores[:, 0]
    pc2 = scores[:, 1]
    radius = np.sqrt(pc1**2 + pc2**2)
    top_1_threshold = quantile_threshold(sys_values, 0.99, "high")
    global_top_mask = sys_values >= top_1_threshold
    regions = [
        ("pc1_high_top_5_percent", pc1 >= quantile_threshold(pc1, 0.95, "high")),
        ("pc1_low_top_5_percent", pc1 <= quantile_threshold(pc1, 0.05, "low")),
        ("pc2_high_top_5_percent", pc2 >= quantile_threshold(pc2, 0.95, "high")),
        ("pc2_low_top_5_percent", pc2 <= quantile_threshold(pc2, 0.05, "low")),
        ("pc_radius_high_top_5_percent", radius >= quantile_threshold(radius, 0.95, "high")),
    ]
    return [summarize_region(name, mask, sys_values, global_top_mask) for name, mask in regions]


def top_loadings(features: list[str], components: np.ndarray, count: int = 8) -> dict[str, list[dict]]:
    out: dict[str, list[dict]] = {}
    for component_index, weights in enumerate(components, start=1):
        order = np.argsort(np.abs(weights))[::-1][:count]
        out[f"pc{component_index}"] = [
            {"column": features[index], "loading": float(weights[index])}
            for index in order
        ]
    return out


def source_counts(observation_rows: list[dict]) -> dict[str, int]:
    counts: dict[str, int] = {}
    for row in observation_rows:
        name = row.get("dataset")
        counts[name] = counts.get(name, 0) + 1
    return dict(sorted(counts.items()))


def main() -> None:
    args = parse_args()
    started = time.perf_counter()
    dataset = args.dataset
    poly_path = dataset / "polytope-table.jsonl"
    observation_path = dataset / "observation-table.jsonl"
    poly_rows = read_jsonl(poly_path, args.limit)
    observation_rows = read_jsonl(observation_path, args.limit)

    features, excluded = choose_features(poly_rows)
    if len(features) < 2:
        raise SystemExit("Need at least two allowed nonconstant numeric features for PCA.")

    z = standardized_matrix(poly_rows, features)
    scores, components, explained = fit_pca(z, args.components)
    sys_values = np.array([float(row["sys"]) for row in poly_rows], dtype=float)

    summary = {
        "method": "pca-projection",
        "dataset": {
            "path": str(dataset),
            "limit": args.limit,
            "polytope_rows": len(poly_rows),
            "observation_rows": len(observation_rows),
            "max_sys": float(sys_values.max()),
            "sys_gt_1_rows": int((sys_values > 1.0).sum()),
            "source_counts": source_counts(observation_rows),
            "sha256": {
                "polytope-table.jsonl": sha256(poly_path),
                "observation-table.jsonl": sha256(observation_path),
            },
        },
        "validity_guard": {
            "input_policy": "Fit PCA only on allowed scalar intrinsic polytope columns.",
            "sys_use": "Used only after fitting for audit and interpretation.",
            "excluded_inputs": excluded,
            "included_feature_count": len(features),
            "included_features": features,
        },
        "pca": {
            "component_count": int(components.shape[0]),
            "explained_variance_ratio": [float(value) for value in explained],
            "top_loadings": top_loadings(features, components),
        },
        "audit": {
            "global_sys_mean": float(sys_values.mean()),
            "global_sys_p90": float(np.quantile(sys_values, 0.9)),
            "global_sys_p99": float(np.quantile(sys_values, 0.99)),
            "candidate_regions": candidate_region_audit(scores, sys_values),
        },
        "runtime_seconds": time.perf_counter() - started,
    }
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(summary, indent=2, sort_keys=True) + "\n")


if __name__ == "__main__":
    main()
