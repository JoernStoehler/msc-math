#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = ["numpy"]
# ///
"""Build a target-free multi-view atlas over reviewed factor populations."""

from __future__ import annotations

import argparse
from collections import defaultdict
import csv
import hashlib
import json
import math
from pathlib import Path
import subprocess
from typing import Any

import numpy as np

import shape_quality
from shape_quality import SCHEMA, SMALL_SAMPLE, Shape, bounded_selection_key, load_shapes, within_metrics

ATLAS_SCHEMA = "generator-distribution-atlas-next-v1"
FEATURES = ("log_perimeter", "covariance_anisotropy", "radial_rms", "angle_gap_cv")
SATURATION = (4, 8, 12, 24)


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def groups(shapes: list[Shape]) -> dict[tuple[int, str], list[Shape]]:
    result: dict[tuple[int, str], list[Shape]] = defaultdict(list)
    for shape in shapes:
        result[(shape.side_count, shape.law)].append(shape)
    return {key: sorted(value, key=bounded_selection_key) for key, value in sorted(result.items())}


def l2(left: np.ndarray, right: np.ndarray) -> float:
    """Fast declared-grid rotation quotient (circular cross-correlation)."""
    correlation = np.fft.ifft(np.fft.fft(left) * np.conjugate(np.fft.fft(right))).real / len(left)
    mse = np.mean(left * left) + np.mean(right * right) - 2.0 * np.max(correlation)
    return math.sqrt(max(0.0, float(mse)))


def distances(shapes: list[Shape], linf: bool = False) -> tuple[np.ndarray, np.ndarray]:
    n = len(shapes)
    a, b = np.zeros((n, n)), np.zeros((n, n))
    for i in range(n):
        for j in range(i + 1, n):
            a[i, j] = a[j, i] = l2(shapes[i].support, shapes[j].support)
            if linf:
                rolls = np.stack([np.roll(shapes[j].support, k) for k in range(len(shapes[j].support))])
                b[i, j] = b[j, i] = float(np.min(np.max(np.abs(rolls - shapes[i].support), axis=1)))
    return a, b


def invariant_features(shape: Shape) -> np.ndarray:
    v = shape.vertices
    edge = np.roll(v, -1, axis=0) - v
    lengths = np.linalg.norm(edge, axis=1)
    perimeter = float(np.sum(lengths))
    covariance = np.cov(v, rowvar=False)
    eigenvalues = np.linalg.eigvalsh(covariance)
    covariance_anisotropy = float(max(eigenvalues) / max(min(eigenvalues), 1e-15))
    turns = np.arctan2(np.abs(edge[:, 0] * np.roll(edge[:, 1], -1) - edge[:, 1] * np.roll(edge[:, 0], -1)), np.sum(edge * np.roll(edge, -1, axis=0), axis=1))
    return np.array([math.log(perimeter), covariance_anisotropy, math.sqrt(float(np.mean(np.sum(v * v, axis=1)))), float(np.std(turns) / np.mean(turns))])


def write_tsv(path: Path, rows: list[dict[str, Any]]) -> None:
    fields = sorted({field for row in rows for field in row})
    with path.open("w", newline="") as handle:
        writer = csv.DictWriter(handle, fieldnames=fields, delimiter="\t", lineterminator="\n")
        writer.writeheader()
        for row in rows:
            writer.writerow({field: "NA" if row.get(field) is None else row[field] for field in fields})


def within_view(shapes: list[Shape]) -> list[dict[str, Any]]:
    rows = []
    for (side, population), members in groups(shapes).items():
        d, inf = distances(members, linf=True)
        summary, summary_inf = within_metrics(d, 1e-9), within_metrics(inf, 1e-9)
        rows.append({"side_count": side, "population": population, "n": len(members), "sample_status": "small-sample" if len(members) < SMALL_SAMPLE else "descriptive", "pairwise_l2_mean": summary["pairwise_mean"], "pairwise_l2_median": summary["pairwise_median"], "nearest_l2_mean": summary["nearest_neighbor_mean"], "duplicate_l2_fraction": summary["duplicate_pair_fraction"], "positive_gram_spectrum_participation_ratio": summary["positive_gram_spectrum"]["positive_gram_spectrum_participation_ratio"], "negative_eigenmass_fraction": summary["positive_gram_spectrum"]["negative_eigenmass_fraction"], "pairwise_linf_mean": summary_inf["pairwise_mean"], "nearest_linf_mean": summary_inf["nearest_neighbor_mean"], "spectrum_definition": "participation ratio of positive eigenvalues of the centered squared-distance Gram matrix; not an intrinsic or metric dimension", "negative_eigenmass_definition": "fraction of absolute Gram eigenmass that is negative; diagnostic of non-Euclidean embedding, not a quality score"})
    return rows


