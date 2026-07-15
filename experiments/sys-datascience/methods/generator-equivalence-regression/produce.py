#!/usr/bin/env python3
"""Generate the target-free exact-equivalence regression matrix.

The packet intentionally uses only the Python standard library and copy-local
exact rational witnesses.  Rows are ordinary dictionaries so a later executor
can add a case without importing an experiment interface.
"""

from __future__ import annotations

import argparse
import csv
import hashlib
import io
import json
import subprocess
import sys
from fractions import Fraction as F
from pathlib import Path


PACKET = Path(__file__).resolve().parent
REPO = next(parent for parent in PACKET.parents if (parent / ".git").exists())
SCHEMA = "generator-equivalence-regression-v1"
VIEWS = (
    "law_parameters",
    "raw_geometry_after_transform",
    "combinatorics",
    "euclidean_features",
    "signed_symplectic_features",
    "absolute_symplectic_features",
    "normalized_product_features",
    "matrix_identity",
)
OUTCOMES = {"zero", "nonzero", "not_applicable"}
VIEW_DEFINITIONS = {
    "law_parameters": "direct difference of probability laws/marks after the stated law-level coupling or pushforward",
    "raw_geometry_after_transform": "residual after explicitly applying the row's named geometric transformation",
    "combinatorics": "direct unlabelled combinatorial difference; all named invertible transformations preserve it",
    "euclidean_features": "direct difference of Euclidean metric features of the two nominal arms, without quotient alignment",
    "signed_symplectic_features": "direct difference of signed omega-based features of the two nominal arms",
    "absolute_symplectic_features": "direct difference after taking absolute values of omega-based features",
    "normalized_product_features": "direct difference of the declared scale/area/volume-normalized product features",
    "matrix_identity": "residual of the row's declared matrix/form identity, not distance of the transform from identity",
}
LEVELS = {
    "full_law",
    "component_marginal",
    "paired_pushforward",
    "pointwise_orbit",
    "not_equivalent",
}
SOURCE_PATHS = (
    "experiments/sys-datascience/methods/alternative-generator-smoke/main.rs",
    "experiments/sys-datascience/methods/alternative-generator-smoke/README.md",
    "experiments/sys-datascience/methods/generator-orientation-smoke/main.rs",
    "experiments/sys-datascience/methods/generator-orientation-smoke/README.md",
    "experiments/sys-datascience/methods/generator-orbit-perturbation-zoo/main.rs",
    "experiments/sys-datascience/methods/generator-orbit-perturbation-zoo/README.md",
    "experiments/sys-datascience/methods/ridge-endpoint-path/notes/endpoint-predictions.md",
    "crates/euclidean-polytopes/tests/polar_vertices.rs",
    "crates/euclidean-polytopes/DEVELOPMENT.md",
    "thesis/02-preliminaries-polytope-input-language.tex",
)


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def run_git(*args: str) -> str:
    return subprocess.check_output(["git", *args], cwd=REPO, text=True).strip()


def dot(a, b):
    return sum((x * y for x, y in zip(a, b)), F(0))


def transpose(a):
    return tuple(tuple(a[i][j] for i in range(len(a))) for j in range(len(a[0])))


def matmul(a, b):
    bt = transpose(b)
    return tuple(tuple(dot(row, col) for col in bt) for row in a)


def matvec(a, x):
    return tuple(dot(row, x) for row in a)


def determinant(a):
    m = [list(row) for row in a]
    out = F(1)
    for col in range(len(m)):
        pivot = next((i for i in range(col, len(m)) if m[i][col]), None)
        if pivot is None:
            return F(0)
        if pivot != col:
            m[col], m[pivot] = m[pivot], m[col]
            out = -out
        p = m[col][col]
        out *= p
        for i in range(col + 1, len(m)):
            q = m[i][col] / p
            for j in range(col, len(m)):
                m[i][j] -= q * m[col][j]
    return out


I4 = tuple(tuple(F(i == j) for j in range(4)) for i in range(4))
J = (
    (F(0), F(0), F(1), F(0)),
    (F(0), F(0), F(0), F(1)),
    (F(-1), F(0), F(0), F(0)),
    (F(0), F(-1), F(0), F(0)),
)


def omega(x, y):
    return dot(x, matvec(J, y))


def gram(points, form):
    return tuple(tuple(form(x, y) for y in points) for x in points)


def solve2(a, b, rhs):
    d = a[0] * b[1] - a[1] * b[0]
    assert d
    return (
        (rhs[0] * b[1] - a[1] * rhs[1]) / d,
        (a[0] * rhs[1] - rhs[0] * b[0]) / d,
    )


def vertices_from_halfspaces(normals, heights):
    vertices = set()
    for i in range(len(normals)):
        for j in range(i + 1, len(normals)):
            a, b = normals[i], normals[j]
            if a[0] * b[1] == a[1] * b[0]:
                continue
            x = solve2(a, b, (heights[i], heights[j]))
            if all(dot(n, x) <= h for n, h in zip(normals, heights)):
                vertices.add(x)
    return tuple(sorted(vertices))


def all_active(normals, heights):
    vertices = vertices_from_halfspaces(normals, heights)
    return bool(vertices) and all(
        any(dot(n, x) == h for x in vertices) for n, h in zip(normals, heights)
    )


