#!/usr/bin/env python3
# /// script
# dependencies = ["numpy"]
# ///
"""Build target-free exact-geometry feature rows for reviewed generator panels.

The inputs are source rows, not a new sampler.  Orientation geometry is read
from the reviewed row payload.  Tangential geometry is read only from a
geometry-sidecar replay produced by the same deterministic producer command;
the row identity and latent pairing are checked before any feature is used.
"""

from __future__ import annotations

import argparse
import itertools
import json
import math
from collections import Counter
from fractions import Fraction
from pathlib import Path
from typing import Any

import numpy as np

ROW_SCHEMA = "generator-exact-feature-augmenter-row-v1"
ORIENTATION_SCHEMA = "generator-orientation-smoke-row-v2"
TANGENTIAL_SCHEMA = "alternative-generator-smoke-row-v2"
VARIANTS = {"identity", "u2-deterministic", "u2-haar", "so4-deterministic", "so4-haar"}
ARMS = {"factorial-baseline", "factorial-q", "factorial-p", "factorial-both"}
ORIENTATION_BUCKETS = {"3x3", "4x4", "4x6", "6x6"}
TANGENTIAL_BUCKETS = {"3x3", "4x6", "6x6"}


class AnalysisError(ValueError):
    pass


def rat(value: str | int | float) -> Fraction:
    if isinstance(value, str):
        if "/" in value:
            a, b = value.split("/", 1)
            return Fraction(int(a), int(b))
        return Fraction(value)
    return Fraction(value)


def _matrix(payload: Any, label: str) -> list[list[Fraction]]:
    if not isinstance(payload, list) or not payload or any(not isinstance(v, list) or len(v) != 4 for v in payload):
        raise AnalysisError(f"{label} must be a nonempty list of 4-vectors")
    try:
        return [[rat(x) for x in vertex] for vertex in payload]
    except (ValueError, ZeroDivisionError) as exc:
        raise AnalysisError(f"{label} contains invalid rational values") from exc


def omega(a: list[Fraction], b: list[Fraction]) -> Fraction:
    return a[0] * b[2] - a[2] * b[0] + a[1] * b[3] - a[3] * b[1]


def _incidence_from_signature(signature: Any, vertex_count: int, facet_count: int) -> list[list[bool]]:
    if not isinstance(signature, list) or len(signature) != vertex_count:
        raise AnalysisError("orientation incidence signature does not match primal vertices")
    out = []
    for facets in signature:
        if not isinstance(facets, list) or any(not isinstance(i, int) or not 0 <= i < facet_count for i in facets):
            raise AnalysisError("orientation incidence signature contains an invalid facet")
        row = [False] * facet_count
        for i in facets:
            if row[i]:
                raise AnalysisError("orientation incidence signature contains a duplicate facet")
            row[i] = True
        out.append(row)
    return out


def _check_geometry(duals: list[list[Fraction]], vertices: list[list[Fraction]], incidence: list[list[bool]], source: dict[str, Any]) -> None:
    if len(incidence) != len(vertices) or any(len(row) != len(duals) for row in incidence):
        raise AnalysisError("geometry incidence dimensions do not match exact payload")
    if source.get("facet_count") is not None and source["facet_count"] != len(duals):
        raise AnalysisError("source facet count disagrees with geometry payload")
    if source.get("vertex_count") is not None and source["vertex_count"] != len(vertices):
        raise AnalysisError("source vertex count disagrees with geometry payload")
    for v, flags in zip(vertices, incidence):
        for a, is_incident in zip(duals, flags):
            dot = sum(x * y for x, y in zip(a, v))
            if is_incident and dot != 1:
                raise AnalysisError("incident exact primal/dual join is not equality")
            if not is_incident and dot > 1:
                raise AnalysisError("nonincident exact primal/dual join violates half-space")


def _two_faces(incidence: list[list[bool]]) -> list[tuple[tuple[int, int], list[int]]]:
    facets = len(incidence[0])
    out = []
    for left in range(facets):
        for right in range(left + 1, facets):
            vertices = [i for i, row in enumerate(incidence) if row[left] and row[right]]
            if len(vertices) < 3:
                continue
            degrees = []
            for vi in vertices:
                degree = sum(
                    1
                    for vj in vertices
                    if vj != vi and any(
                        incidence[vi][k] and incidence[vj][k]
                        for k in range(facets)
                        if k not in (left, right)
                    )
                )
                degrees.append(degree)
            if all(d == 2 for d in degrees):
                out.append(((left, right), vertices))
    return out