def between_view(shapes: list[Shape]) -> list[dict[str, Any]]:
    by_side: dict[int, dict[str, list[Shape]]] = defaultdict(lambda: defaultdict(list))
    for shape in shapes:
        by_side[shape.side_count][shape.law].append(shape)
    rows = []
    for side, populations in sorted(by_side.items()):
        names = sorted(populations)
        for i, left in enumerate(names):
            for right in names[i + 1 :]:
                pair = sorted(populations[left], key=bounded_selection_key) + sorted(populations[right], key=bounded_selection_key)
                d, _ = distances(pair)
                split = len(populations[left])
                cross = d[:split, split:]
                lf = np.stack([invariant_features(x) for x in populations[left]])
                rf = np.stack([invariant_features(x) for x in populations[right]])
                rows.append({"side_count": side, "population_left": left, "population_right": right, "n_left": split, "n_right": len(populations[right]), "sample_status": "small-sample" if min(split, len(populations[right])) < SMALL_SAMPLE else "descriptive", "cross_l2_mean": float(np.mean(cross)), "cross_l2_median": float(np.median(cross)), "cross_l2_min": float(np.min(cross)), "raw_feature_centroid_separation": float(np.linalg.norm(np.mean(lf, axis=0) - np.mean(rf, axis=0))), "centroid_definition": "Euclidean distance between unstandardized four-feature centroids; anisotropy dominates scale and this is neither balanced evidence nor a score"})
    return rows


def overlap_view(shapes: list[Shape]) -> list[dict[str, Any]]:
    by_side: dict[int, dict[str, list[Shape]]] = defaultdict(lambda: defaultdict(list))
    for shape in shapes:
        by_side[shape.side_count][shape.law].append(shape)
    rows = []
    for side, populations in sorted(by_side.items()):
        names = sorted(populations)
        for i, left in enumerate(names):
            for right in names[i + 1 :]:
                lm, rm = populations[left], populations[right]
                d, _ = distances(lm + rm)
                cross = d[: len(lm), len(lm) :]
                left_d, right_d = np.min(cross, axis=1), np.min(cross, axis=0)
                def threshold(members: list[Shape]) -> float | None:
                    if len(members) < 2:
                        return None
                    inner, _ = distances(members)
                    return float(np.median(inner[np.triu_indices(len(members), 1)]))
                lt, rt = threshold(lm), threshold(rm)
                rows.append({"side_count": side, "population_left": left, "population_right": right, "n_left": len(lm), "n_right": len(rm), "sample_status": "small-sample" if min(len(lm), len(rm)) < SMALL_SAMPLE else "descriptive", "left_to_right_nearest_mean": float(np.mean(left_d)), "right_to_left_nearest_mean": float(np.mean(right_d)), "left_overlap_fraction": None if lt is None else float(np.mean(left_d <= lt)), "right_overlap_fraction": None if rt is None else float(np.mean(right_d <= rt)), "overlap_definition": "directed nearest cross-population distance <= source within-population pairwise median"})
    return rows


