"""Produce one exact rational nonregular-pentagon calibration certificate.

Standard library only. No retained experiment is read or replaced. The JSON
certificate supplies exact input geometry and all normal-template placements;
the mathematical implication uses BMP's imported covering criterion and the
calibration-transfer argument documented alongside this producer.
"""

import itertools
import json
from fractions import Fraction as F
from functools import cmp_to_key


def dot(a, b):
    return sum(x * y for x, y in zip(a, b))


def cross(a, b):
    return a[0] * b[1] - a[1] * b[0]


def scale(a, t):
    return tuple(t * x for x in a)


def add(a, b):
    return tuple(x + y for x, y in zip(a, b))


def area(points):
    def half(point):
        return 0 if point[1] > 0 or (point[1] == 0 and point[0] >= 0) else 1

    def compare(a, b):
        if half(a) != half(b):
            return half(a) - half(b)
        determinant = cross(a, b)
        return -1 if determinant > 0 else 1 if determinant < 0 else 0

    ordered = sorted(points, key=cmp_to_key(compare))
    value = sum(cross(ordered[i], ordered[(i + 1) % len(ordered)])
                for i in range(len(ordered))) / 2
    assert value > 0
    return value


def encode(value):
    if isinstance(value, F):
        return str(value)
    if isinstance(value, (list, tuple)):
        return [encode(item) for item in value]
    if isinstance(value, dict):
        return {key: encode(item) for key, item in value.items()}
    return value


def main():
    if not __debug__:
        raise RuntimeError("This certificate producer requires enabled assertions; omit -O")
    normals = [(F(1), F(0)), (F(309, 1000), F(951, 1000)),
               (F(-809, 1000), F(588, 1000)), (F(-809, 1000), F(-588, 1000)),
               (F(309, 1000), F(-951, 1000))]
    heights = [F(809, 1000), F(813, 1000), F(806, 1000), F(810, 1000), F(805, 1000)]
    assert all(height > 0 for height in heights)
    assert all(cross(a, b) != 0 for a, b in itertools.combinations(normals, 2))
    vertices = []
    for i, j in itertools.combinations(range(5), 2):
        a, b = normals[i], normals[j]
        determinant = cross(a, b)
        vertex = ((heights[i] * b[1] - a[1] * heights[j]) / determinant,
                  (a[0] * heights[j] - heights[i] * b[0]) / determinant)
        if all(dot(row, vertex) <= height for row, height in zip(normals, heights)):
            vertices.append(vertex)
    assert len(vertices) == 5
    assert all(sum(dot(row, v) == height for v in vertices) == 2
               for row, height in zip(normals, heights))

    def support(direction):
        return max(dot(direction, vertex) for vertex in vertices)

    differences = [add(a, scale(b, -1)) for a in vertices for b in vertices]
    reference_vertices = [scale(row, sign / (2 * (support(row) + support(scale(row, -1)))))
                          for row in normals for sign in (F(1), F(-1))]
    assert all(dot(difference, b) <= F(1, 2)
               for difference in differences for b in reference_vertices)
    # Verify the polar vertex inventory directly before using it for area labels.
    polar_vertices = set()
    for a, b in itertools.combinations(differences, 2):
        determinant = cross(a, b)
        if determinant == 0:
            continue
        vertex = ((b[1] - a[1]) / (2 * determinant),
                  (a[0] - b[0]) / (2 * determinant))
        if all(dot(row, vertex) <= F(1, 2) for row in differences):
            polar_vertices.add(vertex)
    assert polar_vertices == set(reference_vertices)

    covers = []
    for indices in itertools.combinations(range(5), 3):
        a, b, c = [normals[i] for i in indices]
        weights = [cross(b, c), cross(c, a), cross(a, b)]
        if all(weight < 0 for weight in weights):
            weights = [-weight for weight in weights]
        if not all(weight > 0 for weight in weights):
            continue
        normalization = sum(weight * support(normals[i]) for i, weight in zip(indices, weights))
        steps = [scale(normals[i], weight / normalization) for i, weight in zip(indices, weights)]
        assert add(add(steps[0], steps[1]), steps[2]) == (F(0), F(0))
        assert sum(support(step) for step in steps) == 1
        triangle = [(F(0), F(0)), steps[0], add(steps[0], steps[1])]
        cover = None
        for edge in range(3):
            translation = scale(add(triangle[edge], triangle[(edge + 1) % 3]), F(-1, 2))
            placed = [add(vertex, translation) for vertex in triangle]
            margin = min(F(1, 2) - dot(difference, vertex)
                         for difference in differences for vertex in placed)
            if margin >= 0:
                cover = {"normal_indices": indices, "closure_weights": weights,
                         "unit_length_steps": steps, "triangle_vertices": triangle,
                         "translation": translation, "minimum_margin": margin}
                break
        if cover is None:
            raise RuntimeError(f"No checked midpoint cover for template {indices}")
        covers.append(cover)
    assert len(covers) == 5
    # A positive noncollinear closure proves boundedness of the halfplane body.
    assert covers
    factor_area, reference_area = area(vertices), area(reference_vertices)
    ratio = 1 / (2 * factor_area * reference_area)
    assert ratio < 1
    certificate = {
        "arithmetic": "exact rational, Python fractions.Fraction",
        "factor": {"facet_normals": normals, "facet_heights": heights, "vertices": vertices},
        "reference_body": {"definition": "(Q-Q)^polar/2", "vertices": reference_vertices},
        "positive_closure_triples": len(covers), "normal_template_classes": 2 * len(covers),
        "covers": covers,
        "exact_control_labels": {"capacity_of_reference_product": F(1),
                                 "factor_area": factor_area, "reference_area": reference_area,
                                 "product_four_volume": factor_area * reference_area,
                                 "systolic_ratio": ratio},
        "opposite_order_argument": "T_other=u-T; central inversion preserves the reference body",
        "conditional_mathematical_conclusion": "BMP criterion plus the documented transfer gives c(B_Q x_L Q)=1 and c(K x_L Q)=W_Q(K) for every symmetric K",
        "status": "new control example, not human-accepted or a production-algorithm certificate",
    }
    print(json.dumps(encode(certificate), indent=2))


if __name__ == "__main__":
    main()
