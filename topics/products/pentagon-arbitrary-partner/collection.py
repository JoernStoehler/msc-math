"""Small heterogeneous geometry collection, with deterministic lineage and exact-monomial integration."""
import hashlib
import json
import math
from pathlib import Path
from time import perf_counter
import numpy as np
from scipy.spatial import ConvexHull
from evaluator import evaluate, regular, rotation, templates

ROOT = Path(__file__).parent
VERSION = 'partner-geometry-v1'
ANGLES = 2*np.pi*np.arange(128)/128
DIRECTIONS = np.column_stack([np.cos(ANGLES), np.sin(ANGLES)])
Q = regular(5,np.pi/5)


def hull(points):
    p = np.asarray(points)
    return p[ConvexHull(p).vertices]


def area_centroid(points):
    p=np.asarray(points); q=np.roll(p,-1,axis=0)
    cross=p[:,0]*q[:,1]-p[:,1]*q[:,0]
    area=float(sum(cross)/2)
    return area, np.sum((p+q)*cross[:,None],axis=0)/(6*area)


def monomial(p, a, b):
    """Uniform-area E[x^a y^b] via oriented triangles (0,v_i,v_i+1)."""
    area,_=area_centroid(p)
    total=0.
    for v,w in zip(p,np.roll(p,-1,axis=0)):
        twice_area=v[0]*w[1]-v[1]*w[0]
        total += twice_area*sum(math.comb(a,i)*math.comb(b,j)*v[0]**i*w[0]**(a-i)*
            v[1]**j*w[1]**(b-j)*math.factorial(i+j)*math.factorial(a+b-i-j)/
            math.factorial(a+b+2) for i in range(a+1) for j in range(b+1))
    return total/area


def moments(p):
    area,centroid=area_centroid(p); centered=p-centroid
    m={(a,n-a):monomial(centered,a,n-a) for n in range(5) for a in range(n+1)}
    cov=np.array([[m[2,0],m[1,1]],[m[1,1],m[0,2]]])
    eig,U=np.linalg.eigh(cov)
    white=centered@((U/np.sqrt(eig))@U.T)
    wm={(a,n-a):monomial(white,a,n-a) for n in [3,4] for a in range(n+1)}
    skew2=sum(math.comb(3,a)*wm[a,3-a]**2 for a in range(4))
    radial4=wm[4,0]+2*wm[2,2]+wm[0,4]
    return {'area':area,'centroid':centroid.tolist(),'covariance':cov.tolist(),
            'centered_monomials':{f'{a},{b}':val for (a,b),val in m.items()},
            'covariance_trace':float(np.trace(cov)), 'covariance_determinant':float(np.linalg.det(cov)),
            'inertia_condition':float(eig[1]/eig[0]),'affine_skew_tensor_norm2':float(skew2),
            'affine_radial_fourth_moment':float(radial4)}


def difference(p):
    return hull((p[:,None,:]-p[None,:,:]).reshape(-1,2))


def two_bounce(p,q=Q):
    dp,dq=difference(p),difference(q)
    hd=ConvexHull(dp)
    A,b=hd.equations[:,:2],-hd.equations[:,2]
    rays=ConvexHull(dq).equations[:,:2]
    candidates=list(dp)
    for u in rays:
        denominators=A@u
        positive=denominators>1e-12
        candidates.append(u*np.min(b[positive]/denominators[positive]))
    candidates=np.array(candidates)
    values=np.max(candidates@dq.T,axis=1)
    index=int(np.argmin(values))
    return float(values[index]),candidates[index].tolist()


def from_halfplanes(A,b):
    vertices=[]
    for i in range(len(b)):
        for j in range(i):
            M=A[[i,j]]
            if abs(np.linalg.det(M))<1e-10: continue
            x=np.linalg.solve(M,b[[i,j]])
            if np.max(A@x-b)<1e-9: vertices.append(x)
    return hull(vertices)


