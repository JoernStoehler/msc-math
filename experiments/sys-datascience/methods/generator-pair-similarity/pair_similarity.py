#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = ["numpy"]
# ///

"""Small, explicit polytope-pair similarity catalog and calibration runner.

This is intentionally a target-free comparison surface.  It accepts planar
``factor-shape-row-v1`` rows for the real smoke, while the 4D methods operate
on explicitly supplied point/facet arrays and are calibrated on synthetic
fixtures below.  No method silently converts one representation into another.
"""

from __future__ import annotations

import argparse
import csv
import hashlib
import itertools
import json
import math
import sys
import time
from dataclasses import dataclass
from pathlib import Path
from typing import Any, Callable, Iterable, Sequence

import numpy as np


SCHEMA = "generator-pair-similarity-report-v1"
EPS = 1e-12
MAX_EXACT_ASSIGNMENT = 8
SUPPORT_ANGLES = 128


@dataclass(frozen=True)
class MethodContract:
    name: str
    kind: str
    object: str
    quotient: str
    complexity: str
    knobs: str
    failure_boundary: str


CATALOG = (
    MethodContract("raw_vertex_l1_l2_linf", "metrics on ordered equal-length coordinate sequences", "planar CCW vertices", "none; source order and frame retained", "O(F)", "norm", "unequal F unavailable; deliberately order-sensitive"),
    MethodContract("canonical_cyclic_vertex_l2", "metric after deterministic canonical cyclic-start map", "planar CCW vertices", "cyclic starting index only; no translation/scale/rotation quotient", "O(F^2)", "lexicographic tolerance", "does not repair reversal or arbitrary facet relabeling"),
    MethodContract("exact_vertex_assignment_l2", "metric on unordered equal-cardinality vertex sets when exhaustive", "planar vertices", "vertex permutation only", "O(F! F), certified only F<=8", "max_facets", "above cap returns unavailable; greedy fallback is explicitly heuristic"),
    MethodContract("cyclic_dihedral_vertex_l2", "metrics after source-order-preserving cyclic or dihedral matching", "planar cyclic vertex/facet order", "C_F, or D_F when reflection is intentionally identified", "O(F^2)", "allow_reflection", "does not permit arbitrary facet matching"),
    MethodContract("permitted_permutation_assignment_l2", "metric on a caller-declared permutation quotient when the declared set is a group", "equal-cardinality vertices/facets", "only supplied incidence-automorphism or product-factor permutations", "O(|P| F)", "permitted permutations", "caller must justify closure and completeness of P; no default automorphism solver"),
    MethodContract("planar_support_rms_cyclic_grid", "numerical pseudometric on sampled normalized support functions", "strict convex planar polygons", "translation (area centroid), positive scale (area), and exactly C_G; D_G only when reflection is requested", "O(F G^2) for declared grid G", "support grid G, allow_reflection", "off-grid rotations are nonzero; approximate continuous rotation only by an explicit grid-convergence study"),
    MethodContract("variable_facet_sampled_support", "numerical pseudometric of convex bodies with unequal facet counts", "strict convex planar polygons", "translation, area scale, exactly C_G (or D_G with reflection)", "O(F G^2)", "support grid G, allow_reflection", "a sampled support/Hausdorff surrogate, not certified continuous Hausdorff distance"),
    MethodContract("euclidean_gram_and_procrustes", "metrics on fixed labeled nonconstant 4D configurations after stated normalization", "equal-F 4D point/facet configurations", "translation, positive Frobenius scale, O(4), but not relabeling", "O(F^2) / O(4^3)", "none", "facet order remains meaningful; Gram uses finite precision"),
    MethodContract("symplectic_gram_quotient", "metric only under validated analytic-center/volume-one spanning facet-covector contract and exhaustive permutation search", "equal-F 4D facet covectors", "linear symplectic maps and facet relabeling; normalization contract supplies translation/scale", "O(F! F^2), exact disposition F<=8", "max_facets", "float prototype is not an exact certificate; no answer above cap"),
    MethodContract("incidence_isomorphism_hamming", "metric on unlabeled incidence matrices when exhaustive", "facet-by-vertex 0/1 incidence", "facet and vertex permutations", "O(F! V log V), certified only F<=7", "max_facets", "not a geometric distance; above cap unavailable"),
    MethodContract("named_feature_l2", "metric on an explicitly fixed normalized feature vector; otherwise only a representation distance", "named scalar dictionaries", "only coordinates whose normalization is declared", "O(d)", "feature schema and scales", "does not cover variable-F geometry or prove feature completeness"),
    MethodContract("response_signature_l2", "heuristic dissimilarity", "planar polygon under fixed linear-map bank", "translation and area normalization inside each response; no rotation quotient", "O(B G F)", "named transform bank/support grid", "bank-dependent and not a privileged geometry definition"),
    MethodContract("product_factor_support_pair", "metric on an ordered pair of chosen factor representations when the factor distance is a metric", "two planar product factors", "factorwise chosen quotient; optional q/p swap only", "two factor comparisons", "factor distance, allow_factor_swap", "not unrestricted matching among all product facets"),
    MethodContract("normalized_vertex_cloud_hausdorff", "metric on canonically centered/RMS-normalized finite vertex clouds", "variable-cardinality vertices in a fixed Euclidean frame", "translation and positive RMS scale; no rotation or affine quotient", "O(nm d)", "normalization", "vertex-cloud Hausdorff is not polytope Hausdorff; loses face interiors"),
    MethodContract("symplectic_containment_gauge", "deferred; no implementation or numerical claim", "convex bodies", "would require an explicitly specified affine/linear symplectic containment optimization", "unmeasured", "optimization model and certificate", "not substituted by Gram, support, or feature distances"),
)


