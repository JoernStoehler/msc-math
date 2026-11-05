#!/usr/bin/env python3
"""Independent finite moment verification; never used for prediction."""
import argparse,hashlib,importlib.util,json,itertools,math
from fractions import Fraction as F
from pathlib import Path

def closures(a):
 out=[];n=len(a)
 for k in [2,3]:
  for ids in itertools.combinations(range(n),k):
   # Solve all equations on the selected support, choosing an independent row set.
   rows=[[F(1)]*k,[a[i][0] for i in ids],[a[i][1] for i in ids]];rhs=[F(1),F(0),F(0)];ans=None
   for eq in itertools.combinations(range(3),k):
    z=[rows[j][:]+[rhs[j]] for j in eq]
    for j in range(k):
     pivot=next((i for i in range(j,k) if z[i][j]),None)
     if pivot is None:break
     z[j],z[pivot]=z[pivot],z[j];scale=z[j][j];z[j]=[x/scale for x in z[j]]
     for i in range(k):
      if i!=j:
       scale=z[i][j];z[i]=[x-scale*y for x,y in zip(z[i],z[j])]
    else:
     ans=[z[i][-1] for i in range(k)];break
   if ans is None or min(ans)<=0:continue
   if any(sum(x*y for x,y in zip(row,ans))!=val for row,val in zip(rows,rhs)):continue
   v=[F(0)]*n
   for i,w in zip(ids,ans):v[i]=w
   if v not in out:out.append(v)
 return out

def action(a,b,word,alpha,gamma):
 # Genuine four-dimensional weighted covectors and their symplectic forward sum.
 z=[(alpha[i]*v[0]/2,alpha[i]*v[1]/2,F(0),F(0)) for i,v in enumerate(a)]+[(F(0),F(0),gamma[j]*v[0]/2,gamma[j]*v[1]/2) for j,v in enumerate(b)]
 def omega(u,v):return u[0]*v[2]+u[1]*v[3]-u[2]*v[0]-u[3]*v[1]
 return sum(omega(z[i],z[j]) for k,i in enumerate(word) for j in word[k+1:])

