#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = ["numpy"]
# ///

"""Target-free variable-facet convex-body support-distance packet."""
from __future__ import annotations

import argparse, csv, hashlib, itertools, json, math, subprocess, time
from dataclasses import dataclass
from fractions import Fraction
from pathlib import Path
from typing import Any, Iterable, Sequence
import numpy as np

SCHEMA="generator-variable-f-body-distance-report-v1"; DIM=4; EPS=1e-10
ORIENTATION_SCHEMA="generator-orientation-smoke-row-v2"; FEATURE_SCHEMA="alternative-generator-smoke-row-v2"
class ReconstructionError(ValueError): pass
@dataclass(frozen=True)
class Body:
    name: str; normals: tuple[tuple[Fraction,...],...]; vertices: np.ndarray; exact_vertex_count: int; combination_count: int

def rat(x: str|int|float|Fraction)->Fraction: return x if isinstance(x,Fraction) else Fraction(x)
def dot(a: Sequence[Fraction],b: Sequence[Fraction])->Fraction: return sum((x*y for x,y in zip(a,b)),Fraction())
def normals_of(values: Sequence[Sequence[str|int|float|Fraction]])->tuple[tuple[Fraction,...],...]:
    try: n=tuple(tuple(rat(x) for x in row) for row in values)
    except (ValueError,ZeroDivisionError) as exc: raise ReconstructionError("normal contains invalid rational") from exc
    if len(n)<5 or any(len(row)!=DIM or not any(row) for row in n): raise ReconstructionError("expected at least five nonzero 4D normals")
    if len(set(n))!=len(n): raise ReconstructionError("redundant duplicate normalized inequality")
    return n
def solve(a: Sequence[Sequence[Fraction]])->tuple[Fraction,...]|None:
    w=[list(row)+[Fraction(1)] for row in a]
    for c in range(DIM):
        p=next((r for r in range(c,DIM) if w[r][c]),None)
        if p is None:return None
        w[c],w[p]=w[p],w[c]; d=w[c][c]; w[c]=[x/d for x in w[c]]
        for r in range(DIM):
            if r!=c and w[r][c]:
                f=w[r][c]; w[r]=[x-f*y for x,y in zip(w[r],w[c])]
    return tuple(w[r][-1] for r in range(DIM))
def reconstruct_body(name: str, values: Sequence[Sequence[str|int|float|Fraction]])->Body:
    """Exact reconstruction of a_i.x<=1; reject redundant/unbounded inputs."""
    n=normals_of(values); vertices=set(); joins=0
    for idx in itertools.combinations(range(len(n)),DIM):
        joins+=1; v=solve([n[i] for i in idx])
        if v is not None and all(dot(a,v)<=1 for a in n):vertices.add(v)
    if len(vertices)<5:raise ReconstructionError("inequalities do not reconstruct a full-dimensional bounded body")
    ordered=tuple(sorted(vertices)); p=np.asarray([[float(x) for x in v] for v in ordered])
    if np.linalg.matrix_rank(p[1:]-p[0],tol=1e-10)!=DIM:raise ReconstructionError("reconstructed vertices are not four-dimensional")
    unused=[i for i,a in enumerate(n) if not any(dot(a,v)==1 for v in ordered)]
    if unused:raise ReconstructionError(f"redundant inequality indices: {unused}")
    return Body(name,n,p,len(ordered),joins)

def canonical(body: Body,matrix: np.ndarray|None=None)->np.ndarray:
    p=body.vertices if matrix is None else body.vertices@matrix.T; p=p-np.mean(p,axis=0)
    r=math.sqrt(float(np.mean(np.sum(p*p,axis=1))))
    if not math.isfinite(r) or r<=EPS:raise ReconstructionError("zero/nonfinite RMS vertex radius")
    return p/r
def primitive_directions(level:int)->np.ndarray:
    rows=[]
    for raw in itertools.product(range(-level,level+1),repeat=DIM):
        if any(raw) and math.gcd(*(abs(x) for x in raw))==1:
            v=np.asarray(raw,dtype=float);rows.append(v/np.linalg.norm(v))
    return np.asarray(rows)