def as_points(values: Sequence[Sequence[float]], dimension: int | None = None) -> np.ndarray:
    points = np.asarray(values, dtype=float)
    if points.ndim != 2 or len(points) == 0 or not np.all(np.isfinite(points)):
        raise ValueError("expected a nonempty finite point matrix")
    if dimension is not None and points.shape[1] != dimension:
        raise ValueError(f"expected dimension {dimension}, got {points.shape[1]}")
    return points


def polygon_area(vertices: np.ndarray) -> float:
    return float(0.5 * np.sum(vertices[:, 0] * np.roll(vertices[:, 1], -1) - vertices[:, 1] * np.roll(vertices[:, 0], -1)))


def require_ccw_polygon(values: Sequence[Sequence[float]]) -> np.ndarray:
    vertices = as_points(values, 2)
    if len(vertices) < 3 or polygon_area(vertices) <= EPS:
        raise ValueError("requires at least three strict CCW polygon vertices")
    edges = np.roll(vertices, -1, axis=0) - vertices
    turns = edges[:, 0] * np.roll(edges[:, 1], -1) - edges[:, 1] * np.roll(edges[:, 0], -1)
    if np.any(turns <= EPS):
        raise ValueError("requires strict convex CCW cyclic vertices")
    return vertices


def raw_coordinate_distances(left: Sequence[Sequence[float]], right: Sequence[Sequence[float]]) -> dict[str, float]:
    a, b = as_points(left), as_points(right)
    if a.shape != b.shape:
        raise ValueError("raw coordinate metrics require equal shape and source order")
    delta = (a - b).reshape(-1)
    return {"l1": float(np.sum(np.abs(delta))), "l2": float(np.linalg.norm(delta)), "linf": float(np.max(np.abs(delta)))}


def canonical_cyclic(vertices: Sequence[Sequence[float]]) -> np.ndarray:
    """Choose the lexicographically least cyclic rotation, preserving orientation."""
    points = as_points(vertices, 2)
    candidates = [np.roll(points, -shift, axis=0) for shift in range(len(points))]
    return min(candidates, key=lambda item: tuple(item.reshape(-1).tolist())).copy()


def canonical_cyclic_l2(left: Sequence[Sequence[float]], right: Sequence[Sequence[float]]) -> float:
    return raw_coordinate_distances(canonical_cyclic(left), canonical_cyclic(right))["l2"]


