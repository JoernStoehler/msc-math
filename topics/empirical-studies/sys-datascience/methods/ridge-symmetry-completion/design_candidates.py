#!/usr/bin/env python3
"""Freeze the reviewed two-point regular 3x6 continuation without target calls."""

import hashlib
import json
import math
from pathlib import Path


ROOT = Path(__file__).resolve().parent
ARTIFACTS = ROOT / "artifacts"
DELTAS = (
    ("delta1", 0.4929785313028738, 0.966454139826057),
    ("delta2", 0.5082886534505863, 0.982782061720280),
)


def rotate(vertices, theta):
    c, s = math.cos(theta), math.sin(theta)
    return [(c*x-s*y, s*x+c*y) for x, y in vertices]


def area(vertices):
    return 0.5 * sum(x*y1-y*x1 for (x,y),(x1,y1) in zip(vertices, vertices[1:]+vertices[:1]))


def edges(vertices):
    return [(x1-x,y1-y) for (x,y),(x1,y1) in zip(vertices, vertices[1:]+vertices[:1])]


def hrep(vertices):
    rows = []
    for (x,y),(x1,y1) in zip(vertices, vertices[1:]+vertices[:1]):
        dx, dy = x1-x, y1-y
        length = math.hypot(dx, dy)
        nx, ny = dy/length, -dx/length
        rows.append({"normal":[nx,ny], "height":nx*x+ny*y})
    return rows


def ridge_r(p, q):
    numerator = sum(abs(ax*bx+ay*by) for ax,ay in edges(p) for bx,by in edges(q))
    return numerator / math.sqrt(area(p)*area(q))


def main():
    ARTIFACTS.mkdir(exist_ok=True)
    triangle = [(2*math.cos(2*math.pi*i/3), 2*math.sin(2*math.pi*i/3)) for i in range(3)]
    hexagon0 = [
        (2/math.sqrt(3)*math.cos(math.pi+2*math.pi*i/6),
         2/math.sqrt(3)*math.sin(math.pi+2*math.pi*i/6))
        for i in range(6)
    ]
    rows = []
    for label, delta, predicted_sys in DELTAS:
        hexagon = rotate(hexagon0, delta)
        rows.append({
            "schema":"ridge-symmetry-completion.candidate.v1",
            "candidate_id":f"ridge-symmetry-3x6-{label}",
            "bucket":"3x6", "path_label":label,
            "relative_rotation_rad":delta,
            "selection_rule":"one-third/two-thirds of the frozen gap from delta0 to pi/6",
            "branch_h_predicted_sys":predicted_sys,
            "q_vertices_ccw":triangle, "p_vertices_ccw":hexagon,
            "q_hrep":hrep(triangle), "p_hrep":hrep(hexagon),
            "edge_formula_r":ridge_r(triangle, hexagon),
            "q_area":area(triangle), "p_area":area(hexagon),
        })
    candidate_path = ARTIFACTS / "candidates.jsonl"
    candidate_path.write_text("".join(json.dumps(row,separators=(",",":"))+"\n" for row in rows))
    digest = hashlib.sha256(candidate_path.read_bytes()).hexdigest()
    (ARTIFACTS / "freeze.json").write_text(json.dumps({
        "schema":"ridge-symmetry-completion.freeze.v1",
        "status":"target-free",
        "candidate_count":2,
        "ordered_candidate_ids":[row["candidate_id"] for row in rows],
        "candidates_sha256":digest,
        "decision_rule":"evaluate delta1; stop if sys>1, otherwise evaluate delta2",
        "endpoint_fact":"sys(pi/6)=1 is theorem-level and is not a target call",
    }, indent=2)+"\n")
    print(digest)


if __name__ == "__main__":
    main()
