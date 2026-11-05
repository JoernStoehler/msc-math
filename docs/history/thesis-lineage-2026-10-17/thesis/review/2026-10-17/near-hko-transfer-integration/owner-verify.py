#!/usr/bin/env python3
"""Independent rational halfplanes, Gaussian closures, prefix-capacity oracle and LP/rounding audit."""
import argparse, hashlib, itertools, json
from fractions import Fraction as F
from pathlib import Path

def digest(p): return hashlib.sha256(p.read_bytes()).hexdigest()
def dot(a,b): return sum((x*y for x,y in zip(a,b)),F(0))
def cross(a,b): return a[0]*b[1]-a[1]*b[0]
def solve(rows,b,n):
    a=[list(r)+[v] for r,v in zip(rows,b)];piv=[];i=0
    for j in range(n):
        k=next((k for k in range(i,len(a)) if a[k][j]),None)
        if k is None:continue
        a[i],a[k]=a[k],a[i];q=a[i][j];a[i]=[x/q for x in a[i]]
        for k in range(len(a)):
            if k!=i:
                q=a[k][j];a[k]=[x-q*y for x,y in zip(a[k],a[i])]
        piv.append(j);i+=1
    if any(not any(r[:n]) and r[n] for r in a) or len(piv)!=n:return None
    out=[F(0)]*n
    for i,j in enumerate(piv):out[j]=a[i][n]
    return out

def closures(a):
    allv=[]
    for k in range(1,4):
        for s in itertools.combinations(range(len(a)),k):
            v=solve([[F(1)]*k]+[[a[i][j] for i in s] for j in range(2)],[F(1),F(0),F(0)],k)
            if v is None or min(v)<=0:continue
            x=[F(0)]*len(a)
            for i,t in zip(s,v):x[i]=t
            allv.append(x)
    assert allv
    return allv

def area(a):
    points=set()
    for u,v in itertools.combinations(a,2):
        d=cross(u,v)
        if not d:continue
        p=((v[1]-u[1])/d,(u[0]-v[0])/d)
        if all(dot(w,p)<=1 for w in a):points.add(p)
    points=sorted(points);assert len(points)==len(a)==5
    def hull(seq):
        out=[]
        for p in seq:
            while len(out)>=2 and cross((out[-1][0]-out[-2][0],out[-1][1]-out[-2][1]),(p[0]-out[-1][0],p[1]-out[-1][1]))<=0:out.pop()
            out.append(p)
        return out
    h=hull(points)[:-1]+hull(points[::-1])[:-1]
    assert len(h)==5
    for u in a:assert sum(dot(u,p)==1 for p in h)==2
    return sum(cross(h[i],h[(i+1)%len(h)]) for i in range(len(h)))/2

def feasible(a,v):
    assert min(v)>=0 and sum(v)==1
    assert all(sum(v[i]*a[i][k] for i in range(len(a)))==0 for k in range(2))

def direct(a,b,w,x,y):
    verts=[(u[0],u[1],F(0),F(0)) for u in a]+[(F(0),F(0),v[0],v[1]) for v in b]
    beta=[t/2 for t in x+y]
    return sum((beta[i]*beta[j]*(dot(verts[i][:2],verts[j][2:])-dot(verts[i][2:],verts[j][:2])) for p,i in enumerate(w) for j in w[p+1:]),F(0))

def prefix_value(a,b,x,y):
    # Independent of angular words: all cyclic orders of the <=3 active A labels,
    # then each B step chooses its best A-prefix independently (closure identity).
    steps=[(x[i]*u[0],x[i]*u[1]) for i,u in enumerate(a) if x[i]]
    bs=[(y[j]*v[0],y[j]*v[1]) for j,v in enumerate(b) if y[j]]
    first=steps[0];best=F(0)
    for tail in itertools.permutations(steps[1:]):
        pref=[(F(0),F(0))];total=[F(0),F(0)]
        for u in (first,)+tail:
            total=[total[k]+u[k] for k in range(2)];pref.append(tuple(total))
        assert total==[0,0]
        value=sum(max(dot(p,v) for p in pref[:-1]) for v in bs)/2
        best=max(best,value)
    return best

def audit_lp(a,b,v):
    n,m=len(a),len(b);W=list(map(F,v['W']));dual=list(map(F,v['dual']));q=F(v['Q']);w=v['word'];pos={j:i for i,j in enumerate(w)}
    assert sorted(w)==list(range(n+m)) and len(W)==n*m and len(dual)==1+2*n+2*m
    assert min(W)>=0 and sum(W)==1
    for i in range(n):
        for k in range(2):assert sum(W[i*m+j]*b[j][k] for j in range(m))==0
    for j in range(m):
        for k in range(2):assert sum(W[i*m+j]*a[i][k] for i in range(n))==0
    objective=F(0)
    for i in range(n):
        for j in range(m):
            c=dot(a[i],b[j])*(1 if pos[i]<pos[n+j] else -1)/4
            slack=dual[0]+sum(dual[1+2*i+k]*b[j][k] for k in range(2))+sum(dual[1+2*n+2*j+k]*a[i][k] for k in range(2))-c
            assert slack>=0 and slack==F(v['dual_slack'][i*m+j])
            objective+=W[i*m+j]*c
    assert objective==q==dual[0]

