#!/usr/bin/env python3
"""Frozen full-partner containment support selector. No support-pair search."""
import argparse, hashlib, importlib.util, json, math
from fractions import Fraction as F
from itertools import combinations
from pathlib import Path

ROOT=Path(__file__).resolve().parents[3]
sp=importlib.util.spec_from_file_location('response',ROOT/'patterns/upper-tail/signed-prefix-response/analyze.py')
m=importlib.util.module_from_spec(sp);sp.loader.exec_module(m)
dot=m.dot;cross=m.cross

def solve(matrix,values,tol=0):
    a=[list(r)+[v] for r,v in zip(matrix,values)];n=len(values)
    for j in range(n):
        pivot=next((i for i in range(j,n) if abs(a[i][j])>tol),None)
        if pivot is None:return None
        a[j],a[pivot]=a[pivot],a[j];s=a[j][j];a[j]=[x/s for x in a[j]]
        for i in range(n):
            if i!=j:
                s=a[i][j];a[i]=[x-s*y for x,y in zip(a[i],a[j])]
    return [r[-1] for r in a]

def canonical(a,tol=0):
    order=m.shadow_word(a,[],1,tol);vs=[]
    for k,i in enumerate(order):
        u=a[i];v=a[order[(k+1)%len(order)]];d=cross(u,v)
        assert abs(d)>tol
        vs.append(((v[1]-u[1])/d,(u[0]-v[0])/d))
    area=sum(cross(vs[i],vs[(i+1)%len(vs)]) for i in range(len(vs)))/2
    assert area>0 and all(dot(n,v)<=1+tol*100 for n in a for v in vs)
    weights=[0]*len(a)
    for k,i in enumerate(order):
        edge=tuple(vs[k][c]-vs[k-1][c] for c in range(2));normal=a[i];jn=(-normal[1],normal[0])
        weights[i]=dot(edge,jn)/(dot(normal,normal)*2*area)
    assert min(weights)>0 and abs(sum(weights)-1)<=tol*100
    assert all(abs(sum(w*v[c] for w,v in zip(weights,a)))<=tol*100 for c in range(2))
    return weights,vs,area

def first_best(vertices,co,tol=0):
    maximum=max(dot(v,co) for v in vertices)
    return next(v for v in vertices if dot(v,co)>=maximum-tol)

def dual_certificate(a,co,alpha,tol=0):
    maximum=dot(alpha,co)
    for ids in combinations(range(len(a)),3):
        z=solve([[1,*a[i]] for i in ids],[co[i] for i in ids],tol)
        if z is None:continue
        t,x,y=z;slack=[t+x*v[0]+y*v[1]-c for v,c in zip(a,co)]
        if min(slack)>=-tol*100 and abs(t-maximum)<=tol*100:
            assert all(abs(alpha[i]*slack[i])<=tol*100 for i in range(len(a)))
            return dict(t=t,z=[x,y],slack=slack,active=ids,primal=maximum)
    raise AssertionError('no containment LP dual certificate')

