"""Derive v2 long-form candidate-formula observations from raw evidence."""
from __future__ import annotations

from collections import Counter
from fractions import Fraction
import json
import math
import sys
from pathlib import Path


def rat(text: str | None) -> Fraction | None:
    if text is None:
        return None
    n, d = text.split("/", 1)
    return Fraction(int(n), int(d))


def norm(x: list[float]) -> float:
    return math.sqrt(sum(a * a for a in x))


def matvec(h: list[list[float]], x: list[float]) -> list[float]:
    return [sum(a * b for a, b in zip(row, x, strict=True)) for row in h]


def frobenius(h: list[list[float]]) -> float:
    return math.sqrt(sum(a * a for row in h for a in row))


def value(kind: str, unit: str, role: str, *, f64: float | None = None, text: str | None = None, boolean: bool | None = None) -> dict:
    return {"value_kind": kind, "unit": unit, "role": role, "f64_value": f64, "text_value": text, "boolean_value": boolean}


def add(evals: list[dict], formula_id: str, row: dict, center: str, status: str, values: list[dict]) -> None:
    evals.append({"formula_id": formula_id, "target_polytope_id": row["target_polytope_id"], "sigma_active_reeb_word": row["sigma_active_reeb_word"], "center": center, "status": status, "values": values})


def exact_beta_truth(status: str) -> str:
    if status in {"positive_beta_q_positive_action", "positive_beta_q_nonpositive"}:
        return "true"
    if status in {"inconsistent", "consistent_no_strict_positive_beta"}:
        return "false"
    raise ValueError(f"unknown exact lifecycle status {status}")


def role_f64(item: dict, role: str) -> float | None:
    for x in item["values"]:
        if x["role"] == role:
            return x["f64_value"]
    return None


def role_bool(item: dict, role: str) -> bool | None:
    for x in item["values"]:
        if x["role"] == role:
            return x["boolean_value"]
    return None


