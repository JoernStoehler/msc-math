#!/usr/bin/env python3
"""Frozen width-contact selector on ten fixed retrospective controls. Stdlib only.
Reads only registered geometry and retained labels; writes --out only.
No body-capacity producer, support-pair search, fitting or adaptive support change.
"""
from fractions import Fraction as F
from itertools import combinations, permutations
from pathlib import Path
import argparse, csv, gzip, hashlib, json, math, sys, time
SOURCE_SHA='66bf82010e92e0f26b0df226f4e6c0eef05d21eb22a0967c7f669530f6545736'
WIDTH_SHA='dcffead16ab5ad0fcdf6ae55d78375bea5b6ee7274877a33846cf4c3c2a7d57f'
FIXED=['random_5x5_s1_64','random_5x6_s2_150','random_6x6_s3_124','random_5x5_s3_114','random_5x6_s3_29','random_6x6_s2_3','random_5x5_s1_221']
BUCKETS=[(5,5),(5,6),(6,6)]
def dot(a,b):return sum(x*y for x,y in zip(a,b))
def cross(a,b):return a[0]*b[1]-a[1]*b[0]
def vertices(a,tol=0):
    out=[]
    for i,j in combinations(range(len(a)),2):
        d=cross(a[i],a[j])
        if abs(d)<=tol:continue
        v=((a[j][1]-a[i][1])/d,(a[i][0]-a[j][0])/d)
        if all(dot(u,v)<=1+tol for u in a):out.append(v)
    return list(dict.fromkeys(out))
def area(a):
    vv=vertices(a);cx=sum(v[0] for v in vv)/len(vv);cy=sum(v[1] for v in vv)/len(vv)
    vv.sort(key=lambda v:math.atan2(v[1]-cy,v[0]-cx))
    return abs(sum(cross(vv[i],vv[(i+1)%len(vv)]) for i in range(len(vv))))/2

def closures(a,tol=0):
    """LP-maximal anchor closures, selected by opposite support geometry only."""
    vv=vertices(a,tol);result=[]
    for i,u in enumerate(a):
        hh=max(-dot(u,v) for v in vv);alpha=1/(1+hh);supports={}
        for v in vv:
            if abs(-dot(u,v)-hh)>tol:continue
            active=[j for j,z in enumerate(a) if abs(dot(z,v)-1)<=tol]
            for j in active:
                if abs(cross(u,a[j]))<=tol and dot(u,a[j])<0:
                    # alpha*u+(1-alpha)*a[j]=0, independently checked below.
                    ids=tuple(sorted((i,j)));ww=[alpha if z==i else 1-alpha for z in ids]
                    if all(abs(sum(w*a[z][c] for z,w in zip(ids,ww)))<=tol for c in range(2)):
                        supports[ids]=ww
            for j,k in combinations(active,2):
                ids=tuple(sorted((i,j,k)));uu=[a[z] for z in ids]
                cc=[cross(uu[1],uu[2]),cross(uu[2],uu[0]),cross(uu[0],uu[1])];ss=sum(cc)
                if abs(ss)<=tol:continue
                ww=[z/ss for z in cc]
                if min(ww)<=tol:continue
                assert abs(ww[ids.index(i)]-alpha)<=tol
                supports[ids]=ww
        assert supports,(i,a)
        result.append(dict(anchor=i,alpha=alpha,step=tuple(alpha*z for z in u),supports=[dict(indices=ids,weights=ww) for ids,ww in sorted(supports.items())]))
    return result

def best_word(a,b,p,q):
    us=[a[i] for i in p['indices']];vs=[b[j] for j in q['indices']]
    n=len(us);m=len(vs);weights=[v/2 for v in p['weights']+q['weights']]
    best=None;count=0
    for tail in permutations(range(1,n+m)):
        word=(0,)+tail;pos={v:i for i,v in enumerate(word)}
        value=sum(weights[i]*weights[n+j]*dot(u,v)*(1 if pos[i]<pos[n+j] else -1) for i,u in enumerate(us) for j,v in enumerate(vs));count+=1
        if best is None or value>best[0]:best=(value,word)
    Q,word=best;assert Q>0
    return dict(capacity=1/(2*Q),Q=Q,word=word,weights=weights,supports=[p['indices'],q['indices']],word_evaluations=count)

