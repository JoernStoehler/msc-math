#!/usr/bin/env python3
"""Optional reproducibility checks for ../math/research.tex.

The mathematical proofs are in the note and do not require this script.
Symbolic identities and Fraction comparisons are exact. All NumPy checks
are explicitly floating-point diagnostics, not capacity certificates.

Run: python check.py --output results.json
Dependencies: Python >= 3.10, numpy, sympy. No network access is used.
"""
from __future__ import annotations

import argparse
import itertools
import json
import math
import platform
from fractions import Fraction as F
from pathlib import Path

import numpy as np
import sympy as sp


def exact_checks() -> dict:
    rt5 = sp.sqrt(5)
    lam = (rt5 - 1) / 2
    h = (1 + rt5) / 4
    sine36 = sp.sqrt(10 - 2 * rt5) / 4
    sine72 = sp.sqrt(10 + 2 * rt5) / 4
    n = [sp.Matrix(v) for v in [
        (1, 0), ((rt5 - 1) / 4, sine72), (-h, sine36),
        (-h, -sine36), ((rt5 - 1) / 4, -sine72),
    ]]
    e, a, b = n[0], lam * n[2], lam * n[3]
    reflect = sp.diag(1, -1)
    checks: dict[str, bool] = {}

    def equal(name: str, left, right) -> None:
        difference = left - right
        entries = list(difference) if isinstance(difference, sp.MatrixBase) else [difference]
        ok = all(sp.simplify(sp.expand(v)) == 0 for v in entries)
        if not ok:
            raise AssertionError(f"Exact identity failed: {name}: {difference}")
        checks[name] = True

    equal("closure", e + a + b, sp.zeros(2, 1))
    equal("normal_relation_1", n[2] + n[3], -e / lam)
    equal("normal_relation_2", lam * n[1], e + n[2])
    equal("normal_relation_3", lam * n[4], e + n[3])
    T1 = e * e.T - a * a.T
    T2 = e * b.T - a * e.T
    equal("tensor_identity_1", T1,
          lam**2 * e * e.T - lam**2 * e * n[3].T - lam**3 * n[1] * n[2].T)
    equal("tensor_identity_2", T2,
          lam**2 * n[1] * n[3].T - lam**2 * n[2] * n[4].T)
    equal("identity_1_mass", 2 * lam**2 + lam**3, sp.Integer(1))
    if not bool(0 < 2 * lam**2 < 1):
        raise AssertionError("Second identity's convex mass is not below one")
    checks["identity_2_mass_below_one"] = True
    possibilities = [(e, a), (e, b), (a, e), (b, e), (a, b), (b, a)]
    representations = [T1, T1 * reflect, T2 * reflect, T2,
                       -reflect * T1 * reflect, -reflect * T1]
    for j, ((v, z), representation) in enumerate(zip(possibilities, representations)):
        equal(f"six_order_table_{j + 1}", e * v.T - a * z.T, representation)
    # The other six orders follow by left reflection, also checked directly.
    for j, (v, z) in enumerate(possibilities):
        equal(f"left_reflection_order_{j + 1}",
              e * v.T - b * z.T, reflect * (e * v.T - a * z.T))

    # All twelve isosceles coefficients: verify the four squared norm types.
    t, u = sp.symbols("t u", real=True)
    xs = [sp.Integer(1), (-1 + sp.I*t)/2, (-1 - sp.I*t)/2]
    ys = [sp.Integer(1), (-1 + sp.I*u)/2, (-1 - sp.I*u)/2]
    norm_types = [((3+t*u)**2+(t+u)**2)/16,
                  ((3-t*u)**2+(t-u)**2)/16,
                  (t+u)**2/4, (t-u)**2/4]
    occurrences = [0, 0, 0, 0]
    for last_q in (1, 2):
        for p in itertools.permutations(range(3)):
            z = xs[0]*sp.conjugate(ys[p[0]])-xs[last_q]*sp.conjugate(ys[p[1]])
            norm = sp.expand(z * sp.conjugate(z))
            matched = [k for k, v in enumerate(norm_types) if sp.expand(norm-v) == 0]
            if len(matched) != 1:
                raise AssertionError(f"Unexpected isosceles norm: {norm}")
            occurrences[matched[0]] += 1
    checks["all_12_isosceles_norms"] = True
    equal("isosceles_dominance_factorization", norm_types[0]-norm_types[2],
          (t-sp.sqrt(3))*(u-sp.sqrt(3))*(3+t*u+sp.sqrt(3)*(t+u))/16)

    # Exact pentagon edge contact and exact systolic constant.
    tau = 2 * a[1]
    edge_z = (3+tau**2)/4 + sp.I*tau/2
    equal("pentagon_switch_edge_contact", edge_z,
          (1-lam)+lam*(h+sp.I*sine36))
    area = sp.Rational(5, 2)*sine72
    prefactor = (1+h)**4/(2*area**2)
    equal("systolic_prefactor", prefactor, (5+2*rt5)/10)
    equal("sharp_systolic_constant", prefactor/((1+h)/2), (3+rt5)/5)

    T, U, V, r0 = F(49, 100), F(22, 5), F(73, 100), F(4237, 11007)
    f = lambda x: 8*x**3-4*x**2-4*x+1
    assert f(F(111,500)) > 0 > f(F(223,1000))
    assert f(F(9,10)) < 0
    assert F(19,81) < T*T
    assert F(237679,12321) < U*U
    assert F(447,200)**2 < 5
    assert 5 - 2*F(447,200) < V*V
    assert F(223,1223)*F(19,9) == r0
    assert F(19,20) > F(19,20)**2
    assert 1-F(2,1225) > F(99,100)
    values = [((3+T*T)**2+4*T*T)/16,
              r0*r0*(T+U)**2/4,
              r0**4*((3+U*U)**2+4*U*U)/16,
              ((3+V*T)**2+(V+T)**2)/16,
              r0*r0*(V+U)**2/4]
    printed_values = [F(1145864801,1600000000),
                      F(476971178161,538462440000),
                      F(116304862857201778241,146783035890944010000),
                      F(1276254929,1600000000),
                      F(58326597081,59829160000)]
    assert values == printed_values
    thresholds = [F(361,400)]*3+[F(9801,10000)]*2
    assert all(v < threshold for v, threshold in zip(values, thresholds))
    return {
        "status": "all exact checks passed",
        "symbolic_checks": checks,
        "isosceles_squared_norm_type_multiplicities": occurrences,
        "rational_bounds": [{"value": str(v), "strict_threshold": str(th),
                             "positive_gap": str(th-v)}
                            for v, th in zip(values, thresholds)],
        "scope": "Exact finite algebra and inequalities only; full geometric proofs are in research.tex."
    }


