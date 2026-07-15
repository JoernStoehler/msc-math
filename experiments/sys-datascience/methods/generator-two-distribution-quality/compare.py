#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = ["numpy"]
# ///

"""Target-free, side-count-stratified comparison of two polygon laws.

The packet deliberately keeps the methods small and copy-local.  No target or
capacity value is read.  Rows are compared within a fixed side-count stratum;
the final table retains per-stratum rows; it never pools side-count strata.
"""

from __future__ import annotations

import argparse
from collections import defaultdict
import hashlib
import json
import math
from pathlib import Path
from typing import Any

import numpy as np


SCHEMA = "factor-shape-row-v1"
EPS = 1e-12
VIEW_NAMES = ("raw_ordered", "canonicalized", "chord_multiset_quotient")
DESCRIPTIVE_FLOOR_ROWS_PER_SIDE = 10


def _stable_int(*parts: Any) -> int:
    payload = json.dumps(parts, sort_keys=True, separators=(",", ":")).encode()
    return int.from_bytes(hashlib.sha256(payload).digest()[:8], "little")


def _area(v: np.ndarray) -> float:
    return float(0.5 * np.sum(v[:, 0] * np.roll(v[:, 1], -1) - v[:, 1] * np.roll(v[:, 0], -1)))


def _validate_vertices(raw: Any, n: int, context: str) -> np.ndarray:
    v = np.asarray(raw, dtype=float)
    if v.shape != (n, 2) or not np.all(np.isfinite(v)):
        raise ValueError(f"{context}: expected finite ({n},2) vertices")
    if _area(v) <= EPS:
        raise ValueError(f"{context}: vertices must be CCW and non-degenerate")
    edges = np.roll(v, -1, axis=0) - v
    turns = edges[:, 0] * np.roll(edges[:, 1], -1) - edges[:, 1] * np.roll(edges[:, 0], -1)
    if np.any(turns <= EPS):
        raise ValueError(f"{context}: vertices are not strict convex CCW")
    return v


def _normalize(v: np.ndarray) -> np.ndarray:
    # Translation and positive scale are gauge for the factor comparison.
    c = np.mean(v, axis=0)
    x = v - c
    return x / math.sqrt(_area(v))


def _rotate(x: np.ndarray, theta: float) -> np.ndarray:
    r = np.array([[math.cos(theta), -math.sin(theta)], [math.sin(theta), math.cos(theta)]])
    return x @ r.T


def _cyclic_canonical(x: np.ndarray) -> np.ndarray:
    # Input is CCW.  The lexicographically smallest cyclic start removes the
    # arbitrary vertex index while retaining orientation.
    keys = [tuple(np.round(np.r_[np.roll(x, -i, axis=0)], 14).ravel()) for i in range(len(x))]
    return np.roll(x, -min(range(len(x)), key=lambda i: keys[i]), axis=0)


def _edge_turn_canonical(x: np.ndarray) -> np.ndarray:
    """Rotation-invariant cyclic descriptor, including regular polygons.

    Edge lengths and positive exterior turns are invariant under a common
    rotation.  Choosing the lexicographically smallest cyclic row removes the
    arbitrary starting facet.  The descriptor retains CCW orientation (a
    reflection reverses the sequence) and does not require a principal-axis
    eigenvalue gap.
    """
    edges = np.roll(x, -1, axis=0) - x
    lengths = np.linalg.norm(edges, axis=1)
    directions = np.arctan2(edges[:, 1], edges[:, 0])
    exterior = np.mod(np.roll(directions, -1) - directions, 2.0 * math.pi)
    descriptor = np.column_stack((lengths, exterior))
    return _cyclic_canonical(descriptor)