def main() -> None:
    out = Path(sys.argv[1]) if len(sys.argv) > 1 else Path("artifacts/soundness-v2")
    rows = [json.loads(x) for x in (out / "raw_rows.jsonl").read_text().splitlines() if x]
    registry = json.loads((out / "formula_registry.json").read_text())
    registry_ids = {x["formula_id"] for x in registry}
    evals: list[dict] = []
    eligibility: Counter[str] = Counter()
    exact_coverage: Counter[str] = Counter()
    center_counts: Counter[str] = Counter()
    lifecycle_counts: Counter[str] = Counter()
    for row in rows:
        lifecycle = row["exact_lifecycle_status"]
        lifecycle_counts[lifecycle] += 1
        beta_truth = exact_beta_truth(lifecycle)
        target_q = rat(row["exact_positive_witness_q"])
        unique_exact = row["exact_row_reduction_system_status"] == "consistent_unique"
        target_beta = [float(rat(x)) for x in row["exact_positive_witness_beta"]] if lifecycle.startswith("positive_beta") and unique_exact else None
        exact_action = rat(row["exact_positive_witness_action"])
        singular = [x for x in row["kkt_augmented_singular_values_f64"] if x > 0.0]
        eigen = [abs(x) for x in row["kkt_augmented_eigenvalues_f64"] if abs(x) > 0.0]
        sigma_min, eigen_min = (min(singular) if singular else None), (min(eigen) if eigen else None)
        h = row["qp_objective_hessian_h_f64"]
        for c in row["centers"]:
            name = c["center_id"]
            center_counts[name] += c["center_availability"] == "available"
            numeric_full_rank = c["center_rank_f64"] == row["sigma_length"] + 5
            q_raw = c["center_q_raw_f64"]
            raw_id = f"error_q_abs__{name}_q_raw__to_exact_positive_witness_q"
            if q_raw is not None and target_q is not None:
                observed = abs(q_raw - float(target_q))
                eligibility[raw_id] += 1; exact_coverage[raw_id] += 1
                add(evals, raw_id, row, name, "observed_not_a_bound", [value("absolute_error", "Q", "observed_error", f64=observed)])
            residual = c["center_full_kkt_residual_norm_f64"]
            qbound_id = f"bound_q_abs__{name}__eigen_residual_9over2"
            qbound = None
            if residual is not None and eigen_min is not None and numeric_full_rank:
                qbound = 4.5 * residual * residual / eigen_min
                observed = None if target_q is None or c["center_q_corrected_f64"] is None else abs(c["center_q_corrected_f64"] - float(target_q))
                sound = None if observed is None else observed <= qbound
                eligibility[qbound_id] += 1; exact_coverage[qbound_id] += observed is not None
                add(evals, qbound_id, row, name, "conjectured_candidate_bound", [value("absolute_bound", "Q", "candidate_bound", f64=qbound), value("absolute_error", "Q", "observed_error", f64=observed), value("truth", "boolean", "sound", boolean=sound)])
            radius_id = f"bound_beta_l2__{name}__inverse_singular_residual"
            radius = None
            if residual is not None and sigma_min is not None and numeric_full_rank:
                radius = residual / sigma_min
                observed = None if target_beta is None or c["center_beta_f64"] is None else norm([a-b for a,b in zip(c["center_beta_f64"], target_beta, strict=True)])
                sound = None if observed is None else observed <= radius
                eligibility[radius_id] += 1; exact_coverage[radius_id] += observed is not None
                add(evals, radius_id, row, name, "heuristic_inverse_norm_diagnostic", [value("l2_bound", "beta", "candidate_bound", f64=radius), value("l2_error", "beta", "observed_error_unique_exact_only", f64=observed), value("truth", "boolean", "sound", boolean=sound)])
            qprop_id = f"bound_q_abs__{name}__beta_radius_first_plus_quadratic"
            beta = c["center_beta_f64"]
            if radius is not None and beta is not None:
                bound = norm(matvec(h, beta))*radius + 0.5*frobenius(h)*radius*radius
                observed = None if target_q is None or q_raw is None else abs(q_raw-float(target_q))
                sound = None if observed is None else observed <= bound
                eligibility[qprop_id] += 1; exact_coverage[qprop_id] += observed is not None
                add(evals, qprop_id, row, name, "heuristic_beta_radius_propagation", [value("absolute_bound", "Q", "candidate_bound", f64=bound), value("absolute_error", "Q", "observed_error", f64=observed), value("truth", "boolean", "sound", boolean=sound)])
            action_id = f"interval_action__{name}__positive_q_monotone"
            corrected = c["center_q_corrected_f64"]
            if corrected is not None and qbound is not None and corrected-qbound > 0.0:
                lower, upper = 0.5/(corrected+qbound), 0.5/(corrected-qbound)
                sound = None if exact_action is None else lower <= float(exact_action) <= upper
                eligibility[action_id] += 1; exact_coverage[action_id] += exact_action is not None
                add(evals, action_id, row, name, "candidate_interval_not_verified", [value("interval_endpoint", "action", "lower", f64=lower), value("interval_endpoint", "action", "upper", f64=upper), value("action", "action", "exact_target", f64=None if exact_action is None else float(exact_action)), value("truth", "boolean", "sound", boolean=sound)])
            predicate_id = f"predicate_beta_positive__{name}__from_radius"
            margin = c["center_beta_margin_f64"]
            if margin is not None and radius is not None:
                predicate = "true" if margin > radius else "false" if margin < -radius else "indeterminate"
                sound = (predicate != "false") if beta_truth == "true" else (predicate != "true")
                eligibility[predicate_id] += 1; exact_coverage[predicate_id] += 1
                add(evals, predicate_id, row, name, "heuristic_ternary_predicate", [value("ternary_predicate", "truth", "f64_predicate", text=predicate), value("ternary_predicate", "truth", "exact_beta_truth", text=beta_truth), value("truth", "boolean", "sound", boolean=sound)])
    emitted = {x["formula_id"] for x in evals}
    if emitted - registry_ids:
        raise SystemExit(f"analyzer emitted unregistered formulas: {sorted(emitted-registry_ids)}")
    coverage = {}
    for formula in registry:
        items = [x for x in evals if x["formula_id"] == formula["formula_id"]]
        sound_items = [x for x in items if role_bool(x, "sound") is not None]
        slack = [role_f64(x,"candidate_bound")-role_f64(x,"observed_error") for x in sound_items if role_f64(x,"candidate_bound") is not None and role_f64(x,"observed_error") is not None]
        coverage[formula["formula_id"]] = {"eligible_rows":eligibility[formula["formula_id"]], "exact_target_covered_rows":exact_coverage[formula["formula_id"]], "undercoverage_rows":eligibility[formula["formula_id"]]-exact_coverage[formula["formula_id"]], "evaluated_rows":len(items), "sound_rows":sum(role_bool(x,"sound") is True for x in sound_items), "unsound_rows":sum(role_bool(x,"sound") is False for x in sound_items), "sharpness_min_slack":min(slack) if slack else None, "sharpness_max_slack":max(slack) if slack else None}
    with (out/"formula_evaluations.jsonl").open("w") as f:
        for item in evals: f.write(json.dumps(item,sort_keys=True)+"\n")
    analysis={"schema_version":"qp-soundness-analysis-v2","raw_row_count":len(rows),"formula_registry_entry_count":len(registry),"emitted_formula_entry_count":len(emitted),"formula_evaluation_value_schema":"long-form-value-v1","formula_coverage":coverage,"available_center_counts":dict(center_counts),"exact_lifecycle_status":dict(lifecycle_counts),"interpretation_boundary":"Candidate bounds are empirical diagnostics, not verified enclosures. Exact Q comparisons use either positive-beta Q sign; beta-vector errors remain unique-exact only. HKO remains stored-binary64 rational only."}
    (out/"analysis.json").write_text(json.dumps(analysis,indent=2,sort_keys=True)+"\n")
    (out/"interpretation.md").write_text("# QP soundness v2 interpretation boundary\n\nFormula evaluations use the stable `long-form-value-v1` schema. Exact lifecycle separates inconsistent, no strict-positive beta, positive-beta/nonpositive-Q, and positive-beta/positive-Q-action outcomes. Only unique exact systems support vector beta-error comparisons.\n")


if __name__ == "__main__":
    main()
