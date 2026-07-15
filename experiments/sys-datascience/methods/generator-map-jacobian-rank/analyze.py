#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = ["numpy==2.5.1"]
# ///

"""Local Jacobian-rank audit for copy-local planar factor generators."""

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


HERE = Path(__file__).resolve().parent
TAU = 2.0 * math.pi
STEPS = (1e-4, 3e-5, 1e-5, 3e-6, 1e-6)
RELATIVE_THRESHOLDS = (1e-6, 1e-7, 1e-8, 1e-10)
PRIMARY_RELATIVE_THRESHOLD = 1e-6
ABSOLUTE_THRESHOLD = 1e-8
SOURCE_FILES = ("analyze.py", "README.md", "test_packet.py", "test_reproducibility.py")


class MapFailure(Exception):
    pass


@dataclass(frozen=True)
class Evaluation:
    vertices: np.ndarray
    discrete_state: tuple[Any, ...]


@dataclass(frozen=True)
class Chart:
    vector: np.ndarray
    key: tuple[bool, int]
    label_status: str
    closest_distinct_label_distance: float | None


@dataclass(frozen=True)
class Base:
    law: str
    parameter: str
    side_count: int
    seed: int
    latent: np.ndarray
    expected_rank: int
    expected_upper_bound: int
    evaluator: Callable[[np.ndarray], Evaluation]


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for block in iter(lambda: handle.read(1 << 20), b""):
            digest.update(block)
    return digest.hexdigest()


def source_contract() -> dict[str, Any]:
    revision = subprocess.check_output(
        ["git", "log", "-1", "--format=%H", "--", *SOURCE_FILES], cwd=HERE, text=True
    ).strip()
    dirty = bool(
        subprocess.check_output(
            ["git", "status", "--porcelain", "--", *SOURCE_FILES], cwd=HERE, text=True
        ).strip()
    )
    return {
        "contract": "generator-map-jacobian-rank-source-v1",
        "declared_source_revision": revision,
        "source_dirty": dirty,
        "analyzer_sha256": sha256(Path(__file__)),
        "numpy_version": np.__version__,
    }


def polygon_area(vertices: np.ndarray) -> float:
    return float(0.5 * np.sum(vertices[:, 0] * np.roll(vertices[:, 1], -1) - vertices[:, 1] * np.roll(vertices[:, 0], -1)))


def validate_polygon(vertices: np.ndarray) -> None:
    if vertices.ndim != 2 or vertices.shape[1] != 2 or len(vertices) < 3 or not np.all(np.isfinite(vertices)):
        raise MapFailure("invalid-vertex-array")
    edges = np.roll(vertices, -1, axis=0) - vertices
    turns = edges[:, 0] * np.roll(edges[:, 1], -1) - edges[:, 1] * np.roll(edges[:, 0], -1)
    diameter = float(np.max(np.linalg.norm(vertices[:, None] - vertices[None, :], axis=2)))
    if polygon_area(vertices) <= 1e-11 * max(diameter * diameter, 1.0):
        raise MapFailure("nonpositive-or-degenerate-area")
    if np.min(turns) <= 1e-11 * max(diameter * diameter, 1.0):
        raise MapFailure("non-strict-convexity")


def _candidate(vertices: np.ndarray, key: tuple[bool, int]) -> np.ndarray:
    reversed_order, shift = key
    candidate = vertices[::-1] if reversed_order else vertices
    candidate = np.roll(candidate, -shift, axis=0)
    anchor = candidate[0]
    if np.linalg.norm(anchor) <= 1e-10:
        raise MapFailure("chart-anchor-at-center")
    angle = math.atan2(anchor[1], anchor[0])
    cosine, sine = math.cos(angle), math.sin(angle)
    rotation = np.array([[cosine, sine], [-sine, cosine]])
    return (candidate @ rotation.T).reshape(-1)