def _views(v: np.ndarray) -> dict[str, np.ndarray]:
    x = _normalize(v)
    ordered = x.ravel()
    canonical = _edge_turn_canonical(x).ravel()
    # Pairwise chord distances are invariant under translation, rotation,
    # reflection, and vertex-index assignment.  This is the strongest quotient
    # view; it must not be described as preserving orientation.
    d = np.linalg.norm(x[:, None, :] - x[None, :, :], axis=2)
    # Sorting removes the arbitrary edge/vertex assignment as well as cyclic
    # indexing.  It intentionally forgets adjacency; that loss is disclosed
    # by the view name and interpretation boundary.
    quotient = np.sort(d[np.triu_indices(len(x), 1)])
    return {"raw_ordered": ordered, "canonicalized": canonical, "chord_multiset_quotient": quotient}


def load_rows(path: Path) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    seen: set[str] = set()
    with path.open() as handle:
        for lineno, line in enumerate(handle, 1):
            if not line.strip():
                continue
            row = json.loads(line)
            if row.get("schema") != SCHEMA:
                raise ValueError(f"line {lineno}: expected schema {SCHEMA!r}")
            sid, law, n = row.get("sample_id"), row.get("population", row.get("law")), row.get("side_count")
            if not isinstance(sid, str) or not sid or sid in seen:
                raise ValueError(f"line {lineno}: sample_id must be unique and non-empty")
            if not isinstance(law, str) or not law or not isinstance(n, int) or n < 3:
                raise ValueError(f"line {lineno}: invalid population or side_count")
            v = _validate_vertices(row.get("vertices_ccw", row.get("vertices")), n, f"line {lineno}")
            row = dict(row)
            row["population"] = law
            row["_views"] = _views(v)
            rows.append(row)
            seen.add(sid)
    if not rows:
        raise ValueError(f"{path}: no valid rows")
    return rows


def _pairwise(a: np.ndarray, b: np.ndarray | None = None) -> np.ndarray:
    if b is None:
        b = a
    return np.linalg.norm(a[:, None, :] - b[None, :, :], axis=2)


def _energy(a: np.ndarray, b: np.ndarray) -> float:
    # V-statistic is symmetric and finite-sample self-zero, but no negative-
    # type claim is made for the quotient view.
    return float(2 * np.mean(_pairwise(a, b)) - np.mean(_pairwise(a)) - np.mean(_pairwise(b)))


def _mmd(a: np.ndarray, b: np.ndarray, bandwidth: float) -> float:
    bw2 = max(bandwidth * bandwidth, EPS)
    kaa = np.exp(-_pairwise(a) ** 2 / (2 * bw2))
    kbb = np.exp(-_pairwise(b) ** 2 / (2 * bw2))
    kab = np.exp(-_pairwise(a, b) ** 2 / (2 * bw2))
    return float(np.mean(kaa) + np.mean(kbb) - 2 * np.mean(kab))


def _sliced_wasserstein(a: np.ndarray, b: np.ndarray, seed: int, projections: int = 32) -> float:
    rng = np.random.default_rng(seed)
    p = rng.normal(size=(projections, a.shape[1]))
    p /= np.linalg.norm(p, axis=1, keepdims=True)
    out = []
    # Quantile interpolation makes unequal sample sizes explicit rather than
    # silently truncating one sample.
    for direction in p:
        x, y = a @ direction, b @ direction
        q = np.linspace(0.0, 1.0, max(len(x), len(y)))
        out.append(np.mean(np.abs(np.quantile(x, q) - np.quantile(y, q))))
    return float(np.mean(out))


def _group_labels(rows: list[dict[str, Any]], seed: int) -> np.ndarray:
    return np.array([_stable_int(seed, r.get("pair_bucket", r["sample_id"])) % 5 for r in rows], dtype=int)