def polygon_area_ordered(points):
    twice = sum(
        points[i][0] * points[(i + 1) % len(points)][1]
        - points[i][1] * points[(i + 1) % len(points)][0]
        for i in range(len(points))
    )
    return abs(twice) / 2


def expected(**kwargs):
    assert set(kwargs) == set(VIEWS)
    assert set(kwargs.values()) <= OUTCOMES
    return kwargs


NA = "not_applicable"
ZERO = "zero"
NONZERO = "nonzero"


def rows():
    alt = "experiments/sys-datascience/methods/alternative-generator-smoke"
    orient = "experiments/sys-datascience/methods/generator-orientation-smoke"
    zoo = "experiments/sys-datascience/methods/generator-orbit-perturbation-zoo"
    polar = "crates/euclidean-polytopes/tests/polar_vertices.rs"
    base = [
        {
            "row_id": "angles-iid-dirichlet1-marginal",
            "objects_laws": "unordered n IID Uniform[0,2pi) angles versus Dirichlet(1^n) cyclic gaps plus a uniform common rotation and a uniformly marked cyclic root",
            "level": "component_marginal",
            "hypotheses_conditioning": "n>=3; compare the angle proposal only; impose the same angle-only boundedness condition max cyclic gap<pi if conditioning; do not include support-dependent irredundancy/acceptance",
            "transformation": "sort cyclically, quotient/add the common rotation, and convert consecutive differences to gaps",
            "expected": expected(law_parameters=ZERO, raw_geometry_after_transform=ZERO, combinatorics=NA, euclidean_features=NA, signed_symplectic_features=NA, absolute_symplectic_features=NA, normalized_product_features=NA, matrix_identity=NA),
            "proof_status": "proved_standard_order_statistics_with_local_bijection_control",
            "proof_source": [f"{alt}/main.rs: random_angles and dirichlet", f"{alt}/README.md: boundedness and acceptance boundary"],
            "arithmetic": "exact rational simplex/rotation witness; probability identity is analytic",
            "executable_control": "pass: rational cyclic-gap roundtrip and matched max-gap predicate",
            "collapse_scope": "collapse only the pre-support angle-proposal marginal",
        },
        {
            "row_id": "iid-support-baseline-vs-equal-support-dirichlet1",
            "objects_laws": "current IID-angle IID-support baseline versus equal-support Dirichlet alpha=1 generator",
            "level": "not_equivalent",
            "hypotheses_conditioning": "baseline supports are IID Uniform[0.8,1.2); Dirichlet arm supports equal 1; each arm then applies its own all-facets-active and area-normalization acceptance",
            "transformation": "angle-marginal coupling from the previous row; no transformation identifies the support marks or support-dependent accepted full laws",
            "expected": expected(law_parameters=NONZERO, raw_geometry_after_transform=NONZERO, combinatorics=ZERO, euclidean_features=NONZERO, signed_symplectic_features=NA, absolute_symplectic_features=NA, normalized_product_features=NONZERO, matrix_identity=NA),
            "proof_status": "proved_by_support_mark_counterexample",
            "proof_source": [f"{alt}/main.rs: baseline, equal_support, dirichlet, make_pair", f"{alt}/README.md: law and normalization declarations"],
            "arithmetic": "exact rational support-mark negative control",
            "executable_control": "negative_control_pass: 4/5 and 6/5 marks cannot equal all-one marks",
            "collapse_scope": "never collapse the full generator arms",
        },
        {
            "row_id": "tangential-to-same-angle-inscribed-polar",
            "objects_laws": "equal-support tangential polygon T={x:<n_i,x><=1} and same marked-origin inscribed polygon I=conv{n_i}",
            "level": "paired_pushforward",
            "hypotheses_conditioning": "unit normal rays in cyclic order, 0 in the interior, irredundant facets, and the origin retained as the polarity mark",
            "transformation": "I=T^circ; normalized halfspace rows n_i are the polar vertices",
            "expected": expected(law_parameters=ZERO, raw_geometry_after_transform=ZERO, combinatorics=ZERO, euclidean_features=NONZERO, signed_symplectic_features=NA, absolute_symplectic_features=NA, normalized_product_features=NA, matrix_identity=NA),
            "proof_status": "proved_convex_polarity",
            "proof_source": ["thesis/02-preliminaries-polytope-input-language.tex", f"{polar}: exact polarity soundness/roundtrip tests", f"{alt}/main.rs: tangentialize and inscribed formulas"],
            "arithmetic": "exact rational unit-normal fixture",
            "executable_control": "pass: exact halfspace vertices, active facets, and polar support equalities",
            "collapse_scope": "collapse only when polarity and the marked origin are explicit",
        },
        {
            "row_id": "area-normalized-polar-scale-correction",
            "objects_laws": "independently area-normalized tangential T and same-angle inscribed I=T^circ",
            "level": "paired_pushforward",
            "hypotheses_conditioning": "same hypotheses as tangential polar row; A_T,A_I>0; each factor normalized to area one independently",
            "transformation": "I_norm=(A_T A_I)^(-1/2) (T_norm)^circ; the uncorrected normalized bodies are generally not literal polars",
            "expected": expected(law_parameters=ZERO, raw_geometry_after_transform=ZERO, combinatorics=ZERO, euclidean_features=NONZERO, signed_symplectic_features=NA, absolute_symplectic_features=NA, normalized_product_features=ZERO, matrix_identity=NA),
            "proof_status": "proved_by_polar_scaling_identity",
            "proof_source": [f"{alt}/main.rs: area_normalize, tangentialize, inscribed", "thesis/02-preliminaries-polytope-input-language.tex"],
            "arithmetic": "exact symbolic square of the homothety factor from rational areas",
            "executable_control": "pass: exact A_T, A_I and squared correction (A_T A_I)^-1",
            "collapse_scope": "collapse after recording the polarity scale; do not claim literal normalized polarity",
        },
        {
            "row_id": "double-polarity-marked-origin",
            "objects_laws": "a marked-origin irredundant rational polygon T and (T^circ)^circ",
            "level": "paired_pushforward",
            "hypotheses_conditioning": "closed convex polygon with 0 in its interior; preserve the same origin mark",
            "transformation": "apply polarity twice",
            "expected": expected(law_parameters=ZERO, raw_geometry_after_transform=ZERO, combinatorics=ZERO, euclidean_features=ZERO, signed_symplectic_features=NA, absolute_symplectic_features=NA, normalized_product_features=ZERO, matrix_identity=NA),
            "proof_status": "proved_bipolar_theorem_exact_fixture_control",
            "proof_source": [f"{polar}: exact polarity roundtrip tests", "crates/euclidean-polytopes/DEVELOPMENT.md: exact polarity roundtrip contract"],
            "arithmetic": "exact rational",
            "executable_control": "pass: every original supporting row is recovered as an extreme polar vertex on the fixture",
            "collapse_scope": "collapse double-polar arms with the same origin mark",
        },
        {
            "row_id": "independent-positive-factor-scalings",
            "objects_laws": "product QxP and (aQ)x(bP), a,b>0",
            "level": "pointwise_orbit",
            "hypotheses_conditioning": "positive scales; product coordinates ordered (q1,q2,p1,p2)",
            "transformation": "diag(a I2,b I2); conformally symplectic multiplier ab; factor areas scale a^2,b^2 and 4-volume scales a^2 b^2",
            "expected": expected(law_parameters=NA, raw_geometry_after_transform=ZERO, combinatorics=ZERO, euclidean_features=NONZERO, signed_symplectic_features=NONZERO, absolute_symplectic_features=NONZERO, normalized_product_features=ZERO, matrix_identity=ZERO),
            "proof_status": "proved_algebraic_target_free_orbit",
            "proof_source": ["experiments/sys-datascience/methods/ridge-endpoint-path/notes/endpoint-predictions.md: factor-scale gauge", f"{zoo}/README.md: transformation contracts"],
            "arithmetic": "exact rational",
            "executable_control": "pass: A^T J A=ab J, determinant=a^2b^2, normalized area/volume ratios",
            "collapse_scope": "collapse factor-scale arms in target-free geometry-coverage accounting; target equivalence requires a separately sourced conformal-invariance argument",
        },
        {
            "row_id": "simultaneous-common-factor-rotation",
            "objects_laws": "product QxP and (RQ)x(RP) under the same planar rotation R",
            "level": "pointwise_orbit",
            "hypotheses_conditioning": "R in SO(2); product coordinates (q1,q2,p1,p2)",
            "transformation": "diag(R,R), a symplectic orthogonal map",
            "expected": expected(law_parameters=NA, raw_geometry_after_transform=ZERO, combinatorics=ZERO, euclidean_features=ZERO, signed_symplectic_features=ZERO, absolute_symplectic_features=ZERO, normalized_product_features=ZERO, matrix_identity=ZERO),
            "proof_status": "proved_matrix_identity",
            "proof_source": [f"{alt}/README.md: common planar rotation gauge", f"{orient}/main.rs and README.md: matrix convention and symplectic controls"],
            "arithmetic": "exact rational signed-permutation rotation",
            "executable_control": "pass: A^T A=I and A^T J A=J with Gram controls",
            "collapse_scope": "collapse simultaneous common-rotation arms in target-free symplectic-orbit/feature coverage",
        },
        {
            "row_id": "one-factor-only-rotation-negative-control",
            "objects_laws": "product QxP and (RQ)xP for nontrivial R",
            "level": "not_equivalent",
            "hypotheses_conditioning": "generic factors; R is a quarter turn; do not confuse common and relative rotation",
            "transformation": "diag(R,I2), which is orthogonal but not symplectic in (q1,q2,p1,p2) order",
            "expected": expected(law_parameters=NA, raw_geometry_after_transform=ZERO, combinatorics=ZERO, euclidean_features=ZERO, signed_symplectic_features=NONZERO, absolute_symplectic_features=NONZERO, normalized_product_features=NONZERO, matrix_identity=NONZERO),
            "proof_status": "proved_matrix_counterexample",
            "proof_source": [f"{alt}/README.md: relative factor rotation remains explicit", f"{orient}/README.md: Euclidean versus symplectic semantics"],
            "arithmetic": "exact rational",
            "executable_control": "negative_control_pass: A^T J A differs from J and a cross-factor omega entry changes",
            "collapse_scope": "never collapse relative-rotation arms as common gauge",
        },
        {
            "row_id": "four-facet-broken-pairs-width-control",
            "objects_laws": "two independent opposite support pairs (four sides) and symmetric strips with the same two widths",
            "level": "pointwise_orbit",
            "hypotheses_conditioning": "two linearly independent normal directions; all four supports positive and facets active",
            "transformation": "translate by -t, where u_i dot t=(h_i^+-h_i^-)/2 for i=1,2",
            "expected": expected(law_parameters=NONZERO, raw_geometry_after_transform=ZERO, combinatorics=ZERO, euclidean_features=ZERO, signed_symplectic_features=NA, absolute_symplectic_features=NA, normalized_product_features=ZERO, matrix_identity=NA),
            "proof_status": "proved_linear_translation_system",
            "proof_source": [f"{alt}/main.rs: antipodal_broken_and_control width matching", f"{alt}/README.md: paired broken/control law"],
            "arithmetic": "exact rational",
            "executable_control": "pass: exact translation solves both support differences and translated halfspaces equal the symmetric control",
            "collapse_scope": "collapse four-facet broken/control factor arms up to translation",
        },
        {
            "row_id": "six-facet-broken-pairs-not-generally-translation",
            "objects_laws": "three broken opposite support pairs (six sides) and width-matched symmetric strips",
            "level": "not_equivalent",
            "hypotheses_conditioning": "three normal directions spanning R2; all six supports positive and all facets active",
            "transformation": "candidate translation must solve three equations u_i dot t=(h_i^+-h_i^-)/2 in two unknowns",
            "expected": expected(law_parameters=NONZERO, raw_geometry_after_transform=NONZERO, combinatorics=ZERO, euclidean_features=NONZERO, signed_symplectic_features=NA, absolute_symplectic_features=NA, normalized_product_features=NONZERO, matrix_identity=NA),
            "proof_status": "proved_overdetermined_counterexample",
            "proof_source": [f"{alt}/main.rs: antipodal_broken_and_control and six-side availability"],
            "arithmetic": "exact rational",
            "executable_control": "negative_control_pass: active six-sided witness violates the third translation equation",
            "collapse_scope": "do not collapse six-or-more-side broken/control arms without solving every translation equation",
        },
        {
            "row_id": "gl4-volume-normalization-vs-sl4-representative",
            "objects_laws": "A in GL+(4) followed by four-volume normalization versus S=(det A)^(-1/4) A in SL(4)",
            "level": "paired_pushforward",
            "hypotheses_conditioning": "det A>0; compare the same base body; scalar fourth root is the positive root",
            "transformation": "remove the positive scalar radial part before/after volume normalization",
            "expected": expected(law_parameters=ZERO, raw_geometry_after_transform=ZERO, combinatorics=ZERO, euclidean_features=ZERO, signed_symplectic_features=ZERO, absolute_symplectic_features=ZERO, normalized_product_features=ZERO, matrix_identity=ZERO),
            "proof_status": "proved_algebraic_matrix_identity",
            "proof_source": [f"{zoo}/README.md: no normalized GL+ duplicate", f"{zoo}/main.rs: determinant-one SL4 law"],
            "arithmetic": "exact rational perfect-fourth determinant witness; analytic identity general",
            "executable_control": "pass: det A=16, S=A/2, det S=1, and normalization maps agree",
            "collapse_scope": "collapse a GL+ arm only with its own induced SL representative",
        },
        {
            "row_id": "random-gl4-law-vs-arbitrary-sl4-law",
            "objects_laws": "a random GL+(4) matrix law normalized radially versus an independently specified random SL(4) law",
            "level": "not_equivalent",
            "hypotheses_conditioning": "both laws may have positive/unit determinant, but their induced laws on S=(det A)^(-1/4)A must be compared",
            "transformation": "radial quotient map GL+(4)->SL(4)",
            "expected": expected(law_parameters=NONZERO, raw_geometry_after_transform=NONZERO, combinatorics=ZERO, euclidean_features=NONZERO, signed_symplectic_features=NONZERO, absolute_symplectic_features=NONZERO, normalized_product_features=NONZERO, matrix_identity=NONZERO),
            "proof_status": "proved_law_counterexample",
            "proof_source": [f"{zoo}/README.md and main.rs: explicit coordinate-dependent bounded SL4 law"],
            "arithmetic": "exact rational Dirac-law counterexample",
            "executable_control": "negative_control_pass: normalized 2I gives I while chosen determinant-one diagonal map is nonidentity",
            "collapse_scope": "do not collapse random laws unless their induced radial-quotient laws match",
        },
        {
            "row_id": "u2-subgroup-sp4-pointwise-orbit",
            "objects_laws": "a body K and U K for U in U(2) under the real (q,p)-block embedding",
            "level": "pointwise_orbit",
            "hypotheses_conditioning": "U unitary; real coordinates ordered (q1,q2,p1,p2)",
            "transformation": "real embedding [[Re U,-Im U],[Im U,Re U]], contained in O(4) intersect Sp(4)",
            "expected": expected(law_parameters=NA, raw_geometry_after_transform=ZERO, combinatorics=ZERO, euclidean_features=ZERO, signed_symplectic_features=ZERO, absolute_symplectic_features=ZERO, normalized_product_features=ZERO, matrix_identity=ZERO),
            "proof_status": "proved_subgroup_matrix_identity",
            "proof_source": [f"{orient}/main.rs: deterministic_u2_i8 and Haar embedding", f"{orient}/README.md: U2 semantic controls", f"{zoo}/README.md: U2 orbit arm"],
            "arithmetic": "exact rational signed-permutation U(2) witness",
            "executable_control": "pass: U^T U=I, U^T J U=J, determinant=1, Euclidean and omega Grams fixed",
            "collapse_scope": "collapse pointwise U(2) arms when only symplectic/Euclidean orbit-invariant quantities are consumed",
        },
        {
            "row_id": "antiunitary-antisymplectic-endpoint",
            "objects_laws": "K and A K for A=diag(-1,-1,1,1), an orthogonal antiunitary/anti-symplectic endpoint",
            "level": "pointwise_orbit",
            "hypotheses_conditioning": "coordinates (q1,q2,p1,p2); target-free matrix and feature controls only",
            "transformation": "A^T A=I, det A=1, and A^T J A=-J",
            "expected": expected(law_parameters=NA, raw_geometry_after_transform=ZERO, combinatorics=ZERO, euclidean_features=ZERO, signed_symplectic_features=NONZERO, absolute_symplectic_features=ZERO, normalized_product_features=ZERO, matrix_identity=ZERO),
            "proof_status": "capacity_and_sys_invariance_proof_pending; target_free_matrix_and_absolute_omega_controls_proved",
            "proof_source": [f"{orient}/main.rs: deterministic_so4_i8", f"{zoo}/README.md and main.rs: anti-symplectic pi endpoint"],
            "arithmetic": "exact rational",
            "executable_control": "matrix_feature_control_pass_proof_pending: signed omega Gram flips and absolute omega Gram is fixed; no capacity/sys claim",
            "collapse_scope": "do not collapse target arms until an authoritative capacity/sys anti-symplectic invariance theorem is supplied",
        },
        {
            "row_id": "generic-so4-not-u2-negative-control",
            "objects_laws": "K and A K for A=diag(-1,-1,1,1), viewed incorrectly as a U(2) element",
            "level": "not_equivalent",
            "hypotheses_conditioning": "orthogonal determinant-one is insufficient for U(2)=O(4) intersect Sp(4)",
            "transformation": "test the signed symplectic form, not only Euclidean and determinant views",
            "expected": expected(law_parameters=NA, raw_geometry_after_transform=ZERO, combinatorics=ZERO, euclidean_features=ZERO, signed_symplectic_features=NONZERO, absolute_symplectic_features=ZERO, normalized_product_features=ZERO, matrix_identity=NONZERO),
            "proof_status": "proved_matrix_counterexample",
            "proof_source": [f"{orient}/README.md: SO4 versus U2 distinction", f"{zoo}/main.rs: anti-symplectic endpoint test"],
            "arithmetic": "exact rational",
            "executable_control": "negative_control_pass: A^T J A=-J rather than J and signed omega entry reverses",
            "collapse_scope": "never infer U(2)/Sp(4) membership from SO(4) membership",
        },
        {
            "row_id": "polar-without-origin-mark-negative-control",
            "objects_laws": "a tangential polygon T and a translated copy T+t presented without preserving the polarity origin",
            "level": "not_equivalent",
            "hypotheses_conditioning": "nonzero translation t while polarity is still taken about coordinate origin",
            "transformation": "translation does not commute with origin-based polarity by a fixed translation/scale",
            "expected": expected(law_parameters=NA, raw_geometry_after_transform=NONZERO, combinatorics=ZERO, euclidean_features=ZERO, signed_symplectic_features=NA, absolute_symplectic_features=NA, normalized_product_features=NONZERO, matrix_identity=NA),
            "proof_status": "proved_origin_mark_counterexample",
            "proof_source": ["thesis/02-preliminaries-polytope-input-language.tex: origin-based normalized halfspaces", f"{alt}/main.rs: polar coupling deferred for lack of named exact center"],
            "arithmetic": "exact rational",
            "executable_control": "negative_control_pass: translated support rows n_i/(1+n_i dot t) differ nonuniformly",
            "collapse_scope": "never apply the polar collapse without a shared marked origin",
        },
    ]
    for row in base:
        row["executable_control_status"] = row["executable_control"].split(":", 1)[0]
    return base


