#!/usr/bin/env python3
"""Exact bounded prototype for the symplectic-Gram quotient distance.

The implementation deliberately uses only ``fractions.Fraction``.  It accepts
already normalized exact configurations, and it can derive the declared
analytic-center/volume-one normalization for exact parallelotope
presentations.  Permutation optimization is exhaustive and fails closed above
``MAX_FACETS`` or on timeout.
"""

from __future__ import annotations

import argparse
import hashlib
import itertools
import json
import math
import subprocess
import time
from dataclasses import dataclass
from fractions import Fraction
from pathlib import Path
from typing import Iterable, Sequence

DIMENSION = 4
MAX_FACETS = 8
COORDINATE_ORDER = "q1,q2,p1,p2"
SCHEMA = "generator-target-quotient-distance-smoke-v1"

Scalar = Fraction
Vector = tuple[Scalar, Scalar, Scalar, Scalar]
Matrix = tuple[Vector, Vector, Vector, Vector]
Gram = tuple[tuple[Scalar, ...], ...]


class ContractError(ValueError):
    """An exact input or bounded-search contract was violated."""


def q(value: int | str | Fraction) -> Fraction:
    return value if isinstance(value, Fraction) else Fraction(value)


def vector(values: Sequence[int | str | Fraction]) -> Vector:
    if len(values) != DIMENSION:
        raise ContractError("vectors must have four coordinates")
    return tuple(q(value) for value in values)  # type: ignore[return-value]


def dot(left: Vector, right: Vector) -> Fraction:
    return sum((a * b for a, b in zip(left, right)), Fraction())


def add(left: Vector, right: Vector) -> Vector:
    return tuple(a + b for a, b in zip(left, right))  # type: ignore[return-value]


def scale(scalar: Fraction, value: Vector) -> Vector:
    return tuple(scalar * entry for entry in value)  # type: ignore[return-value]


def omega(left: Vector, right: Vector) -> Fraction:
    return (
        left[0] * right[2]
        + left[1] * right[3]
        - left[2] * right[0]
        - left[3] * right[1]
    )


def matrix(rows: Sequence[Sequence[int | str | Fraction]]) -> Matrix:
    if len(rows) != DIMENSION:
        raise ContractError("matrices must have four rows")
    return tuple(vector(row) for row in rows)  # type: ignore[return-value]


IDENTITY = matrix(
    ((1, 0, 0, 0), (0, 1, 0, 0), (0, 0, 1, 0), (0, 0, 0, 1))
)
J_MATRIX = matrix(
    ((0, 0, 1, 0), (0, 0, 0, 1), (-1, 0, 0, 0), (0, -1, 0, 0))
)


def transpose(value: Matrix) -> Matrix:
    return tuple(tuple(value[row][col] for row in range(4)) for col in range(4))  # type: ignore[return-value]


def matmul(left: Matrix, right: Matrix) -> Matrix:
    right_t = transpose(right)
    return tuple(
        tuple(dot(left_row, right_col) for right_col in right_t)
        for left_row in left
    )  # type: ignore[return-value]


def matvec(left: Matrix, right: Vector) -> Vector:
    return tuple(dot(row, right) for row in left)  # type: ignore[return-value]


def determinant(value: Matrix) -> Fraction:
    work = [list(row) for row in value]
    result = Fraction(1)
    for column in range(4):
        pivot = next((row for row in range(column, 4) if work[row][column]), None)
        if pivot is None:
            return Fraction()
        if pivot != column:
            work[pivot], work[column] = work[column], work[pivot]
            result = -result
        pivot_value = work[column][column]
        result *= pivot_value
        for row in range(column + 1, 4):
            ratio = work[row][column] / pivot_value
            for entry in range(column + 1, 4):
                work[row][entry] -= ratio * work[column][entry]
    return result


def inverse(value: Matrix) -> Matrix:
    work = [list(row) + list(identity_row) for row, identity_row in zip(value, IDENTITY)]
    for column in range(4):
        pivot = next((row for row in range(column, 4) if work[row][column]), None)
        if pivot is None:
            raise ContractError("matrix is singular")
        work[pivot], work[column] = work[column], work[pivot]
        pivot_value = work[column][column]
        work[column] = [entry / pivot_value for entry in work[column]]
        for row in range(4):
            if row == column:
                continue
            ratio = work[row][column]
            work[row] = [
                entry - ratio * pivot_entry
                for entry, pivot_entry in zip(work[row], work[column])
            ]
    return tuple(tuple(row[4:]) for row in work)  # type: ignore[return-value]


