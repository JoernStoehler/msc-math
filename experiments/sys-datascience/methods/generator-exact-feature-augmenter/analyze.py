#!/usr/bin/env python3
# /// script
# dependencies = ["numpy"]
# ///
"""Enforce the target-free feature packet contract and emit paired summaries."""
from __future__ import annotations
import argparse, copy, json, math
from collections import defaultdict
from pathlib import Path
from augment import AnalysisError, load_rows

FEATURE_SCHEMA = "generator-exact-feature-augmenter-row-v2"
VARIANTS = {"identity", "u2-deterministic", "u2-haar", "so4-deterministic", "so4-haar"}
ARMS = {"factorial-baseline", "factorial-q", "factorial-p", "factorial-both"}
FACE_FEATURES = ("mean", "std", "min", "q25", "median", "q75", "q90", "q95", "max", "sum")
EUCL = tuple(f"euclidean_ridge_area_{x}" for x in FACE_FEATURES)
SYMP = tuple(f"symplectic_ridge_area_{x}" for x in FACE_FEATURES) + ("symplectic_ridge_area_max_share", "symplectic_ridge_area_top3_share", "symplectic_ridge_area_entropy", "symplectic_ridge_area_effective_face_count", "symplectic_ridge_area_normalized_entropy")
KAPPA = tuple(f"kappa_{x}" for x in FACE_FEATURES) + ("kappa_euclidean_weighted_mean", "kappa_euclidean_covariance")
COV = ("ordinary_eigenvalue_min", "ordinary_eigenvalue_max", "condition", "nu1", "nu2", "rho")
CONTROL_ATOL = 1e-9
CONTROL_RTOL = 1e-9

def _finite(value, label):
    if value is None: return
    if isinstance(value, bool) or not isinstance(value, (int,float)) or not math.isfinite(float(value)): raise AnalysisError(f"{label} is not finite")

def _close(a,b,atol=CONTROL_ATOL,rtol=CONTROL_RTOL):
    return a is None and b is None or a is not None and b is not None and abs(float(a)-float(b)) <= atol + rtol*max(abs(float(a)),abs(float(b)),1.0)

def _strict_scope(row):
    return (row.get("source_kind")=="orientation" and row.get("bucket")=="3x3" and row.get("map_variant")=="identity") or (row.get("source_kind")=="tangential" and row.get("bucket")=="3x3" and row.get("law")=="factorial-baseline")

def validate(rows, require_complete=True):
    if not rows: raise AnalysisError("empty feature packet")
    ids=set()
    for r in rows:
        if r.get("schema") != FEATURE_SCHEMA: raise AnalysisError("unexpected feature schema")
        if not isinstance(r.get("source_id"),str) or r["source_id"] in ids: raise AnalysisError("duplicate or missing source_id")
        ids.add(r["source_id"])
        if r.get("geometry_validation_status") != "validated" or r.get("coordinate_order") != "q1,q2,p1,p2": raise AnalysisError("geometry validation/coordinate contract failed")
        if r.get("ordering_failure_count") != 0: raise AnalysisError("face ordering failure present")
        if r.get("two_face_count") != r.get("ordered_two_face_count", -1) + r.get("ordering_failure_count", -1): raise AnalysisError("two-face population/order accounting mismatch")
        if not r.get("decomposition_identity_ok") or r.get("decomposition_max_abs_error",math.inf)>1e-10: raise AnalysisError("decomposition audit failed")
        for key in ("capacity","sys","iterations","iteration","bounce_label","target"):
            if r.get(key) is not None: raise AnalysisError(f"target field {key} present")
        for key in EUCL+SYMP+KAPPA+tuple(f"decomposition_{x}" for x in ("max_abs_error","relative_error"))+("volume","volume_sqrt"):
            _finite(r.get(key),key)
        cov=r.get("vertex_covariance",{}); status=cov.get("status")
        if status=="eligible":
            for key in COV: _finite(cov.get(key),f"covariance.{key}")
            if cov.get("condition",math.inf)>1e10: raise AnalysisError("eligible covariance exceeds condition limit")
        if r.get("strict_cycle") is not None:
            if not _strict_scope(r): raise AnalysisError("forbidden row has strict-cycle metadata")
            if not isinstance(r["strict_cycle"].get("strict_sign_cell"),bool): raise AnalysisError("strict-cycle metadata incomplete")
    if not require_complete: return
    og=defaultdict(list)
    for r in rows:
        if r.get("source_kind")=="orientation": og[(r.get("base_id"),r.get("bucket"),r.get("map_variant"))].append(r)
    bases={(b,k) for b,k,_ in og}
    for b,k in bases:
        if {v for bb,kk,v in og if bb==b and kk==k} != VARIANTS: raise AnalysisError(f"incomplete orientation five-variant grid for {b}/{k}")
    tg=defaultdict(list)
    for r in rows:
        if r.get("source_kind")=="tangential": tg[(r.get("source_pairing_id"),r.get("bucket"),r.get("law"))].append(r)
    pairs={(p,k) for p,k,_ in tg}
    for p,k in pairs:
        if {law for pp,kk,law in tg if pp==p and kk==k} != ARMS: raise AnalysisError(f"incomplete tangential four-arm grid for {p}/{k}")
    _enforce_orientation(rows)