def regular_normals(n: int) -> np.ndarray:
    theta = 2 * np.pi * np.arange(n) / n
    return np.column_stack([np.cos(theta), np.sin(theta)])


def closure_steps(n: int) -> list[np.ndarray]:
    """Enumerate positive 3-point closures independently from gap classification."""
    u = regular_normals(n) / np.cos(np.pi/n)
    answer = []
    # For odd n there are no antipodal two-point closures.
    assert n % 2 == 1
    for inds in itertools.combinations(range(n), 3):
        vs = u[list(inds)]
        mat = np.vstack([vs.T, np.ones(3)])
        if abs(np.linalg.det(mat)) < 1e-12:
            continue
        alpha = np.linalg.solve(mat, [0., 0., 1.])
        if np.min(alpha) > 1e-10:
            x = alpha[:, None] * vs
            assert np.linalg.norm(x.sum(axis=0)) < 1e-10
            answer.append(x)
    return answer


WORDS = [(0,)+p for p in itertools.permutations(range(1, 6))]
PAIR_INDICES = list(itertools.combinations(range(6), 2))
# Weight signs for *all 120* cyclic six-entry words, including grouped ones.
SIGN_MATRIX = np.zeros((len(WORDS), len(PAIR_INDICES)))
BLOCK_COUNTS = []
for wi, word in enumerate(WORDS):
    pos = {entry: k for k, entry in enumerate(word)}
    for pi, (a, b) in enumerate(PAIR_INDICES):
        SIGN_MATRIX[wi, pi] = 1.0 if pos[a] < pos[b] else -1.0
    colors = [v >= 3 for v in word]
    BLOCK_COUNTS.append(sum(colors[k] != colors[(k+1) % 6] for k in range(6)))
BLOCK_COUNTS = np.array(BLOCK_COUNTS)


def all_word_tensors(xs: list[np.ndarray], ys: list[np.ndarray]) -> tuple[np.ndarray,np.ndarray]:
    """Compute Q from the original full word sum, NOT the reduced six-block formula."""
    all_tensors = []
    for x in xs:
        for y in ys:
            base = np.zeros((len(PAIR_INDICES), 2, 2))
            # Weighted facet vectors are x/2 and y/2. In canonical index order
            # every cross-color pair has its q entry first.
            for pi, (a, b) in enumerate(PAIR_INDICES):
                if a < 3 <= b:
                    base[pi] = np.outer(x[a], y[b-3])/4
            all_tensors.append(np.einsum("wp,pab->wab", SIGN_MATRIX, base))
    tensors = np.concatenate(all_tensors)
    counts = np.tile(BLOCK_COUNTS, len(xs)*len(ys))
    return tensors, counts


def rotation(theta: float) -> np.ndarray:
    c, s = np.cos(theta), np.sin(theta)
    return np.array([[c,-s],[s,c]])


def max_q(tensors: np.ndarray, R: np.ndarray) -> float:
    return float(np.einsum("wab,ab->w", tensors, R).max())


