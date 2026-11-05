"""Independent retained-row arithmetic and regular synthetic width check.
No capacity producer and no imports from worker scripts.
"""
import argparse,gzip,json,math,hashlib
from pathlib import Path
import numpy as np

parser=argparse.ArgumentParser(description=__doc__)
parser.add_argument('--rows',type=Path,required=True)
parser.add_argument('--summary',type=Path,required=True)
parser.add_argument('--out',type=Path,required=True)
args=parser.parse_args()
rows=list(map(json.loads,gzip.open(args.rows,'rt')))
summary=json.loads(args.summary.read_text())
S=2.5*math.sin(2*math.pi/5)
w=1+math.cos(math.pi/5)
HKO=(3+math.sqrt(5))/5
normals=[(math.cos(math.pi/5+2*math.pi*j/5),math.sin(math.pi/5+2*math.pi*j/5)) for j in range(5)]

def inv2(g):
    a,b=g[0];c,d=g[1];det=a*d-b*c
    return [[d/det,-b/det],[-c/det,a/det]]

def calc_l(p,q):
    a=inv2(p['G']);b=inv2(q['G'])
    m=[[sum(a[i][k]*b[j][k] for k in range(2)) for j in range(2)] for i in range(2)]
    den=max(abs(sum(x[i]*m[i][j]*y[j] for i in range(2) for j in range(2))) for x in normals for y in normals)
    return w*w/den

def describe(rr):
    c=np.array([r['capacity'] for r in rr])
    p=np.array([math.sqrt(r['L']*r['width']) for r in rr])
    b=np.array([r['width'] for r in rr])
    pe=np.abs(p/c-1);be=np.abs(b/c-1)
    cov=np.array([r['p']['coverage']*r['q']['coverage'] for r in rr])
    own=np.array([r['core_own_sys'] for r in rr])
    return dict(n=len(rr),center_relative_error_median_q90_max=np.quantile(pe,[.5,.9,1]).tolist(),
                width_relative_error_median_q90_max=np.quantile(be,[.5,.9,1]).tolist(),
                center_beats_width=int(sum(abs(float(p[i]-c[i]))<abs(float(b[i]-c[i]))-1e-12*max(1,c[i]) for i in range(len(rr)))),
                MAE_center_over_width=float(np.mean(abs(p-c))/np.mean(abs(b-c))),
                coverage_product_median=float(np.median(cov)),core_own_sys_median=float(np.median(own)),
                own_to_HKO_median=float(np.median(own/HKO)),
                center_sys_abs_error_median_max=np.quantile([abs(r['prediction_sys']-r['sys']) for r in rr],[.5,1]).tolist(),
                log_area_loss_median=float(np.median(-np.log(cov))),
                log_family_deficit_median=float(np.median(-np.log(own/HKO))))

result={'rows':len(rows),'row_file_sha256':hashlib.sha256((args.rows).read_bytes()).hexdigest(),'buckets':{}}
assert len(rows)==10240
assert len({r['name'] for r in rows})==len(rows)
formula_error=max(abs(calc_l(r['p'],r['q'])/r['L']-1) for r in rows)
bracket_violations=[r['name'] for r in rows if r['L']>r['capacity']*(1+1e-8) or r['capacity']>r['width']*(1+1e-8)]
decomposition_error=max(abs(r['core_sys']-r['coverage_product']*r['core_own_sys']) for r in rows)
assert formula_error<1e-9 and not bracket_violations and decomposition_error<1e-12
result.update(max_scalar_formula_relative_difference=formula_error,bracket_violations=bracket_violations,max_area_decomposition_abs_difference=decomposition_error)
combined=[]
for bucket in ['5x5','5x6','6x6']:
    rr=[r for r in rows if r['bucket']==bucket]
    top=sorted(rr,key=lambda r:(-r['sys'],r['name']))[:math.ceil(.02*len(rr))]
    assert [r['name'] for r in top]==summary['buckets'][bucket]['top2_names']
    d=describe(top);s=summary['buckets'][bucket]['top2']
    assert d['center_beats_width']==s['center_beats_width_capacity_count']
    assert abs(d['MAE_center_over_width']-s['center_over_width_capacity_MAE'])<1e-12
    for field,tag in [('center_relative_error_median_q90_max','center'),('width_relative_error_median_q90_max','width')]:
        expected=[s[tag]['capacity_absolute_relative'][q] for q in ['median','q90','max']]
        assert max(abs(a-b) for a,b in zip(d[field],expected))<1e-14
    result['buckets'][bucket]=d;combined+=top
result['combined_top2']=describe(combined)

def hull(points):
    pp=sorted(set(tuple(map(float,x)) for x in points))
    def cross(o,a,b):return (a[0]-o[0])*(b[1]-o[1])-(a[1]-o[1])*(b[0]-o[0])
    lo=[];up=[]
    for p in pp:
        while len(lo)>=2 and cross(lo[-2],lo[-1],p)<=1e-13:lo.pop()
        lo.append(p)
    for p in reversed(pp):
        while len(up)>=2 and cross(up[-2],up[-1],p)<=1e-13:up.pop()
        up.append(p)
    return lo[:-1]+up[:-1]

def difference_polar(verts):
    d=hull([(a[0]-b[0],a[1]-b[1]) for a in verts for b in verts]);ans=[]
    for i,a in enumerate(d):
        b=d[(i+1)%len(d)];n=(b[1]-a[1],a[0]-b[0]);h=n[0]*a[0]+n[1]*a[1]
        assert h>0;ans.append((n[0]/h,n[1]/h))
    return ans

vv=[(math.cos(2*math.pi*j/5),math.sin(2*math.pi*j/5)) for j in range(5)]
dp=difference_polar(vv);controls=[]
for r in summary['HKO_control']['regular_rotation_controls']:
    theta=r['theta'];ct=math.cos(theta);st=math.sin(theta)
    q=[(ct*x-st*y,st*x+ct*y) for x,y in vv];dq=difference_polar(q)
    width=1/max(x[0]*y[0]+x[1]*y[1] for x in dp for y in dq)
    center=math.sqrt(r['L']*width);sys_hat=center*center/(2*S*S)
    assert abs(width/r['known_capacity']-1)<1e-12
    assert abs(sys_hat-r['predicted_sys'])<1e-12
    controls.append(dict(theta=theta,width_from_difference_polars=width,known_capacity=r['known_capacity'],width_relative_difference=width/r['known_capacity']-1,center_sys=sys_hat))
assert abs(controls[-1]['center_sys']-HKO)<1e-12
result['regular_width_controls']=controls
result['limits']='Retained binary64 arithmetic and local existing affine theorem checked; no capacity optimization, new exact certificate, full proof re-audit, external literature search or prospective test.'
args.out.write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps(result,indent=2))