def cyclic_dihedral_vertex_l2(left: Sequence[Sequence[float]], right: Sequence[Sequence[float]], *, allow_reflection: bool = False) -> float:
    """Order-respecting matching; its dihedral mode is an explicit reflection quotient."""
    a, b = as_points(left, 2), as_points(right, 2)
    if a.shape != b.shape:
        raise ValueError("cyclic/dihedral matching requires equal planar vertex counts")
    candidates = [np.roll(b, shift, axis=0) for shift in range(len(b))]
    if allow_reflection:
        candidates.extend(np.roll(b[::-1], shift, axis=0) for shift in range(len(b)))
    return min(float(np.linalg.norm(a - candidate)) for candidate in candidates)


def exact_assignment_l2(left: Sequence[Sequence[float]], right: Sequence[Sequence[float]], max_facets: int = MAX_EXACT_ASSIGNMENT) -> float | None:
    a, b = as_points(left), as_points(right)
    if len(a) != len(b):
        return None
    if len(a) > max_facets:
        return None
    return min(float(np.linalg.norm(a - b[list(permutation)])) for permutation in itertools.permutations(range(len(a))))


def greedy_assignment_l2_upper_bound(left: Sequence[Sequence[float]], right: Sequence[Sequence[float]]) -> float:
    """A deterministic upper bound, deliberately not called an optimal distance."""
    a, b = as_points(left), as_points(right)
    if len(a) != len(b):
        raise ValueError("assignment requires equal cardinality")
    remaining = set(range(len(b)))
    chosen = []
    for point in a:
        index = min(remaining, key=lambda item: (float(np.linalg.norm(point - b[item])), item))
        chosen.append(index)
        remaining.remove(index)
    return float(np.linalg.norm(a - b[chosen]))


def permitted_permutation_assignment_l2(left: Sequence[Sequence[float]], right: Sequence[Sequence[float]], permitted: Iterable[Sequence[int]]) -> float:
    """Compare only a caller-supplied matching family, never all permutations by default."""
    a, b = as_points(left), as_points(right)
    if a.shape != b.shape:
        raise ValueError("permitted matching requires equal shape")
    candidates = []
    for permutation in permitted:
        indices = tuple(permutation)
        if sorted(indices) != list(range(len(a))):
            raise ValueError("each permitted matching must be a permutation of the indices")
        candidates.append(float(np.linalg.norm(a - b[list(indices)])))
    if not candidates:
        raise ValueError("permitted matching family is empty")
    return min(candidates)


def centered_area_one(vertices: Sequence[Sequence[float]]) -> np.ndarray:
    points = require_ccw_polygon(vertices)
    area = polygon_area(points)
    # Polygon centroid, not a hidden Fourier-mode quotient.
    cross = points[:, 0] * np.roll(points[:, 1], -1) - np.roll(points[:, 0], -1) * points[:, 1]
    center = np.sum((points + np.roll(points, -1, axis=0)) * cross[:, None], axis=0) / (6.0 * area)
    return (points - center) / math.sqrt(area)


def support(vertices: np.ndarray, angles: np.ndarray) -> np.ndarray:
    directions = np.column_stack((np.cos(angles), np.sin(angles)))
    return np.max(vertices @ directions.T, axis=0)


def planar_support_distance(left: Sequence[Sequence[float]], right: Sequence[Sequence[float]], *, grid: int = SUPPORT_ANGLES, allow_reflection: bool = False) -> dict[str, float | bool | int | str]:
    """Sampled-support pseudometric modulo exactly C_G or D_G, not a continuous quotient."""
    if grid < 16:
        raise ValueError("support grid must be at least 16")
    a, b = centered_area_one(left), centered_area_one(right)
    angles = 2.0 * math.pi * np.arange(grid) / grid
    ha = support(a, angles)

    def compare(candidate: np.ndarray) -> tuple[float, float]:
        values = []
        for rotation in range(grid):
            hb = support(candidate, angles - rotation * 2.0 * math.pi / grid)
            delta = ha - hb
            values.append((float(math.sqrt(np.mean(delta * delta))), float(np.max(np.abs(delta)))))
        return min(values)

    candidates = [(False, b)]
    if allow_reflection:
        # Reverse after x reflection to recover CCW order for later callers.
        candidates.append((True, (b * np.array([-1.0, 1.0]))[::-1]))
    reflected, (rms, linf) = min(((flag, compare(candidate)) for flag, candidate in candidates), key=lambda item: item[1])
    return {"support_rms": rms, "support_linf": linf, "reflection_used": reflected, "rotation_group": f"D_{grid}" if allow_reflection else f"C_{grid}", "support_grid": grid}


