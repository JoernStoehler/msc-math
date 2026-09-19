# /// script
# requires-python = ">=3.11"
# dependencies = ["numpy>=2", "scipy>=1.14"]
# ///
"""Small, capacity-free, reproducible covariance-blind representation panel.

Run with uv run --script PATH. Writes only beside this source, under artifacts/.
Geometry is floating point; no capacity or ratio certificates are claimed.
"""
from __future__ import annotations
import hashlib
import itertools
import json
import math
from pathlib import Path
import time

import numpy as np
from scipy.spatial import ConvexHull, HalfspaceIntersection

HERE = Path(__file__).resolve().parent
J = np.array([[0,0,1,0],[0,0,0,1],[-1,0,0,0],[0,-1,0,0]], float)
B = np.array([[0,1,0,0],[-1,0,0,0],[0,0,0,-1],[0,0,1,0]], float)
FORMS = np.stack([J, B, J @ B])

def jsonable(x):
    if isinstance(x, np.ndarray): return x.tolist()
    if isinstance(x, np.generic): return x.item()
    raise TypeError(type(x))

def save(path, obj):
    path.write_text(json.dumps(obj, default=jsonable, indent=2, sort_keys=True)+"\n")

def extreme(points):
    points = np.asarray(points, float)
    return points[ConvexHull(points).vertices]

def from_normals(normals):
    normals = np.asarray(normals, float)
    return extreme(HalfspaceIntersection(np.c_[normals, -np.ones(len(normals))], np.zeros(4)).intersections)

def triangulate(vertices):
    hull = ConvexHull(vertices)
    interior = vertices.mean(0)
    simplices = np.concatenate([np.broadcast_to(interior, (len(hull.simplices),1,4)), vertices[hull.simplices]], axis=1)
    weights = np.abs(np.linalg.det(simplices[:,1:]-simplices[:,:1])) / math.factorial(4)
    return hull, simplices, weights

def moments(vertices):
    hull, simplices, weights = triangulate(vertices)
    vol = weights.sum()
    prob = weights/vol
    mean = np.einsum('s,si->i', prob, simplices.mean(1))
    centered = simplices-mean
    sums = centered.sum(1)
    covariance = np.einsum('s,sij->ij',prob,(np.einsum('sai,saj->sij', centered,centered)+np.einsum('si,sj->sij',sums,sums))/30)
    return float(vol), mean, covariance, abs(float(hull.volume)-vol)

def dirichlet4():
    out = np.empty((5,)*4)
    for idx in itertools.product(range(5),repeat=4):
        counts = np.bincount(idx,minlength=5)
        out[idx] = math.prod(math.factorial(int(n)) for n in counts) / (5*6*7*8)
    return out

D4 = dirichlet4()

def fourth_moment(vertices):
    _, simplices, weights = triangulate(vertices)
    tensor = np.zeros((4,)*4)
    for simplex, weight in zip(simplices,weights/weights.sum()):
        tensor += weight*np.einsum('abcd,ai,bj,ck,dl->ijkl', D4,simplex,simplex,simplex,simplex,optimize=True)
    return tensor

def facet_data(vertices):
    hull = ConvexHull(vertices)
    # Qhull triangulates facets. Merge coplanar equations, never nearby bodies.
    equations = []
    for equation in hull.equations:
        if not any(np.max(np.abs(equation-other)) < 2e-9 for other in equations): equations.append(equation)
    equations = np.array(equations)
    assert np.max(equations[:,-1]) < 0
    polar = equations[:,:4]/(-equations[:,-1,None])
    incidence = np.abs(vertices@polar.T-1) < 2e-8
    ridges = []
    seen = set()
    for i,j in itertools.combinations(range(len(polar)),2):
        ids = tuple(np.flatnonzero(incidence[:,i]&incidence[:,j]).tolist())
        if len(ids)<3 or ids in seen: continue
        x = vertices[list(ids)]
        _, singular, vh = np.linalg.svd(x-x.mean(0),full_matrices=False)
        if singular[1] < 1e-8 or singular[2] > 1e-7: continue
        plane = (x-x.mean(0))@vh[:2].T
        order = ConvexHull(plane).vertices
        ordered = x[order]
        area_vector = np.array([0.5*np.einsum('ni,ij,nj->',ordered,omega,np.roll(ordered,-1,axis=0)) for omega in FORMS])
        ridges.append(dict(vertex_ids=[ids[k] for k in order], area_vector=area_vector))
        seen.add(ids)
    return polar, incidence, ridges

