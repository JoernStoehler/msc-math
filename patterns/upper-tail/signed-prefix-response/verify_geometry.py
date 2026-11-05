#!/usr/bin/env python3
"""Root independent rational geometry and direct4D check; no selector imports."""
import argparse,hashlib,itertools,json,math
from fractions import Fraction as F
from functools import cmp_to_key
from pathlib import Path
def dot(x, y):
    return sum(a*b for a,b in zip(x,y))

def cross(x,y):
    return x[0]*y[1]-x[1]*y[0]

def hull(points):
    points=sorted(set(points))
    def half(seq):
        out=[]
        for p in seq:
            while len(out)>1 and cross(tuple(out[-1][i]-out[-2][i] for i in range(2)),tuple(p[i]-out[-1][i] for i in range(2)))<=0:
                out.pop()
            out.append(p)
        return out
    return half(points)[:-1]+half(reversed(points))[:-1]

def geometry(normals):
    v=[]
    for a,b in itertools.combinations(normals,2):
        determinant=cross(a,b)
        if not determinant:
            continue
        point=((b[1]-a[1])/determinant,(a[0]-b[0])/determinant)
        if all(dot(n,point)<=1 for n in normals):
            v.append(point)
    v=hull(v)
    assert len(v)>=3
    area=sum(cross(a,b) for a,b in zip(v,v[1:]+v[:1]))/2
    assert area>0
    anchors=[]
    for n in normals:
        t=1/(1+max(-dot(n,p) for p in v))
        anchors.append((t,tuple(t*z for z in n)))
    return v,area,anchors

def omega(x,y):
    return x[0]*y[2]+x[1]*y[3]-x[2]*y[0]-x[3]*y[1]

def objective(normals,weights,word):
    return sum(weights[i]*weights[j]*omega(normals[i],normals[j]) for k,i in enumerate(word) for j in word[k+1:])


def edge_area(edges):
    def cmp(u,v):
        hu=0 if u[1]>0 or (u[1]==0 and u[0]>=0) else 1
        hv=0 if v[1]>0 or (v[1]==0 and v[0]>=0) else 1
        if hu!=hv:return hu-hv
        z=cross(u,v)
        return -1 if z>0 else 1 if z<0 else 0
    edges=sorted([e for e in edges if any(e)],key=cmp_to_key(cmp));point=(F(0),F(0));twice=F(0)
    for edge in edges:
        twice+=cross(point,edge);point=tuple(x+y for x,y in zip(point,edge))
    assert point==(0,0) and twice>=0
    return twice/2

def mixed_area(normals,k,alpha,gamma,sign):
    u=[tuple(F(a)*x for x in n[:2]) for a,n in zip(alpha,normals[:k])]
    v=[(-sign*F(b)*n[3],sign*F(b)*n[2]) for b,n in zip(gamma,normals[k:])]
    return (edge_area(u+v)-edge_area(u)-edge_area(v))/2

def evaluate(normals,k,alpha,gamma,word):
    weights=[F(v)/2 for v in alpha+gamma]
    assert min(weights)>=0 and sum(weights[:k])==sum(weights[k:])==F(1,2)
    assert all(sum(w*n[c] for w,n in zip(weights,normals))==0 for c in range(4))
    assert len(set(word))==len(word) and all(0<=i<len(normals) for i in word)
    assert all(i in word for i,w in enumerate(weights) if w)
    return objective(normals,weights,word)

