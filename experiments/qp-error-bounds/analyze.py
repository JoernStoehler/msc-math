#!/usr/bin/env python3
"""Offline formula inventory and consumer projections for wide Rust rows."""
from __future__ import annotations

import argparse
import json
import math
import time
from collections import Counter, defaultdict
from fractions import Fraction
from pathlib import Path


def _local(expression: str, *, target: str, center: str, required_atoms: tuple[str, ...],
           hypotheses: str, consumers: tuple[str, ...], status: str = "implemented") -> dict:
    """Registry entry for a packet-local formula, kept beside its evaluator."""
    return {
        "expression": expression, "target": target, "center": center,
        "required_atoms": list(required_atoms), "hypotheses": hypotheses,
        "arithmetic_model": "binary64 observation plus exact-rational comparisons",
        "consumers": list(consumers), "implementation_status": status,
    }


LOCAL_FORMULAS = {
    "local.beta_static_margin.v1": _local("||beta_f64-beta_exact||_inf <= 1e-9", target="unique_exact_beta", center="beta_f64", required_atoms=("beta_f64", "beta_error_linf"), hypotheses="unique exact KKT selection", consumers=("beta stability",)),
    "local.q_residual_diagnostic.v1": _local("|Q_proposal_corrected-Q_proposal_raw|", target="proposal_q_correction", center="proposal_q_corrected_f64", required_atoms=("proposal_q_raw_f64", "proposal_q_corrected_f64", "proposal_q_error_bound_f64"), hypotheses="diagnostic only; no binary64 theorem or input-rounding term", consumers=("Q correction audit",)),
    "local.q_beta_radius_raw.v1": _local("|Q_proposal_raw-Q_exact| <= inverse-KKT radius", target="q_exact", center="proposal_q_f64", required_atoms=("proposal_q_f64", "proposal_q_beta_radius_bound"), hypotheses="exact Q target exists; full KKT inverse diagnostic", consumers=("raw-Q audit",)),
    "local.action_reciprocal.v1": _local("A=1/(2 Q_corrected)", target="accepted_action", center="action_f64", required_atoms=("action_f64",), hypotheses="accepted positive corrected Q", consumers=("action report",)),
    "consumer.volume_f64_error.v1": _local("|volume_f64-volume_exact|", target="volume_exact", center="volume_f64", required_atoms=("volume_f64", "volume_exact"), hypotheses="f64 and exact incidence volumes share geometry", consumers=("volume consumer",)),
    "consumer.sys_volume_propagation.v1": _local("|sys|*volume_error/volume_f64", target="volume_exact", center="volume_f64", required_atoms=("sys_f64", "volume_f64", "volume_exact"), hypotheses="volume-only first-order propagation", consumers=("sys consumer",)),
    "consumer.sys_gt_one.v1": _local("sys_f64 > 1", target="sys_f64", center="sys_f64", required_atoms=("sys_f64",), hypotheses="reported sys ratio", consumers=("sys predicate",)),
    "consumer.derivative_category.v1": _local("derivative availability", target="derivative_f64", center="derivative_f64", required_atoms=("derivative_f64",), hypotheses="production derivative returned", consumers=("derivative consumer",)),
    "consumer.recovery_category.v1": _local("recovery validity category", target="recovery_valid", center="recovery_valid", required_atoms=("recovery_valid",), hypotheses="production recovery returned", consumers=("recovery consumer",)),
    "predictor.invalid_branch_bucket.v1": _local("5% action window crossed with beta-margin bucket", target="action_f64", center="action_f64", required_atoms=("action_f64", "beta_margin_f64"), hypotheses="unconditional population annotation", consumers=("predictor audit",)),
    "local.beta_inverse_radius.v1": _local("||beta_f64-beta_exact||_inf <= ||K^-1||_inf residual", target="unique_exact_beta", center="beta_f64", required_atoms=("kkt_matrix_f64", "kkt_residual_vector_f64", "beta_error_linf"), hypotheses="matching selected center and full-KKT inverse", consumers=("beta stability",)),
    "local.beta_eta_ternary.v1": _local("sign(beta) from inverse-radius eta", target="exact_beta_predicate", center="beta_f64", required_atoms=("kkt_matrix_f64", "kkt_residual_vector_f64"), hypotheses="local diagnostic, not reduced-Hessian theorem", consumers=("beta predicate audit",)),
    "local.q_first_order.v1": _local("first-order plus quadratic proposal Q correction", target="proposal_q_correction", center="proposal_q_corrected_f64", required_atoms=("proposal_q_raw_f64", "proposal_q_corrected_f64", "proposal_q_error_bound_f64", "proposal_residual_norm"), hypotheses="local residual diagnostic; input-rounding term omitted", consumers=("Q correction audit",)),
    "local.q_correction_quadratic.v1": _local("||proposal residual||^2/(2 spectral gap)", target="proposal_q_correction", center="proposal_q_corrected_f64", required_atoms=("proposal_q_raw_f64", "proposal_q_corrected_f64", "qp_h_f64", "proposal_residual_norm"), hypotheses="local quadratic residual diagnostic", consumers=("Q correction audit",)),
    "local.q_action_interval.v1": _local("A interval from proposal corrected-Q bound", target="proposal_action_interval", center="proposal_q_corrected_f64", required_atoms=("proposal_q_corrected_f64", "proposal_q_error_bound_f64"), hypotheses="monotone reciprocal endpoints; proposal diagnostic", consumers=("action interval consumer",)),
    "local.projected_hessian.v1": _local("V^T H V", target="projected_hessian", center="beta_f64", required_atoms=("qp_c_f64", "qp_h_f64"), hypotheses="retained C/H matrices", consumers=("spectral audit",)),
    "local.compatible_minima.v1": _local("overlap of proposal action intervals", target="proposal_action_interval", center="action_interval", required_atoms=("proposal_q_corrected_f64", "proposal_q_error_bound_f64"), hypotheses="unconditional interval projection; no route recall claim", consumers=("minimizer consumer",)),
}


def exact(text: str) -> Fraction:
    n, d = text.split("/", 1)
    return Fraction(int(n), int(d))


def exact_value(text: str | None) -> Fraction:
    if text is None:
        raise ValueError("missing exact value")
    return exact(text)


def observed_error(f64: float | None, reference: str | None) -> Fraction | None:
    if reference is None or f64 is None or not math.isfinite(f64):
        return None
    return abs(Fraction.from_float(f64) - exact(reference))


def observed_difference(left: float | None, right: float | None) -> Fraction | None:
    if left is None or right is None or not math.isfinite(left) or not math.isfinite(right):
        return None
    return abs(Fraction.from_float(left) - Fraction.from_float(right))


def row_q_corrected(row: dict) -> float | None:
    return row.get("proposal_q_corrected_f64") if row.get("proposal_q_corrected_f64") is not None else row.get("q_corrected_f64")


def row_proposal_q_raw(row: dict) -> float | None:
    return row.get("proposal_q_raw_f64") if row.get("proposal_q_raw_f64") is not None else row.get("q_raw_f64")


def row_proposal_action(row: dict) -> float | None:
    return row.get("proposal_action_f64")


def row_accepted_action(row: dict) -> float | None:
    return row.get("accepted_action_f64") if row.get("accepted_action_f64") is not None else row.get("action_f64")