def source_collection():
    rng = np.random.default_rng(2026091801)
    cube = np.array(list(itertools.product([-1.,1.],repeat=4)))
    cross = np.r_[np.eye(4),-np.eye(4)]
    simplex = np.r_[np.eye(4),-np.ones((1,4))]
    yield 'simplex', 'exact-shape-control', simplex
    yield 'cube', 'exact-shape-control', cube
    yield 'crosspolytope', 'exact-shape-control', cross
    vertices24=[]
    for pair in itertools.combinations(range(4),2):
        for signs in itertools.product([-1.,1.],repeat=2):
            point=np.zeros(4);point[list(pair)]=signs;vertices24.append(point)
    yield '24cell', 'higher-symmetry-control', np.array(vertices24)
    for n in [8,12,18]:
        for rep in range(2):
            yield f'gaussian_hull_{n}_{rep}', 'generic-vertex-hull', extreme(rng.normal(size=(n,4)))
    for n in [8,12]:
        cloud=rng.normal(size=(n,4))
        yield f'symmetric_hull_{2*n}', 'symmetric-nonproduct', extreme(np.r_[cloud,-cloud])
    for n in [4,8]:
        normals=rng.normal(size=(n,4));normals/=np.linalg.norm(normals,axis=1,keepdims=True)
        yield f'facet_intersection_{8+n}', 'generic-halfspace', from_normals(np.r_[cross,normals/.8])
    normals=np.r_[cross,[[.55,.55,.55,.55],[.75,-.45,.3,.55]]]
    yield 'asymmetrically_cut_cube', 'controlled-truncation', from_normals(normals)

def orientations():
    rng=np.random.default_rng(2026091802)
    values=[np.array([1.,0,0]),np.array([0.,1,0]),np.array([0.,0,1]),np.ones(3)/np.sqrt(3)]
    values += [v/np.linalg.norm(v) for v in rng.normal(size=(124,3))]
    return np.array(values)

def pairing_fourth(tensor, omega):
    transformed=np.einsum('abcd,ae,bf,cg,dh->efgh',tensor,omega,omega,omega,omega,optimize=True)
    return float(np.einsum('abcd,abcd->',transformed,tensor))

def measure(vertices, polar, tensor, ridges, volume, covariance, u):
    omega=np.einsum('a,aij->ij',u,FORMS)
    dp=polar@omega@polar.T
    pp=vertices@omega@vertices.T
    delta=float(np.max(np.abs(dp)))
    second=float(-np.trace(covariance@omega@covariance@omega))
    fourth=pairing_fourth(tensor,omega)
    unsigned=np.abs(np.array([r['area_vector'] for r in ridges])@u)
    centered_vertices=vertices-vertices.mean(0)
    vertex_covariance=centered_vertices.T@centered_vertices/len(vertices)
    frequencies=np.sort(np.abs(np.linalg.eigvals(vertex_covariance@omega)))[::2]
    return dict(orientation=u, omega=omega,
        dual_pairing_max=delta, dual_reciprocal_over_volume_sqrt=1/(delta*np.sqrt(volume)),
        dual_pairing_argmax=list(np.unravel_index(np.argmax(np.abs(dp)),dp.shape)),
        primal_pairing_max_over_volume_sqrt=float(np.max(np.abs(pp))/np.sqrt(volume)),
        vertex_pairing_kurtosis=float(np.mean(pp**4)/np.mean(pp**2)**2),
        vertex_covariance_rho=float(frequencies[1]/frequencies[0]),
        vertex_covariance_nu1_over_volume_sqrt=float(frequencies[0]/np.sqrt(volume)),
        vertex_covariance_nu2_over_volume_sqrt=float(frequencies[1]/np.sqrt(volume)),
        volume_pairing_second=second, volume_pairing_fourth=fourth, volume_pairing_kurtosis=fourth/second**2,
        ridge_area_sum_over_volume_sqrt=float(unsigned.sum()/np.sqrt(volume)),
        ridge_area_max_share=float(unsigned.max()/unsigned.sum()))