def _grouped_classifiers(a: np.ndarray, b: np.ndarray, rows_a: list[dict[str, Any]], rows_b: list[dict[str, Any]], seed: int) -> dict[str, Any]:
    x = np.vstack([a, b]); y = np.r_[np.zeros(len(a), dtype=int), np.ones(len(b), dtype=int)]
    groups = np.r_[_group_labels(rows_a, seed), _group_labels(rows_b, seed)]
    fold_scores: list[float] = []
    knn_scores: list[float] = []
    for g in sorted(set(groups)):
        test = groups == g
        train = ~test
        if not np.any(test) or len(set(y[train])) < 2:
            continue
        means = [np.mean(x[train & (y == c)], axis=0) for c in (0, 1)]
        pred = np.argmin(np.stack([np.linalg.norm(x[test] - means[0], axis=1), np.linalg.norm(x[test] - means[1], axis=1)]), axis=0)
        fold_scores.append(float(np.mean(pred == y[test])))
        train_x, train_y = x[train], y[train]
        d = _pairwise(x[test], train_x)
        k = min(5, len(train_x))
        nearest = np.argpartition(d, kth=k - 1, axis=1)[:, :k]
        knn_pred = (np.mean(train_y[nearest], axis=1) >= 0.5).astype(int)
        knn_scores.append(float(np.mean(knn_pred == y[test])))
    return {
        "grouped_folds": len(fold_scores),
        "nearest_centroid_accuracy": float(np.mean(fold_scores)) if fold_scores else None,
        "knn5_accuracy": float(np.mean(knn_scores)) if knn_scores else None,
        "interpretation": "diagnostic separability only; not coverage evidence",
    }


def _mixing(a: np.ndarray, b: np.ndarray) -> dict[str, float]:
    x = np.vstack([a, b]); y = np.r_[np.zeros(len(a), dtype=int), np.ones(len(b), dtype=int)]
    d = _pairwise(x); np.fill_diagonal(d, np.inf)
    nearest = np.argmin(d, axis=1)
    same = float(np.mean(y[nearest] == y))
    return {"cross_nearest_neighbor_mixing": 1.0 - same, "within_nearest_fraction": same}


def _precision_recall(a: np.ndarray, b: np.ndarray, quantile: float = 0.9) -> dict[str, float]:
    def one(src: np.ndarray, dst: np.ndarray) -> tuple[float, float]:
        own = _pairwise(src); np.fill_diagonal(own, np.inf)
        radius = np.quantile(np.min(own, axis=1), quantile) if len(src) > 1 else 0.0
        nearest = np.min(_pairwise(dst, src), axis=1)
        precision = float(np.mean(nearest <= radius)) if len(dst) else 0.0
        return precision, radius
    p_ab, ra = one(a, b); p_ba, rb = one(b, a)
    # Coverage/recall uses each source law's local scale; the direction labels
    # are explicit to avoid conflating precision with support coverage.
    return {"precision_b_relative_to_a": p_ab, "precision_a_relative_to_b": p_ba, "radius_a": ra, "radius_b": rb, "coverage_a_by_b": p_ba, "coverage_b_by_a": p_ab}


def _region_overlap(a: np.ndarray, b: np.ndarray, quantile: float = 0.9) -> dict[str, float]:
    def radius(x: np.ndarray) -> tuple[np.ndarray, float]:
        med = x[np.argmin(np.sum(_pairwise(x), axis=1))]
        return med, float(np.quantile(np.linalg.norm(x - med, axis=1), quantile))
    ma, ra = radius(a); mb, rb = radius(b)
    return {"quantile_region_overlap": float(0.5 * (float(np.linalg.norm(ma - mb) <= ra) + float(np.linalg.norm(ma - mb) <= rb))), "center_distance": float(np.linalg.norm(ma - mb)), "radius_a": ra, "radius_b": rb}


def _js_occupancy(a: np.ndarray, b: np.ndarray, bins: int = 8, smoothing: float = 0.5) -> float:
    # A one-dimensional projection keeps occupancy interpretable and avoids an
    # exponentially sparse high-dimensional grid.
    direction = np.arange(1, a.shape[1] + 1, dtype=float); direction /= np.linalg.norm(direction)
    x, y = a @ direction, b @ direction
    edges = np.quantile(np.r_[x, y], np.linspace(0, 1, bins + 1)); edges = np.unique(edges)
    if len(edges) < 3:
        return 0.0
    px = np.histogram(x, edges)[0].astype(float) + smoothing
    py = np.histogram(y, edges)[0].astype(float) + smoothing
    px /= px.sum(); py /= py.sum(); m = 0.5 * (px + py)
    return float(0.5 * np.sum(px * np.log(px / m)) + 0.5 * np.sum(py * np.log(py / m)))


