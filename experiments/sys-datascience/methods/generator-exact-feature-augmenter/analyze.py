#!/usr/bin/env python3
# /// script
# dependencies = ["numpy"]
# ///
"""Enforce the target-free feature packet contract and emit paired summaries."""
from __future__ import annotations
import argparse, copy, hashlib, json, math, subprocess
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
DESIGNS = {"disposable": ({"3x3"}, {"3x3", "4x6", "6x6"}, 1, 1), "retained": ({"3x3", "4x4", "4x6", "6x6"}, {"3x3", "4x6", "6x6"}, 2, 64)}
EXPECTED_ORIENTATION_BUCKETS = {"3x3", "4x4", "4x6", "6x6"}
EXPECTED_TANGENTIAL_BUCKETS = {"3x3", "4x6", "6x6"}

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
        if not isinstance(r.get("base_id"),str) or not isinstance(r.get("bucket"),str) or r.get("source_kind") not in {"orientation","tangential"}: raise AnalysisError("missing feature provenance fields")
        if not isinstance(r.get("source_sample_id"),str) or not isinstance(r.get("source_pairing_id"),str): raise AnalysisError("missing source identity fields")
        if r["source_kind"] == "orientation" and r.get("map_variant") not in VARIANTS: raise AnalysisError("invalid orientation map_variant")
        if r["source_kind"] == "tangential" and (r.get("law") not in ARMS or not isinstance(r.get("source_pairing_id"),str)): raise AnalysisError("invalid tangential law/pairing")
        for key in ("facet_count","vertex_count","two_face_count","ordered_two_face_count","ordering_failure_count"):
            if not isinstance(r.get(key),int) or isinstance(r.get(key),bool): raise AnalysisError(f"feature field {key} has wrong type")
        if r.get("ordering_failure_count") != 0: raise AnalysisError("face ordering failure present")
        if r.get("two_face_count") != r.get("ordered_two_face_count", -1) + r.get("ordering_failure_count", -1): raise AnalysisError("two-face population/order accounting mismatch")
        if not r.get("decomposition_identity_ok") or r.get("decomposition_max_abs_error",math.inf)>1e-10: raise AnalysisError("decomposition audit failed")
        for key in ("capacity","sys","iterations","iteration","bounce_label","target"):
            if r.get(key) is not None: raise AnalysisError(f"target field {key} present")
        for key in EUCL+SYMP+KAPPA+tuple(f"decomposition_{x}" for x in ("max_abs_error","relative_error"))+("volume","volume_sqrt"):
            if r.get(key) is None: raise AnalysisError(f"required feature {key} is null")
            _finite(r.get(key),key)
        cov=r.get("vertex_covariance",{}); status=cov.get("status")
        if status != "eligible": raise AnalysisError("reviewed feature covariance must be eligible")
        if cov.get("expected_vertex_count") != cov.get("distinct_vertex_count") or cov.get("expected_vertex_count") != r.get("vertex_count"): raise AnalysisError("covariance vertex counts are incomplete")
        for key in COV:
            if cov.get(key) is None: raise AnalysisError(f"required covariance feature {key} is null")
            _finite(cov.get(key),f"covariance.{key}")
        if cov.get("condition",math.inf)>1e10: raise AnalysisError("eligible covariance exceeds condition limit")
        strict_allowed=_strict_scope(r)
        if strict_allowed and r.get("strict_cycle") is None: raise AnalysisError("allowed row lacks strict-cycle metadata")
        if not strict_allowed and r.get("strict_cycle") is not None: raise AnalysisError("forbidden row has strict-cycle metadata")
        if r.get("strict_cycle") is not None:
            cyc=r["strict_cycle"]; signs=cyc.get("strict_signs")
            if not isinstance(cyc.get("strict_sign_cell"),bool) or not isinstance(cyc.get("strict_cycle_feasible"),bool) or type(cyc.get("strict_cycle_count")) is not int or cyc.get("strict_cycle_count")<0: raise AnalysisError("strict-cycle metadata malformed")
            if not isinstance(signs,list) or len(signs)!=3 or any(not isinstance(row,list) or len(row)!=3 or any(type(x) is not int or x not in {-1,0,1} for x in row) for row in signs): raise AnalysisError("strict-cycle signs malformed")
            if cyc["strict_cycle_feasible"] != (cyc["strict_cycle_count"]>0) or cyc["strict_sign_cell"] != all(x != 0 for row in signs for x in row): raise AnalysisError("strict-cycle metadata inconsistent")
    if not require_complete: return
    og=defaultdict(list)
    for r in rows:
        if r.get("source_kind")=="orientation": og[(r.get("base_id"),r.get("bucket"),r.get("map_variant"))].append(r)
    bases={(b,k) for b,k,_ in og}
    for b,k in bases:
        if k not in EXPECTED_ORIENTATION_BUCKETS or {v for bb,kk,v in og if bb==b and kk==k} != VARIANTS: raise AnalysisError(f"incomplete orientation five-variant grid for {b}/{k}")
    tg=defaultdict(list)
    for r in rows:
        if r.get("source_kind")=="tangential": tg[(r.get("source_pairing_id"),r.get("bucket"),r.get("law"))].append(r)
    pairs={(p,k) for p,k,_ in tg}
    for p,k in pairs:
        if k not in EXPECTED_TANGENTIAL_BUCKETS or {law for pp,kk,law in tg if pp==p and kk==k} != ARMS: raise AnalysisError(f"incomplete tangential four-arm grid for {p}/{k}")
    _enforce_orientation(rows)