def row_q_bound(row: dict) -> float | None:
    return row.get("proposal_q_error_bound_f64") if row.get("proposal_q_error_bound_f64") is not None else row.get("q_error_bound")


def fraction_text(value: Fraction | None) -> str | None:
    return None if value is None else f"{value.numerator}/{value.denominator}"


def vec_norm(values: list[float] | None) -> float | None:
    if values is None or not values or not all(math.isfinite(x) for x in values):
        return None
    return math.sqrt(sum(x * x for x in values))


def mat_inf_norm(matrix: list[list[float]] | None) -> float | None:
    if matrix is None or not matrix or not all(matrix):
        return None
    return max(sum(abs(x) for x in row) for row in matrix)


def mat_vec(matrix: list[list[float]], vector: list[float]) -> list[float]:
    return [sum(a * b for a, b in zip(row, vector)) for row in matrix]


def transpose(matrix: list[list[float]]) -> list[list[float]]:
    return [list(col) for col in zip(*matrix)] if matrix else []


def mat_mul(left: list[list[float]], right: list[list[float]]) -> list[list[float]]:
    if not left or not right:
        return []
    rt = transpose(right)
    return [[sum(a * b for a, b in zip(row, col)) for col in rt] for row in left]


def dot(left: list[float], right: list[float]) -> float:
    return sum(a * b for a, b in zip(left, right))


def gauss_jordan_inverse(matrix: list[list[float]], tol: float = 1e-13) -> list[list[float]] | None:
    n = len(matrix)
    if n == 0 or any(len(row) != n for row in matrix):
        return None
    aug = [list(map(float, row)) + [1.0 if i == j else 0.0 for j in range(n)] for i, row in enumerate(matrix)]
    for col in range(n):
        pivot = max(range(col, n), key=lambda i: abs(aug[i][col]))
        if abs(aug[pivot][col]) <= tol:
            return None
        aug[col], aug[pivot] = aug[pivot], aug[col]
        scale = aug[col][col]
        aug[col] = [x / scale for x in aug[col]]
        for i in range(n):
            if i == col:
                continue
            scale = aug[i][col]
            if scale:
                aug[i] = [a - scale * b for a, b in zip(aug[i], aug[col])]
    return [row[n:] for row in aug]


def symmetric_eigenvalues(matrix: list[list[float]] | None, tol: float = 1e-14) -> list[float] | None:
    """Small Jacobi eigensolver; avoids making the offline analyzer depend on numpy."""
    if matrix is None or not matrix or any(len(row) != len(matrix) for row in matrix):
        return None
    a = [list(map(float, row)) for row in matrix]
    n = len(a)
    if any(not math.isfinite(x) for row in a for x in row):
        return None
    for _ in range(max(20, 12 * n * n)):
        p, q = 0, 1 if n > 1 else 0
        off = 0.0
        for i in range(n):
            for j in range(i + 1, n):
                if abs(a[i][j]) > off:
                    off, p, q = abs(a[i][j]), i, j
        if off <= tol:
            break
        theta = 0.5 * math.atan2(2.0 * a[p][q], a[q][q] - a[p][p])
        c, s = math.cos(theta), math.sin(theta)
        for i in range(n):
            if i in (p, q):
                continue
            aip, aiq = a[i][p], a[i][q]
            a[i][p] = a[p][i] = c * aip - s * aiq
            a[i][q] = a[q][i] = s * aip + c * aiq
        app, aqq, apq = a[p][p], a[q][q], a[p][q]
        a[p][p] = c * c * app - 2 * s * c * apq + s * s * aqq
        a[q][q] = s * s * app + 2 * s * c * apq + c * c * aqq
        a[p][q] = a[q][p] = 0.0
    return [a[i][i] for i in range(n)]


def nullspace_orthonormal(matrix: list[list[float]] | None, tol: float = 1e-11) -> list[list[float]] | None:
    """Return orthonormal columns spanning ker(matrix), via RREF + Gram--Schmidt."""
    if matrix is None or not matrix:
        return None
    a = [list(map(float, row)) for row in matrix]
    rows, cols = len(a), len(a[0])
    if any(len(row) != cols for row in a):
        return None
    pivots: list[int] = []
    r = 0
    for c in range(cols):
        pivot = max(range(r, rows), key=lambda i: abs(a[i][c]))
        if abs(a[pivot][c]) <= tol:
            continue
        a[r], a[pivot] = a[pivot], a[r]
        scale = a[r][c]
        a[r] = [x / scale for x in a[r]]
        for i in range(rows):
            if i != r and abs(a[i][c]) > tol:
                scale = a[i][c]
                a[i] = [x - scale * y for x, y in zip(a[i], a[r])]
        pivots.append(c)
        r += 1
        if r == rows:
            break
    free = [c for c in range(cols) if c not in pivots]
    basis: list[list[float]] = []
    for f in free:
        x = [0.0] * cols
        x[f] = 1.0
        for i, p in enumerate(pivots):
            x[p] = -a[i][f]
        for prior in basis:
            projection = dot(x, prior)
            x = [u - projection * v for u, v in zip(x, prior)]
        norm = vec_norm(x) or 0.0
        if norm > tol:
            basis.append([u / norm for u in x])
    return [list(col) for col in zip(*basis)] if basis else []


def unavailable_reason(spec: dict) -> str:
    formula_id = spec["id"]
    if formula_id.startswith("geometry.") or formula_id.startswith("preprocessing."):
        return "geometry/incidence oracle atom is not retained for this case"
    if formula_id.startswith("derivative."):
        return "the required volume or systolic gradient atom is not retained"
    if formula_id.startswith("recovery."):
        return "the production recovery route did not return a witness for this row"
    if formula_id.startswith("predictor."):
        return "sys branch gradient/window-selection atoms are not retained"
    if formula_id.startswith("bound."):
        return "formal perturbation/eigenpair atom is not retained"
    if formula_id.startswith("volume."):
        return "incidence-backed volume atom is unavailable for this case"
    if formula_id.startswith("fallback."):
        return "exact fallback policy/timing atom is not retained"
    if formula_id.startswith("aggregation."):
        return "aggregate has no action interval or fallback-policy atom"
    if formula_id.startswith("consumer."):
        return "consumer-specific atom is not retained"
    return "source formula is retained but its required atom is not exposed"


def category(label: str, truth: str) -> str:
    if truth == "unavailable":
        return "exact_unavailable"
    if label == "indeterminate" and truth in {"true", "false"}:
        return f"indeterminate|{truth}"
    if label == truth:
        return f"{label}|{truth}_sound"
    return f"{label}|{truth}_unsound"


def row_predicate_category(row: dict, center: str) -> str | None:
    if center == "beta":
        return category(row["f64_beta_predicate"], row["exact_beta_predicate"])
    if center == "q":
        return category(row["f64_q_predicate"], row["exact_q_predicate"])
    return None