def normalized_configuration(values: Sequence[Sequence[float]]) -> np.ndarray:
    points = as_points(values, 4)
    centered = points - np.mean(points, axis=0)
    norm = float(np.linalg.norm(centered))
    if norm <= EPS:
        raise ValueError("configuration is constant after translation removal")
    return centered / norm


def euclidean_gram_distance(left: Sequence[Sequence[float]], right: Sequence[Sequence[float]]) -> float:
    a, b = normalized_configuration(left), normalized_configuration(right)
    if a.shape != b.shape:
        raise ValueError("fixed-F Euclidean Gram distance requires equal shape/order")
    return float(np.linalg.norm(a @ a.T - b @ b.T))


def procrustes_distance(left: Sequence[Sequence[float]], right: Sequence[Sequence[float]]) -> float:
    a, b = normalized_configuration(left), normalized_configuration(right)
    if a.shape != b.shape:
        raise ValueError("fixed-F Procrustes distance requires equal shape/order")
    _, singular_values, _ = np.linalg.svd(a.T @ b, full_matrices=False)
    return float(math.sqrt(max(0.0, 2.0 - 2.0 * float(np.sum(singular_values)))))


def symplectic_gram(facets: Sequence[Sequence[float]]) -> np.ndarray:
    a = as_points(facets, 4)
    if float(np.linalg.norm(a)) <= EPS:
        raise ValueError("zero facet configuration")
    # Scale is not estimated here: callers must supply the declared
    # analytic-center/volume-one normalization.  Frobenius-normalizing rows
    # would destroy genuine symplectic-map invariance.
    j = np.array(((0.0, 0.0, 1.0, 0.0), (0.0, 0.0, 0.0, 1.0), (-1.0, 0.0, 0.0, 0.0), (0.0, -1.0, 0.0, 0.0)))
    return a @ j @ a.T


def symplectic_gram_quotient(left: Sequence[Sequence[float]], right: Sequence[Sequence[float]], max_facets: int = MAX_EXACT_ASSIGNMENT) -> float | None:
    a, b = symplectic_gram(left), symplectic_gram(right)
    if a.shape != b.shape:
        return None
    if len(a) > max_facets:
        return None
    return min(float(np.linalg.norm(a - b[np.ix_(permutation, permutation)])) / len(a) for permutation in itertools.permutations(range(len(a))))


def incidence_isomorphism_hamming(left: Sequence[Sequence[int]], right: Sequence[Sequence[int]], max_facets: int = 7) -> float | None:
    a = np.asarray(left, dtype=int)
    b = np.asarray(right, dtype=int)
    if a.ndim != 2 or b.ndim != 2 or a.shape != b.shape or not np.all((a == 0) | (a == 1)) or not np.all((b == 0) | (b == 1)):
        return None
    if a.shape[0] > max_facets:
        return None
    best = math.inf
    for permutation in itertools.permutations(range(a.shape[0])):
        candidate = b[list(permutation)]
        # Vertex labels are immaterial: canonicalize the multiset of incidence columns.
        candidate = candidate[:, np.lexsort(candidate[::-1])]
        reference = a[:, np.lexsort(a[::-1])]
        best = min(best, float(np.mean(reference != candidate)))
    return best


def named_feature_l2(left: dict[str, float], right: dict[str, float], scales: dict[str, float]) -> float:
    if set(left) != set(right) or set(left) != set(scales):
        raise ValueError("features and named normalization scales must have identical keys")
    if any(not math.isfinite(value) for value in [*left.values(), *right.values(), *scales.values()]) or any(value <= 0 for value in scales.values()):
        raise ValueError("features must be finite and scales positive")
    return math.sqrt(sum(((left[name] - right[name]) / scales[name]) ** 2 for name in sorted(scales)))


RESPONSE_BANK = (
    ("identity", np.eye(2)),
    ("stretch_q", np.diag((1.35, 1.0 / 1.35))),
    ("shear", np.array(((1.0, 0.35), (0.0, 1.0)))),
)