def body_chart(vertices: np.ndarray, preferred_key: tuple[bool, int] | None = None) -> Chart:
    """Local body chart modulo translation, scale, rotation, cycle, and reversal.

    The base chooses a deterministic dihedral representative. Perturbations use
    that same representative, which is the local-label continuity contract.
    Exact stabilizer ties are allowed only when their chart vectors coincide.
    """
    if polygon_area(vertices) < 0:
        vertices = vertices[::-1]
    validate_polygon(vertices)
    centered = vertices - np.mean(vertices, axis=0)
    scale = math.sqrt(float(np.mean(np.sum(centered * centered, axis=1))))
    if scale <= 1e-12:
        raise MapFailure("zero-chart-scale")
    normalized = centered / scale
    if preferred_key is not None:
        return Chart(_candidate(normalized, preferred_key), preferred_key, "linked-local-label", None)
    candidates = []
    for reversed_order in (False, True):
        for shift in range(len(vertices)):
            key = (reversed_order, shift)
            vector = _candidate(normalized, key)
            rounded = tuple(np.round(vector, decimals=12))
            candidates.append((rounded, key, vector))
    candidates.sort(key=lambda item: (item[0], item[1]))
    best_round, best_key, best = candidates[0]
    rounded_ties = [item for item in candidates if item[0] == best_round]
    if any(np.max(np.abs(item[2] - best)) > 1e-9 for item in rounded_ties[1:]):
        raise MapFailure("nonidentical-canonical-label-tie")
    distances = [float(np.linalg.norm(item[2] - best)) for item in candidates if item[1] != best_key and np.linalg.norm(item[2] - best) > 1e-9]
    status = "benign-stabilizer-tie" if len(rounded_ties) > 1 else "generic-unique-label"
    return Chart(best, best_key, status, min(distances) if distances else None)


def angle_polygon(angles: np.ndarray, log_heights: np.ndarray) -> Evaluation:
    n = len(angles)
    gaps = np.diff(np.r_[angles, angles[0] + TAU])
    if np.any(gaps <= 1e-5) or np.any(gaps >= math.pi - 1e-5):
        raise MapFailure("normal-fan-topology-boundary")
    heights = np.exp(log_heights)
    normals = np.column_stack((np.cos(angles), np.sin(angles)))
    vertices = []
    for i in range(n):
        j = (i + 1) % n
        determinant = normals[i, 0] * normals[j, 1] - normals[i, 1] * normals[j, 0]
        if abs(determinant) <= 1e-10:
            raise MapFailure("adjacent-normal-singularity")
        vertices.append([(heights[i] * normals[j, 1] - heights[j] * normals[i, 1]) / determinant, (normals[i, 0] * heights[j] - normals[j, 0] * heights[i]) / determinant])
    vertices_array = np.asarray(vertices)
    validate_polygon(vertices_array)
    slack = heights[None, :] - vertices_array @ normals.T
    incident = np.zeros_like(slack, dtype=bool)
    for i in range(n):
        incident[i, i] = True
        incident[i, (i + 1) % n] = True
    if np.min(slack[~incident]) <= 1e-9:
        raise MapFailure("inactive-or-nearly-inactive-facet")
    return Evaluation(vertices_array, ("angle-factor", n))


def baseline_base(n: int, seed: int) -> Base:
    rng = np.random.default_rng(seed)
    for _ in range(2000):
        angles = np.sort(rng.uniform(0, TAU, n))
        heights = rng.uniform(.8, 1.2, n)
        latent = np.r_[angles, np.log(heights)]
        def evaluate(value: np.ndarray, n: int = n) -> Evaluation:
            local_angles = value[:n]
            return angle_polygon(local_angles, value[n:])
        try:
            evaluate(latent)
            return Base("current-baseline", "delta=0.2", n, seed, latent, 2 * n - 4, 2 * n - 4, evaluate)
        except MapFailure:
            continue
    raise MapFailure("baseline-base-exhausted")


def dirichlet_base(n: int, seed: int, alpha: float) -> Base:
    rng = np.random.default_rng(seed)
    for _ in range(2000):
        gamma = rng.gamma(alpha, 1.0, n)
        rotation = rng.uniform(0, TAU)
        latent = np.r_[rotation, np.log(gamma)]
        def evaluate(value: np.ndarray, n: int = n) -> Evaluation:
            weights = np.exp(value[1:] - np.max(value[1:]))
            gaps = TAU * weights / np.sum(weights)
            angles = value[0] + np.r_[0.0, np.cumsum(gaps[:-1])]
            return angle_polygon(angles, np.zeros(n))
        try:
            evaluate(latent)
            return Base("equal-support-dirichlet", f"alpha={alpha:g}", n, seed, latent, n - 1, n - 1, evaluate)
        except MapFailure:
            continue
    raise MapFailure("dirichlet-base-exhausted")


