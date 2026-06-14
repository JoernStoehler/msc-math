#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = ["sympy"]
# ///

"""Assemble exact reduced sys-row prototypes for the Packet 3 checker.

Goal: combine the exact capacity prototype rows and the exact HKO volume row
      into exact prototype sys rows in the facet-major `R^40` coordinate order.
Input Artifacts: experiments/hko-local-maximum/history/exact-witness/segment-a-gradient-reduction.json
                 experiments/hko-local-maximum/history/exact-witness/hko-volume-derivative.json
Output Artifacts: experiments/hko-local-maximum/history/exact-witness/reduced-sys-prototypes.json
"""

from __future__ import annotations

import json
from functools import lru_cache
from pathlib import Path

from sympy import Rational, expand, simplify, sympify, symbols
from sympy.polys.numberfields import to_number_field


EXPERIMENT_DIR = Path(__file__).resolve().parent
SEGMENT_PATH = EXPERIMENT_DIR / "segment-a-gradient-reduction.json"
VOLUME_PATH = EXPERIMENT_DIR / "hko-volume-derivative.json"
OUTPUT_PATH = EXPERIMENT_DIR / "reduced-sys-prototypes.json"

T_EXPR = symbols("t")
LAMBDA = symbols("lambda")
FIELD_GENERATOR_EXPR = sympify("sqrt(5 - 2*sqrt(5))")
FIELD_DEGREE = to_number_field(FIELD_GENERATOR_EXPR).minpoly.degree()


def rational_json(value):
    value = sympify(value)
    num, den = value.as_numer_denom()
    return {"num": int(num), "den": int(den)}


@lru_cache(maxsize=None)
def coeff_vector_json_cached(expr_text):
    expr = sympify(expr_text)
    alg = to_number_field(expr, FIELD_GENERATOR_EXPR)
    coeffs_desc = list(alg.coeffs())
    coeffs_asc = list(reversed(coeffs_desc))
    coeffs_asc += [0] * (FIELD_DEGREE - len(coeffs_asc))
    return [rational_json(coeff) for coeff in coeffs_asc]


def coeff_vector_json(expr):
    return coeff_vector_json_cached(str(simplify(sympify(expr))))


@lru_cache(maxsize=None)
def field_expr_str_cached(expr_text):
    expr = sympify(expr_text)
    return str(to_number_field(expr, FIELD_GENERATOR_EXPR).as_expr())


def field_expr_str(expr):
    return field_expr_str_cached(str(simplify(sympify(expr))))


def row_json(row):
    return [field_expr_str(entry) for entry in row]


def row_power_basis_json(row):
    return [coeff_vector_json(entry) for entry in row]


def parse_expr_text(expr_text):
    return sympify(expr_text.replace("lambda", "lam"), locals={"lam": LAMBDA})


def sys_row_from_capacity_row(capacity_row, q_value, total_volume, volume_row):
    capacity = simplify(1 / (2 * q_value))
    sys_value = simplify(capacity**2 / (2 * total_volume))
    row = [
        simplify((capacity / total_volume) * dc - (sys_value / total_volume) * dv)
        for dc, dv in zip(capacity_row, volume_row, strict=True)
    ]
    return capacity, sys_value, row


