#!/usr/bin/env python3
# /// script
# dependencies = ["numpy"]
# ///
"""Exact-geometry, target-free feature augmentation for reviewed panels."""

from __future__ import annotations

import argparse
import hashlib
import itertools
import json
import math
import subprocess
from collections import Counter
from fractions import Fraction
from pathlib import Path
from typing import Any

import numpy as np

ROW_SCHEMA = "generator-exact-feature-augmenter-row-v2"
ORIENTATION_SCHEMA = "generator-orientation-smoke-row-v2"
TANGENTIAL_SCHEMA = "alternative-generator-smoke-row-v2"
VARIANTS = {"identity", "u2-deterministic", "u2-haar", "so4-deterministic", "so4-haar"}
ARMS = {"factorial-baseline", "factorial-q", "factorial-p", "factorial-both"}
ORIENTATION_BUCKETS = {"3x3", "4x4", "4x6", "6x6"}
TANGENTIAL_BUCKETS = {"3x3", "4x6", "6x6"}
ALLOWED_COVARIANCE_STATUSES = {"eligible", "unexpected_distinct_vertex_count", "insufficient_or_nonfinite_vertices", "covariance_not_positive_definite", "ordinary_condition_exceeds_limit", "unstable_williamson_invariants", "unstable_williamson_roots", "nonfinite_rho"}
FACE_FEATURES = ("mean", "std", "min", "q25", "median", "q75", "q90", "q95", "max", "sum")
SYMP_EXTRA = ("max_share", "top3_share", "entropy", "effective_face_count", "normalized_entropy")
TIMING_KEYS = {"generation_ms", "validation_ms", "target_ms", "geometry_generation_ms", "geometry_validation_ms"}
CONTRACT_KEYS = (
    "law", "wishlist_item", "law_version", "identity_scope", "seed", "row_index", "attempt", "attempts",
    "rejections", "parameter", "pair_bucket", "facet_count", "accepted", "validation_status",
    "rejection_reason", "factor_q_area", "factor_p_area", "factor_q_support_cv", "factor_p_support_cv",
    "factor_q_gap_cv", "factor_p_gap_cv", "factor_q_isoperimetric_ratio", "factor_p_isoperimetric_ratio",
    "pairing_id", "volume", "capacity", "sys", "iterations", "iteration", "bounce_label", "target", "target_ms",
)


class AnalysisError(ValueError):
    pass


def rat(value: str | int | float) -> Fraction:
    if isinstance(value, str):
        return Fraction(value) if "/" not in value else Fraction(*map(int, value.split("/", 1)))
    return Fraction(value)


def _matrix(payload: Any, label: str) -> list[list[Fraction]]:
    if not isinstance(payload, list) or not payload or any(not isinstance(v, list) or len(v) != 4 for v in payload):
        raise AnalysisError(f"{label} must be a nonempty list of 4-vectors")
    try:
        return [[rat(x) for x in v] for v in payload]
    except (ValueError, ZeroDivisionError) as exc:
        raise AnalysisError(f"{label} contains invalid rational values") from exc


def omega(a: list[Fraction], b: list[Fraction]) -> Fraction:
    return a[0] * b[2] - a[2] * b[0] + a[1] * b[3] - a[3] * b[1]


def _incidence_signature(signature: Any, nv: int, nf: int) -> list[list[bool]]:
    if not isinstance(signature, list) or len(signature) != nv:
        raise AnalysisError("incidence signature does not match primal vertices")
    out = []
    for facets in signature:
        if not isinstance(facets, list) or any(not isinstance(i, int) or not 0 <= i < nf for i in facets):
            raise AnalysisError("incidence signature contains an invalid facet")
        row = [False] * nf
        for i in facets:
            if row[i]:
                raise AnalysisError("incidence signature contains a duplicate facet")
            row[i] = True
        out.append(row)
    return out


def _check_geometry(duals, vertices, incidence, source) -> None:
    if len(incidence) != len(vertices) or any(len(r) != len(duals) for r in incidence):
        raise AnalysisError("geometry incidence dimensions do not match exact payload")
    if source.get("facet_count") is not None and source["facet_count"] != len(duals):
        raise AnalysisError("source facet count disagrees with geometry payload")
    for v, flags in zip(vertices, incidence):
        for a, incident in zip(duals, flags):
            dot = sum(x * y for x, y in zip(a, v))
            if incident and dot != 1:
                raise AnalysisError("incident exact primal/dual join is not equality")
            if not incident and dot >= 1:
                raise AnalysisError("nonincident exact primal/dual join has unflagged equality")


