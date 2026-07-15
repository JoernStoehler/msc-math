#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = ["numpy"]
# ///

"""Target-free dimension, density-body, and topology diagnostics for polytope laws.

The estimators in this file deliberately share only Euclidean distance matrices.
They are descriptive calibration tools, not a test that a generator has a
manifold-valued population or a theorem about a quotient space.
"""

from __future__ import annotations

import argparse
from collections import Counter, defaultdict
from dataclasses import dataclass
import hashlib
import json
import math
from pathlib import Path
import subprocess
import time
from typing import Any, Callable

import numpy as np


SCHEMA = "generator-distribution-dimension-report-v1"
EPS = 1e-12


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for block in iter(lambda: handle.read(1 << 20), b""):
            digest.update(block)
    return digest.hexdigest()


def repository_relative_display(path: Path) -> str:
    """Keep retained reports portable when an input comes from another worktree."""
    parts = path.resolve().parts
    try:
        start = parts.index("experiments")
    except ValueError:
        return path.name
    return "/".join(parts[start:])


def pairwise_distances(points: np.ndarray) -> np.ndarray:
    squared = np.sum(points * points, axis=1)[:, None] + np.sum(points * points, axis=1)[None, :] - 2 * points @ points.T
    return np.sqrt(np.maximum(squared, 0.0))


def stable_rank(values: np.ndarray, tolerance: float = 1e-10) -> int:
    if values.size == 0 or values[0] <= EPS:
        return 0
    return int(np.sum(values > tolerance * values[0]))


def global_pca(points: np.ndarray) -> dict[str, Any]:
    centered = points - np.mean(points, axis=0)
    values = np.linalg.eigvalsh(centered.T @ centered / max(len(points) - 1, 1))[::-1]
    total = float(np.sum(values))
    participation = float(total * total / np.sum(values * values)) if total > EPS else 0.0
    return {
        "ambient_dimension": int(points.shape[1]),
        "numerical_rank": stable_rank(values),
        "participation_ratio": participation,
        "explained_variance": (values / total).tolist() if total > EPS else values.tolist(),
        "eigenvalues": values.tolist(),
    }


def local_pca(points: np.ndarray, neighbors: int) -> dict[str, Any]:
    distance = pairwise_distances(points)
    dimensions: list[float] = []
    ratios: list[float] = []
    for i in range(len(points)):
        ids = np.argsort(distance[i])[1 : neighbors + 1]
        cloud = points[ids] - np.mean(points[ids], axis=0)
        values = np.linalg.eigvalsh(cloud.T @ cloud / max(len(ids) - 1, 1))[::-1]
        total = float(np.sum(values))
        dimensions.append(float(total * total / np.sum(values * values)) if total > EPS else 0.0)
        ratios.append(float(values[0] / total) if total > EPS else 0.0)
    return {"neighbors": neighbors, "local_participation_ratio": summary(dimensions), "first_pc_fraction": summary(ratios)}


def summary(values: list[float] | np.ndarray) -> dict[str, float]:
    array = np.asarray(values, dtype=float)
    return {"min": float(np.min(array)), "q25": float(np.quantile(array, .25)), "median": float(np.median(array)), "q75": float(np.quantile(array, .75)), "max": float(np.max(array)), "mean": float(np.mean(array))}


def nonzero_knn(distance: np.ndarray, max_k: int) -> tuple[np.ndarray, int]:
    ordered = np.sort(distance, axis=1)[:, 1 : max_k + 1]
    duplicate_rows = int(np.sum(np.any(ordered <= EPS, axis=1)))
    return ordered, duplicate_rows


def twonn(distance: np.ndarray) -> dict[str, Any]:
    ordered, duplicate_rows = nonzero_knn(distance, 2)
    valid = (ordered[:, 0] > EPS) & (ordered[:, 1] > ordered[:, 0] + EPS)
    ratios = ordered[valid, 1] / ordered[valid, 0]
    estimate = float(1.0 / np.mean(np.log(ratios))) if len(ratios) and np.mean(np.log(ratios)) > EPS else None
    return {"estimate": estimate, "valid_rows": int(np.sum(valid)), "duplicate_or_tied_rows": int(len(distance) - np.sum(valid)), "assumption": "locally homogeneous Poisson sampling; boundary, mixtures, curvature, and noise violate it"}