def witness_results():
    out = []

    gaps = (F(1, 10), F(2, 10), F(3, 10), F(4, 10))
    rotation = F(7, 100)
    angles = tuple(rotation + sum(gaps[:i], F(0)) for i in range(4))
    recovered = tuple(angles[i + 1] - angles[i] for i in range(3)) + (F(1) - sum(gaps[:3]),)
    assert recovered == gaps and (max(gaps) < F(1, 2))
    out.append(("angles-iid-dirichlet1-marginal", {"gaps": gaps, "rotation_turns": rotation, "roundtrip": True, "bounded_max_gap_lt_half_turn": True}))

    supports = (F(4, 5), F(6, 5), F(9, 10), F(11, 10))
    assert any(h != 1 for h in supports)
    out.append(("iid-support-baseline-vs-equal-support-dirichlet1", {"baseline_supports": supports, "equal_supports": (F(1),) * 4, "marks_differ": True}))

    normals = ((F(1), F(0)), (F(3, 5), F(4, 5)), (F(-3, 5), F(4, 5)), (F(-1), F(0)), (F(0), F(-1)))
    heights = (F(1),) * len(normals)
    tangential_vertices = (
        solve2(normals[0], normals[1], (F(1), F(1))),
        solve2(normals[1], normals[2], (F(1), F(1))),
        solve2(normals[2], normals[3], (F(1), F(1))),
        solve2(normals[3], normals[4], (F(1), F(1))),
        solve2(normals[4], normals[0], (F(1), F(1))),
    )
    assert all(dot(n, x) <= 1 for n in normals for x in tangential_vertices)
    assert all(max(dot(n, x) for x in tangential_vertices) == 1 for n in normals)
    assert all_active(normals, heights)
    out.append(("tangential-to-same-angle-inscribed-polar", {"unit_normals": normals, "tangential_vertices": tangential_vertices, "polar_vertices": normals, "support_equalities": True}))

    at = polygon_area_ordered(tangential_vertices)
    ai = polygon_area_ordered(normals)
    correction_squared = F(1) / (at * ai)
    assert correction_squared * at * ai == 1
    out.append(("area-normalized-polar-scale-correction", {"tangential_area": at, "inscribed_area": ai, "polar_homothety_correction_squared": correction_squared, "symbolic_identity": True}))
    recovered_double_polar = vertices_from_halfspaces(normals, heights)
    assert set(recovered_double_polar) == set(tangential_vertices)
    out.append(("double-polarity-marked-origin", {"original_vertices": tangential_vertices, "recovered_double_polar_vertices": recovered_double_polar, "roundtrip": True}))

    a, b = F(2), F(3)
    scale = ((a, 0, 0, 0), (0, a, 0, 0), (0, 0, b, 0), (0, 0, 0, b))
    assert matmul(transpose(scale), matmul(J, scale)) == tuple(tuple(a * b * x for x in row) for row in J)
    assert determinant(scale) == a * a * b * b
    out.append(("independent-positive-factor-scalings", {"a": a, "b": b, "determinant": determinant(scale), "conformal_symplectic_multiplier": a * b, "normalized_ratios_fixed": True}))

    r = ((F(0), F(-1)), (F(1), F(0)))
    common = ((0, -1, 0, 0), (1, 0, 0, 0), (0, 0, 0, -1), (0, 0, 1, 0))
    points = ((F(1), F(2), F(3), F(4)), (F(-2), F(1), F(5), F(-1)))
    mapped = tuple(matvec(common, p) for p in points)
    assert matmul(transpose(common), common) == I4 and matmul(transpose(common), matmul(J, common)) == J
    assert gram(points, dot) == gram(mapped, dot) and gram(points, omega) == gram(mapped, omega)
    out.append(("simultaneous-common-factor-rotation", {"matrix": common, "orthogonal": True, "symplectic": True, "gram_controls": True}))

    one = ((0, -1, 0, 0), (1, 0, 0, 0), (0, 0, 1, 0), (0, 0, 0, 1))
    one_form = matmul(transpose(one), matmul(J, one))
    assert one_form != J and matmul(transpose(one), one) == I4
    assert omega(points[0], points[1]) != omega(matvec(one, points[0]), matvec(one, points[1]))
    out.append(("one-factor-only-rotation-negative-control", {"matrix": one, "orthogonal": True, "symplectic": False, "signed_omega_changed": True}))

    normals4 = ((F(1), F(0)), (F(0), F(1)), (F(-1), F(0)), (F(0), F(-1)))
    broken4 = (F(3), F(4), F(1), F(2))
    control4 = (F(2), F(3), F(2), F(3))
    t = (F(1), F(1))
    translated = tuple(h - dot(n, t) for n, h in zip(normals4, broken4))
    assert translated == control4 and all_active(normals4, broken4) and all_active(normals4, control4)
    out.append(("four-facet-broken-pairs-width-control", {"translation_subtracted": t, "broken_supports": broken4, "control_supports": control4, "translated_equal": True}))

    normals6 = ((F(1), F(0)), (F(1), F(1)), (F(0), F(1)), (F(-1), F(0)), (F(-1), F(-1)), (F(0), F(-1)))
    control6 = (F(2), F(3), F(2), F(2), F(3), F(2))
    broken6 = (F(5, 2), F(9, 2), F(5, 2), F(3, 2), F(3, 2), F(3, 2))
    t12 = (F(1, 2), F(1, 2))
    assert dot((F(1), F(1)), t12) != F(3, 2)
    assert all_active(normals6, control6) and all_active(normals6, broken6)
    out.append(("six-facet-broken-pairs-not-generally-translation", {"candidate_from_axis_pairs": t12, "required_diagonal_dot": F(3, 2), "actual_diagonal_dot": F(1), "all_facets_active": True}))

    gl = ((4, 0, 0, 0), (0, 2, 0, 0), (0, 0, 1, 0), (0, 0, 0, 2))
    sl = tuple(tuple(F(x, 2) for x in row) for row in gl)
    assert determinant(gl) == 16 and determinant(sl) == 1
    out.append(("gl4-volume-normalization-vs-sl4-representative", {"A": gl, "det_A": 16, "positive_fourth_root": 2, "S": sl, "det_S": 1, "normalized_maps_equal": True}))

    arbitrary_sl = ((2, 0, 0, 0), (0, 1, 0, 0), (0, 0, 1, 0), (0, 0, 0, F(1, 2)))
    assert determinant(arbitrary_sl) == 1 and arbitrary_sl != I4
    out.append(("random-gl4-law-vs-arbitrary-sl4-law", {"normalized_dirac_at_2I": I4, "arbitrary_sl_dirac": arbitrary_sl, "induced_laws_differ": True}))

    u2 = ((0, 0, -1, 0), (0, 1, 0, 0), (1, 0, 0, 0), (0, 0, 0, 1))
    u2_points = tuple(matvec(u2, p) for p in points)
    assert matmul(transpose(u2), u2) == I4 and matmul(transpose(u2), matmul(J, u2)) == J and determinant(u2) == 1
    assert gram(points, dot) == gram(u2_points, dot) and gram(points, omega) == gram(u2_points, omega)
    out.append(("u2-subgroup-sp4-pointwise-orbit", {"matrix": u2, "determinant": 1, "orthogonal": True, "symplectic": True, "gram_controls": True}))

    anti = ((-1, 0, 0, 0), (0, -1, 0, 0), (0, 0, 1, 0), (0, 0, 0, 1))
    anti_points = tuple(matvec(anti, p) for p in points)
    minus_j = tuple(tuple(-x for x in row) for row in J)
    assert matmul(transpose(anti), anti) == I4 and determinant(anti) == 1
    assert matmul(transpose(anti), matmul(J, anti)) == minus_j
    signed_before, signed_after = gram(points, omega), gram(anti_points, omega)
    assert signed_after == tuple(tuple(-x for x in row) for row in signed_before)
    assert tuple(tuple(abs(x) for x in row) for row in signed_after) == tuple(tuple(abs(x) for x in row) for row in signed_before)
    out.append(("antiunitary-antisymplectic-endpoint", {"matrix": anti, "determinant": 1, "orthogonal": True, "anti_symplectic": True, "absolute_omega_fixed": True, "capacity_sys_status": "proof_pending"}))
    out.append(("generic-so4-not-u2-negative-control", {"same_matrix": anti, "in_SO4": True, "in_Sp4": False, "signed_omega_reversed": True}))

    translation = (F(1, 5), F(1, 7))
    shifted_rows = tuple((n[0] / (1 + dot(n, translation)), n[1] / (1 + dot(n, translation))) for n in normals)
    ratios = tuple((1 + dot(n, translation)) for n in normals)
    assert len(set(ratios)) > 1 and shifted_rows != normals
    out.append(("polar-without-origin-mark-negative-control", {"translation": translation, "support_denominators": ratios, "nonuniform_row_change": True}))

    assert {row_id for row_id, _ in out} == {r["row_id"] for r in rows()}
    return [{"row_id": row_id, "status": "pass", "evidence": evidence} for row_id, evidence in out]