def numeric_checks() -> dict:
    """Diagnostics only. Exhaustive supports/words, floating-point arithmetic."""
    triples = {n: closure_steps(n) for n in (3,5,7)}
    assert {n:len(x) for n,x in triples.items()} == {3:1,5:5,7:14}
    regular_results = []
    cached_tensors = {}
    for n, m in ((3,3),(5,5),(7,7),(5,7)):
        tensors, counts = all_word_tensors(triples[n], triples[m])
        cached_tensors[n,m] = tensors
        L = math.lcm(n,m)
        if n == 3:
            radius = 2/(3*np.sqrt(3))
            phase = np.pi/6
            N = 6
        else:
            radius = 1/(2*(1+np.cos(np.pi/n))*(1+np.cos(np.pi/m)))
            phase = 0.
            N = 2*L
        vertex_angles = phase+2*np.pi*np.arange(N)/N
        vertices = radius*np.column_stack([np.cos(vertex_angles),np.sin(vertex_angles)])
        # All exact predicted hull vertices and facet normals are included,
        # as well as additional deterministic angles.
        angle_checks = list(vertex_angles)+list(vertex_angles+np.pi/N)+list(np.linspace(-2.9,8.1,113))
        errors = []
        for theta in angle_checks:
            observed = max_q(tensors, rotation(float(theta)))
            expected = float(np.max(vertices @ [np.cos(theta),np.sin(theta)]))
            errors.append(abs(observed-expected))
        assert max(errors) < 5e-13
        I = np.eye(2)
        J = rotation(np.pi/2)
        coeffs = np.column_stack([np.einsum("wab,ab->w",tensors,I),
                                  np.einsum("wab,ab->w",tensors,J)])
        max_halfspace_violation = -float("inf")
        for theta in vertex_angles+np.pi/N:
            residual = coeffs @ [np.cos(theta),np.sin(theta)]-radius*np.cos(np.pi/N)
            max_halfspace_violation = max(max_halfspace_violation,float(residual.max()))
        assert max_halfspace_violation < 5e-13
        six_max_radius = float(np.linalg.norm(coeffs[counts == 6],axis=1).max())
        regular_results.append({
            "pair": [n,m], "closure_counts": [len(triples[n]),len(triples[m])],
            "cyclic_words_per_pair": len(WORDS), "total_candidates": len(tensors),
            "angles_checked": len(angle_checks), "max_absolute_Q_error": max(errors),
            "max_predicted_hull_halfspace_violation": max_halfspace_violation,
            "max_six_block_coefficient_radius": six_max_radius,
            "predicted_hull_circumradius": radius,
            "predicted_hull_inradius": radius*np.cos(np.pi/N),
        })

    pent_tensors = cached_tensors[5,5]
    n5 = regular_normals(5)
    w2 = (1+np.cos(np.pi/5))**2
    affine_cases = [
        (np.eye(2),np.eye(2)),
        (np.eye(2),rotation(np.pi/10)),
        (np.array([[2.,1.],[0.,.5]]),np.array([[1.,-.3],[.2,1.7]])),
        (np.array([[-1.,.4],[.7,2.]]),np.array([[.5,2.],[-1.,1.]])),
        (np.array([[3.,0.],[0.,.25]]),np.array([[1.,1.1],[0.,1.]])),
        (rotation(.73) @ np.diag([1.3,.8]),np.array([[1.,-.7],[-.1,-1.2]])),
    ]
    affine_results = []
    area = 2.5*np.sin(2*np.pi/5)
    benchmark = (3+np.sqrt(5))/5
    for G,H in affine_cases:
        for theta in (-.37,0.,np.pi/10,1.27):
            invG, invH = np.linalg.inv(G),np.linalg.inv(H)
            # R_theta H is the second primal transformation.
            effective = invG @ rotation(theta) @ invH.T
            expected_q = float(np.abs(n5 @ effective @ n5.T).max()/(2*w2))
            # Fully enumerated original word tensors paired with the actual
            # transformed bilinear form; no optimized support is assumed.
            observed_q = max_q(pent_tensors,effective)
            assert abs(observed_q-expected_q) < 1e-12
            observed_capacity = 1/(2*observed_q)
            ratio = observed_capacity**2/(2*area**2*abs(np.linalg.det(G)*np.linalg.det(H)))
            assert ratio <= benchmark+1e-11
            affine_results.append({
                "G":G.tolist(),"H":H.tolist(),"theta":theta,
                "absolute_Q_error":abs(observed_q-expected_q),
                "capacity":observed_capacity,"systolic_ratio":ratio,
            })
    return {
        "status": "all floating-point diagnostics passed",
        "warning": "These diagnostics are not proofs, interval certificates, or an exhaustive parameter search.",
        "arithmetic": "NumPy float64",
        "regular_pairs": regular_results,
        "affine_pentagon_cases": affine_results,
        "largest_affine_Q_error":max(v["absolute_Q_error"] for v in affine_results),
    }


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output",type=Path,default=Path(__file__).with_name("results.json"))
    args = parser.parse_args()
    result = {
        "description":"Optional checks accompanying the mathematical research handoff.",
        "environment":{"python":platform.python_version(),"numpy":np.__version__,"sympy":sp.__version__},
        "exact":exact_checks(),
        "floating_point":numeric_checks(),
    }
    args.output.parent.mkdir(parents=True,exist_ok=True)
    args.output.write_text(json.dumps(result,indent=2)+"\n",encoding="utf-8")
    print(f"All exact checks and floating-point diagnostics passed. Results: {args.output}")


if __name__ == "__main__":
    main()
