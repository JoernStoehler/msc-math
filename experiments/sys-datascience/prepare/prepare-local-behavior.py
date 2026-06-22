# /// script
# requires-python = ">=3.12"
# dependencies = ["numpy"]
# ///

"""Prepare reusable local-behavior feature tables from a run-local producer dir."""

from __future__ import annotations

import argparse
import csv
import json
import math
from collections import defaultdict
from pathlib import Path
from typing import Any

import numpy as np


POSITIVE_FLOOR = 1.0e-300
DEFAULT_BRANCH_THRESHOLD_RELATIVE = 1.0e-3


def read_jsonl(path: Path) -> list[dict[str, Any]]:
    if not path.exists():
        return []
    rows: list[dict[str, Any]] = []
    with path.open("r", encoding="utf-8") as handle:
        for line in handle:
            line = line.strip()
            if line:
                rows.append(json.loads(line))
    return rows


def write_jsonl(path: Path, rows: list[dict[str, Any]]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w", encoding="utf-8") as handle:
        for row in rows:
            handle.write(json.dumps(clean_json(row), sort_keys=True) + "\n")


def clean_json(value: Any) -> Any:
    if isinstance(value, dict):
        return {key: clean_json(item) for key, item in value.items()}
    if isinstance(value, list):
        return [clean_json(item) for item in value]
    if isinstance(value, tuple):
        return [clean_json(item) for item in value]
    if isinstance(value, np.ndarray):
        return clean_json(value.tolist())
    if isinstance(value, np.generic):
        return clean_json(value.item())
    if isinstance(value, float):
        return value if math.isfinite(value) else None
    return value


def sigma_key(sigma: list[int] | tuple[int, ...]) -> tuple[int, ...]:
    if not sigma:
        return ()
    items = tuple(int(item) for item in sigma)
    rotations = [items[index:] + items[:index] for index in range(len(items))]
    return min(rotations)


def sigma_label(sigma: tuple[int, ...]) -> str:
    return "-".join(str(item) for item in sigma)


def flat_duals(row: dict[str, Any]) -> np.ndarray:
    return np.array(row["dual_vertices"], dtype=float).reshape(-1)


def flat_vector_rows(rows: list[list[float]] | list[tuple[float, ...]]) -> np.ndarray:
    return np.array(rows, dtype=float).reshape(-1)


def omega0(left: np.ndarray, right: np.ndarray) -> float:
    return float(left[0] * right[2] + left[1] * right[3] - left[2] * right[0] - left[3] * right[1])


def kkt_solve(dual_vertices: list[list[float]], sigma: tuple[int, ...]) -> dict[str, Any]:
    duals = np.array(dual_vertices, dtype=float)
    m = len(sigma)
    size = m + 5
    kkt = np.zeros((size, size), dtype=float)
    rhs = np.zeros(size, dtype=float)
    for i in range(m):
        for j in range(i + 1, m):
            value = omega0(duals[sigma[i]], duals[sigma[j]])
            kkt[i, j] = value
            kkt[j, i] = value
    for i, facet_index in enumerate(sigma):
        for coord in range(4):
            value = duals[facet_index, coord]
            kkt[i, m + coord] = value
            kkt[m + coord, i] = value
        kkt[i, m + 4] = 1.0
        kkt[m + 4, i] = 1.0
    rhs[m + 4] = 1.0

    try:
        solution = np.linalg.solve(kkt, rhs)
        outcome = "linear_solve_positive_q"
    except np.linalg.LinAlgError:
        return {
            "kkt_outcome": "singular_matrix",
            "action": None,
            "q": None,
            "beta_min": None,
            "beta_max": None,
            "beta_strictly_positive_f64": None,
            "admissible_stationary_point_f64": None,
            "residual_norm": None,
            **eigen_summary(kkt),
        }

    beta = solution[:m]
    h = kkt[:m, :m]
    q = 0.5 * float(beta @ h @ beta)
    residual = float(np.linalg.norm(kkt @ solution - rhs))
    beta_min = float(np.min(beta))
    beta_max = float(np.max(beta))
    if not math.isfinite(q) or q <= 0.0:
        action = None
        outcome = "linear_solve_nonpositive_q"
    else:
        action = 0.5 / q
    return {
        "kkt_outcome": outcome,
        "action": action,
        "q": q,
        "beta_min": beta_min,
        "beta_max": beta_max,
        "beta_strictly_positive_f64": bool(beta_min > 0.0),
        "admissible_stationary_point_f64": bool(action is not None and beta_min > 0.0),
        "residual_norm": residual,
        **eigen_summary(kkt),
    }


def eigen_summary(matrix: np.ndarray) -> dict[str, Any]:
    values = np.linalg.eigvalsh(matrix)
    max_abs = float(np.max(np.abs(values))) if values.size else 0.0
    threshold = max_abs * 1.0e-3
    positives = values[values > threshold]
    negatives = values[values < -threshold]
    return {
        "kkt_n_positive_strict": int(positives.size),
        "kkt_n_negative_strict": int(negatives.size),
        "kkt_n_zero_strict": int(values.size - positives.size - negatives.size),
        "kkt_min_abs_eigenvalue": float(np.min(np.abs(values))) if values.size else 0.0,
        "kkt_max_abs_eigenvalue": max_abs,
        "kkt_strict_eigen_threshold": threshold,
    }


def branch_sys(action: float | None, volume: float | None) -> float | None:
    if action is None or volume is None or volume <= 0.0:
        return None
    return action * action / (2.0 * volume)


def payload_sigma_actions(payload: dict[str, Any]) -> dict[tuple[int, ...], float]:
    actions: dict[tuple[int, ...], float] = {}
    for item in payload.get("sigmas", []):
        sigma = sigma_key(item.get("perm", []))
        action = item.get("action")
        if action is not None:
            actions[sigma] = float(action)
    return actions


def symmetry_tangent_matrix(duals: list[list[float]]) -> np.ndarray:
    rows: list[np.ndarray] = []
    n = len(duals)
    identity = np.eye(4)
    for coord in range(4):
        row = np.zeros((n, 4))
        row[:, coord] = 1.0
        rows.append(row.reshape(-1))
    rows.append(np.array(duals, dtype=float).reshape(-1))

    generators = [
        np.array([[1, 0, 0, 0], [0, 0, 0, 0], [0, 0, -1, 0], [0, 0, 0, 0]], dtype=float),
        np.array([[0, 0, 0, 0], [0, 1, 0, 0], [0, 0, 0, 0], [0, 0, 0, -1]], dtype=float),
        np.array([[0, 1, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, -1, 0]], dtype=float),
        np.array([[0, 0, 0, 0], [1, 0, 0, 0], [0, 0, 0, -1], [0, 0, 0, 0]], dtype=float),
        np.array([[0, 0, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 1, 0, 0]], dtype=float),
        np.array([[0, 0, 0, 0], [0, 0, 0, 1], [1, 0, 0, 0], [0, 0, 0, 0]], dtype=float),
    ]
    dual_array = np.array(duals, dtype=float)
    for generator in generators:
        rows.append((dual_array @ generator.T).reshape(-1))
    if not rows:
        return np.zeros((0, 0))
    return np.stack(rows, axis=1)


def quotient_distance(base_duals: list[list[float]], target_duals: list[list[float]]) -> float | None:
    delta = flat_vector_rows(target_duals) - flat_vector_rows(base_duals)
    tangent = symmetry_tangent_matrix(base_duals)
    if tangent.size == 0:
        return float(np.linalg.norm(delta))
    q, r = np.linalg.qr(tangent)
    diag = np.abs(np.diag(r)) if r.size else np.array([])
    rank = int(np.sum(diag > 1.0e-9))
    if rank == 0:
        return float(np.linalg.norm(delta))
    projection = q[:, :rank] @ (q[:, :rank].T @ delta)
    return float(np.linalg.norm(delta - projection))


def direction_family(label: str) -> str:
    if label.startswith("random_unit_direction"):
        return "random"
    if "gradient" in label or "maximin" in label:
        return "gradient"
    return label


def make_branch_facts(
    samples: list[dict[str, Any]],
    computed: dict[str, dict[str, Any]],
    gradients: list[dict[str, Any]],
    branch_thresholds_by_basepoint: dict[str, float],
) -> list[dict[str, Any]]:
    gradient_sigmas_by_basepoint: dict[str, set[tuple[int, ...]]] = defaultdict(set)
    for row in gradients:
        gradient_sigmas_by_basepoint[row["basepoint_id"]].add(sigma_key(row["sigma"]))

    rows: list[dict[str, Any]] = []
    seen: set[tuple[str, str, tuple[int, ...]]] = set()
    for sample in samples:
        if sample["status"] != "ok" or not sample.get("target_poly_id"):
            continue
        base_payload = computed[sample["base_poly_id"]]
        target_payload = computed[sample["target_poly_id"]]
        base_actions = payload_sigma_actions(base_payload)
        target_actions = payload_sigma_actions(target_payload)
        sigmas = set(base_actions) | set(target_actions) | gradient_sigmas_by_basepoint[sample["basepoint_id"]]
        branch_threshold_relative = branch_thresholds_by_basepoint.get(
            sample["basepoint_id"], DEFAULT_BRANCH_THRESHOLD_RELATIVE
        )
        for point_role, payload, actions, counterpart_id in [
            ("base", base_payload, base_actions, sample["target_poly_id"]),
            ("target", target_payload, target_actions, sample["base_poly_id"]),
        ]:
            min_action = min(actions.values()) if actions else None
            for sigma in sorted(sigmas):
                key = (sample["sample_id"], point_role, sigma)
                if key in seen:
                    continue
                seen.add(key)
                candidate_action = actions.get(sigma)
                rel_gap = None
                if candidate_action is not None and min_action and min_action > 0.0:
                    rel_gap = candidate_action / min_action - 1.0
                kkt = kkt_solve(payload["dual_vertices"], sigma)
                source_tags = []
                if sigma in actions:
                    source_tags.append(f"{point_role}_candidate_window")
                if sigma in gradient_sigmas_by_basepoint[sample["basepoint_id"]]:
                    source_tags.append("base_gradient_branch")
                rows.append(
                    {
                        "branch_fact_id": f"{sample['sample_id']}:{point_role}:{sigma_label(sigma)}",
                        "sample_id": sample["sample_id"],
                        "basepoint_id": sample["basepoint_id"],
                        "point_role": point_role,
                        "poly_id": payload["poly_id"],
                        "counterpart_poly_id": counterpart_id,
                        "direction_label": sample["direction_label"],
                        "radius": sample["radius"],
                        "sigma": list(sigma),
                        "sigma_len": len(sigma),
                        "source_tags": source_tags,
                        "is_candidate_window": sigma in actions,
                        "is_min_action_branch": rel_gap is not None and abs(rel_gap) <= 1.0e-10,
                        "is_near_active_branch": rel_gap is not None and rel_gap <= branch_threshold_relative,
                        "near_active_threshold_relative": branch_threshold_relative,
                        "candidate_action": candidate_action,
                        "candidate_relative_action_gap_from_min": rel_gap,
                        "transition_allowed": None,
                        "transition_status": "not_evaluated_in_prepare",
                        **kkt,
                    }
                )
    return rows


def index_branch_facts(rows: list[dict[str, Any]]) -> dict[tuple[str, str, tuple[int, ...]], dict[str, Any]]:
    return {
        (row["sample_id"], row["point_role"], sigma_key(row["sigma"])): row
        for row in rows
    }


def make_pairs(
    samples: list[dict[str, Any]],
    computed: dict[str, dict[str, Any]],
    branch_facts: list[dict[str, Any]],
    gradients: list[dict[str, Any]],
) -> list[dict[str, Any]]:
    facts_by_sample_role: dict[tuple[str, str], list[dict[str, Any]]] = defaultdict(list)
    for row in branch_facts:
        facts_by_sample_role[(row["sample_id"], row["point_role"])].append(row)
    gradients_by_basepoint: dict[str, list[dict[str, Any]]] = defaultdict(list)
    for row in gradients:
        gradients_by_basepoint[row["basepoint_id"]].append(row)

    rows: list[dict[str, Any]] = []
    for sample in samples:
        if sample["status"] != "ok" or not sample.get("target_poly_id"):
            continue
        base_payload = computed[sample["base_poly_id"]]
        target_payload = computed[sample["target_poly_id"]]
        delta = flat_duals(target_payload) - flat_duals(base_payload)
        base_facts = facts_by_sample_role[(sample["sample_id"], "base")]
        target_facts = facts_by_sample_role[(sample["sample_id"], "target")]
        base_min = {sigma_key(row["sigma"]) for row in base_facts if row["is_min_action_branch"]}
        target_min = {sigma_key(row["sigma"]) for row in target_facts if row["is_min_action_branch"]}
        base_candidate = {sigma_key(row["sigma"]) for row in base_facts if row["is_candidate_window"]}
        base_near = {sigma_key(row["sigma"]) for row in base_facts if row["is_near_active_branch"]}
        base_near_thresholds = {
            row.get("near_active_threshold_relative")
            for row in base_facts
            if row.get("near_active_threshold_relative") is not None
        }
        branch_predictions = []
        for gradient in gradients_by_basepoint[sample["basepoint_id"]]:
            grad = flat_vector_rows(gradient["sys_sigma_gradient"])
            branch_predictions.append(float(delta @ grad))
        clarke_predicted_delta = min(branch_predictions) if branch_predictions else None
        observed_delta = sample.get("observed_delta_sys")
        prediction_error = (
            observed_delta - clarke_predicted_delta
            if observed_delta is not None and clarke_predicted_delta is not None
            else None
        )
        status = target_branch_status(target_min, base_min, base_near, base_candidate)
        rows.append(
            {
                "sample_id": sample["sample_id"],
                "basepoint_id": sample["basepoint_id"],
                "input_poly_id": sample["input_poly_id"],
                "base_poly_id": sample["base_poly_id"],
                "target_poly_id": sample["target_poly_id"],
                "direction_label": sample["direction_label"],
                "direction_family": direction_family(sample["direction_label"]),
                "radius": sample["radius"],
                "base_sys": sample["base_sys"],
                "target_sys": sample["target_sys"],
                "observed_delta_sys": observed_delta,
                "producer_predicted_delta_sys": sample.get("predicted_delta_sys"),
                "clarke_predicted_delta_sys": clarke_predicted_delta,
                "clarke_prediction_error": prediction_error,
                "clarke_prediction_abs_error": abs(prediction_error) if prediction_error is not None else None,
                "ambient_distance": float(np.linalg.norm(delta)),
                "symmetry_quotient_distance": quotient_distance(
                    base_payload["dual_vertices"], target_payload["dual_vertices"]
                ),
                "base_min_branch_count": len(base_min),
                "target_min_branch_count": len(target_min),
                "min_branch_sets_equal": bool(base_min and base_min == target_min),
                "min_branch_sets_intersect": bool(base_min & target_min),
                "target_min_branches_all_in_base_min_branch_set": bool(target_min and target_min <= base_min),
                "target_min_branches_all_in_base_candidate_window": bool(target_min and target_min <= base_candidate),
                "target_min_branches_all_in_base_near_active": bool(target_min and target_min <= base_near),
                "target_branch_status_at_base": status,
                "near_active_threshold_relative": (
                    sorted(base_near_thresholds)[0] if len(base_near_thresholds) == 1 else None
                ),
                "branch_gradient_count": len(branch_predictions),
            }
        )
    return rows


def target_branch_status(
    target_min: set[tuple[int, ...]],
    base_min: set[tuple[int, ...]],
    base_near: set[tuple[int, ...]],
    base_candidate: set[tuple[int, ...]],
) -> str:
    if not target_min:
        return "no_target_min_branch"
    if target_min == base_min:
        return "same_min_branch_set"
    if target_min <= base_min:
        return "target_min_subset_of_base_min_branch_set"
    if target_min & base_min:
        return "target_min_partly_in_base_min_branch_set"
    if target_min <= base_near:
        return "target_min_in_base_near_active"
    if target_min <= base_candidate:
        return "target_min_in_base_candidate_window"
    if target_min & base_candidate:
        return "target_min_partly_in_base_candidate_window"
    return "target_min_missing_from_base_candidate_window"


def make_branch_variation(
    samples: list[dict[str, Any]],
    computed: dict[str, dict[str, Any]],
    branch_facts: list[dict[str, Any]],
) -> list[dict[str, Any]]:
    facts = index_branch_facts(branch_facts)
    rows: list[dict[str, Any]] = []
    for sample in samples:
        if sample["status"] != "ok" or not sample.get("target_poly_id"):
            continue
        base_payload = computed[sample["base_poly_id"]]
        target_payload = computed[sample["target_poly_id"]]
        sigmas = {
            sigma
            for sample_id, _role, sigma in facts
            if sample_id == sample["sample_id"]
        }
        for sigma in sorted(sigmas):
            base = facts.get((sample["sample_id"], "base", sigma))
            target = facts.get((sample["sample_id"], "target", sigma))
            if not base or not target:
                continue
            base_sys_sigma = branch_sys(base.get("action"), base_payload.get("volume"))
            target_sys_sigma = branch_sys(target.get("action"), target_payload.get("volume"))
            if base_sys_sigma is None or target_sys_sigma is None:
                continue
            delta = target_sys_sigma - base_sys_sigma
            scale = max(abs(base_sys_sigma), POSITIVE_FLOOR)
            rows.append(
                {
                    "sample_id": sample["sample_id"],
                    "basepoint_id": sample["basepoint_id"],
                    "sigma": list(sigma),
                    "direction_label": sample["direction_label"],
                    "direction_family": direction_family(sample["direction_label"]),
                    "radius": sample["radius"],
                    "base_sys_sigma": base_sys_sigma,
                    "target_sys_sigma": target_sys_sigma,
                    "delta_sys_sigma": delta,
                    "relative_abs_delta_sys_sigma": abs(delta) / scale,
                    "base_is_min_action_branch": base["is_min_action_branch"],
                    "target_is_min_action_branch": target["is_min_action_branch"],
                    "base_candidate_relative_action_gap_from_min": base.get(
                        "candidate_relative_action_gap_from_min"
                    ),
                    "target_candidate_relative_action_gap_from_min": target.get(
                        "candidate_relative_action_gap_from_min"
                    ),
                    "base_kkt_outcome": base["kkt_outcome"],
                    "target_kkt_outcome": target["kkt_outcome"],
                }
            )
    return rows


def make_gradient_projections(
    pairs: list[dict[str, Any]],
    samples: list[dict[str, Any]],
    computed: dict[str, dict[str, Any]],
    gradients: list[dict[str, Any]],
) -> list[dict[str, Any]]:
    samples_by_id = {row["sample_id"]: row for row in samples}
    pairs_by_id = {row["sample_id"]: row for row in pairs}
    gradients_by_basepoint: dict[str, list[dict[str, Any]]] = defaultdict(list)
    for row in gradients:
        gradients_by_basepoint[row["basepoint_id"]].append(row)
    rows: list[dict[str, Any]] = []
    for sample_id, pair in pairs_by_id.items():
        sample = samples_by_id[sample_id]
        base_payload = computed[sample["base_poly_id"]]
        target_payload = computed[sample["target_poly_id"]]
        delta = flat_duals(target_payload) - flat_duals(base_payload)
        delta_norm = float(np.linalg.norm(delta))
        for gradient in gradients_by_basepoint[sample["basepoint_id"]]:
            grad = flat_vector_rows(gradient["sys_sigma_gradient"])
            grad_norm = float(np.linalg.norm(grad))
            predicted_delta = float(delta @ grad)
            if grad_norm > 0.0:
                projection_scalar = predicted_delta / grad_norm
                orthogonal_sq = max(0.0, delta_norm * delta_norm - projection_scalar * projection_scalar)
                orthogonal_norm = math.sqrt(orthogonal_sq)
            else:
                projection_scalar = None
                orthogonal_norm = delta_norm
            rows.append(
                {
                    "sample_id": sample_id,
                    "basepoint_id": sample["basepoint_id"],
                    "sigma": gradient["sigma"],
                    "orbit_index": gradient["orbit_index"],
                    "direction_label": sample["direction_label"],
                    "direction_family": direction_family(sample["direction_label"]),
                    "radius": sample["radius"],
                    "ambient_distance": pair["ambient_distance"],
                    "delta_norm": delta_norm,
                    "gradient_norm": grad_norm,
                    "projection_scalar": projection_scalar,
                    "orthogonal_residual_norm": orthogonal_norm,
                    "branch_predicted_delta_sys": predicted_delta,
                    "observed_delta_sys": pair["observed_delta_sys"],
                    "prediction_error": (
                        pair["observed_delta_sys"] - predicted_delta
                        if pair["observed_delta_sys"] is not None
                        else None
                    ),
                }
            )
    return rows


def quantile(values: list[float], q: float) -> float | None:
    clean = [value for value in values if value is not None and math.isfinite(value)]
    if not clean:
        return None
    return float(np.quantile(clean, q))


def write_radius_summary(path: Path, pairs: list[dict[str, Any]]) -> None:
    groups: dict[tuple[float, str], list[dict[str, Any]]] = defaultdict(list)
    for row in pairs:
        groups[(float(row["radius"]), row["direction_family"])].append(row)
    fields = [
        "radius",
        "direction_family",
        "n",
        "min_branch_equal_fraction",
        "min_branch_intersect_fraction",
        "target_min_in_base_candidate_fraction",
        "target_min_in_base_near_fraction",
        "median_ambient_distance",
        "median_symmetry_quotient_distance",
        "median_abs_clarke_prediction_error",
        "p90_abs_clarke_prediction_error",
        "median_observed_delta_sys",
    ]
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w", newline="", encoding="utf-8") as handle:
        writer = csv.DictWriter(handle, fieldnames=fields)
        writer.writeheader()
        for (radius, family), rows in sorted(groups.items()):
            n = len(rows)
            writer.writerow(
                {
                    "radius": radius,
                    "direction_family": family,
                    "n": n,
                    "min_branch_equal_fraction": sum(row["min_branch_sets_equal"] for row in rows) / n,
                    "min_branch_intersect_fraction": sum(row["min_branch_sets_intersect"] for row in rows) / n,
                    "target_min_in_base_candidate_fraction": sum(
                        row["target_min_branches_all_in_base_candidate_window"] for row in rows
                    )
                    / n,
                    "target_min_in_base_near_fraction": sum(
                        row["target_min_branches_all_in_base_near_active"] for row in rows
                    )
                    / n,
                    "median_ambient_distance": quantile(
                        [row["ambient_distance"] for row in rows], 0.5
                    ),
                    "median_symmetry_quotient_distance": quantile(
                        [row["symmetry_quotient_distance"] for row in rows], 0.5
                    ),
                    "median_abs_clarke_prediction_error": quantile(
                        [row["clarke_prediction_abs_error"] for row in rows], 0.5
                    ),
                    "p90_abs_clarke_prediction_error": quantile(
                        [row["clarke_prediction_abs_error"] for row in rows], 0.9
                    ),
                    "median_observed_delta_sys": quantile(
                        [row["observed_delta_sys"] for row in rows], 0.5
                    ),
                }
            )


def main() -> None:
    parser = argparse.ArgumentParser(
        description="Prepare reusable local-behavior feature tables from a producer run."
    )
    parser.add_argument("run_dir", type=Path)
    parser.add_argument("--out-dir", type=Path)
    args = parser.parse_args()

    run_dir = args.run_dir.resolve()
    out_dir = args.out_dir.resolve() if args.out_dir else run_dir / "prepared"
    computed_rows = read_jsonl(run_dir / "computed-polytopes.jsonl")
    basepoints = read_jsonl(run_dir / "local-behavior-basepoints.jsonl")
    samples = read_jsonl(run_dir / "local-behavior-samples.jsonl")
    gradients = read_jsonl(run_dir / "local-behavior-branch-gradients.jsonl")
    computed = {row["poly_id"]: row for row in computed_rows}
    branch_thresholds_by_basepoint = {
        row["basepoint_id"]: float(row.get("branch_threshold_relative", DEFAULT_BRANCH_THRESHOLD_RELATIVE))
        for row in basepoints
        if row.get("basepoint_id")
    }

    missing = sorted(
        {
            poly_id
            for sample in samples
            for poly_id in [sample.get("base_poly_id"), sample.get("target_poly_id")]
            if poly_id and poly_id not in computed
        }
    )
    if missing:
        raise SystemExit(f"computed-polytopes.jsonl is missing {len(missing)} poly_id values")

    branch_facts = make_branch_facts(samples, computed, gradients, branch_thresholds_by_basepoint)
    pairs = make_pairs(samples, computed, branch_facts, gradients)
    branch_variation = make_branch_variation(samples, computed, branch_facts)
    gradient_projections = make_gradient_projections(pairs, samples, computed, gradients)

    write_jsonl(out_dir / "local-behavior-branch-facts.jsonl", branch_facts)
    write_jsonl(out_dir / "local-behavior-pairs.jsonl", pairs)
    write_jsonl(out_dir / "local-behavior-branch-variation.jsonl", branch_variation)
    write_jsonl(out_dir / "local-behavior-gradient-projections.jsonl", gradient_projections)
    write_radius_summary(out_dir / "local-behavior-radius-summary.csv", pairs)
    print(
        f"prepared pairs={len(pairs)} branch_facts={len(branch_facts)} "
        f"branch_variation={len(branch_variation)} gradient_projections={len(gradient_projections)} "
        f"out_dir={out_dir}"
    )


if __name__ == "__main__":
    main()