def make_eval(row_id: str, run_id: str, case_id: str, target_id: str, formula_id: str,
              center: str | None = None, error: Fraction | None = None,
              bound: float | None = None, value: object = None,
              predicate_category: str | None = None, note: str | None = None,
              comparison_id: str | None = None) -> dict:
    bound_exact = Fraction.from_float(bound) if bound is not None and math.isfinite(bound) else None
    applicable = error is not None and bound_exact is not None
    return {
        "row_id": row_id, "run_id": run_id, "case_id": case_id,
        "target_id": target_id, "formula_id": formula_id,
        "center_id": center, "value": value,
        "E": float(error) if error is not None else None,
        "E_exact": fraction_text(error), "B": bound,
        "B_exact": fraction_text(bound_exact), "applicable": applicable,
        "covered": error <= bound_exact if applicable else None,
        "undercoverage": float(error - bound_exact) if applicable and error > bound_exact else 0.0 if applicable else None,
        "category": predicate_category, "note": note, "comparison_id": comparison_id,
    }


def q_interval(row: dict, q: float | None = None, error_bound: float | None = None) -> dict | None:
    """Monotone action interval for A=1/(2Q), retaining endpoint semantics."""
    q = row_q_corrected(row) if q is None else q
    error_bound = row_q_bound(row) if error_bound is None else error_bound
    if q is None or error_bound is None or not math.isfinite(q) or not math.isfinite(error_bound) or q <= 0:
        return None
    lower_q, upper_q = q - error_bound, q + error_bound
    return {
        "q_center": q, "q_error_bound": error_bound,
        "q_lower": lower_q, "q_upper": upper_q,
        "action_lower": 0.5 / upper_q if upper_q > 0 else 0.0,
        "action_upper": 0.5 / lower_q if lower_q > 0 else None,
        "endpoint_policy": "A_lower=1/(2*q_upper); A_upper=1/(2*q_lower), infinity when q_lower<=0",
    }


def inverse_beta_radius(row: dict, prefer_proposal: bool = False) -> tuple[float, float] | None:
    """Return (beta radius, full inverse radius) from the retained KKT atoms."""
    matrix, rhs = row.get("kkt_matrix_f64"), row.get("kkt_rhs_f64")
    beta = (row.get("proposal_beta_f64") if prefer_proposal else row.get("beta_f64")) or row.get("proposal_beta_f64")
    residual = (proposal_residual_vector(row) if prefer_proposal else row.get("kkt_residual_vector_f64")) or proposal_residual_vector(row)
    if matrix is None or rhs is None or beta is None or residual is None:
        return None
    inverse = gauss_jordan_inverse(matrix)
    if inverse is None:
        return None
    rho = max((abs(x) for x in residual), default=0.0)
    row_radius = [sum(abs(x) for x in row_) * rho for row_ in inverse]
    beta_radius = max(row_radius[: len(beta)], default=0.0)
    return beta_radius, max(row_radius, default=0.0)


def proposal_residual_vector(row: dict) -> list[float] | None:
    """Reconstruct the retained proposal residual when only its norm is serialized."""
    beta = row.get("proposal_beta_f64")
    mu = row.get("proposal_mu_f64")
    xi = row.get("proposal_xi_f64")
    matrix, rhs = row.get("kkt_matrix_f64"), row.get("kkt_rhs_f64")
    if beta is None or mu is None or xi is None or matrix is None or rhs is None:
        return None
    # The augmented KKT unknown is (beta[m], mu[4], xi), not a second
    # four-vector: xi is the single affine multiplier coordinate.
    x = beta + mu + [xi]
    if len(x) != len(matrix):
        return None
    return [a - b for a, b in zip(mat_vec(matrix, x), rhs)]


def source_sigma_min(matrix: list[list[float]] | None) -> float | None:
    if matrix is None or not matrix:
        return None
    gram = mat_mul(matrix, transpose(matrix))
    values = symmetric_eigenvalues(gram)
    if not values:
        return None
    positive = [x for x in values if x > 1e-14]
    return math.sqrt(min(positive)) if positive else None


def center_for_formula(formula_id: str) -> str | None:
    if "beta_exact" in formula_id or formula_id.endswith("exact_predicate"):
        return "beta_exact"
    if "beta" in formula_id:
        return "beta_f64"
    if "action" in formula_id:
        return "action_f64"
    if "q_raw" in formula_id:
        return "q_raw_f64"
    if "q_correction" in formula_id:
        return "q_correction_f64"
    if "q_" in formula_id or formula_id.startswith("kkt.q"):
        return "q_corrected_f64"
    if "systolic_ratio" in formula_id or formula_id.endswith("sys_capacity_ratio"):
        return "sys_f64"
    if "volume" in formula_id:
        return "volume_f64"
    return None