def _two_faces(incidence: list[list[bool]]) -> list[tuple[tuple[int, int], list[int]]]:
    """Enumerate every facet pair with at least three common vertices.

    A pair whose induced graph is not a cycle remains in the population and is
    reported by `_ordered_face` as an ordering failure rather than disappearing.
    """
    nf = len(incidence[0]); out = []
    for left in range(nf):
        for right in range(left + 1, nf):
            vs = [i for i, row in enumerate(incidence) if row[left] and row[right]]
            if len(vs) >= 3:
                out.append(((left, right), vs))
    return out


def _ordered_face(face, incidence) -> list[int] | None:
    facets, vertices = face
    neighbors = {v: [] for v in vertices}
    for i, left in enumerate(vertices):
        for right in vertices[i + 1 :]:
            if any(incidence[left][k] and incidence[right][k] for k in range(len(incidence[0])) if k not in facets):
                neighbors[left].append(right); neighbors[right].append(left)
    if any(len(n) != 2 for n in neighbors.values()):
        return None
    order = [vertices[0]]; previous, current = -1, vertices[0]
    while True:
        choices = [x for x in neighbors[current] if x != previous]
        if not choices: return None
        nxt = choices[0]
        if nxt == order[0]: break
        if nxt in order: return None
        order.append(nxt); previous, current = current, nxt
    return order if len(order) == len(vertices) else None


def _euclidean_area(points) -> float:
    if len(points) < 3: return 0.0
    pairs = [(i, j) for i in range(4) for j in range(i + 1, 4)]
    biv = [Fraction(0) for _ in pairs]
    for a, b in zip(points, points[1:] + points[:1]):
        for k, (i, j) in enumerate(pairs): biv[k] += a[i] * b[j] - a[j] * b[i]
    return 0.5 * math.sqrt(sum(float(x * x) for x in biv))


def _summary(values: list[float]) -> dict[str, float | None]:
    if not values: return {k: None for k in FACE_FEATURES}
    a = np.asarray(values, dtype=float)
    return {"mean": float(np.mean(a)), "std": float(np.std(a)), "min": float(np.min(a)), "q25": float(np.quantile(a, .25)), "median": float(np.quantile(a, .5)), "q75": float(np.quantile(a, .75)), "q90": float(np.quantile(a, .9)), "q95": float(np.quantile(a, .95)), "max": float(np.max(a)), "sum": float(np.sum(a))}


def _covariance(vertices: list[list[Fraction]], expected: int) -> dict[str, Any]:
    unique = sorted({tuple(float(x) for x in v) for v in vertices})
    out: dict[str, Any] = {"distinct_vertex_count": len(unique), "expected_vertex_count": expected, "status": "ineligible"}
    if len(unique) != expected: out["status"] = "unexpected_distinct_vertex_count"; return out
    if len(unique) < 2 or any(not math.isfinite(x) for v in unique for x in v): out["status"] = "insufficient_or_nonfinite_vertices"; return out
    x = np.asarray(unique, dtype=float); c = np.cov(x, rowvar=False, bias=True); eig = np.linalg.eigvalsh(c)
    lo, hi = float(np.min(eig)), float(np.max(eig)); out.update(ordinary_eigenvalue_min=lo, ordinary_eigenvalue_max=hi)
    if not (math.isfinite(lo) and math.isfinite(hi) and lo > 0): out["status"] = "covariance_not_positive_definite"; return out
    condition = hi / lo; out["condition"] = condition
    if not math.isfinite(condition) or condition > 1e10: out["status"] = "ordinary_condition_exceeds_limit"; return out
    j = np.array([[0,0,1,0],[0,0,0,1],[-1,0,0,0],[0,-1,0,0]], dtype=float); jc = j @ c
    s, p = float(-.5 * np.trace(jc @ jc)), float(np.linalg.det(c)); disc = s*s - 4*p
    scale = max(abs(s*s), abs(4*p), 1.0)
    if not (math.isfinite(s) and math.isfinite(p) and s > 0 and p > 0 and disc >= -1e-12 * scale): out["status"] = "unstable_williamson_invariants"; return out
    n2sq = .5 * (s + math.sqrt(max(0.0, disc))); n1sq = p / n2sq
    if not (math.isfinite(n1sq) and math.isfinite(n2sq) and n1sq > 0 and n2sq >= n1sq): out["status"] = "unstable_williamson_roots"; return out
    n1, n2 = math.sqrt(n1sq), math.sqrt(n2sq); rho = n2 / n1
    if not all(math.isfinite(z) for z in (n1, n2, rho)): out["status"] = "nonfinite_rho"; return out
    out.update(nu1=n1, nu2=n2, rho=rho, status="eligible"); return out


