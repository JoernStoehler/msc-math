"""Additional adverse populations for the pairing/volume candidate.

No capacity calls. Integer cases carry exact Pfaffian sums using int64; their
small coordinates and generator counts keep all arithmetic far below overflow.
Every geometry is retained, not just selected successes.
"""
from __future__ import annotations

import itertools
import json
from pathlib import Path
import time

import numpy as np

from scout import J, measure, record


def fast_metrics(v):
    """Array of shape (batch,m,4). No determinants needed for the search."""
    w=v@J.astype(v.dtype)@np.swapaxes(v,-1,-2)
    a,b,c,d=np.array(list(itertools.combinations(range(v.shape[1]),4))).T
    phi=np.abs(np.triu(w,1)).sum(axis=(-1,-2))
    psi=np.abs(w[:,a,b]*w[:,c,d]-w[:,a,c]*w[:,b,d]+w[:,a,d]*w[:,b,c]).sum(axis=1)
    return phi,psi,4*psi/phi**2


def main():
    start=time.perf_counter()
    rng=np.random.default_rng(71809026)
    e=np.eye(4,dtype=int)
    c6=np.array([e[0],e[2],-e[0]+e[1],e[3],-e[1],-e[2]-e[3]])
    roots={"A4":np.array([*e,*[e[i]-e[j] for i in range(4) for j in range(i+1,4)]]),
           "D4":np.array([e[i]+s*e[j] for i in range(4) for j in range(i+1,4) for s in [-1,1]]),
           "C6_lagrangian_equality":c6}
    integer=[]
    for m in (6,7,8,10):
        v=rng.integers(-2,3,size=(1000,m,4),dtype=np.int64)
        phi,psi,ratio=fast_metrics(v)
        integer.append({"source":"integer_uniform_-2_2","m":m,
                        "generators":v.tolist(),"phi":phi.tolist(),"psi":psi.tolist(),
                        "candidate_ratio":ratio.tolist(),
                        "exact_candidate_slack":(phi**2-4*psi).tolist(),
                        "full_dimensional":(psi>0).tolist()})
    perturbations=[]
    # The arbitrary planar equality family is more adverse than generic
    # Gaussian data. C6 has six generators and a connected pairing graph.
    for eps in (.3,.1,.03,.01,.003,.001):
        v=c6[None,:,:]+eps*rng.normal(size=(256,6,4))
        phi,psi,ratio=fast_metrics(v)
        perturbations.append({"source":"C6_equality_neighborhood","epsilon":eps,
                              "generators":v.tolist(),"phi":phi.tolist(),"psi":psi.tolist(),
                              "candidate_ratio":ratio.tolist()})
    # Body-invariance controls independent of the anticipated inequality.
    base=roots["D4"].astype(float)
    transformed=base.copy()
    transformed[::2]*=-1
    split=np.vstack([base[1:],.25*base[0],.75*base[0]])
    shear=np.block([[np.eye(2),np.array([[2.,1.],[1.,-1.]])],[np.zeros((2,2)),np.eye(2)]])
    controls={"D4":measure(base),"reoriented":measure(transformed),"split":measure(split),
              "symplectic_shear":measure(base@shear.T),"scaled_3":measure(base*3)}
    checks={"symplectic_shear_form_error":float(np.max(np.abs(shear.T@J@shear-J))),
            "phi_invariance_error":max(abs(controls[k]["phi"]-controls["D4"]["phi"]) for k in ("reoriented","split","symplectic_shear")),
            "volume_invariance_error":max(abs(controls[k]["volume"]-controls["D4"]["volume"]) for k in ("reoriented","split","symplectic_shear")),
            "phi_scaling_error":abs(controls["scaled_3"]["phi"]-9*controls["D4"]["phi"]),
            "volume_scaling_error":abs(controls["scaled_3"]["volume"]-81*controls["D4"]["volume"])}
    summaries=[]
    for group in integer+perturbations:
        rs=np.array(group["candidate_ratio"])
        summaries.append({k:group[k] for k in ("source","m","epsilon") if k in group}|{
            "count":len(rs),"minimum_ratio":float(rs.min()),"maximum_ratio":float(rs.max()),"violations":int(np.sum(rs>1+1e-12)),
            **({"rank_deficient_count":sum(not x for x in group["full_dimensional"]),
                "minimum_exact_slack":min(group["exact_candidate_slack"])} if "full_dimensional" in group else {})})
    result={"schema":"zonotope-adverse-v1","seed":71809026,
            "status":"integer Pfaffian comparisons exact for integer populations; floating-point otherwise",
            "roots":[record(v,name) for name,v in roots.items()],"checks":checks,"controls":controls,
            "summary":summaries,"integer_populations":integer,"equality_perturbations":perturbations,
            "wall_seconds":time.perf_counter()-start}
    Path(__file__).with_name("adversarial.json").write_text(json.dumps(result,separators=(",",":"))+"\n")
    print(json.dumps({"checks":checks,"summary":summaries,"wall_seconds":result["wall_seconds"]},indent=2))


if __name__=="__main__":
    main()