def rank(rows: Sequence[Vector]) -> int:
    work = [list(row) for row in rows]
    rank_value = 0
    for column in range(4):
        pivot = next(
            (row for row in range(rank_value, len(work)) if work[row][column]),
            None,
        )
        if pivot is None:
            continue
        work[pivot], work[rank_value] = work[rank_value], work[pivot]
        pivot_value = work[rank_value][column]
        for row in range(rank_value + 1, len(work)):
            ratio = work[row][column] / pivot_value
            for entry in range(column, 4):
                work[row][entry] -= ratio * work[rank_value][entry]
        rank_value += 1
    return rank_value


def solve4(rows: Sequence[Vector], rhs: Sequence[Fraction]) -> Vector | None:
    if len(rows) != 4 or len(rhs) != 4:
        raise ContractError("solve4 expects four equations")
    work = [list(row) + [value] for row, value in zip(rows, rhs)]
    for column in range(4):
        pivot = next((row for row in range(column, 4) if work[row][column]), None)
        if pivot is None:
            return None
        work[pivot], work[column] = work[column], work[pivot]
        pivot_value = work[column][column]
        work[column] = [entry / pivot_value for entry in work[column]]
        for row in range(4):
            if row == column:
                continue
            ratio = work[row][column]
            work[row] = [
                entry - ratio * pivot_entry
                for entry, pivot_entry in zip(work[row], work[column])
            ]
    return tuple(row[4] for row in work)  # type: ignore[return-value]


def is_symplectic(value: Matrix) -> bool:
    return matmul(matmul(transpose(value), J_MATRIX), value) == J_MATRIX


def is_orthogonal(value: Matrix) -> bool:
    return matmul(transpose(value), value) == IDENTITY


def exact_fourth_root(value: Fraction) -> Fraction:
    if value <= 0:
        raise ContractError("volume must be positive")
    numerator = math.isqrt(value.numerator)
    numerator = math.isqrt(numerator) if numerator * numerator == value.numerator else -1
    denominator = math.isqrt(value.denominator)
    denominator = (
        math.isqrt(denominator) if denominator * denominator == value.denominator else -1
    )
    if (
        numerator < 0
        or denominator <= 0
        or numerator**4 != value.numerator
        or denominator**4 != value.denominator
    ):
        raise ContractError("exact volume is not a rational fourth power")
    return Fraction(numerator, denominator)


def enumerate_vertices(duals: Sequence[Vector]) -> tuple[Vector, ...]:
    """Enumerate vertices of ``{x: a_i.x <= 1}`` by exact 4-tuples."""

    vertices: set[Vector] = set()
    one = Fraction(1)
    for indices in itertools.combinations(range(len(duals)), 4):
        candidate = solve4([duals[index] for index in indices], [one] * 4)
        if candidate is not None and all(dot(row, candidate) <= one for row in duals):
            vertices.add(candidate)
    return tuple(sorted(vertices))


def _has_closed_hemisphere_separator(rows: Sequence[Vector]) -> bool:
    """Decide whether all rows lie in a closed hemisphere.

    Rank four makes each affine feasibility problem pointed.  A nonzero
    separator can be scaled so one coordinate is +/-1; a nonempty pointed
    polyhedron has a vertex, found by three active row constraints.
    """

    for coordinate in range(4):
        axis = vector([1 if index == coordinate else 0 for index in range(4)])
        for sign in (Fraction(1), Fraction(-1)):
            for indices in itertools.combinations(range(len(rows)), 3):
                candidate = solve4(
                    [axis] + [rows[index] for index in indices],
                    [sign, Fraction(), Fraction(), Fraction()],
                )
                if candidate is not None and all(dot(row, candidate) >= 0 for row in rows):
                    return True
    return False


def validate_normalized_configuration(duals: Sequence[Vector]) -> tuple[Vector, ...]:
    rows = tuple(vector(row) for row in duals)
    if len(rows) < 5:
        raise ContractError("a full-dimensional bounded 4-polytope needs at least five facets")
    if len(set(rows)) != len(rows):
        raise ContractError("duplicate normalized inequalities are redundant")
    if rank(rows) != 4:
        raise ContractError("facet covectors do not span four dimensions")
    if _has_closed_hemisphere_separator(rows):
        raise ContractError("facet covectors do not positively span")

    vertices = enumerate_vertices(rows)
    if not vertices:
        raise ContractError("normalized presentation has no exact vertices")
    for facet_index, facet in enumerate(rows):
        incident = [vertex_value for vertex_value in vertices if dot(facet, vertex_value) == 1]
        if len(incident) < 4:
            raise ContractError(f"facet {facet_index} is redundant or degenerate")
        differences = [
            add(vertex_value, scale(Fraction(-1), incident[0]))
            for vertex_value in incident[1:]
        ]
        if rank(differences) != 3:
            raise ContractError(f"facet {facet_index} is redundant or degenerate")
    return rows


