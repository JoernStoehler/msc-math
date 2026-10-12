"""Retain two-segment response polynomials instead of a flat sample table."""
from __future__ import annotations
import itertools
import json
from pathlib import Path
import numpy as np
from scout import J,measure,record


def coefficients(v,u,w):
    v=np.asarray(v);u=np.asarray(u);w=np.asarray(w)
    assert np.array_equal(v,np.round(v)) and np.array_equal(u,np.round(u)) and np.array_equal(w,np.round(w))
    v=v.astype(np.int64);u=u.astype(np.int64);w=w.astype(np.int64)
    # Coordinates here are <=3 and m<=10, so products/sums cannot overflow.
    def exact(volumes):
        pairing=volumes@J.astype(np.int64)@volumes.T
        a,b,c,d=np.array(list(itertools.combinations(range(len(volumes)),4))).T
        pf=pairing[a,b]*pairing[c,d]-pairing[a,c]*pairing[b,d]+pairing[a,d]*pairing[b,c]
        return int(16*np.abs(pf).sum()),int(np.abs(np.triu(pairing,1)).sum())
    vol,phi=exact(v)
    volu=exact(np.vstack([v,u]))[0]
    volw=exact(np.vstack([v,w]))[0]
    voluw=exact(np.vstack([v,u,w]))[0]
    a=np.abs(v@J.astype(np.int64)@u).sum();b=np.abs(v@J.astype(np.int64)@w).sum();c=abs(u@J.astype(np.int64)@w)
    return {"volume":[vol,volu-vol,volw-vol,voluw-volu-volw+vol],
            "phi":[phi,int(a),int(b),int(c)]}


def main():
    rng=np.random.default_rng(54181809)
    e=np.eye(4)
    cube=e
    c6=np.array([e[0],e[2],-e[0]+e[1],e[3],-e[1],-e[2]-e[3]])
    # Arbitrary planar zonogon A and a quarter-turned copy in the other
    # Lagrangian plane. This is a known mixed-area equality population.
    planar=np.array([[2,1],[-1,3],[-2,-1],[1,2]],float)
    turn=np.array([[0,1],[-1,0]])
    matched=np.vstack([np.column_stack([planar,np.zeros_like(planar)]),
                       np.column_stack([np.zeros_like(planar),planar@turn.T])])
    source=np.array([[1,0,0,0],[0,1,0,0],[0,0,1,0],[0,1,0,-1],[0,0,1,-1],[1,-1,-1,0]])
    bases={"cube":cube,"C6":c6,"matched_planar_zonogons":matched,"known_volume_counterexample":source,
           "generic_integer":rng.integers(-2,3,(8,4))}
    params=np.concatenate([[0],np.logspace(-3,3,13)])
    t,s=np.meshgrid(params,params,indexing="ij")
    features=np.stack([np.ones_like(t),t,s,t*s],axis=-1)
    rows=[]
    for name,v in bases.items():
        base=record(v,name)
        pairs=[rng.integers(-2,3,(2,4)) for _ in range(64)]
        pairs.extend(np.array([e[i],e[j]]) for i,j in itertools.combinations(range(4),2))
        if name=="known_volume_counterexample":
            pairs.append(np.array([[1,0,0,-1],[0,1,-1,0]]))
        for k,(u,w) in enumerate(pairs):
            co=coefficients(v,u,w)
            volume=features@np.array(co["volume"])
            phi=features@np.array(co["phi"])
            ratio=volume/(4*phi**2)
            a,b,c,d=co["volume"];x,y,z,cross=co["phi"]
            # The constant sign of volume's log mixed derivative is ad-bc.
            # For Φ it is x*cross-y*z, also independent of t,s.
            row={"base_id":base["body_id"],"base_source":name,"sample":k,
                 "u":u.tolist(),"w":w.tolist(),"coefficients":co,
                 "maximum_candidate_ratio":float(ratio.max()),
                 "maximizer_grid_parameters":[float(t.flat[ratio.argmax()]),float(s.flat[ratio.argmax()])],
                 "volume_log_mixed_numerator":a*d-b*c,
                 "phi_log_mixed_numerator":x*cross-y*z,
                 "candidate_first_derivative_u":8*x*y-b,
                 "candidate_first_derivative_w":8*x*z-c,
                 "candidate_quadratic_copositivity_margin":16*y*z+8*x*cross-d}
            row["all_nonnegative_parameters_certificate"]=(4*x*x-a>=0 and 8*x*y-b>=0 and 8*x*z-c>=0 and 16*y*z+8*x*cross-d>=0)
            rows.append(row)
    result={"schema":"zonotope-segment-polynomials-v1","seed":54181809,
            "polynomial_order":"[constant,t,s,t*s] for Z+t[-u,u]+s[-w,w]",
            "status":"exact integer polynomial coefficients and sufficient orthant certificates; grid ratios floating point",
            "parameter_grid":params.tolist(),"bases":[record(v,name) for name,v in bases.items()],"paths":rows}
    result["summary"]={"paths":len(rows),"grid_evaluations":len(rows)*len(params)**2,
                       "max_candidate_ratio":max(r["maximum_candidate_ratio"] for r in rows),
                       "all_parameter_certified_paths":sum(r["all_nonnegative_parameters_certificate"] for r in rows),
                       "opposite_log_mixed_signs":sum(r["volume_log_mixed_numerator"]*r["phi_log_mixed_numerator"]<0 for r in rows)}
    Path(__file__).with_name("segment_paths.json").write_text(json.dumps(result,indent=2)+"\n")
    print(json.dumps(result["summary"],indent=2))
    for name in bases:
        group=[r for r in rows if r["base_source"]==name]
        print(name, {"max":max(r["maximum_candidate_ratio"] for r in group),
                     "first_derivative_max_abs":max(abs(r["candidate_first_derivative_u"]) for r in group),
                     "minimum_quadratic_margin":min(r["candidate_quadratic_copositivity_margin"] for r in group),
                     "positive_phi_log_mixed":sum(r["phi_log_mixed_numerator"]>0 for r in group)})


if __name__=="__main__":main()