def eval_row_formula(row: dict, formula_id: str, row_id: str) -> dict | None:
    run_id, case_id, target = row["run_id"], row["case_id"], row["target_id"]
    q_center = row_q_corrected(row)
    q_center_id = "proposal_q_corrected_f64" if row.get("proposal_q_corrected_f64") is not None else "q_corrected_f64"
    q_error = observed_error(q_center, row.get("q_exact"))
    correction_error = observed_difference(q_center, row.get("q_raw_f64"))
    raw_error = observed_error(row.get("q_raw_f64"), row.get("q_exact"))
    beta_error = row.get("beta_error_linf") if row.get("beta_error_linf") is not None else row.get("proposal_beta_error_linf")
    beta_center = row.get("beta_f64") if row.get("beta_f64") is not None else row.get("proposal_beta_f64")
    beta_center_id = "beta_f64" if row.get("beta_f64") is not None else "proposal_beta_f64"
    beta_margin = row.get("beta_margin_f64") if row.get("beta_margin_f64") is not None else row.get("proposal_beta_margin_f64")
    if formula_id in {"local.beta_static_margin.v1", "bound.beta_margin_heuristic"} and beta_error is not None:
        return make_eval(row_id, run_id, case_id, target, formula_id, beta_center_id, Fraction.from_float(beta_error), 1e-9, beta_margin, row_predicate_category(row, "beta"), "heuristic threshold; not a theorem")
    if formula_id == "local.beta_inverse_radius.v1":
        radius = inverse_beta_radius(row, prefer_proposal=beta_center_id == "proposal_beta_f64")
        if radius is not None and beta_error is not None:
            return make_eval(row_id, run_id, case_id, target, formula_id, beta_center_id, Fraction.from_float(beta_error), radius[0], beta_center, row_predicate_category(row, "beta"), "full augmented-KKT inverse radius; target is the same exact solver")
    if formula_id == "local.beta_eta_ternary.v1":
        radius = inverse_beta_radius(row, prefer_proposal=beta_center_id == "proposal_beta_f64")
        beta = beta_center
        if radius is not None and beta:
            eta = radius[0]
            lower, upper = min(beta) - eta, min(beta) + eta
            ternary = "true" if lower > 0 else "false" if upper < 0 else "indeterminate"
            return make_eval(row_id, run_id, case_id, target, formula_id, beta_center_id, value={"eta": eta, "predicate": ternary}, predicate_category=category(ternary, row.get("exact_beta_predicate", "unavailable")), note="local verified inverse-radius ternary; not the reduced-Hessian eta theorem")
    if formula_id == "bound.beta_verified_inverse":
        # The formal reduced-Hessian bound needs V,w,gamma atoms, which this
        # producer does not expose. Keep it unavailable rather than silently
        # substituting the full-KKT inverse radius (reported as local above).
        return None
    if formula_id == "bound.beta_perturbation_eta":
        return None
    if formula_id == "local.q_residual_diagnostic.v1" and correction_error is not None and q_center is not None:
        return make_eval(row_id, run_id, case_id, target, formula_id, q_center_id, correction_error, row_q_bound(row), q_center, row_predicate_category(row, "q"), "residual-correction diagnostic against raw-Q center; not a binary64 theorem", "proposal_q_correction_f64")
    if formula_id == "kkt.q_error_bound":
        # The source bound controls a solver correction around its accepted
        # center; this packet does not expose the theorem's input-rounding
        # hypotheses.  Keep the formula inventory entry, but do not emit a
        # proposal-center evaluation under the source ID.
        return None
    if formula_id == "local.q_beta_radius_raw.v1" and raw_error is not None:
        # This diagnostic must use the same SVD-proposal residual as the
        # unconditional q center and bound, never the accepted solver residual.
        radius_bound = row.get("proposal_q_beta_radius_bound") if row.get("proposal_q_beta_radius_bound") is not None else row.get("q_beta_radius_bound")
        return make_eval(row_id, run_id, case_id, target, formula_id, "q_raw_f64", raw_error, radius_bound, row.get("q_raw_f64"), row_predicate_category(row, "q"), "full-KKT inverse-radius diagnostic; not projected theorem")
    if formula_id == "local.q_first_order.v1" and correction_error is not None and q_center is not None:
        sigma_min = source_sigma_min(row.get("qp_c_f64"))
        hnorm = max((abs(x) for x in symmetric_eigenvalues(row.get("qp_h_f64")) or []), default=None)
        beta_norm, residual_norm = vec_norm(row.get("proposal_beta_f64")), vec_norm(proposal_residual_vector(row))
        radius = inverse_beta_radius(row, prefer_proposal=True)
        if sigma_min and hnorm is not None and beta_norm is not None and residual_norm is not None and radius:
            bound = hnorm * beta_norm / sigma_min * residual_norm + 0.5 * hnorm * radius[0] ** 2
            return make_eval(row_id, run_id, case_id, target, formula_id, q_center_id, correction_error, bound, q_center, row_predicate_category(row, "q"), "local residual-correction diagnostic; input-rounding term and theorem correspondence remain open", "proposal_q_correction_f64")
    if formula_id == "bound.q_first_order":
        return None
    if formula_id == "local.q_correction_quadratic.v1" and correction_error is not None and q_center is not None:
        residual_norm = vec_norm(proposal_residual_vector(row))
        eig = [abs(x) for x in symmetric_eigenvalues(row.get("qp_h_f64")) or [] if abs(x) > 1e-13]
        if residual_norm is not None and eig:
            bound = residual_norm * residual_norm / (2.0 * min(eig))
            return make_eval(row_id, run_id, case_id, target, formula_id, q_center_id, correction_error, bound, q_center, row_predicate_category(row, "q"), "local quadratic residual-correction diagnostic; not the source theorem", "proposal_q_correction_f64")
    if formula_id == "bound.q_correction_second_order":
        return None
    if formula_id == "local.q_action_interval.v1":
        interval = q_interval(row)
        if interval is not None:
            return make_eval(row_id, run_id, case_id, target, formula_id, q_center_id, value=interval, note="explicit monotone endpoints around unconditional proposal corrected-Q center")
    if formula_id in {"kkt.action_interval_from_q_bound", "safety.action_interval_from_q_error"}:
        return None
    if formula_id == "local.action_reciprocal.v1" and row.get("action_f64") is not None:
        return make_eval(row_id, run_id, case_id, target, formula_id, "action_f64", value=row["action_f64"], note="A=1/(2Q) from corrected Q")
    if formula_id in {"local.projected_hessian.v1", "kkt.projected_hessian"}:
        basis = nullspace_orthonormal(row.get("qp_c_f64"))
        h = row.get("qp_h_f64")
        if basis and h:
            projected = mat_mul(mat_mul(transpose(basis), h), basis)
            return make_eval(row_id, run_id, case_id, target, formula_id, "beta_f64", value={"basis": basis, "hessian": projected, "eigenvalues": symmetric_eigenvalues(projected)}, note="computed from retained C and H; no perturbed/reference eigensystem")
    if formula_id == "kkt.eigen_rank_tiers":
        eigenvalues = row.get("proposal_eigenvalues_f64")
        if eigenvalues:
            scale = max((abs(x) for x in eigenvalues), default=0.0)
            strict = scale * 1e-3
            return make_eval(row_id, run_id, case_id, target, formula_id, value={"eigenvalues": eigenvalues, "permissive_count": sum(abs(x) > 1e-14 for x in eigenvalues), "strict_threshold": strict, "strict_count": sum(abs(x) > strict for x in eigenvalues)})
    if formula_id == "kkt.pseudoinverse_solution" and row.get("proposal_beta_f64") is not None:
        return make_eval(row_id, run_id, case_id, target, formula_id, "beta_f64", value={"beta": row.get("proposal_beta_f64"), "mu": row.get("proposal_mu_f64"), "xi": row.get("proposal_xi_f64")}, note="serialized least-squares/SVD proposal, not an accepted candidate")
    if formula_id == "kkt.constraint_svd_solution" and row.get("proposal_singular_values_f64") is not None:
        return make_eval(row_id, run_id, case_id, target, formula_id, "beta_f64", value={"rank": row.get("proposal_rank"), "nullity": row.get("proposal_nullity"), "singular_values": row.get("proposal_singular_values_f64"), "beta": row.get("proposal_beta_f64")}, note="constraint SVD proposal atoms")
    if formula_id == "kkt.exact_kkt_oracle" and row.get("action_exact") is not None:
        return make_eval(row_id, run_id, case_id, target, formula_id, "exact_action", value=row["action_exact"])
    if formula_id == "qp.kkt_augmented_system" and row.get("kkt_matrix_f64") is not None and row.get("kkt_rhs_f64") is not None:
        return make_eval(row_id, run_id, case_id, target, formula_id, value={"matrix": row["kkt_matrix_f64"], "rhs": row["kkt_rhs_f64"]})
    if formula_id == "consumer.volume_f64_error.v1":
        error = observed_error(row.get("volume_f64"), row.get("volume_exact"))
        if error is not None:
            return make_eval(row_id, run_id, case_id, target, formula_id, "volume", error=error, value=row.get("volume_f64"))
    if formula_id == "consumer.sys_volume_propagation.v1" and row.get("sys_f64") is not None and row.get("volume_f64") not in {None, 0}:
        volume_error = observed_error(row.get("volume_f64"), row.get("volume_exact"))
        if volume_error is not None:
            value = abs(row["sys_f64"]) * float(volume_error) / row["volume_f64"]
            return make_eval(row_id, run_id, case_id, target, formula_id, "sys", value=value, note="volume-only first-order propagation")
    if formula_id == "consumer.sys_gt_one.v1":
        return make_eval(row_id, run_id, case_id, target, formula_id, value=(row["sys_f64"] > 1) if row.get("sys_f64") is not None else None)
    if formula_id == "consumer.derivative_category.v1":
        return make_eval(row_id, run_id, case_id, target, formula_id, value="available" if row.get("derivative_linf") is not None else "unavailable")
    if formula_id == "consumer.recovery_category.v1":
        value = "valid" if row.get("recovery_valid") is True else "invalid" if row.get("recovery_valid") is False else "unavailable"
        return make_eval(row_id, run_id, case_id, target, formula_id, value=value)
    if formula_id == "consumer.orbit_recovery_acceptance" and row.get("recovery_valid") is not None:
        return make_eval(row_id, run_id, case_id, target, formula_id, value=bool(row["recovery_valid"]), note="accepted only when closure and facet-violation checks pass")
    if formula_id == "consumer.capacity_cache_fields":
        fields = {key: row.get(key) for key in ("capacity_f64", "volume_f64", "volume_exact", "sys_f64")}
        if any(value is not None for value in fields.values()):
            return make_eval(row_id, run_id, case_id, target, formula_id, center_for_formula(formula_id), value=fields)
    if formula_id == "consumer.visualization_export_gate" and row.get("recovery_closure_error") is not None and row.get("recovery_max_violation") is not None:
        return make_eval(row_id, run_id, case_id, target, formula_id, value=row["recovery_closure_error"] <= 1e-6 and row["recovery_max_violation"] <= 1e-4)
    if formula_id.startswith("geometry.") and not row.get("geometry_status", "").startswith("exact_incidence_available"):
        return None
    if formula_id in {"geometry.edges_from_incidence", "geometry.two_face_incidence"}:
        incidence = row.get("geometry_vertex_facet_incidence")
        if incidence:
            if formula_id == "geometry.edges_from_incidence":
                facet_sets = [set(i for i, present in enumerate(vertex) if present) for vertex in incidence]
                edges = [[u, v] for u in range(len(facet_sets)) for v in range(u + 1, len(facet_sets)) if len(facet_sets[u] & facet_sets[v]) >= 3]
                return make_eval(row_id, run_id, case_id, target, formula_id, value=edges, note="computed from retained vertex-facet incidence")
            faces = {}
            facets = range(len(incidence[0]))
            for i in facets:
                for j in range(i + 1, len(incidence[0])):
                    vertices = [v for v, row_ in enumerate(incidence) if row_[i] and row_[j]]
                    if len(vertices) >= 3:
                        faces[f"{i},{j}"] = vertices
            return make_eval(row_id, run_id, case_id, target, formula_id, value=faces, note="computed from retained vertex-facet incidence")
    if formula_id == "volume.facet_volume_centroid":
        return None
    if formula_id in {"derivative.volume_gradient", "derivative.systolic_ratio_gradient", "derivative.clarke_directional"}:
        return None
    field_map = {
        "qp.assembly_C": "qp_c_f64", "qp.assembly_d": "qp_d_f64", "qp.assembly_H": "qp_h_f64",
        "qp.kkt_augmented_system": "kkt_matrix_f64", "kkt.residual_norm": "kkt_residual_norm",
        "kkt.q_raw": "q_raw_f64", "kkt.q_correction": "q_correction_f64", "kkt.q_corrected": "q_corrected_f64",
        "kkt.beta_margin": "beta_margin_f64", "kkt.beta_epsilon_predicate": "f64_beta_predicate",
        "kkt.beta_exact_predicate": "exact_beta_predicate", "kkt.action_from_q": "action_f64",
        "kkt.q_positive_guard": "f64_q_predicate",
        "predictor.sysext_invalid_branch_raw_q": "q_raw_f64",
        "predictor.sysext_invalid_branch_beta_margin": "beta_margin_f64",
        "geometry.facet_intersection": "geometry_facet_intersection", "geometry.omega0": "omega_matrix_f64",
        "geometry.transition_sign": "geometry_transition_matrix", "volume.four_volume_origin_star": "volume_exact",
        "volume.systolic_ratio": "sys_f64", "derivative.capacity_gradient": "derivative_f64",
        "recovery.beta_to_dwell_times": "recovery_dwell_times", "recovery.max_violation": "recovery_max_violation",
        "recovery.closure_error": "recovery_closure_error", "recovery.shoelace_action": "recovery_action_f64",
        "consumer.sys_capacity_ratio": "sys_f64",
    }
    if formula_id == "kkt.residual_norm":
        residual = row.get("kkt_residual_vector_f64") or proposal_residual_vector(row)
        norm = vec_norm(residual)
        if norm is not None:
            return make_eval(row_id, run_id, case_id, target, formula_id, "kkt_residual_f64", value=norm)
    field = field_map.get(formula_id)
    if field and row.get(field) is not None:
        center = center_for_formula(formula_id)
        return make_eval(row_id, run_id, case_id, target, formula_id, center, value=row.get(field), predicate_category=row_predicate_category(row, center) if center else None)
    return None


