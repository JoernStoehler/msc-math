#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = ["sympy"]
# ///

"""Derive the exact HKO volume-derivative row in dual coordinates.

Goal: certify the exact HKO2024 dual-vertex volume-derivative row used by the
      reduced Packet 3 checker surface.
Input Artifacts: None
Output Artifacts: experiments/hko-local-maximum/history/exact-witness/hko-volume-derivative.json
"""

from __future__ import annotations

import json
from pathlib import Path

from sympy import Matrix, Rational, simplify, sqrt, sympify
from sympy.polys.numberfields import to_number_field


EXPERIMENT_DIR = Path(__file__).resolve().parent
OUTPUT_PATH = EXPERIMENT_DIR / "hko-volume-derivative.json"

T_EXPR = sqrt(5 - 2 * sqrt(5))
T_ALG = to_number_field(T_EXPR)
MINPOLY = T_ALG.minpoly
FIELD_DEGREE = MINPOLY.degree()


def rational_json(value):
    value = sympify(value)
    num, den = value.as_numer_denom()
    return {"num": int(num), "den": int(den)}


def coeff_vector_json(expr):
    alg = to_number_field(sympify(expr), T_EXPR)
    coeffs_desc = list(alg.coeffs())
    coeffs_asc = list(reversed(coeffs_desc))
    coeffs_asc += [0] * (FIELD_DEGREE - len(coeffs_asc))
    return [rational_json(coeff) for coeff in coeffs_asc]


def vector_json(vec):
    return [coeff_vector_json(entry) for entry in vec]


def field_expr_str(expr):
    return str(to_number_field(simplify(sympify(expr)), T_EXPR).as_expr())


def polygon_vertices_from_duals(duals_2d):
    vertices = []
    facet_count = len(duals_2d)
    for facet in range(facet_count):
        next_facet = (facet + 1) % facet_count
        system = Matrix(
            [
                [duals_2d[facet][0], duals_2d[facet][1]],
                [duals_2d[next_facet][0], duals_2d[next_facet][1]],
            ]
        )
        vertices.append([simplify(entry) for entry in system.LUsolve(Matrix([1, 1]))])
    return vertices


def polygon_area(vertices):
    total = 0
    for index, vertex in enumerate(vertices):
        next_vertex = vertices[(index + 1) % len(vertices)]
        total += vertex[0] * next_vertex[1] - next_vertex[0] * vertex[1]
    return simplify(total / 2)


def squared_distance(lhs, rhs):
    return simplify(sum((lhs[idx] - rhs[idx]) ** 2 for idx in range(len(lhs))))