def geometry(points):
    p=hull(points); m=moments(p); center=np.array(m['centroid']); centered=p-center
    hp=np.max(centered@DIRECTIONS.T,axis=0)
    hm=np.max(-centered@DIRECTIONS.T,axis=0)
    widths=hp+hm
    H=ConvexHull(centered); offsets=-H.equations[:,2]
    polar=hull(H.equations[:,:2]/offsets[:,None])
    polar_area,_=area_centroid(polar)
    d=difference(p); diff_area,_=area_centroid(d)
    A2,witness=two_bounce(p)
    perimeter=float(np.sum(np.linalg.norm(np.roll(p,-1,axis=0)-p,axis=1)))
    m.update({'vertex_count':len(p),'perimeter':perimeter,
              'centroid_polar_area':polar_area,'centroid_polar_area_product':m['area']*polar_area,
              'difference_area':diff_area,'difference_area_excess':diff_area/(4*m['area'])-1,
              'isoperimetric_quotient':4*np.pi*m['area']/perimeter**2,
              'support_odd_rms_over_width':float(np.sqrt(np.mean((hp-hm)**2))/np.mean(widths)),
              'width_anisotropy':float(np.max(widths)/np.min(widths)),
              'A2':A2,'A2_witness':witness,'support_field':hp.tolist(),'width_field':widths.tolist(),
              'polar_vertices':polar.tolist()})
    return m


def body_id(vertices):
    return hashlib.sha256(np.array(vertices,dtype='<f8').tobytes()).hexdigest()[:20]


def bases():
    rng=np.random.default_rng(2026091805)
    result=[]
    def add(name,family,p,source):
        result.append((name,family,hull(p),source))
    for law in ['disk','gaussian']:
        for i,n in enumerate([8,16,36,80]):
            if law=='disk':
                theta=rng.uniform(0,2*np.pi,n); radii=np.sqrt(rng.uniform(0,1,n))
                points=np.column_stack([np.cos(theta),np.sin(theta)])*radii[:,None]
            else: points=rng.normal(size=(n,2))
            H=rotation(float(rng.uniform(0,np.pi)))@np.diag([1+.25*i,1/(1+.25*i)])
            add(f'{law}_{n}',f'cloud_{law}',points@H.T,{'point_count':n,'linear_map':H.tolist(),'raw_points':points.tolist()})
    for i,n in enumerate([5,7,10,14,20,28,40,60]):
        theta=2*np.pi*(np.arange(n)+rng.uniform(-.2,.2,n))/n+rng.uniform(0,2*np.pi)
        A=np.column_stack([np.cos(theta),np.sin(theta)])
        b=np.exp(rng.normal(0,.15,n))
        add(f'halfplanes_{n}','halfplanes',from_halfplanes(A,b),{'normals':A.tolist(),'offsets':b.tolist()})
    for i,n in enumerate([3,5,8,12]):
        g=rng.normal(size=(n,2)); g*=rng.uniform(.3,1.3,n)[:,None]/n
        # Planar zonotope boundary from sorted directed edges.
        edges=np.concatenate([2*g,-2*g]); edges=edges[np.argsort(np.arctan2(edges[:,1],edges[:,0]))]
        p=np.cumsum(edges,axis=0); p-=p.mean(axis=0)
        add(f'zonotope_{n}','zonotope',p,{'generators':g.tolist()})
    for i,n in enumerate([24,36,60,90]):
        phases=rng.uniform(0,2*np.pi,3); theta=2*np.pi*np.arange(n)/n
        b=1+.10*np.cos(2*theta+phases[0])+.055*np.cos(3*theta+phases[1])+.012*np.cos(5*theta+phases[2])
        A=np.column_stack([np.cos(theta),np.sin(theta)])
        add(f'smooth_{n}','smooth_support',from_halfplanes(A,b),{'phases':phases.tolist(),'harmonics':[2,3,5],'amplitudes':[.10,.055,.012],'normals':A.tolist(),'offsets':b.tolist()})
    ts=templates()
    for i in range(4):
        shift=rng.normal(size=(10,2))*(.015+.045*i)
        p=np.concatenate([np.array(t['vertices'])+s for t,s in zip(ts,shift)])
        add(f'cover_{i}','template_hull',p,{'template_translations':shift.tolist()})
    for n,angle in [(3,.17),(4,.17),(5,0),(5,np.pi/10),(7,.17)]:
        add(f'regular_{n}_{angle:.4f}','regular_control',regular(n,np.pi/5+angle),{'n':n,'angle':np.pi/5+angle})
    return result


