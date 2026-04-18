#!/usr/bin/env python3
"""
Verify the widened exact-clarke seed witness in SageMath.

Goal: reconstruct the quartic field and replay the exact geometry, symmetry,
      and widened seed-row checks on one backend-neutral witness artifact.
Input Artifacts: experiments/hko-local-maximum/exact-clarke/widened-seed-witness.json
Output Artifacts: experiments/hko-local-maximum/exact-clarke/widened-seed-witness-verification.json
"""

from __future__ import annotations

import json
from pathlib import Path

from sage.all import QQ, NumberField, PolynomialRing, matrix, vector


EXPERIMENT_DIR = Path(__file__).resolve().parent
WITNESS_PATH = EXPERIMENT_DIR / "widened-seed-witness.json"
OUTPUT_PATH = EXPERIMENT_DIR / "widened-seed-witness-verification.json"


def q_from_json(entry):
    return QQ(entry["num"]) / QQ(entry["den"])


def polynomial_from_desc_coefficients(coefficients_desc):
    ring = PolynomialRing(QQ, "x")
    x = ring.gen()
    degree = len(coefficients_desc) - 1
    return sum(q_from_json(coeff) * x ** (degree - index) for index, coeff in enumerate(coefficients_desc))


def field_element_from_coeff_vector(K, coeffs):
    generator = K.gen()
    return sum(q_from_json(coeff) * generator ** index for index, coeff in enumerate(coeffs))


def vector_from_coeff_matrix(K, coeff_matrix):
    return vector(K, [field_element_from_coeff_vector(K, coeffs) for coeffs in coeff_matrix])


def rows_to_matrix(K, rows):
    return matrix(K, [list(vector_from_coeff_matrix(K, row)) for row in rows])