def _sample_size_status(n_a: int, n_b: int) -> tuple[str, str]:
    if n_a <= 15 and n_b <= 15:
        return "uncalibrated_descriptive", "descriptive_only_do_not_treat_as_estimate_or_ranking"
    if n_a >= DESCRIPTIVE_FLOOR_ROWS_PER_SIDE and n_b >= DESCRIPTIVE_FLOOR_ROWS_PER_SIDE:
        return "descriptive_floor_met_uncalibrated", "descriptive_only_do_not_treat_as_estimate_or_ranking"
    return "below_descriptive_floor", "descriptive_only_do_not_treat_as_estimate_or_ranking"


def compare_stratum(a_rows: list[dict[str, Any]], b_rows: list[dict[str, Any]], view: str, seed: int) -> dict[str, Any]:
    a = np.stack([r["_views"][view] for r in a_rows]); b = np.stack([r["_views"][view] for r in b_rows])
    cross = _pairwise(a, b)
    med = float(np.median(cross))
    status, disposition = _sample_size_status(len(a), len(b))
    out: dict[str, Any] = {"view": view, "n_a": len(a), "n_b": len(b), "sample_size_status": status, "disposition": disposition, "energy_v": _energy(a, b), "mmd_rbf": {str(mult): _mmd(a, b, max(med * mult, EPS)) for mult in (0.5, 1.0, 2.0, 4.0)}, "sliced_wasserstein": _sliced_wasserstein(a, b, seed), "median_cross_distance": med}
    out.update(_mixing(a, b)); out.update(_precision_recall(a, b)); out.update(_region_overlap(a, b)); out["occupancy_js"] = _js_occupancy(a, b)
    out["classifier"] = _grouped_classifiers(a, b, a_rows, b_rows, seed)
    out["warning"] = "representation-induced separation if raw/canonicalized changes disappear in chord_multiset_quotient"
    return out


def analyze(rows: list[dict[str, Any]], left: str, right: str, seed: int) -> dict[str, Any]:
    by_law: dict[str, list[dict[str, Any]]] = defaultdict(list)
    for r in rows: by_law[r["population"]].append(r)
    if left not in by_law or right not in by_law: raise ValueError(f"both populations required; available={sorted(by_law)}")
    all_n = sorted({r["side_count"] for r in by_law[left]} | {r["side_count"] for r in by_law[right]})
    by_n: dict[int, tuple[list[dict[str, Any]], list[dict[str, Any]]]] = {}
    inventory = []
    for n in all_n:
        a, b = [r for r in by_law[left] if r["side_count"] == n], [r for r in by_law[right] if r["side_count"] == n]
        status, disposition = _sample_size_status(len(a), len(b)) if a and b else ("unpaired", "omitted_from_pairwise_metrics")
        inventory.append({"side_count": n, "left_count": len(a), "right_count": len(b), "paired": bool(a and b), "sample_size_status": status, "disposition": disposition})
        if a and b: by_n[n] = (a, b)
    strata = []
    for n, (a, b) in by_n.items():
        comparisons = [compare_stratum(a, b, view, _stable_int(seed, n, view)) for view in VIEW_NAMES]
        energies = {c["view"]: c["energy_v"] for c in comparisons}
        quotient_ratio = energies["chord_multiset_quotient"] / max(abs(energies["raw_ordered"]), EPS)
        if quotient_ratio < 0.5:
            note = "representation-induced separation likely: chord multiset quotient is less than half raw energy"
        elif energies["canonicalized"] < 0.5 * max(abs(energies["raw_ordered"]), EPS):
            note = "frame/index-induced component: canonicalization removes at least half raw energy"
        else:
            note = "separation persists across the chord multiset quotient; not representation-only"
        stratum_status, stratum_disposition = _sample_size_status(len(a), len(b))
        strata.append({"side_count": n, "n_a": len(a), "n_b": len(b), "sample_size_status": stratum_status, "disposition": stratum_disposition, "comparisons": comparisons, "representation_diagnostic": {"energy_by_view": energies, "quotient_to_raw_energy_ratio": quotient_ratio, "note": note}})
    return {"schema": "two-distribution-quality-report-v1", "left_population": left, "right_population": right, "seed": seed, "sample_size_contract": {"descriptive_floor_rows_per_side": DESCRIPTIVE_FLOOR_ROWS_PER_SIDE, "status_values": ["uncalibrated_descriptive", "descriptive_floor_met_uncalibrated", "below_descriptive_floor"], "meaning": "This packet has no empirical calibration of metric estimates; statuses are descriptive sample-size flags only."}, "stratum_inventory": inventory, "strata": strata, "disposition": {"implemented": ["energy distance", "MMD bandwidth ladder", "sliced Wasserstein", "grouped nearest-centroid and kNN diagnostics", "cross-nearest-neighbor mixing", "precision/recall coverage-density", "quantile-region overlap", "smoothed occupancy Jensen-Shannon"], "deferred": ["full optimal-transport Wasserstein", "learned classifier (dependency-free grouped nearest-centroid retained instead)", "high-dimensional occupancy grids (sparse and bin-sensitive)", "actual cyclic, dihedral, or optimal vertex-assignment distance; chord_multiset_quotient is a lossy adjacency-free surrogate"], "claim_boundary": "No metric is a target-transfer score or a population law ranking; retained strata are descriptive-only and uncalibrated."}}