def aggregate_formula(aggregate: dict, rows: list[dict], formula_id: str) -> dict | None:
    actions = sorted((row_proposal_action(r), r["sigma"]) for r in rows if row_proposal_action(r) is not None)
    if not actions:
        return None
    run_id, case_id = aggregate["run_id"], aggregate["case_id"]
    target = rows[0]["target_id"]
    row_id = f"{case_id}:aggregate"
    universe = rows[0].get("universe_contract", "")
    if formula_id == "enumeration.hk_transition_pruned_cycles" and "transition" in universe:
        return make_eval(row_id, run_id, case_id, target, formula_id, value=len(rows))
    if formula_id == "enumeration.hk_unpruned_cycles":
        return None
    # These source formulas require route-specific or interval atoms that this
    # producer does not retain.  A transition-cycle count or symmetric
    # action-error interval would be a different mathematical target.
    if formula_id in {
        "enumeration.product_billiard_sigma",
        "performance.route_counts_and_fallback_resolutions",
        "candidate.indeterminate_interval_overlap",
    }:
        return None
    if formula_id == "candidate.f64_action_sort_and_partition":
        # The source route also sorts indeterminate candidates by retained
        # lower action bounds; this packet has no such interval atoms.
        return None
    if formula_id in {"local.compatible_minima.v1", "aggregation.minimizer_set_from_action_intervals"}:
        intervals = []
        for r in rows:
            interval = q_interval(r)
            if interval is not None:
                intervals.append((r["sigma"], interval["action_lower"], interval["action_upper"]))
        if not intervals:
            return None
        finite_upper = [x[2] for x in intervals if x[2] is not None]
        lower = min(x[1] for x in intervals)
        upper = min(finite_upper, default=None)
        compatible = [sigma for sigma, lo, hi in intervals if upper is None or lo <= upper]
        return make_eval(row_id, run_id, case_id, target, formula_id, "action_interval", value={"minimum_lower": lower, "minimum_upper": upper, "compatible_sigma": compatible}, note="compatible action intervals; no per-sigma recall claim")
    if formula_id == "aggregation.low_action_window":
        cutoff = aggregate.get("proposal_low_action_window_cutoff", aggregate.get("f64_low_action_window_cutoff"))
        if cutoff is None:
            return None
        selected = [r["sigma"] for r in rows if row_proposal_action(r) is not None and row_proposal_action(r) <= cutoff]
        return make_eval(row_id, run_id, case_id, target, formula_id, "action_f64", value={"cutoff": cutoff, "sigma": selected}, note="producer 5% f64 low-action window")
    if formula_id == "consumer.best_sigma_action_ranking":
        return make_eval(row_id, run_id, case_id, target, formula_id, value={"sigma_star": actions[0][1], "capacity": actions[0][0]})
    if formula_id == "predictor.sysext_invalid_branch_bucket":
        # The source policy also retains at most three candidate rows per
        # bucket; aggregate counts alone do not expose that selected subset.
        return None
    if formula_id == "predictor.invalid_branch_bucket.v1":
        cutoff = aggregate.get("proposal_low_action_window_cutoff", aggregate.get("f64_low_action_window_cutoff"))
        buckets = Counter("outside_window" if row_proposal_action(r) is None or cutoff is None or row_proposal_action(r) > cutoff else ("beta_invalid" if r.get("beta_margin_f64") is not None and r["beta_margin_f64"] <= 0 else "high_q_candidate") for r in rows)
        return make_eval(row_id, run_id, case_id, target, formula_id, value=dict(buckets))
    if formula_id in {"predictor.candidate_window_branch_gap", "predictor.candidate_window_second_gap", "predictor.candidate_window_visibility"}:
        return None
    return None