def regular_base(n: int, seed: int) -> Base:
    rotation = np.random.default_rng(seed).uniform(0, TAU)
    latent = np.array([rotation])
    def evaluate(value: np.ndarray, n: int = n) -> Evaluation:
        return angle_polygon(value[0] + TAU * np.arange(n) / n, np.zeros(n))
    return Base("equal-support-dirichlet", "regular", n, seed, latent, 0, 0, evaluate)


def zonogon_base(n: int, seed: int) -> Base:
    if n % 2:
        raise MapFailure("zonogon-odd-side-count")
    r = n // 2
    rng = np.random.default_rng(seed)
    angles = np.sort(rng.uniform(0, math.pi, r))
    lengths = rng.uniform(.5, 1.5, r)
    latent = np.r_[angles, np.log(lengths)]
    def evaluate(value: np.ndarray, r: int = r) -> Evaluation:
        local_angles, local_lengths = value[:r], np.exp(value[r:])
        if np.any(np.diff(local_angles) <= 1e-5) or local_angles[0] <= 1e-5 or local_angles[-1] >= math.pi - 1e-5:
            raise MapFailure("zonogon-direction-order-boundary")
        edges = []
        start = np.zeros(2)
        for angle, length in zip(local_angles, local_lengths):
            vector = np.array([math.cos(angle), math.sin(angle)])
            start -= length * vector
            edges.extend(((angle, 2 * length * vector), (angle + math.pi, -2 * length * vector)))
        edges.sort(key=lambda item: item[0])
        vertices, point = [], start
        for _, edge in edges:
            vertices.append(point.copy())
            point += edge
        vertices_array = np.asarray(vertices)
        validate_polygon(vertices_array)
        return Evaluation(vertices_array, ("zonogon-edge-order", tuple(np.argsort(np.r_[local_angles, local_angles + math.pi]))))
    evaluate(latent)
    return Base("zonogon", "lengths=uniform(0.5,1.5)", n, seed, latent, n - 2, n - 2, evaluate)


def mutation_base(n: int, seed: int, steps: int = 4, scale: float = .03) -> Base:
    rng = np.random.default_rng(seed)
    spacing = TAU / n
    for _ in range(200):
        latent = np.r_[rng.uniform(0, TAU), rng.normal(0, scale, 2 * steps * n)]
        def evaluate(value: np.ndarray, n: int = n, steps: int = steps, spacing: float = spacing) -> Evaluation:
            angles = value[0] + spacing * np.arange(n, dtype=float)
            heights = np.ones(n)
            offset = 1
            states = []
            for step in range(steps):
                angle_noise = value[offset : offset + n]; offset += n
                support_noise = value[offset : offset + n]; offset += n
                cap = .2 * spacing
                clipped = np.clip(angle_noise, -cap, cap)
                clip_state = tuple(np.where(angle_noise < -cap, -1, np.where(angle_noise > cap, 1, 0)).tolist())
                angles = angles + clipped
                heights *= np.exp(.5 * support_noise)
                order = np.argsort(angles, kind="stable")
                angles = angles[order]
                gaps = np.diff(np.r_[angles, angles[0] + TAU])
                if np.any(gaps < .2 * spacing) or np.any(gaps >= math.pi):
                    raise MapFailure("mutation-gap-boundary")
                states.append((step, clip_state, tuple(order.tolist())))
            result = angle_polygon(angles, np.log(heights))
            return Evaluation(result.vertices, ("regular-mutation", tuple(states)))
        try:
            evaluate(latent)
            return Base("regular-mutation", "steps=4,scale=0.03", n, seed, latent, 2 * n - 4, 2 * n - 4, evaluate)
        except MapFailure:
            continue
    raise MapFailure("mutation-base-exhausted")


def convex_hull_indices(points: np.ndarray) -> list[int]:
    order = sorted(range(len(points)), key=lambda i: (points[i, 0], points[i, 1], i))
    def cross(i: int, j: int, k: int) -> float:
        left, right = points[j] - points[i], points[k] - points[i]
        return float(left[0] * right[1] - left[1] * right[0])
    lower: list[int] = []
    for index in order:
        while len(lower) >= 2 and cross(lower[-2], lower[-1], index) <= 1e-11:
            lower.pop()
        lower.append(index)
    upper: list[int] = []
    for index in reversed(order):
        while len(upper) >= 2 and cross(upper[-2], upper[-1], index) <= 1e-11:
            upper.pop()
        upper.append(index)
    return lower[:-1] + upper[:-1]


