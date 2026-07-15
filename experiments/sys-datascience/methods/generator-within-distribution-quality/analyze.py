#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = ["numpy"]
# ///

"""Finite-sample diversity and coverage diagnostics for one shape distribution.

This is deliberately copy-local.  A future generator needs only an adapter that
emits ``shape-row-v1`` (or the existing ``factor-shape-row-v1``) rows with
vertices in cyclic order.  All summaries are descriptive at the observed n.
"""

from __future__ import annotations

import argparse
from collections import Counter, defaultdict
import hashlib
import itertools
import json
import math
from pathlib import Path
from typing import Any

import numpy as np


SCHEMA = "shape-row-v1"
SEED = 20260715
SMALL_N = 20
GOOD_TURING_N = 50
ANALYZER_REPO_PATH = "experiments/sys-datascience/methods/generator-within-distribution-quality/analyze.py"


def polygon_area(v: np.ndarray) -> float:
    return float(0.5 * np.sum(v[:, 0] * np.roll(v[:, 1], -1) - v[:, 1] * np.roll(v[:, 0], -1)))


def regular_polygon(n: int = 8) -> np.ndarray:
    angles = 2 * np.pi * np.arange(n) / n
    return np.column_stack((np.cos(angles), np.sin(angles)))


def normalize_vertices(vertices: Any) -> np.ndarray:
    v = np.asarray(vertices, dtype=float)
    if v.ndim != 2 or v.shape[1] != 2 or v.shape[0] < 3 or not np.all(np.isfinite(v)):
        raise ValueError("vertices must be finite n-by-2 with n >= 3")
    area = polygon_area(v)
    if area <= 0:
        raise ValueError("vertices must be strictly CCW with positive area")
    edge_a = np.roll(v, -1, axis=0) - v
    edge_b = np.roll(v, -2, axis=0) - np.roll(v, -1, axis=0)
    turns = edge_a[:, 0] * edge_b[:, 1] - edge_a[:, 1] * edge_b[:, 0]
    diameter = float(np.max(np.linalg.norm(v[:, None, :] - v[None, :, :], axis=2)))
    if np.any(turns <= 1e-12 * diameter * diameter):
        raise ValueError("vertices must be strictly convex and cyclic CCW")
    # The centroid is a deliberately simple frame for the raw view.  It does
    # not quotient cyclic order, rotation, or reflection.
    return (v - np.mean(v, axis=0)) / math.sqrt(area)


def validate_row(row: dict[str, Any], line: int) -> dict[str, Any]:
    if row.get("schema") not in {SCHEMA, "factor-shape-row-v1"}:
        raise ValueError(f"line {line}: unsupported schema {row.get('schema')!r}")
    sample_id = row.get("sample_id")
    if not isinstance(sample_id, str) or not sample_id:
        raise ValueError(f"line {line}: sample_id must be nonempty")
    raw = row.get("vertices_ccw", row.get("vertices"))
    v = normalize_vertices(raw)
    n = row.get("side_count", len(v))
    if n != len(v):
        raise ValueError(f"line {line}: side_count disagrees with vertices")
    population = row.get("population", row.get("law", "unknown"))
    if not isinstance(population, str) or not population:
        raise ValueError(f"line {line}: population/law must be nonempty")
    out = dict(row)
    out["sample_id"] = sample_id
    out["population"] = population
    out["side_count"] = int(n)
    out["_vertices"] = v
    return out


def load_rows(path: Path) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    seen: set[str] = set()
    with path.open() as handle:
        for line, text in enumerate(handle, 1):
            if not text.strip():
                continue
            row = json.loads(text)
            if not isinstance(row, dict):
                raise ValueError(f"line {line}: expected JSON object")
            item = validate_row(row, line)
            if item["sample_id"] in seen:
                raise ValueError(f"duplicate sample_id {item['sample_id']!r}")
            seen.add(item["sample_id"])
            rows.append(item)
    if not rows:
        raise ValueError(f"{path}: no rows")
    return rows


def sha256_bytes(payload: bytes) -> str:
    return hashlib.sha256(payload).hexdigest()


