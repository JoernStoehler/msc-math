#!/usr/bin/env python3
"""Exact triangle-product falsifier for the all-single 3-bounce claim.

The implementation is deliberately dependency-free.  Stage one consumes only
the retained 3x3 dual coordinates and freezes geometry-only predicates.  Stage
two then joins that frozen file to the retained class-minimum target artifact.
"""
from __future__ import annotations

import argparse
import itertools
import json
import random
import time
from fractions import Fraction
from pathlib import Path

F = Fraction
ZERO = F(0)
ONE = F(1)


def rat(s: str) -> F:
    n, d = s.split("/", 1)
    return F(int(n), int(d))


def frac_json(x: F) -> str:
    return f"{x.numerator}/{x.denominator}"


def vadd(a, b):
    return tuple(x + y for x, y in zip(a, b))


def vsub(a, b):
    return tuple(x - y for x, y in zip(a, b))


def dot(a, b):
    return sum((x * y for x, y in zip(a, b)), ZERO)


def cross2(a, b):
    return a[0] * b[1] - a[1] * b[0]


def solve2(a, b, c, d, e, f):
    det = a * d - b * c
    if det == 0:
        return None
    return ((e * d - b * f) / det, (a * f - e * c) / det)


def primal_vertices(duals):
    """Vertices of {x : <dual_i,x> <= 1}, exactly."""
    out = []
    for i, j in itertools.combinations(range(3), 2):
        p = solve2(duals[i][0], duals[i][1], duals[j][0], duals[j][1], ONE, ONE)
        if p is not None and all(dot(u, p) <= ONE for u in duals):
            out.append(p)
    out = list(dict.fromkeys(out))
    if len(out) != 3:
        raise ValueError(f"expected a full-dimensional triangle, got {len(out)} primal vertices")
    return out


def polar_from_vertices(vertices):
    """Vertices of the polar of a polygon given by its vertices."""
    out = []
    for i, j in itertools.combinations(range(len(vertices)), 2):
        p = solve2(vertices[i][0], vertices[i][1], vertices[j][0], vertices[j][1], ONE, ONE)
        if p is not None and all(dot(v, p) <= ONE for v in vertices):
            out.append(p)
    out = list(dict.fromkeys(out))
    if len(out) < 4:
        raise ValueError("difference-body polar is not full-dimensional")
    return out


def difference_polar(duals, offset):
    ids = range(3) if offset == 0 else range(3, 6)
    tri = [tuple(duals[i][offset : offset + 2]) for i in ids]
    vertices = primal_vertices(tri)
    diffs = [vsub(a, b) for a in vertices for b in vertices]
    return polar_from_vertices(diffs)


def barycentric(duals, offset):
    ids = range(3) if offset == 0 else range(3, 6)
    tri = [tuple(duals[i][offset : offset + 2]) for i in ids]
    # Solve [x_i; y_i; 1] lambda = [0; 0; 1] by Cramer's rule.
    mat = [[tri[i][0] for i in range(3)], [tri[i][1] for i in range(3)], [ONE] * 3]

    def det(m):
        return (
            m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1])
            - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
            + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0])
        )

    det0 = det(mat)
    if det0 == 0:
        raise ValueError("dual triangle is not full-dimensional")
    rhs = [ZERO, ZERO, ONE]
    out = []
    for col in range(3):
        m = [row[:] for row in mat]
        for row in range(3):
            m[row][col] = rhs[row]
        out.append(det(m) / det0)
    if not all(x > ZERO for x in out):
        raise ValueError("origin is not strictly interior to the dual triangle")
    return out


def omega(a, b):
    return a[0] * b[2] - a[2] * b[0] + a[1] * b[3] - a[3] * b[1]


def canonical_phenotype(signs):
    forms = []
    for qr in range(3):
        for pc in range(3):
            m = [[signs[(i + qr) % 3][(j + pc) % 3] for j in range(3)] for i in range(3)]
            forms.append("".join("+" if x > 0 else "-" if x < 0 else "0" for row in m for x in row))
    return min(forms)