AXES=np.vstack((np.eye(DIM),-np.eye(DIM)))
def dist(a:np.ndarray,b:np.ndarray,directions:np.ndarray)->tuple[float,float]:
    delta=np.max(a@directions.T,axis=0)-np.max(b@directions.T,axis=0)
    return float(np.max(abs(delta))),float(math.sqrt(np.mean(delta*delta)))
def parity(p:Sequence[int])->int:return -1 if sum(p[i]>p[j] for i in range(len(p)) for j in range(i+1,len(p)))%2 else 1
def so4_banks()->tuple[list[np.ndarray],list[np.ndarray]]:
    coarse=[];fine=[]
    for signs in itertools.product((-1.,1.),repeat=4):
        m=np.diag(signs)
        if round(np.linalg.det(m))==1:coarse.append(m)
    for p in itertools.permutations(range(4)):
        for signs in itertools.product((-1.,1.),repeat=4):
            if parity(p)*math.prod(signs)==1:
                m=np.zeros((4,4))
                for r,c in enumerate(p):m[r,c]=signs[r]
                fine.append(m)
    return coarse,fine
def u2_banks()->tuple[list[np.ndarray],list[np.ndarray]]:
    def m(p:tuple[int,int],ph:tuple[int,int])->np.ndarray:
        a=np.zeros((2,2));b=np.zeros((2,2))
        for r,c in enumerate(p):a[r,c]=round(math.cos(ph[r]*math.pi/2));b[r,c]=round(math.sin(ph[r]*math.pi/2))
        return np.block([[a,-b],[b,a]])
    coarse=[m(p,h) for p in itertools.permutations((0,1)) for h in itertools.product((0,2),repeat=2)]
    fine=[m(p,h) for p in itertools.permutations((0,1)) for h in itertools.product((0,1,2,3),repeat=2)]
    return coarse,fine
def finite_bank(left:Body,right:Body,directions:np.ndarray,bank:Iterable[np.ndarray])->tuple[float,float,int]:
    a=canonical(left); values=[dist(a,canonical(right,m),directions) for m in bank]
    i,v=min(enumerate(values),key=lambda q:(q[1][1],q[1][0],q[0]));return v[0],v[1],i
def symplectic_gram(left:Body,right:Body)->float|None:
    if len(left.normals)!=len(right.normals) or len(left.normals)>8:return None
    j=np.asarray(((0.,0.,1.,0.),(0.,0.,0.,1.),(-1.,0.,0.,0.),(0.,-1.,0.,0.)))
    a=np.asarray([[float(x) for x in r] for r in left.normals]);b=np.asarray([[float(x) for x in r] for r in right.normals]);ga=a@j@a.T;gb=b@j@b.T
    return min(float(np.linalg.norm(ga-gb[np.ix_(p,p)]))/len(a) for p in itertools.permutations(range(len(a))))

def cube_normals()->list[list[str]]:
    out=[]
    for i in range(4):
        for s in (-1,1):
            r=["0"]*4;r[i]=str(s);out.append(r)
    return out
def ball_normals()->list[list[str]]:
    out=cube_normals()
    for i,j in ((0,1),(2,3)):
        for s,t in itertools.product((-1,1),repeat=2):
            r=[Fraction() for _ in range(4)];r[i]=Fraction(3*s,5);r[j]=Fraction(4*t,5);out.append([str(x) for x in r])
    return out
def scale_normals(n:Sequence[Sequence[str]],diag:Sequence[Fraction])->list[list[str]]:return [[str(rat(x)/d) for x,d in zip(row,diag)] for row in n]
def synthetic()->tuple[dict[str,Body],dict[str,str]]:
    cube=cube_normals();ball=ball_normals()
    bodies={
      "synthetic:cube":reconstruct_body("synthetic:cube",cube),"synthetic:cube_permuted":reconstruct_body("synthetic:cube_permuted",list(reversed(cube))),
      "synthetic:truncated_cube":reconstruct_body("synthetic:truncated_cube",cube+[["2/7"]*4]),
      "synthetic:narrow_cut_cube":reconstruct_body("synthetic:narrow_cut_cube",cube+[["50/199"]*4]),
      "synthetic:ball_approximant":reconstruct_body("synthetic:ball_approximant",ball),
      "synthetic:ellipsoid_approximant":reconstruct_body("synthetic:ellipsoid_approximant",scale_normals(ball,(Fraction(3,2),Fraction(1),Fraction(1),Fraction(1,2))))}
    try:reconstruct_body("synthetic:cube_redundant",cube+[cube[0]])
    except ReconstructionError as e:return bodies,{"synthetic:cube_redundant":f"fail_closed:{e}"}
    raise AssertionError("redundant cube accepted")