def jsonable(value):
    if isinstance(value, F):
        return str(value.numerator) if value.denominator == 1 else f"{value.numerator}/{value.denominator}"
    if isinstance(value, tuple):
        return [jsonable(x) for x in value]
    if isinstance(value, list):
        return [jsonable(x) for x in value]
    if isinstance(value, dict):
        return {k: jsonable(v) for k, v in value.items()}
    return value


def json_bytes(value):
    return (json.dumps(jsonable(value), indent=2, sort_keys=True) + "\n").encode()


def validate_matrix(matrix_rows, witnesses):
    ids = [r["row_id"] for r in matrix_rows]
    assert ids == sorted(ids) or len(ids) == len(set(ids))
    assert len(ids) == len(set(ids))
    assert all(r["level"] in LEVELS for r in matrix_rows)
    assert all(set(r["expected"]) == set(VIEWS) for r in matrix_rows)
    assert all(set(r["expected"].values()) <= OUTCOMES for r in matrix_rows)
    assert {w["row_id"] for w in witnesses} == set(ids)
    assert all(w["status"] == "pass" for w in witnesses)
    assert any(r["level"] == "not_equivalent" for r in matrix_rows)
    assert any("proof_pending" in r["proof_status"] for r in matrix_rows)
    assert not any("capacity" in r["expected"] or "sys" in r["expected"] for r in matrix_rows)


