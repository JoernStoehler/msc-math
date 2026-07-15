"""Derive candidate-bound coverage and observations from v2 raw evidence."""
from __future__ import annotations

from collections import Counter, defaultdict
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


def add(evals: list[dict], formula_id: str, row: dict, center: str, **values: object) -> None:
    evals.append({
        "formula_id": formula_id,
        "target_polytope_id": row["target_polytope_id"],
        "sigma_active_reeb_word": row["sigma_active_reeb_word"],
        "center": center,
        **values,
    })


def main() -> None:
    out = Path(sys.argv[1]) if len(sys.argv) > 1 else Path("artifacts/soundness-v2")
    rows = [json.loads(x) for x in (out / "raw_rows.jsonl").read_text().splitlines() if x]
    registry = json.loads((out / "formula_registry.json").read_text())
    registry_ids = {x["formula_id"] for x in registry}
    evals: list[dict] = []
    eligibility: Counter[str] = Counter()
    exact_coverage: Counter[str] = Counter()
    center_counts: Counter[str] = Counter()
    exact_status: Counter[str] = Counter()
    for row in rows:
        exact_status[row["exact_positive_witness_status"]] += 1
        target_q = rat(row["exact_positive_witness_q"]) if row["exact_positive_witness_status"] == "exists" else None
        unique_exact = row["exact_row_reduction_system_status"] == "consistent_unique"
        target_beta = [float(rat(x)) for x in row["exact_positive_witness_beta"]] if row["exact_positive_witness_status"] == "exists" and unique_exact else None
        singular = [x for x in row["kkt_augmented_singular_values_f64"] if x > 0.0]
        eigen = [abs(x) for x in row["kkt_augmented_eigenvalues_f64"] if abs(x) > 0.0]
        sigma_min = min(singular) if singular else None
        eigen_min = min(eigen) if eigen else None
        h = row["qp_objective_hessian_h_f64"]
        for c in row["centers"]:
            name = c["center_id"]
            center_counts[name] += c["center_availability"] == "available"
            numeric_full_rank = c["center_rank_f64"] == row["sigma_length"] + 5
            raw_id = f"error_q_abs__{name}_q_raw__to_exact_positive_witness_q"
            q_raw = c["center_q_raw_f64"]
            if q_raw is not None and target_q is not None:
                eligibility[raw_id] += 1
                exact_coverage[raw_id] += 1
                add(evals, raw_id, row, name, exact_target="exact_positive_witness_q", observed_error_abs=abs(q_raw - float(target_q)), status="observed_not_a_bound")
            residual = c["center_full_kkt_residual_norm_f64"]
            qbound_id = f"bound_q_abs__{name}__eigen_residual_9over2"
            qbound = None
            if residual is not None and eigen_min is not None and numeric_full_rank:
                qbound = 4.5 * residual * residual / eigen_min
                eligibility[qbound_id] += 1
                observed = None if target_q is None or c["center_q_corrected_f64"] is None else abs(c["center_q_corrected_f64"] - float(target_q))
                if observed is not None:
                    exact_coverage[qbound_id] += 1
                add(evals, qbound_id, row, name, candidate_bound_abs=qbound, observed_error_abs=observed, sound=None if observed is None else observed <= qbound, status="conjectured_candidate_bound")
            radius_id = f"bound_beta_l2__{name}__inverse_singular_residual"
            radius = None
            if residual is not None and sigma_min is not None and numeric_full_rank:
                radius = residual / sigma_min
                eligibility[radius_id] += 1
                observed = None if target_beta is None or c["center_beta_f64"] is None else norm([a - b for a, b in zip(c["center_beta_f64"], target_beta, strict=True)])
                if observed is not None:
                    exact_coverage[radius_id] += 1
                add(evals, radius_id, row, name, candidate_bound_l2=radius, observed_error_l2=observed, sound=None if observed is None else observed <= radius, status="heuristic_inverse_norm_diagnostic")
            qprop_id = f"bound_q_abs__{name}__beta_radius_first_plus_quadratic"
            beta = c["center_beta_f64"]
            qprop = None
            if radius is not None and beta is not None:
                qprop = norm(matvec(h, beta)) * radius + 0.5 * frobenius(h) * radius * radius
                eligibility[qprop_id] += 1
                observed = None if target_q is None or q_raw is None else abs(q_raw - float(target_q))
                if observed is not None:
                    exact_coverage[qprop_id] += 1
                add(evals, qprop_id, row, name, candidate_bound_abs=qprop, observed_error_abs=observed, sound=None if observed is None else observed <= qprop, status="heuristic_beta_radius_propagation")
            action_id = f"interval_action__{name}__positive_q_monotone"
            corrected = c["center_q_corrected_f64"]
            if corrected is not None and qbound is not None and corrected - qbound > 0.0:
                eligibility[action_id] += 1
                lower = 0.5 / (corrected + qbound)
                upper = 0.5 / (corrected - qbound)
                exact_action = rat(row["exact_positive_witness_action"])
                observed = None if exact_action is None else float(exact_action)
                if observed is not None:
                    exact_coverage[action_id] += 1
                add(evals, action_id, row, name, candidate_action_lower=lower, candidate_action_upper=upper, exact_action=observed, sound=None if observed is None else lower <= observed <= upper, status="candidate_interval_not_verified")
            predicate_id = f"predicate_beta_positive__{name}__from_radius"
            margin = c["center_beta_margin_f64"]
            if margin is not None and radius is not None:
                eligibility[predicate_id] += 1
                predicate = "true" if margin > radius else "false" if margin < -radius else "indeterminate"
                exact = row["exact_positive_witness_status"]
                if exact == "exists" and unique_exact:
                    exact_coverage[predicate_id] += 1
                add(evals, predicate_id, row, name, f64_predicate=predicate, exact_predicate=exact, sound=None if exact != "exists" or not unique_exact else predicate != "false", status="heuristic_ternary_predicate")
    emitted = {x["formula_id"] for x in evals}
    unknown = emitted - registry_ids
    if unknown:
        raise SystemExit(f"analyzer emitted unregistered formulas: {sorted(unknown)}")
    coverage = {}
    for formula in registry:
        items = [x for x in evals if x["formula_id"] == formula["formula_id"]]
        sound_items = [x for x in items if x.get("sound") is not None]
        slack = [x.get("candidate_bound_abs", x.get("candidate_bound_l2")) - x.get("observed_error_abs", x.get("observed_error_l2")) for x in sound_items if x.get("candidate_bound_abs", x.get("candidate_bound_l2")) is not None]
        coverage[formula["formula_id"]] = {
            "eligible_rows": eligibility[formula["formula_id"]],
            "exact_target_covered_rows": exact_coverage[formula["formula_id"]],
            "undercoverage_rows": eligibility[formula["formula_id"]] - exact_coverage[formula["formula_id"]],
            "evaluated_rows": len(items),
            "sound_rows": sum(x.get("sound") is True for x in sound_items),
            "unsound_rows": sum(x.get("sound") is False for x in sound_items),
            "sharpness_min_slack": min(slack) if slack else None,
            "sharpness_max_slack": max(slack) if slack else None,
        }
    with (out / "formula_evaluations.jsonl").open("w") as f:
        for item in evals:
            f.write(json.dumps(item, sort_keys=True) + "\n")
    analysis = {"schema_version": "qp-soundness-analysis-v2", "raw_row_count": len(rows), "formula_registry_entry_count": len(registry), "emitted_formula_entry_count": len(emitted), "formula_coverage": coverage, "available_center_counts": dict(center_counts), "exact_positive_witness_status": dict(exact_status), "interpretation_boundary": "Candidate bounds are empirical diagnostics, not verified enclosures. Each comparison is same-sigma and stored-rational-target only; it does not establish algebraic HKO behavior, candidate recall beyond the declared stream, or physical-orbit identity."}
    (out / "analysis.json").write_text(json.dumps(analysis, indent=2, sort_keys=True) + "\n")
    (out / "interpretation.md").write_text("# QP soundness v2 interpretation boundary\n\nCandidate-bound rows distinguish eligible, exact-covered, and undercovered observations. Ordinary f64 arithmetic is never labelled a verified enclosure. HKO rows use only their stored binary64 rational target; active words are candidate words, not physical-orbit sets.\n")


if __name__ == "__main__":
    main()
