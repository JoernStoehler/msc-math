#!/usr/bin/env python3
"""Independent rational verification. Does not import selector or consume labels.
Source primal vertex projections supply opposite support geometry; Gaussian
elimination supplies closure weights; direct 4D omega supplies HK objective.
Only the frozen width-contact support pairs have their cyclic orders enumerated.
"""
import argparse, hashlib, json, time
from fractions import Fraction as F
from itertools import combinations, permutations
from pathlib import Path

DEFAULT_SOURCE=Path('/home/joern/.cache/msc-math/artifacts/polytope-datasets/f7bc6be841e9d30741d5bf7ec8d4f0c0c74ec22745d3325b792e64cec333ca96/files/random-product.jsonl')
def dot(x,y): return sum((a*b for a,b in zip(x,y)),F(0))
def det(x,y): return x[0]*y[1]-x[1]*y[0]
def omega(u,v): return u[0]*v[2]-u[2]*v[0]+u[1]*v[3]-u[3]*v[1]
def solve(mat,rhs):
    """Unique solution of possibly overdetermined equations, or None."""
    aug=[list(row)+[v] for row,v in zip(mat,rhs)]; n=len(mat[0]); rank=0; piv=[]
    for col in range(n):
        chosen=next((r for r in range(rank,len(aug)) if aug[r][col]),None)
        if chosen is None: continue
        aug[rank],aug[chosen]=aug[chosen],aug[rank]
        div=aug[rank][col]; aug[rank]=[z/div for z in aug[rank]]
        for r in range(len(aug)):
            if r!=rank:
                factor=aug[r][col]
                aug[r]=[z-factor*w for z,w in zip(aug[r],aug[rank])]
        piv.append(col);rank+=1
    if any(not any(row[:-1]) and row[-1] for row in aug) or rank!=n:return None
    out=[F(0)]*n
    for r,c in enumerate(piv):out[c]=aug[r][-1]
    return tuple(out)
def hull(points):
    out=[]
    for p in sorted(set(points)):
        while len(out)>1 and det(tuple(out[-1][d]-out[-2][d] for d in range(2)),tuple(p[d]-out[-1][d] for d in range(2)))<=0:out.pop()
        out.append(p)
    lower=out;out=[]
    for p in sorted(set(points),reverse=True):
        while len(out)>1 and det(tuple(out[-1][d]-out[-2][d] for d in range(2)),tuple(p[d]-out[-1][d] for d in range(2)))<=0:out.pop()
        out.append(p)
    return lower[:-1]+out[:-1]
def area(points):
    hh=hull(points);return abs(sum(det(hh[i],hh[(i+1)%len(hh)]) for i in range(len(hh))))/2

def geometry(row):
    k=row['k']; dual=[tuple(map(F,v)) for v in row['dual_vertices_rational']]
    primal=[tuple(map(F,v)) for v in row['vertices_rational']]
    assert all(all(z==0 for z in v[2:]) for v in dual[:k])
    assert all(all(z==0 for z in v[:2]) for v in dual[k:])
    a=[v[:2] for v in dual[:k]];b=[v[2:] for v in dual[k:]]
    pv=sorted(set(v[:2] for v in primal));qv=sorted(set(v[2:] for v in primal))
    assert set(primal)=={p+q for p in pv for q in qv}
    for normals,verts in [(a,pv),(b,qv)]:
        assert all(dot(u,v)<=1 for u in normals for v in verts)
        independently_enumerated=set()
        for i,j in combinations(range(len(normals)),2):
            v=solve([normals[i],normals[j]],[F(1),F(1)])
            if v is not None and all(dot(u,v)<=1 for u in normals):independently_enumerated.add(v)
        assert independently_enumerated==set(verts)
    return a,b,pv,qv

def anchor_closures(normals,verts):
    out=[]
    for i,u in enumerate(normals):
        h=max(-dot(u,v) for v in verts); alpha=1/(1+h)
        opposite=[v for v in verts if -dot(u,v)==h]; choices={}
        # Independent small LP-vertex enumeration with generic linear solving.
        for size in (2,3):
            for indices in combinations(range(len(normals)),size):
                if i not in indices:continue
                weights=solve([[normals[z][c] for z in indices] for c in range(2)]+[[F(1)]*size],[F(0),F(0),F(1)])
                if weights is None or min(weights)<=0 or weights[indices.index(i)]!=alpha:continue
                # Every other support normal must meet a common opposite point.
                assert any(all(dot(normals[z],v)==1 for z in indices if z!=i) for v in opposite)
                assert sum(weights)==1 and all(sum(w*normals[z][c] for z,w in zip(indices,weights))==0 for c in range(2))
                choices[indices]=weights
        assert choices
        out.append((alpha,choices))
    return out