@dataclass(frozen=True)
class FacetInequality:
    normal: Vector
    offset: Fraction


@dataclass(frozen=True)
class Presentation:
    facets: tuple[FacetInequality, ...]
    center: Vector


@dataclass(frozen=True)
class NormalizedConfiguration:
    duals: tuple[Vector, ...]
    center_convention: str
    scale_convention: str
    exact_volume: Fraction
    volume_quarter_root: Fraction


def normalize_parallelotope(presentation: Presentation) -> NormalizedConfiguration:
    """Normalize an exact centrally symmetric 4-parallelotope presentation.

    Opposite normalized facet pairs certify that the declared center is the
    symmetry center, hence the analytic center.  The four independent pair
    representatives give exact volume ``16 / |det U|``.
    """

    normalized: list[Vector] = []
    for facet in presentation.facets:
        slack = facet.offset - dot(facet.normal, presentation.center)
        if slack <= 0:
            raise ContractError("declared center is not strictly interior")
        normalized.append(scale(Fraction(1, 1) / slack, facet.normal))
    if len(normalized) != 8:
        raise ContractError("parallelotope normalization requires exactly eight facets")

    unused = set(range(8))
    representatives: list[Vector] = []
    while unused:
        left = min(unused)
        opposite = scale(Fraction(-1), normalized[left])
        matches = [index for index in unused if index != left and normalized[index] == opposite]
        if len(matches) != 1:
            raise ContractError("facets do not form four exact opposite pairs")
        unused.remove(left)
        unused.remove(matches[0])
        representatives.append(normalized[left])
    if rank(representatives) != 4:
        raise ContractError("opposite facet pairs do not define a parallelotope")

    exact_volume = Fraction(16, 1) / abs(determinant(matrix(representatives)))
    volume_quarter_root = exact_fourth_root(exact_volume)
    volume_one = tuple(scale(volume_quarter_root, row) for row in normalized)
    checked = validate_normalized_configuration(volume_one)
    return NormalizedConfiguration(
        duals=checked,
        center_convention="exact symmetry center = analytic center",
        scale_convention="ordinary Euclidean volume one",
        exact_volume=exact_volume,
        volume_quarter_root=volume_quarter_root,
    )


def gram(duals: Sequence[Vector]) -> Gram:
    return tuple(tuple(omega(left, right) for right in duals) for left in duals)


def squared_frobenius(left: Gram, right: Gram) -> Fraction:
    if len(left) != len(right):
        raise ContractError("labeled Gram matrices have unequal sizes")
    return sum(
        (
            (left[row][column] - right[row][column]) ** 2
            for row in range(len(left))
            for column in range(len(left))
        ),
        start=Fraction(),
    )


def permuted_squared_frobenius(left: Gram, right: Gram, permutation: tuple[int, ...]) -> Fraction:
    return sum(
        (
            (left[row][column] - right[permutation[row]][permutation[column]]) ** 2
            for row in range(len(left))
            for column in range(len(left))
        ),
        start=Fraction(),
    )


def fraction_text(value: Fraction | None) -> str | None:
    if value is None:
        return None
    return str(value.numerator) if value.denominator == 1 else f"{value.numerator}/{value.denominator}"


def radical_text(squared_distance: Fraction | None) -> str | None:
    if squared_distance is None:
        return None
    numerator_root = math.isqrt(squared_distance.numerator)
    denominator_root = math.isqrt(squared_distance.denominator)
    if numerator_root**2 == squared_distance.numerator and denominator_root**2 == squared_distance.denominator:
        return fraction_text(Fraction(numerator_root, denominator_root))
    return f"sqrt({fraction_text(squared_distance)})"