def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def main():
 p=argparse.ArgumentParser();p.add_argument('--sample',type=Path,required=True);p.add_argument('--truth',type=Path,required=True);p.add_argument('--directory',type=Path,default=Path(__file__).parent);o=p.parse_args();pred=json.loads((o.directory/'predictions.json').read_text());sample=json.loads(o.sample.read_text());truth={r['name']:r for r in json.loads(o.truth.read_text())['rows']};assert pred['sample_sha256']==sha(o.sample);count=0
 for r,pr in zip(sample['rows'],pred['rows']):
  assert r['name']==pr['name'];a=[[F(x) for x in v] for v in r['A']];b=[[F(x) for x in v] for v in r['B']];ca=closures(a);cb=closures(b);N=len(ca)*len(cb)
  for z in pr['moments']:
   vals=[action(a,b,z['word'],x,y) for x in ca for y in cb];assert min(vals)>=0
   assert sum(vals)/N==F(z['m1']);assert sum(x*x for x in vals)/N==F(z['m2']);assert F(z['L'])==F(z['m2'])/F(z['m1']);count+=N
  L=max(F(z['L']) for z in pr['moments']);V=F(pr['volume']);assert F(pr['rigorous_bound'])==1/(8*V*L*L);assert F(pr['rigorous_bound'])>=F(truth[r['name']]['ratio'])
 # Invariance checks on six independent geometries, before sharing the producer's order conventions.
 s=importlib.util.spec_from_file_location('action_probe',o.directory/'probe.py');m=importlib.util.module_from_spec(s);s.loader.exec_module(m);invariants=0
 def sig(a,b):return sorted((z['m1'],z['m2']) for z in m.score(a,b))
 for r in sample['rows'][::10]:
  a=[[F(x) for x in v] for v in r['A']];b=[[F(x) for x in v] for v in r['B']];original=sig(a,b)
  # Covector transform a'=G^-T a,b'=Gb for G=[[1,1],[0,1]].
  assert sig([(x,y-x) for x,y in a],[(x+y,y) for x,y in b])==original;invariants+=1
  # Orientation-reversing G=diag(-1,1) swaps the two handedness moments.
  assert sig([(-x,y) for x,y in a],[(-x,y) for x,y in b])==original;invariants+=1
  tr=(F(1,100),F(-1,100));trans=lambda vs:[(x/(1-x*tr[0]-y*tr[1]),y/(1-x*tr[0]-y*tr[1])) for x,y in vs]
  assert sig(trans(a),trans(b))==original;invariants+=1
  assert sig(a[::-1],b[1:]+b[:1])==original;invariants+=1
 # Exact Q(sqrt5) moment algebra, represented by pairs a+b*sqrt5.
 def add(x,y):return(x[0]+y[0],x[1]+y[1])
 def mul(x,y):return(x[0]*y[0]+5*x[1]*y[1],x[0]*y[1]+x[1]*y[0])
 zero=(F(0),F(0));a0=(F(3,4),F(-1,4));phi_inv=(F(-1,2),F(1,2));beta=(F(1,4),F(1,4));loads=[a0,(F(-1,2),F(0)),phi_inv,(F(-1,2),F(0)),a0];total=zero;square=zero
 for l in loads:total=add(total,l);square=add(square,mul(l,l))
 assert total==zero;assert square==mul((F(5),F(0)),a0)
 bound_co=add(beta,mul((F(-4),F(0)),a0));rms_co=add(beta,mul((F(-3),F(0)),a0));assert bound_co==(F(-11,4),F(5,4));assert rms_co==(F(-2),F(1))
 # Exact HKO moment calibration and whole support-path identities in Q(sqrt5).
 repo=next((p for p in Path(__file__).resolve().parents if (p/'AGENTS.md').is_file()),Path.cwd())
 sp=importlib.util.spec_from_file_location('action_q5',repo/'patterns/upper-tail/mixed-area-global/verify.py');q=importlib.util.module_from_spec(sp);sp.loader.exec_module(q)
 K=q.K;rt=K(0,1);be=(1+rt)/4;r=(rt-1)/4;rho=(3+rt)/5;gold=(1+rt)/2
 n=[[K(1),K(0)],[r,K(1)],[-be,2*r],[-be,-2*r],[r,K(-1)]];bn=[q.J(v) for v in n];d=[K(1),-be,r,r,-be];ca=q.closures(n);ell=[q.dot(c,d) for c in ca];base=m.score(n,bn);a0=5*(5-rt)/4;L0=max(z['L'] for z in base);B0=1/(8*a0*a0*L0*L0)
 assert B0==K(F(698,1089),F(104,363));path=[]
 for t in [F(1,25),F(-1,25),F(1,32),F(-1,32),F(1,100),F(-1,100)]:
  h=[1+t*x for x in d];dual=[[x/hh for x in v] for v,hh in zip(n,h)];now=m.score(dual,bn);S1=sum(1/(1+t*x) for x in ell)/5;S2=sum(1/((1+t*x)*(1+t*x)) for x in ell)/5
  for z,z0 in zip(now,base):assert z['m1']==z0['m1']*S1 and z['m2']==z0['m2']*S2
  calibrated_bound=rho*S1*S1/((1-be*t*t)*S2*S2);calibrated_rms=rho/((1-be*t*t)*S2);core=min(q.dot(c,h) for c in ca);actual=rho*core*core/(1-be*t*t)
  path.append(dict(t=str(t),bound_predictor=q.serial(calibrated_bound),RMS_predictor=q.serial(calibrated_rms),ratio=q.serial(actual),above_one=actual>1,display=dict(bound=float(calibrated_bound),RMS=float(calibrated_rms),ratio=float(actual))))
 out=dict(status='PASS',sample_sha256=sha(o.sample),predictions_sha256=sha(o.directory/'predictions.json'),truth_sha256=sha(o.truth),rows=len(sample['rows']),direct_four_dimensional_actions=count,exact_invariance_controls=invariants,exact_load_sum='0',exact_load_mean_square='(3-sqrt5)/4',bound_predictor_quadratic_coefficient='(5*sqrt5-11)/4',rms_predictor_quadratic_coefficient='sqrt5-2',HKO_bound=q.serial(B0),HKO_bound_over_actual=q.serial(B0/rho),exact_support_path_controls=path,scope='Independent direct HK action averaging and invariance/algebra; theorem proof separate; no prediction refit')
 (o.directory/'verification.json').write_text(json.dumps(out,indent=2)+'\n');print(json.dumps(out))
if __name__=='__main__':main()
