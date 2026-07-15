#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = ["numpy==2.5.1"]
# ///

"""Measure-aware follow-up to the accepted generator-map rank packet."""

from __future__ import annotations

import argparse
from collections import defaultdict
from fractions import Fraction
import hashlib
import importlib.util
import json
import math
from pathlib import Path
import subprocess
import sys
import time
from typing import Any

import numpy as np


HERE = Path(__file__).resolve().parent
RANK_DIR = HERE.parent / "generator-map-jacobian-rank"
RANK_ANALYZER = RANK_DIR / "analyze.py"
DEFAULT_RANK_REPORT = RANK_DIR / "artifacts/report.json"
SOURCE_FILES = ("analyze.py", "README.md", "test_packet.py", "test_reproducibility.py")
DIRICHLET_STEPS = (1e-4, 1e-5, 1e-6)


def load_rank_module() -> Any:
    spec = importlib.util.spec_from_file_location("accepted_generator_rank", RANK_ANALYZER)
    if spec is None or spec.loader is None:
        raise RuntimeError("cannot load accepted rank analyzer")
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


rank = load_rank_module()


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for block in iter(lambda: handle.read(1 << 20), b""):
            digest.update(block)
    return digest.hexdigest()


def source_contract() -> dict[str, Any]:
    revision = subprocess.check_output(["git", "log", "-1", "--format=%H", "--", *SOURCE_FILES], cwd=HERE, text=True).strip()
    dirty = bool(subprocess.check_output(["git", "status", "--porcelain", "--", *SOURCE_FILES], cwd=HERE, text=True).strip())
    return {"contract": "generator-map-distortion-source-v1", "declared_source_revision": revision, "source_dirty": dirty, "analyzer_sha256": sha256(Path(__file__)), "numpy_version": np.__version__}


def summary(values: list[float]) -> dict[str, float]:
    array = np.asarray(values, dtype=float)
    return {"min": float(np.min(array)), "median": float(np.median(array)), "max": float(np.max(array)), "mean": float(np.mean(array))}


def cyclic_gaps(angles: np.ndarray, period: float) -> np.ndarray:
    return np.diff(np.r_[angles, angles[0] + period])


def polygon_margin(vertices: np.ndarray) -> float:
    edges = np.roll(vertices, -1, axis=0) - vertices
    turns = edges[:, 0] * np.roll(edges[:, 1], -1) - edges[:, 1] * np.roll(edges[:, 0], -1)
    diameter = float(np.max(np.linalg.norm(vertices[:, None] - vertices[None, :], axis=2)))
    return float(np.min(turns) / (diameter * diameter))


def angle_factor_margins(angles: np.ndarray, log_heights: np.ndarray, vertices: np.ndarray) -> dict[str, float]:
    n = len(angles)
    gaps = cyclic_gaps(angles, rank.TAU)
    heights = np.exp(log_heights)
    normals = np.column_stack((np.cos(angles), np.sin(angles)))
    slack = heights[None, :] - vertices @ normals.T
    incident = np.zeros_like(slack, dtype=bool)
    for i in range(n):
        incident[i, i] = True; incident[i, (i + 1) % n] = True
    return {"normal_fan_boundary_margin_regular_gap_units": float(np.min(np.minimum(gaps, math.pi - gaps)) / (rank.TAU / n)), "inactive_facet_slack_over_mean_support": float(np.min(slack[~incident]) / np.mean(heights)), "strict_convexity_turn_over_diameter_squared": polygon_margin(vertices)}