def _enforce_orientation(rows):
    groups=defaultdict(dict)
    for r in rows:
        if r.get("source_kind")=="orientation": groups[(r["base_id"],r["bucket"])][r["map_variant"]]=r
    for key,g in groups.items():
        identity=g["identity"]
        for variant,r in g.items():
            if variant=="identity": continue
            orth=variant.startswith("so4-")
            required=list(EUCL)+["volume"]+list(COV[:3])
            if variant.startswith("u2-"): required += list(SYMP)+list(KAPPA)+list(COV[3:])
            for field in required:
                a,b=r.get(field),identity.get(field)
                if field in COV:
                    a=r.get("vertex_covariance",{}).get(field); b=identity.get("vertex_covariance",{}).get(field)
                if field.endswith("status"): continue
                if not _close(a,b): raise AnalysisError(f"orientation {variant} control failed for {key}: {field}")
            if variant.startswith("u2-") and r.get("vertex_covariance",{}).get("status") != identity.get("vertex_covariance",{}).get("status"): raise AnalysisError("U2 Williamson eligibility status changed")

def _core_report(rows):
    groups=defaultdict(list)
    for r in rows: groups[(r["source_kind"],r["bucket"])].append(r)
    out=[]
    for (kind,bucket), values in sorted(groups.items()):
        item={"source_kind":kind,"bucket":bucket,"rows":len(values),"euclidean_mean_range":[min(r["euclidean_ridge_area_mean"] for r in values),max(r["euclidean_ridge_area_mean"] for r in values)],"symplectic_mean_range":[min(r["symplectic_ridge_area_mean"] for r in values),max(r["symplectic_ridge_area_mean"] for r in values)],"kappa_weighted_range":[min(r["kappa_euclidean_weighted_mean"] for r in values),max(r["kappa_euclidean_weighted_mean"] for r in values)]}
        if kind=="tangential":
            arm={}
            for r in values: arm.setdefault(r["law"],[]).append(r["euclidean_ridge_area_mean"])
            item["euclidean_mean_range_by_arm"]={k:[min(v),max(v)] for k,v in sorted(arm.items())}; item["euclidean_mean_all_arm_overlap"]=[max(v[0] for v in item["euclidean_mean_range_by_arm"].values()),min(v[1] for v in item["euclidean_mean_range_by_arm"].values())]
        out.append(item)
    return out