def knn_mle(distance: np.ndarray, ks: list[int]) -> list[dict[str, Any]]:
    ordered, duplicate_rows = nonzero_knn(distance, max(ks))
    output = []
    for k in ks:
        radii = ordered[:, :k]
        valid = (radii[:, -1] > EPS) & np.all(radii[:, :-1] > EPS, axis=1)
        logs = np.log(radii[valid, -1, None] / radii[valid, :-1])
        denominator = np.sum(logs, axis=1)
        local = (k - 1) / denominator[denominator > EPS]
        output.append({"k": k, "estimate": float(np.mean(local)) if len(local) else None, "local_estimate": summary(local) if len(local) else None, "valid_rows": int(len(local)), "duplicate_rows_seen": duplicate_rows})
    return output


def correlation_dimension(distance: np.ndarray, radii_count: int = 18) -> dict[str, Any]:
    upper = distance[np.triu_indices(len(distance), 1)]
    positive = upper[upper > EPS]
    if len(positive) < 10:
        return {"status": "insufficient-positive-pair-distances"}
    low, high = np.quantile(positive, [.03, .55])
    radii = np.geomspace(low, high, radii_count)
    counts = np.array([np.mean(upper <= radius) for radius in radii])
    valid = (counts > 0) & (counts < .8)
    x, y = np.log(radii[valid]), np.log(counts[valid])
    slopes = np.diff(y) / np.diff(x)
    # A reported window is only a diagnostic: choose the longest adjacent run
    # whose slopes vary by at most 20% around their median.
    best: tuple[int, int] | None = None
    for start in range(len(slopes)):
        for end in range(start + 2, len(slopes) + 1):
            part = slopes[start:end]
            median = float(np.median(part))
            if median > EPS and np.max(np.abs(part - median)) <= .2 * median and (best is None or end - start > best[1] - best[0]):
                best = (start, end)
    window = None
    if best is not None:
        start, end = best
        window = {"radius_range": [float(np.exp(x[start])), float(np.exp(x[end]))], "slope": float(np.polyfit(x[start : end + 1], y[start : end + 1], 1)[0]), "intervals": end - start}
    return {"radii": radii.tolist(), "pair_mass": counts.tolist(), "local_log_slopes": slopes.tolist(), "stable_window": window, "warning": "window selection is exploratory and can be manufactured by finite range/noise; no stable window means no correlation-dimension estimate"}


def components_from_knn(distance: np.ndarray, k: int) -> dict[str, Any]:
    n = len(distance)
    parent = list(range(n))
    def find(x: int) -> int:
        while parent[x] != x:
            parent[x] = parent[parent[x]]
            x = parent[x]
        return x
    def union(a: int, b: int) -> None:
        a, b = find(a), find(b)
        if a != b:
            parent[b] = a
    for i in range(n):
        for j in np.argsort(distance[i])[1 : k + 1]:
            union(i, int(j))
    sizes = sorted(Counter(find(i) for i in range(n)).values(), reverse=True)
    return {"k": k, "component_count": len(sizes), "largest_component_fraction": sizes[0] / n, "component_sizes": sizes}


def mass_radius(distance: np.ndarray) -> dict[str, Any]:
    radii = np.sort(distance, axis=1)[:, 1:]
    rows = []
    for mass in [.05, .1, .2, .4]:
        index = max(0, min(radii.shape[1] - 1, math.ceil(mass * (len(distance) - 1)) - 1))
        rows.append({"mass": mass, "radius": summary(radii[:, index])})
    medoid = int(np.argmin(np.sum(distance, axis=1)))
    medoid_radii = np.sort(distance[medoid])[1:]
    return {"per_point_mass_radii": rows, "empirical_medoid_index": medoid, "medoid_mass_radius": [{"mass": mass, "radius": float(medoid_radii[max(0, min(len(medoid_radii) - 1, math.ceil(mass * (len(distance) - 1)) - 1))])} for mass in [.1, .25, .5, .9]], "meaning": "finite-sample concentration summaries, not a density level set or confidence body"}


def nearest_distance(query: np.ndarray, reference: np.ndarray) -> np.ndarray:
    return np.sqrt(np.min(np.maximum(np.sum(query * query, axis=1)[:, None] + np.sum(reference * reference, axis=1)[None, :] - 2 * query @ reference.T, 0.0), axis=1))


def wilson_interval(hits: int, total: int) -> list[float]:
    if total == 0:
        return [float("nan"), float("nan")]
    z = 1.96
    proportion = hits / total
    denominator = 1 + z * z / total
    center = (proportion + z * z / (2 * total)) / denominator
    half_width = z * math.sqrt(proportion * (1 - proportion) / total + z * z / (4 * total * total)) / denominator
    return [center - half_width, center + half_width]