def base_margins(base: Any) -> dict[str, float]:
    evaluation = base.evaluator(base.latent)
    law, n, value = base.law, base.side_count, base.latent
    if law == "current-baseline":
        margins = angle_factor_margins(value[:n], value[n:], evaluation.vertices)
        heights = np.exp(value[n:])
        margins["support_interval_margin_fraction"] = float(np.min(np.minimum(heights - .8, 1.2 - heights)) / .4)
        return margins
    if law == "equal-support-dirichlet":
        if base.parameter == "regular":
            angles = value[0] + rank.TAU * np.arange(n) / n
            margins = angle_factor_margins(angles, np.zeros(n), evaluation.vertices)
            margins["gap_simplex_boundary_margin"] = 1 / n
            return margins
        weights = np.exp(value[1:] - np.max(value[1:])); proportions = weights / np.sum(weights)
        gaps = rank.TAU * proportions
        angles = value[0] + np.r_[0., np.cumsum(gaps[:-1])]
        margins = angle_factor_margins(angles, np.zeros(n), evaluation.vertices)
        margins["gap_simplex_boundary_margin"] = float(np.min(proportions))
        return margins
    if law == "zonogon":
        r = n // 2; angles, lengths = value[:r], np.exp(value[r:])
        chamber_gaps = np.diff(np.r_[0., angles, math.pi])
        return {"direction_order_boundary_margin_regular_units": float(np.min(chamber_gaps) / (math.pi / r)), "length_interval_margin_fraction": float(np.min(np.minimum(lengths - .5, 1.5 - lengths))), "strict_convexity_turn_over_diameter_squared": polygon_margin(evaluation.vertices)}
    if law == "regular-mutation":
        spacing = rank.TAU / n; cap = .2 * spacing; offset = 1
        angles = value[0] + spacing * np.arange(n); gap_margins = []; clip_margins = []
        for _ in range(4):
            angle_noise = value[offset : offset + n]; offset += n
            offset += n
            clip_margins.extend((cap - np.abs(angle_noise)) / cap)
            angles = np.sort(angles + np.clip(angle_noise, -cap, cap))
            gaps = cyclic_gaps(angles, rank.TAU)
            gap_margins.extend((gaps - .2 * spacing) / spacing)
            gap_margins.extend((math.pi - gaps) / spacing)
        return {"clip_boundary_margin_cap_units": float(np.min(clip_margins)), "mutation_gap_acceptance_margin_spacing_units": float(np.min(gap_margins)), "strict_convexity_turn_over_diameter_squared": polygon_margin(evaluation.vertices)}
    if law == "primal-hull-uniform-disk":
        count = n + 4; u, angles = value[:count], value[count:]
        points = np.column_stack((np.sqrt(u) * np.cos(angles), np.sqrt(u) * np.sin(angles)))
        active = list(evaluation.discrete_state[1]); inactive = [i for i in range(count) if i not in active]
        vertices = points[active]
        diameter = float(np.max(np.linalg.norm(vertices[:, None] - vertices[None, :], axis=2)))
        edge = np.roll(vertices, -1, axis=0) - vertices
        inactive_slack = []
        for point in points[inactive]:
            cross = edge[:, 0] * (point[1] - vertices[:, 1]) - edge[:, 1] * (point[0] - vertices[:, 0])
            inactive_slack.append(float(np.min(cross / (np.linalg.norm(edge, axis=1) * diameter))))
        origin_cross = edge[:, 0] * (-vertices[:, 1]) - edge[:, 1] * (-vertices[:, 0])
        return {"radial_latent_boundary_margin": float(np.min(np.minimum(u, 1 - u))), "inactive_point_hull_slack_over_diameter": min(inactive_slack), "origin_interior_slack_over_edge_diameter": float(np.min(origin_cross / (np.linalg.norm(edge, axis=1) * diameter))), "strict_convexity_turn_over_diameter_squared": polygon_margin(vertices)}
    raise ValueError(law)


def spectrum_record(record: dict[str, Any]) -> dict[str, Any]:
    chosen = next(item for item in record["spectra"] if item["step"] == 1e-5)
    singular = np.asarray(chosen["singular_values"], dtype=float)
    dimension = int(record["expected_rank"])
    nonzero = singular[:dimension]
    return {"step": 1e-5, "rank": dimension, "condition_number_native_latents": float(nonzero[0] / nonzero[-1]) if dimension else None, "log_pseudodeterminant_native_latents": float(np.sum(np.log(nonzero))) if dimension else None, "first_null_to_leading_ratio": float(singular[dimension] / singular[0]) if dimension < len(singular) and singular[0] else None, "contract": "Diagnostics in the accepted native latent coordinates and fixed body-chart Euclidean metric; not a law density or cross-law invariant."}


def dirichlet_gap_geometry(proportions: np.ndarray, preferred_key: tuple[bool, int] | None = None) -> tuple[np.ndarray, Any]:
    gaps = rank.TAU * proportions
    if np.any(gaps <= 0) or np.any(gaps >= math.pi):
        raise rank.MapFailure("Dirichlet gap left accepted fan chamber")
    angles = np.r_[0., np.cumsum(gaps[:-1])]
    evaluation = rank.angle_polygon(angles, np.zeros(len(gaps)))
    chart = rank.body_chart(evaluation.vertices, preferred_key)
    return chart.vector, chart


