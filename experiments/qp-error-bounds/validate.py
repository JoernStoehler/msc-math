#!/usr/bin/env python3
"""Fail-closed integrity checks for the unconditional wide-row packet.

The producer emits rows before deciding whether a candidate is feasible.  This
validator therefore checks both sides of that boundary: atoms must be present
on rejected rows, while proposal/result fields must never be smuggled onto a
rejected row or be copied between targets.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import math
import re
from collections import Counter, defaultdict
from fractions import Fraction
from pathlib import Path
from typing import Any


EXPECTED_SCHEMA = "qp-wide-row-v1"
EXPECTED_PRODUCER = "wide-row-rust-v1"
EXPECTED_INVENTORY_SCHEMA = "qp-formula-inventory-v1"
EXPECTED_INVENTORY_COUNT = 101
EXPECTED_INVENTORY_REVISION = "ff3de9f9^"
EXPECTED_COMMAND = "bash experiments/qp-error-bounds/run.sh"
EXACT_STATUSES = {"feasible", "infeasible_or_singular", "algebraic_oracle_unavailable"}
F64_STATUSES = {"feasible", "infeasible", "singular_matrix", "NumericalFailure", "Unsupported", "TypeCViolation", "ConstraintViolation"}
TARGETS = {"original_rational", "stored_dyadic"}
PREDICATES = {"true", "false", "indeterminate"}
EXACT_PREDICATES = {"true", "false", "indeterminate", "unavailable"}
PREDICATE_CATEGORIES = {
    "exact_unavailable",
    "true|true_sound", "true|false_unsound",
    "false|true_unsound", "false|false_sound",
    "true|indeterminate_unsound", "false|indeterminate_unsound",
    "indeterminate|true", "indeterminate|false",
    "indeterminate|indeterminate", "indeterminate|indeterminate_sound",
}
EXACT_RE = re.compile(r"^-?(?:0|[1-9][0-9]*)/[1-9][0-9]*$")
LOCAL_FORMULAS = {
    "local.beta_static_margin.v1", "local.q_residual_diagnostic.v1",
    "local.q_beta_radius_raw.v1", "local.action_reciprocal.v1",
    "consumer.volume_f64_error.v1", "consumer.sys_volume_propagation.v1",
    "consumer.sys_gt_one.v1", "consumer.derivative_category.v1",
    "consumer.recovery_category.v1", "predictor.invalid_branch_bucket.v1",
    "local.beta_inverse_radius.v1", "local.beta_eta_ternary.v1", "local.q_first_order.v1",
    "local.q_correction_quadratic.v1", "local.q_action_interval.v1", "local.projected_hessian.v1",
    "local.compatible_minima.v1",
}
LOCAL_REGISTRY_REQUIRED_FIELDS = {"expression", "target", "center", "required_atoms", "hypotheses", "arithmetic_model", "consumers", "implementation_status"}
SOURCE_CONTENT_FILES = (
    "experiments/qp-error-bounds/src/main.rs",
    "experiments/qp-error-bounds/analyze.py",
    "experiments/qp-error-bounds/validate.py",
    "experiments/qp-error-bounds/test_wide.py",
    "experiments/qp-error-bounds/formula_inventory.json",
    "experiments/qp-error-bounds/coverage_ledger.json",
    "experiments/qp-error-bounds/README.md",
)


def source_content_id() -> str:
    root = Path(__file__).resolve().parents[3]
    lines = []
    for relative in SOURCE_CONTENT_FILES:
        digest = hashlib.sha256((root / relative).read_bytes()).hexdigest()
        lines.append(f"{digest}  {relative}\n")
    return hashlib.sha256("".join(lines).encode()).hexdigest()

# Formula evaluations are a projection of atoms, never an independent source.
FORMULA_ATOM = {
    "qp.assembly_C": "qp_c_f64", "qp.assembly_d": "qp_d_f64", "qp.assembly_H": "qp_h_f64",
    "kkt.q_raw": "q_raw_f64", "kkt.q_correction": "q_correction_f64",
    "kkt.q_corrected": "q_corrected_f64", "kkt.residual_norm": "kkt_residual_norm",
    "kkt.beta_margin": "beta_margin_f64", "kkt.beta_epsilon_predicate": "f64_beta_predicate",
    "kkt.beta_exact_predicate": "exact_beta_predicate", "kkt.action_from_q": "action_f64",
    "kkt.q_positive_guard": "f64_q_predicate", "kkt.exact_kkt_oracle": "action_exact",
    "qp.kkt_augmented_system": "kkt_matrix_f64", "geometry.facet_intersection": "geometry_facet_intersection",
    "geometry.omega0": "omega_matrix_f64", "geometry.transition_sign": "geometry_transition_matrix",
    "volume.four_volume_origin_star": "volume_exact", "volume.systolic_ratio": "sys_f64",
    "derivative.capacity_gradient": "derivative_f64", "recovery.beta_to_dwell_times": "recovery_dwell_times",
    "recovery.max_violation": "recovery_max_violation", "recovery.closure_error": "recovery_closure_error",
    "recovery.shoelace_action": "recovery_action_f64", "consumer.sys_capacity_ratio": "sys_f64",
    "predictor.sysext_invalid_branch_raw_q": "q_raw_f64",
    "predictor.sysext_invalid_branch_beta_margin": "beta_margin_f64",
}
FORMULA_CENTER = {
    "bound.beta_margin_heuristic": "beta_f64", "local.beta_static_margin.v1": "beta_f64",
    "local.q_residual_diagnostic.v1": "q_corrected_f64", "local.q_beta_radius_raw.v1": "q_raw_f64",
    "consumer.volume_f64_error.v1": "volume", "consumer.sys_volume_propagation.v1": "sys",
    "kkt.beta_epsilon_predicate": "beta_f64", "kkt.beta_exact_predicate": "beta_exact",
    "kkt.beta_margin": "beta_f64", "kkt.q_corrected": "q_corrected_f64", "kkt.q_correction": "q_corrected_f64",
    "kkt.q_error_bound": "q_corrected_f64", "kkt.q_raw": "q_raw_f64", "kkt.q_correction": "q_correction_f64", "kkt.q_positive_guard": "q_corrected_f64",
    "kkt.exact_kkt_oracle": "exact_action", "predictor.sysext_invalid_branch_beta_margin": "beta",
    "predictor.sysext_invalid_branch_beta_margin": "beta_f64",
    "local.beta_inverse_radius.v1": "beta_f64", "local.beta_eta_ternary.v1": "beta_f64",
    "local.q_first_order.v1": "q_corrected_f64", "local.q_correction_quadratic.v1": "q_corrected_f64",
    "local.q_action_interval.v1": "q_corrected_f64", "local.projected_hessian.v1": "beta_f64",
    "local.compatible_minima.v1": "action_interval", "aggregation.low_action_window": "action_f64",
    "aggregation.minimizer_set_from_action_intervals": "action_interval", "bound.q_first_order": "q_corrected_f64",
    "bound.q_correction_second_order": "q_corrected_f64", "kkt.action_interval_from_q_bound": "q_corrected_f64",
    "safety.action_interval_from_q_error": "q_corrected_f64", "kkt.action_from_q": "action_f64",
    "recovery.beta_to_dwell_times": "beta_f64", "recovery.shoelace_action": "action_f64",
    "volume.four_volume_origin_star": "volume_f64", "volume.systolic_ratio": "sys_f64",
    "consumer.sys_capacity_ratio": "sys_f64",
}


def _read_json(path: Path, errors: list[str]) -> dict[str, Any] | None:
    try:
        value = json.loads(path.read_text())
    except (OSError, json.JSONDecodeError) as exc:
        errors.append(f"invalid or missing {path.name}: {exc}")
        return None
    if not isinstance(value, dict):
        errors.append(f"{path.name} is not an object")
        return None
    return value


def _read_jsonl(path: Path, errors: list[str]) -> list[dict[str, Any]]:
    try:
        lines = path.read_text().splitlines()
    except OSError as exc:
        errors.append(f"invalid or missing {path.name}: {exc}")
        return []
    values: list[dict[str, Any]] = []
    for number, line in enumerate(lines, 1):
        if not line.strip():
            errors.append(f"blank/truncated line in {path.name}:{number}")
            continue
        try:
            value = json.loads(line)
        except json.JSONDecodeError as exc:
            errors.append(f"truncated or invalid JSON in {path.name}:{number}: {exc}")
            continue
        if not isinstance(value, dict):
            errors.append(f"non-object row in {path.name}:{number}")
            continue
        values.append(value)
    return values


def _finite(value: Any) -> bool:
    if isinstance(value, float):
        return math.isfinite(value)
    if isinstance(value, list):
        return all(_finite(v) for v in value)
    if isinstance(value, dict):
        return all(_finite(v) for v in value.values())
    return True


def _fraction(text: Any) -> Fraction | None:
    if not isinstance(text, str) or not EXACT_RE.fullmatch(text):
        return None
    try:
        value = Fraction(text)
    except (ValueError, ZeroDivisionError):
        return None
    return value if f"{value.numerator}/{value.denominator}" == text else None


def _expected_category(f64_label: str, exact_label: str) -> str:
    if exact_label == "unavailable":
        return "exact_unavailable"
    if f64_label == "indeterminate":
        return f"indeterminate|{exact_label}"
    suffix = "sound" if f64_label == exact_label else "unsound"
    return f"{f64_label}|{exact_label}_{suffix}"


def _same(a: Any, b: Any) -> bool:
    """JSON atom equality, allowing only serialization-scale float drift."""
    if isinstance(a, float) and isinstance(b, (float, int)):
        return math.isclose(a, float(b), rel_tol=0.0, abs_tol=1e-15)
    if isinstance(a, list) and isinstance(b, list) and len(a) == len(b):
        return all(_same(x, y) for x, y in zip(a, b))
    if isinstance(a, dict) and isinstance(b, dict) and a.keys() == b.keys():
        return all(_same(a[k], b[k]) for k in a)
    return a == b


def validate(directory: Path) -> list[str]:
    errors: list[str] = []
    manifest = _read_json(directory / "manifest.json", errors)
    inventory = _read_json(directory / "formula_inventory.json", errors)
    ledger = _read_json(directory / "coverage_ledger.json", errors)
    analysis = _read_json(directory / "analysis.json", errors)
    rows = _read_jsonl(directory / "raw_rows.jsonl", errors)
    aggregates = _read_jsonl(directory / "aggregates.jsonl", errors)
    evaluations = _read_jsonl(directory / "formula_evaluations.jsonl", errors)
    if manifest is None or inventory is None or ledger is None:
        return errors

    populations = ledger.get("populations")
    if ledger.get("schema") != "qp-population-coverage-v1" or not isinstance(populations, list):
        errors.append("coverage ledger schema/populations missing")
    else:
        ledger_cases = {entry.get("case_id") for entry in populations if isinstance(entry, dict)}
        if any(not isinstance(entry, dict) or {"case_id", "source_pointer", "selection", "target_oracle", "completeness", "intended_question", "status"} - entry.keys() for entry in populations):
            errors.append("coverage ledger entry metadata incomplete")
        if len(ledger_cases) != len(populations):
            errors.append("coverage ledger duplicate or invalid case identity")
        if not isinstance(ledger.get("source_audit_gaps"), list) or len(ledger["source_audit_gaps"]) < 2:
            errors.append("coverage ledger source-audit gaps incomplete")

    # Manifest/config/code identity is checked before any numerical claim.
    required_manifest = {"run_id", "source_revision", "source_content_id", "producer_version", "schema_version", "command", "rows", "aggregates", "expected_case_ids"}
    missing = required_manifest - manifest.keys()
    if missing:
        errors.append(f"manifest missing identity/config fields: {sorted(missing)}")
    if manifest.get("schema_version") != EXPECTED_SCHEMA:
        errors.append("manifest schema identity mismatch")
    if manifest.get("producer_version") != EXPECTED_PRODUCER:
        errors.append("manifest producer/code identity mismatch")
    if manifest.get("command") != EXPECTED_COMMAND:
        errors.append("manifest command/config identity mismatch")
    try:
        live_source_id = source_content_id()
    except OSError as exc:
        errors.append(f"cannot compute live source content identity: {exc}")
    else:
        if manifest.get("source_content_id") != live_source_id:
            errors.append("source content identity mismatch")
    if isinstance(manifest.get("source_revision"), str) and manifest.get("run_id") != f"wide-{manifest['source_revision']}":
        errors.append("run/source revision identity mismatch")
    if manifest.get("rows") != len(rows):
        errors.append("row count mismatch (truncation)")
    if manifest.get("aggregates") != len(aggregates):
        errors.append("aggregate count mismatch (truncation)")
    if not isinstance(manifest.get("expected_case_ids"), list):
        errors.append("manifest expected_case_ids is not a list")

    formulas = inventory.get("formulas")
    formula_ids = {f.get("id") for f in formulas if isinstance(f, dict)} if isinstance(formulas, list) else set()
    if inventory.get("schema") != EXPECTED_INVENTORY_SCHEMA or inventory.get("formula_count") != len(formulas or []) or inventory.get("formula_count") != EXPECTED_INVENTORY_COUNT or len(formula_ids) != len(formulas or []) or inventory.get("source_revision") != EXPECTED_INVENTORY_REVISION:
        errors.append("formula inventory count/schema/source identity mismatch")
    known_formulas = formula_ids | LOCAL_FORMULAS

    # Rows and repeated population metadata.
    rows_by_case: dict[str, list[dict[str, Any]]] = defaultdict(list)
    row_map: dict[tuple[str, str, tuple[int, ...]], dict[str, Any]] = {}
    for index, row in enumerate(rows):
        if not _finite(row):
            errors.append(f"non-finite numeric atom in row {index}")
        required = {"run_id", "source_revision", "producer_version", "schema_version", "case_id", "sigma", "target_id", "exact_solver_status", "f64_solver_status", "lifecycle_events", "route_count_scope", "predicate_category", "q_predicate_category", "exact_beta_particular_predicate", "exact_beta_selection_status"}
        missing_row = required - row.keys()
        if missing_row:
            errors.append(f"row {index} missing required fields: {sorted(missing_row)}")
            continue
        if row.get("run_id") != manifest.get("run_id") or row.get("source_revision") != manifest.get("source_revision"):
            errors.append("mixed run or source revision")
        if row.get("producer_version") != manifest.get("producer_version") or row.get("schema_version") != manifest.get("schema_version"):
            errors.append("mixed producer/schema artifact")
        target = row.get("target_id")
        if target not in TARGETS:
            errors.append("invalid target id")
        if target == "original_rational":
            if row.get("target_coordinate_kind") != "original_rational_coordinates" or row.get("original_rational_dual_vertices_exact") is None or row.get("stored_dyadic_dual_vertices_exact") is not None:
                errors.append("original target/cohort/source mislabeling")
            if row.get("intended_algebraic_status") != "not_applicable_original_rational_source":
                errors.append("original target has algebraic oracle label")
        elif target == "stored_dyadic":
            if row.get("target_coordinate_kind") != "exact_binary64_dyadic_coordinates" or row.get("stored_dyadic_dual_vertices_exact") is None or row.get("original_rational_dual_vertices_exact") is not None:
                errors.append("stored-dyadic target/cohort/source mislabeling")
            if row.get("intended_algebraic_status") != "unavailable_no_genuine_algebraic_oracle":
                errors.append("stored-dyadic target has intended algebraic label")
        sigma = row.get("sigma")
        if not isinstance(sigma, list) or not sigma or any(not isinstance(x, int) or isinstance(x, bool) or x < 0 for x in sigma) or len(set(sigma)) != len(sigma):
            errors.append("invalid sigma identity")
            sigma_key: tuple[int, ...] = tuple()
        else:
            sigma_key = tuple(sigma)
        identity = (str(row.get("case_id")), str(row.get("run_id")), sigma_key)
        if identity in row_map:
            errors.append("duplicate row identity")
        row_map[identity] = row
        rows_by_case[str(row.get("case_id"))].append(row)
        for key in ("beta_exact", "q_exact", "action_exact", "volume_exact"):
            values = row.get(key) if key == "beta_exact" else [row.get(key)]
            if values is None:
                values = []
            for value in values:
                if value is not None and _fraction(value) is None:
                    errors.append("noncanonical exact value")

        f64_status = row.get("f64_solver_status")
        if f64_status not in F64_STATUSES:
            errors.append("invalid f64 solver status")
        exact_status = row.get("exact_solver_status")
        if exact_status not in EXACT_STATUSES:
            errors.append("ambiguous exact solver status")
        if row.get("route_count_scope") != "case_population_summary_repeated_on_each_sigma_row":
            errors.append("route count scope is not population-level")
        if row.get("route_attempt_status") == "unavailable:aggregate_route_only" and any(row.get(k) is not None for k in ("route_retained", "route_pruned", "route_candidate_order_f64", "route_candidate_order_exact", "route_q_rank_desc", "route_action_rank_asc", "route_exact_action_rank_asc", "route_maximum_q_member", "route_minimum_action_member", "route_low_action_window_member")):
            errors.append("aggregate-only route row carries concrete per-sigma route status")
        counts = [row.get(k) for k in ("route_population_sigma_count", "route_population_admissible_count", "route_population_indeterminate_count", "route_population_failure_count")]
        if any(not isinstance(x, int) or isinstance(x, bool) or x < 0 for x in counts) or counts[1] + counts[2] + counts[3] > counts[0]:
            errors.append("invalid population route counts")
        events = row.get("lifecycle_events")
        if not isinstance(events, list) or len(events) != 5 or events[:3] != ["declared", "route_eligible", "attempted"] or events[3] != row.get("lifecycle_stage") or events[4] != f64_status:
            errors.append("incomplete lifecycle")
        if row.get("lifecycle_stage") != "visited" or row.get("lifecycle_reason") not in {"production_kkt", "production_outcome"} or (row.get("lifecycle_reason") == "production_kkt") != (f64_status == "feasible"):
            errors.append("inconsistent lifecycle outcome")

        # q_raw and residual atoms can be mathematically defined before a
        # production feasibility decision.  They are deliberately allowed on
        # rejected rows; beta/action and downstream route proposals are not.
        proposal_fields = ("beta_f64", "mu_f64", "xi_f64", "q_corrected_f64", "q_correction_f64", "action_f64", "q_error_bound", "q_beta_radius_bound", "derivative_linf", "derivative_components", "derivative_f64", "recovery_closure_error", "recovery_max_violation", "recovery_action_error", "recovery_action_f64", "recovery_dwell_times", "recovery_valid")
        if f64_status != "feasible" and any(row.get(k) is not None for k in proposal_fields):
            errors.append("unconditional proposal on rejected/non-feasible row")
        if f64_status != "feasible" and any(row.get(k) is not None for k in ("accepted_q_raw_f64", "accepted_q_corrected_f64", "accepted_q_correction_f64", "accepted_action_f64")):
            errors.append("accepted solver projection on rejected/non-feasible row")
        if f64_status == "feasible" and any(row.get(k) is None for k in ("beta_f64", "mu_f64", "xi_f64", "q_raw_f64", "q_corrected_f64", "q_correction_f64", "action_f64")):
            errors.append("feasible row missing solver proposal")
        if row.get("proposal_q_f64") is not None and row.get("proposal_q_raw_f64") != row.get("proposal_q_f64"):
            errors.append("proposal raw-Q aliases disagree")
        if f64_status == "feasible" and any(row.get(k) is None for k in ("accepted_q_raw_f64", "accepted_q_corrected_f64", "accepted_q_correction_f64", "accepted_action_f64")):
            errors.append("feasible row missing accepted solver projection")
        # `q_raw_f64` is the unconditional proposal center and is valid even
        # when production never produced a corrected-Q result.  Formula joins
        # keep its `q_raw_f64` center distinct from `q_corrected_f64`.

        # Exact status and values have one unambiguous interpretation.
        if exact_status == "feasible":
            if row.get("q_exact") is None or row.get("exact_beta_predicate") != "true" or row.get("exact_q_predicate") not in {"true", "false"}:
                errors.append("feasible exact status without exact oracle atoms")
            if row.get("beta_error_linf") is not None and row.get("exact_beta_selection_status") != "unique_exact_solution":
                errors.append("beta error reported without unique exact selection")
        else:
            algebra_status = row.get("exact_algebra_status")
            consistent = algebra_status == "consistent_no_positive_beta"
            beta_allowed = consistent and row.get("q_exact") is not None
            if not consistent and any(row.get(k) is not None for k in ("beta_exact", "q_exact", "action_exact", "action_exact_defined")):
                errors.append("inconsistent exact status carries exact proposal")
            if consistent and (not beta_allowed or row.get("action_exact") is not None or row.get("exact_beta_predicate") != "false"):
                errors.append("consistent exact status lacks exact diagnostic atoms or carries physical action")
            if not consistent and (row.get("exact_beta_predicate") != "unavailable" or row.get("exact_q_predicate") != "unavailable"):
                errors.append("unavailable exact status carries exact predicate")
        if exact_status == "algebraic_oracle_unavailable" and target != "stored_dyadic":
            errors.append("algebraic-oracle status on non-dyadic target")
        if row.get("action_exact") is not None and (_fraction(row.get("action_exact")) is None or _fraction(row.get("action_exact")) <= 0):
            errors.append("invalid exact action proposal")
        if row.get("action_f64") is not None and (not isinstance(row["action_f64"], (int, float)) or row["action_f64"] <= 0):
            errors.append("invalid f64 action proposal")

        for label in ("f64_beta_predicate", "f64_q_predicate"):
            if row.get(label) not in PREDICATES:
                errors.append(f"invalid {label}")
        for label in ("exact_beta_predicate", "exact_q_predicate"):
            if row.get(label) not in EXACT_PREDICATES:
                errors.append(f"invalid {label}")
        if row.get("exact_solver_status") == "feasible":
            beta_exact = [_fraction(v) for v in row.get("beta_exact") or []]
            if beta_exact and not all(v is not None and v > 0 for v in beta_exact):
                errors.append("selected exact beta reference is not strictly positive")
            q_truth = "true" if _fraction(row.get("q_exact")) is not None and _fraction(row["q_exact"]) > 0 else "false"
            if row.get("exact_beta_predicate") != "true" or row.get("exact_q_predicate") != q_truth:
                errors.append("exact feasibility predicate does not match exact solver status")
        elif row.get("exact_algebra_status") == "consistent_no_positive_beta":
            beta_exact = [_fraction(v) for v in row.get("beta_exact") or []]
            beta_truth = "true" if beta_exact and all(v is not None and v > 0 for v in beta_exact) else "false" if any(v is not None and v < 0 for v in beta_exact) else "indeterminate"
            q_value = _fraction(row.get("q_exact"))
            q_truth = "true" if q_value is not None and q_value > 0 else "false" if q_value is not None else "unavailable"
            if row.get("exact_beta_predicate") != "false" or row.get("exact_q_predicate") != q_truth:
                errors.append("consistent exact feasibility predicate does not match status")
            witness = [_fraction(v) for v in row.get("exact_beta_witness") or []]
            witness_truth = "true" if witness and all(v is not None and v > 0 for v in witness) else "false" if any(v is not None and v < 0 for v in witness) else "indeterminate" if witness else "unavailable"
            if row.get("exact_beta_particular_predicate") != witness_truth:
                errors.append("particular beta classification does not match witness")
        elif row.get("exact_algebra_status") == "rational_system_inconsistent":
            if row.get("exact_beta_predicate") != "unavailable" or row.get("exact_beta_particular_predicate") != "unavailable":
                errors.append("inconsistent exact system has beta feasibility classification")
        if row.get("f64_solver_status") != "feasible":
            expected_beta, expected_q = "indeterminate", "indeterminate"
        else:
            margin = row.get("beta_margin_f64")
            expected_beta = "true" if margin is not None and margin > 1e-9 else "false" if margin is not None and margin < -1e-9 else "indeterminate"
            q = row.get("q_corrected_f64")
            expected_q = "true" if q is not None and q > 1e-15 else "false" if q is not None and q < -1e-15 else "indeterminate"
        if row.get("f64_beta_predicate") != expected_beta or row.get("f64_q_predicate") != expected_q:
            errors.append("f64 predicate does not match solver atoms")
        if row.get("predicate_category") != _expected_category(row.get("f64_beta_predicate"), row.get("exact_beta_predicate")) or row.get("q_predicate_category") != _expected_category(row.get("f64_q_predicate"), row.get("exact_q_predicate")):
            errors.append("predicate category mismatch")
        if row.get("predicate_category") not in PREDICATE_CATEGORIES or row.get("q_predicate_category") not in PREDICATE_CATEGORIES:
            errors.append("invalid predicate category")

    # Metadata repeated over a case is part of the population contract.
    for case_id, case_rows in rows_by_case.items():
        for key in ("cohort", "source_family", "source_id", "universe_contract", "target_id", "route_population_sigma_count", "route_population_admissible_count", "route_population_indeterminate_count", "route_population_failure_count"):
            if len({json.dumps(r.get(key), sort_keys=True) for r in case_rows}) != 1:
                errors.append("inconsistent repeated population metadata")
        population = case_rows[0].get("route_population_sigma_count", 0)
        if isinstance(population, int) and population < len(case_rows):
            errors.append("emitted rows exceed declared population")

    # Named cohort contracts prevent silently collapsing a regression role into
    # a generic HKO or generated population.
    named_sigmas = {
        "hko_beta_boundary": [0, 1, 6, 7, 3, 4, 5, 9],
        "hko_near_singular_false_acceptance": [1, 8, 7, 3, 4, 5, 9],
        "hko_residual_q_failure": [0, 1, 7, 3, 9, 5],
        "hko_rank_deficient": [1, 7, 2, 8, 4, 6, 5],
        "hypercube_exact_zero_beta_boundary": [0, 2, 1, 5, 6],
    }
    for case_id, sigma in named_sigmas.items():
        case_rows = rows_by_case.get(case_id, [])
        if len(case_rows) != 1 or case_rows[0].get("sigma") != sigma:
            errors.append("named population sigma/cohort mismatch")
    for case_id in ("hko_beta_boundary", "hko_near_singular_false_acceptance", "hko_residual_q_failure", "hko_rank_deficient"):
        for row in rows_by_case.get(case_id, []):
            if row.get("target_id") != "stored_dyadic" or row.get("source_family") != "stored_dyadic_hko_like" or row.get("intended_algebraic_status") != "unavailable_no_genuine_algebraic_oracle":
                errors.append("HKO stored-dyadic/algebraic cohort separation mismatch")

    # Aggregates are reconstructed from rows, including ties and 5% windows.
    aggregate_map: dict[str, dict[str, Any]] = {}
    for aggregate in aggregates:
        case_id = aggregate.get("case_id")
        if case_id in aggregate_map:
            errors.append("duplicate aggregate identity")
        aggregate_map[case_id] = aggregate
        if aggregate.get("run_id") != manifest.get("run_id"):
            errors.append("mixed aggregate run identity")
        case_rows = rows_by_case.get(case_id, [])
        if aggregate.get("row_count") != len(case_rows):
            errors.append("aggregate row count mismatch")
        if aggregate.get("universe_contract") != (case_rows[0].get("universe_contract") if case_rows else None) or aggregate.get("candidate_completeness") != aggregate.get("universe_contract"):
            errors.append("aggregate population contract mismatch")
        if not aggregate.get("low_action_window_definition"):
            errors.append("missing low-action-window definition")
        actions = sorted((r["proposal_action_f64"], tuple(r["sigma"])) for r in case_rows if isinstance(r.get("proposal_action_f64"), (int, float)))
        accepted_actions = sorted((r.get("accepted_action_f64", r.get("action_f64")), tuple(r["sigma"])) for r in case_rows if isinstance(r.get("accepted_action_f64", r.get("action_f64")), (int, float)))
        exact_actions = sorted((_fraction(r["action_exact"]), tuple(r["sigma"])) for r in case_rows if _fraction(r.get("action_exact")) is not None)
        fmin = actions[0][0] if actions else None
        emin = exact_actions[0][0] if exact_actions else None
        expected_f64_min = fmin
        expected_exact_min = None if emin is None else f"{emin.numerator}/{emin.denominator}"
        if not _same(aggregate.get("f64_min_action"), expected_f64_min) or aggregate.get("exact_min_action") != expected_exact_min:
            errors.append("aggregate minimum mismatch")
            if aggregate.get("exact_min_action") != expected_exact_min:
                errors.append("aggregate exact minimum mismatch")
        expected_f64_runner = actions[1][0] if len(actions) > 1 else None
        expected_exact_runner = None if len(exact_actions) < 2 else f"{exact_actions[1][0].numerator}/{exact_actions[1][0].denominator}"
        if not _same(aggregate.get("f64_runner_up_action"), expected_f64_runner) or aggregate.get("exact_runner_up_action") != expected_exact_runner:
            errors.append("aggregate runner-up mismatch")
        fcount = sum(a == fmin for a, _ in actions) if fmin is not None else 0
        ecount = sum(a == emin for a, _ in exact_actions) if emin is not None else 0
        cutoff = fmin * 1.05 if fmin is not None else None
        low_f = sum(a <= cutoff for a, _ in actions) if cutoff is not None else 0
        low_e = sum(a <= emin * Fraction(21, 20) for a, _ in exact_actions) if emin is not None else None
        expected_sigma = list(actions[0][1]) if actions else None
        checks = (("f64_minimizer_count", fcount), ("exact_minimizer_count", ecount), ("f64_low_action_window_count", low_f), ("exact_low_action_window_count", low_e), ("f64_low_action_window_cutoff", cutoff), ("f64_minimizer_sigma", expected_sigma))
        for key, expected in checks:
            if not _same(aggregate.get(key), expected):
                errors.append("aggregate filter/count reconstruction mismatch")
        proposal_checks = (("proposal_min_action", fmin), ("proposal_runner_up_action", actions[1][0] if len(actions) > 1 else None), ("proposal_minimizer_count", fcount), ("proposal_low_action_window_count", low_f), ("proposal_low_action_window_cutoff", cutoff), ("proposal_minimizer_sigma", expected_sigma))
        for key, expected in proposal_checks:
            if not _same(aggregate.get(key), expected):
                errors.append("aggregate proposal reconstruction mismatch")
        accepted_min = accepted_actions[0][0] if accepted_actions else None
        accepted_cutoff = accepted_min * 1.05 if accepted_min is not None else None
        accepted_count = sum(abs(a - accepted_min) <= 1e-12 for a, _ in accepted_actions) if accepted_min is not None else 0
        accepted_low = sum(a <= accepted_cutoff for a, _ in accepted_actions) if accepted_cutoff is not None else 0
        accepted_checks = (("accepted_min_action", accepted_min), ("accepted_runner_up_action", accepted_actions[1][0] if len(accepted_actions) > 1 else None), ("accepted_minimizer_count", accepted_count), ("accepted_low_action_window_count", accepted_low), ("accepted_low_action_window_cutoff", accepted_cutoff))
        for key, expected in accepted_checks:
            if not _same(aggregate.get(key), expected):
                errors.append("accepted aggregate reconstruction mismatch")
    expected_cases = set(manifest.get("expected_case_ids", []))
    if set(aggregate_map) != expected_cases or set(rows_by_case) != expected_cases:
        errors.append("missing or unexpected case aggregate")
    if isinstance(populations, list):
        ledger_cases = {entry.get("case_id") for entry in populations if isinstance(entry, dict)}
        if ledger_cases != expected_cases:
            errors.append("coverage ledger case set mismatch")
        for entry in populations:
            if not isinstance(entry, dict) or entry.get("case_id") not in rows_by_case:
                continue
            case_rows = rows_by_case[entry["case_id"]]
            contract = case_rows[0].get("universe_contract") if case_rows else None
            if entry.get("completeness") != contract and not str(entry.get("completeness", "")).startswith(str(contract)):
                errors.append("coverage ledger completeness/row contract mismatch")
            if entry.get("case_id") == "random_3x5_s0_0" and len(case_rows) < 4:
                errors.append("product tie population missing named rows")
            if entry.get("case_id") == "seed99540836_q4_p5_attempt405000000000" and len(case_rows) < 1000:
                errors.append("pinned transition population below declared high-information size")

    # Analysis and formula evaluations must identify this exact packet.
    if analysis is not None:
        if analysis.get("run_id") != manifest.get("run_id") or analysis.get("source_revision") != manifest.get("source_revision") or analysis.get("row_count") != len(rows) or analysis.get("inventory_count") != inventory.get("formula_count"):
            errors.append("mixed analysis artifact identity")
        for key, expected in (("target_counts", Counter(r.get("target_id") for r in rows)), ("cohort_counts", Counter(r.get("cohort") for r in rows)), ("exact_status_counts", Counter(r.get("exact_solver_status") for r in rows)), ("predicate_category_counts", Counter(r.get("predicate_category") for r in rows)), ("q_predicate_category_counts", Counter(r.get("q_predicate_category") for r in rows))):
            if analysis.get(key) != dict(expected):
                errors.append("analysis population/filter count mismatch")
        eval_counts = Counter(e.get("formula_id") for e in evaluations)
        summaries = analysis.get("formula_summary", {})
        if not isinstance(summaries, dict):
            errors.append("analysis formula summary missing")
        else:
            for formula_id, count in eval_counts.items():
                if formula_id not in summaries or summaries[formula_id].get("value_evaluation_count") != count:
                    errors.append("analysis formula evaluation count mismatch")
            for formula_id, summary in summaries.items():
                if not isinstance(summary, dict) or not isinstance(summary.get("value_evaluation_count"), int) or summary["value_evaluation_count"] < 0:
                    errors.append("invalid analysis formula count")
        registry = analysis.get("local_formula_registry")
        if not isinstance(registry, dict) or set(registry) != LOCAL_FORMULAS:
            errors.append("local formula registry identity mismatch")
        elif any(not isinstance(spec, dict) or not LOCAL_REGISTRY_REQUIRED_FIELDS <= spec.keys() for spec in registry.values()):
            errors.append("local formula registry metadata incomplete")
        six = analysis.get("six_predicate_categories")
        if six is not None:
            expected_six = {
                "beta": dict(Counter(r.get("predicate_category") for r in rows)),
                "q": dict(Counter(r.get("q_predicate_category") for r in rows)),
            }
            if six != expected_six:
                errors.append("six predicate category count mismatch")
        # Consumer-margin summaries are recomputed from the packet atoms.  In
        # particular this rejects restoring the former scalar-|Q|/|A|
        # denominator under a plausible-looking E/M or B/M value.
        reports = analysis.get("strata_reports")
        if not isinstance(reports, dict):
            errors.append("analysis consumer-margin strata missing")
        else:
            try:
                from analyze import _consumer_margins, _stratum_rows

                rows_by_id = {f"{row['case_id']}:{row['sigma']}": row for row in rows}
                aggregates_by_case = {aggregate["case_id"]: aggregate for aggregate in aggregates}
                strata = _stratum_rows(rows, aggregates)
                for name, stratum_rows in strata.items():
                    ids = {f"{row['case_id']}:{row['sigma']}" for row in stratum_rows}
                    expected = _consumer_margins(
                        [evaluation for evaluation in evaluations if evaluation.get("row_id") in ids],
                        rows_by_id, name, aggregates_by_case,
                    )
                    actual = reports.get(name, {}).get("consumer_margins")
                    if actual is None or not _same(actual, expected):
                        errors.append("analysis consumer margin mismatch")
            except (ImportError, KeyError, TypeError, ValueError):
                errors.append("analysis consumer-margin reconstruction failed")

    for evaluation in evaluations:
        if not _finite(evaluation):
            errors.append("non-finite formula evaluation")
        formula_id = evaluation.get("formula_id")
        if formula_id not in known_formulas:
            errors.append("unknown formula evaluation")
            continue
        if evaluation.get("run_id") != manifest.get("run_id"):
            errors.append("mixed formula-evaluation run identity")
        row_id = evaluation.get("row_id")
        owner: dict[str, Any] | None = None
        if isinstance(row_id, str) and ":aggregate" in row_id:
            case_id = row_id.removesuffix(":aggregate")
            owner = rows_by_case.get(case_id, [None])[0]
        elif isinstance(row_id, str):
            case_id, _, sigma_text = row_id.partition(":")
            owner = next((r for r in rows_by_case.get(case_id, []) if str(r.get("sigma")) == sigma_text), None)
        if owner is None:
            errors.append("formula evaluation references unknown row")
            continue
        if evaluation.get("case_id") != owner.get("case_id") or evaluation.get("target_id") != owner.get("target_id"):
            errors.append("formula target/row identity mismatch")
        center = evaluation.get("center_id")
        expected_center = FORMULA_CENTER.get(formula_id)
        aliases = {
            "beta_f64": {"beta_f64", "proposal_beta_f64"},
            "q_corrected_f64": {"q_corrected_f64", "proposal_q_corrected_f64"},
        }
        if expected_center is not None and center not in aliases.get(expected_center, {expected_center}):
            errors.append("formula center mismatch")
        if evaluation.get("E") is not None or evaluation.get("B") is not None:
            if center is None:
                errors.append("error/bound evaluation without center")
            if evaluation.get("E_exact") is None and evaluation.get("E") is not None:
                errors.append("error evaluation missing exact representation")
            if evaluation.get("B_exact") is None and evaluation.get("B") is not None:
                errors.append("bound evaluation missing exact representation")
        if evaluation.get("applicable") != (evaluation.get("E") is not None and evaluation.get("B") is not None):
            errors.append("formula applicability mismatch")
        if formula_id in FORMULA_ATOM and owner.get(FORMULA_ATOM[formula_id]) is not None:
            expected = owner[FORMULA_ATOM[formula_id]]
            if formula_id == "qp.kkt_augmented_system":
                expected = {"matrix": owner.get("kkt_matrix_f64"), "rhs": owner.get("kkt_rhs_f64")}
            if not _same(evaluation.get("value"), expected):
                errors.append("proxy formula evaluation value")
        if formula_id == "kkt.exact_kkt_oracle" and owner.get("action_exact") is None:
            errors.append("exact formula evaluated without exact action")
        if evaluation.get("category") is not None and evaluation.get("category") not in PREDICATE_CATEGORIES:
            errors.append("invalid formula predicate category")
        if formula_id in {"local.q_residual_diagnostic.v1", "local.q_first_order.v1", "local.q_correction_quadratic.v1"}:
            if evaluation.get("center_id") != "proposal_q_corrected_f64" or evaluation.get("comparison_id") != "proposal_q_correction_f64":
                errors.append("proposal Q correction diagnostic center/target mismatch")
    return sorted(set(errors))


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("directory", type=Path)
    args = parser.parse_args()
    errors = validate(args.directory)
    if errors:
        raise SystemExit("\n".join(errors))
    print("valid packet")