@dataclass(frozen=True)
class SearchResult:
    status: str
    exact: bool
    facet_count: int
    evaluated_permutations: int
    total_permutations: int
    elapsed_seconds: float
    squared_frobenius: Fraction | None
    squared_distance: Fraction | None
    distance_radical: str | None
    distance_approx: float | None
    minimizing_permutations: int
    second_distinct_squared_frobenius: Fraction | None
    multiple_minimizers: bool
    near_symmetry: bool | None

    def as_json(self) -> dict[str, object]:
        return {
            "status": self.status,
            "exact": self.exact,
            "facet_count": self.facet_count,
            "evaluated_permutations": self.evaluated_permutations,
            "total_permutations": self.total_permutations,
            "elapsed_seconds": self.elapsed_seconds,
            "squared_frobenius": fraction_text(self.squared_frobenius),
            "squared_distance": fraction_text(self.squared_distance),
            "distance_radical": self.distance_radical,
            "distance_approx": self.distance_approx,
            "minimizing_permutations": self.minimizing_permutations,
            "second_distinct_squared_frobenius": fraction_text(
                self.second_distinct_squared_frobenius
            ),
            "multiple_minimizers": self.multiple_minimizers,
            "near_symmetry": self.near_symmetry,
        }


def quotient_distance(
    left: NormalizedConfiguration,
    right: NormalizedConfiguration,
    *,
    max_facets: int = MAX_FACETS,
    timeout_seconds: float | None = None,
    near_symmetry_relative_gap: Fraction = Fraction(1, 1_000_000),
) -> SearchResult:
    facet_count = len(left.duals)
    if facet_count != len(right.duals):
        raise ContractError("unequal_facet_counts: quotient distance is stratum-local")
    if facet_count > max_facets:
        raise ContractError(f"facet_count_exceeds_exact_bound: {facet_count} > {max_facets}")
    if timeout_seconds is not None and timeout_seconds < 0:
        raise ContractError("timeout must be nonnegative")

    left_gram = gram(left.duals)
    right_gram = gram(right.duals)
    start = time.perf_counter()
    total = math.factorial(facet_count)
    best: Fraction | None = None
    distinct_values: set[Fraction] = set()
    minimizers = 0
    evaluated = 0
    for permutation in itertools.permutations(range(facet_count)):
        if timeout_seconds is not None and time.perf_counter() - start >= timeout_seconds:
            return SearchResult(
                status="timeout",
                exact=False,
                facet_count=facet_count,
                evaluated_permutations=evaluated,
                total_permutations=total,
                elapsed_seconds=time.perf_counter() - start,
                squared_frobenius=None,
                squared_distance=None,
                distance_radical=None,
                distance_approx=None,
                minimizing_permutations=0,
                second_distinct_squared_frobenius=None,
                multiple_minimizers=False,
                near_symmetry=None,
            )
        objective = permuted_squared_frobenius(left_gram, right_gram, permutation)
        evaluated += 1
        distinct_values.add(objective)
        if best is None or objective < best:
            best = objective
            minimizers = 1
        elif objective == best:
            minimizers += 1

    assert best is not None
    ordered = sorted(distinct_values)
    second = ordered[1] if len(ordered) > 1 else None
    scale_value = max(
        Fraction(1),
        sum(entry * entry for row in left_gram for entry in row)
        + sum(entry * entry for row in right_gram for entry in row),
    )
    near_symmetry = second is not None and second - best <= near_symmetry_relative_gap * scale_value
    squared_distance = best / (facet_count * facet_count)
    return SearchResult(
        status="exact",
        exact=True,
        facet_count=facet_count,
        evaluated_permutations=evaluated,
        total_permutations=total,
        elapsed_seconds=time.perf_counter() - start,
        squared_frobenius=best,
        squared_distance=squared_distance,
        distance_radical=radical_text(squared_distance),
        distance_approx=math.sqrt(float(squared_distance)),
        minimizing_permutations=minimizers,
        second_distinct_squared_frobenius=second,
        multiple_minimizers=minimizers > 1,
        near_symmetry=near_symmetry,
    )


def triangle_holds(left_squared: Fraction, middle_squared: Fraction, right_squared: Fraction) -> bool:
    """Check ``sqrt(right) <= sqrt(left) + sqrt(middle)`` exactly."""

    if right_squared <= left_squared + middle_squared:
        return True
    difference = right_squared - left_squared - middle_squared
    return difference * difference <= 4 * left_squared * middle_squared


def base_cube() -> Presentation:
    facets = []
    for coordinate in range(4):
        positive = [0, 0, 0, 0]
        positive[coordinate] = 2
        facets.append(FacetInequality(vector(positive), Fraction(1)))
        facets.append(FacetInequality(scale(Fraction(-1), vector(positive)), Fraction(1)))
    return Presentation(tuple(facets), vector((0, 0, 0, 0)))