def main() -> None:
    segment = json.loads(SEGMENT_PATH.read_text())
    volume = json.loads(VOLUME_PATH.read_text())

    coordinate_order = segment["coordinate_order"]
    if coordinate_order != volume["coordinate_order"]:
        raise ValueError("Segment and volume artifacts disagree on R^40 coordinate order")

    total_volume = parse_expr_text(volume["total_volume"])
    volume_row = [parse_expr_text(entry) for entry in volume["volume_derivative_row_flat"]]

    prototype_rows = {}
    for name in [
        "endpoint_family_left",
        "left_endpoint",
        "midpoint",
        "right_endpoint",
        "endpoint_family_right",
    ]:
        q_value = parse_expr_text(segment["prototype_rows"][name]["q_value_signed"])
        capacity_row = [
            parse_expr_text(entry) for entry in segment["prototype_rows"][name]["capacity_row_flat"]
        ]
        capacity, sys_value, sys_row = sys_row_from_capacity_row(
            capacity_row,
            q_value,
            total_volume,
            volume_row,
        )
        prototype_rows[name] = {
            "q_value_signed": field_expr_str(q_value),
            "capacity": str(simplify(capacity)),
            "sys_value": str(simplify(sys_value)),
            "sys_row_flat": row_json(sys_row),
            "sys_row_flat_power_basis": row_power_basis_json(sys_row),
        }

    symbolic_q = parse_expr_text(segment["prototype_rows"]["symbolic_segment_q_value"])
    symbolic_capacity = simplify(1 / (2 * symbolic_q))
    symbolic_sys_value = simplify(symbolic_capacity**2 / (2 * total_volume))
    symbolic_capacity_row = [
        parse_expr_text(entry) for entry in segment["prototype_rows"]["symbolic_capacity_row_flat"]
    ]
    symbolic_sys_row = [
        simplify((symbolic_capacity / total_volume) * dc - (symbolic_sys_value / total_volume) * dv)
        for dc, dv in zip(symbolic_capacity_row, volume_row, strict=True)
    ]

    max_degree = 0
    for entry in symbolic_sys_row:
        polynomial = expand(entry)
        if polynomial == 0:
            continue
        max_degree = max(max_degree, polynomial.as_poly(LAMBDA).degree())

    lagrange_left = parse_expr_text(segment["lagrange_coefficients"]["left"])
    lagrange_mid = parse_expr_text(segment["lagrange_coefficients"]["midpoint"])
    lagrange_right = parse_expr_text(segment["lagrange_coefficients"]["right"])

    left_row = [parse_expr_text(entry) for entry in prototype_rows["left_endpoint"]["sys_row_flat"]]
    midpoint_row = [parse_expr_text(entry) for entry in prototype_rows["midpoint"]["sys_row_flat"]]
    right_row = [parse_expr_text(entry) for entry in prototype_rows["right_endpoint"]["sys_row_flat"]]
    interpolation_residual = [
        simplify(
            symbolic_entry
            - (
                lagrange_left * left_entry
                + lagrange_mid * midpoint_entry
                + lagrange_right * right_entry
            )
        )
        for symbolic_entry, left_entry, midpoint_entry, right_entry in zip(
            symbolic_sys_row,
            left_row,
            midpoint_row,
            right_row,
            strict=True,
        )
    ]

    left_endpoint_residual = [
        simplify(parse_expr_text(left_entry) - parse_expr_text(family_entry))
        for left_entry, family_entry in zip(
            prototype_rows["left_endpoint"]["sys_row_flat"],
            prototype_rows["endpoint_family_left"]["sys_row_flat"],
            strict=True,
        )
    ]
    right_endpoint_residual = [
        simplify(parse_expr_text(right_entry) - parse_expr_text(family_entry))
        for right_entry, family_entry in zip(
            prototype_rows["right_endpoint"]["sys_row_flat"],
            prototype_rows["endpoint_family_right"]["sys_row_flat"],
            strict=True,
        )
    ]

    payload = {
        "field_generator": volume["field_generator"],
        "coordinate_order": coordinate_order,
        "total_volume": field_expr_str(total_volume),
        "common_volume_row_flat": volume["volume_derivative_row_flat"],
        "common_volume_row_flat_power_basis": volume["volume_derivative_row_flat_power_basis"],
        "lagrange_coefficients": segment["lagrange_coefficients"],
        "symbolic_segment_capacity": str(simplify(symbolic_capacity)),
        "symbolic_segment_sys_value": str(simplify(symbolic_sys_value)),
        "sys_row_polynomial_degree_max": max_degree,
        "prototype_rows": prototype_rows,
        "symbolic_segment_sys_row_flat": [str(simplify(entry)) for entry in symbolic_sys_row],
        "sys_row_interpolation_residual_by_coordinate": {
            label: field_expr_str(entry)
            for label, entry in zip(coordinate_order, interpolation_residual, strict=True)
        },
        "endpoint_sys_row_coincidence_residual_by_coordinate": {
            "left": {
                label: field_expr_str(entry)
                for label, entry in zip(coordinate_order, left_endpoint_residual, strict=True)
            },
            "right": {
                label: field_expr_str(entry)
                for label, entry in zip(coordinate_order, right_endpoint_residual, strict=True)
            },
        },
        "theorem_use": (
            "These are the exact prototype sys rows for the reduced Packet 3 checker. "
            "On the neighboring seven-facet segment, the exact sys row family is degree 2 "
            "in lambda and is recovered by the prototype rows at lambda = 0, 1/2, 1. "
            "The segment endpoints coincide exactly with the two six-facet endpoint-family rows."
        ),
    }
    OUTPUT_PATH.write_text(json.dumps(payload, indent=2) + "\n")


if __name__ == "__main__":
    main()