def _is_retained(row: dict) -> bool:
    events = row.get("lifecycle_events") or []
    state = str(row.get("route_state", row.get("lifecycle_stage", ""))).lower()
    return "retained" in events or state == "retained" or row.get("production_retained") is True or row.get("route_retained") is True


def _stratum_rows(rows: list[dict], aggregates: list[dict]) -> dict[str, list[dict]]:
    by_case: dict[str, list[dict]] = defaultdict(list)
    for row in rows:
        by_case[row["case_id"]].append(row)
    aggregate_map = {a["case_id"]: a for a in aggregates}
    result: dict[str, list[dict]] = {"all_rows": list(rows), "all_computable_rows": list(rows), "production_visited": [], "production_retained": [], "unconditional_maximum_q": [], "unconditional_minimum_action": [], "exact_minimizers": [], "low_action_window": []}
    for row in rows:
        case_rows = by_case[row["case_id"]]
        finite_q = [row_proposal_q_raw(r) for r in case_rows if row_proposal_q_raw(r) is not None]
        if row.get("unconditional_maximum_q_member") is True or (finite_q and row_proposal_q_raw(row) == max(finite_q)):
            result["unconditional_maximum_q"].append(row)
        if str(row.get("route_attempt_status", "")).startswith("production_route_"):
            result["production_visited"].append(row)
        if _is_retained(row):
            result["production_retained"].append(row)
        aggregate = aggregate_map.get(row["case_id"], {})
        exact_min = aggregate.get("exact_min_action")
        if row.get("unconditional_minimum_action_member") is True:
            result["unconditional_minimum_action"].append(row)
        if exact_min is not None and row.get("action_exact") == exact_min:
            result["exact_minimizers"].append(row)
        cutoff = aggregate.get("proposal_low_action_window_cutoff", aggregate.get("f64_low_action_window_cutoff"))
        if row.get("unconditional_low_action_window_member") is True or (cutoff is not None and row_proposal_action(row) is not None and row_proposal_action(row) <= cutoff):
            result["low_action_window"].append(row)
    return result