def primal_hull_base(n: int, seed: int) -> Base:
    rng = np.random.default_rng(seed)
    count = n + 4
    for _ in range(10000):
        u = rng.uniform(.02, .98, count)
        angles = rng.uniform(0, TAU, count)
        latent = np.r_[u, angles]
        def evaluate(value: np.ndarray, count: int = count, n: int = n) -> Evaluation:
            local_u, local_angles = value[:count], value[count:]
            if np.any(local_u <= 0) or np.any(local_u >= 1):
                raise MapFailure("primal-radial-boundary")
            radii = np.sqrt(local_u)
            points = np.column_stack((radii * np.cos(local_angles), radii * np.sin(local_angles)))
            hull = convex_hull_indices(points)
            if len(hull) != n:
                raise MapFailure("primal-hull-side-count-change")
            vertices = points[hull]
            validate_polygon(vertices)
            edges = np.roll(vertices, -1, axis=0) - vertices
            origin_cross = edges[:, 0] * (-vertices[:, 1]) - edges[:, 1] * (-vertices[:, 0])
            if np.min(origin_cross) <= 1e-8:
                raise MapFailure("primal-origin-not-strictly-interior")
            return Evaluation(vertices, ("primal-hull-active-set", tuple(hull)))
        try:
            evaluate(latent)
            return Base("primal-hull-uniform-disk", "points=n+4,origin=interior", n, seed, latent, 2 * n - 4, 2 * n - 4, evaluate)
        except MapFailure:
            continue
    raise MapFailure("primal-base-exhausted")


def rank_from_spectrum(singular: np.ndarray, relative: float) -> int:
    if len(singular) == 0 or singular[0] <= ABSOLUTE_THRESHOLD:
        return 0
    return int(np.sum(singular > max(ABSOLUTE_THRESHOLD, relative * singular[0])))


def finite_difference_base(base: Base) -> dict[str, Any]:
    try:
        base_evaluation = base.evaluator(base.latent)
        base_chart = body_chart(base_evaluation.vertices)
    except MapFailure as error:
        return {"status": "base-failure", "reason": str(error)}
    spectra = []
    for step in STEPS:
        jacobian = np.empty((len(base_chart.vector), len(base.latent)))
        for column in range(len(base.latent)):
            plus = base.latent.copy(); plus[column] += step
            minus = base.latent.copy(); minus[column] -= step
            try:
                plus_evaluation, minus_evaluation = base.evaluator(plus), base.evaluator(minus)
                if plus_evaluation.discrete_state != base_evaluation.discrete_state or minus_evaluation.discrete_state != base_evaluation.discrete_state:
                    raise MapFailure("discrete-topology-or-active-set-change")
                plus_chart = body_chart(plus_evaluation.vertices, base_chart.key)
                minus_chart = body_chart(minus_evaluation.vertices, base_chart.key)
            except MapFailure as error:
                return {"status": "perturbation-failure", "step": step, "column": column, "reason": str(error), "base_chart_label_status": base_chart.label_status}
            jacobian[:, column] = (plus_chart.vector - minus_chart.vector) / (2 * step)
        singular = np.linalg.svd(jacobian, compute_uv=False)
        spectra.append({"step": step, "singular_values": singular.tolist(), "rank_by_relative_threshold": {f"{threshold:.0e}": rank_from_spectrum(singular, threshold) for threshold in RELATIVE_THRESHOLDS}})
    primary_key = f"{PRIMARY_RELATIVE_THRESHOLD:.0e}"
    primary_ranks = [entry["rank_by_relative_threshold"][primary_key] for entry in spectra]
    smallest = [entry["singular_values"][base.expected_rank - 1] if base.expected_rank > 0 and len(entry["singular_values"]) >= base.expected_rank else None for entry in spectra]
    discrete_summary: dict[str, Any] = {"kind": str(base_evaluation.discrete_state[0])}
    if base_evaluation.discrete_state[0] == "regular-mutation":
        states = base_evaluation.discrete_state[1]
        clipped = sum(abs(value) == 1 for _, clip_state, _ in states for value in clip_state)
        discrete_summary.update({"angle_increment_count": 4 * base.side_count, "clipped_angle_increment_count": clipped, "all_angle_increments_unclipped": clipped == 0})
    if base_evaluation.discrete_state[0] == "primal-hull-active-set":
        discrete_summary["active_point_indices"] = list(base_evaluation.discrete_state[1])
    return {"status": "ok", "latent_dimension": len(base.latent), "chart_dimension": len(base_chart.vector), "expected_rank": base.expected_rank, "expected_upper_bound": base.expected_upper_bound, "base_discrete_state_summary": discrete_summary, "base_chart_key": list(base_chart.key), "base_chart_label_status": base_chart.label_status, "closest_distinct_label_distance": base_chart.closest_distinct_label_distance, "spectra": spectra, "primary_relative_threshold": PRIMARY_RELATIVE_THRESHOLD, "primary_rank_stable": len(set(primary_ranks)) == 1, "primary_ranks": primary_ranks, "expected_rank_matched_all_steps": all(rank == base.expected_rank for rank in primary_ranks), "expected_last_singular_values": smallest}