def build_provenance(input_path: Path, seed: int) -> dict[str, Any]:
    """Return deterministic identity/provenance; omit mutable HEAD by contract."""
    try:
        input_label = str(input_path.resolve().relative_to(Path(__file__).resolve().parent))
    except ValueError:
        repo_root = Path(__file__).resolve().parents[4]
        try:
            input_label = str(input_path.resolve().relative_to(repo_root))
        except ValueError:
            input_label = str(input_path)
    return {
        "input_path_as_invoked": input_label,
        "input_sha256": sha256_bytes(input_path.read_bytes()),
        "analyzer_repo_path": ANALYZER_REPO_PATH,
        "analyzer_source_sha256": sha256_bytes(Path(__file__).read_bytes()),
        "source_revision_contract": "analyzer_source_sha256 identifies the exact analyzer bytes; mutable VCS HEAD is intentionally omitted to avoid self-referential artifacts",
        "source_dirty_contract": "run only from a clean worktree for this owned path; a dirty source invalidates the artifact and must be repaired before reuse",
        "command_template": "uv run --script analyze.py --input <input> --out-dir <out-dir>",
        "seed": seed,
    }


def canonicalize_json(value: Any) -> Any:
    """Round floating diagnostics so separate BLAS/JSON runs compare bytewise."""
    if isinstance(value, float):
        return round(value, 12)
    if isinstance(value, dict):
        return {key: canonicalize_json(item) for key, item in value.items()}
    if isinstance(value, list):
        return [canonicalize_json(item) for item in value]
    return value


def render_report(report: dict[str, Any]) -> str:
    return json.dumps(canonicalize_json(report), indent=2, sort_keys=True) + "\n"


def procrustes_distance(left: np.ndarray, right: np.ndarray) -> float:
    """Best orientation-preserving cyclic vertex alignment (frame view)."""
    if left.shape != right.shape:
        raise ValueError("frame distance requires equal side counts")
    best = float("inf")
    for shift in range(len(right)):
        candidate = np.roll(right, shift, axis=0)
        cross = candidate.T @ left
        u, _, vt = np.linalg.svd(cross)
        r = u @ vt
        if np.linalg.det(r) < 0:
            u[:, -1] *= -1
            r = u @ vt
        aligned = candidate @ r
        best = min(best, float(np.sqrt(np.mean((left - aligned) ** 2))))
    return best


def raw_distance(left: np.ndarray, right: np.ndarray) -> float:
    if left.shape != right.shape:
        raise ValueError("raw distance requires equal side counts")
    return float(np.sqrt(np.mean((left - right) ** 2)))


def distance_matrix(rows: list[dict[str, Any]], view: str) -> np.ndarray:
    n = len(rows)
    result = np.zeros((n, n), dtype=float)
    fn = raw_distance if view == "raw_ordered" else procrustes_distance
    for i, j in itertools.combinations(range(n), 2):
        d = round(fn(rows[i]["_vertices"], rows[j]["_vertices"]), 12)
        result[i, j] = result[j, i] = d
    return result


def quantile_or_none(values: np.ndarray, q: float) -> float | None:
    return float(np.quantile(values, q)) if len(values) else None


def pair_summary(distance: np.ndarray) -> dict[str, Any]:
    n = len(distance)
    pairs = distance[np.triu_indices(n, 1)]
    if not len(pairs):
        return {"n": n, "pair_count": 0, "status": "small-sample", "mean": None}
    masked = distance.copy()
    np.fill_diagonal(masked, np.inf)
    nn = np.min(masked, axis=1)
    eps = max(1e-12, float(np.quantile(pairs, 0.1)) * 0.1)
    return {
        "n": n,
        "pair_count": int(len(pairs)),
        "status": "small-sample" if n < SMALL_N else "usable-descriptive",
        "mean": float(np.mean(pairs)),
        "median": float(np.median(pairs)),
        "p90": float(np.quantile(pairs, 0.9)),
        "nearest_neighbor_mean": float(np.mean(nn)),
        "nearest_neighbor_median": float(np.median(nn)),
        "duplicate_pair_fraction": float(np.mean(pairs <= 1e-10)),
        "near_duplicate_fraction_eps": float(np.mean(pairs <= eps)),
        "near_duplicate_eps": eps,
    }


