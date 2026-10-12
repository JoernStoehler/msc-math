#!/usr/bin/env python3
"""Derive exact ten-fit support formulas, residuals and sign workload.

Reads accepted joint denominator certificate/source as a library; never runs
the earlier producer. Writes only sibling formulas.json. Standard library.
"""
import importlib.util
from pathlib import Path
import json,hashlib,time
from itertools import combinations

ROOT=Path(__file__).resolve().parent
ENGINE=ROOT.parent/'joint-denominators'/'extract.py'
spec=importlib.util.spec_from_file_location('accepted_field_engine',ENGINE)
engine=importlib.util.module_from_spec(spec);spec.loader.exec_module(engine)
K=engine.K

def trim(p):
    p=list(p)
    while len(p)>1 and p[-1]==0:p.pop()
    return p
def add(p,q):return trim([(p[i] if i<len(p) else K(0))+(q[i] if i<len(q) else K(0)) for i in range(max(len(p),len(q)))])
def neg(p):return [-c for c in p]
def sub(p,q):return add(p,neg(q))
def mul(p,q):
    r=[K(0)]*(len(p)+len(q)-1)
    for i,a in enumerate(p):
        for j,b in enumerate(q):r[i+j]+=a*b
    return trim(r)
def scale(p,c):return trim([c*v for v in p])
def evaluate(p,x):
    r=K(0)
    for c in reversed(p):r=r*x+c
    return r
def encode(p):return [c.data() for c in p]
def decode(p):return [K(*v) for v in p]
def coeff_key(p):return tuple((c.a,c.b) for c in trim(p))
def inverse(matrix):
    n=len(matrix);a=[list(row)+[K(int(i==j)) for j in range(n)] for i,row in enumerate(matrix)]
    for k in range(n):
        pivot=next(i for i in range(k,n) if a[i][k]!=0);a[k],a[pivot]=a[pivot],a[k]
        d=a[k][k];a[k]=[v/d for v in a[k]]
        for i in range(n):
            if i!=k:
                d=a[i][k];a[i]=[v-d*w for v,w in zip(a[i],a[k])]
    return [row[n:] for row in a]

def geometry():
    z=K(0,1);a=(z-1)/4;b=(z+1)/4;w=1+b
    lam0=[K(1)/(2*w),K(1)/(2*w),K(0),b/w,K(0)]
    lams=[[lam0[(i-l)%5] for i in range(5)] for l in range(5)]
    edges=[]
    for i in range(5):
        row=[K(0)]*5;row[(i-1)%5]=K(1);row[(i+1)%5]=K(1);row[i]=-2*a;edges.append(row)
    matrices=[]
    for ids in combinations(range(5),3):
        inv=inverse([lams[i][:3] for i in ids])
        assert all(sum(lams[ids[i]][k]*inv[k][j] for k in range(3))==int(i==j) for i in range(3) for j in range(3))
        matrices.append((ids,inv))
    return z,a,lams,edges,matrices

