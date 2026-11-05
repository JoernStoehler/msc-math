#!/usr/bin/env python3
"""One exact selected circumscribed-triangle bound, 26 retained width survivors.
Standard library only; HKO is a separate floating/known-formula control.
No capacity-evaluator or historical target search is called.
"""
from fractions import Fraction as F
from itertools import combinations,permutations
from functools import cmp_to_key
from pathlib import Path
import argparse,csv,gzip,hashlib,json,math,time
SOURCE_SHA='66bf82010e92e0f26b0df226f4e6c0eef05d21eb22a0967c7f669530f6545736'
WIDTH_SHA='dcffead16ab5ad0fcdf6ae55d78375bea5b6ee7274877a33846cf4c3c2a7d57f'
def cross(a,b):return a[0]*b[1]-a[1]*b[0]
def dot(a,b):return a[0]*b[0]+a[1]*b[1]
def polar_order(a):
    def half(x):return int(x[1]<0 or (x[1]==0 and x[0]<0))
    def cmp(x,y):
        if half(x)!=half(y):return half(x)-half(y)
        z=cross(x,y);return (z<0)-(z>0)
    return sorted(a,key=cmp_to_key(cmp))
def vertices(a):
    a=polar_order(a);v=[]
    for i,u in enumerate(a):
        w=a[(i+1)%len(a)];d=cross(u,w)
        assert d!=0
        v.append(((w[1]-u[1])/d,(u[0]-w[0])/d))
    assert all(dot(x,y)<=1+1e-12 if isinstance(x[0],float) else dot(x,y)<=1 for x in a for y in v)
    return v
def area(a):
    v=vertices(a);return abs(sum(cross(v[i],v[(i+1)%len(v)]) for i in range(len(v))))/2

def triangles(a,tolerance=0.):
    out=[]
    for idx in combinations(range(len(a)),3):
        u=[a[i] for i in idx];cc=[cross(u[1],u[2]),cross(u[2],u[0]),cross(u[0],u[1])];s=sum(cc)
        if s==0:continue
        weights=[v/s for v in cc]
        if min(weights)<=0:continue
        ar=area(u);out.append(dict(indices=idx,normals=u,weights=weights,area=ar))
    assert out
    best=min(r['area'] for r in out)
    return [r for r in out if abs(r['area']-best)<=tolerance],len(out)

ORDERS=[(0,)+tail for tail in permutations(range(1,6))]
# Each is one cyclic distinct-facet word; all weights are strictly positive.
def bound(p,q):
    u=p['normals'];v=q['normals'];a=p['weights'];b=q['weights'];vals=[]
    h=[[a[i]*b[j]*dot(u[i],v[j])/4 for j in range(3)] for i in range(3)]
    for word in ORDERS:
        pos={j:i for i,j in enumerate(word)}
        vals.append(sum(h[i][j] if pos[i]<pos[j+3] else -h[i][j] for i in range(3) for j in range(3)))
    Q=max(vals);assert Q>0
    return 1/(2*Q)

def witness(p,q):
    n=[(u[0],u[1],F(0),F(0)) for u in p['normals']]+[(F(0),F(0),v[0],v[1]) for v in q['normals']]
    weights=[x/2 for x in p['weights']+q['weights']]
    def omega(x,y):return x[0]*y[2]+x[1]*y[3]-x[2]*y[0]-x[3]*y[1]
    def Q(word):return sum(weights[i]*weights[j]*omega(n[i],n[j]) for pos,i in enumerate(word) for j in word[pos+1:])
    word=max(ORDERS,key=Q);value=Q(word)
    assert 1/(2*value)==bound(p,q)
    return dict(word=word,weights=[str(x) for x in weights],exact_Q=str(value))