def response_signature(vertices: Sequence[Sequence[float]], grid: int = 64) -> dict[str, float]:
    polygon = require_ccw_polygon(vertices)
    values: dict[str, float] = {}
    angles = 2.0 * math.pi * np.arange(grid) / grid
    for name, transform in RESPONSE_BANK:
        standardized = centered_area_one(polygon @ transform.T)
        h = support(standardized, angles)
        values[f"{name}:support_cv"] = float(np.std(h) / np.mean(h))
        values[f"{name}:isoperimetric"] = float(4.0 * math.pi / np.sum(np.linalg.norm(np.roll(standardized, -1, axis=0) - standardized, axis=1)) ** 2)
    return values


def response_signature_l2(left: Sequence[Sequence[float]], right: Sequence[Sequence[float]]) -> float:
    a, b = response_signature(left), response_signature(right)
    return named_feature_l2(a, b, {name: 1.0 for name in a})


def product_factor_support_pair(
    left: tuple[Sequence[Sequence[float]], Sequence[Sequence[float]]],
    right: tuple[Sequence[Sequence[float]], Sequence[Sequence[float]]],
    *,
    allow_factor_swap: bool = False,
    support_grid: int = SUPPORT_ANGLES,
) -> float:
    """Factorwise direct geometry comparison; no arbitrary matching across factors."""
    def combine(a: tuple[Sequence[Sequence[float]], Sequence[Sequence[float]]], b: tuple[Sequence[Sequence[float]], Sequence[Sequence[float]]]) -> float:
        first = float(planar_support_distance(a[0], b[0], grid=support_grid)["support_rms"])
        second = float(planar_support_distance(a[1], b[1], grid=support_grid)["support_rms"])
        return math.hypot(first, second)
    values = [combine(left, right)]
    if allow_factor_swap:
        values.append(combine(left, (right[1], right[0])))
    return min(values)


def normalized_vertex_cloud(values: Sequence[Sequence[float]]) -> np.ndarray:
    points = as_points(values)
    centered = points - np.mean(points, axis=0)
    rms = math.sqrt(float(np.mean(np.sum(centered * centered, axis=1))))
    if rms <= EPS:
        raise ValueError("vertex cloud has zero RMS radius")
    return centered / rms


def normalized_vertex_cloud_hausdorff(left: Sequence[Sequence[float]], right: Sequence[Sequence[float]]) -> float:
    a, b = normalized_vertex_cloud(left), normalized_vertex_cloud(right)
    if a.shape[1] != b.shape[1]:
        raise ValueError("vertex clouds must share a coordinate dimension")
    distances = np.linalg.norm(a[:, None, :] - b[None, :, :], axis=2)
    return float(max(np.max(np.min(distances, axis=1)), np.max(np.min(distances, axis=0))))


def regular_polygon(count: int, rotation: float = 0.0) -> np.ndarray:
    angles = rotation + 2.0 * math.pi * np.arange(count) / count
    return np.column_stack((np.cos(angles), np.sin(angles)))


def synthetic_planar_cases() -> dict[str, np.ndarray]:
    base = regular_polygon(5, 0.12) @ np.array(((1.17, 0.11), (0.0, 0.81))).T
    rotation = 13 * 2.0 * math.pi / SUPPORT_ANGLES
    transformed = 2.7 * base @ np.array(((math.cos(rotation), -math.sin(rotation)), (math.sin(rotation), math.cos(rotation)))).T + np.array((3.0, -2.0))
    off_grid = base @ np.array(((math.cos(0.01), -math.sin(0.01)), (math.sin(0.01), math.cos(0.01)))).T
    permuted = np.roll(base, 2, axis=0)
    deformed = base @ np.array(((1.45, 0.18), (0.0, 0.75))).T
    reflected = (base * np.array((-1.0, 1.0)))[::-1]
    return {"base": base, "translated_scaled_grid_rotated": transformed, "off_grid_rotated": off_grid, "cyclic_reordered": permuted, "deformed": deformed, "reflected": reflected}