def dirichlet_acceptance_normalizer(n: int, alpha: int) -> dict[str, Any]:
    """Exact P(max p_i < 1/2) for symmetric integer-alpha Dirichlet.

    Two coordinates cannot both exceed 1/2. The marginal p_1 is
    Beta(alpha,(n-1)alpha), whose upper tail at 1/2 equals the indicated
    finite binomial sum.
    """
    if alpha < 1:
        raise ValueError("integer alpha must be positive")
    trials = n * alpha - 1
    upper_tail_numerator = sum(math.comb(trials, k) for k in range(alpha))
    rejection_probability = n * Fraction(upper_tail_numerator, 2**trials)
    normalizer = Fraction(1, 1) - rejection_probability
    if normalizer <= 0:
        raise ValueError("empty accepted Dirichlet chamber")
    return {"n": n, "alpha": alpha, "formula": "1 - n * 2^(-(n*alpha-1)) * sum_{k=0}^{alpha-1} binom(n*alpha-1,k)", "exact_numerator": normalizer.numerator, "exact_denominator": normalizer.denominator, "exact_rejection_numerator": rejection_probability.numerator, "exact_rejection_denominator": rejection_probability.denominator, "rejection_probability": float(rejection_probability), "value": float(normalizer), "log_value": math.log1p(-float(rejection_probability))}


def asymmetric_reflection_control() -> dict[str, Any]:
    proportions = np.array([.1, .2, .3, .4])
    base_vector, _ = dirichlet_gap_geometry(proportions)
    cyclic_distances = [float(np.linalg.norm(base_vector - dirichlet_gap_geometry(np.roll(proportions, shift))[0])) for shift in range(4)]
    reflected_gap_distance = float(np.linalg.norm(base_vector - dirichlet_gap_geometry(proportions[::-1])[0]))
    angles = np.r_[0., np.cumsum(rank.TAU * proportions[:-1])]
    vertices = rank.angle_polygon(angles, np.zeros(4)).vertices
    traversal_reversal_distance = float(np.linalg.norm(rank.body_chart(vertices).vector - rank.body_chart(vertices[::-1]).vector))
    return {"asymmetric_gap_proportions": proportions.tolist(), "cyclic_start_chart_distances": cyclic_distances, "same_vertex_set_traversal_reversal_chart_distance": traversal_reversal_distance, "reflected_gap_sequence_chart_distance": reflected_gap_distance, "conclusion": "The n cyclic starts represent the same rotation-quotiented body. Reversing vertex traversal also represents the same vertex set, but reversing the positive gap sequence produces a spatially reflected, generically distinct body; reflection is not quotiented."}


def dirichlet_change_of_variables(proportions: np.ndarray, alphas: tuple[float, ...] = (1., 4., 16.)) -> dict[str, Any]:
    n = len(proportions); coordinates = rank.TAU * proportions[:-1]
    _, base_chart = dirichlet_gap_geometry(proportions)
    steps = []
    for step in DIRICHLET_STEPS:
        jacobian = np.empty((2 * n, n - 1))
        for column in range(n - 1):
            plus = coordinates.copy(); plus[column] += step
            minus = coordinates.copy(); minus[column] -= step
            def expanded(local: np.ndarray) -> np.ndarray:
                return np.r_[local, rank.TAU - np.sum(local)] / rank.TAU
            plus_vector, _ = dirichlet_gap_geometry(expanded(plus), base_chart.key)
            minus_vector, _ = dirichlet_gap_geometry(expanded(minus), base_chart.key)
            jacobian[:, column] = (plus_vector - minus_vector) / (2 * step)
        singular = np.linalg.svd(jacobian, compute_uv=False)
        steps.append({"step": step, "singular_values": singular.tolist(), "log_body_volume_jacobian": float(np.sum(np.log(singular)))})
    log_jacobian = steps[1]["log_body_volume_jacobian"]
    law_densities = {}
    for alpha in alphas:
        integer_alpha = int(alpha)
        if integer_alpha != alpha:
            raise ValueError("exact conditioning normalizer is implemented for integer alpha only")
        normalizer = dirichlet_acceptance_normalizer(n, integer_alpha)
        log_unconditioned = math.lgamma(n * alpha) - n * math.lgamma(alpha) + (alpha - 1) * float(np.sum(np.log(proportions))) - (n - 1) * math.log(rank.TAU)
        log_conditioned = log_unconditioned - normalizer["log_value"]
        single_branch = log_conditioned - log_jacobian
        law_densities[f"alpha={alpha:g}"] = {"acceptance_normalizer": normalizer, "log_unconditioned_density_wrt_gap_coordinates": log_unconditioned, "log_acceptance_conditioned_density_wrt_gap_coordinates": log_conditioned, "log_single_linked_label_branch_density_wrt_body_hausdorff_measure": single_branch, "log_generic_unlabeled_body_density_including_n_cyclic_preimages": single_branch + math.log(n)}
    return {"reference_measure": "Acceptance-conditioned Lebesgue density dg_1...dg_(n-1) on {g_i>0, sum g_i=2pi, max g_i<pi}, with the fixed body-chart Euclidean Hausdorff measure", "discrete_quotient_contract": f"A generic rotation-quotiented body has n={n} cyclic starting-facet preimages of equal Dirichlet density. Reversing clockwise/counterclockwise traversal labels the same vertex set, but reversing the positive gap sequence is a spatial reflection and is not another preimage because reflection is not quotiented.", "steps": steps, "log_body_volume_step_spread": float(max(item["log_body_volume_jacobian"] for item in steps) - min(item["log_body_volume_jacobian"] for item in steps)), "law_densities_at_same_body_point": law_densities}