def main():
    ap=argparse.ArgumentParser();ap.add_argument('--source',type=Path,required=True);ap.add_argument('--results',type=Path,required=True);ap.add_argument('--out',type=Path,required=True);ap.add_argument('--challenger',type=Path,required=True);args=ap.parse_args()
    results=json.loads(args.results.read_text());assert hashlib.sha256(args.source.read_bytes()).hexdigest()==results['source_sha256']
    rows={r['name']:r for r in results['rows']};names=set(rows);checked=[];witness_count=0
    for line in args.source.open():
        original=json.loads(line)
        if original['name'] not in rows:continue
        r=rows.pop(original['name']);n=[tuple(map(F,u)) for u in original['dual_vertices_rational']];k=original['k'];s=r['selection'];sa=s['seed_alpha'];sg=s['seed_gamma'];seed=s['seed']['best'];pi,qi=seed['supports']
        sw=[pi[i] if i<len(pi) else k+qi[i-len(pi)] for i in seed['word']]
        seedq=evaluate(n,k,sa,sg,sw);assert seedq==F(seed['Q'])>0;witness_count+=1
        qs=[seedq]
        for v in s['variants']:
            assert sorted(v['word'])==list(range(len(n)))
            start=evaluate(n,k,sa,sg,v['word']);mid=evaluate(n,k,v['alpha'],sg,v['word']);end=evaluate(n,k,v['alpha'],v['gamma'],v['word'])
            assert (start,mid,end)==tuple(F(v[z]) for z in ['seed_Q','after_A_Q','Q'])
            assert 0<=start<=mid<=end
            assert mixed_area(n,k,sa,sg,v['shadow_sign'])==start
            assert mixed_area(n,k,v['alpha'],sg,v['shadow_sign'])==mid
            assert mixed_area(n,k,v['alpha'],v['gamma'],v['shadow_sign'])==end
            if end:assert F(v['capacity'])==1/(2*end)
            qs.append(end);witness_count+=3
        w=s['witness'];q=evaluate(n,k,w['alpha'],w['gamma'],w['word']);assert q==F(w['Q'])==max(qs)>0
        cap=1/(2*q);assert cap==F(s['capacity'])==F(w['capacity']);witness_count+=1
        pa=geometry([v[:2] for v in n[:k]])[1];qa=geometry([v[2:] for v in n[k:]])[1];volume=pa*qa
        assert volume==F(r['exact_volume']);ratio=cap*cap/(2*volume);assert ratio==F(r['exact_ratio_upper_bound'])
        assert r['historical_capacity']==original['capacity'] and r['historical_ratio']==original['sys']
        assert abs(float(cap)/original['capacity']-r['capacity_over_historical'])<1e-14
        assert abs(float(ratio)-r['ratio_upper_bound'])<1e-14
        gain=1-float(cap/F(seed['capacity']));assert abs(gain-r['relative_gain_over_contact'])<1e-14
        checked.append({'name':r['name'],'principal_top3':r['principal_top3'],'exact_strict_gain':q>seedq,'capacity_gain_fraction':gain,'ratio_bound':float(ratio),'capacity_gap_vs_historical':r['capacity_over_historical']-1,'exact_bound_below_one':ratio<1})
    assert not rows and len(checked)==len(names)==20
    regular=[]
    for r in results['regular_controls']:
        t=r['theta'];area=5*math.sin(2*math.pi/5)/2;cap=(1+math.cos(math.pi/5))**2/math.cos(t)
        assert abs(r['known_capacity']/cap-1)<1e-12 and abs(r['selection']['capacity']/cap-1)<1e-10
        ratio=cap*cap/(2*area*area);assert abs(r['ratio']-ratio)<1e-10
        regular.append({'angle':t,'ratio':ratio,'scope':'Known regular formula; baseline inherited, not a new explanation.'})
    assert abs(regular[-1]['ratio']-(3+math.sqrt(5))/5)<1e-12
    f=json.loads(args.challenger.read_text());body=None
    for line in args.source.open():
        x=json.loads(line)
        if x['name']==f['name']:body=x;break
    assert body is not None
    alln=[tuple(map(F,v)) for v in body['dual_vertices_rational']];k=body['k'];w=f['feasible_challenger'];pi,qi=w['supports'];n=[alln[i] for i in pi]+[alln[k+j] for j in qi];ww=list(map(F,w['weights']))
    assert min(ww)>0 and sum(ww[:len(pi)])==sum(ww[len(pi):])==F(1,2)
    assert all(sum(u*v[c] for u,v in zip(ww,n))==0 for c in range(4))
    q=objective(n,ww,w['word']);assert q==F(w['Q'])>0
    challenger_capacity=1/(2*q);assert challenger_capacity==F(w['capacity'])
    r=next(r for r in results['rows'] if r['name']==f['name']);new_capacity=F(r['selection']['capacity']);ratio=new_capacity/challenger_capacity;assert ratio>F(1044,1000)
    a=[F(0)]*k;bb=[F(0)]*(len(alln)-k)
    for i,x in zip(pi,ww[:len(pi)]):a[i]=2*x
    for j,x in zip(qi,ww[len(pi):]):bb[j]=2*x
    challenger_mixed=[mixed_area(alln,k,a,bb,sign) for sign in [1,-1]]
    assert challenger_mixed[0]==q
    selection=r['selection'];selected_q=F(selection['witness']['Q'])
    assert challenger_mixed[0]>selected_q
    fixed=[all(F(x)==F(y) for x,y in zip(v['gamma'],selection['seed_gamma'])) for v in selection['variants']]
    assert all(fixed)
    stationarity={'both_variants_final_B_equals_seed_B':fixed,'challenger_plus_shadow_Q':str(challenger_mixed[0]),'challenger_minus_shadow_Q':str(challenger_mixed[1]),'challenger_plus_over_selected_Q':str(challenger_mixed[0]/selected_q),'interpretation':'With separately verified exact LP optima, both outputs are coordinatewise stationary; the explicit plus-shadow competitor beats both. Repeating the same strict-improvement sweeps from these points cannot repair this miss. No other initialization or global optimum claim.'}
    residual={'name':f['name'],'exact_new_bound_over_feasible_challenger':str(ratio),'guaranteed_capacity_gap_fraction':float(ratio)-1,'scope':'Existing feasible competitor retained outside predictor; no new support search, exact true capacity not required.'};witness_count+=1
    summary={'coordinate_trap':stationarity,'residual_obstruction':residual,'status':'PASS','scope':'Root independent rational all-intersection convex-hull areas and direct4D closure/objective at seed, shadow start, A update, B update, and selected witness; all exact rational. Does not establish true-capacity optimality or held-out generalization.','rational_bodies':20,'direct_4d_evaluations':witness_count,'exact_mixed_area_stage_identities':120,'strict_gains':sum(r['exact_strict_gain'] for r in checked),'principal_gains':sum(r['exact_strict_gain'] for r in checked if r['principal_top3']),'principal_count':sum(r['principal_top3'] for r in checked),'all20_ratio_bounds_below_one':all(r['exact_bound_below_one'] for r in checked),'rows':checked,'regular_controls':regular,'results_sha256':hashlib.sha256(args.results.read_bytes()).hexdigest()}
    args.out.write_text(json.dumps(summary,indent=2)+'\n');print(json.dumps({k:v for k,v in summary.items() if k not in ['scope','rows','regular_controls']}))
if __name__=='__main__':main()