def synthetic_4d_cases() -> dict[str, np.ndarray]:
    base = np.array(((1, 0, 0, 0), (0, 1, 0, 0), (0, 0, 1, 0), (0, 0, 0, 1), (-1, -1, -1, -1), (1, -1, 1, -1)), dtype=float)
    symplectic = np.diag((2.0, 0.5, 0.5, 2.0))
    # Swap q1 and q2 while leaving the p coordinates fixed: orthogonal, but
    # it does not preserve dq1^dp1 + dq2^dp2.
    orthogonal_not_symplectic = np.array(((0, 1, 0, 0), (1, 0, 0, 0), (0, 0, 1, 0), (0, 0, 0, 1)), dtype=float)
    return {"base": base, "facet_permuted": base[[2, 5, 0, 4, 1, 3]], "symplectic": base @ symplectic.T, "orthogonal_not_symplectic": base @ orthogonal_not_symplectic.T, "deformed": base @ np.diag((1.3, 1.0, 1.0, 1.0)).T}


def triangle_check(distance: Callable[[np.ndarray, np.ndarray], float], cases: dict[str, np.ndarray], tolerance: float = 1e-8) -> dict[str, Any]:
    violations = []
    for left, middle, right in itertools.product(cases, repeat=3):
        d_lr = distance(cases[left], cases[right])
        if d_lr > distance(cases[left], cases[middle]) + distance(cases[middle], cases[right]) + tolerance:
            violations.append([left, middle, right])
    return {"triple_count": len(cases) ** 3, "passed": not violations, "violations": violations}