def hko():
    n=5;h=math.cos(math.pi/5)
    a=[(math.cos((2*i+1)*math.pi/5)/h,math.sin((2*i+1)*math.pi/5)/h) for i in range(5)]
    b=[(math.cos((2*i+1)*math.pi/5+math.pi/10)/h,math.sin((2*i+1)*math.pi/5+math.pi/10)/h) for i in range(5)]
    p,_=triangles(a,tolerance=1e-12);q,_=triangles(b,tolerance=1e-12)
    cap=min(bound(x,y) for x in p for y in q);vol=area(a)*area(b);s=cap*cap/(2*vol)
    known_sys=(3+math.sqrt(5))/5;known_c2=(1+h)**2/math.cos(math.pi/10)
    assert s>=known_sys-1e-12
    return dict(kind='Separate regular-pentagon numerical control; exact known sys imported, not certified here',selected_triangles=[len(p),len(q)],word_evaluations=120*len(p)*len(q),triangle_capacity_upper_bound=cap,triangle_ratio_upper_bound=s,known_width_capacity=known_c2,known_exact_ratio_expression='(3+sqrt(5))/5',known_ratio=known_sys,tightens_width=cap<known_c2-1e-12)
def main():
    ap=argparse.ArgumentParser();ap.add_argument('--source',type=Path,required=True);ap.add_argument('--width',type=Path,required=True);ap.add_argument('--out',type=Path,required=True);args=ap.parse_args();start=time.perf_counter()
    assert hashlib.sha256(args.source.read_bytes()).hexdigest()==SOURCE_SHA
    assert hashlib.sha256(args.width.read_bytes()).hexdigest()==WIDTH_SHA
    with gzip.open(args.width,'rt') as f:ww={r['name']:r for r in csv.DictReader(f) if int(r['k'])>=5 and float(r['sys2'])>1}
    assert len(ww)==26;rows=[]
    for line in args.source.open():
        r=json.loads(line)
        if r['name'] not in ww:continue
        w=ww[r['name']];aa=[[F(v) for v in u] for u in r['dual_vertices_rational']];a=[u[:2] for u in aa[:r['k']]];b=[u[2:] for u in aa[r['k']:]]
        p,np=triangles(a);q,nq=triangles(b);selected=[(bound(x,y),x,y) for x in p for y in q];cap,x,y=min(selected,key=lambda x:x[0]);vol=area(a)*area(b);s=cap*cap/(2*vol)
        assert all(sum(x['weights'][j]*x['normals'][j][k] for j in range(3))==0 for k in range(2))
        assert all(sum(y['weights'][j]*y['normals'][j][k] for j in range(3))==0 for k in range(2))
        rows.append(dict(name=r['name'],bucket=f"{r['k']}x{r['m']}",historical_sys=r['sys'],width_ratio_upper_bound=float(w['sys2']),triangle_ratio_upper_bound=float(s),exact_triangle_ratio=str(s),exact_original_area_product=str(vol),certificate=witness(x,y),selected_support_indices=[x['indices'],y['indices']],feasible_triangle_counts=[np,nq],selected_min_area_tie_counts=[len(p),len(q)],word_evaluations=120*len(p)*len(q),tightens_width=float(cap)<float(w['c2'])-1e-10,rejects_above_one_exact=s<=1))
    assert len(rows)==26
    report=dict(method='Minimum-area circumscribed supporting triangles only; exact rational historical input geometry; complete120cyclic words per selected triangle pair, no full body evaluator.',source_sha256=SOURCE_SHA,width_sha256=WIDTH_SHA,rows=rows,summary=dict(rows=len(rows),tightens_width=sum(r['tightens_width'] for r in rows),rejects_above_one_exact=sum(r['rejects_above_one_exact'] for r in rows),word_evaluations=sum(r['word_evaluations'] for r in rows)),hko=hko(),elapsed_seconds=time.perf_counter()-start)
    args.out.parent.mkdir(parents=True,exist_ok=True);args.out.write_text(json.dumps(report,indent=2)+'\n');print(json.dumps({k:v for k,v in report.items() if k not in ['rows']},indent=2))
if __name__=='__main__':main()