def build_verification_payload(witness):
    polynomial = polynomial_from_desc_coefficients(
        witness["field"]["minimal_polynomial_coefficients_desc"]
    )
    K = NumberField(polynomial, witness["field"]["generator_name"])

    dual_vertices = [
        vector_from_coeff_matrix(K, dual_vertex)
        for dual_vertex in witness["geometry"]["dual_vertices_power_basis"]
    ]
    symmetry_columns = [
        vector_from_coeff_matrix(K, column)
        for column in witness["symmetry_basis"]["columns_power_basis"]
    ]
    symmetry_row_matrix = matrix(K, [list(column) for column in symmetry_columns])
    symmetry_column_matrix = symmetry_row_matrix.transpose()
    symmetry_rank = symmetry_row_matrix.rank()

    common_capacity = field_element_from_coeff_vector(
        K, witness["expected_common_scalars"]["signed_capacity_power_basis"]
    )
    common_sys_value = field_element_from_coeff_vector(
        K, witness["expected_common_scalars"]["sys_value_power_basis"]
    )

    row_family_summaries = []
    combined_seed_rows = []
    all_checks_pass = True

    for family in witness["row_families"]:
        rows = [entry["sys_row_flat_power_basis"] for entry in family["rows"]]
        row_matrix = rows_to_matrix(K, rows)
        actual_rank = row_matrix.rank()

        closure_failures = []
        normalization_failures = []
        capacity_failures = []
        sys_value_failures = []
        row_ids = []

        for entry in family["rows"]:
            row_ids.append(entry["seed_id"])
            closure = vector_from_coeff_matrix(K, entry["closure_check_power_basis"])
            if any(component != 0 for component in closure):
                closure_failures.append(entry["seed_id"])

            normalization = field_element_from_coeff_vector(
                K, entry["normalization_check_power_basis"]
            )
            if normalization != 1:
                normalization_failures.append(entry["seed_id"])

            capacity = field_element_from_coeff_vector(K, entry["signed_capacity_power_basis"])
            if capacity != common_capacity:
                capacity_failures.append(entry["seed_id"])

            sys_value = field_element_from_coeff_vector(K, entry["sys_value_power_basis"])
            if sys_value != common_sys_value:
                sys_value_failures.append(entry["seed_id"])

        family_passed = (
            len(family["rows"]) == family["expected_row_count"]
            and actual_rank == family["expected_rank"]
            and not closure_failures
            and not normalization_failures
            and not capacity_failures
            and not sys_value_failures
        )
        all_checks_pass = all_checks_pass and family_passed

        row_family_summaries.append(
            {
                "family_id": family["family_id"],
                "expected_row_count": family["expected_row_count"],
                "actual_row_count": len(family["rows"]),
                "expected_rank": family["expected_rank"],
                "actual_rank": int(actual_rank),
                "row_ids": row_ids,
                "closure_failures": closure_failures,
                "normalization_failures": normalization_failures,
                "capacity_failures": capacity_failures,
                "sys_value_failures": sys_value_failures,
                "passed": family_passed,
            }
        )
        combined_seed_rows.extend(rows)

    widened_seed_matrix = rows_to_matrix(K, combined_seed_rows)
    widened_seed_rank = widened_seed_matrix.rank()
    seed_plus_symmetry_rank = widened_seed_matrix.stack(symmetry_row_matrix).rank()
    widened_seed_right_kernel = widened_seed_matrix.right_kernel()
    widened_seed_right_kernel_dimension = widened_seed_right_kernel.dimension()
    symmetry_residual_matrix = widened_seed_matrix * symmetry_column_matrix
    symmetry_annihilation_passed = all(
        symmetry_residual_matrix[row, col] == 0
        for row in range(symmetry_residual_matrix.nrows())
        for col in range(symmetry_residual_matrix.ncols())
    )
    symmetry_residual_summary = []
    for col, label in enumerate(witness["symmetry_basis"]["labels"]):
        nonzero_entries = []
        for row in range(symmetry_residual_matrix.nrows()):
            value = symmetry_residual_matrix[row, col]
            if value != 0:
                nonzero_entries.append(
                    {
                        "row_index": row,
                        "row_id": row_family_summaries[0]["row_ids"][row]
                        if row < len(row_family_summaries[0]["row_ids"])
                        else row_family_summaries[1]["row_ids"][row - len(row_family_summaries[0]["row_ids"])],
                        "value": str(value),
                    }
                )
        symmetry_residual_summary.append(
            {
                "label": label,
                "nonzero_count": len(nonzero_entries),
                "sample_nonzero_entries": nonzero_entries[:3],
                "passed": len(nonzero_entries) == 0,
            }
        )

    summary = {
        "field": {
            "generator_name": witness["field"]["generator_name"],
            "degree": int(K.degree()),
            "minimal_polynomial": str(polynomial),
            "passed": (
                K.degree() == witness["field"]["degree"]
                and str(polynomial) == witness["field"]["minimal_polynomial_formula"]
            ),
        },
        "geometry": {
            "n_dual_vertices": len(dual_vertices),
            "dual_vertex_lengths": sorted({len(vertex) for vertex in dual_vertices}),
            "current_float_cross_check_max_abs_diff": witness["geometry"][
                "current_float_cross_check_max_abs_diff"
            ],
            "passed": len(dual_vertices) == 10 and all(len(vertex) == 4 for vertex in dual_vertices),
        },
        "symmetry_basis": {
            "expected_rank": witness["symmetry_basis"]["expected_rank"],
            "actual_rank": int(symmetry_rank),
            "n_columns": len(symmetry_columns),
            "ambient_dimension": len(symmetry_columns[0]),
            "passed": symmetry_rank == witness["symmetry_basis"]["expected_rank"],
        },
        "row_families": row_family_summaries,
        "widened_seed_union": {
            "expected_row_count": witness["expected_total_seed_rows"],
            "actual_row_count": len(combined_seed_rows),
            "actual_rank": int(widened_seed_rank),
            "ambient_dimension": widened_seed_matrix.ncols(),
            "right_kernel_dimension": int(widened_seed_right_kernel_dimension),
            "seed_plus_symmetry_rank": int(seed_plus_symmetry_rank),
            "symmetry_annihilation_passed": symmetry_annihilation_passed,
            "symmetry_residual_summary": symmetry_residual_summary,
            "kernel_dimension_minus_symmetry_dimension": int(
                widened_seed_right_kernel_dimension - symmetry_rank
            ),
        },
    }
    summary["passed"] = (
        all_checks_pass
        and summary["field"]["passed"]
        and summary["geometry"]["passed"]
        and summary["symmetry_basis"]["passed"]
        and len(combined_seed_rows) == witness["expected_total_seed_rows"]
    )
    return summary


def main():
    witness = json.loads(WITNESS_PATH.read_text())
    verification = build_verification_payload(witness)
    OUTPUT_PATH.write_text(json.dumps(verification, indent=2) + "\n")


if __name__ == "__main__":
    main()