def calibration() -> dict[str, Any]:
    p, q = synthetic_planar_cases(), synthetic_4d_cases()
    reflection_off = planar_support_distance(p["base"], p["reflected"], allow_reflection=False)["support_rms"]
    reflection_on = planar_support_distance(p["base"], p["reflected"], allow_reflection=True)["support_rms"]
    symp_perm = symplectic_gram_quotient(q["base"], q["facet_permuted"])
    symp_map = symplectic_gram_quotient(q["base"], q["symplectic"])
    symp_outside = symplectic_gram_quotient(q["base"], q["orthogonal_not_symplectic"])
    raw_triangle_cases = {name: value for name, value in p.items() if name != "translated_scaled_grid_rotated"}
    product_base = (p["base"], p["deformed"])
    product_swapped = (p["deformed"], p["base"])
    matrix = [
        ("planar_identity", "raw_l2", raw_coordinate_distances(p["base"], p["base"])["l2"], "zero"),
        ("cyclic_source_reorder", "raw_l2", raw_coordinate_distances(p["base"], p["cyclic_reordered"])["l2"], "positive"),
        ("cyclic_source_reorder", "cyclic_l2", cyclic_dihedral_vertex_l2(p["base"], p["cyclic_reordered"]), "zero"),
        ("cyclic_source_reorder", "unrestricted_assignment", float(exact_assignment_l2(p["base"], p["cyclic_reordered"])), "zero"),
        ("translation_scale_C_G_rotation", "planar_support_C_G", float(planar_support_distance(p["base"], p["translated_scaled_grid_rotated"])["support_rms"]), "zero"),
        ("off_grid_rotation", "planar_support_C_G", float(planar_support_distance(p["base"], p["off_grid_rotated"])["support_rms"]), "positive"),
        ("reflection", "planar_support_C_G", float(reflection_off), "positive"),
        ("reflection", "planar_support_D_G_requested", float(reflection_on), "zero"),
        ("symplectic_map", "euclidean_gram", euclidean_gram_distance(q["base"], q["symplectic"]), "positive"),
        ("symplectic_map", "symplectic_gram", float(symp_map), "zero"),
        ("orthogonal_non_symplectic_map", "euclidean_gram", euclidean_gram_distance(q["base"], q["orthogonal_not_symplectic"]), "zero"),
        ("orthogonal_non_symplectic_map", "symplectic_gram", float(symp_outside), "positive"),
        ("facet_permutation", "fixed_label_euclidean_gram", euclidean_gram_distance(q["base"], q["facet_permuted"]), "positive"),
        ("facet_permutation", "symplectic_gram_permutation", float(symp_perm), "zero"),
        ("variable_facet_4_vs_5", "sampled_support", float(planar_support_distance(regular_polygon(4), regular_polygon(5))["support_rms"]), "positive"),
        ("variable_facet_4_vs_5", "unrestricted_assignment", exact_assignment_l2(regular_polygon(4), regular_polygon(5)), "unavailable"),
        ("product_factor_swap", "ordered_factor_pair", product_factor_support_pair(product_base, product_swapped), "positive"),
        ("product_factor_swap", "swap_enabled_factor_pair", product_factor_support_pair(product_base, product_swapped, allow_factor_swap=True), "zero"),
    ]
    equivalence_matrix = []
    for relation, method, value, expectation in matrix:
        passed = value is None if expectation == "unavailable" else (value is not None and (abs(value) < 1e-8 if expectation == "zero" else value > 1e-4))
        equivalence_matrix.append({"relation": relation, "method": method, "expected": expectation, "observed": value, "passed": passed})
    return {
        "identity_and_invariance": {
            "raw_identity": raw_coordinate_distances(p["base"], p["base"]),
            "canonical_removes_cyclic_start": canonical_cyclic_l2(p["base"], p["cyclic_reordered"]),
            "exact_assignment_removes_cyclic_start": exact_assignment_l2(p["base"], p["cyclic_reordered"]),
            "planar_translation_scale_C_G_rotation": planar_support_distance(p["base"], p["translated_scaled_grid_rotated"])["support_rms"],
            "off_grid_rotation_C_G_residual": planar_support_distance(p["base"], p["off_grid_rotated"])["support_rms"],
            "reflection_separate_in_C_G": reflection_off,
            "reflection_quotiented_in_D_G_when_requested": reflection_on,
            "euclidean_gram_orthogonal": euclidean_gram_distance(q["base"], q["orthogonal_not_symplectic"]),
            "symplectic_gram_facet_permutation": symp_perm,
            "symplectic_gram_symplectic_map": symp_map,
            "symplectic_gram_orthogonal_non_symplectic": symp_outside,
        },
        "separation": {
            "planar_deformation": planar_support_distance(p["base"], p["deformed"])["support_rms"],
            "response_deformation": response_signature_l2(p["base"], p["deformed"]),
            "variable_facet_cloud": normalized_vertex_cloud_hausdorff(regular_polygon(4), regular_polygon(5)),
        },
        "symmetry": {
            "raw_l1": triangle_check(lambda a, b: raw_coordinate_distances(a, b)["l1"], raw_triangle_cases),
            "raw_l2": triangle_check(lambda a, b: raw_coordinate_distances(a, b)["l2"], raw_triangle_cases),
            "raw_linf": triangle_check(lambda a, b: raw_coordinate_distances(a, b)["linf"], raw_triangle_cases),
            "canonical_l2": triangle_check(canonical_cyclic_l2, raw_triangle_cases),
            "assignment_l2": triangle_check(lambda a, b: float(exact_assignment_l2(a, b)), raw_triangle_cases),
            "planar_support_rms": triangle_check(lambda a, b: float(planar_support_distance(a, b)["support_rms"]), raw_triangle_cases, 2e-6),
            "euclidean_gram": triangle_check(euclidean_gram_distance, q),
            "procrustes": triangle_check(procrustes_distance, q),
            "symplectic_gram": triangle_check(lambda a, b: float(symplectic_gram_quotient(a, b)), q, 2e-8),
            "variable_facet_vertex_cloud_hausdorff": triangle_check(normalized_vertex_cloud_hausdorff, raw_triangle_cases),
        },
        "equivalence_regression_matrix": equivalence_matrix,
        "claimed_controls_pass": bool(reflection_off > 1e-3 and reflection_on < 1e-9 and planar_support_distance(p["base"], p["off_grid_rotated"])["support_rms"] > 1e-4 and symp_perm is not None and symp_perm < 1e-12 and symp_map is not None and symp_map < 1e-12 and symp_outside is not None and symp_outside > 1e-3 and all(row["passed"] for row in equivalence_matrix)),
    }


def load_factor_shapes(path: Path) -> list[dict[str, Any]]:
    rows = []
    with path.open() as handle:
        for number, line in enumerate(handle, 1):
            if not line.strip():
                continue
            row = json.loads(line)
            if row.get("schema") != "factor-shape-row-v1" or not isinstance(row.get("population"), str) or not row.get("population"):
                raise ValueError(f"{path}:{number}: expected complete factor-shape-row-v1")
            require_ccw_polygon(row.get("vertices_ccw"))
            rows.append(row)
    if not rows:
        raise ValueError(f"{path}: no factor-shape-row-v1 records")
    return rows