def reparameterization_counterexamples() -> dict[str, Any]:
    original = np.eye(2); scaled = np.diag([10., 2.]); anisotropic = np.diag([10., .1])
    def item(matrix: np.ndarray) -> dict[str, float]:
        singular = np.linalg.svd(matrix, compute_uv=False); determinant = abs(float(np.linalg.det(matrix)))
        return {"condition_number": float(singular[0] / singular[-1]), "log_pseudodeterminant": float(np.sum(np.log(singular))), "log_transformed_latent_density_for_unit_original_density": math.log(determinant), "log_induced_body_density": math.log(determinant) - float(np.sum(np.log(singular)))}
    return {"coordinate_scaling": {"original": item(original), "scaled_diag_10_2": item(scaled), "anisotropic_det_one_diag_10_point1": item(anisotropic), "conclusion": "Raw pseudo-determinant and condition number change under latent coordinates; density-minus-volume-Jacobian remains invariant when the latent density is transformed."}, "redundant_fiber": {"one_coordinate_singular_value": 1.0, "duplicated_coordinate_map_x_equals_u_plus_v_singular_value": math.sqrt(2), "conclusion": "Adding/reparameterizing fiber coordinates changes nonzero singular values; pushforward density requires coarea integration over the fiber, not inverse pseudo-determinant."}}