def validate_design(rows, design="disposable"):
    if design not in DESIGNS: raise AnalysisError(f"unknown design {design}")
    orientation_buckets,tangential_buckets,bases_per_bucket, pairs_per_bucket=DESIGNS[design]
    validate(rows, require_complete=True)
    orientations=[r for r in rows if r["source_kind"]=="orientation"]; tangentials=[r for r in rows if r["source_kind"]=="tangential"]
    if {r["bucket"] for r in orientations} != orientation_buckets: raise AnalysisError("orientation bucket set is incomplete")
    if {r["bucket"] for r in tangentials} != tangential_buckets: raise AnalysisError("tangential bucket set is incomplete")
    for bucket in orientation_buckets:
        bases={r["base_id"] for r in orientations if r["bucket"]==bucket}
        if len(bases)!=bases_per_bucket: raise AnalysisError(f"expected {bases_per_bucket} orientation bases in {bucket}")
    for bucket in tangential_buckets:
        pairs={r["source_pairing_id"] for r in tangentials if r["bucket"]==bucket}
        if len(pairs)!=pairs_per_bucket: raise AnalysisError(f"expected {pairs_per_bucket} tangential pairs in {bucket}")
    expected=5*len(orientation_buckets)*bases_per_bucket + 4*len(tangential_buckets)*pairs_per_bucket
    if len(rows)!=expected: raise AnalysisError(f"expected exactly {expected} feature rows, found {len(rows)}")
    if len({(r["base_id"],r["bucket"],r.get("map_variant")) for r in orientations}) != 5*len(orientation_buckets)*bases_per_bucket: raise AnalysisError("duplicate orientation logical cell")
    if len({(r["source_pairing_id"],r["bucket"],r.get("law")) for r in tangentials}) != 4*len(tangential_buckets)*pairs_per_bucket: raise AnalysisError("duplicate tangential logical cell")