def stable_select(rows: Iterable[dict[str, Any]], per_population: int) -> list[dict[str, Any]]:
    buckets: dict[str, list[dict[str, Any]]] = {}
    for row in rows:
        buckets.setdefault(row["population"], []).append(row)
    chosen = []
    for population in sorted(buckets):
        ranked = sorted(buckets[population], key=lambda row: (hashlib.sha256(("generator-pair-similarity-smoke-v1/" + row["sample_id"]).encode()).digest(), row["sample_id"]))
        chosen.extend(ranked[:per_population])
    return chosen


def smoke_report(rows: list[dict[str, Any]], per_population: int) -> dict[str, Any]:
    chosen = stable_select(rows, per_population)
    examples = []
    methods: dict[str, list[float]] = {"raw_l2": [], "canonical_l2": [], "assignment_l2": [], "support_rms": [], "support_rms_reflection": [], "response_l2": []}
    for left, right in itertools.combinations(chosen, 2):
        a, b = left["vertices_ccw"], right["vertices_ccw"]
        if len(a) != len(b):
            continue
        values = {
            "raw_l2": raw_coordinate_distances(a, b)["l2"],
            "canonical_l2": canonical_cyclic_l2(a, b),
            "assignment_l2": exact_assignment_l2(a, b),
            "support_rms": planar_support_distance(a, b)["support_rms"],
            "support_rms_reflection": planar_support_distance(a, b, allow_reflection=True)["support_rms"],
            "response_l2": response_signature_l2(a, b),
        }
        for name, value in values.items():
            if value is not None:
                methods[name].append(float(value))
        if len(examples) < 18:
            examples.append({"left": left["sample_id"], "right": right["sample_id"], "left_population": left["population"], "right_population": right["population"], "side_count": len(a), **values})
    summary = {name: {"pair_count": len(values), "mean": float(np.mean(values)) if values else None, "median": float(np.median(values)) if values else None} for name, values in methods.items()}
    return {"schema": SCHEMA, "input_row_count": len(rows), "selected_row_count": len(chosen), "selection": "lowest SHA-256(sample_id) per population; descriptive deterministic smoke only", "population_counts": {population: sum(row["population"] == population for row in chosen) for population in sorted({row["population"] for row in chosen})}, "pair_examples": examples, "comparison_table": summary, "excluded_unequal_side_count_pairs": sum(1 for a, b in itertools.combinations(chosen, 2) if len(a["vertices_ccw"]) != len(b["vertices_ccw"]))}


def write_outputs(report: dict[str, Any], output: Path) -> None:
    output.mkdir(parents=True, exist_ok=True)
    (output / "report.json").write_text(json.dumps(report, indent=2, sort_keys=True) + "\n")
    rows = []
    for name, values in report["smoke"]["comparison_table"].items():
        rows.append({"method": name, **values})
    with (output / "comparison.tsv").open("w", newline="") as handle:
        writer = csv.DictWriter(handle, fieldnames=["method", "pair_count", "mean", "median"], delimiter="\t", lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1 << 20), b""):
            digest.update(chunk)
    return digest.hexdigest()


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--input", type=Path, required=True, help="existing factor-shape-row-v1 JSONL")
    parser.add_argument("--out-dir", type=Path, required=True)
    parser.add_argument("--per-population", type=int, default=2)
    args = parser.parse_args()
    if args.per_population <= 0:
        raise SystemExit("--per-population must be positive")
    started = time.perf_counter()
    rows = load_factor_shapes(args.input)
    report = {"schema": SCHEMA, "catalog": [contract.__dict__ for contract in CATALOG], "calibration": calibration(), "smoke": smoke_report(rows, args.per_population), "run": {"input": str(args.input), "input_sha256": sha256_file(args.input), "producer_sha256": sha256_file(Path(__file__)), "per_population": args.per_population, "target_free": True}}
    write_outputs(report, args.out_dir)
    print(
        f"volatile wall_seconds={time.perf_counter() - started:.6f}; "
        "not retained in scientific artifacts",
        file=sys.stderr,
    )


if __name__ == "__main__":
    main()