def _strict_cycles(duals):
    signs = [[(omega(duals[i], duals[3+j]) > 0) - (omega(duals[i], duals[3+j]) < 0) for j in range(3)] for i in range(3)]
    words = []
    for qr in itertools.permutations((1,2)):
        q = (0,) + qr
        for p in itertools.permutations((0,1,2)):
            word = tuple(x for pair in zip(q,p) for x in (pair[0], pair[1]+3)); ok = True
            for k in range(6):
                a,b = word[k], word[(k+1)%6]
                if a < 3 <= b: ok &= omega(duals[a], duals[b]) > 0
                elif b < 3 <= a: ok &= omega(duals[b], duals[a]) < 0
                else: ok = False
            if ok: words.append(word)
    return {"strict_sign_cell": all(s != 0 for r in signs for s in r), "strict_cycle_feasible": bool(words), "strict_cycle_count": len(words), "strict_signs": signs}


def _target_guard(row: dict[str, Any]) -> None:
    for key in ("capacity", "sys", "iterations", "iteration", "bounce_label", "target"):
        if row.get(key) is not None: raise AnalysisError(f"target field {key} present")
    if row.get("target_ms", 0) not in (None, 0, 0.0): raise AnalysisError("target execution time present")


def feature_row(source: dict[str, Any], source_kind: str, geometry: dict[str, Any] | None = None) -> dict[str, Any]:
    _target_guard(source)
    if source_kind == "orientation":
        if source.get("schema") != ORIENTATION_SCHEMA or source.get("base_accepted") is not True: raise AnalysisError("orientation input is not accepted")
        duals = _matrix(source.get("transformed_dual_vertices_rational"), "orientation dual vertices"); vertices = _matrix(source.get("reconstructed_primal_vertices_rational"), "orientation primal vertices")
        incidence = _incidence_signature(source.get("labeled_incidence_signature"), len(vertices), len(duals)); source_id = source.get("transformed_id")
        if not isinstance(source_id, str) or not isinstance(source.get("transformed_geometry_id"), str): raise AnalysisError("orientation source lacks geometry identity")
        strict = source.get("bucket") == "3x3" and source.get("map_variant") == "identity"; bucket = source.get("bucket"); base_id = source.get("base_id")
        expected = int(source.get("q_sides", 0)) * int(source.get("p_sides", 0))
    else:
        if geometry is None: raise AnalysisError("tangential geometry requires separate replay")
        _target_guard(geometry)
        if source.get("schema") != TANGENTIAL_SCHEMA or source.get("accepted") is not True or source.get("validation_status") != "survived": raise AnalysisError("tangential authoritative row is not accepted")
        duals = _matrix(geometry.get("geometry_dual_vertices_rational"), "tangential sidecar dual vertices"); vertices = _matrix(geometry.get("geometry_primal_vertices_rational"), "tangential sidecar primal vertices")
        raw_incidence = geometry.get("geometry_vertex_facet_incidence")
        if not isinstance(raw_incidence, list) or any(not isinstance(r,list) or any(type(x) is not bool for x in r) for r in raw_incidence): raise AnalysisError("tangential replay incidence entries must be literal bool")
        incidence = raw_incidence
        if incidence is None: raise AnalysisError("tangential replay lacks incidence")
        if geometry.get("geometry_source_sample_id") != source.get("sample_id") or geometry.get("geometry_source_pairing_id") != source.get("pairing_id"): raise AnalysisError("tangential replay identity does not join authoritative row")
        if geometry.get("geometry_volume") is None or abs(float(geometry["geometry_volume"]) - float(source["volume"])) > 1e-12: raise AnalysisError("tangential replay volume mismatch")
        strict = source.get("pair_bucket") == "3x3" and source.get("law") == "factorial-baseline"; bucket = source.get("pair_bucket"); base_id = source.get("pairing_id"); source_id = source.get("sample_id")
        try: expected = int(str(bucket).split("x", 1)[0]) * int(str(bucket).split("x", 1)[1])
        except (ValueError, IndexError): raise AnalysisError("tangential bucket is not kxm")
    _check_geometry(duals, vertices, incidence, source)
    if len(vertices) != expected: raise AnalysisError("primal vertex count does not equal product k*m")
    if source_kind == "orientation" and source.get("vertex_count") != len(vertices): raise AnalysisError("orientation source vertex_count disagrees with geometry")
    if any(sum(row) != 4 for row in incidence) or any(sum(row[j] for row in incidence) == 0 for j in range(len(duals))): raise AnalysisError("geometry is not a simple product incidence")
    volume = float(source.get("exact_volume_as_f64", source.get("volume")))
    if not math.isfinite(volume) or volume <= 0: raise AnalysisError("volume must be positive finite")
    faces = _two_faces(incidence)
    expected_faces = expected + len(duals) if source_kind == "orientation" else expected + len(duals)
    if len(faces) != expected_faces: raise AnalysisError(f"expected product two-face count {expected_faces}, found {len(faces)}")
    eucl=[]; symp=[]; kappas=[]; failures=0
    for face in faces:
        order = _ordered_face(face, incidence)
        if order is None: failures += 1; continue
        points=[vertices[i] for i in order]; e=_euclidean_area(points); s=abs(float(sum(omega(points[i],points[(i+1)%len(points)]) for i in range(len(points)))/2)); eucl.append(e); symp.append(s); kappas.append(s/e if e > 0 else None)
    paired=[(e,k) for e,k in zip(eucl,kappas) if k is not None]; sqrt_v=math.sqrt(volume); row={
        "schema": ROW_SCHEMA, "source_kind": source_kind, "source_id": source_id, "source_sample_id": source.get("sample_id"), "source_pairing_id": source.get("pairing_id") or (source.get("base_id") if source_kind == "orientation" else None), "base_id": base_id, "bucket": bucket, "map_variant": source.get("map_variant") if source_kind == "orientation" else None, "law": source.get("law") if source_kind == "tangential" else None,
        "facet_count":len(duals), "vertex_count":len(vertices), "two_face_count":len(faces), "ordered_two_face_count":len(eucl), "ordering_failure_count":failures, "geometry_validation_status":"validated", "coordinate_order":"q1,q2,p1,p2", "volume":volume, "volume_sqrt":sqrt_v, "strict_cycle":_strict_cycles(duals) if strict else None,
    }
    for prefix, values in (("euclidean_ridge_area", [x/sqrt_v for x in eucl]), ("symplectic_ridge_area", [x/sqrt_v for x in symp]), ("kappa", [x for x in kappas if x is not None])):
        row.update({f"{prefix}_{k}":v for k,v in _summary(values).items()})
    total=sum(symp); probs=[x/total for x in symp if total>0 and x>0]; entropy=float(-sum(p*math.log(p) for p in probs)) if probs else 0.0
    row.update({"symplectic_ridge_area_max_share": max(symp)/total if total>0 else 0.0, "symplectic_ridge_area_top3_share": sum(sorted(symp,reverse=True)[:3])/total if total>0 else 0.0, "symplectic_ridge_area_entropy":entropy, "symplectic_ridge_area_effective_face_count":math.exp(entropy), "symplectic_ridge_area_normalized_entropy":entropy/math.log(len(symp)) if len(symp)>1 else 0.0})
    weighted=float(sum(e*k for e,k in paired)/sum(e for e,_ in paired)) if paired else None; covariance=float(np.cov([e for e,_ in paired],[k for _,k in paired],bias=True)[0,1]) if len(paired)>1 else None; err=max((abs(s-e*k) for e,s,k in zip(eucl,symp,kappas) if k is not None),default=0.0)
    row.update({"kappa_euclidean_weighted_mean":weighted,"kappa_euclidean_covariance":covariance,"decomposition_max_abs_error":err,"decomposition_relative_error":err/max((abs(s) for s in symp),default=1.0),"decomposition_identity_ok":err <= 1e-10})
    row["vertex_covariance"]=_covariance(vertices, expected)
    cov=row["vertex_covariance"]
    if cov.get("status") not in ALLOWED_COVARIANCE_STATUSES: raise AnalysisError("unknown covariance status")
    return row


