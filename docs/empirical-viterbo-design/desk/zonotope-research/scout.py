"""Bounded capacity-free zonotope pairing/volume scout; numpy only.

Rows are half-segment vectors: Z = sum_i [-v_i, v_i].  Standard coordinates
are (q1,q2,p1,p2). This script has no external data or producer dependency.
"""
from __future__ import annotations

import hashlib
import itertools
import json
from pathlib import Path
import time

import numpy as np

J = np.array([[0,0,1,0],[0,0,0,1],[-1,0,0,0],[0,-1,0,0]], dtype=float)


def measure(v):
    v = np.asarray(v, dtype=float)
    w = v @ J @ v.T
    phi = np.abs(np.triu(w, 1)).sum()
    quads = np.array(list(itertools.combinations(range(len(v)), 4)))
    if len(quads) == 0:
        vol = pfvol = 0.0
    else:
        a,b,c,d = quads.T
        pf = w[a,b]*w[c,d]-w[a,c]*w[b,d]+w[a,d]*w[b,c]
        pfvol = 16*np.abs(pf).sum()
        vol = 16*np.abs(np.linalg.det(v[quads])).sum()
    return {"phi": float(phi), "volume": float(vol), "pfaffian_volume": float(pfvol),
            "candidate_ratio": float(vol/(4*phi*phi)) if phi else None}


def record(v, source, **extra):
    rows = np.asarray(v).tolist()
    body_id = hashlib.sha256(json.dumps(rows,separators=(",",":"),sort_keys=True).encode()).hexdigest()
    return {"body_id": body_id, "source":source, "generators":rows, **measure(v), **extra}


def run():
    start = time.perf_counter()
    rng = np.random.default_rng(20260918)
    rows = []
    for m in (4,5,6,8,12):
        for source in ("gaussian", "sparse_integer", "near_symplectic_split", "near_lagrangian"):
            for k in range(32):
                if source == "gaussian":
                    v = rng.normal(size=(m,4))
                elif source == "sparse_integer":
                    v = rng.integers(-1,2,size=(m,4))
                elif source == "near_symplectic_split":
                    v = rng.normal(size=(m,4))
                    v[:m//2, [1,3]] *= .03
                    v[m//2:, [0,2]] *= .03
                else:
                    v = rng.normal(size=(m,4))
                    v[:,2:] *= .03
                rows.append(record(v,source,sample=k))
    # Exact geometric controls, and the source's [0,v] generators rescaled
    # to our [-v,v] convention. Keeping the integer rows multiplies every
    # source 4-volume by 16, leaving the interaction sign unchanged.
    controls = [record(np.eye(4), "unit_cube"),
                record(np.array([[.3,0,0,0],[.7,0,0,0],*np.eye(4)[1:]]),"split_cube")]
    source_v = np.array([[1,0,0,0],[0,1,0,0],[0,0,1,0],[0,1,0,-1],[0,0,1,-1],[1,-1,-1,0]])
    u = [1,0,0,-1]
    w = [0,1,-1,0]
    for tags, added in (("base",[]),("plus_u",[u]),("plus_w",[w]),("plus_u_w",[u,w])):
        controls.append(record(np.vstack([source_v,*added]),"Skorupinski_2608.07702v1_"+tags))
    # A modest random local search is adversarial evidence, not a global
    # optimizer or a certificate. Every accepted step is retained.
    seeds = sorted(rows,key=lambda r:r["candidate_ratio"] or 0,reverse=True)[:4]
    paths = []
    for seed in seeds:
        v = np.array(seed["generators"])
        best = seed["candidate_ratio"]
        path = [{"iteration":0, "ratio":best, "generators":v.tolist()}]
        for it in range(1200):
            trial = v.copy()
            i = rng.integers(len(v))
            trial[i] += rng.normal(size=4) * (0.12 if it < 400 else 0.03 if it < 800 else .005)
            r = measure(trial)
            if r["candidate_ratio"] is not None and r["candidate_ratio"] > best:
                v,best = trial,r["candidate_ratio"]
                path.append({"iteration":it+1,"ratio":best,"generators":v.tolist()})
        paths.append({"seed_body_id":seed["body_id"],"steps":path,"final":record(v,"local_search")})
    allrows = rows+controls+[p["final"] for p in paths]
    result = {"schema":"zonotope-pairing-scout-v1", "seed":20260918,
              "coordinates":"q1,q2,p1,p2", "generator_convention":"half-segments [-v_i,v_i]",
              "status":"floating-point observations; no capacity evaluated",
              "summary":{"sample_count":len(rows),"maximum_candidate_ratio":max(r["candidate_ratio"] or 0 for r in allrows),
                         "max_pfaffian_determinant_relative_discrepancy":max(abs(r["volume"]-r["pfaffian_volume"])/max(1,r["volume"]) for r in allrows)},
              "controls":controls,"samples":rows,"local_search":paths,
              "wall_seconds":time.perf_counter()-start}
    out=Path(__file__).with_name("scout.json")
    out.write_text(json.dumps(result,indent=2)+"\n")
    print(json.dumps({**result["summary"],"wall_seconds":result["wall_seconds"]},indent=2))


if __name__ == "__main__":
    run()