def load_jsonl(path:Path)->tuple[list[dict[str,Any]],str]:
    raw=path.read_bytes()
    if not raw.endswith(b"\n"):raise ValueError(f"{path}: missing final newline")
    return [json.loads(line) for line in raw.splitlines()],hashlib.sha256(raw).hexdigest()
def rows_to_bodies(rows:Iterable[dict[str,Any]],schema:str,prefix:str)->dict[str,Body]:
    out={}
    for row in rows:
        if row.get("schema")!=schema:continue
        if schema==ORIENTATION_SCHEMA:
            ok=row.get("base_accepted") is True and row.get("reconstruction_status")=="reconstructed" and row.get("semantic_invariants_passed") is True; normals=row.get("transformed_dual_vertices_rational");ident=row.get("transformed_id")
        else:
            ok=row.get("accepted") is True and row.get("validation_status")=="survived" and all(row.get(k) is None for k in ("capacity","sys","iterations","target")) and row.get("target_ms") in (None,0,0.0);normals=row.get("geometry_dual_vertices_rational");ident=row.get("sample_id")
        if ok and isinstance(ident,str) and isinstance(normals,list):out[f"{prefix}:{ident}"]=reconstruct_body(f"{prefix}:{ident}",normals)
    return out
def rel(path:Path)->str:
    try:return str(path.resolve().relative_to(Path(__file__).resolve().parents[4]))
    except ValueError:return str(path)

def compare(name:str,left:Body,right:Body,directions:dict[str,np.ndarray],u2:tuple[list[np.ndarray],list[np.ndarray]],so4:tuple[list[np.ndarray],list[np.ndarray]])->dict[str,Any]:
    started=time.perf_counter();direct={k:dist(canonical(left),canonical(right),v) for k,v in directions.items()}
    uc,uf=finite_bank(left,right,directions["primitive_level_3"],u2[0]),finite_bank(left,right,directions["primitive_level_3"],u2[1])
    sc,sf=finite_bank(left,right,directions["primitive_level_3"],so4[0]),finite_bank(left,right,directions["primitive_level_3"],so4[1])
    return {"case":name,"left":left.name,"right":right.name,"left_facets":len(left.normals),"right_facets":len(right.normals),"direct_sampled":{k:{"linf":v[0],"l2":v[1]} for k,v in direct.items()},"u2_finite_bank":{"linf":uf[0],"l2":uf[1],"bank_size":len(u2[1]),"winning_index":uf[2]},"so4_finite_bank":{"linf":sf[0],"l2":sf[1],"bank_size":len(so4[1]),"winning_index":sf[2]},"residuals":{"direct_direction_refinement_abs_linf":abs(direct["primitive_level_3"][0]-direct["primitive_level_2"][0]),"direct_direction_refinement_abs_l2":abs(direct["primitive_level_3"][1]-direct["primitive_level_2"][1]),"u2_finite_bank_search_refinement_abs_linf":abs(uf[0]-uc[0]),"u2_finite_bank_search_refinement_abs_l2":abs(uf[1]-uc[1]),"so4_finite_bank_search_refinement_abs_linf":abs(sf[0]-sc[0]),"so4_finite_bank_search_refinement_abs_l2":abs(sf[1]-sc[1])},"cost_observation_seconds":time.perf_counter()-started}