def calibrated_mass_region(points: np.ndarray, seed: int, alpha: float = .1, q_probes: int = 2048) -> dict[str, Any]:
    """Split-conformal radius around a finite sample, plus a separate chart audit.

    The first coverage is against an exchangeable future draw from the same
    sampling law.  The second is Monte Carlo coverage under an intentionally
    artificial, bounded coordinate-box reference measure Q; it says nothing
    about population support or probability mass.
    """
    n = len(points)
    if n < 24:
        return {"status": "insufficient-samples", "n": n, "need_at_least": 24}
    rng = np.random.default_rng(seed)
    permutation = rng.permutation(n)
    train_count = n // 2
    calibration_count = n // 4
    train = points[permutation[:train_count]]
    calibration = points[permutation[train_count : train_count + calibration_count]]
    holdout = points[permutation[train_count + calibration_count :]]
    calibration_distances = np.sort(nearest_distance(calibration, train))
    order = math.ceil((len(calibration_distances) + 1) * (1 - alpha))
    if order > len(calibration_distances):
        return {"status": "calibration-too-small-for-alpha", "calibration_count": len(calibration_distances), "alpha": alpha}
    radius = float(calibration_distances[order - 1])
    holdout_hits = int(np.sum(nearest_distance(holdout, train) <= radius))
    # Q is deliberately data-derived and coordinate dependent: a box around the
    # training sample expanded by 10% of each observed coordinate range.
    lower, upper = np.min(train, axis=0), np.max(train, axis=0)
    span = upper - lower
    fallback = np.maximum(np.std(train, axis=0), 1.0)
    span = np.where(span > EPS, span, fallback)
    q_lower, q_upper = lower - .1 * span, upper + .1 * span
    probes = rng.uniform(q_lower, q_upper, size=(q_probes, points.shape[1]))
    q_hits = int(np.sum(nearest_distance(probes, train) <= radius))
    return {"status": "ok", "split": {"train": len(train), "calibration": len(calibration), "holdout": len(holdout), "seed": seed}, "alpha": alpha, "radius": radius, "law_mass_coverage": {"holdout_fraction": holdout_hits / len(holdout), "wilson_95_interval": wilson_interval(holdout_hits, len(holdout)), "meaning": "independent holdout frequency only; split-conformal marginal coverage requires exchangeable IID rows from the declared law, not a selected/mixed stratum"}, "reference_chart_coverage_Q": {"measure": "uniform on the coordinatewise training bounding box expanded by 10% of each observed range", "probe_count": q_probes, "fraction": q_hits / q_probes, "wilson_95_monte_carlo_interval": wilson_interval(q_hits, q_probes), "meaning": "coverage of this arbitrary bounded chart measure Q only; neither support coverage nor probability mass under the generator law"}}


def assess(points: np.ndarray, ks: list[int], seed: int = 0) -> dict[str, Any]:
    if len(points) < max(12, max(ks) + 2):
        return {"status": "insufficient-samples", "n": len(points), "need_at_least": max(12, max(ks) + 2)}
    distance = pairwise_distances(points)
    return {"status": "ok", "n": len(points), "global_pca": global_pca(points), "local_pca": [local_pca(points, k) for k in ks], "twonn": twonn(distance), "knn_mle": knn_mle(distance, ks), "correlation_dimension": correlation_dimension(distance), "knn_connectivity": [components_from_knn(distance, k) for k in ks], "mass_radius": mass_radius(distance), "calibrated_mass_region": calibrated_mass_region(points, seed)}


def polygon_duals(vertices: np.ndarray) -> np.ndarray:
    # Translation is explicitly removed before H conversion.  For a CCW polygon,
    # (edge_y, -edge_x) is an outward normal, and n.x <= n.v is its H inequality.
    vertices = vertices - np.mean(vertices, axis=0)
    edge = np.roll(vertices, -1, axis=0) - vertices
    normal = np.column_stack((edge[:, 1], -edge[:, 0]))
    support = np.sum(normal * vertices, axis=1)
    if np.any(support <= EPS):
        raise ValueError("centered polygon does not contain origin strictly")
    return normal / support[:, None]


def parse_factor_id(sample_id: str) -> str:
    suffix = "/factor="
    if suffix not in sample_id:
        raise ValueError(f"factor sample id lacks factor role: {sample_id}")
    return sample_id.rsplit(suffix, 1)[0]