def analyze(rank_report_path: Path) -> dict[str, Any]:
    report = json.loads(rank_report_path.read_text())
    if report.get("schema") != "generator-map-jacobian-rank-report-v1": raise ValueError("wrong rank report schema")
    if report["source_contract"].get("source_dirty") is not False: raise ValueError("rank report was not generated from clean source")
    if report["source_contract"]["analyzer_sha256"] != sha256(RANK_ANALYZER): raise ValueError("rank analyzer hash mismatch")
    records = []
    grouped: dict[tuple[str, str, int], list[dict[str, Any]]] = defaultdict(list)
    dirichlet_reference = []
    for source in report["generator_results"]["bases"]:
        base = rank.build_base(source["law"], source["parameter"], source["side_count"], source["seed"])
        item = {"law": source["law"], "parameter": source["parameter"], "side_count": source["side_count"], "seed": source["seed"], "spectrum": spectrum_record(source), "rejection_boundary_margins": base_margins(base)}
        records.append(item); grouped[(item["law"], item["parameter"], item["side_count"])].append(item)
        if item["law"] == "equal-support-dirichlet" and item["parameter"] in {"alpha=1", "alpha=4", "alpha=16"}:
            weights = np.exp(base.latent[1:] - np.max(base.latent[1:])); proportions = weights / np.sum(weights)
            dirichlet_reference.append({"source_population": item["parameter"], "side_count": item["side_count"], "seed": item["seed"], "gap_proportions": proportions.tolist(), "change_of_variables": dirichlet_change_of_variables(proportions)})
    strata = []
    for (law, parameter, n), values in sorted(grouped.items()):
        numeric_margins = defaultdict(list)
        for value in values:
            for name, number in value["rejection_boundary_margins"].items(): numeric_margins[name].append(number)
        conditions = [value["spectrum"]["condition_number_native_latents"] for value in values if value["spectrum"]["condition_number_native_latents"] is not None]
        log_dets = [value["spectrum"]["log_pseudodeterminant_native_latents"] for value in values if value["spectrum"]["log_pseudodeterminant_native_latents"] is not None]
        strata.append({"law": law, "parameter": parameter, "side_count": n, "base_count": len(values), "native_latent_condition_number": summary(conditions) if conditions else None, "native_latent_log_pseudodeterminant": summary(log_dets) if log_dets else None, "rejection_boundary_margins": {name: summary(numbers) for name, numbers in sorted(numeric_margins.items())}})
    dirichlet_summaries = []
    for n in sorted({item["side_count"] for item in dirichlet_reference}):
        for source_population in ("alpha=1", "alpha=4", "alpha=16"):
            selected = [item for item in dirichlet_reference if item["side_count"] == n and item["source_population"] == source_population]
            evaluated = {}
            for alpha in ("alpha=1", "alpha=4", "alpha=16"):
                values = [item["change_of_variables"]["law_densities_at_same_body_point"][alpha]["log_generic_unlabeled_body_density_including_n_cyclic_preimages"] for item in selected]
                evaluated[alpha] = summary(values)
            dirichlet_summaries.append({"side_count": n, "reference_population": source_population, "reference_count": len(selected), "evaluated_law_log_density": evaluated})
    return {"schema": "generator-map-distortion-report-v1", "target_free": True, "source_contract": source_contract(), "inputs": {"rank_report_repository_path": "experiments/sys-datascience/methods/generator-map-jacobian-rank/artifacts/report.json", "rank_report_sha256": sha256(rank_report_path), "rank_analyzer_sha256": sha256(RANK_ANALYZER)}, "reparameterization_counterexamples": reparameterization_counterexamples(), "asymmetric_reflection_control": asymmetric_reflection_control(), "dirichlet_acceptance_normalizers": [dirichlet_acceptance_normalizer(n, alpha) for n in (4, 6, 8) for alpha in (1, 4, 16)], "dirichlet_common_measure_comparisons": dirichlet_reference, "dirichlet_reference_summaries": dirichlet_summaries, "per_base_diagnostics": records, "stratum_summaries": strata, "density_dispositions": {"equal_support_dirichlet_alpha_1_4_16": "implemented on the acceptance-conditioned common gap-simplex Lebesgue measure and fixed body-chart Hausdorff measure", "regular_equal_support": "Dirac law; no density relative to the continuous gap measure", "current_baseline": "abandoned: translation/scale quotient fibers carry nonconstant latent law mass and require coarea integration", "zonogon": "abandoned: rotation/scale fibers and native length-coordinate choice require an explicit quotient reference measure", "regular_mutation": "abandoned: many-to-one step latents, clipping strata, and fibers require coarea integration", "primal_hull": "abandoned: inactive interior points and active-set conditioning contribute fiber probability mass"}, "scientific_disposition": "The acceptance-conditioned common-measure Dirichlet comparison is informative about alpha-dependent concentration around the finite reference bodies. For every other law, this packet stops at native-coordinate conditioning and boundary diagnostics; no cross-law density claim is supported.", "supported_interpretation": "Within a fixed native parameterization, condition numbers describe local sensitivity anisotropy and normalized rejection margins describe proximity to named implementation boundaries. Dirichlet alpha densities are comparable only under the declared acceptance-conditioned common gap/body measures.", "prohibited_interpretation": "Raw pseudo-determinants or condition numbers are not cross-law density, coverage, quality, naturalness, topology, rare-mode mass, target, or sys evidence. Boundary margins are not rejection probabilities."}


def main() -> None:
    parser = argparse.ArgumentParser(); parser.add_argument("--rank-report", type=Path, default=DEFAULT_RANK_REPORT); parser.add_argument("--out-dir", type=Path, required=True); args = parser.parse_args()
    started = time.monotonic(); output = analyze(args.rank_report); args.out_dir.mkdir(parents=True, exist_ok=True); (args.out_dir / "report.json").write_text(json.dumps(output, indent=2, sort_keys=True) + "\n"); print(json.dumps({"out": str(args.out_dir / "report.json"), "runtime_seconds_observed_not_retained": time.monotonic() - started}, sort_keys=True))


if __name__ == "__main__": main()