def collect():
    start=perf_counter(); rows=[]
    def record(name,family,p,source,parent=None):
        # Retain original input coordinates; no silent area normalization.
        p=hull(p); out=evaluate(p); g=geometry(p)
        fields={k:g.pop(k) for k in ['support_field','width_field','polar_vertices','centered_monomials','covariance']}
        g['capacity']=out['capacity']; g['sys']=out['sys']; g['capacity_over_A2']=out['capacity']/g['A2']
        g['fit_max_over_min']=max(f['scale'] for f in out['fits'])/out['capacity']
        out={'body_id':body_id(p),'name':name,'family':family,'parent_id':parent,
             'source':source,'measurement_version':VERSION,'vertices':p.tolist(),
             'scalars':g,'fields':fields,'fits':out['fits'],'facet_normals':out['facet_normals'],
             'facet_offsets':out['facet_offsets'],'evaluation_seconds':out['seconds']}
        rows.append(out); return out
    all_bases=bases()
    for name,family,p,source in all_bases: record(name,family,p,source)
    selected=['disk_16','halfplanes_14','smooth_36','regular_5_0.3142']
    for name in selected:
        parent=next(row for row in rows if row['name']==name); p=np.array(parent['vertices'])
        for t in [.125,.25,.375,.5]:
            pt=hull(((1-t)*p[:,None,:]-t*p[None,:,:]).reshape(-1,2))
            record(f'{name}_sym_{t}','symmetrization_path',pt,{'operation':'(1-t)K+t(-K)','t':t},parent['body_id'])
    out=ROOT/'collection-v1.jsonl'
    out.write_text(''.join(json.dumps(row,separators=(',',':'))+'\n' for row in rows))
    meta={'measurement_version':VERSION,'seed':2026091805,'base_count':len(all_bases),'row_count':len(rows),
          'seconds':perf_counter()-start,'width_grid_angles':ANGLES.tolist(),
          'Q_vertices':Q.tolist(),'units':'raw coordinates; area degree2; lengths,capacity,A2 degree1 in K',
          'status':'exploratory floating-point; no exact capacity certification',
          'script_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
          'evaluator_sha256':hashlib.sha256((ROOT/'evaluator.py').read_bytes()).hexdigest(),
          'data_sha256':hashlib.sha256(out.read_bytes()).hexdigest()}
    (ROOT/'collection-v1-metadata.json').write_text(json.dumps(meta,indent=2)+'\n')
    print(json.dumps({k:v for k,v in meta.items() if k not in ['width_grid_angles','Q_vertices']},indent=2))
    return rows


def checks():
    square=regular(4,np.pi/4)*np.sqrt(2)
    m=moments(square)
    assert abs(m['area']-4)<1e-12
    assert np.max(abs(np.array(m['covariance'])-np.eye(2)/3))<1e-12
    assert abs(m['centered_monomials']['4,0']-.2)<1e-12
    assert abs(m['centered_monomials']['2,2']-1/9)<1e-12
    triangle=np.array([[0.,0.],[1.,0.],[0.,1.]])
    assert abs(monomial(triangle,2,1)-1/30)<1e-12
    assert abs(two_bounce(square,square)[0]-4)<1e-12
    # Area-moment affine laws and translation-independent noncentral invariants.
    H=np.array([[1.2,.4],[-.2,.9]])
    transformed=moments(square@H.T+[2,-3])
    assert np.max(abs(np.array(transformed['covariance'])-H@np.array(m['covariance'])@H.T))<1e-11
    assert abs(transformed['affine_radial_fourth_moment']-m['affine_radial_fourth_moment'])<1e-10

if __name__=='__main__':
    checks()
    collect()