def write_outputs(report: dict[str, Any], out_dir: Path) -> None:
    out_dir.mkdir(parents=True, exist_ok=True)
    (out_dir / "report.json").write_text(json.dumps(report, indent=2, sort_keys=True) + "\n")
    lines = ["side_count\tview\tn_a\tn_b\tsample_size_status\tdisposition\tenergy_v\tsliced_wasserstein\tmedian_cross_distance\tcross_nearest_neighbor_mixing\toccupancy_js\tnearest_centroid_accuracy\tknn5_accuracy"]
    for s in report["strata"]:
        for c in s["comparisons"]:
            clf = c["classifier"]
            vals = [s["side_count"], c["view"], c["n_a"], c["n_b"], c["sample_size_status"], c["disposition"], c["energy_v"], c["sliced_wasserstein"], c["median_cross_distance"], c["cross_nearest_neighbor_mixing"], c["occupancy_js"], clf["nearest_centroid_accuracy"], clf["knn5_accuracy"]]
            lines.append("\t".join("NA" if v is None else f"{v:.10g}" if isinstance(v, float) else str(v) for v in vals))
    (out_dir / "comparison.tsv").write_text("\n".join(lines) + "\n")
    guide = [
        "method\tdetects\tknob_or_symmetry\tsample_size_dependence\tfailure_case",
        "energy_v\tglobal metric displacement\tsymmetric, self-zero V-statistic\tquadratic pair cost; unstable for tiny strata\tquotient distance need not be negative type",
        "mmd_rbf\tkernel-scale-sensitive location/shape difference\tbandwidth 0.5,1,2,4 times median\tmedian bandwidth noisy for small n\tbandwidth can hide narrow or broad changes",
        "sliced_wasserstein\tprojected one-dimensional transport\t32 seeded projections\tquantile noise and projection Monte Carlo\tprojection misses directions; not full OT",
        "grouped_nearest_centroid\theld-out linear-centroid separability\tdeterministic grouped folds\trequires at least two valid training groups\taccuracy is not support coverage",
        "knn5\tlocal neighborhood separability\tk=5, grouped folds\tk and neighborhoods need enough rows\tsample-size and density sensitive",
        "cross_nearest_neighbor_mixing\tlocal interpenetration versus separation\tnearest neighbor\tvery noisy below roughly 10 per side\tunequal density and side-count mixing bias",
        "precision_recall\tfinite-sample local support coverage/density\twithin-law 90% radius\tlocal radius has high variance for small n\tnot a population confidence region",
        "quantile_region_overlap\tcoarse medoid-region overlap\t90% radius\tmedoid and quantile unstable for tiny n\telliptic/nonconvex shapes reduced to one radius",
        "occupancy_js\tcoarse projected occupancy imbalance\t8 bins, additive smoothing 0.5\toccupancy sparse when n is small\tbin/projection sensitive; no high-dimensional grid",
        "chord_multiset_quotient\tassignment-invariant chord-length multiset difference\ttranslation/scale/rotation/reflection invariant; adjacency discarded\tquadratic pair construction; small strata remain noisy\tnot an optimal cyclic/dihedral assignment distance; non-isometric shapes can collide",
    ]
    (out_dir / "method-guide.tsv").write_text("\n".join(guide) + "\n")