def cycle_words(signs):
    """Words with q_0 fixed, hence one representative per temporal rotation."""
    out = []
    for qrest in itertools.permutations((1, 2)):
        q = (0,) + qrest
        for p in itertools.permutations((0, 1, 2)):
            word = tuple(x for pair in zip(q, p) for x in (pair[0], pair[1] + 3))
            good = True
            for k in range(6):
                i, j = word[k], word[(k + 1) % 6]
                if i < 3 and j >= 3:
                    good &= signs[i][j - 3] >= 0
                elif i >= 3 and j < 3:
                    good &= signs[j][i - 3] <= 0
                else:
                    good = False
            if good:
                out.append(word)
    return out


def word_qp(duals, word, qbar, pbar):
    # The q and p masses are s and 1-s.  The objective is C*s*(1-s),
    # maximized exactly at s=1/2 for C>0.
    beta = []
    for i in word:
        if i < 3:
            beta.append(qbar[i] / 2)
        else:
            beta.append(pbar[i - 3] / 2)
    q = sum((beta[i] * beta[j] * omega(duals[word[i]], duals[word[j]])
             for i in range(6) for j in range(i + 1, 6)), ZERO)
    return beta, q, (ONE / (2 * q) if q > ZERO else None)


def a2_exact(duals):
    # For a triangle with dual vertices a_i and positive normalized
    # barycentrics lambda_i of 0, the difference-body polar is
    # conv{+/- lambda_i a_i}.  The two planar factors pair as the standard
    # symplectic cross pairing omega((q,0),(0,p)) = q dot p.
    qbar = barycentric(duals, 0)
    pbar = barycentric(duals, 2)
    m = max((abs(qbar[i] * pbar[j] * omega(duals[i], duals[3 + j]))
             for i in range(3) for j in range(3)), default=ZERO)
    if m <= ZERO:
        raise ValueError("difference-body polar pairing is not positive")
    return ONE / m


def a2_exact_geometry(duals):
    """Independent finite half-space reconstruction used as a formula check."""
    dp = difference_polar(duals, 0)
    dq = difference_polar(duals, 2)
    m = max((x[0] * y[0] + x[1] * y[1] for x in dp for y in dq), default=ZERO)
    if m <= ZERO:
        raise ValueError("difference-body polar pairing is not positive")
    return ONE / m


def parse_duals(row):
    return [tuple(rat(s) for s in item) for item in row["dual_vertices_rational"]]


def freeze_row(name, duals):
    signs = [[(omega(duals[i], duals[3 + j]) > ZERO) - (omega(duals[i], duals[3 + j]) < ZERO)
              for j in range(3)] for i in range(3)]
    qbar = barycentric(duals, 0)
    pbar = barycentric(duals, 2)
    words = []
    for word in cycle_words(signs):
        beta, q, action = word_qp(duals, word, qbar, pbar)
        words.append({
            "word": list(word),
            "weights": [frac_json(x) for x in beta],
            "objective_q": frac_json(q),
            "action": None if action is None else frac_json(action),
        })
    return {
        "name": name,
        "cross_signs": signs,
        "strict_sign_cell": all(x != 0 for row in signs for x in row),
        "canonical_phenotype": canonical_phenotype(signs),
        "transition_feasible_word_count": len(words),
        "transition_feasible_words": words,
        "a2_exact": frac_json(a2_exact(duals)),
    }


def read_geometry(input_path, limit):
    rows = []
    with input_path.open() as stream:
        for line in stream:
            row = json.loads(line)
            if row.get("k") == 3 and row.get("m") == 3:
                rows.append(freeze_row(row["name"], parse_duals(row)))
                if len(rows) == limit:
                    break
    if len(rows) != limit:
        raise RuntimeError(f"expected {limit} 3x3 rows, found {len(rows)}")
    return rows