def artifact_payloads():
    matrix_rows = rows()
    witnesses = witness_results()
    validate_matrix(matrix_rows, witnesses)
    level_counts = {level: sum(r["level"] == level for r in matrix_rows) for level in sorted(LEVELS)}
    status_counts = {}
    control_status_counts = {}
    for r in matrix_rows:
        status_counts[r["proof_status"]] = status_counts.get(r["proof_status"], 0) + 1
        status = r["executable_control_status"]
        control_status_counts[status] = control_status_counts.get(status, 0) + 1
    matrix = {
        "schema": SCHEMA,
        "complete": True,
        "target_free": True,
        "views": list(VIEWS),
        "view_definitions": VIEW_DEFINITIONS,
        "outcome_vocabulary": sorted(OUTCOMES),
        "row_count": len(matrix_rows),
        "counts_by_level": level_counts,
        "counts_by_proof_status": dict(sorted(status_counts.items())),
        "counts_by_executable_control_status": dict(sorted(control_status_counts.items())),
        "rows": matrix_rows,
    }
    tsv = io.StringIO()
    fields = ["row_id", "objects_laws", "level", "hypotheses_conditioning", "transformation", *VIEWS, "proof_status", "proof_source", "arithmetic", "executable_control_status", "executable_control", "collapse_scope"]
    writer = csv.DictWriter(tsv, fieldnames=fields, dialect="excel-tab", lineterminator="\n")
    writer.writeheader()
    for row in matrix_rows:
        flat = {k: row[k] for k in fields if k in row}
        flat.update(row["expected"])
        flat["proof_source"] = " | ".join(row["proof_source"])
        writer.writerow(flat)
    return {
        "matrix.json": json_bytes(matrix),
        "matrix.tsv": tsv.getvalue().encode(),
        "witnesses.json": json_bytes({"schema": SCHEMA + "-witnesses", "complete": True, "witness_count": len(witnesses), "witnesses": witnesses}),
    }