def greedy_coverage(distance: np.ndarray, max_k: int = 8) -> dict[str, Any]:
    n = len(distance)
    if not n:
        return {"status": "empty"}
    med = float(np.median(distance[np.triu_indices(n, 1)])) if n > 1 else 0.0
    selected = [0]
    nearest = distance[:, 0].copy()
    out = []
    for k in range(1, min(max_k, n) + 1):
        if k > 1:
            candidate = int(np.argmax(nearest))
            selected.append(candidate)
            nearest = np.minimum(nearest, distance[:, candidate])
        out.append({"k": k, "radius": float(np.max(nearest)), "radius_over_pair_median": float(np.max(nearest) / med) if med > 0 else None})
    return {"status": "small-sample" if n < SMALL_N else "usable-descriptive", "curves": out}


def occupancy(rows: list[dict[str, Any]]) -> dict[str, Any]:
    # A view-cell is intentionally coarse unless the producer supplies an
    # explicit combinatorial_cell/f_vector label.  Fixed-side planar polygons
    # otherwise have one combinatorial type, so inventing extra cells would be
    # misleading.
    labels = []
    supplied_combinatorial = 0
    for row in rows:
        explicit = row.get("combinatorial_cell", row.get("f_vector"))
        if explicit is not None:
            labels.append("combinatorial:" + json.dumps(explicit, sort_keys=True, separators=(",", ":")))
            supplied_combinatorial += 1
            continue
        v = row["_vertices"]
        edges = np.roll(v, -1, axis=0) - v
        lengths = np.linalg.norm(edges, axis=1)
        cv = float(np.std(lengths) / np.mean(lengths))
        labels.append(str(row["side_count"]) + ":" + str(min(4, int(cv / 0.1))))
    counts = Counter(labels)
    n = len(labels)
    frequencies = Counter(counts.values())
    f1 = frequencies.get(1, 0)
    entropy = -sum((c / n) * math.log(c / n) for c in counts.values()) if n else 0.0
    unseen = f1 / n if n else None
    return {
        "cell_definition": "producer combinatorial_cell/f_vector when supplied; otherwise side_count plus edge-length-CV bin floor(CV/0.1), capped at 4",
        "combinatorial_label_rows": supplied_combinatorial,
        "cell_count_observed": len(counts),
        "occupied_cells": dict(sorted(counts.items())),
        "plugin_entropy": entropy,
        "effective_number_exp_entropy": math.exp(entropy),
        "good_turing_unseen_mass": unseen,
        "good_turing_status": "small-sample-limit" if n < GOOD_TURING_N else "descriptive-singleton-estimate",
        "singleton_cell_count": f1,
        "n": n,
    }


def cluster_balance(distance: np.ndarray) -> dict[str, Any]:
    n = len(distance)
    if n < 2:
        return {"status": "small-sample", "cluster_count": n}
    pairs = distance[np.triu_indices(n, 1)]
    threshold = float(np.quantile(pairs, 0.25))
    seen = set()
    sizes = []
    for root in range(n):
        if root in seen:
            continue
        stack = [root]
        seen.add(root)
        size = 0
        while stack:
            i = stack.pop()
            size += 1
            for j in np.flatnonzero(distance[i] <= threshold):
                j = int(j)
                if j not in seen:
                    seen.add(j)
                    stack.append(j)
        sizes.append(size)
    sizes.sort(reverse=True)
    return {"status": "small-sample" if n < SMALL_N else "usable-descriptive", "threshold": threshold, "cluster_count": len(sizes), "cluster_sizes": sizes, "largest_fraction": sizes[0] / n}