def main():
    started=time.time()
    output=HERE/'artifacts';output.mkdir(exist_ok=True)
    us=orientations()
    rows=[];bodies=[];checks=[]
    for name,family,original in source_collection():
        volume0,center,cov,error0=moments(original)
        eig,frame=np.linalg.eigh(cov)
        whitening=(frame/np.sqrt(eig))@frame.T
        vertices=(original-center)@whitening.T
        volume,center1,cov1,error=moments(vertices)
        tensor=fourth_moment(vertices)
        polar,incidence,ridges=facet_data(vertices)
        body_id=hashlib.sha256(np.asarray(original,dtype='<f8').tobytes()).hexdigest()
        body=dict(body_id=body_id,name=name,population=family,source_vertices=original,source_volume=volume0,
            source_centroid=center,source_volume_covariance=cov,whitening_map=whitening,vertices=vertices,
            volume=volume,volume_covariance=cov1,volume_fourth_moment=tensor,polar_vertices=polar,
            vertex_facet_incidence=incidence.astype(int),ridges=ridges,
            numerical_status='f64 Qhull topology; deterministic simplex integration; no capacity calculation')
        bodies.append(body)
        current=[]
        for k,u in enumerate(us):
            record=measure(vertices,polar,tensor,ridges,volume,cov1,u)
            record.update(body_id=body_id,body_name=name,population=family,orientation_id=k,definition_version='volume-pairing-and-dual-v1')
            current.append(record)
        rows.extend(current)
        checks.append(dict(body=name,vertex_count=len(vertices),facet_count=len(polar),ridge_count=len(ridges),
            centroid_error=float(np.max(np.abs(center1))),covariance_identity_error=float(np.max(np.abs(cov1-np.eye(4)))),
            triangulation_volume_error=error,relative_volume_transform_error=abs(volume-volume0*np.linalg.det(whitening))/volume,
            second_moment_error=max(abs(r['volume_pairing_second']-4) for r in current)))
        print(name,len(vertices),len(polar),'kurtosis range',min(r['volume_pairing_kurtosis'] for r in current),max(r['volume_pairing_kurtosis'] for r in current),flush=True)
    for filename,items in [('bodies.jsonl',bodies),('measurements.jsonl',rows)]:
        with (output/filename).open('w') as stream:
            for item in items:stream.write(json.dumps(item,default=jsonable,sort_keys=True)+'\n')
    save(output/'checks.json',dict(checks=checks,elapsed_seconds=time.time()-started))
    save(output/'definitions.json',dict(version='volume-pairing-and-dual-v1',form_basis=FORMS,
        pair_measure='independent uniform-volume X,Y; exact polynomial simplex integration in f64',
        vertex_pair_measure='independent uniform distinct-extreme-vertex draws, centered at volume centroid',
        dual_reciprocal='1/max_ij |a_i^T Omega a_j|; centered polar a_i, divided by sqrt(volume); called c_J only on centrally symmetric bodies',
        orientation_contract='Omega(u)=sum u_i forms_i, |u|=1; represents R^T J R in SO(4)/U(2); geometry and covariance stay fixed',
        whitening_contract='non-symplectic generally; source bodies and map retained; not capacity preserving',
        row_count=len(rows),body_count=len(bodies),source_seed=2026091801,orientation_seed=2026091802))

if __name__=='__main__':main()