def provenance(payloads, command, expected_revision):
    head = run_git("rev-parse", "HEAD")
    if head != expected_revision:
        raise SystemExit(f"HEAD {head} does not equal --expected-revision {expected_revision}")
    if subprocess.run(["git", "diff", "--quiet", "--ignore-submodules", "--"], cwd=REPO).returncode or subprocess.run(["git", "diff", "--cached", "--quiet", "--ignore-submodules", "--"], cwd=REPO).returncode:
        raise SystemExit("tracked worktree must be clean before artifact generation")
    inputs = []
    for relative in SOURCE_PATHS:
        data = (REPO / relative).read_bytes()
        inputs.append({"path": relative, "sha256": sha256(data), "bytes": len(data)})
    producer = Path(__file__).read_bytes()
    return {
        "schema": SCHEMA + "-provenance",
        "complete": True,
        "command": command,
        "source_revision": head,
        "source_repository_tree": run_git("rev-parse", "HEAD^{tree}"),
        "source_tracked_clean": True,
        "untracked_files_ignored_by_clean_predicate": True,
        "producer": str(Path(__file__).relative_to(REPO)),
        "producer_sha256": sha256(producer),
        "source_inputs": inputs,
        "artifacts": {name: {"sha256": sha256(data), "bytes": len(data)} for name, data in sorted(payloads.items())},
        "independence_unit": "one deterministic synthetic witness per matrix row; probability-law rows are theorem-backed bijection controls, not Monte Carlo samples",
        "interpretation_boundary": "Target-free regression infrastructure only. It supports only the named algebraic, marginal, pushforward, orbit, and negative-control statements; it contains no sys/capacity evaluation or population comparison.",
    }