def transform_presentation(
    presentation: Presentation,
    primal_map: Matrix,
    translation: Vector,
) -> Presentation:
    inverse_transpose = transpose(inverse(primal_map))
    facets = []
    for facet in presentation.facets:
        normal = matvec(inverse_transpose, facet.normal)
        facets.append(FacetInequality(normal, facet.offset + dot(normal, translation)))
    center = add(matvec(primal_map, presentation.center), translation)
    return Presentation(tuple(facets), center)


def permute_configuration(
    configuration: NormalizedConfiguration, permutation: Sequence[int]
) -> NormalizedConfiguration:
    if sorted(permutation) != list(range(len(configuration.duals))):
        raise ContractError("not a facet permutation")
    return NormalizedConfiguration(
        tuple(configuration.duals[index] for index in permutation),
        configuration.center_convention,
        configuration.scale_convention,
        configuration.exact_volume,
        configuration.volume_quarter_root,
    )


def givens(left: int, right: int, cosine: Fraction, sine: Fraction) -> Matrix:
    if cosine * cosine + sine * sine != 1:
        raise ContractError("Givens parameters must lie on the unit circle")
    rows = [list(row) for row in IDENTITY]
    rows[left][left] = cosine
    rows[left][right] = -sine
    rows[right][left] = sine
    rows[right][right] = cosine
    return matrix(rows)


def smoke_configurations() -> dict[str, NormalizedConfiguration]:
    base_presentation = base_cube()
    base = normalize_parallelotope(base_presentation)
    symplectic_map = matrix(
        ((2, 0, 0, 0), (0, Fraction(1, 3), 0, 0), (0, 0, Fraction(1, 2), 0), (0, 0, 0, 3))
    )
    scale_map = matrix(
        ((3, 0, 0, 0), (0, 3, 0, 0), (0, 0, 3, 0), (0, 0, 0, 3))
    )
    translation = vector((Fraction(7, 5), Fraction(-2, 3), Fraction(5, 7), Fraction(11, 13)))
    so4_map = matmul(
        givens(0, 3, Fraction(5, 13), Fraction(12, 13)),
        matmul(
            givens(1, 2, Fraction(7, 25), Fraction(24, 25)),
            givens(0, 1, Fraction(3, 5), Fraction(4, 5)),
        ),
    )
    nonsymplectic_gl = matrix(
        ((2, 0, 0, 0), (0, 1, 0, 0), (0, 0, 1, 0), (0, 0, 0, 8))
    )
    epsilon = Fraction(1, 10_000)
    near_map = matrix(
        (
            (1 + epsilon, 0, 0, 0),
            (0, 1, 0, 0),
            (0, 0, 1, 0),
            (0, 0, 0, Fraction(1, 1) / (1 + epsilon)),
        )
    )
    return {
        "base": base,
        "permuted": permute_configuration(base, (3, 0, 7, 2, 5, 1, 6, 4)),
        "nonorthogonal_symplectic": normalize_parallelotope(
            transform_presentation(base_presentation, symplectic_map, vector((0, 0, 0, 0)))
        ),
        "translated_scaled": normalize_parallelotope(
            transform_presentation(base_presentation, scale_map, translation)
        ),
        "so4_outside_u2": normalize_parallelotope(
            transform_presentation(base_presentation, so4_map, vector((0, 0, 0, 0)))
        ),
        "nonsymplectic_gl": normalize_parallelotope(
            transform_presentation(base_presentation, nonsymplectic_gl, vector((0, 0, 0, 0)))
        ),
        "near_symmetry": normalize_parallelotope(
            transform_presentation(base_presentation, near_map, vector((0, 0, 0, 0)))
        ),
    }


def _git_state(repo_root: Path) -> dict[str, object]:
    revision = subprocess.check_output(
        ["git", "rev-parse", "HEAD"], cwd=repo_root, text=True
    ).strip()
    dirty = bool(
        subprocess.check_output(
            ["git", "status", "--porcelain"], cwd=repo_root, text=True
        ).strip()
    )
    return {"revision": revision, "dirty": dirty}


