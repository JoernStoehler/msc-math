#!/usr/bin/env python3
"""Verify sparse HK upper-bound witnesses and areas independently of search.
Standard library only. No triangle/word optimization or historical capacity.
"""
from fractions import Fraction as F
from itertools import combinations
from pathlib import Path
import argparse,hashlib,json

def cross(a,b):return a[0]*b[1]-a[1]*b[0]
def hull(v):
    points=sorted(set(v));lo=[];hi=[]
    def turn(a,b,c):return cross((b[0]-a[0],b[1]-a[1]),(c[0]-a[0],c[1]-a[1]))
    for p in points:
        while len(lo)>1 and turn(lo[-2],lo[-1],p)<=0:lo.pop()
        lo.append(p)
    for p in reversed(points):
        while len(hi)>1 and turn(hi[-2],hi[-1],p)<=0:hi.pop()
        hi.append(p)
    return lo[:-1]+hi[:-1]
def area_from_all_intersections(normals):
    vertices=[]
    for a,b in combinations(normals,2):
        d=cross(a,b)
        if not d:continue
        x=((b[1]-a[1])/d,(a[0]-b[0])/d)
        if all(u[0]*x[0]+u[1]*x[1]<=1 for u in normals):vertices.append(x)
    vv=hull(vertices);assert len(vv)>=3
    return abs(sum(cross(vv[i],vv[(i+1)%len(vv)]) for i in range(len(vv))))/2

def main():
    ap=argparse.ArgumentParser();ap.add_argument('--source',type=Path,required=True);ap.add_argument('--certificate',type=Path,required=True);args=ap.parse_args()
    report=json.loads(args.certificate.read_text());assert hashlib.sha256(args.source.read_bytes()).hexdigest()==report['source_sha256']
    cert={r['name']:r for r in report['rows']};assert len(cert)==26
    verified=0;rejected=0;max_rejected=F(0)
    for line in args.source.open():
        r=json.loads(line)
        if r['name'] not in cert:continue
        row=cert.pop(r['name']);a=[[F(z) for z in u] for u in r['dual_vertices_rational']];p=[u[:2] for u in a[:r['k']]];q=[u[2:] for u in a[r['k']:]];vol=area_from_all_intersections(p)*area_from_all_intersections(q)
        assert vol==F(row['exact_original_area_product'])
        i,j=row['selected_support_indices'];normals=[a[x] for x in i]+[a[r['k']+x] for x in j]
        c=row['certificate'];weights=list(map(F,c['weights']));word=c['word'];assert sorted(word)==list(range(6));assert min(weights)>0
        assert sum(weights[:3])==sum(weights[3:])==F(1,2)
        assert all(sum(w*u[k] for w,u in zip(weights,normals))==0 for k in range(4))
        def omega(x,y):return x[0]*y[2]+x[1]*y[3]-x[2]*y[0]-x[3]*y[1]
        Q=sum(weights[x]*weights[y]*omega(normals[x],normals[y]) for pos,x in enumerate(word) for y in word[pos+1:]);assert Q==F(c['exact_Q']) and Q>0
        # Any feasible HK word has Q <= Qmax. Hence c <= 1/(2Q).
        ratio=1/(8*Q*Q*vol);assert ratio==F(row['exact_triangle_ratio'])
        assert (ratio<=1)==row['rejects_above_one_exact']
        verified+=1;rejected+=ratio<=1
        if ratio<=1:max_rejected=max(max_rejected,ratio)
    assert not cert and verified==26 and rejected==report['summary']['rejects_above_one_exact']==25
    print(json.dumps(dict(status='PASS',rational_upper_bound_witnesses=verified,exact_above_one_rejections=rejected,largest_rejected_upper_bound=float(max_rejected),hko='Separate numerical control excluded from this exact verifier')))
if __name__=='__main__':main()