def synthetic_calibrations() -> dict[str, Any]:
    rng = np.random.default_rng(77)
    cases: dict[str, tuple[np.ndarray, int, str]] = {}
    full = rng.normal(size=(7, 7)); cases["full_rank"] = (full, 7, "generic square linear map")
    lower = np.vstack((rng.normal(size=(3, 5)), np.zeros((4, 5)))); cases["lower_rank"] = (lower, 3, "three-row image in seven outputs")
    duplicate = np.array([[1., 0., 0.], [1., 0., 0.], [0., 1., 0.], [0., 0., 1.]])
    cases["duplicate_output_coordinate"] = (duplicate, 3, "one duplicated output coordinate does not add rank")
    gauge = np.column_stack((np.eye(4), np.zeros((4, 3)))); cases["three_gauge_directions"] = (gauge, 4, "three latent directions are exact kernel directions")
    near = np.diag([1., .1, 1e-5, 5e-8]); cases["near_singular"] = (near, 4, "rank is intentionally threshold-dependent")
    output = {}
    for name, (matrix, analytic_rank, meaning) in cases.items():
        singular = np.linalg.svd(matrix, compute_uv=False)
        output[name] = {"analytic_rank": analytic_rank, "meaning": meaning, "singular_values": singular.tolist(), "rank_by_relative_threshold": {f"{threshold:.0e}": rank_from_spectrum(singular, threshold) for threshold in RELATIVE_THRESHOLDS}}
    return output


def build_base(law: str, parameter: str, n: int, seed: int) -> Base:
    if law == "current-baseline": return baseline_base(n, seed)
    if law == "equal-support-dirichlet" and parameter == "regular": return regular_base(n, seed)
    if law == "equal-support-dirichlet": return dirichlet_base(n, seed, float(parameter.split("=")[1]))
    if law == "zonogon": return zonogon_base(n, seed)
    if law == "regular-mutation": return mutation_base(n, seed)
    if law == "primal-hull-uniform-disk": return primal_hull_base(n, seed)
    raise ValueError(law)


def analytic_rank_reason(law: str, parameter: str, n: int) -> str:
    if law == "current-baseline":
        return f"2n angle/support parameters lose four similarity/body directions, capped by the full chart: 2n-4={2*n-4}"
    if law == "equal-support-dirichlet" and parameter == "regular":
        return "only global rotation varies, and the chart quotients rotation: rank 0"
    if law == "equal-support-dirichlet":
        return f"n positive gaps modulo their sum give n-1={n-1}; global rotation is chart gauge"
    if law == "zonogon":
        return f"n/2 directions plus n/2 lengths, modulo common rotation and scale: n-2={n-2}"
    if law == "regular-mutation":
        return f"on the retained all-unclipped stratum, linked step latents vary final angles/supports up to the full chart: 2n-4={2*n-4}; open saturated clipping strata can have lower rank"
    if law == "primal-hull-uniform-disk":
        return f"on a fixed active set, n hull vertices vary freely; four similarity directions are quotiented: 2n-4={2*n-4}"
    raise ValueError(law)