def _enforce_orientation(rows):
    groups=defaultdict(dict)
    for r in rows:
        if r.get("source_kind")=="orientation": groups[(r["base_id"],r["bucket"])][r["map_variant"]]=r
    for key,g in groups.items():
        identity=g["identity"]
        for variant,r in g.items():
            if variant=="identity": continue
            required=list(EUCL)+["volume"]+list(COV[:3])
            if variant.startswith("u2-"): required += list(SYMP)+list(KAPPA)+list(COV[3:])
            for field in required:
                a,b=r.get(field),identity.get(field)
                if field in COV:
                    a=r.get("vertex_covariance",{}).get(field); b=identity.get("vertex_covariance",{}).get(field)
                if field.endswith("status"): continue
                if not _close(a,b): raise AnalysisError(f"orientation {variant} control failed for {key}: {field}")
            if r.get("vertex_covariance",{}).get("status") != identity.get("vertex_covariance",{}).get("status"): raise AnalysisError(f"orientation {variant} covariance eligibility status changed")

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
            def delta(field):
                if field in COV: a,b=r.get("vertex_covariance",{}).get(field),i.get("vertex_covariance",{}).get(field)
                else: a,b=r.get(field),i.get(field)
                return None if a is None or b is None else a-b
            orient.append({"base_id":base,"bucket":bucket,"variant":v,"delta_volume":delta("volume"),"delta_euclidean_mean":delta("euclidean_ridge_area_mean"),"delta_symplectic_sum":delta("symplectic_ridge_area_sum"),"delta_symplectic_mean":delta("symplectic_ridge_area_mean"),"delta_symplectic_max_share":delta("symplectic_ridge_area_max_share"),"delta_kappa_weighted":delta("kappa_euclidean_weighted_mean"),"delta_williamson_nu1":delta("nu1"),"delta_williamson_nu2":delta("nu2"),"delta_williamson_rho":delta("rho"),"delta_covariance_condition":delta("condition"),"orthogonal_controls_pass":all(control(f) for f in EUCL+("volume",)+COV[:3]) if v.startswith("so4-") else None,"u2_controls_pass":all(control(f) for f in EUCL+SYMP+KAPPA+("volume",)+COV) if v.startswith("u2-") else None})
    scaled = defaultdict(float); scaled_family = {"orthogonal": defaultdict(float), "u2": defaultdict(float)}
    for (base,bucket), g in og.items():
        i=g["identity"]
        for v,r in g.items():
            if v == "identity": continue
            family="u2" if v.startswith("u2-") else "orthogonal"
            fields = EUCL + SYMP + KAPPA + ("volume",) + COV
            for field in fields:
                if family == "orthogonal" and field not in EUCL+("volume",)+COV[:3]: continue
                if family == "u2" and field not in EUCL+SYMP+KAPPA+("volume",)+COV: continue
                if field in COV:
                    a,b=r.get("vertex_covariance",{}).get(field),i.get("vertex_covariance",{}).get(field)
                else: a,b=r.get(field),i.get(field)
                if a is not None and b is not None:
                    error=abs(float(a)-float(b))/max(abs(float(b)),1.0); scaled[field]=max(scaled[field],error)
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
            arm_ranges={a:[min(v),max(v)] for a,v in by_arm.items()}; lo=max(v[0] for v in arm_ranges.values()); hi=min(v[1] for v in arm_ranges.values()); union=max(v[1] for v in arm_ranges.values())-min(v[0] for v in arm_ranges.values()); overlap=max(0.0,hi-lo); overlap_tolerance=CONTROL_ATOL+CONTROL_RTOL*max(abs(lo),abs(hi),1.0); has_overlap=lo <= hi+overlap_tolerance
            ranges[field]={"arm_ranges":arm_ranges,"overlap_interval":[lo,hi],"overlap_within_tolerance":has_overlap,"overlap_tolerance":overlap_tolerance,"union_normalized_overlap":overlap/union if union>0 else 1.0}
        tangential_overlap.append({"bucket":b,"euclidean_feature_overlap":ranges})
    return {"schema":"generator-exact-feature-augmenter-report-v2","rows":len(rows),"groups":core,"orientation_paired_deltas":orient,"orientation_max_scaled_error_by_field":dict(sorted(scaled.items())),"orientation_max_scaled_error_by_family":{k:dict(sorted(v.items())) for k,v in scaled_family.items()},"orientation_max_scaled_error":max(scaled.values(),default=0.0),"orientation_orthogonal_controls_pass":all(x["orthogonal_controls_pass"] for x in orient if x["variant"].startswith("so4-")),"orientation_u2_controls_pass":all(x["u2_controls_pass"] for x in orient if x["variant"].startswith("u2-")),"tangential_paired_deltas":tang,"tangential_distributions":tangential_distributions,"tangential_euclidean_overlap":tangential_overlap,"strict_cycle_metadata_rows":sum(r.get("strict_cycle") is not None for r in rows),"strict_cycle_used_for_grouping_or_selection":False,"tolerances":{"absolute":CONTROL_ATOL,"relative":CONTROL_RTOL,"decomposition_abs":1e-10}}