def _ordered_face(face: tuple[tuple[int, int], list[int]], incidence: list[list[bool]]) -> list[int] | None:
    facets, vertices = face
    neighbors: dict[int, list[int]] = {v: [] for v in vertices}
    for i, left in enumerate(vertices):
        for right in vertices[i + 1 :]:
            shared = any(incidence[left][k] and incidence[right][k] for k in range(len(incidence[0])) if k not in facets)
            if shared:
                neighbors[left].append(right)
                neighbors[right].append(left)
    if any(len(n) != 2 for n in neighbors.values()):
        return None
    order = [vertices[0]]
    previous, current = -1, vertices[0]
    while True:
        choices = [x for x in neighbors[current] if x != previous]
        if not choices:
            return None
        nxt = choices[0]
        if nxt == order[0]:
            break
        if nxt in order:
            return None
        order.append(nxt)
        previous, current = current, nxt
    return order if len(order) == len(vertices) else None


def _euclidean_area(points: list[list[Fraction]]) -> float:
    if len(points) < 3:
        return 0.0
    # The norm of the 4D bivector integral is twice the polygon area.
    biv = [Fraction(0) for _ in range(6)]
    pairs = [(i, j) for i in range(4) for j in range(i + 1, 4)]
    for a, b in zip(points, points[1:] + points[:1]):
        for k, (i, j) in enumerate(pairs):
            biv[k] += a[i] * b[j] - a[j] * b[i]
    return 0.5 * math.sqrt(sum(float(x * x) for x in biv))


def _summary(values: list[float], prefix: str) -> dict[str, float | int | None]:
    if not values:
        return {f"{prefix}_{name}": None for name in ("mean", "std", "min", "q25", "median", "q75", "q90", "q95", "max", "sum")}
    a = np.asarray(values, dtype=float)
    return {
        f"{prefix}_mean": float(np.mean(a)), f"{prefix}_std": float(np.std(a)),
        f"{prefix}_min": float(np.min(a)), f"{prefix}_q25": float(np.quantile(a, .25)),
        f"{prefix}_median": float(np.quantile(a, .5)), f"{prefix}_q75": float(np.quantile(a, .75)),
        f"{prefix}_q90": float(np.quantile(a, .9)), f"{prefix}_q95": float(np.quantile(a, .95)),
        f"{prefix}_max": float(np.max(a)), f"{prefix}_sum": float(np.sum(a)),
    }


def _covariance(vertices: list[list[Fraction]], expected: int) -> dict[str, Any]:
    unique = sorted({tuple(float(x) for x in v) for v in vertices})
    out: dict[str, Any] = {"distinct_vertex_count": len(unique), "expected_vertex_count": expected, "status": "ineligible"}
    if len(unique) != expected or len(unique) < 2:
        out["status"] = "unexpected_distinct_vertex_count"
        return out
    x = np.asarray(unique, dtype=float)
    cov = np.cov(x, rowvar=False, bias=True)
    eig = np.linalg.eigvalsh(cov)
    lo, hi = float(np.min(eig)), float(np.max(eig))
    out.update({"ordinary_eigenvalue_min": lo, "ordinary_eigenvalue_max": hi})
    if not (math.isfinite(lo) and lo > 0):
        out["status"] = "covariance_not_positive_definite"
        return out
    out["condition"] = hi / lo
    j = np.array([[0, 0, 1, 0], [0, 0, 0, 1], [-1, 0, 0, 0], [0, -1, 0, 0]], dtype=float)
    s = float(-0.5 * np.trace((j @ cov) @ (j @ cov)))
    p = float(np.linalg.det(cov))
    disc = s * s - 4 * p
    if not (math.isfinite(s) and math.isfinite(p) and s > 0 and p > 0 and disc >= -1e-12 * max(abs(s * s), abs(4 * p), 1)):
        out["status"] = "unstable_williamson_invariants"
        return out
    n2sq = .5 * (s + math.sqrt(max(0.0, disc)))
    n1sq = p / n2sq
    n1, n2 = math.sqrt(n1sq), math.sqrt(n2sq)
    out.update({"nu1": n1, "nu2": n2, "rho": n2 / n1, "status": "eligible"})
    return out