def feature_views(shapes: list[Shape]) -> tuple[list[dict[str, Any]], list[dict[str, Any]], list[dict[str, Any]]]:
    grouped = groups(shapes)
    arrays = {key: np.stack([invariant_features(x) for x in members]) for key, members in grouped.items()}
    spectrum, overlap, confounding = [], [], []
    for (side, population), values in arrays.items():
        if len(values) < 2:
            continue
        cov = np.cov(values, rowvar=False)
        eig = np.linalg.eigvalsh(cov)[::-1]
        positive = eig[eig > max(float(eig[0]), 1.0) * 1e-12]
        rank = None if not len(positive) else float(np.sum(positive) ** 2 / np.sum(positive * positive))
        for i, name in enumerate(FEATURES):
            spectrum.append({"side_count": side, "population": population, "n": len(values), "feature": name, "mean": float(np.mean(values[:, i])), "std": float(np.std(values[:, i], ddof=1)), "q05": float(np.quantile(values[:, i], .05)), "q95": float(np.quantile(values[:, i], .95)), "eigenvalue_1": float(eig[0]), "eigenvalue_2": float(eig[1]), "eigenvalue_3": float(eig[2]), "eigenvalue_4": float(eig[3]), "raw_feature_covariance_spectrum_participation_ratio": rank, "feature_scale_contract": "raw unstandardized features; covariance_anisotropy dominates this spectrum, so it is neither balanced evidence nor a score"})
    by_side: dict[int, dict[str, np.ndarray]] = defaultdict(dict)
    for (side, population), values in arrays.items(): by_side[side][population] = values
    for side, populations in sorted(by_side.items()):
        names = sorted(populations)
        for i, name in enumerate(FEATURES):
            pooled = np.concatenate([populations[p][:, i] for p in names]); grand = float(np.mean(pooled)); total = float(np.sum((pooled - grand) ** 2)); between = float(sum(len(populations[p]) * (float(np.mean(populations[p][:, i])) - grand) ** 2 for p in names))
            confounding.append({"side_count": side, "feature": name, "population_label_eta_squared": None if not total else between / total, "pooled_n": len(pooled), "interpretation": "descriptive label confounding, not a mechanism claim"})
        for i, left in enumerate(names):
            for right in names[i + 1 :]:
                for j, name in enumerate(FEATURES):
                    lq, rq = np.quantile(populations[left][:, j], [.05, .95]), np.quantile(populations[right][:, j], [.05, .95]); inter = max(0., min(lq[1], rq[1]) - max(lq[0], rq[0])); union = max(lq[1], rq[1]) - min(lq[0], rq[0])
                    overlap.append({"side_count": side, "population_left": left, "population_right": right, "feature": name, "left_q05": lq[0], "left_q95": lq[1], "right_q05": rq[0], "right_q95": rq[1], "q05_q95_interval_overlap": None if not union else inter / union})
    return spectrum, overlap, confounding


def occupancy_view(shapes: list[Shape]) -> list[dict[str, Any]]:
    result = []
    for population in sorted({x.law for x in shapes}):
        members = [x for x in shapes if x.law == population]; total = len(members)
        for kind, values, defined, note in [("side_count", [str(x) for x in sorted({x.side_count for x in members})], True, "fixed-panel accepted-row allocation, not a natural law probability"), ("source_bucket", sorted({str(x.row.get("pair_bucket")) for x in members}), False, "producer bucket is a source label, not a combinatorial type")]:
            for value in values:
                count = sum((str(x.side_count) if kind == "side_count" else str(x.row.get("pair_bucket"))) == value for x in members)
                result.append({"population": population, "occupancy_kind": kind, "category": value, "count": count, "fraction": count / total, "defined": defined, "interpretation": note})
    return result


def costs(paths: list[Path]) -> list[dict[str, Any]]:
    result = []
    for path in paths:
        report = json.loads(path.read_text())
        for item in report.get("per_population", []):
            accepted, requested, elapsed = int(item["accepted"]), int(item["requested"]), float(item["total_generation_ms"])
            result.append({"population": f"{item['law']}[{item['parameter']}]", "side_count": item["side_count"], "requested": requested, "accepted": accepted, "exhausted": item["exhausted"], "acceptance_fraction": accepted / requested if requested else None, "total_generation_ms": elapsed, "generation_ms_per_accepted": elapsed / accepted if accepted else None, "max_attempts_observed": item["max_attempts_observed"], "source_revision": report.get("source_revision"), "source_dirty": report.get("source_dirty")})
    return result


def saturation_view(shapes: list[Shape]) -> list[dict[str, Any]]:
    result = []
    for (side, population), members in groups(shapes).items():
        d, _ = distances(members); previous = None
        for requested in SATURATION:
            n = min(requested, len(members))
            if n < 2: continue
            value = float(np.mean(d[:n, :n][np.triu_indices(n, 1)])); change = None if previous in (None, 0.) else abs(value - previous) / abs(previous)
            result.append({"side_count": side, "population": population, "requested_n": requested, "used_n": n, "pairwise_l2_mean": value, "relative_change": change, "sample_status": "small-sample" if n < SMALL_SAMPLE else "descriptive", "selection": "deterministic SHA-256 rank prefix"}); previous = value
    return result