def selected_supports(a,b,pv,qv):
    ac=anchor_closures(a,pv);bc=anchor_closures(b,qv)
    scores={(i,j):abs(ac[i][0]*bc[j][0]*dot(u,v)) for i,u in enumerate(a) for j,v in enumerate(b)}
    maximum=max(scores.values()); contacts=sorted(ij for ij,v in scores.items() if v==maximum)
    pairs={}
    for i,j in contacts:
        for si,wi in ac[i][1].items():
            for sj,wj in bc[j][1].items():pairs[(si,sj)]=(wi,wj)
    return maximum,contacts,pairs,[sum(len(z[1]) for z in ac),sum(len(z[1]) for z in bc)]

def direct(a,b,si,sj,weights,word):
    vv=[a[z]+(F(0),F(0)) for z in si]+[(F(0),F(0))+b[z] for z in sj]
    assert min(weights)>0 and sum(weights)==1
    assert all(sum(w*v[c] for w,v in zip(weights,vv))==0 for c in range(4))
    assert sorted(word)==list(range(len(vv)))
    return sum(weights[word[i]]*weights[word[j]]*omega(vv[word[i]],vv[word[j]]) for i in range(len(word)) for j in range(i+1,len(word)))

def enumerate_selected(a,b,pairs):
    maximum=F(0); count=0
    for (si,sj),(wi,wj) in pairs.items():
        weights=tuple(w/2 for w in wi+wj);n=len(weights)
        # Omega matrix built directly in four coordinates; no product sign formula.
        vv=[a[z]+(F(0),F(0)) for z in si]+[(F(0),F(0))+b[z] for z in sj]
        weighted=[[weights[i]*weights[j]*omega(vv[i],vv[j]) for j in range(n)] for i in range(n)]
        for tail in permutations(range(1,n)):
            word=(0,)+tail
            value=sum(weighted[word[i]][word[j]] for i in range(n) for j in range(i+1,n));count+=1
            if value>maximum:maximum=value
    assert maximum>0
    return maximum,count

def transformed(a,b,pv,qv,kind):
    if kind=='paired_GL':
        # Primal q -> A q, p -> A^-T p, A=[[2,1],[1,1]].
        f=lambda v:(2*v[0]+v[1],v[0]+v[1]);g=lambda v:(v[0]-v[1],-v[0]+2*v[1])
        return [g(v) for v in a],[f(v) for v in b],[f(v) for v in pv],[g(v) for v in qv]
    # Strictly interior rational translations avoid invalid polar representations.
    t=(F(1,100),F(-1,137));s=(F(-1,113),F(1,127))
    norm=lambda u,z:tuple(x/(1+dot(u,z)) for x in u)
    assert all(1+dot(u,t)>0 for u in a) and all(1+dot(u,s)>0 for u in b)
    return [norm(u,t) for u in a],[norm(u,s) for u in b],[tuple(v[c]+t[c] for c in range(2)) for v in pv],[tuple(v[c]+s[c] for c in range(2)) for v in qv]