def validate_a2_formula(input_path, checks=16):
    """Cross-check the fast barycentric formula against half-space vertices."""
    done = 0
    with input_path.open() as stream:
        for line in stream:
            row = json.loads(line)
            if row.get("k") != 3 or row.get("m") != 3:
                continue
            duals = parse_duals(row)
            if a2_exact(duals) != a2_exact_geometry(duals):
                raise AssertionError(f"A2 formula mismatch on {row['name']}")
            done += 1
            if done == checks:
                break
    if done != checks:
        raise RuntimeError(f"A2 formula check saw {done} rows, expected {checks}")
    return done


def reveal(input_path, class_path, frozen):
    targets = {}
    with input_path.open() as stream:
        for line in stream:
            row = json.loads(line)
            if row.get("name") in {x["name"] for x in frozen}:
                targets[row["name"]] = row
    classes = {}
    with class_path.open() as stream:
        for line in stream:
            row = json.loads(line)
            if row["name"] in targets:
                classes[row["name"]] = row
    revealed = []
    for g in frozen:
        row, cls = targets[g["name"]], classes[g["name"]]
        a2 = rat(g["a2_exact"])
        actions = [rat(w["action"]) for w in g["transition_feasible_words"] if w["action"]]
        all_single = min(actions) if actions else None
        c3 = cls["class_minima"].get("3")
        c2 = cls["class_minima"].get("2")
        stored_a3 = None if c3 is None else rat(c3["action_exact"])
        stored_a2 = None if c2 is None else rat(c2["action_exact"])
        strict_ok = all(a < a2 for a in actions) if (g["strict_sign_cell"] and actions) else None
        revealed.append({
            "name": g["name"],
            "strict_sign_cell": g["strict_sign_cell"],
            "all_single_available": bool(actions),
            "stored_a3_available": c3 is not None,
            "a3_availability_matches": bool(actions) == (c3 is not None),
            "all_single_min_action": None if all_single is None else frac_json(all_single),
            "stored_a3_action": None if stored_a3 is None else frac_json(stored_a3),
            "stored_a3_action_matches_all_single": (stored_a3 == all_single) if (stored_a3 is not None and all_single is not None) else None,
            "stored_a2_action": None if stored_a2 is None else frac_json(stored_a2),
            "a2_exact_matches_stored_a2": stored_a2 == a2,
            "a2_exact": g["a2_exact"],
            "strict_dominance_all_single": strict_ok,
            "producer_bounces": row["bounces"],
            "producer_label_matches_availability": row["bounces"] == (3 if actions else 2),
            "stored_sys": row["sys"],
            "stored_capacity": row["capacity"],
            "stored_volume": row["volume"],
            "sys_recomputed": row["capacity"] ** 2 / (2 * row["volume"]),
            "sys_abs_error": abs(row["sys"] - row["capacity"] ** 2 / (2 * row["volume"])),
        })
    return revealed


def make_triangle(rng):
    # Positive integer barycentric relation and rational vertices.
    while True:
        l = [F(rng.randint(1, 20)) for _ in range(3)]
        u = (F(rng.randint(-100, 100), 10), F(rng.randint(-100, 100), 10))
        v = (F(rng.randint(-100, 100), 10), F(rng.randint(-100, 100), 10))
        if cross2(u, v) == 0:
            continue
        w = (-(l[0] * u[0] + l[1] * v[0]) / l[2], -(l[0] * u[1] + l[1] * v[1]) / l[2])
        if cross2(u, w) != 0 and cross2(v, w) != 0:
            return [(u[0], u[1], ZERO, ZERO), (v[0], v[1], ZERO, ZERO),
                    (w[0], w[1], ZERO, ZERO)]


