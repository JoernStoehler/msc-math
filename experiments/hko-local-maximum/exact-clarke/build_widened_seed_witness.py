#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# ///

"""Assemble a backend-neutral widened representative-row witness.

Goal: freeze the current Packet 3 widened seed surface into one self-contained
      exact witness artifact that later exact backends and Sage verifiers can
      both consume.
Input Artifacts: experiments/hko-local-maximum/exact-clarke/hko-geometry.json
                 experiments/hko-local-maximum/exact-clarke/hko-symmetry-tangent.json
                 experiments/hko-local-maximum/exact-clarke/hko-volume-derivative.json
                 experiments/hko-local-maximum/exact-clarke/endpoint-seed-rows.json
                 experiments/hko-local-maximum/exact-clarke/midpoint-seed-rows.json
Output Artifacts: experiments/hko-local-maximum/exact-clarke/widened-seed-witness.json
"""

from __future__ import annotations

import json
from pathlib import Path

from sympy import simplify, sqrt, sympify
from sympy.polys.numberfields import to_number_field


EXPERIMENT_DIR = Path(__file__).resolve().parent
GEOMETRY_PATH = EXPERIMENT_DIR / "hko-geometry.json"
SYMMETRY_PATH = EXPERIMENT_DIR / "hko-symmetry-tangent.json"
VOLUME_PATH = EXPERIMENT_DIR / "hko-volume-derivative.json"
ENDPOINT_ROWS_PATH = EXPERIMENT_DIR / "endpoint-seed-rows.json"
MIDPOINT_ROWS_PATH = EXPERIMENT_DIR / "midpoint-seed-rows.json"
OUTPUT_PATH = EXPERIMENT_DIR / "widened-seed-witness.json"

T_EXPR = sqrt(5 - 2 * sqrt(5))
FIELD_DEGREE = to_number_field(T_EXPR).minpoly.degree()


def rational_json(value):
    value = sympify(value)
    num, den = value.as_numer_denom()
    return {"num": int(num), "den": int(den)}


def coeff_vector_json(expr):
    alg = to_number_field(simplify(sympify(expr)), T_EXPR)
    coeffs_desc = list(alg.coeffs())
    coeffs_asc = list(reversed(coeffs_desc))
    coeffs_asc += [0] * (FIELD_DEGREE - len(coeffs_asc))
    return [rational_json(coeff) for coeff in coeffs_asc]


def canonical_expr_str(expr):
    return str(to_number_field(simplify(sympify(expr)), T_EXPR).as_expr())


def convert_seed_row(row):
    return {
        "seed_id": row["seed_id"],
        "subset": row["subset"],
        "representative_permutation": row["representative_permutation"],
        "exact_beta_profile": row["exact_beta_profile"],
        "closure_check": row["closure_check"],
        "closure_check_power_basis": [coeff_vector_json(entry) for entry in row["closure_check"]],
        "normalization_check": row["normalization_check"],
        "normalization_check_power_basis": coeff_vector_json(row["normalization_check"]),
        "signed_capacity": row["capacity"],
        "signed_capacity_power_basis": coeff_vector_json(row["capacity"]),
        "sys_value": row["sys_value"],
        "sys_value_power_basis": coeff_vector_json(row["sys_value"]),
        "sys_row_flat_power_basis": row["sys_row_flat_power_basis"],
    }


def ensure_constant_scalar(rows, key):
    first = canonical_expr_str(rows[0][key])
    for row in rows[1:]:
        current = canonical_expr_str(row[key])
        if current != first:
            raise ValueError(f"expected common {key}, found {first!r} and {current!r}")
    return first


def main() -> None:
    geometry = json.loads(GEOMETRY_PATH.read_text())
    symmetry = json.loads(SYMMETRY_PATH.read_text())
    volume = json.loads(VOLUME_PATH.read_text())
    endpoint_rows = json.loads(ENDPOINT_ROWS_PATH.read_text())
    midpoint_rows = json.loads(MIDPOINT_ROWS_PATH.read_text())

    endpoint_entries = endpoint_rows["endpoint_seed_rows"]
    midpoint_entries = midpoint_rows["midpoint_seed_rows"]
    all_rows = endpoint_entries + midpoint_entries

    common_signed_capacity = ensure_constant_scalar(all_rows, "capacity")
    common_sys_value = ensure_constant_scalar(all_rows, "sys_value")

    payload = {
        "witness_version": 1,
        "witness_scope": (
            "Current widened Packet 3 representative-row surface for the exact Clarke route. "
            "This is not the final theorem witness because two asymmetric "
            "seven-facet representatives remain unresolved and the active matrix itself "
            "is not yet assembled."
        ),
        "source_artifacts": [
            GEOMETRY_PATH.name,
            SYMMETRY_PATH.name,
            VOLUME_PATH.name,
            ENDPOINT_ROWS_PATH.name,
            MIDPOINT_ROWS_PATH.name,
        ],
        "field": {
            "generator_name": geometry["field"]["generator_name"],
            "generator_formula": geometry["field"]["formulas"]["generator"],
            "degree": geometry["field"]["degree"],
            "minimal_polynomial_formula": geometry["field"]["formulas"]["minimal_polynomial"],
            "minimal_polynomial_coefficients_asc": geometry["field"]["minimal_polynomial"],
            "minimal_polynomial_coefficients_desc": [
                {"num": 1, "den": 1},
                {"num": 0, "den": 1},
                {"num": -10, "den": 1},
                {"num": 0, "den": 1},
                {"num": 5, "den": 1},
            ],
        },
        "facet_order": geometry["facet_order"],
        "coordinate_order": volume["coordinate_order"],
        "geometry": {
            "dual_vertices_power_basis": geometry["dual_vertices_power_basis"],
            "current_float_cross_check_max_abs_diff": geometry["cross_check_current_numerical"][
                "max_abs_diff"
            ],
        },
        "symmetry_basis": {
            "labels": symmetry["labels"],
            "columns_power_basis": symmetry["columns_power_basis"],
            "expected_rank": symmetry["rank"],
        },
        "expected_common_scalars": {
            "signed_capacity": common_signed_capacity,
            "signed_capacity_power_basis": coeff_vector_json(common_signed_capacity),
            "sys_value": common_sys_value,
            "sys_value_power_basis": coeff_vector_json(common_sys_value),
        },
        "row_families": [
            {
                "family_id": "endpoint_seed_rows",
                "expected_row_count": endpoint_rows["n_exactified_endpoint_seed_rows"],
                "expected_rank": endpoint_rows["rank_of_endpoint_seed_rows"],
                "rows": [convert_seed_row(row) for row in endpoint_entries],
            },
            {
                "family_id": "midpoint_seed_rows",
                "expected_row_count": midpoint_rows["n_exactified_midpoint_seed_rows"],
                "expected_rank": midpoint_rows["rank_of_midpoint_seed_rows"],
                "rows": [convert_seed_row(row) for row in midpoint_entries],
            },
        ],
        "expected_total_seed_rows": len(all_rows),
        "current_limitations": [
            "Two asymmetric seven-facet representatives are not included yet.",
            "This witness does not yet contain an active-gradient matrix, kernel basis, or symmetry-equality certificate.",
            "This witness currently covers the widened representative surface, not the final theorem-facing active set.",
        ],
        "theorem_use": (
            "Use this artifact as the first backend-neutral Sage-facing witness "
            "surface for Packet 3. Any producer that can emit this shape can "
            "delegate exact verification of ranks and scalar invariants to Sage."
        ),
    }
    OUTPUT_PATH.write_text(json.dumps(payload, indent=2) + "\n")


if __name__ == "__main__":
    main()