def report(rows):
    core=_core_report(rows); orient=[]; og=defaultdict(dict)
    for r in rows:
        if r["source_kind"]=="orientation": og[(r["base_id"],r["bucket"])][r["map_variant"]]=r
    for (base,bucket), g in sorted(og.items()):
        i=g["identity"]
        for v,r in sorted(g.items()):
            if v=="identity": continue
            def control(field):
                if field in COV: return _close(r.get("vertex_covariance",{}).get(field), i.get("vertex_covariance",{}).get(field))
                return _close(r.get(field), i.get(field))
            orient.append({"base_id":base,"bucket":bucket,"variant":v,"delta_volume":r["volume"]-i["volume"],"delta_euclidean_mean":r["euclidean_ridge_area_mean"]-i["euclidean_ridge_area_mean"],"orthogonal_controls_pass":all(control(f) for f in EUCL+("volume",)+COV[:3]) if v.startswith("so4-") else None,"u2_controls_pass":all(control(f) for f in EUCL+SYMP+KAPPA+("volume",)+COV) if v.startswith("u2-") else None})
    scaled = defaultdict(float); scaled_family = {"orthogonal": defaultdict(float), "u2": defaultdict(float)}
    for (base,bucket), g in og.items():
        i=g["identity"]
        for v,r in g.items():
            if v == "identity": continue
            fields = EUCL + SYMP + KAPPA + ("volume",) + COV
            for field in fields:
                if field in COV:
                    a,b=r.get("vertex_covariance",{}).get(field),i.get("vertex_covariance",{}).get(field)
                else: a,b=r.get(field),i.get(field)
                if a is not None and b is not None:
                    error=abs(float(a)-float(b))/max(abs(float(b)),1.0); scaled[field]=max(scaled[field],error)
                    family="u2" if v.startswith("u2-") else "orthogonal"
                    if family == "orthogonal" and field not in EUCL+("volume",)+COV[:3]: continue
                    if family == "u2" and field not in EUCL+SYMP+KAPPA+("volume",)+COV: continue
                    scaled_family[family][field]=max(scaled_family[family][field],error)
    tang=[]; tg=defaultdict(dict); tangential_distributions=[]; tangential_overlap=[]
    for r in rows:
        if r["source_kind"]=="tangential": tg[(r["source_pairing_id"],r["bucket"])][r["law"]]=r
    for (p,b),g in sorted(tg.items()):
        i=g["factorial-baseline"]
        for law,r in sorted(g.items()):
            if law!="factorial-baseline":
                tang.append({"pairing_id":p,"bucket":b,"law":law,"delta_euclidean_sum":r["euclidean_ridge_area_sum"]-i["euclidean_ridge_area_sum"],"delta_euclidean_mean":r["euclidean_ridge_area_mean"]-i["euclidean_ridge_area_mean"],"delta_symplectic_sum":r["symplectic_ridge_area_sum"]-i["symplectic_ridge_area_sum"],"delta_symplectic_mean":r["symplectic_ridge_area_mean"]-i["symplectic_ridge_area_mean"],"delta_symplectic_max_share":r["symplectic_ridge_area_max_share"]-i["symplectic_ridge_area_max_share"],"delta_kappa_weighted":r["kappa_euclidean_weighted_mean"]-i["kappa_euclidean_weighted_mean"],"delta_rho":r["vertex_covariance"].get("rho")-i["vertex_covariance"].get("rho") if r["vertex_covariance"].get("rho") is not None and i["vertex_covariance"].get("rho") is not None else None,"delta_condition":r["vertex_covariance"].get("condition")-i["vertex_covariance"].get("condition") if r["vertex_covariance"].get("condition") is not None and i["vertex_covariance"].get("condition") is not None else None})
    dist_groups=defaultdict(list)
    for r in rows:
        if r["source_kind"] == "tangential": dist_groups[(r["bucket"],r["law"])].append(r)
    for (b,law), values in sorted(dist_groups.items()):
        distributions={}
        for field in EUCL+SYMP+KAPPA:
            xs=[r[field] for r in values if r.get(field) is not None]
            distributions[field]={"count":len(xs),"min":min(xs) if xs else None,"max":max(xs) if xs else None,"mean":sum(xs)/len(xs) if xs else None}
        for name in ("rho","condition"):
            xs=[r["vertex_covariance"].get(name) for r in values if r["vertex_covariance"].get(name) is not None]
            statuses=defaultdict(int)
            for r in values: statuses[r["vertex_covariance"].get("status")]+=1
            distributions[name]={"count":len(xs),"min":min(xs) if xs else None,"max":max(xs) if xs else None,"mean":sum(xs)/len(xs) if xs else None,"status_counts":dict(statuses)}
        tangential_distributions.append({"bucket":b,"law":law,"rows":len(values),"features":distributions})
    for b in sorted({r["bucket"] for r in rows if r["source_kind"]=="tangential"}):
        bucket_rows=[r for r in rows if r["source_kind"]=="tangential" and r["bucket"]==b]; ranges={}
        for field in EUCL:
            by_arm=defaultdict(list)
            for r in bucket_rows: by_arm[r["law"]].append(r[field])
            arm_ranges={a:[min(v),max(v)] for a,v in by_arm.items()}; lo=max(v[0] for v in arm_ranges.values()); hi=min(v[1] for v in arm_ranges.values()); union=max(v[1] for v in arm_ranges.values())-min(v[0] for v in arm_ranges.values()); overlap=max(0.0,hi-lo)
            ranges[field]={"arm_ranges":arm_ranges,"overlap_interval":[lo,hi],"overlap":overlap>0,"union_normalized_overlap":overlap/union if union>0 else 1.0}
        tangential_overlap.append({"bucket":b,"euclidean_feature_overlap":ranges})
    return {"schema":"generator-exact-feature-augmenter-report-v2","rows":len(rows),"groups":core,"orientation_paired_deltas":orient,"orientation_max_scaled_error_by_field":dict(sorted(scaled.items())),"orientation_max_scaled_error_by_family":{k:dict(sorted(v.items())) for k,v in scaled_family.items()},"orientation_max_scaled_error":max(scaled.values(),default=0.0),"orientation_orthogonal_controls_pass":all(x["orthogonal_controls_pass"] for x in orient if x["variant"].startswith("so4-")),"orientation_u2_controls_pass":all(x["u2_controls_pass"] for x in orient if x["variant"].startswith("u2-")),"tangential_paired_deltas":tang,"tangential_distributions":tangential_distributions,"tangential_euclidean_overlap":tangential_overlap,"strict_cycle_metadata_rows":sum(r.get("strict_cycle") is not None for r in rows),"strict_cycle_used_for_grouping_or_selection":False,"tolerances":{"absolute":CONTROL_ATOL,"relative":CONTROL_RTOL,"decomposition_abs":1e-10}}

def _metadata_behavior(rows):
    altered=copy.deepcopy(rows)
    for r in altered:
        if r.get("strict_cycle") is not None: r["strict_cycle"]=None
    a=report(rows); b=report(altered)
    for key in ("strict_cycle_metadata_rows",): a.pop(key,None); b.pop(key,None)
    return a==b

def main():
    p=argparse.ArgumentParser(); p.add_argument("--input",type=Path,required=True); p.add_argument("--out-dir",type=Path,required=True); a=p.parse_args(); rows,h=load_rows(a.input); validate(rows); a.out_dir.mkdir(parents=True,exist_ok=True); rep=report(rows); rep["metadata_behavioral_audit"]=_metadata_behavior(rows); rep["input"]={"path":str(a.input),"sha256":h,"rows":len(rows)}; (a.out_dir/"report.json").write_text(json.dumps(rep,indent=2)+"\n")

if __name__=="__main__": main()