def select(a,b,tol=0):
    ac,av,area_a=canonical(a,tol);bc,bv,area_b=canonical(b,tol)
    ca=m.closures(a,tol);cb=m.closures(b,tol);variants=[]
    for sign in [1,-1]:
        word=m.shadow_word(a,b,sign,tol)
        if sign<0:word=word[::-1]
        for first in ['A','B']:
            alpha=ac[:];gamma=bc[:]
            co=m.coefficients(a,b,word,gamma if first=='A' else alpha,first)
            if first=='A':alpha=first_best(ca,co,tol)
            else:gamma=first_best(cb,co,tol)
            first_q=m.Q(a,b,word,alpha,gamma)
            dual=dual_certificate(a if first=='A' else b,co,alpha if first=='A' else gamma,tol)
            # Independently calculate the geometric containment coefficients.
            normals=a if first=='A' else b;partner=bv if first=='A' else av;partner_area=area_b if first=='A' else area_a
            # For A: h_Q(-sign J a). For B: h_P(sign J b).
            orientation=-sign if first=='A' else sign
            geometric=[max(dot(v,(-orientation*n[1],orientation*n[0])) for v in partner)/(4*partner_area) for n in normals]
            assert abs(dot(alpha if first=='A' else gamma,geometric)-first_q)<=tol*100
            # Angular-prefix and support-function coefficients differ by a linear form.
            geomdual=dual_certificate(normals,geometric,alpha if first=='A' else gamma,tol)
            second='B' if first=='A' else 'A';co2=m.coefficients(a,b,word,alpha if second=='B' else gamma,second)
            if second=='B':gamma=first_best(cb,co2,tol)
            else:alpha=first_best(ca,co2,tol)
            final=m.Q(a,b,word,alpha,gamma);assert final>=first_q-tol*100 and final>0
            seconddual=dual_certificate(b if second=='B' else a,co2,gamma if second=='B' else alpha,tol)
            variants.append(dict(sign=sign,first=first,word=word,first_Q=first_q,Q=final,alpha=alpha,gamma=gamma,capacity=1/(2*final),first_coefficients=co,first_dual=dual,containment_coefficients=geometric,containment_dual=geomdual,containment_radius=geomdual['t']*4*partner_area,second_coefficients=co2,second_dual=seconddual))
    maximum=max(v['Q'] for v in variants);chosen=next(v for v in variants if v['Q']>=maximum-tol)
    return dict(canonical_alpha=ac,canonical_gamma=bc,area_A=area_a,area_B=area_b,LP_vertex_counts=[len(ca),len(cb)],variants=variants,chosen=chosen,capacity=chosen['capacity'])

def main():
    p=argparse.ArgumentParser();p.add_argument('--source',type=Path,required=True);p.add_argument('--out',type=Path,required=True);p.add_argument('--twenty',action='store_true');args=p.parse_args()
    assert hashlib.sha256(args.source.read_bytes()).hexdigest()==m.old.SOURCE_SHA
    prior=json.loads((ROOT/'patterns/upper-tail/signed-prefix-response/results.json').read_text());names=[r['name'] for r in prior['rows']] if args.twenty else ['random_5x5_s1_64']
    source={}
    for line in args.source.open():
        r=json.loads(line)
        if r['name'] in names:source[r['name']]=r
    rows=[]
    for name in names:
        r=source[name];v=[list(map(F,u)) for u in r['dual_vertices_rational']];a=[u[:2] for u in v[:r['k']]];b=[u[2:] for u in v[r['k']:]];s=select(a,b)
        old=next(z for z in prior['rows'] if z['name']==name)
        rows.append(dict(name=name,geometry=r['dual_vertices_rational'],k=r['k'],selection=s,prior_response_capacity=old['selection']['capacity'],exact_new_over_prior=s['capacity']/F(old['selection']['capacity']),historical_capacity=r['capacity'],historical_ratio=r['sys'],principal_top3=old['principal_top3'],capacity_over_historical=float(s['capacity'])/r['capacity'],ratio_bound=float(s['capacity']**2/(2*s['area_A']*s['area_B']))))
    regular=[];h=math.cos(math.pi/5)
    for theta in [0,math.pi/20,math.pi/10]:
        a=[(math.cos((2*i+1)*math.pi/5)/h,math.sin((2*i+1)*math.pi/5)/h) for i in range(5)];b=[(math.cos((2*i+1)*math.pi/5+theta)/h,math.sin((2*i+1)*math.pi/5+theta)/h) for i in range(5)]
        s=select(a,b,1e-11);known=(1+h)**2/math.cos(theta)
        regular.append(dict(theta=theta,selection=s,known_capacity=known,capacity_over_known=s['capacity']/known))
    result=dict(method='Full-partner containment contact vertex, one other-factor response, both shadow signs and first-factor exchange; no inherited seeds',source_sha256=m.old.SOURCE_SHA,plan_sha256=hashlib.sha256(Path(__file__).with_name('PLAN.md').read_bytes()).hexdigest(),analyze_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),rows=rows,regular_controls=regular)
    args.out.write_text(json.dumps(m.stringify(result),indent=2)+'\n')
    print(json.dumps(dict(rows=[dict(name=r['name'],new_over_prior=float(r['exact_new_over_prior']),capacity_over_historical=r['capacity_over_historical']) for r in rows],regular=[r['capacity_over_known'] for r in regular])))
if __name__=='__main__':main()