def stress(seed, count):
    rng = random.Random(seed)
    strict = zero = 0
    feasible = 0
    violations = []
    near = None
    for idx in range(count):
        q = make_triangle(rng)
        p2 = make_triangle(rng)
        p = [(ZERO, ZERO, u[0], u[1]) for u in p2]
        duals = q + p
        g = freeze_row(f"stress_{idx:05d}", duals)
        signs = g["cross_signs"]
        if g["strict_sign_cell"]:
            strict += 1
        else:
            zero += 1
        a2 = rat(g["a2_exact"])
        for w in g["transition_feasible_words"]:
            if w["action"] is None:
                continue
            feasible += 1
            action = rat(w["action"])
            margin = a2 - action
            if margin <= ZERO:
                item = {"index": idx, "a2_exact": g["a2_exact"], "word": w,
                        "cross_signs": signs, "strict_sign_cell": g["strict_sign_cell"],
                        "margin_exact": frac_json(margin)}
                violations.append(item)
            elif near is None or margin < rat(near["margin_exact"]):
                near = {"index": idx, "a2_exact": g["a2_exact"], "word": w,
                        "cross_signs": signs, "margin_exact": frac_json(margin)}
    strict_violations = [x for x in violations if x["strict_sign_cell"]]
    boundary_equalities = [x for x in violations if not x["strict_sign_cell"]]
    return {"seed": seed, "requested_pairs": count, "strict_sign_pairs": strict,
            "zero_pairing_pairs": zero, "feasible_all_single_words": feasible,
            "strict_counterexamples": strict_violations,
            "boundary_equalities_or_violations": boundary_equalities,
            "counterexamples": violations, "near_boundary_extremizer": near}


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--input", type=Path, required=True)
    ap.add_argument("--class-minima", type=Path, required=True)
    ap.add_argument("--out", type=Path, required=True)
    ap.add_argument("--stress-count", type=int, default=20000)
    ap.add_argument("--stress-seed", type=int, default=20260714)
    args = ap.parse_args()
    args.out.mkdir(parents=True, exist_ok=True)
    t0 = time.perf_counter()
    formula_checks = validate_a2_formula(args.input)
    frozen = read_geometry(args.input, 1024)
    (args.out / "geometry-freeze.json").write_text(json.dumps({"row_count": len(frozen), "rows": frozen}, separators=(",", ":")) + "\n")
    revealed = reveal(args.input, args.class_minima, frozen)
    (args.out / "target-reveal.json").write_text(json.dumps({"row_count": len(revealed), "rows": revealed}, separators=(",", ":")) + "\n")
    stress_result = stress(args.stress_seed, args.stress_count)
    (args.out / "stress-summary.json").write_text(json.dumps(stress_result, separators=(",", ":")) + "\n")
    strict_rows = [x for x in revealed if x["strict_sign_cell"]]
    violations = [x for x in stress_result["strict_counterexamples"]]
    summary = {
        "retained_3x3_rows": len(frozen),
        "strict_sign_rows": sum(x["strict_sign_cell"] for x in frozen),
        "zero_pairing_rows": sum(not x["strict_sign_cell"] for x in frozen),
        "feasible_word_total": sum(x["transition_feasible_word_count"] for x in frozen),
        "revealed_strict_rows": len(strict_rows),
        "stored_a2_exact_mismatches": sum(not x["a2_exact_matches_stored_a2"] for x in revealed),
        "a3_availability_mismatches": sum(not x["a3_availability_matches"] for x in revealed),
        "a3_action_mismatches": sum(x["stored_a3_action_matches_all_single"] is False for x in revealed),
        "producer_label_availability_mismatches": sum(not x["producer_label_matches_availability"] for x in revealed),
        "strict_dominance_feasible_rows": sum(x["strict_dominance_all_single"] is not None for x in revealed),
        "strict_dominance_true_rows": sum(x["strict_dominance_all_single"] is True for x in revealed),
        "strict_dominance_unavailable_rows": sum(x["strict_dominance_all_single"] is None for x in revealed),
        "stored_sys_max_abs_error": max(x["sys_abs_error"] for x in revealed),
        "a2_formula_geometry_cross_checks": formula_checks,
        "stress": {k: stress_result[k] for k in ("seed", "requested_pairs", "strict_sign_pairs", "zero_pairing_pairs", "feasible_all_single_words")},
        "stress_strict_counterexample_count": len(violations),
        "stress_boundary_equality_count": len(stress_result["boundary_equalities_or_violations"]),
        "wall_seconds": time.perf_counter() - t0,
    }
    (args.out / "summary.json").write_text(json.dumps(summary, indent=2) + "\n")


if __name__ == "__main__":
    main()