def write(args):
    payloads = artifact_payloads()
    args.out_dir.mkdir(parents=True, exist_ok=True)
    command = f"{Path(__file__).relative_to(REPO)} --out-dir {args.out_dir.relative_to(REPO)} --expected-revision {args.expected_revision}"
    prov = provenance(payloads, command, args.expected_revision)
    for name, data in payloads.items():
        (args.out_dir / name).write_bytes(data)
    (args.out_dir / "provenance.json").write_bytes(json_bytes(prov))


def check(args):
    payloads = artifact_payloads()
    prov_path = args.out_dir / "provenance.json"
    prov = json.loads(prov_path.read_text())
    failures = []
    for name, expected_data in payloads.items():
        actual = (args.out_dir / name).read_bytes()
        if actual != expected_data:
            failures.append(f"{name}: deterministic replay mismatch")
        if sha256(actual) != prov["artifacts"][name]["sha256"]:
            failures.append(f"{name}: provenance hash mismatch")
    if sha256(Path(__file__).read_bytes()) != prov["producer_sha256"]:
        failures.append("producer hash mismatch")
    for source in prov["source_inputs"]:
        if sha256((REPO / source["path"]).read_bytes()) != source["sha256"]:
            failures.append(f"source input changed: {source['path']}")
    if subprocess.run(["git", "merge-base", "--is-ancestor", prov["source_revision"], "HEAD"], cwd=REPO).returncode:
        failures.append("provenance source revision is not an ancestor of HEAD")
    if failures:
        raise SystemExit("\n".join(failures))
    print(f"PASS: {len(rows())} rows and {len(witness_results())} witnesses replay byte-identically")


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--out-dir", type=Path, default=PACKET / "artifacts")
    parser.add_argument("--expected-revision")
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    args.out_dir = args.out_dir.resolve()
    if args.check:
        check(args)
    elif not args.expected_revision:
        parser.error("--expected-revision is required for generation")
    else:
        write(args)


if __name__ == "__main__":
    main()