def _metadata_behavior(rows):
    altered=copy.deepcopy(rows)
    for r in altered:
        if r.get("strict_cycle") is not None: r["strict_cycle"]=None
    a=report(rows); b=report(altered)
    for key in ("strict_cycle_metadata_rows",): a.pop(key,None); b.pop(key,None)
    return a==b

def verify_manifest(input_path, augment_report_path, expected_revision=None, require_clean=False):
    rows,h=load_rows(input_path); manifest=json.loads(Path(augment_report_path).read_text())
    if manifest.get("schema") != "generator-exact-feature-augmenter-report-v2": raise AnalysisError("unexpected augment report schema")
    output=manifest.get("feature_output",{})
    if output.get("schema") != FEATURE_SCHEMA or output.get("rows") != len(rows) or output.get("sha256") != h: raise AnalysisError("feature output does not match augment report")
    provenance=manifest.get("provenance",{}); revision=subprocess.check_output(["git","rev-parse","HEAD"],text=True).strip(); dirty=bool(subprocess.check_output(["git","status","--porcelain"],text=True).strip())
    if expected_revision and expected_revision != revision: raise AnalysisError("revision mismatch")
    if require_clean and dirty: raise AnalysisError("repository is dirty")
    if provenance.get("source_revision") != revision or provenance.get("source_dirty") is not False: raise AnalysisError("augment provenance revision/clean evidence mismatch")
    for evidence in provenance.get("inputs",[]):
        path=Path(evidence.get("path",""))
        if not path.exists(): raise AnalysisError(f"provenance input missing: {path}")
        payload=path.read_bytes()
        if hashlib.sha256(payload).hexdigest() != evidence.get("sha256"): raise AnalysisError(f"provenance input hash mismatch: {path}")
        if evidence.get("rows") != len(payload.splitlines()): raise AnalysisError(f"provenance input row count mismatch: {path}")
    return rows,h,manifest

def main():
    p=argparse.ArgumentParser(); p.add_argument("--input",type=Path,required=True); p.add_argument("--augment-report",type=Path,required=True); p.add_argument("--out-dir",type=Path,required=True); p.add_argument("--design",choices=sorted(DESIGNS),default="disposable"); p.add_argument("--require-clean",action="store_true"); p.add_argument("--expected-revision"); a=p.parse_args()
    rows,h,manifest=verify_manifest(a.input,a.augment_report,a.expected_revision,a.require_clean)
    validate_design(rows,a.design); orientation_buckets,tangential_buckets,bases_per_bucket,pairs_per_bucket=DESIGNS[a.design]; expected_rows=5*len(orientation_buckets)*bases_per_bucket+4*len(tangential_buckets)*pairs_per_bucket; a.out_dir.mkdir(parents=True,exist_ok=True); rep=report(rows); rep["design_count_audit"]={"design":a.design,"orientation_buckets":sorted(orientation_buckets),"tangential_buckets":sorted(tangential_buckets),"bases_per_orientation_bucket":bases_per_bucket,"pairs_per_tangential_bucket":pairs_per_bucket,"expected_rows":expected_rows,"observed_rows":len(rows),"pass":expected_rows==len(rows)}; rep["metadata_behavioral_audit"]=_metadata_behavior(rows); rep["input"]={"path":str(a.input),"sha256":h,"rows":len(rows),"augment_report":str(a.augment_report),"design":a.design,"provenance_verified":True}; (a.out_dir/"report.json").write_text(json.dumps(rep,indent=2)+"\n")

if __name__=="__main__": main()
