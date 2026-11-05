#!/usr/bin/env python3
"""Frozen two angular-shadow words, one A->B response each. No all-pair capacity search."""
import argparse,csv,gzip,hashlib,importlib.util,json,math,time
from fractions import Fraction as F
from functools import cmp_to_key
from itertools import combinations
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3] if 'patterns' in Path(__file__).parts else Path('/workspaces/msc-math-worktrees/patterns-2026-10-17')
spec=importlib.util.spec_from_file_location('contact',ROOT/'patterns/upper-tail/geometric-selector/analyze.py');old=importlib.util.module_from_spec(spec);spec.loader.exec_module(old)
def dot(a,b):return sum(x*y for x,y in zip(a,b))
def cross(a,b):return a[0]*b[1]-a[1]*b[0]
def closures(a,tol=0):
    out=[];n=len(a)
    for i,j in combinations(range(n),2):
        if abs(cross(a[i],a[j]))<=tol and dot(a[i],a[j])<0:
            c=next(k for k in range(2) if abs(a[i][k])>tol);w=-a[j][c]/(a[i][c]-a[j][c]);v=[0]*n;v[i]=w;v[j]=1-w;out.append(v)
    for ids in combinations(range(n),3):
        x,y,z=[a[i] for i in ids];c=[cross(y,z),cross(z,x),cross(x,y)];s=sum(c)
        if abs(s)<=tol:continue
        w=[v/s for v in c]
        if min(w)<=tol:continue
        v=[0]*n
        for i,q in zip(ids,w):v[i]=q
        out.append(v)
    assert out
    return out

def shadow_word(a,b,sign,tol=0):
    vectors=a+[(sign*-v[1],sign*v[0]) for v in b]
    def half(v):return 0 if v[1]>tol or (abs(v[1])<=tol and v[0]>=0) else 1
    def cmp(i,j):
        u,v=vectors[i],vectors[j];h=half(u)-half(v)
        if h:return h
        d=cross(u,v)
        if abs(d)>tol:return -1 if d>0 else 1
        return (i>j)-(i<j)
    return sorted(range(len(vectors)),key=cmp_to_key(cmp))

def Q(a,b,word,alpha,gamma):
    n=len(a);pos={i:p for p,i in enumerate(word)}
    return sum(alpha[i]*gamma[j]*dot(u,v)*(1 if pos[i]<pos[n+j] else -1) for i,u in enumerate(a) for j,v in enumerate(b))/4

def coefficients(a,b,word,other,which):
    n=len(a);pos={i:p for p,i in enumerate(word)}
    if which=='A':return [sum(other[j]*dot(u,v)*(1 if pos[i]<pos[n+j] else -1) for j,v in enumerate(b))/4 for i,u in enumerate(a)]
    return [sum(other[i]*dot(u,v)*(1 if pos[i]<pos[n+j] else -1) for i,u in enumerate(a))/4 for j,v in enumerate(b)]

def response(a,b,word,alpha,gamma,ca,cb,tol=0):
    if Q(a,b,word,alpha,gamma)<0:word=word[::-1]
    initial=Q(a,b,word,alpha,gamma);co=coefficients(a,b,word,gamma,'A');best=max(ca,key=lambda w:dot(w,co));candidate=dot(best,co)
    if candidate>initial+tol:alpha=best
    mid=Q(a,b,word,alpha,gamma);ac=co;co=coefficients(a,b,word,alpha,'B');best=max(cb,key=lambda w:dot(w,co));candidate=dot(best,co)
    if candidate>mid+tol:gamma=best
    final=Q(a,b,word,alpha,gamma)
    assert final+tol>=mid and mid+tol>=initial
    return dict(word=word,seed_Q=initial,after_A_Q=mid,Q=final,alpha=alpha,gamma=gamma,A_coefficients=ac,B_coefficients=co,capacity=1/(2*final) if final>0 else None)

def select(a,b,tol=0):
    seed=old.select(a,b,tol);best=seed['best'];n=len(a);alpha=[0]*n;gamma=[0]*len(b);si,sj=best['supports']
    for i,w in zip(si,best['weights'][:len(si)]):alpha[i]=2*w
    for j,w in zip(sj,best['weights'][len(si):]):gamma[j]=2*w
    ca=closures(a,tol);cb=closures(b,tol)
    variants=[response(a,b,shadow_word(a,b,sign,tol),alpha[:],gamma[:],ca,cb,tol)|{'shadow_sign':sign} for sign in [1,-1]]
    chosen=max(variants,key=lambda r:r['Q']);use=chosen['Q']>best['Q']+tol
    fullword=[si[i] if i<len(si) else n+sj[i-len(si)] for i in best['word']]
    witness=chosen if use else dict(word=fullword,Q=best['Q'],alpha=alpha,gamma=gamma,capacity=best['capacity'])
    return dict(seed=seed,seed_alpha=alpha,seed_gamma=gamma,variants=variants,chosen_source=('shadow_'+str(chosen['shadow_sign'])) if use else 'contact',witness=witness,capacity=witness['capacity'],LP_vertex_counts=[len(ca),len(cb)])