def _strict_cycles(duals: list[list[Fraction]]) -> dict[str, Any]:
    signs = [[(omega(duals[i], duals[3 + j]) > 0) - (omega(duals[i], duals[3 + j]) < 0) for j in range(3)] for i in range(3)]
    strict_cell = all(s != 0 for row in signs for s in row)
    words = []
    for qrest in itertools.permutations((1, 2)):
        q = (0,) + qrest
        for p in itertools.permutations((0, 1, 2)):
            word = tuple(x for pair in zip(q, p) for x in (pair[0], pair[1] + 3))
            ok = True
            for k in range(6):
                a, b = word[k], word[(k + 1) % 6]
                if a < 3 <= b:
                    ok &= omega(duals[a], duals[b]) > 0
                elif b < 3 <= a:
                    ok &= omega(duals[b], duals[a]) < 0
                else:
                    ok = False
            if ok:
                words.append(word)
    return {"strict_sign_cell": strict_cell, "strict_cycle_feasible": bool(words), "strict_cycle_count": len(words), "strict_signs": signs}


def feature_row(source: dict[str, Any], source_kind: str) -> dict[str, Any]:
    for key in ("capacity", "sys", "iterations", "iteration", "bounce_label", "target"):
        if source.get(key) is not None:
            raise AnalysisError(f"target field {key} present in exact-feature input")
    if source.get("target_ms", 0) not in (0, 0.0, None):
        raise AnalysisError("target execution time present in exact-feature input")
    if source_kind == "orientation":
        if source.get("schema") != ORIENTATION_SCHEMA or source.get("base_accepted") is not True:
            raise AnalysisError("orientation input is not an accepted reviewed row")
        duals = _matrix(source.get("transformed_dual_vertices_rational"), "orientation dual vertices")
        vertices = _matrix(source.get("reconstructed_primal_vertices_rational"), "orientation primal vertices")
        incidence = _incidence_from_signature(source.get("labeled_incidence_signature"), len(vertices), len(duals))
        if not isinstance(source.get("transformed_id"), str) or not isinstance(source.get("transformed_geometry_id"), str):
            raise AnalysisError("orientation source lacks transformed geometry identity")
        strict_allowed = source.get("bucket") == "3x3" and source.get("map_variant") == "identity"
        bucket = source.get("bucket")
        identity = source.get("transformed_id")
    else:
        if source.get("schema") != TANGENTIAL_SCHEMA or source.get("accepted") is not True or source.get("validation_status") != "survived":
            raise AnalysisError("tangential input is not an accepted geometry row")
        duals = _matrix(source.get("geometry_dual_vertices_rational"), "tangential sidecar dual vertices")
        vertices = _matrix(source.get("geometry_primal_vertices_rational"), "tangential sidecar primal vertices")
        incidence = source.get("geometry_vertex_facet_incidence")
        if not isinstance(incidence, list):
            raise AnalysisError("tangential row lacks geometry-sidecar incidence")
        incidence = [[bool(x) for x in row] for row in incidence]
        strict_allowed = source.get("pair_bucket") == "3x3" and source.get("law") == "factorial-baseline"
        bucket = source.get("pair_bucket")
        identity = source.get("sample_id")
        if source.get("geometry_volume") is None or abs(float(source["geometry_volume"]) - float(source["volume"])) > 1e-12:
            raise AnalysisError("tangential sidecar volume does not join source row")
        if source.get("geometry_source_sample_id") != source.get("sample_id") or source.get("geometry_source_pairing_id") != source.get("pairing_id"):
            raise AnalysisError("tangential geometry sidecar identity does not join source row")
    _check_geometry(duals, vertices, incidence, source)
    volume = float(source.get("exact_volume_as_f64", source.get("volume")))
    if not math.isfinite(volume) or volume <= 0:
        raise AnalysisError("source volume must be positive and finite")
    faces = _two_faces(incidence)
    eucl, symp, kappas, failures = [], [], [], 0
    for face in faces:
        order = _ordered_face(face, incidence)
        if order is None:
            failures += 1
            continue
        points = [vertices[i] for i in order]
        e = _euclidean_area(points)
        s = abs(float(sum(omega(points[i], points[(i + 1) % len(points)]) for i in range(len(points))) / 2))
        eucl.append(e); symp.append(s)
        kappas.append(s / e if e > 0 else None)
    kappas_f = [x for x in kappas if x is not None]
    sqrt_v = math.sqrt(volume)
    row: dict[str, Any] = {
        "schema": ROW_SCHEMA, "source_kind": source_kind, "source_id": identity,
        "source_sample_id": source.get("sample_id"), "source_pairing_id": source.get("pairing_id"),
        "bucket": bucket, "facet_count": len(duals), "vertex_count": len(vertices), "two_face_count": len(faces),
        "map_variant": source.get("map_variant") if source_kind == "orientation" else None,
        "law": source.get("law") if source_kind == "tangential" else None,
        "ordered_two_face_count": len(eucl), "ordering_failure_count": failures,
        "geometry_validation_status": "validated", "coordinate_order": "q1,q2,p1,p2",
        "volume": volume, "volume_sqrt": sqrt_v,
        "strict_cycle": _strict_cycles(duals) if strict_allowed else None,
    }
    row.update({f"euclidean_ridge_area_{k}": v / sqrt_v if v is not None else None for k, v in _summary(eucl, "x").items()})
    row.update({f"symplectic_ridge_area_{k}": v / sqrt_v if v is not None else None for k, v in _summary(symp, "x").items()})
    row.update({f"kappa_{k}": v for k, v in _summary(kappas_f, "x").items()})
    paired = [(e, k) for e, k in zip(eucl, kappas) if k is not None]
    weighted = float(sum(e * k for e, k in paired) / sum(e for e, _ in paired)) if paired else None
    cov = float(np.cov(np.asarray([e for e, _ in paired]), np.asarray([k for _, k in paired]), bias=True)[0, 1]) if len(paired) > 1 else None
    err = max((abs(s - e * k) for e, s, k in zip(eucl, symp, kappas) if k is not None), default=0.0)
    row.update({"kappa_euclidean_weighted_mean": weighted, "kappa_euclidean_covariance": cov, "decomposition_max_abs_error": err, "decomposition_identity_ok": err <= 1e-10})
    expected = source.get("q_sides", 0) * source.get("p_sides", 0) if source_kind == "orientation" else len(vertices)
    row["vertex_covariance"] = _covariance(vertices, int(expected))
    return row