def main():
    start=time.perf_counter();source=ROOT.parent/'joint-denominators'/'joint-denominators.json'
    packet=json.loads(source.read_text());z,a,lams,edges,matrices=geometry()
    vectors={}; open_domains=[]; closed_domains=[]
    for si,s in enumerate(packet['joint']['topology_strips']):
        for ti,t in enumerate(s['substrips']):
            for ci,c in enumerate(t['cells']):
                ds=tuple(tuple(K(*v) for v in packet['circuits'][l]['candidates'][ids[0]]['coefficients']) for l,ids in enumerate(c['formula_candidates']))
                if ds not in vectors:vectors[ds]=len(vectors)
                open_domains.append(dict(vector=vectors[ds],incidence=[si,ti,ci]))
    allvectors=list(vectors)
    for fi,f in enumerate(packet['joint']['closed_fibers']):
        for ci,c in enumerate(f['cells']):
            matches=[]
            for ds in allvectors:
                if all(any(tuple(K(*v) for v in packet['circuits'][l]['candidates'][i]['coefficients'])==ds[l] for i in val['candidate_ids']) for l,val in enumerate(c['vector'])):matches.append(vectors[ds])
            assert matches
            closed_domains.append(dict(vectors=matches,incidence=[fi,ci]))
    polynomial_map={};polyrecords=[]
    def register(p,kind,owner):
        p=trim(p)
        if all(c==0 for c in p):return dict(zero=True)
        leading=p[-1];monic=scale(p,K(1)/leading);key=coeff_key(monic)
        if key not in polynomial_map:
            polynomial_map[key]=len(polyrecords);polyrecords.append(dict(coefficients=encode(monic),degree=len(monic)-1,occurrences=[]))
        idx=polynomial_map[key];polyrecords[idx]['occurrences'].append(dict(kind=kind,owner=owner,multiplier=leading.data()))
        return dict(polynomial=idx,multiplier=leading.data())
    formulas=[];dedup={}
    for vi,ds in enumerate(allvectors):
        # Bilinear supports use [constant,x,u,xu], or [intercept(u),slope(u)].
        dpolys=[([v[0],v[2]],[v[1],v[3]]) for v in ds]
        for ids,inv in matrices:
            h=[[[sum(inv[j][k]*dpolys[ids[k]][xp][up] for k in range(3)) for up in range(2)] for xp in range(2)] for j in range(3)]+[[[K(0)]*2,[K(0)]*2] for _ in range(2)]
            key=tuple(coeff_key(p) for coordinate in h for p in coordinate)
            dedup.setdefault(key,[]).append([vi,list(ids)])
            residuals=[]
            for rowi,row in enumerate(lams+edges):
                r=[trim([sum(row[j]*h[j][xp][up] for j in range(5)) for up in range(2)]) for xp in range(2)]
                if rowi<5:r=[sub(r[xp],dpolys[rowi][xp]) for xp in range(2)]
                residuals.append(r)
            P=[[K(0)] for _ in range(3)]
            for i in range(5):
                for j in range(2):
                    for k in range(2):P[j+k]=add(P[j+k],sub(mul(h[i][j],h[(i+1)%5][k]),scale(mul(h[i][j],h[i][k]),a)))
            q=list(P);q[1]=sub(q[1],[K(1)/(3+z),K(0),(5-2*z)/(3+z)])
            owner=len(formulas);bounds=[]
            for ri,(r,s) in enumerate(residuals):
                rpred=register(r,'residual_intercept',[owner,ri]);spred=register(s,'residual_slope',[owner,ri])
                if all(v==0 for v in s):continue
                gap_at_bound=add(sub(mul(q[0],mul(s,s)),mul(q[1],mul(r,s))),mul(q[2],mul(r,r)))
                membership=sub(scale(mul(q[2],r),K(2)),mul(q[1],s))
                bounds.append(dict(residual=ri,numerator=encode(neg(r)),denominator=encode(s),
                    endpoint_gap=register(gap_at_bound,'endpoint_gap',[owner,ri]),
                    stationary_membership=register(membership,'stationary_membership',[owner,ri])))
            stationary=sub(scale(mul(q[0],q[2]),K(4)),mul(q[1],q[1]))
            formulas.append(dict(vector=vi,fit_triple=list(ids),supports=[[encode(p) for p in coordinate] for coordinate in h],
                residuals=[[encode(p) for p in residual] for residual in residuals],
                area=[encode(p) for p in P],gap=[encode(p) for p in q],bounds=bounds,
                stationary_gap=register(stationary,'stationary_gap',owner),
                q0=register(q[0],'q0',owner),q1=register(q[1],'q1',owner),q2=register(q[2],'q2',owner)))
    out=dict(status='Exact formula/workload derivation; no positivity claim',
        input=str(source),input_sha256=hashlib.sha256(source.read_bytes()).hexdigest(),
        field_engine=str(ENGINE),field_engine_sha256=hashlib.sha256(ENGINE.read_bytes()).hexdigest(),
        encoding='[a,b]=a+b sqrt5; polynomials ascending u powers; supports [intercept(u),slope(u)]',
        quotient='h3=h4=0',gap='P-x*(1+(5-2sqrt5)*u²)/(3+sqrt5)',
        vector_count=len(vectors),vectors=[[[v.data() for v in coeff] for coeff in ds] for ds in allvectors],
        fit_matrices=[dict(rows=list(ids),inverse=[[v.data() for v in row] for row in inv]) for ids,inv in matrices],
        open_domains=open_domains,closed_domains=closed_domains,formulas=formulas,
        distinct_support_formula_count=len(dedup),support_formula_classes=[dict(owners=owners) for owners in dedup.values()],
        polynomials=polyrecords,degree_counts={str(d):sum(p['degree']==d for p in polyrecords) for d in range(5)},
        elapsed_seconds=round(time.perf_counter()-start,3))
    (ROOT/'formulas.json').write_text(json.dumps(out,indent=2)+'\n')
    print(json.dumps({k:out[k] for k in ['vector_count','distinct_support_formula_count','degree_counts','elapsed_seconds']}))
    print('formula_count',len(formulas),'open_domains',len(open_domains),'closed_domains',len(closed_domains),'unique_polys',len(polyrecords))

if __name__=='__main__':main()