def calibration(seed: int = 20260715) -> dict[str, Any]:
    """Calibrate qualitative method behavior on known Gaussian controls."""
    rng = np.random.default_rng(seed); d = 12; n = 80
    base = rng.normal(size=(n, d))
    mode_a = np.vstack([rng.normal(loc=-2, scale=0.35, size=(32, d)), rng.normal(loc=2, scale=0.35, size=(8, d))])
    mode_b = np.vstack([rng.normal(loc=-2, scale=0.35, size=(8, d)), rng.normal(loc=2, scale=0.35, size=(32, d))])
    outlier_b = base[40:].copy(); outlier_b[:2] += 10.0
    cases = {
        "same_law_split": (base[:40], base[40:]),
        "location_change": (base[:40], base[40:] + 1.5),
        "scale_change": (base[:40], 2.0 * base[40:]),
        "narrow_broad": (0.3 * base[:40], 2.0 * base[40:]),
        "mixture_weights_same_support": (mode_a, mode_b),
        "disjoint_modes": (base[:40] - 3, base[40:] + 3),
        "outliers": (base[:40], outlier_b),
        "high_dim_noise": (base[:40], np.c_[base[40:, :2], rng.normal(size=(40, d - 2)) * 4]),
    }
    rows = []
    for name, (a, b) in cases.items():
        rows.append({"case": name, "energy_v": _energy(a, b), "mmd_bandwidths": {str(m): _mmd(a, b, np.median(_pairwise(a, b)) * m) for m in (0.5, 1, 2, 4)}, "sliced_wasserstein": _sliced_wasserstein(a, b, _stable_int(seed, name)), "mixing": _mixing(a, b), "occupancy_js": _js_occupancy(a, b)})
    return {"schema": "two-distribution-calibration-v1", "purpose": "synthetic qualitative method controls; not empirical calibration of metric estimates", "seed": seed, "dimension": d, "sample_size_per_side": 40, "cases": rows, "interpretation": "same-law should be small on most views; location/scale/disjoint controls should separate; high-dimensional noise tests projection and bandwidth sensitivity. Qualitative checks are not inferential thresholds."}


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--input", type=Path)
    parser.add_argument("--left-population")
    parser.add_argument("--right-population")
    parser.add_argument("--out-dir", type=Path, default=Path("artifacts"))
    parser.add_argument("--seed", type=int, default=20260715)
    parser.add_argument("--calibrate", action="store_true")
    args = parser.parse_args()
    if args.calibrate:
        result = calibration(args.seed)
        result["provenance"] = {"implementation": "compare.py", "implementation_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest(), "command_contract": "two-distribution-quality-v1"}
        args.out_dir.mkdir(parents=True, exist_ok=True); (args.out_dir / "calibration.json").write_text(json.dumps(result, indent=2, sort_keys=True) + "\n"); return
    if args.input is None or args.left_population is None or args.right_population is None: parser.error("--input, --left-population, and --right-population are required")
    report = analyze(load_rows(args.input), args.left_population, args.right_population, args.seed)
    report["provenance"] = {
        "input_path": str(args.input),
        "input_sha256": hashlib.sha256(args.input.read_bytes()).hexdigest(),
        "implementation": "compare.py",
        "implementation_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
        "command_contract": "two-distribution-quality-v1",
    }
    write_outputs(report, args.out_dir)


if __name__ == "__main__":
    main()