def load_product_dual_views(path: Path) -> tuple[dict[int, dict[str, np.ndarray]], dict[int, dict[str, dict[str, np.ndarray]]], dict[str, Any]]:
    grouped: dict[str, dict[str, dict[str, Any]]] = defaultdict(dict)
    with path.open() as handle:
        for line in handle:
            row = json.loads(line)
            if row.get("schema") != "factor-shape-row-v1":
                raise ValueError("expected factor-shape-row-v1")
            grouped[parse_factor_id(row["sample_id"])][row["factor_role"]] = row
    views: dict[int, dict[str, list[np.ndarray]]] = defaultdict(lambda: defaultdict(list))
    population_views: dict[int, dict[str, dict[str, list[np.ndarray]]]] = defaultdict(lambda: defaultdict(lambda: defaultdict(list)))
    dropped = 0
    for _, pair in grouped.items():
        if set(pair) != {"q", "p"}:
            dropped += 1
            continue
        q, p = (polygon_duals(np.asarray(pair[role]["vertices_ccw"], dtype=float)) for role in ("q", "p"))
        f = len(q) + len(p)
        population = str(pair["q"].get("population", pair["q"].get("law")))
        if population != str(pair["p"].get("population", pair["p"].get("law"))):
            raise ValueError("q/p factor populations disagree")
        dual = np.vstack((np.column_stack((q, np.zeros((len(q), 2)))), np.column_stack((np.zeros((len(p), 2)), p))))
        raw = dual.reshape(-1)
        lex = dual[np.lexsort((dual[:, 3], dual[:, 2], dual[:, 1], dual[:, 0]))].reshape(-1)
        gram = dual @ dual.T
        views[f]["fixed_order_dual"].append(raw)
        views[f]["facet_permutation_canonical_dual"].append(lex)
        views[f]["orthogonal_and_permutation_invariant_gram_spectrum"].append(np.linalg.eigvalsh(gram)[::-1])
        population_views[f][population]["fixed_order_dual"].append(raw)
        population_views[f][population]["facet_permutation_canonical_dual"].append(lex)
        population_views[f][population]["orthogonal_and_permutation_invariant_gram_spectrum"].append(np.linalg.eigvalsh(gram)[::-1])
    stacked = {f: {name: np.vstack(rows) for name, rows in view.items()} for f, view in views.items()}
    population_stacked = {f: {population: {name: np.vstack(rows) for name, rows in by_view.items()} for population, by_view in by_population.items()} for f, by_population in population_views.items()}
    return stacked, population_stacked, {"pairs_seen": len(grouped), "incomplete_pairs_dropped": dropped}