def _ratio_summary(values: list[float]) -> dict:
    return {
        "count": len(values),
        "max": max(values, default=None),
        "median": sorted(values)[len(values) // 2] if values else None,
    }


def _exact_fraction(value: object) -> Fraction | None:
    if not isinstance(value, str):
        return None
    try:
        return exact_value(value)
    except (TypeError, ValueError, ZeroDivisionError):
        return None


def _selection_margin(stratum: str | None, row: dict, case_rows: list[dict], aggregate: dict) -> tuple[float | None, str]:
    """Return the margin of the *consumer decision*, never a value magnitude.

    The unconditional and all-computable populations do not define a scalar
    decision, so their q/action margins are intentionally unavailable.  The
    four explicit consumers below use the proposal population (or the exact
    action population for exact minimizers) and preserve ties as zero.
    """
    if stratum == "unconditional_maximum_q":
        values = [row_proposal_q_raw(candidate) for candidate in case_rows]
        values = [value for value in values if value is not None and math.isfinite(value)]
        value = row_proposal_q_raw(row)
        if not values or value is None or not math.isfinite(value):
            return None, "proposal max-Q gap to next distinct proposal Q (unavailable)"
        maximum = max(values)
        if value != maximum:
            return None, "proposal max-Q gap to next distinct proposal Q (unavailable for non-max row)"
        distinct = sorted(set(values), reverse=True)
        if sum(candidate == maximum for candidate in values) > 1:
            return 0.0, "proposal max-Q gap to next distinct proposal Q"
        if len(distinct) <= 1:
            return None, "proposal max-Q gap to next distinct proposal Q (unavailable)"
        return maximum - distinct[1], "proposal max-Q gap to next distinct proposal Q"
    if stratum == "unconditional_minimum_action":
        values = [row_proposal_action(candidate) for candidate in case_rows]
        values = [value for value in values if value is not None and math.isfinite(value)]
        value = row_proposal_action(row)
        if not values or value is None or not math.isfinite(value):
            return None, "proposal minimum-action gap to next distinct action (unavailable)"
        minimum = min(values)
        if value != minimum:
            return None, "proposal minimum-action gap to next distinct action (unavailable for non-min row)"
        distinct = sorted(set(values))
        if sum(candidate == minimum for candidate in values) > 1:
            return 0.0, "proposal minimum-action gap to next distinct action"
        if len(distinct) <= 1:
            return None, "proposal minimum-action gap to next distinct action (unavailable)"
        return distinct[1] - minimum, "proposal minimum-action gap to next distinct action"
    if stratum == "low_action_window":
        cutoff = aggregate.get("proposal_low_action_window_cutoff", aggregate.get("f64_low_action_window_cutoff"))
        value = row_proposal_action(row)
        if cutoff is None or value is None or not math.isfinite(value):
            return None, "distance to declared 1.05 proposal-window cutoff (unavailable)"
        return max(0.0, float(cutoff) - value), "distance to declared 1.05 proposal-window cutoff"
    if stratum == "exact_minimizers":
        minimum = _exact_fraction(aggregate.get("exact_min_action"))
        runner_up = _exact_fraction(aggregate.get("exact_runner_up_action"))
        value = _exact_fraction(row.get("action_exact"))
        if minimum is None or value is None or value != minimum or runner_up is None:
            return None, "exact minimum/runner-up action gap (unavailable)"
        exact_values = [_exact_fraction(candidate.get("action_exact")) for candidate in case_rows]
        ties = sum(candidate == minimum for candidate in exact_values if candidate is not None)
        if ties > 1:
            return 0.0, "exact minimum/runner-up action gap"
        return float(runner_up - minimum), "exact minimum/runner-up action gap"
    return None, "no scalar q/action consumer decision for this stratum"


def _beta_zero_decision_margin(row: dict) -> float | None:
    """Distance of a unique exact beta reference from the zero decision."""
    if row.get("exact_solver_status") != "feasible":
        return None
    beta = row.get("beta_exact")
    if not isinstance(beta, list) or not beta or any(_exact_fraction(value) is None for value in beta):
        return None
    values = [abs(float(_exact_fraction(value))) for value in beta]
    return min(values) if values else None


def _consumer_margins(
    evaluations: list[dict],
    rows_by_id: dict[str, dict],
    stratum: str | None = None,
    aggregates_by_case: dict[str, dict] | None = None,
) -> dict:
    out = {
        "margin_definition": "q/action ratios use the actual stratum selection margin; beta uses beta_zero_decision_margin only for a unique exact beta reference",
        "E_le_B": 0, "undercoverage": 0, "undercoverage_outliers": [],
        "zero_E": 0, "zero_B": 0, "zero_E_and_B": 0,
        "margin_available_count": 0, "margin_unavailable_count": 0, "margin_zero_count": 0,
        "selection_margin": {"definition": "actual stratum consumer margin", "available_count": 0, "unavailable_count": 0, "zero_count": 0},
        "E_over_M": [], "B_over_M": [], "beta_zero_decision_margin": [],
    }
    aggregates_by_case = aggregates_by_case or {}
    rows_by_case: dict[str, list[dict]] = defaultdict(list)
    for candidate in rows_by_id.values():
        rows_by_case[candidate["case_id"]].append(candidate)
    for evaluation in evaluations:
        if not evaluation.get("applicable"):
            continue
        e, b = evaluation.get("E"), evaluation.get("B")
        if e is None or b is None:
            continue
        if e <= b:
            out["E_le_B"] += 1
        else:
            out["undercoverage"] += 1
            if len(out["undercoverage_outliers"]) < 20:
                out["undercoverage_outliers"].append({"row_id": evaluation["row_id"], "formula_id": evaluation["formula_id"], "E": e, "B": b, "gap": e - b})
        if e == 0:
            out["zero_E"] += 1
        if b == 0:
            out["zero_B"] += 1
        if e == 0 and b == 0:
            out["zero_E_and_B"] += 1
        row = rows_by_id.get(evaluation["row_id"])
        if row is None:
            continue
        center = str(evaluation.get("center_id", ""))
        if "beta" in center:
            margin = _beta_zero_decision_margin(row)
            if margin is None:
                out["margin_unavailable_count"] += 1
            else:
                out["margin_available_count"] += 1
                if margin == 0:
                    out["margin_zero_count"] += 1
                out["beta_zero_decision_margin"].append({"row_id": evaluation["row_id"], "margin": margin, "E_over_M": e / margin if margin > 0 else None, "B_over_M": b / margin if margin > 0 else None})
            continue
        if "q" not in center and "action" not in center:
            continue
        case_rows = rows_by_case.get(row["case_id"], [])
        margin, definition = _selection_margin(stratum, row, case_rows, aggregates_by_case.get(row["case_id"], {}))
        if margin is None:
            out["margin_unavailable_count"] += 1
            out["selection_margin"]["unavailable_count"] += 1
            continue
        out["margin_available_count"] += 1
        out["selection_margin"]["available_count"] += 1
        if margin == 0:
            out["margin_zero_count"] += 1
            out["selection_margin"]["zero_count"] += 1
            # Ties are decision margins, not denominators.
            continue
        out["E_over_M"].append(e / margin)
        out["B_over_M"].append(b / margin)
    out["E_over_M"] = _ratio_summary(out["E_over_M"])
    out["B_over_M"] = _ratio_summary(out["B_over_M"])
    out["selection_margin"].update({
        "definition": {
            "unconditional_maximum_q": "per-case proposal max-Q gap to the next distinct proposal Q",
            "unconditional_minimum_action": "per-case proposal minimum-action gap to the next distinct action",
            "low_action_window": "distance to the declared 1.05 proposal-window cutoff",
            "exact_minimizers": "exact minimum/runner-up action gap when defined",
        }.get(stratum, "unavailable for this stratum"),
    })
    beta_entries = out["beta_zero_decision_margin"]
    beta_margins = [entry["margin"] for entry in beta_entries]
    out["beta_zero_decision_margin"] = {
        "definition": "min_i |exact beta_i| for a unique exact feasible beta reference; zero is an explicit margin",
        "count": len(beta_margins), "available_count": len(beta_margins),
        "zero_count": sum(margin == 0 for margin in beta_margins),
        "unavailable_count": sum(1 for evaluation in evaluations if "beta" in str(evaluation.get("center_id", ""))) - len(beta_margins),
        "max": max(beta_margins, default=None), "median": sorted(beta_margins)[len(beta_margins) // 2] if beta_margins else None,
        "E_over_M": _ratio_summary([entry["E_over_M"] for entry in beta_entries if entry["E_over_M"] is not None]),
        "B_over_M": _ratio_summary([entry["B_over_M"] for entry in beta_entries if entry["B_over_M"] is not None]),
    }
    return out


def evaluate(directory: Path) -> dict:
    analysis_started = time.perf_counter()
    manifest = json.loads((directory / "manifest.json").read_text())
    inventory = json.loads((directory / "formula_inventory.json").read_text())
    rows = [json.loads(line) for line in (directory / "raw_rows.jsonl").read_text().splitlines() if line]
    aggregates = [json.loads(line) for line in (directory / "aggregates.jsonl").read_text().splitlines() if line]
    formulas = inventory["formulas"] + [{"id": key, "family": key.split(".", 1)[0], **value} for key, value in LOCAL_FORMULAS.items()]
    evaluations = []
    rows_by_case: dict[str, list[dict]] = defaultdict(list)
    for row in rows:
        rows_by_case[row["case_id"]].append(row)
        row_id = f"{row['case_id']}:{row['sigma']}"
        for spec in formulas:
            evaluation = eval_row_formula(row, spec["id"], row_id)
            if evaluation is not None:
                evaluations.append(evaluation)
    for aggregate in aggregates:
        for spec in formulas:
            evaluation = aggregate_formula(aggregate, rows_by_case[aggregate["case_id"]], spec["id"])
            if evaluation is not None:
                evaluations.append(evaluation)
    summary = {}
    total_contexts = len(rows) + len(aggregates)
    for spec in formulas:
        subset = [e for e in evaluations if e["formula_id"] == spec["id"]]
        covered_subset = [e for e in subset if e["applicable"]]
        summary[spec["id"]] = {
            "family": spec.get("family"), "expression": spec.get("expression"),
            "source": spec.get("source", []), "implementation_status": spec.get("implementation_status"),
            "inventory_entry": True, "value_evaluation_count": len(subset),
            "bound_eligible_count": len(covered_subset),
            "evaluated": len(subset), "eligible": len(covered_subset),
            "unavailable_context_count": max(total_contexts - len(subset), 0),
            "covered": sum(bool(e["covered"]) for e in covered_subset),
            "coverage_rate": sum(bool(e["covered"]) for e in covered_subset) / len(covered_subset) if covered_subset else None,
            "unavailable_reason": (unavailable_reason(spec) if spec.get("implementation_status") in {"implemented", "partial", "packet_local", "local"} else "source formula is retained as an unimplemented/unchecked route") if len(subset) < total_contexts else None,
            "categories": dict(Counter(e["category"] for e in subset if e.get("category"))),
        }
        if covered_subset:
            summary[spec["id"]]["undercoverage_count"] = sum(not bool(e["covered"]) for e in covered_subset)
            summary[spec["id"]]["undercoverage_outlier_rows"] = [e["row_id"] for e in covered_subset if e.get("covered") is False][:20]
            summary[spec["id"]]["zero_E_count"] = sum(e.get("E") == 0 for e in covered_subset)
            summary[spec["id"]]["zero_B_count"] = sum(e.get("B") == 0 for e in covered_subset)
        else:
            summary[spec["id"]]["undercoverage_count"] = 0
            summary[spec["id"]]["undercoverage_outlier_rows"] = []
            summary[spec["id"]]["zero_E_count"] = 0
            summary[spec["id"]]["zero_B_count"] = 0
    consumer_summary = {}
    for case_id, case_rows in rows_by_case.items():
        volume_errors = [observed_error(r.get("volume_f64"), r.get("volume_exact")) for r in case_rows]
        volume_errors = [float(x) for x in volume_errors if x is not None]
        sys_values = [r["sys_f64"] for r in case_rows if r.get("sys_f64") is not None]
        volume_sys_bounds = [abs(r["sys_f64"]) * float(error) / r["volume_f64"] for r in case_rows if r.get("sys_f64") is not None and r.get("volume_f64") not in {None, 0} for error in [observed_error(r.get("volume_f64"), r.get("volume_exact"))] if error is not None]
        derivatives = Counter("available" if r.get("derivative_linf") is not None else "unavailable" for r in case_rows)
        recovery = Counter("valid" if r.get("recovery_valid") is True else "invalid" if r.get("recovery_valid") is False else "unavailable" for r in case_rows)
        actions = sorted(row_proposal_action(r) for r in case_rows if row_proposal_action(r) is not None)
        accepted_actions = sorted(row_accepted_action(r) for r in case_rows if row_accepted_action(r) is not None)
        q_values = [row_proposal_q_raw(r) for r in case_rows if row_proposal_q_raw(r) is not None]
        high_q_cutoff = max(q_values, default=None)
        high_q_count = sum(q >= 0.95 * high_q_cutoff for q in q_values) if high_q_cutoff is not None else 0
        consumer_summary[case_id] = {
            "volume_f64_error_max": max(volume_errors, default=None),
            "sys_volume_error_bound_max": max(volume_sys_bounds, default=None),
            "sys_f64_values": sorted(set(sys_values)), "sys_gt_one": any(x > 1 for x in sys_values) if sys_values else None,
            "sys_gt_one_count": sum(x > 1 for x in sys_values) if sys_values else None, "derivative_categories": dict(derivatives),
            "recovery_categories": dict(recovery),
            "proposal_family_min_action": actions[0] if actions else None,
            "proposal_family_runner_up_action": actions[1] if len(actions) > 1 else None,
            "proposal_family_pairwise_margin": actions[1] - actions[0] if len(actions) > 1 else None,
            "proposal_family_minimizer_count": sum(1 for x in actions if x == actions[0]) if actions else 0,
            "accepted_family_min_action": accepted_actions[0] if accepted_actions else None,
            "accepted_family_runner_up_action": accepted_actions[1] if len(accepted_actions) > 1 else None,
            "accepted_family_pairwise_margin": accepted_actions[1] - accepted_actions[0] if len(accepted_actions) > 1 else None,
            "accepted_family_minimizer_count": sum(1 for x in accepted_actions if x == accepted_actions[0]) if accepted_actions else 0,
            "high_q_count": high_q_count,
            "predictor_bucket_counts": dict(Counter("beta_invalid" if (r.get("beta_margin_f64") if r.get("beta_margin_f64") is not None else r.get("proposal_beta_margin_f64")) is not None and (r.get("beta_margin_f64") if r.get("beta_margin_f64") is not None else r.get("proposal_beta_margin_f64")) <= 0 else "high_q_candidate" if row_proposal_q_raw(r) is not None and high_q_cutoff is not None and row_proposal_q_raw(r) >= 0.95 * high_q_cutoff else "unavailable" for r in case_rows)),
        }
    rows_by_id = {f"{r['case_id']}:{r['sigma']}": r for r in rows}
    strata = _stratum_rows(rows, aggregates)
    aggregates_by_case = {aggregate["case_id"]: aggregate for aggregate in aggregates}
    strata_reports = {}
    for name, stratum in strata.items():
        ids = {f"{r['case_id']}:{r['sigma']}" for r in stratum}
        stratum_evals = [e for e in evaluations if e["row_id"] in ids]
        strata_reports[name] = {
            "row_count": len(stratum),
            "computable_evaluation_count": len(stratum_evals),
            "predicate_categories": dict(Counter(e.get("category") for e in stratum_evals if e.get("category"))),
            "formula_coverage": {formula_id: {"evaluated": sum(e["formula_id"] == formula_id for e in stratum_evals), "eligible": sum(e["formula_id"] == formula_id and e.get("applicable") for e in stratum_evals), "covered": sum(e["formula_id"] == formula_id and e.get("covered") is True for e in stratum_evals), "undercoverage": sum(e["formula_id"] == formula_id and e.get("covered") is False for e in stratum_evals)} for formula_id in sorted({e["formula_id"] for e in stratum_evals})},
            "consumer_margins": _consumer_margins(stratum_evals, rows_by_id, name, aggregates_by_case),
        }
    payload = {
        "run_id": manifest["run_id"], "source_revision": manifest["source_revision"], "row_count": len(rows),
        "inventory_count": inventory["formula_count"], "inventory_source_revision": inventory.get("source_revision"),
        "inventory_source_note": "retained source-audit snapshot; source paths are reviewed metadata, not run-generated evidence",
        "formula_count_evaluated": len(formulas),
        "formula_summary": summary, "local_formula_registry": LOCAL_FORMULAS,
        "consumer_summary": consumer_summary, "strata_reports": strata_reports,
        "applicable_formula_ids": sorted(key for key, value in summary.items() if value["evaluated"]),
        "target_counts": dict(Counter(r["target_id"] for r in rows)),
        "cohort_counts": dict(Counter(r["cohort"] for r in rows)),
        "exact_status_counts": dict(Counter(r["exact_solver_status"] for r in rows)),
        "predicate_category_counts": dict(Counter(r["predicate_category"] for r in rows)),
        "q_predicate_category_counts": dict(Counter(r["q_predicate_category"] for r in rows)),
        "six_predicate_categories": {"beta": dict(Counter(r["predicate_category"] for r in rows)), "q": dict(Counter(r["q_predicate_category"] for r in rows))},
        "phase_timings": {
            "f64_solver_seconds": sum(r.get("f64_solver_elapsed_us", 0.0) for r in rows) / 1e6,
            "exact_solver_seconds": sum(r.get("exact_solver_elapsed_us", 0.0) for r in rows) / 1e6,
            "producer_elapsed_seconds": manifest.get("producer_elapsed_seconds", manifest.get("elapsed_seconds")),
            "serialization_elapsed_seconds": manifest.get("serialization_elapsed_seconds"),
            "analysis_elapsed_seconds": time.perf_counter() - analysis_started,
        },
        "supported": ["production Rust route atoms", "101 source-backed formula inventory", "offline consumer/ranking/predictor projections"],
        "prohibited": ["binary64 theorem", "global HK capacity for capped universes", "algebraic transfer without an algebraic oracle"],
    }
    (directory / "formula_evaluations.jsonl").write_text("".join(json.dumps(e, sort_keys=True) + "\n" for e in evaluations))
    (directory / "analysis.json").write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    return payload


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("directory", type=Path)
    args = parser.parse_args()
    print(json.dumps(evaluate(args.directory), indent=2, sort_keys=True))