def _source_sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def make_smoke_report(repo_root: Path) -> dict[str, object]:
    configurations = smoke_configurations()
    base = configurations["base"]
    controls: dict[str, object] = {}
    control_results: dict[str, SearchResult] = {}
    for name in (
        "base",
        "permuted",
        "nonorthogonal_symplectic",
        "translated_scaled",
        "so4_outside_u2",
        "nonsymplectic_gl",
    ):
        control_results[name] = quotient_distance(base, configurations[name])
        controls[name] = control_results[name].as_json()
    near_result = quotient_distance(
        configurations["near_symmetry"], configurations["near_symmetry"]
    )
    controls["near_symmetry_self_match"] = near_result.as_json()

    triangle_names = ("base", "so4_outside_u2", "nonsymplectic_gl")
    pair_results: dict[tuple[str, str], SearchResult] = {}
    for left, right in itertools.combinations(triangle_names, 2):
        if left == "base":
            pair_results[(left, right)] = control_results[right]
        else:
            pair_results[(left, right)] = quotient_distance(
                configurations[left], configurations[right]
            )
    triangle_checks = []
    for left, middle, right in itertools.permutations(triangle_names, 3):
        def pair_squared(first: str, second: str) -> Fraction:
            key = tuple(sorted((first, second), key=triangle_names.index))
            result = pair_results[key]  # type: ignore[index]
            assert result.squared_distance is not None
            return result.squared_distance

        triangle_checks.append(
            {
                "left": left,
                "middle": middle,
                "right": right,
                "holds_exactly": triangle_holds(
                    pair_squared(left, middle),
                    pair_squared(middle, right),
                    pair_squared(left, right),
                ),
            }
        )

    timeout = quotient_distance(base, base, timeout_seconds=0).as_json()
    source_path = Path(__file__)
    symplectic_map = matrix(
        ((2, 0, 0, 0), (0, Fraction(1, 3), 0, 0), (0, 0, Fraction(1, 2), 0), (0, 0, 0, 3))
    )
    so4_map = matmul(
        givens(0, 3, Fraction(5, 13), Fraction(12, 13)),
        matmul(
            givens(1, 2, Fraction(7, 25), Fraction(24, 25)),
            givens(0, 1, Fraction(3, 5), Fraction(4, 5)),
        ),
    )
    return {
        "schema": SCHEMA,
        "scientific_question": "Does the full symplectic Gram matrix give an exact fixed-F target-quotient metric at smoke size?",
        "coordinate_order": COORDINATE_ORDER,
        "normalization": {
            "center": "analytic center; certified here by exact central symmetry",
            "scale": "ordinary Euclidean volume one",
            "base_exact_volume": fraction_text(base.exact_volume),
        },
        "algorithm": {
            "search": "exhaustive simultaneous facet permutations",
            "max_facets": MAX_FACETS,
            "permutations_at_bound": math.factorial(MAX_FACETS),
            "arithmetic": "fractions.Fraction; exact squared distances and symbolic square roots",
            "near_symmetry_relative_gap": "1/1000000",
        },
        "matrix_controls": {
            "nonorthogonal_symplectic_is_symplectic": is_symplectic(symplectic_map),
            "nonorthogonal_symplectic_is_orthogonal": is_orthogonal(symplectic_map),
            "so4_is_orthogonal": is_orthogonal(so4_map),
            "so4_determinant": fraction_text(determinant(so4_map)),
            "so4_is_symplectic": is_symplectic(so4_map),
        },
        "distance_controls": controls,
        "timeout_control": timeout,
        "triangle_fixture_names": list(triangle_names),
        "triangle_checks": triangle_checks,
        "triangle_all_hold": all(check["holds_exactly"] for check in triangle_checks),
        "provenance": {
            "git": _git_state(repo_root),
            "producer": str(source_path.relative_to(repo_root)),
            "producer_sha256": _source_sha256(source_path),
            "command": "uv run --script quotient_distance.py --out artifacts/smoke-report.json",
        },
        "allowed_claim": "For validated normalized spanning configurations with one fixed F<=8, the packet computes the stated permutation quotient distance exactly unless it reports timeout.",
        "prohibited_claims": [
            "cross-facet-count distance",
            "polynomial-time or scalable permutation optimization",
            "numerical evidence as proof of Gram completeness",
            "equivalence with the natural topology of all polytope quotients",
        ],
    }


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--out", type=Path, required=True)
    arguments = parser.parse_args()
    repo_root = Path(__file__).resolve().parents[4]
    report = make_smoke_report(repo_root)
    arguments.out.parent.mkdir(parents=True, exist_ok=True)
    arguments.out.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n")
    print(json.dumps({"out": str(arguments.out), "schema": SCHEMA}, sort_keys=True))


if __name__ == "__main__":
    main()