def load_rows(path: Path) -> tuple[list[dict[str, Any]], str]:
    payload=path.read_bytes()
    if not payload.endswith(b"\n"): raise AnalysisError(f"{path}: no final newline")
    rows=[]; seen=set()
    for n,line in enumerate(payload.splitlines(),1):
        try: row=json.loads(line)
        except json.JSONDecodeError as exc: raise AnalysisError(f"{path}:{n}: invalid JSON") from exc
        ident=(row.get("sample_id") or row.get("source_id")) if isinstance(row,dict) else None
        if not isinstance(ident,str) or ident in seen: raise AnalysisError(f"{path}:{n}: duplicate or invalid sample_id")
        seen.add(ident); rows.append(row)
    if not rows: raise AnalysisError(f"{path}: empty input")
    return rows, hashlib.sha256(payload).hexdigest()


def _join_tangential(source_rows, replay_rows):
    if any(r.get("schema") != TANGENTIAL_SCHEMA for r in source_rows + replay_rows):
        raise AnalysisError("authoritative/replay tangential schema mismatch")
    source={r["sample_id"]:r for r in source_rows}; replay={r["sample_id"]:r for r in replay_rows}
    if set(source) != set(replay): raise AnalysisError("authoritative/replay sample_id sets differ")
    for ident in source:
        a,b=source[ident],replay[ident]
        for key in CONTRACT_KEYS:
            if a.get(key) != b.get(key): raise AnalysisError(f"authoritative/replay contract mismatch for {ident}: {key}")
        if b.get("geometry_dual_vertices_rational") is None: raise AnalysisError("replay row lacks geometry sidecar")
    return [(source[ident], replay[ident]) for ident in sorted(source)]