def run_packet(seeds: list[int], sides: list[int]) -> dict[str, Any]:
    populations = [("current-baseline", "delta=0.2"), ("equal-support-dirichlet", "alpha=1"), ("equal-support-dirichlet", "alpha=4"), ("equal-support-dirichlet", "alpha=16"), ("equal-support-dirichlet", "regular"), ("zonogon", "lengths=uniform(0.5,1.5)"), ("regular-mutation", "steps=4,scale=0.03"), ("primal-hull-uniform-disk", "points=n+4,origin=interior")]
    bases = []
    for law, parameter in populations:
        for n in sides:
            for seed in seeds:
                record: dict[str, Any] = {"law": law, "parameter": parameter, "side_count": n, "seed": seed, "full_polygon_chart_dimension": 2 * n - 4, "analytic_rank_reason": analytic_rank_reason(law, parameter, n)}
                try:
                    base = build_base(law, parameter, n, seed)
                    record.update(finite_difference_base(base))
                except MapFailure as error:
                    record.update({"status": "base-construction-failure", "reason": str(error)})
                bases.append(record)
    grouped: dict[tuple[str, str, int], list[dict[str, Any]]] = defaultdict(list)
    for record in bases:
        grouped[(record["law"], record["parameter"], record["side_count"])].append(record)
    strata = []
    for (law, parameter, n), records in sorted(grouped.items()):
        successful = [record for record in records if record["status"] == "ok"]
        ranks = [record["primary_ranks"][0] for record in successful]
        strata.append({"law": law, "parameter": parameter, "side_count": n, "base_count": len(records), "successful_count": len(successful), "failure_reasons": dict(sorted(Counter(record.get("reason", record["status"]) for record in records if record["status"] != "ok").items())), "primary_rank_counts": dict(sorted(Counter(ranks).items())), "step_stable_count": sum(record["primary_rank_stable"] for record in successful), "expected_rank_match_count": sum(record["expected_rank_matched_all_steps"] for record in successful), "expected_rank": successful[0]["expected_rank"] if successful else None, "analytic_rank_reason": records[0]["analytic_rank_reason"], "full_polygon_chart_dimension": 2 * n - 4})
    return {"bases": bases, "strata": strata}


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--out-dir", type=Path, required=True)
    parser.add_argument("--seeds", default="11,29,47,83")
    parser.add_argument("--side-counts", default="4,6,8")
    args = parser.parse_args()
    seeds = [int(value) for value in args.seeds.split(",")]
    sides = [int(value) for value in args.side_counts.split(",")]
    started = time.monotonic()
    report = {"schema": "generator-map-jacobian-rank-report-v1", "target_free": True, "source_contract": source_contract(), "configuration": {"seeds": seeds, "side_counts": sides, "step_ladder": list(STEPS), "relative_rank_thresholds": list(RELATIVE_THRESHOLDS), "primary_relative_rank_threshold": PRIMARY_RELATIVE_THRESHOLD, "absolute_rank_threshold": ABSOLUTE_THRESHOLD, "rng_boundary": "NumPy PCG64 deterministically samples the copied laws; it does not replay Rust ChaCha bytes."}, "analytic_calibrations": synthetic_calibrations(), "generator_results": run_packet(seeds, sides), "method_dispositions": {"requested_generators": "all implemented, including fixed-active-set primal hull", "abandoned_generators": [], "optional_low_frequency_support_fields": "not attempted; separately active support-process work made this optional extension unnecessary"}, "analytic_status": "Expected ranks are agent-derived local dimension counts, not Jörn-reviewed theorems.", "supported_interpretation": "A stable matched rank shows that this copy-local generator map has the stated local image rank at the retained base and within the declared similarity/body chart. Mutation full-rank observations apply to the retained all-unclipped stratum only; clipping has open saturated lower-rank strata.", "prohibited_interpretation": "Local rank does not establish one law-wide generic rank, global support, law density, topology, rare-mode mass, chart coverage, generator naturalness, or any target/sys value. Failure at a discrete boundary is not a low-rank observation, while a stable saturated clipping stratum may be genuinely lower-rank."}
    args.out_dir.mkdir(parents=True, exist_ok=True)
    (args.out_dir / "report.json").write_text(json.dumps(report, indent=2, sort_keys=True) + "\n")
    print(json.dumps({"out": str(args.out_dir / "report.json"), "runtime_seconds_observed_not_retained": time.monotonic() - started}, sort_keys=True))


if __name__ == "__main__":
    main()