def audit_round(a,b,variants,r,ca,cb):
    n,m=len(a),len(b);expected=[]
    for v in variants:
        W=list(map(F,v['W']));word=v['word']
        for kind in ['row','column']:
            for k in range(n if kind=='row' else m):
                weights=[W[k*m+j] for j in range(m)] if kind=='row' else [W[i*m+k] for i in range(n)]
                mass=sum(weights)
                if not mass:continue
                fixed=[t/mass for t in weights]
                if kind=='row':
                    feasible(b,fixed);x=max(ca,key=lambda x:direct(a,b,word,x,fixed));y=fixed
                else:
                    feasible(a,fixed);y=max(cb,key=lambda y:direct(a,b,word,fixed,y));x=fixed
                expected.append((v['shadow_sign'],kind,k,x,y,direct(a,b,word,x,y),mass))
    assert len(expected)==r['candidate_count']==len(r['candidates'])
    for e,c in zip(expected,r['candidates']):
        sign,kind,k,x,y,q,mass=e
        assert (sign,kind,k)==(c['shadow_sign'],c['kind'],c['index'])
        assert x==list(map(F,c['alpha'])) and y==list(map(F,c['gamma']))
        assert q==F(c['Q']) and mass==F(c['condition_mass'])
    assert F(r['best']['Q'])==max(e[5] for e in expected)
    return len(expected)

def main():
    p=argparse.ArgumentParser();p.add_argument('--directory',type=Path,required=True);p.add_argument('--repo',type=Path,required=True);p.add_argument('--out',type=Path,required=True);args=p.parse_args();d=args.directory
    sample=json.loads((d/'sample.json').read_text());pred=json.loads((d/'predictions.json').read_text());out=json.loads((d/'results.json').read_text());h=json.loads((args.repo/'patterns/upper-tail/conditional-coupling/hko-exact.json').read_text())
    assert digest(d/'predictions.json')=='466172b4064e7a23c8325e1fc8f54af6227103c45ed7692003458b0c69566702'
    assert digest(d/'PLAN.md')=='937925eefdab98bfe5bc70cb6addc0800612248a4126e6eeb00c265ed9d7b227'
    manifest=json.loads((d/'frozen-manifest.json').read_text())
    for name,hsh in manifest['sources'].items():assert digest(args.repo/name)==hsh,name
    for name,hsh in manifest['freeze'].items():assert digest(d/name)==hsh,name
    assert [r['name'] for r in pred['rows']]==[r['name'] for r in sample['rows']]
    assert [r['name'] for r in out['rows']]==[r['name'] for r in sample['rows'][:len(out['rows'])]]
    assert digest(d/'sample.json')=='83f89a57fd12bf6a072c8d551edfdbba83e26f30e101521d6708c075f3b2d3a0'
    base=[[F(c['rational'])+F(c['sqrt5'])*F('559017/250000') for c in u] for u in h['A']+h['B']]
    assert len(sample['rows'])==12
    for i,s in enumerate(sample['rows']):
        seed=i//2;eps=F(1,[1000,100][i%2]);assert s['seed']==seed and F(s['epsilon'])==eps
        sha=hashlib.sha256(('msc-math/thesis/near-hko-validation/v1:'+str(seed)).encode()).digest()
        expected=[[base[j][k]+eps*F(sha[2*j+k]%9-4,4) for k in range(2)] for j in range(10)]
        assert expected==[list(map(F,u)) for u in s['A']+s['B']]
    results=[];totalresponses=0;paircount=0
    for r in out['rows']:
        s=next(s for s in sample['rows'] if s['name']==r['name']);pr=next(t for t in pred['rows'] if t['name']==r['name'])
        for k in ['A','B','variants','rounding']:assert r[k]==pr[k]
        assert r['A']==s['A'] and r['B']==s['B']
        a=[list(map(F,u)) for u in r['A']];b=[list(map(F,u)) for u in r['B']];ca,cb=closures(a),closures(b)
        for x in ca:feasible(a,x)
        for y in cb:feasible(b,y)
        vol=area(a)*area(b)
        for v in r['variants']:audit_lp(a,b,v)
        totalresponses+=audit_round(a,b,r['variants'],r['rounding'],ca,cb)
        values=[prefix_value(a,b,x,y) for x in ca for y in cb];truth=max(values);paircount+=len(values)
        assert truth==F(r['truth']['Q'])
        t=r['truth'];x=list(map(F,t['alpha']));y=list(map(F,t['gamma']));feasible(a,x);feasible(b,y);assert direct(a,b,t['word'],x,y)==truth
        cap=1/(2*truth);ratio=cap*cap/(2*vol);rounded=F(r['rounding']['best']['Q'])
        assert F(t['capacity'])==cap and F(r['volume'])==vol and F(r['ratio'])==ratio
        assert r['exact_recovery']==(rounded==truth)
        results.append(dict(name=r['name'],closures=[len(ca),len(cb)],capacity=str(cap),ratio=str(ratio),ratio_float=float(ratio),capacity_over_true=str(truth/rounded),exact_recovery=rounded==truth))
    receipt=dict(status='PASS',sample_size=12,scored=len(results),unscored=12-len(results),source_sample_sha256=digest(d/'sample.json'),input_hashes={n:digest(d/n) for n in ['predictions.json','results.json','PLAN.md']},root_checker_sha256=digest(Path(__file__)),exact_support_pairs=paircount,conditional_responses=totalresponses,LP_certificates=2*len(results),above_one=sum(F(x['ratio'])>1 for x in results),exact_recoveries=sum(x['exact_recovery'] for x in results),rows=results,oracle='All Gaussian closure-vertex pairs; both cyclic A orientations with independent best-prefix B insertions; uses sparse-product reduction and prefix identity, no angular sorting oracle import.')
    args.out.write_text(json.dumps(receipt,indent=2)+'\n');print(json.dumps({k:v for k,v in receipt.items() if k not in ['rows','input_hashes']},indent=2))
if __name__=='__main__':main()