def _git_state():
    try:
        revision=subprocess.check_output(["git","rev-parse","HEAD"],text=True).strip(); dirty=bool(subprocess.check_output(["git","status","--porcelain"],text=True).strip())
    except Exception: revision="unknown"; dirty=None
    return revision, dirty

def _provenance(paths, hashes, rows, revision, dirty):
    return {"inputs":[{"path":str(p),"sha256":h,"rows":len(r)} for p,h,r in zip(paths,hashes,rows)],"source_revision":revision,"source_dirty":dirty,"schemas":{"orientation":ORIENTATION_SCHEMA,"tangential":TANGENTIAL_SCHEMA,"feature":ROW_SCHEMA},"target_fields_rejected":["capacity","sys","iterations","iteration","bounce_label","target","target_ms"],"tolerances":{"decomposition_abs":1e-10,"replay_volume_abs":1e-12,"covariance_condition_max":1e10}}


def main() -> None:
    p=argparse.ArgumentParser(); p.add_argument("--orientation",type=Path); p.add_argument("--tangential-source",type=Path); p.add_argument("--tangential-replay",type=Path); p.add_argument("--out-dir",type=Path,required=True); p.add_argument("--require-clean",action="store_true"); p.add_argument("--expected-revision"); p.add_argument("--expected-orientation-sha256"); p.add_argument("--expected-tangential-source-sha256"); p.add_argument("--expected-tangential-replay-sha256"); a=p.parse_args()
    revision,dirty=_git_state()
    if a.require_clean and dirty: p.error("repository is dirty")
    if a.expected_revision and a.expected_revision != revision: p.error("revision mismatch")
    if not a.orientation and not (a.tangential_source and a.tangential_replay): p.error("provide orientation or both tangential source/replay")
    if bool(a.tangential_source) != bool(a.tangential_replay): p.error("tangential source and replay must be supplied together")
    out=a.out_dir; out.mkdir(parents=True,exist_ok=True); feature=[]; paths=[]; hashes=[]; loaded=[]
    if a.orientation:
        rows,h=load_rows(a.orientation);
        if a.expected_orientation_sha256 and a.expected_orientation_sha256 != h: p.error("orientation SHA256 mismatch")
        feature.extend(feature_row(r,"orientation") for r in rows); paths.append(a.orientation); hashes.append(h); loaded.append(rows)
    if a.tangential_source:
        src,hs=load_rows(a.tangential_source); rep,hr=load_rows(a.tangential_replay)
        if a.expected_tangential_source_sha256 and a.expected_tangential_source_sha256 != hs: p.error("tangential source SHA256 mismatch")
        if a.expected_tangential_replay_sha256 and a.expected_tangential_replay_sha256 != hr: p.error("tangential replay SHA256 mismatch")
        pairs=_join_tangential(src,rep); feature.extend(feature_row(s,"tangential",g) for s,g in pairs); paths += [a.tangential_source,a.tangential_replay]; hashes += [hs,hr]; loaded += [src,rep]
    with (out/"features.jsonl").open("w") as f:
        for r in feature: f.write(json.dumps(r,sort_keys=True,allow_nan=False)+"\n")
    feature_bytes=(out/"features.jsonl").read_bytes(); report={"schema":"generator-exact-feature-augmenter-report-v2","rows":len(feature),"source_kinds":dict(Counter(r["source_kind"] for r in feature)),"provenance":_provenance(paths,hashes,loaded,revision,dirty),"feature_output":{"path":str(out/"features.jsonl"),"sha256":hashlib.sha256(feature_bytes).hexdigest(),"rows":len(feature),"schema":ROW_SCHEMA},"command":" ".join(__import__("sys").argv)}
    (out/"augment-report.json").write_text(json.dumps(report,indent=2)+"\n")


if __name__ == "__main__": main()