def select(a,b,tol=0):
    ca=closures(a,tol);cb=closures(b,tol)
    contacts=[(abs(dot(x['step'],y['step'])),x,y) for x in ca for y in cb];M=max(z[0] for z in contacts)
    selected=[(x,y) for z,x,y in contacts if abs(z-M)<=tol]
    pairs={}
    for x,y in selected:
        for p in x['supports']:
            for q in y['supports']:pairs[(p['indices'],q['indices'])]=(p,q)
    words=[best_word(a,b,p,q) for p,q in pairs.values()];best=min(words,key=lambda z:z['capacity'])
    assert best['capacity']<=1/M+tol
    return dict(width_capacity=1/M,anchor_contacts=[(x['anchor'],y['anchor']) for x,y in selected],support_pair_count=len(pairs),anchor_closure_counts=[sum(len(x['supports']) for x in cc) for cc in [ca,cb]],possible_support_pairs=len(ca)*len(cb),word_evaluations=sum(x['word_evaluations'] for x in words),best=best)

def stringify(x):
    if isinstance(x,F):return str(x)
    if isinstance(x,dict):return {k:stringify(v) for k,v in x.items()}
    if isinstance(x,(list,tuple)):return [stringify(v) for v in x]
    return x

def regular_controls():
    out=[];h=math.cos(math.pi/5)
    for theta in [0,math.pi/20,math.pi/10]:
        a=[(math.cos((2*i+1)*math.pi/5)/h,math.sin((2*i+1)*math.pi/5)/h) for i in range(5)]
        b=[(math.cos((2*i+1)*math.pi/5+theta)/h,math.sin((2*i+1)*math.pi/5+theta)/h) for i in range(5)]
        s=select(a,b,1e-11);known=(1+h)**2/math.cos(theta);vol=(5*math.sin(2*math.pi/5)/2)**2
        assert abs(s['best']['capacity']/known-1)<1e-10
        out.append(dict(angle=theta,known_capacity=known,known_ratio=known**2/(2*vol),selected=s,relative_capacity_error=s['best']['capacity']/known-1,tie_tolerance_absolute=1e-11))
    return out

def main():
    ap=argparse.ArgumentParser();ap.add_argument('--source',type=Path,required=True);ap.add_argument('--width',type=Path,required=True);ap.add_argument('--out',type=Path,required=True);aa=ap.parse_args();start=time.perf_counter()
    assert hashlib.sha256(aa.source.read_bytes()).hexdigest()==SOURCE_SHA
    assert hashlib.sha256(aa.width.read_bytes()).hexdigest()==WIDTH_SHA
    with gzip.open(aa.width,'rt') as f:width={r['name']:r for r in csv.DictReader(f)}
    hash_names=[min((r['name'] for r in width.values() if (int(r['k']),int(r['m']))==bucket),key=lambda n:hashlib.sha256(n.encode()).hexdigest()) for bucket in BUCKETS]
    names=FIXED+hash_names;assert len(set(names))==10;geometry={}
    for line in aa.source.open():
        r=json.loads(line)
        if r['name'] in names:geometry[r['name']]=r
    assert len(geometry)==10;selected={}
    # Complete geometry-only selection before accessing any retained capacity label.
    for name in names:
        r=geometry[name];a=[[F(z) for z in u] for u in r['dual_vertices_rational']];p=[u[:2] for u in a[:r['k']]];q=[u[2:] for u in a[r['k']:]]
        selected[name]=(select(p,q),area(p)*area(q))
    rows=[]
    for name in names:
        r=geometry[name];s,volume=selected[name];cap=s['best']['capacity'];wc=s['width_capacity'];w=width[name]
        assert abs(float(wc)/float(w['c2'])-1)<1e-9
        ratio=cap*cap/(2*volume)
        rows.append(dict(name=name,bucket=f"{r['k']}x{r['m']}",sample_kind='old-rule hard case' if name in FIXED else 'name-hash control',selection=s,exact_original_area_product=volume,exact_ratio_upper_bound=ratio,ratio_upper_bound=float(ratio),historical_capacity=r['capacity'],historical_ratio=r['sys'],capacity_over_historical=float(cap)/r['capacity'],width_over_historical=float(wc)/r['capacity'],relative_gain_over_width=1-float(cap/wc),exact_bound_below_one=ratio<=1))
    report=dict(method='Frozen LP-maximal closure at width-attaining anchor pairs only; no added supports',source_sha256=SOURCE_SHA,width_sha256=WIDTH_SHA,retrospective_control_names=names,rows=rows,regular_controls=regular_controls(),elapsed_seconds=time.perf_counter()-start)
    aa.out.mkdir(parents=True,exist_ok=True);(aa.out/'results.json').write_text(json.dumps(stringify(report),indent=2)+'\n')
    print(json.dumps([{k:r[k] for k in ['name','sample_kind','ratio_upper_bound','historical_ratio','capacity_over_historical','width_over_historical','relative_gain_over_width','exact_bound_below_one']}|{'support_pairs':r['selection']['support_pair_count']} for r in rows],indent=2));print('SECONDS',report['elapsed_seconds'])
if __name__=='__main__':main()