def main():
    ap=argparse.ArgumentParser();ap.add_argument('--source',type=Path,default=DEFAULT_SOURCE);ap.add_argument('--directory',type=Path,default=Path(__file__).parent);args=ap.parse_args();start=time.perf_counter()
    results_path=args.directory/'results.json';falsifier_path=args.directory/'falsifier.json'
    report=json.loads(results_path.read_text());falsifier=json.loads(falsifier_path.read_text())
    stress_path=args.directory/'stress-results.json';stress=json.loads(stress_path.read_text())
    assert stress['source_sha256']==report['source_sha256']
    assert [r['name'] for r in stress['rows']]==['random_5x6_s2_220','random_6x6_s1_194']
    all_rows=report['rows']+stress['rows']
    sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
    assert sha(args.source)==report['source_sha256']==falsifier['source_sha256']
    assert sha(results_path)==falsifier['selector_results_sha256']
    names={r['name'] for r in all_rows};source={}
    for line in args.source.open():
        r=json.loads(line)
        if r['name'] in names:source[r['name']]=r
    assert len(report['rows'])==10 and len(source)==len(names)==12
    checks=[];geometries={}
    for row in all_rows:
        name=row['name'];geom=geometry(source[name]);geometries[name]=geom;a,b,pv,qv=geom;s=row['selection']
        mx,contacts,pairs,counts=selected_supports(*geom)
        assert contacts==[tuple(x) for x in s['anchor_contacts']]
        assert 1/mx==F(s['width_capacity']) and len(pairs)==s['support_pair_count'] and counts==s['anchor_closure_counts']
        best=s['best'];support=tuple(tuple(z) for z in best['supports']);assert support in pairs
        wi,wj=pairs[support];assert tuple(map(F,best['weights']))==tuple(w/2 for w in wi+wj)
        q=direct(a,b,*support,tuple(map(F,best['weights'])),best['word']);assert q==F(best['Q'])
        max_q,word_count=enumerate_selected(a,b,pairs)
        assert q==max_q and word_count==s['word_evaluations']
        capacity=1/(2*max_q);assert capacity==F(best['capacity'])
        vol=area(pv)*area(qv);assert vol==F(row['exact_original_area_product'])
        ratio=capacity*capacity/(2*vol);assert ratio==F(row['exact_ratio_upper_bound'])
        invariance={}
        for kind in ('paired_GL','translation'):
            tg=transformed(*geom,kind);tm,tc,tp,tcounts=selected_supports(*tg)
            assert (tm,tc,set(tp),tcounts)==(mx,contacts,set(pairs),counts)
            tq,tn=enumerate_selected(tg[0],tg[1],tp);assert tq==max_q and tn==word_count
            invariance[kind]='PASS'
        checks.append(dict(name=name,primal_projection_and_halfplane_agreement='PASS',all_anchor_positive_closures_and_maximality='PASS',selected_contact_and_support_completeness='PASS',direct_4D_objective_and_all_selected_orders='PASS',exact_area_and_ratio='PASS',selected_support_pairs=len(pairs),cyclic_orders=word_count,invariance=invariance))
    name=falsifier['name'];a,b,_,_=geometries[name];w=falsifier['feasible_challenger'];si,sj=map(tuple,w['supports']);weights=tuple(map(F,w['weights']))
    assert sum(weights[:len(si)])==sum(weights[len(si):])==F(1,2)
    q=direct(a,b,si,sj,weights,w['word']);assert q==F(w['Q']) and q>0
    capacity=1/(2*q);assert capacity==F(w['capacity'])
    selector=F(falsifier['selector_capacity']);assert selector==F(next(r for r in report['rows'] if r['name']==name)['selection']['best']['capacity'])
    gap=selector/capacity;assert gap==F(falsifier['exact_lower_bound_selector_over_true_capacity']) and gap>=F('1.09904637')
    # A feasible point of the HK maximization yields a capacity upper bound;
    # this verifies the falsifier without any historical capacity/sys labels.
    output=dict(status='PASS',method='Independent stdlib fractions; original primal projections + generic Gaussian elimination + direct 4D symplectic objective; no analyze import, no capacity/sys labels accessed',source_sha256=sha(args.source),results_sha256=sha(results_path),falsifier_sha256=sha(falsifier_path),verify_sha256=sha(Path(__file__)),stress_results_sha256=sha(stress_path),controls=checks,falsifier=dict(name=name,positive_normalized_4D_closure='PASS',direct_HK_Q=str(q),witness_capacity_upper_bound=str(capacity),selector_over_witness=str(gap),selector_over_true_capacity_lower_bound=float(gap),claim='Ctrue <= Cwitness, hence Cselector/Ctrue >= Cselector/Cwitness; no equality claim for Ctrue'),elapsed_seconds=time.perf_counter()-start)
    extreme=next(r for r in stress['rows'] if r['name']=='random_6x6_s1_194');r=F(extreme['exact_ratio_upper_bound']);z=5*r-3
    assert z>0 and z*z>5
    output['HKO_envelope_obstruction']=dict(name=extreme['name'],exact_selected_ratio=str(r),ratio=float(r),comparison='r > (3+sqrt(5))/5',rational_sign_checks={'5r-3_positive':True,'(5r-3)^2_greater_than_5':True},claim='Frozen selector upper envelope exceeds HKO; no claim true ratio exceeds HKO')
    (args.directory/'verification.json').write_text(json.dumps(output,indent=2)+'\n');print(json.dumps({k:output[k] for k in ('status','method','falsifier','elapsed_seconds')},indent=2))
if __name__=='__main__':main()