def run_packet(orientation:Path,feature:Path)->dict[str,Any]:
    directions={"axis_only":AXES,"primitive_level_1":primitive_directions(1),"primitive_level_2":primitive_directions(2),"primitive_level_3":primitive_directions(3)};u2,so4=u2_banks(),so4_banks(); started=time.perf_counter();bodies,dispositions=synthetic(); synthetic_reconstruction_seconds=time.perf_counter()-started
    cases=[("identity_permuted_presentation",bodies["synthetic:cube"],bodies["synthetic:cube_permuted"]),("facet_birth_truncated_cube",bodies["synthetic:cube"],bodies["synthetic:truncated_cube"]),("adversarial_narrow_feature",bodies["synthetic:cube"],bodies["synthetic:narrow_cut_cube"]),("ball_approximant_identity",bodies["synthetic:ball_approximant"],bodies["synthetic:ball_approximant"]),("ball_vs_ellipsoid_approximant",bodies["synthetic:ball_approximant"],bodies["synthetic:ellipsoid_approximant"])]
    orows,oh=load_jsonl(orientation);frows,fh=load_jsonl(feature);started=time.perf_counter();obs=rows_to_bodies(orows,ORIENTATION_SCHEMA,"orientation");orientation_reconstruction_seconds=time.perf_counter()-started;started=time.perf_counter();fbs=rows_to_bodies(frows,FEATURE_SCHEMA,"exact_feature");feature_reconstruction_seconds=time.perf_counter()-started
    def take(needle:str)->tuple[str,Body]:
        hits=sorted((k,v) for k,v in obs.items() if needle in k)
        if not hits:raise ReconstructionError(f"required retained orientation witness unavailable: {needle}")
        return hits[0]
    bn,base=take("bucket=3x3/row=0/attempt=0/map=identity");un,ub=take("bucket=3x3/row=0/attempt=0/map=u2-deterministic");sn,sb=take("bucket=3x3/row=0/attempt=0/map=so4-deterministic");fn,fb=sorted(fbs.items())[0]
    cases += [("retained_orientation_u2",base,ub),("retained_orientation_so4",base,sb),("retained_exact_feature_witness",base,fb)]
    comparisons=[compare(*case,directions,u2,so4) for case in cases];narrow=next(x for x in comparisons if x["case"]=="adversarial_narrow_feature");perm=next(x for x in comparisons if x["case"]=="identity_permuted_presentation")
    fixed={"source":"copy-local bounded direct symplectic-Gram permutation quotient on six normalized dual facets","orientation_identity_vs_u2":symplectic_gram(base,ub),"orientation_identity_vs_so4":symplectic_gram(base,sb),"selected_ids":{"identity":bn,"u2":un,"so4":sn,"exact_feature":fn}}
    controls={"permuted_presentation_zero":perm["direct_sampled"]["primitive_level_3"]["linf"]<1e-12,"redundant_presentation_fail_closed":dispositions["synthetic:cube_redundant"].startswith("fail_closed:"),"narrow_feature_axis_grid_understates_linf":narrow["direct_sampled"]["axis_only"]["linf"] <= .51*narrow["direct_sampled"]["primitive_level_3"]["linf"],"narrow_feature_primitive_grid_detected":narrow["direct_sampled"]["primitive_level_3"]["linf"]>1e-4,"fixed_f_u2_gram_zero":fixed["orientation_identity_vs_u2"]<1e-10,"fixed_f_so4_gram_positive":fixed["orientation_identity_vs_so4"]>1e-4}
    return {"schema":SCHEMA,"target_free":True,"coordinate_order":"q1,q2,p1,p2","method_contract":{"input":"irredundant normalized dual inequalities a_i.x <= 1; exact reconstruction fails closed on invalid/redundant/unreconstructable inputs","normalization":"subtract arithmetic mean of reconstructed vertices and divide by RMS vertex radius: translation and positive global-scale quotient only","direct":"direct_sampled support L_infinity (sampled Hausdorff surrogate) and L2, neither exact Hausdorff","directions":"axis-only plus antipodally complete normalized primitive integer S^3 directions, max coordinate 1/2/3; level 3 primary","groups":"u2_finite_bank: 32 monomial-unitary maps; so4_finite_bank: 192 orientation-preserving signed permutations; neither is a compact-group infimum/Haar integration","no_sp4":"Sp(4) is noncompact and intentionally is not sampled as Haar probability"},"direction_designs":{k:len(v) for k,v in directions.items()},"finite_banks":{"u2_coarse":len(u2[0]),"u2_fine":len(u2[1]),"so4_coarse":len(so4[0]),"so4_fine":len(so4[1])},"reconstruction":{k:{"facets":len(v.normals),"vertices":v.exact_vertex_count,"four_facet_joins_checked":v.combination_count} for k,v in bodies.items()},"cost_observations_seconds":{"synthetic_reconstruction":synthetic_reconstruction_seconds,"orientation_reconstruction":orientation_reconstruction_seconds,"exact_feature_reconstruction":feature_reconstruction_seconds},"presentation_dispositions":dispositions,"comparisons":comparisons,"fixed_f_direct_view":fixed,"controls":controls,"provenance":{"orientation_input":{"path":rel(orientation),"sha256":oh,"rows":len(orows),"accepted_reconstructed_rows":len(obs)},"exact_feature_input":{"path":rel(feature),"sha256":fh,"rows":len(frows),"accepted_target_free_geometry_rows":len(fbs)},"synthetic_definitions":"exact rational normalized inequalities embedded in body_distance.py"},"interpretation":{"answerable_cross_f":"Direct normalized body-shape differences, including facet birth/death, under the stated Euclidean translation/scale convention.","invisible_distinctions":"No symplectic-equivalence, combinatorial-type, exact-Hausdorff, or continuous-U(2)/SO(4)-quotient claim; finite directions can miss features.","deferrals":["normalized surface-area-measure transport: no transport/certificate contract selected","directed symplectic containment gauge: not implemented; would remain a dissimilarity pending metric/computation facts"]}}

