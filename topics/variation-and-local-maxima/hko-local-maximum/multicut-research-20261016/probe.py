# /// script
# requires-python = ">=3.11"
# dependencies = ["numpy", "scipy"]
# ///
"""Numerical cuts; nominal feasible-HK upper-bound estimates, never equality.
Uses the accepted repository F11 packet's exact HKO fixture and old seed words.
All writes go to --out and a fresh temporary extraction directory.
"""
import argparse, hashlib, importlib.util, itertools, json, platform, tempfile, time
from pathlib import Path
from zipfile import ZipFile
import numpy as np
import scipy
from scipy.spatial import HalfspaceIntersection, ConvexHull


def main():
    p=argparse.ArgumentParser()
    p.add_argument('--repo',type=Path,required=True)
    p.add_argument('--out',type=Path,required=True)
    args=p.parse_args(); start=time.perf_counter()
    archive=args.repo/'topics/variation-and-local-maxima/hko-local-maximum/f11-return-20261004/hko_f11_complete_proof.zip'
    with tempfile.TemporaryDirectory(prefix='hko-multicut-') as temporary:
        root=Path(temporary)
        with ZipFile(archive) as z:
            for name in ['inherited/code/exact_core.py','inherited/data/seeds.json']:
                target=root/name; target.parent.mkdir(parents=True,exist_ok=True);target.write_bytes(z.read(name))
        spec=importlib.util.spec_from_file_location('accepted_f11_exact_core',root/'inherited/code/exact_core.py')
        core=importlib.util.module_from_spec(spec);spec.loader.exec_module(core)
        raw=json.loads((root/'inherited/data/seeds.json').read_text())
        A=np.array([[core.nominal(x) for x in row] for row in core.A]);V=np.array([[core.nominal(x) for x in row] for row in core.V]);Q0=core.nominal(core.Q0);V0=core.nominal(core.V0)
        J=np.block([[np.zeros((2,2)),np.eye(2)],[-np.eye(2),np.zeros((2,2))]])
        seeds=[]
        for row in raw:
            sig=row['sigma']; beta=np.array([core.nominal(core.VALUES[x]) for x in row['weights']]);norm=A[sig]
            C=np.vstack((norm.T,np.ones(len(sig))));pivot=row['pivot'];ci=np.linalg.inv(C[:,pivot]);q=np.triu(norm@J@norm.T,1)
            assert np.linalg.norm(C@beta-np.array([0,0,0,0,1]))<1e-12
            assert abs(beta@q@beta-Q0)<1e-12
            seeds.append((sig,beta,ci,pivot))
        incidence=np.isclose(V@A.T,1,atol=1e-12)
        vertex_dirs=[A[np.flatnonzero(ids)].mean(axis=0) for ids in incidence]
        edge_dirs=[]
        for i,j in itertools.combinations(range(len(V)),2):
            shared=np.flatnonzero(incidence[i]&incidence[j])
            if len(shared)==3:edge_dirs.append(A[shared].mean(axis=0))
        assert len(vertex_dirs)==25 and len(edge_dirs)==50
        panels={'vertices-25':vertex_dirs,'edges-50':edge_dirs,'mixed-75':vertex_dirs+edge_dirs}
        def feasible_best(b):
            best=Q0; max_residual=0.;best_info=None
            for si,(sig,beta,ci,pivot) in enumerate(seeds):
                h=np.zeros(len(sig));h[pivot]=-ci@np.r_[b,1]
                zmax=min(-beta[h<0]/h[h<0]);
                for slot in range(len(sig)+1):
                    nn=np.insert(A[sig],slot,b,axis=0);v0=np.insert(beta,slot,0);v1=np.insert(h,slot,1)
                    q=np.triu(nn@J@nn.T,1);linear=v0@q@v1+v1@q@v0;quadratic=v1@q@v1
                    candidates=[0.,zmax]
                    if quadratic<0: candidates.append(np.clip(-linear/(2*quadratic),0,zmax))
                    for z in candidates:
                        weights=v0+z*v1;value=weights@q@weights
                        residual=max(abs(weights.sum()-1),np.linalg.norm(weights@nn),max(0.,-weights.min()))
                        max_residual=max(max_residual,float(residual))
                        if value>best:best=float(value);best_info={'seed':si,'slot':slot,'z':float(z),'minimum_weight':float(weights.min())}
            return best,max_residual,best_info
        rows=[]
        for label,base in panels.items():
            for depth in [0.02,0.01,0.005,0.0025]:
                B=(1+depth)*np.array(base);normals=np.vstack((A,B))
                vertices=HalfspaceIntersection(np.c_[normals,-np.ones(len(normals))],np.zeros(4)).intersections
                vol=ConvexHull(vertices).volume
                active_counts=[int(np.linalg.matrix_rank(vertices[np.abs(vertices@a-1)<1e-8][1:]-vertices[np.abs(vertices@a-1)<1e-8][:1],tol=1e-8)==3) if np.count_nonzero(np.abs(vertices@a-1)<1e-8)>=4 else 0 for a in normals]
                # By cyclic pentagon symmetry, all radial cuts within each vertex/edge
                # panel have equal feasible-bank response, but evaluate every direction.
                best=Q0;residual=0.;witness=None
                for bi,b in enumerate(B):
                    q,rr,info=feasible_best(b);residual=max(residual,rr)
                    if q>best:best=q;witness={'cut':bi,**info}
                ratio_bound=(Q0/best)**2*(V0/vol)
                rows.append({'panel':label,'extra_inequalities':len(B),'numerical_genuine_facets':sum(active_counts),'numerical_genuine_added_facets':sum(active_counts[10:]),'depth':depth,'vertices':len(vertices),'volume':float(vol),'relative_volume_loss':float(1-vol/V0),'relative_capacity_upper_bound':float(Q0/best),'relative_systolic_upper_bound':float(ratio_bound),'hk_constraint_max_residual':residual,'best_witness':witness})
                print(label,depth,'rho_upper/rho0',ratio_bound,'volume_loss',1-vol/V0,flush=True)
        result={'scope':'Floating-point evidence only. Each capacity value estimates a feasible-HK upper bound; residuals and Qhull volume do not provide rigorous numerical enclosures. No exhaustive capacity evaluation or numerical neighborhood radius is certified.','accepted_archive_sha256':hashlib.sha256(archive.read_bytes()).hexdigest(),'script_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),'python':platform.python_version(),'numpy':np.__version__,'scipy':scipy.__version__,'rows':rows,'all_relative_upper_bounds_below_one':all(r['relative_systolic_upper_bound']<1 for r in rows),'seconds':time.perf_counter()-start}
        args.out.parent.mkdir(parents=True,exist_ok=True);args.out.write_text(json.dumps(result,indent=2)+'\n')

if __name__=='__main__':main()