def source_exact_validation_witness(source: Path, target: Path, per_group: int) -> dict[str, Any]:
    grouped: dict[tuple[str, int], list[dict[str, Any]]] = defaultdict(list)
    for line in source.read_text().splitlines():
        if line.strip():
            row = json.loads(line); grouped[(row.get("population", row.get("law")), int(row["side_count"]))].append(row)
    rows = []
    for key, values in sorted(grouped.items()):
        values.sort(key=lambda row: hashlib.sha256(json.dumps(row, sort_keys=True, separators=(",", ":")).encode()).digest()); rows.extend(values[:per_group])
    target.parent.mkdir(parents=True, exist_ok=True); target.write_text("".join(json.dumps(row, sort_keys=True, separators=(",", ":")) + "\n" for row in rows))
    return {"source": str(source), "source_sha256": sha256(source), "retained_rows": len(rows), "per_population_side_count": per_group, "linkage_kind": "population-and-side-count stratum only", "individual_row_linkage": False, "target_evaluation": False, "contract": "Source exact-validation witness rows calibrate the geometry-only strata; they are not a subset of the new panel and their IDs do not match it."}


def producer_provenance(executable: Path, revision: str) -> dict[str, Any]:
    """Bind producer identity to a reproducible source/build closure."""
    if not executable.is_file():
        raise SystemExit(f"producer executable does not exist: {executable}")
    repo = Path(__file__).resolve().parents[4]
    source_paths = (
        "experiments/sys-datascience/methods/generator-zoo-smoke/main.rs",
        "experiments/sys-landscape/Cargo.toml",
        "Cargo.lock",
    )
    blobs = {}
    for path in source_paths:
        result = subprocess.run(["git", "-C", str(repo), "rev-parse", f"{revision}:{path}"], capture_output=True, text=True)
        if result.returncode:
            raise SystemExit(f"cannot resolve producer source blob {revision}:{path}: {result.stderr.strip()}")
        blobs[path] = result.stdout.strip()
    return {
        "executable_path_at_capture": str(executable),
        "executable_sha256": sha256(executable),
        "source_revision": revision,
        "source_blobs": blobs,
        "build_contract": "cargo build --release --locked --package exp-sys-landscape --bin sys-datascience-generator-zoo-smoke at source_revision, with the source blobs and Cargo.lock above",
        "dirty_scope": "producer reports separately record source_dirty for generator-zoo-smoke/main.rs and experiments/sys-landscape/Cargo.toml",
    }


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--input", type=Path, required=True); parser.add_argument("--out-dir", type=Path, required=True); parser.add_argument("--producer-report", type=Path, action="append", default=[]); parser.add_argument("--producer-executable", type=Path, required=True); parser.add_argument("--producer-revision", default="fd9c3e7df08d8c9d04491b8ebbb7b2628d2df32e"); parser.add_argument("--exact-input", type=Path); parser.add_argument("--witness-per-group", type=int, default=2); parser.add_argument("--support-grid", type=int, default=64); parser.add_argument("--steiner-grid", type=int, default=1024); parser.add_argument("--baseline", default="current-baseline[delta=0.2]")
    args = parser.parse_args(); args.out_dir.mkdir(parents=True, exist_ok=True)
    if args.support_grid < 32 or args.steiner_grid < args.support_grid: raise SystemExit("require steiner-grid >= support-grid >= 32")
    shapes = load_shapes(args.input, args.support_grid, args.steiner_grid)
    spectrum, feature_overlap, confounding = feature_views(shapes)
    views = {"within-population.tsv": within_view(shapes), "between-population.tsv": between_view(shapes), "nearest-cross-population.tsv": overlap_view(shapes), "combinatorial-occupancy.tsv": occupancy_view(shapes), "acceptance-cost.tsv": costs(args.producer_report), "sample-size-saturation.tsv": saturation_view(shapes), "feature-spectrum.tsv": spectrum, "feature-range-overlap.tsv": feature_overlap, "feature-law-confounding.tsv": confounding}
    for name, rows in views.items(): write_tsv(args.out_dir / name, rows)
    linkage = None
    if args.exact_input: linkage = source_exact_validation_witness(args.exact_input, args.out_dir / "source-exact-validation-witness/factor-shapes.jsonl", args.witness_per_group); (args.out_dir / "source-exact-validation-witness/linkage.json").write_text(json.dumps(linkage, indent=2, sort_keys=True) + "\n")
    implementation_hashes = {"atlas_py_sha256": sha256(Path(__file__)), "shape_quality_py_sha256": sha256(Path(shape_quality.__file__).resolve())}
    producer_identity = producer_provenance(args.producer_executable, args.producer_revision)
    report = {"schema": ATLAS_SCHEMA, "input_schema": SCHEMA, "input": str(args.input), "input_sha256": sha256(args.input), "rows_validated": len(shapes), "populations": sorted({x.law for x in shapes}), "side_counts": sorted({x.side_count for x in shapes}), "configuration": {"support_grid": args.support_grid, "steiner_grid": args.steiner_grid, "saturation_levels": SATURATION, "small_sample_boundary": SMALL_SAMPLE, "distance_approximation": "declared-grid circular correlation; focused continuous-refinement copy remains in shape_quality.py", "distance_calibration_contract": "grid-aligned rotations are quotiented exactly on the declared grid; arbitrary rotations are only approximated and their error is checked by test_atlas.py", "feature_contract": "covariance_anisotropy is the ratio of eigenvalues of centered vertex covariance and is rotation/translation/scale invariant", "raw_feature_views": "raw_feature_centroid_separation and raw_feature_covariance_spectrum_participation_ratio use unstandardized coordinates; covariance_anisotropy dominates them, so they are neither balanced evidence nor scores", "positive_gram_spectrum_contract": "positive_gram_spectrum_participation_ratio is the participation ratio of positive eigenvalues of the centered squared-distance Gram matrix; it is not an intrinsic or metric dimension. negative_eigenmass_fraction diagnoses failure of a Euclidean embedding."}, "implementation_hashes": implementation_hashes, "views": {name.removesuffix(".tsv").replace("-", "_"): name for name in views}, "producer_reports": [{"path": str(p), "sha256": sha256(p)} for p in args.producer_report], "producer_provenance": producer_identity, "source_exact_validation_witness": linkage, "structural_product_classification": {"status": "deferred", "reason": "The retained panel is planar factors; coordinate/affine/Lagrangian product classes require 4D normals and an explicit classifier. Do not infer productness from factor shape or classifier failure."}, "rank_uncertainty": {"pilot_selection_confirmation": "deferred", "repeated_seed_stability": "deferred", "reason": "One producer seed is retained and no target-derived selection occurred; a confirmation packet must freeze pilot strata and rerun independent seeds.", "cheap_calibration": "feature covariance spectrum, quantile-range overlap, label eta-squared, and deterministic saturation diagnostics are included; they are not uncertainty estimates."}, "interpretation": {"allowed": ["describe finite-panel geometry, overlap, covariance, occupancy, and measured producer cost by named population/side stratum", "identify redundant or under-sampled strata for later target-free or exact work"], "prohibited": ["global quality ranking or combined score", "natural-law probabilities, population generalization, causal mechanism, or sys/target prediction", "individual linkage between geometry-only and source exact-validation witness rows"]}}
    provenance = {"schema": "generator-distribution-atlas-provenance-v1", "panel_input": {"path": str(args.input), "sha256": sha256(args.input)}, "producer_reports": [{"path": str(p), "sha256": sha256(p)} for p in args.producer_report], "producer": producer_identity, "analyzer": implementation_hashes, "contract": "Panel and all derived views are target-free. Rebuild the producer from the pinned source revision and compare executable hash before regeneration."}
    provenance_path = args.out_dir.parent / "panel/provenance.json"
    provenance_path.parent.mkdir(parents=True, exist_ok=True)
    provenance_path.write_text(json.dumps(provenance, indent=2, sort_keys=True) + "\n")
    report["provenance_artifact"] = "artifacts/panel/provenance.json"
    (args.out_dir / "report.json").write_text(json.dumps(report, indent=2, sort_keys=True) + "\n")
    print(json.dumps({"rows": len(shapes), "populations": len(report["populations"]), "out_dir": str(args.out_dir)}))


if __name__ == "__main__": main()
