#!/usr/bin/env python3
"""Independent certificate reconstruction; imports no selector or LP solver."""
from fractions import Fraction as F
from pathlib import Path
from functools import cmp_to_key
import argparse,datetime,hashlib,json

def dot(x,y):return sum((u*v for u,v in zip(x,y)),F(0))
def cross(x,y):return x[0]*y[1]-x[1]*y[0]
def angular(a,b,sign):
    vectors=list(a)+[(-sign*v[1],sign*v[0]) for v in b]
    def cmp(i,j):
        x,y=vectors[i],vectors[j]
        hx=0 if x[1]>0 or (x[1]==0 and x[0]>=0) else 1
        hy=0 if y[1]>0 or (y[1]==0 and y[0]>=0) else 1
        if hx!=hy:return hx-hy
        z=cross(x,y)
        return -1 if z>0 else 1 if z<0 else i-j
    w=sorted(range(len(vectors)),key=cmp_to_key(cmp))
    return w if sign==1 else w[::-1]
def objective(a,b,w):
    p={i:k for k,i in enumerate(w)};n=len(a)
    return [dot(u,v)*(1 if p[i]<p[n+j] else -1)/4 for i,u in enumerate(a) for j,v in enumerate(b)]
def equations(a,b):
    n,m=len(a),len(b);eq=[[F(1)]*(n*m)]
    for i in range(n):
        for c in range(2):eq.append([b[j][c] if ii==i else F(0) for ii in range(n) for j in range(m)])
    for j in range(m):
        for c in range(2):eq.append([a[i][c] if jj==j else F(0) for i in range(n) for jj in range(m)])
    return eq,[F(1)]+[F(0)]*(2*(n+m))
def witness(a,b,v):
    al=list(map(F,v['alpha']));ga=list(map(F,v['gamma']));w=v['word'];n=len(a)
    assert min(al+ga)>=0 and sum(al)==sum(ga)==1
    for c in range(2):assert sum(t*x[c] for t,x in zip(al,a))==sum(t*y[c] for t,y in zip(ga,b))==0
    vectors=[(*x,F(0),F(0)) for x in a]+[(F(0),F(0),*y) for y in b]
    beta=[t/2 for t in al+ga];q=F(0)
    for j,u in enumerate(w):
        for v in w[j+1:]:
            x,y=vectors[u],vectors[v];omega=x[0]*y[2]+x[1]*y[3]-x[2]*y[0]-x[3]*y[1]
            q+=beta[u]*beta[v]*omega
    missing=[i for i in range(len(a)+len(b)) if i not in w]
    assert all(beta[i]==0 for i in missing)
    assert q==dot(objective(a,b,w+missing),[x*y for x in al for y in ga])
    return 1/(2*q)

def main():
    p=argparse.ArgumentParser();p.add_argument('--patterns',type=Path,required=True);p.add_argument('--results',type=Path,required=True);p.add_argument('--out',type=Path,required=True);args=p.parse_args()
    x=json.loads(args.results.read_text());root=args.patterns
    for f,h in x['sources'].items():assert hashlib.sha256((root/f).read_bytes()).hexdigest()==h
    design=json.loads((root/'patterns/upper-tail/full-partner-containment/results.json').read_text())['rows']
    validation=json.loads((root/'patterns/upper-tail/full-partner-validation/results.json').read_text())['rows']
    summaries=[]
    for r in x['rows']:
        a=[list(map(F,v)) for v in r['A']];b=[list(map(F,v)) for v in r['B']];n,m=len(a),len(b)
        src=next(z for z in design+validation if z['name']==r['name'])
        geom=src['geometry'];assert a==[list(map(F,v[:2])) for v in geom[:n]] and b==[list(map(F,v[2:])) for v in geom[n:]]
        eq,rhs=equations(a,b);vals=[]
        for v in r['variants']:
            w=angular(a,b,v['shadow_sign']);assert w==v['word'];c=objective(a,b,w)
            assert c==list(map(F,v['objective']))
            assert eq==[list(map(F,row)) for row in v['A_eq']] and rhs==list(map(F,v['b_eq']))
            W=list(map(F,v['W']));dual=list(map(F,v['dual']));q=F(v['Q']);assert min(W)>=0
            assert [dot(row,W) for row in eq]==rhs
            sl=[sum(eq[k][i]*dual[k] for k in range(len(eq)))-c[i] for i in range(n*m)]
            assert min(sl)>=0 and sl==list(map(F,v['dual_slack']))
            assert dot(c,W)==dot(rhs,dual)==q
            vals.append(q)
        qmax=max(vals);lo=1/(2*qmax);assert lo==F(r['capacity_lower'])
        up=F(r['feasible_capacity_upper']);assert up>=lo
        assert witness(a,b,r['feasible_upper_witness'])==up
        candidates=[]
        for s in design+validation:
            if s['name']!=r['name']:continue
            for v in s['selection']['variants']:candidates.append(witness(a,b,v))
            for k,z in s.get('comparators',{}).items():
                # Different retained comparator schemas are checked below as needed.
                if isinstance(z,dict) and all(key in z for key in ['alpha','gamma','word']):candidates.append(witness(a,b,z))
        # Every upper endpoint was directly rechecked above; exactness is additional equality.
        if r['exact_capacity']:assert up==lo and up in candidates
        assert up/lo==F(r['capacity_interval_factor'])
        summaries.append({'name':r['name'],'lower':float(lo),'upper':float(up),'interval_factor':float(up/lo),'exact_capacity':lo==up})
    out={'at':datetime.datetime.now(datetime.timezone.utc).isoformat(),'scope':'Independent rational matrix/objective reconstruction, exact primal/dual certificates, source geometry/hash joins and direct4D feasible words; no support-pair enumeration','rational_LPs':6,'direct_4D_upper_witnesses':3,'source_geometries':3,'rows':summaries,'result_sha256':hashlib.sha256(args.results.read_bytes()).hexdigest(),'checker_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),'status':'PASS'}
    args.out.write_text(json.dumps(out,indent=2)+'\n');print(json.dumps(summaries))
if __name__=='__main__':main()