def stringify(x):
    if isinstance(x,F):return str(x)
    if isinstance(x,dict):return {k:stringify(v) for k,v in x.items()}
    if isinstance(x,(tuple,list)):return [stringify(v) for v in x]
    return x

def main():
    p=argparse.ArgumentParser();p.add_argument('--source',type=Path,required=True);p.add_argument('--width',type=Path,required=True);p.add_argument('--out',type=Path,required=True);args=p.parse_args();start=time.perf_counter()
    assert hashlib.sha256(args.source.read_bytes()).hexdigest()==old.SOURCE_SHA
    with gzip.open(args.width,'rt') as f:wr=list(csv.DictReader(f))
    names=[]
    for f in ['results.json','stress-results.json']:
        names.extend(r['name'] for r in json.loads((ROOT/'patterns/upper-tail/geometric-selector'/f).read_text())['rows'])
    principal=[]
    for bucket in [(5,5),(5,6),(6,6)]:principal.extend(r['name'] for r in sorted((r for r in wr if (int(r['k']),int(r['m']))==bucket),key=lambda r:(-float(r['sys']),r['name']))[:3])
    names=list(dict.fromkeys(names+principal));assert len(names)==20
    geometry={}
    for line in args.source.open():
        r=json.loads(line)
        if r['name'] in names:geometry[r['name']]=r
    selected={}
    for name in names:
        r=geometry[name];v=[list(map(F,u)) for u in r['dual_vertices_rational']];a=[u[:2] for u in v[:r['k']]];b=[u[2:] for u in v[r['k']:]]
        selected[name]=(select(a,b),old.area(a)*old.area(b))
    rows=[]
    for name in names:
        r=geometry[name];s,volume=selected[name];cap=s['capacity'];seed=s['seed']['best']['capacity'];ratio=cap*cap/(2*volume)
        rows.append(dict(name=name,principal_top3=name in principal,selection=s,exact_volume=volume,exact_ratio_upper_bound=ratio,ratio_upper_bound=float(ratio),historical_capacity=r['capacity'],historical_ratio=r['sys'],capacity_over_historical=float(cap)/r['capacity'],contact_over_historical=float(seed)/r['capacity'],relative_gain_over_contact=1-float(cap/seed)))
    regular=[];h=math.cos(math.pi/5);vol=(5*math.sin(2*math.pi/5)/2)**2
    for theta in [0,math.pi/20,math.pi/10]:
        a=[(math.cos((2*i+1)*math.pi/5)/h,math.sin((2*i+1)*math.pi/5)/h) for i in range(5)];b=[(math.cos((2*i+1)*math.pi/5+theta)/h,math.sin((2*i+1)*math.pi/5+theta)/h) for i in range(5)]
        s=select(a,b,1e-11);cap=s['capacity'];known=(1+h)**2/math.cos(theta)
        assert abs(cap/known-1)<1e-10
        regular.append(dict(theta=theta,selection=s,known_capacity=known,ratio=cap*cap/(2*vol),known_ratio=known*known/(2*vol)))
    result=dict(method='two fixed angular shadow words; exactly one A->B normalized-closure LP sweep each; retain contact minimum',source_sha256=old.SOURCE_SHA,plan_sha256=hashlib.sha256((args.out/'PLAN.md').read_bytes()).hexdigest(),analyze_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),rows=rows,principal_names=principal,regular_controls=regular,elapsed_seconds=time.perf_counter()-start)
    (args.out/'results.json').write_text(json.dumps(stringify(result),indent=2)+'\n')
    print(json.dumps([dict(name=r['name'],tail=r['principal_top3'],gap=r['capacity_over_historical']-1,oldgap=r['contact_over_historical']-1,gain=r['relative_gain_over_contact'],source=r['selection']['chosen_source']) for r in rows],indent=2));print('SECONDS',result['elapsed_seconds'])
if __name__=='__main__':main()