def load_rows(path: Path) -> list[dict[str, Any]]:
    payload = path.read_bytes()
    if not payload.endswith(b"\n"):
        raise AnalysisError(f"{path}: no final newline")
    rows = []
    seen = set()
    for line_number, line in enumerate(payload.splitlines(), 1):
        try:
            row = json.loads(line)
        except json.JSONDecodeError as exc:
            raise AnalysisError(f"{path}:{line_number}: invalid JSON") from exc
        identity = row.get("sample_id", row.get("source_id")) if isinstance(row, dict) else None
        if not isinstance(row, dict) or not isinstance(identity, str) or identity in seen:
            raise AnalysisError(f"{path}:{line_number}: duplicate or invalid source identity")
        seen.add(identity)
        rows.append(row)
    if not rows:
        raise AnalysisError(f"{path}: empty input")
    return rows


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--orientation", type=Path)
    parser.add_argument("--tangential", type=Path)
    parser.add_argument("--out-dir", type=Path, required=True)
    args = parser.parse_args()
    if not args.orientation and not args.tangential:
        parser.error("at least one source panel is required")
    out = args.out_dir; out.mkdir(parents=True, exist_ok=True)
    rows = []
    if args.orientation:
        rows.extend(feature_row(r, "orientation") for r in load_rows(args.orientation))
    if args.tangential:
        rows.extend(feature_row(r, "tangential") for r in load_rows(args.tangential))
    with (out / "features.jsonl").open("w") as f:
        for row in rows:
            f.write(json.dumps(row, sort_keys=True, allow_nan=False) + "\n")
    (out / "augment-report.json").write_text(json.dumps({"schema": "generator-exact-feature-augmenter-report-v1", "rows": len(rows), "source_kinds": dict(Counter(r["source_kind"] for r in rows)), "target_free": True}, indent=2) + "\n")


if __name__ == "__main__":
    main()
