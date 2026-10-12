#!/usr/bin/env python3
"""Design endpoint/rotation-path candidates without evaluating sys."""

import json
import math
from pathlib import Path

ROOT = Path(__file__).resolve().parent / "artifacts"


def signed_area(vs):
    return 0.5 * sum(
        x0 * y1 - y0 * x1
        for (x0, y0), (x1, y1) in zip(vs, vs[1:] + vs[:1])
    )


def center(vs):
    cx = sum(x for x, _ in vs) / len(vs)
    cy = sum(y for _, y in vs) / len(vs)
    return [(x - cx, y - cy) for x, y in vs]


def rotate(vs, theta):
    c, s = math.cos(theta), math.sin(theta)
    return [(c * x - s * y, s * x + c * y) for x, y in vs]


def scale(vs, factor):
    return [(factor * x, factor * y) for x, y in vs]


def edges(vs):
    return [
        (x1 - x0, y1 - y0)
        for (x0, y0), (x1, y1) in zip(vs, vs[1:] + vs[:1])
    ]


def hrep(vs):
    assert signed_area(vs) > 0
    out = []
    for (x0, y0), (x1, y1) in zip(vs, vs[1:] + vs[:1]):
        dx, dy = x1 - x0, y1 - y0
        length = math.hypot(dx, dy)
        # Clockwise edge normal is outward for a CCW polygon.
        nx, ny = dy / length, -dx / length
        h = nx * x0 + ny * y0
        assert h > 0
        out.append({"normal": [nx, ny], "height": h})
    return out


def ridge_r(p, q):
    total = sum(abs(ax * bx + ay * by) for ax, ay in edges(p) for bx, by in edges(q))
    return total / math.sqrt(signed_area(p) * signed_area(q))


def convex_hull(points):
    points = sorted(set(points))
    def cross(o, a, b):
        return (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0])
    lower = []
    for p in points:
        while len(lower) >= 2 and cross(lower[-2], lower[-1], p) <= 0:
            lower.pop()
        lower.append(p)
    upper = []
    for p in reversed(points):
        while len(upper) >= 2 and cross(upper[-2], upper[-1], p) <= 0:
            upper.pop()
        upper.append(p)
    return center(lower[:-1] + upper[:-1])


def difference_body(vs):
    return convex_hull([(x-u, y-v) for x, y in vs for u, v in vs])


def solve_rotation(p, q0, target, hi):
    lo = 0.0
    assert ridge_r(p, q0) <= target <= ridge_r(p, rotate(q0, hi))
    for _ in range(80):
        mid = 0.5 * (lo + hi)
        if ridge_r(p, rotate(q0, mid)) < target:
            lo = mid
        else:
            hi = mid
    return 0.5 * (lo + hi)


def regular(n, phase):
    return [(math.cos(phase + 2*math.pi*i/n), math.sin(phase + 2*math.pi*i/n)) for i in range(n)]


def main():
    # q=.01, .001, .0001 empirical thresholds from the surviving frozen 1M cache.
    targets = {
        "3x6": [11.301776905190875, 10.955292837190573, 10.726592837214973],
        "4x4": [10.920458917858735, 10.0000935222588, 9.33513422859976],
    }
    # Normalize every factor's (constant) support height to 1, inside the
    # frozen producer interval [0.8,1.2]. Independent factor scaling leaves R
    # invariant. The product map diag(alpha I_q,beta I_p) is conformally
    # symplectic with multiplier alpha*beta, so sys is invariant too.
    p36 = scale(regular(3, 0.0), 2.0)  # inradius 1
    q36 = scale(rotate(difference_body(p36), -math.pi / 2), 1.0 / 3.0)  # inradius 1
    p44 = scale(regular(4, math.pi / 4), math.sqrt(2.0))  # inradius 1
    q44 = list(p44)
    families = {"3x6": (p36, q36, math.pi / 6), "4x4": (p44, q44, math.pi / 4)}
    rows = []
    for bucket, (p, q0, hi) in families.items():
        path = []
        for label, target, nominal_q in zip(("q01", "q001", "q0001"), targets[bucket], (1e-2, 1e-3, 1e-4)):
            delta = solve_rotation(p, q0, target, hi)
            path.append((label, delta, nominal_q))
        path.append(("endpoint", 0.0, 0.0))
        for label, delta, nominal_q in path:
            q = rotate(q0, delta)
            rows.append({
                "candidate_id": f"ridge-endpoint-{bucket}-{label}",
                "bucket": bucket,
                "path_label": label,
                "nominal_empirical_q": nominal_q,
                "relative_rotation_rad": delta,
                "q_vertices_ccw": p,
                "p_vertices_ccw": q,
                "q_hrep": hrep(p),
                "p_hrep": hrep(q),
                "edge_formula_r": ridge_r(p, q),
                "q_area": signed_area(p),
                "p_area": signed_area(q),
            })
    with (ROOT / "candidates.jsonl").open("w") as f:
        for row in rows:
            f.write(json.dumps(row, separators=(",", ":")) + "\n")
    with (ROOT / "design-summary.tsv").open("w") as f:
        f.write("candidate_id\tbucket\tlabel\tdelta_rad\tedge_formula_r\tnominal_empirical_q\tmin_support_height\tmax_support_height\n")
        for row in rows:
            hs = [x["height"] for x in row["q_hrep"] + row["p_hrep"]]
            f.write(f"{row['candidate_id']}\t{row['bucket']}\t{row['path_label']}\t{row['relative_rotation_rad']:.17g}\t{row['edge_formula_r']:.17g}\t{row['nominal_empirical_q']:.8g}\t{min(hs):.17g}\t{max(hs):.17g}\n")


if __name__ == "__main__":
    main()