def synthetic_cases(seed: int, n: int) -> dict[str, tuple[np.ndarray, dict[str, Any]]]:
    rng = np.random.default_rng(seed)
    plane = np.column_stack((rng.normal(size=(n, 2)), np.zeros((n, 6))))
    angles = rng.uniform(0, 2 * np.pi, n)
    sphere = np.column_stack((np.cos(angles), np.sin(angles), .03 * rng.normal(size=(n, 4))))
    t = rng.uniform(-1, 1, n)
    curve = np.column_stack((t, t * t, np.sin(3 * t), np.zeros((n, 3))))
    mixture = np.vstack((rng.normal(loc=-2, scale=.35, size=(n // 2, 2)), rng.normal(loc=2, scale=.35, size=(n - n // 2, 2))))
    mixture = np.column_stack((mixture, np.zeros((n, 4))))
    duplicates = plane.copy(); duplicates[: n // 5] = duplicates[0]
    anisotropic = np.column_stack((rng.normal(size=n), .03 * rng.normal(size=n), .003 * rng.normal(size=n), np.zeros((n, 5))))
    boundary = rng.uniform(0, 1, size=(n, 2)); boundary = np.column_stack((boundary, np.zeros((n, 6))))
    return {"plane_2d": (plane, {"known": "2-dimensional linear plane"}), "circle_1d_with_noise": (sphere, {"known": "1-dimensional curved manifold plus noise"}), "curve_1d": (curve, {"known": "1-dimensional curved embedding"}), "two_component_plane_mixture": (mixture, {"known": "two 2-dimensional separated components"}), "duplicate_contaminated_plane": (duplicates, {"known": "2-dimensional plane with exact duplicates"}), "anisotropic_noisy_line": (anisotropic, {"known": "1-dimensional dominant line with two noise scales"}), "bounded_square": (boundary, {"known": "2-dimensional manifold with boundary"})}


def calibration(seed: int, n: int, ks: list[int]) -> dict[str, Any]:
    output = {}
    for name, (points, known) in synthetic_cases(seed, n).items():
        output[name] = {"known_structure": known, "assessment": assess(points, ks, seed + len(output))}
    return output


def revision() -> str | None:
    try:
        return subprocess.check_output(["git", "rev-parse", "HEAD"], text=True).strip()
    except (OSError, subprocess.CalledProcessError):
        return None


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--factor-shapes", type=Path, help="hydrated generator-zoo factor-shapes.jsonl for the real smoke")
    parser.add_argument("--out-dir", type=Path, required=True)
    parser.add_argument("--seed", type=int, default=20260715)
    parser.add_argument("--calibration-n", type=int, default=360)
    parser.add_argument("--ks", default="8,12,20")
    args = parser.parse_args()
    ks = [int(value) for value in args.ks.split(",")]
    if min(ks) < 3 or len(set(ks)) != len(ks):
        raise ValueError("ks must be distinct integers at least 3")
    start = time.monotonic()
    report: dict[str, Any] = {"schema": SCHEMA, "target_free": True, "revision": revision(), "analyzer_sha256": sha256(Path(__file__)), "calibration": calibration(args.seed, args.calibration_n, ks), "topology_disposition": {"persistent_homology": "deferred", "reason": "This copy-local packet declares only numpy. No lightweight persistent-homology dependency with a calibrated coefficient/filtration/noise contract is available here. Graph connectivity is retained as a non-topological neighborhood diagnostic; UMAP/t-SNE are intentionally absent as they cannot establish dimension or topology."}, "secondary_dispositions": {"generator_map_local_rank": "deferred: the reviewed generator maps and their conditioning/rejection semantics are not yet a common differentiable contract; finite-difference rank would otherwise silently measure implementation choices.", "density_cluster_tree": "deferred: stable cluster trees require a selected density estimator, density level/noise calibration, and a population-scale per-law sample, none of which this smoke provides."}, "interpretation": {"supported": "Within a declared fixed-F/view stratum, agreement of estimators across calibration-relevant knobs is descriptive evidence against a gross ambient-filling interpretation. Split-calibrated radii can describe held-out empirical law mass only under the stated exchangeability contract, separately from declared-Q chart coverage.", "prohibited": "No estimator value proves an intrinsic dimension, a manifold population, a quotient dimension, a topology, a density body, or support coverage. Do not pool F, call mass-radius a confidence body, or compare views as though they implemented the same quotient."}, "larger_n_resolves": ["separate local neighborhoods from boundary/mixture effects and make k-range stability checkable", "permits held-out or bootstrap stability of estimator ranges and split-calibrated law-mass coverage within each F/view/population stratum", "reduces Monte Carlo/holdout uncertainty for declared-Q chart coverage and improves graph-component persistence checks across k; it cannot resolve an unchosen quotient or a missing topology contract"]}
    if args.factor_shapes is not None:
        views, population_views, source = load_product_dual_views(args.factor_shapes)
        report["real_smoke"] = {"source": {"repository_relative_path": repository_relative_display(args.factor_shapes), "sha256": sha256(args.factor_shapes)}, "pairing": source, "fixed_f_mixture_diagnostics": {str(f): {name: assess(points, ks, args.seed + f) for name, points in sorted(by_view.items())} for f, by_view in sorted(views.items())}, "fixed_f_population_strata": {str(f): {population: {name: assess(points, ks, args.seed + f) for name, points in sorted(by_view.items())} for population, by_view in sorted(by_population.items())} for f, by_population in sorted(population_views.items())}, "scope": "generator-zoo accepted product factors only. Fixed-F mixture diagnostics intentionally retain population multimodality as a diagnostic; population strata are reported separately and are too small for the declared k range. Neither is a law-level dimension claim."}
    report["runtime_seconds"] = time.monotonic() - start
    args.out_dir.mkdir(parents=True, exist_ok=True)
    (args.out_dir / "report.json").write_text(json.dumps(report, indent=2, sort_keys=True) + "\n")
    print(json.dumps({"out": str(args.out_dir / "report.json"), "runtime_seconds": report["runtime_seconds"]}, sort_keys=True))


if __name__ == "__main__":
    main()