def main() -> None:
    t = T_EXPR
    sqrt5 = (5 - t**2) / 2
    alpha = (3 - sqrt5) / 2
    beta = t * (1 + sqrt5) / 2
    sec36 = sqrt5 - 1

    dual_vertices = [
        Matrix([1, t, 0, 0]),
        Matrix([-alpha, beta, 0, 0]),
        Matrix([-sec36, 0, 0, 0]),
        Matrix([-alpha, -beta, 0, 0]),
        Matrix([1, -t, 0, 0]),
        Matrix([0, 0, t, -1]),
        Matrix([0, 0, beta, alpha]),
        Matrix([0, 0, 0, sec36]),
        Matrix([0, 0, -beta, alpha]),
        Matrix([0, 0, -t, -1]),
    ]

    q_duals = [Matrix([dual[0], dual[1]]) for dual in dual_vertices[:5]]
    p_duals = [Matrix([dual[2], dual[3]]) for dual in dual_vertices[5:]]
    q_vertices = polygon_vertices_from_duals(q_duals)
    p_vertices = polygon_vertices_from_duals(p_duals)

    q_edge_length = simplify(sqrt(squared_distance(q_vertices[0], q_vertices[1])))
    p_edge_length = simplify(sqrt(squared_distance(p_vertices[0], p_vertices[1])))
    q_area = polygon_area(q_vertices)
    p_area = polygon_area(p_vertices)
    total_volume = simplify(q_area * p_area)
    facet_3_volume = simplify(q_edge_length * p_area)

    common_norm_sq = simplify(dual_vertices[0].dot(dual_vertices[0]))
    common_norm = simplify(sqrt(common_norm_sq))
    common_height = simplify(1 / common_norm)
    height_derivative_uniform = facet_3_volume
    scalar_multiple = simplify(facet_3_volume / common_norm**3)

    q_edge_length_formula = simplify(t * (1 + sqrt5) / 2)
    q_area_formula = simplify(5 * t * (3 + sqrt5) / 8)
    total_volume_formula = simplify(25 * (5 + sqrt5) / 32)
    facet_3_volume_formula = simplify(5 * sqrt5 / 4)
    common_norm_sq_formula = simplify(2 * (3 - sqrt5))
    common_height_formula = simplify((1 + sqrt5) / 4)
    scalar_multiple_formula = simplify(Rational(25, 32) + 5 * sqrt5 / 16)

    facet_centroids = []
    normal_line_points = []
    tangent_centroid_residuals = []
    point_on_hyperplane_residuals = []
    volume_derivative_by_facet = []

    for facet in range(10):
        if facet < 5:
            midpoint_2d = [
                simplify((q_vertices[(facet - 1) % 5][coord] + q_vertices[facet][coord]) / 2)
                for coord in range(2)
            ]
            centroid = Matrix([midpoint_2d[0], midpoint_2d[1], 0, 0])
        else:
            p_index = facet - 5
            midpoint_2d = [
                simplify((p_vertices[(p_index - 1) % 5][coord] + p_vertices[p_index][coord]) / 2)
                for coord in range(2)
            ]
            centroid = Matrix([0, 0, midpoint_2d[0], midpoint_2d[1]])

        dual = dual_vertices[facet]
        normal_line_point = simplify(dual / common_norm_sq)
        tangent_residual = [simplify(centroid[idx] - normal_line_point[idx]) for idx in range(4)]
        hyperplane_residual = simplify(dual.dot(centroid) - 1)
        derivative = [simplify(-scalar_multiple_formula * dual[idx]) for idx in range(4)]

        facet_centroids.append(centroid)
        normal_line_points.append(normal_line_point)
        tangent_centroid_residuals.append(tangent_residual)
        point_on_hyperplane_residuals.append(hyperplane_residual)
        volume_derivative_by_facet.append(derivative)

    flattened_row = [
        entry
        for derivative in volume_derivative_by_facet
        for entry in derivative
    ]
    coordinate_labels = [
        f"facet_{facet}_coord_{coord}"
        for facet in range(10)
        for coord in range(4)
    ]

    payload = {
        "field_generator": "t = sqrt(5 - 2*sqrt(5)) = tan(pi/5)",
        "field_minimal_polynomial": "x^4 - 10*x^2 + 5",
        "facet_order": list(range(10)),
        "coordinate_order": coordinate_labels,
        "q_polygon_vertices": [[str(entry) for entry in vertex] for vertex in q_vertices],
        "p_polygon_vertices": [[str(entry) for entry in vertex] for vertex in p_vertices],
        "q_edge_length": str(q_edge_length_formula),
        "p_edge_length": str(q_edge_length_formula),
        "q_area": str(q_area_formula),
        "p_area": str(q_area_formula),
        "total_volume": str(total_volume_formula),
        "facet_3_volume": str(facet_3_volume_formula),
        "common_dual_norm_squared": str(common_norm_sq_formula),
        "common_support_height": str(common_height_formula),
        "height_derivative_uniform": str(facet_3_volume_formula),
        "scalar_multiple_on_dual": str(scalar_multiple_formula),
        "formula_residuals": {
            "q_edge_length": field_expr_str(q_edge_length - q_edge_length_formula),
            "p_edge_length": field_expr_str(p_edge_length - q_edge_length_formula),
            "q_area": field_expr_str(q_area - q_area_formula),
            "p_area": field_expr_str(p_area - q_area_formula),
            "total_volume": field_expr_str(total_volume - total_volume_formula),
            "facet_3_volume": field_expr_str(facet_3_volume - facet_3_volume_formula),
            "common_dual_norm_squared": field_expr_str(common_norm_sq - common_norm_sq_formula),
            "common_support_height": field_expr_str(common_height - common_height_formula),
            "scalar_multiple_on_dual": field_expr_str(scalar_multiple - scalar_multiple_formula),
        },
        "facet_centroids": {
            str(facet): [field_expr_str(entry) for entry in facet_centroids[facet]]
            for facet in range(10)
        },
        "normal_line_points": {
            str(facet): [field_expr_str(entry) for entry in normal_line_points[facet]]
            for facet in range(10)
        },
        "tangent_centroid_residual_by_facet": {
            str(facet): [field_expr_str(entry) for entry in tangent_centroid_residuals[facet]]
            for facet in range(10)
        },
        "facet_hyperplane_residual_by_facet": {
            str(facet): field_expr_str(point_on_hyperplane_residuals[facet])
            for facet in range(10)
        },
        "volume_derivative_formula": (
            "For every HKO facet, the facet centroid equals a_k / |a_k|^2, so "
            "the tangential normal-tilt term vanishes and "
            "dvol/da_k = -(S / |a_k|^3) a_k."
        ),
        "volume_derivative_row_flat": [field_expr_str(entry) for entry in flattened_row],
        "volume_derivative_row_flat_power_basis": {
            label: coeff_vector_json(entry)
            for label, entry in zip(coordinate_labels, flattened_row, strict=True)
        },
        "volume_derivative_by_facet": {
            str(facet): [field_expr_str(entry) for entry in volume_derivative_by_facet[facet]]
            for facet in range(10)
        },
        "volume_derivative_by_facet_power_basis": {
            str(facet): vector_json(volume_derivative_by_facet[facet])
            for facet in range(10)
        },
        "theorem_use": (
            "This is the exact HKO volume row in the Packet 3 facet-major R^40 order. "
            "Together with exact capacity rows, it gives exact sys rows via "
            "sys = c^2 / (2 vol)."
        ),
    }
    OUTPUT_PATH.write_text(json.dumps(payload, indent=2) + "\n")


if __name__ == "__main__":
    main()