def table(report:dict[str,Any])->list[dict[str,Any]]:
    out=[]
    for x in report["comparisons"]:
        d=x["direct_sampled"];r=x["residuals"];out.append({"case":x["case"],"left_facets":x["left_facets"],"right_facets":x["right_facets"],"direct_sampled_linf":d["primitive_level_3"]["linf"],"direct_sampled_l2":d["primitive_level_3"]["l2"],"u2_finite_bank_linf":x["u2_finite_bank"]["linf"],"u2_finite_bank_l2":x["u2_finite_bank"]["l2"],"so4_finite_bank_linf":x["so4_finite_bank"]["linf"],"so4_finite_bank_l2":x["so4_finite_bank"]["l2"],"direction_refinement_l2":r["direct_direction_refinement_abs_l2"],"u2_search_refinement_l2":r["u2_finite_bank_search_refinement_abs_l2"],"so4_search_refinement_l2":r["so4_finite_bank_search_refinement_abs_l2"]})
    return out
def git_state()->dict[str,Any]:
    try:return {"revision":subprocess.check_output(["git","rev-parse","HEAD"],text=True).strip(),"tracked_clean_before_generation":subprocess.run(["git","diff","--quiet"],check=False).returncode==0}
    except (OSError,subprocess.SubprocessError):return {"revision":"unknown","tracked_clean_before_generation":None}
def write(report:dict[str,Any],out:Path)->None:
    out.mkdir(parents=True,exist_ok=True);(out/"report.json").write_text(json.dumps(report,indent=2,sort_keys=True)+"\n")
    fields=list(table(report)[0]);
    with (out/"calibration.tsv").open("w",newline="") as f:
        w=csv.DictWriter(f,fieldnames=fields,delimiter="\t",lineterminator="\n");w.writeheader();w.writerows(table(report))
def main()->None:
    root=Path(__file__).resolve().parents[4];p=argparse.ArgumentParser(description=__doc__);p.add_argument("--orientation-input",type=Path,default=root/"experiments/sys-datascience/methods/generator-orientation-smoke/artifacts/panel-2-per-bucket/rows.jsonl");p.add_argument("--exact-feature-input",type=Path,default=root/"experiments/sys-datascience/methods/generator-exact-feature-augmenter/artifacts/full-panels/tangential-replay/smoke-rows.jsonl");p.add_argument("--out-dir",type=Path,required=True);p.add_argument("--require-tracked-clean",action="store_true");a=p.parse_args();state=git_state()
    if a.require_tracked_clean and state["tracked_clean_before_generation"] is not True:p.error("tracked source tree is not clean before generation")
    report=run_packet(a.orientation_input,a.exact_feature_input);report["run"]={**state,"producer_path":rel(Path(__file__)),"producer_sha256":hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),"terminal_status":"complete"}
    if not all(report["controls"].values()):raise SystemExit(f"calibration controls failed: {report['controls']}")
    write(report,a.out_dir)
if __name__=="__main__":main()