def saturation(distance: np.ndarray, seed: int = SEED) -> dict[str, Any]:
    n = len(distance)
    if n < 2:
        return {"status": "small-sample", "points": []}
    rng = np.random.default_rng(seed)
    points = []
    for size in sorted(set([2, 4, 8, 16, 32, n])):
        if size > n:
            continue
        reps = min(32, max(4, 128 // size))
        means = []
        for _ in range(reps):
            idx = rng.choice(n, size=size, replace=False)
            means.append(float(np.mean(distance[np.ix_(idx, idx)][np.triu_indices(size, 1)])))
        points.append({"n": size, "replicates": reps, "mean_pair_distance": float(np.mean(means)), "sd": float(np.std(means, ddof=1)) if len(means) > 1 else 0.0})
    return {"status": "small-sample" if n < SMALL_N else "usable-descriptive", "points": points, "seed": seed}


def outlier_sensitivity(distance: np.ndarray) -> dict[str, Any]:
    n = len(distance)
    if n < 4:
        return {"status": "small-sample"}
    pairs = distance[np.triu_indices(n, 1)]
    masked = distance.copy()
    np.fill_diagonal(masked, np.inf)
    nn = np.min(masked, axis=1)
    mean_all = float(np.mean(pairs))
    leave_one_out = []
    for drop_index in range(n):
        keep = [i for i in range(n) if i != drop_index]
        reduced = distance[np.ix_(keep, keep)]
        leave_one_out.append(float(np.mean(reduced[np.triu_indices(n - 1, 1)])))
    drop = int(np.argmax(np.abs(np.asarray(leave_one_out) - mean_all)))
    mean_reduced = leave_one_out[drop]
    return {"status": "usable-descriptive", "most-influential_index": drop, "max_nearest_neighbor": float(np.max(nn)), "mean_pair_all": mean_all, "mean_pair_without_most_influential": mean_reduced, "relative_change": float(mean_reduced / mean_all - 1.0) if mean_all else None}


def _tail_score(row: dict[str, Any]) -> float:
    """Frozen target-free novelty score: edge-length coefficient of variation."""
    v = row["_vertices"]
    lengths = np.linalg.norm(np.roll(v, -1, axis=0) - v, axis=1)
    return float(np.std(lengths) / np.mean(lengths))


def rare_discovery(rows: list[dict[str, Any]]) -> dict[str, Any]:
    """Passive-search utility on an input-order split calibration/holdout.

    The event rule is frozen from the first half: an event is a row whose
    target-free edge-CV score is at least the calibration 90th percentile.
    This deliberately measures scalar-tail discovery separately from metric
    coverage. Input order is preserved: no sample-id sorting or post-hoc
    selection is performed. Attempt/accepted costs are consumed only when a
    row declares ``cost_semantics=counts-v1`` and supplies numeric
    ``attempt_count``/``accepted_count`` fields.
    """
    n = len(rows)
    if n < 4:
        return {"status": "small-sample-limit", "n": n}
    split = max(2, n // 2)
    calibration, holdout = rows[:split], rows[split:]
    threshold = float(np.quantile([_tail_score(r) for r in calibration], 0.9))
    hits = [_tail_score(r) >= threshold for r in holdout]
    hit_indices = [i for i, hit in enumerate(hits) if hit]
    block_size = max(1, min(4, len(holdout) // 4))
    blocks = [hits[i : i + block_size] for i in range(0, len(hits), block_size)]
    hit_probability = float(np.mean(hits)) if hits else None
    zero_hit_upper_95 = (1.0 - 0.05 ** (1.0 / len(blocks))) if blocks and not any(hits) else None
    signatures = []
    for i in hit_indices:
        row = holdout[i]
        signature = row.get("event_signature")
        if signature is None:
            signature = f"cv-bin-{min(9, int(_tail_score(row) / 0.05))}"
        signatures.append(str(signature))
    cost_rows = [
        row
        for row in holdout
        if row.get("cost_semantics") == "counts-v1"
        and isinstance(row.get("attempt_count"), (int, float))
        and not isinstance(row.get("attempt_count"), bool)
        and isinstance(row.get("accepted_count"), (int, float))
        and not isinstance(row.get("accepted_count"), bool)
    ]
    if len(cost_rows) == len(holdout):
        cost_status = "declared-counts-v1"
        attempted = sum(float(row["attempt_count"]) for row in cost_rows)
        accepted = sum(float(row["accepted_count"]) for row in cost_rows)
    elif cost_rows:
        cost_status = "partial-declared-counts-v1"
        attempted = accepted = None
    else:
        cost_status = "unavailable-no-declared-count-semantics"
        attempted = accepted = None
    independent_blocks = max(1, len(blocks))
    first = (hit_indices[0] + 1) if hit_indices else None
    return {
        "status": "small-sample-limit" if n < GOOD_TURING_N else "descriptive-split-holdout",
        "n": n,
        "calibration_n": len(calibration),
        "holdout_n": len(holdout),
        "split_contract": "input order; first half calibration, second half holdout",
        "holdout_order": "input order",
        "frozen_score": "edge-length coefficient of variation",
        "frozen_threshold_calibration_p90": threshold,
        "holdout_hit_count": len(hit_indices),
        "holdout_hit_probability": hit_probability,
        "time_to_first_hit_holdout_rows": first,
        "distinct_tail_signatures": sorted(set(signatures)),
        "distinct_tail_signature_count": len(set(signatures)),
        "blocks": len(blocks),
        "block_size": block_size,
        "blocks_with_hit": sum(bool(block) and any(block) for block in blocks),
        "zero_hit_upper_bound_95_if_none": zero_hit_upper_95,
        "attempted_cost": attempted,
        "accepted_cost": accepted,
        "cost_status": cost_status,
        "cost_contract": "only cost_semantics=counts-v1 with attempt_count and accepted_count is consumed; attempts/accepted fields and attempt indices are ignored",
        "independent_block_cost": independent_blocks,
        "interpretation": "scalar tail event only; it is not geometric support coverage or a population discovery probability",
    }


def summarize_stratum(rows: list[dict[str, Any]], seed: int = SEED) -> dict[str, Any]:
    result: dict[str, Any] = {"n": len(rows), "population": rows[0]["population"], "side_count": rows[0]["side_count"], "views": {}, "occupancy": occupancy(rows), "rare_region_discovery": rare_discovery(rows)}
    for view in ("raw_ordered", "frame_adjusted"):
        matrix = distance_matrix(rows, view)
        result["views"][view] = {"pair_distances": pair_summary(matrix), "k_center_coverage": greedy_coverage(matrix), "cluster_balance": cluster_balance(matrix), "saturation": saturation(matrix, seed), "outlier_sensitivity": outlier_sensitivity(matrix)}
    result["interpretation_guardrails"] = [
        "raw/frame disagreement is representation sensitivity, not evidence that one view is correct",
        "support coverage, mass concentration, finite-sample discovery, and feature variance are separate quantities",
        "no p-values or population ranking are supported by this bounded descriptive sample",
    ]
    return result


def affine_shape(base: np.ndarray, stretch: float, shear: float, angle: float, shift: int = 0) -> np.ndarray:
    matrix = np.array([[math.exp(stretch), shear], [0.0, math.exp(-stretch)]])
    rotation = np.array([[math.cos(angle), -math.sin(angle)], [math.sin(angle), math.cos(angle)]])
    v = base @ matrix.T @ rotation.T
    if shift:
        v = np.roll(v, shift, axis=0)
    return v


def synthetic_rows(seed: int = SEED, per_case: int = 18) -> list[dict[str, Any]]:
    rng = np.random.default_rng(seed)
    base = regular_polygon(8)
    rows: list[dict[str, Any]] = []
    cases = ["identical", "concentrated", "broad", "multimodal", "duplicated", "imbalanced", "contaminated-outliers", "rare-mixture", "dependent-duplicates"]
    for case in cases:
        for i in range(per_case):
            if case == "identical":
                stretch, shear, angle = 0.0, 0.0, 0.31 * i
                mode = "same"
            elif case == "concentrated":
                stretch, shear, angle = rng.normal(0, 0.01), rng.normal(0, 0.005), rng.normal(0, 0.02)
                mode = "one"
            elif case == "broad":
                stretch, shear, angle = rng.normal(0, 0.3), rng.normal(0, 0.2), rng.uniform(-math.pi, math.pi)
                mode = "one"
            elif case in {"multimodal", "imbalanced"}:
                mode = "A" if (i < per_case // 2 if case == "multimodal" else i < int(0.8 * per_case)) else "B"
                stretch, shear, angle = ((-0.28, 0.05, 0.2) if mode == "A" else (0.3, -0.05, 1.2))
                stretch += rng.normal(0, 0.015); angle += rng.normal(0, 0.02)
            elif case == "duplicated":
                mode = "one"; j = i // 3; stretch, shear, angle = 0.08 * j, 0.02 * j, 0.1 * j
            elif case == "rare-mixture":
                mode = "rare" if i == per_case - 1 else "common"
                stretch, shear, angle = ((0.8, 0.5, 0.7) if mode == "rare" else (rng.normal(0, 0.02), rng.normal(0, 0.01), rng.normal(0, 0.03)))
            elif case == "dependent-duplicates":
                mode = "block-dependent"
                j = i // 3
                stretch, shear, angle = 0.12 * j, 0.03 * j, 0.1 * j
            else:
                mode = "outlier" if i >= per_case - 2 else "one"
                stretch, shear, angle = ((1.2, 0.8, 0.0) if mode == "outlier" else (rng.normal(0, 0.03), rng.normal(0, 0.02), rng.normal(0, 0.04)))
            vertices = affine_shape(base, stretch, shear, angle, shift=(i % 5))
            rows.append({"schema": SCHEMA, "sample_id": f"synthetic/{case}/{i}", "population": case, "law": "synthetic-v1", "side_count": 8, "mode": mode, "vertices_ccw": vertices.tolist()})
    return rows


def write_jsonl(rows: list[dict[str, Any]], path: Path) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w") as handle:
        for row in rows:
            clean = {k: v for k, v in row.items() if not k.startswith("_")}
            handle.write(json.dumps(clean, sort_keys=True, separators=(",", ":")) + "\n")


def analyze(rows: list[dict[str, Any]], seed: int = SEED, provenance: dict[str, Any] | None = None) -> dict[str, Any]:
    strata: dict[tuple[str, int], list[dict[str, Any]]] = defaultdict(list)
    for row in rows:
        strata[(row["population"], row["side_count"])].append(row)
    results = [summarize_stratum(group, seed) for _, group in sorted(strata.items())]
    return {"schema": "generator-within-distribution-quality-report-v1", "seed": seed, "rows": len(rows), "strata_count": len(results), "strata": results, "provenance": provenance or {"status": "not-supplied; use build_provenance for retained artifacts"}, "metric_contract": {"raw_ordered": "centroid/area-normalized vertex coordinates, preserving cyclic start and orientation", "frame_adjusted": "centroid/area-normalized cyclic vertex alignment with best orientation-preserving 2D rotation", "stratification": "population (law plus knob) and side_count; no pooling", "scaling": "pair and frame distances O(n^2 * side_count), occupancy/rare score O(n), saturation bounded by 32 replicates", "minimum_useful_n": {"pair/NN": 10, "cluster/coverage": 20, "Good-Turing singleton estimate": 50, "split rare-event holdout": 50}}, "dispositions": {"implemented": ["raw and frame-adjusted pair/nearest-neighbor distributions", "duplicate/near-duplicate rates", "greedy k-center coverage curves", "coarse occupancy, entropy/effective number, Good-Turing singleton diagnostic", "subsample saturation", "distance-threshold cluster balance", "leave-one-influential-point-outlier sensitivity", "frozen target-free rare-region discovery on input-order split calibration/holdout"], "deferred": ["certified packing numbers (heuristic k-center retained instead)", "population-level unseen-mass inference", "formal clustering model selection", "p-values and generator rankings"]}}


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--input", type=Path)
    parser.add_argument("--out-dir", type=Path, default=Path("artifacts"))
    parser.add_argument("--write-synthetic-fixture", type=Path)
    parser.add_argument("--seed", type=int, default=SEED)
    args = parser.parse_args()
    if args.write_synthetic_fixture:
        write_jsonl(synthetic_rows(args.seed), args.write_synthetic_fixture)
    if not args.input:
        return
    rows = load_rows(args.input)
    report = analyze(rows, args.seed, build_provenance(args.input, args.seed))
    args.out_dir.mkdir(parents=True, exist_ok=True)
    (args.out_dir / "report.json").write_text(render_report(report))
    table = args.out_dir / "summary.tsv"
    with table.open("w") as handle:
        handle.write("population\tside_count\tn\tview\tpair_mean\tnn_mean\tduplicate_fraction\tcell_count\tgood_turing_unseen_mass\n")
        for stratum in report["strata"]:
            for view, metrics in stratum["views"].items():
                pair = metrics["pair_distances"]
                handle.write("\t".join(map(str, [stratum["population"], stratum["side_count"], stratum["n"], view, pair.get("mean"), pair.get("nearest_neighbor_mean"), pair.get("duplicate_pair_fraction"), stratum["occupancy"]["cell_count_observed"], stratum["occupancy"]["good_turing_unseen_mass"]])) + "\n")


if __name__ == "__main__":
    main()
