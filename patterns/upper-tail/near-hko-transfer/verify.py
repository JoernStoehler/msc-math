#!/usr/bin/env python3
"""Independent standard-library checker; imports no producer or optimizer."""
import argparse,hashlib,json
from fractions import Fraction as F
from functools import cmp_to_key
from itertools import combinations
from pathlib import Path

def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def dot(a,b):return sum(x*y for x,y in zip(a,b))
def det(u,v):return u[0]*v[1]-u[1]*v[0]
def support(v):return [i for i,x in enumerate(v) if x]
def solve(A,b):
 M=[[F(v) for v in row]+[F(x)] for row,x in zip(A,b)];n=len(M)
 for j in range(n):
  pivot=next((i for i in range(j,n) if M[i][j]),None)
  if pivot is None:return None
  M[j],M[pivot]=M[pivot],M[j];v=M[j][j];M[j]=[x/v for x in M[j]]
  for i in range(n):
   if i!=j:
    v=M[i][j];M[i]=[x-v*y for x,y in zip(M[i],M[j])]
 return [v[-1] for v in M]
def rank(rows):
 basis={}
 for source in rows:
  row=list(map(F,source))
  for k,v in sorted(basis.items()):
   c=row[k];row=[x-c*y for x,y in zip(row,v)]
  k=next((i for i,x in enumerate(row) if x),None)
  if k is not None:
   c=row[k];basis[k]=[x/c for x in row]
 return len(basis)
def determinant(A):
 M=[list(map(F,row)) for row in A];value=F(1)
 for j in range(len(M)):
  i=next((i for i in range(j,len(M)) if M[i][j]),None)
  if i is None:return F(0)
  if i!=j:M[i],M[j]=M[j],M[i];value=-value
  pivot=M[j][j];value*=pivot
  for i in range(j+1,len(M)):
   c=M[i][j]/pivot
   for k in range(j+1,len(M)):M[i][k]-=c*M[j][k]
 return value

def stability(a,b,variants):
 A=[[F(1)]*25]
 for i in range(5):
  for k in range(2):A.append([b[j][k] if ii==i else F(0) for ii in range(5) for j in range(5)])
 for j in range(5):
  for k in range(2):A.append([a[i][k] if jj==j else F(0) for i in range(5) for jj in range(5)])
 assert rank(A)==17;out={}
 for v in variants:
  W=list(map(F,v['W']));positive=[i for i,x in enumerate(W) if x>0];nonbasic=[i for i,x in enumerate(W) if x==0];assert len(positive)==17 and set(positive)==set(v['basis']);Ar=[A[i] for i in v['independent_rows']];assert rank(Ar)==17
  basis_det=determinant([[row[j] for j in positive] for row in Ar]);assert basis_det!=0;slack=list(map(F,v['dual_slack']));assert all(slack[i]==0 for i in positive) and min(slack[i] for i in nonbasic)>0
  # An unchanged circular and linear angular chamber exists; no tied rays or cut crossings.
  vectors=a+[[v['shadow_sign']*-u[1],v['shadow_sign']*u[0]] for u in b];ray_dets=[abs(det(u,w)) for u,w in combinations(vectors,2)];assert min(ray_dets)>0;assert min(abs(u[1]) for u in vectors)>0
  out[str(v['shadow_sign'])]=dict(equality_rank=17,positive_count=17,nonbasic_count=8,positive_basis_columns=positive,reduced_rows=v['independent_rows'],positive_basis_determinant=basis_det,min_positive_primal=min(W[i] for i in positive),min_nonbasic_dual_slack=min(slack[i] for i in nonbasic),min_absolute_shadow_ray_determinant=min(ray_dets),min_absolute_shadow_y=min(abs(u[1]) for u in vectors),unique_non_degenerate_LP_optimum=True)
 # No closure combinatorics boundary is met: all pair determinants and triple affine determinants are nonzero.
 geometry={}
 for key,fac in [('A',a),('B',b)]:
  pair=min(abs(det(u,v)) for u,v in combinations(fac,2));trip=min(abs(det([u[0]-w[0],u[1]-w[1]],[v[0]-w[0],v[1]-w[1]])) for u,v,w in combinations(fac,3));assert pair>0 and trip>0;geometry[key]=dict(min_absolute_pair_determinant=pair,min_absolute_triple_affine_determinant=trip)
 return dict(shadows=out,geometry=geometry,conclusion='For exact LP solves, the same unique bases and strict nonzero row/column masses persist locally; finite one-response maxima and exact capacity vary continuously. Strict Q_round<Q_true and sys>1 persist on some open neighborhood inside the planar 5x5 product geometry chamber. No numerical radius or arbitrary alternative-optimizer assumption is needed; actual solver/reconstruction success on every nearby machine input is not asserted.')

def vertices(a):
 out=[];n=len(a)
 for ids in combinations(range(n),2):
  i,j=ids
  if det(a[i],a[j]) or dot(a[i],a[j])>=0:continue
  k=next(k for k in range(2) if a[i][k]);w=-a[j][k]/(a[i][k]-a[j][k]);v=[F(0)]*n;v[i]=w;v[j]=1-w;out.append(v)
 for ids in combinations(range(n),3):
  v=solve([[1]*3,[a[i][0] for i in ids],[a[i][1] for i in ids]],[1,0,0])
  if v is None or min(v)<0:continue
  w=[F(0)]*n
  for i,x in zip(ids,v):w[i]=x
  if w not in out:out.append(w)
 assert out;return out

def feasible(a,v):
 assert min(v)>=0 and sum(v)==1
 assert all(sum(x*u[k] for x,u in zip(v,a))==0 for k in range(2))
def angular(vectors):
 def cmp(i,j):
  u,v=vectors[i],vectors[j];hu=int(u[1]<0 or (u[1]==0 and u[0]<0));hv=int(v[1]<0 or (v[1]==0 and v[0]<0))
  if hu!=hv:return hu-hv
  t=det(u,v)
  return (-1 if t>0 else 1) if t else (i>j)-(i<j)
 return sorted(range(len(vectors)),key=cmp_to_key(cmp))
def word(a,b,sign):
 w=angular(a+[[sign*-v[1],sign*v[0]] for v in b]);return w if sign>0 else w[::-1]
def q(a,b,w,alpha,gamma):
 feasible(a,alpha);feasible(b,gamma);n=len(a);points=[u+[F(0),F(0)] for u in a]+[[F(0),F(0)]+u for u in b];weights=[x/2 for x in alpha+gamma]
 assert len(w)==len(set(w))==len(points) and set(w)==set(range(len(points)))
 return sum(weights[i]*weights[j]*(points[i][0]*points[j][2]+points[i][1]*points[j][3]-points[i][2]*points[j][0]-points[i][3]*points[j][1]) for k,i in enumerate(w) for j in w[k+1:])
def area(a):
 vv=[]
 for i,j in combinations(range(len(a)),2):
  v=solve([a[i],a[j]],[1,1])
  if v is not None and all(dot(u,v)<=1 for u in a) and v not in vv:vv.append(v)
 assert len(vv)==len(a)==5
 for u in a:assert sum(dot(u,v)==1 for v in vv)==2
 # Positive normalized closure using every normal proves boundedness; origin satisfies every row strictly.
 cv=vertices(a);avg=[sum(v[i] for v in cv)/len(cv) for i in range(len(a))];feasible(a,avg);assert min(avg)>0
 center=[sum(v[k] for v in vv)/len(vv) for k in range(2)];order=angular([[x-y for x,y in zip(v,center)] for v in vv]);vv=[vv[i] for i in order];return abs(sum(det(vv[i],vv[(i+1)%len(vv)]) for i in range(len(vv))))/2

def lp(a,b,v):
 W=list(map(F,v['W']));dual=list(map(F,v['dual']));n=len(a);m=len(b);assert min(W)>=0 and sum(W)==1
 assert len(dual)==1+2*n+2*m
 for i in range(n):assert all(sum(W[i*m+j]*b[j][k] for j in range(m))==0 for k in range(2))
 for j in range(m):assert all(sum(W[i*m+j]*a[i][k] for i in range(n))==0 for k in range(2))
 assert v['word']==word(a,b,v['shadow_sign']);pos={x:i for i,x in enumerate(v['word'])};cs=[]
 for i in range(n):
  for j in range(m):
   co=dot(a[i],b[j])*(1 if pos[i]<pos[n+j] else -1)/4;cs.append(co);slack=dual[0]+dot(dual[1+2*i:3+2*i],b[j])+dot(dual[1+2*n+2*j:3+2*n+2*j],a[i])-co;assert slack>=0 and slack==F(v['dual_slack'][i*m+j])
 assert dot(W,cs)==dual[0]==F(v['Q']);return W

def rounding(a,b,variants,rd,ca,cb):
 seen=[];count=0;qs=[]
 for v in variants:
  W=list(map(F,v['W']))
  for kind in ['row','column']:
   for k in range(5):
    vals=W[k*5:k*5+5] if kind=='row' else [W[i*5+k] for i in range(5)];mass=sum(vals)
    if not mass:continue
    count+=1;fixed=[x/mass for x in vals];cand=rd['candidates'][len(seen)];assert (cand['shadow_sign'],cand['kind'],cand['index'])==(v['shadow_sign'],kind,k);assert cand['word']==v['word'] and F(cand['condition_mass'])==mass
    alpha=list(map(F,cand['alpha']));gamma=list(map(F,cand['gamma']));assert (gamma if kind=='row' else alpha)==fixed
    values=[q(a,b,v['word'],u,gamma) for u in ca] if kind=='row' else [q(a,b,v['word'],alpha,u) for u in cb];value=q(a,b,v['word'],alpha,gamma);assert value==max(values)==F(cand['Q']);assert F(cand['capacity'])==1/(2*value)
    # Exact imported max tie convention: first closure vertex in enumeration order.
    selected=(ca if kind=='row' else cb)[values.index(max(values))];assert selected==(alpha if kind=='row' else gamma)
    seen.append(cand);qs.append(value)
 assert rd['candidate_count']==count==len(rd['candidates']);assert rd['best']==seen[qs.index(max(qs))];assert F(rd['capacity'])==1/(2*max(qs));return max(qs)

def canon(v):
 if isinstance(v,str):
  try:return F(v)
  except ValueError:return v
 if isinstance(v,dict):return {k:canon(x) for k,x in v.items()}
 if isinstance(v,list):return [canon(x) for x in v]
 return v

def serial(v):
 if isinstance(v,F):return str(v)
 if isinstance(v,dict):return {k:serial(x) for k,x in v.items()}
 if isinstance(v,list):return [serial(x) for x in v]
 return v

def main():
 p=argparse.ArgumentParser();p.add_argument('--repo',type=Path,required=True);p.add_argument('--directory',type=Path,required=True);o=p.parse_args();r=o.repo;d=o.directory
 frozen=json.loads((d/'frozen-manifest.json').read_text())
 for k,v in frozen['sources'].items():assert sha(r/k)==v
 for k,v in frozen['freeze'].items():assert sha(d/k)==v
 sample=json.loads((d/'sample.json').read_text());h=json.loads((r/'patterns/upper-tail/conditional-coupling/hko-exact.json').read_text());base={k:[[F(v['rational'])+F(2236068,1000000)*F(v['sqrt5']) for v in u] for u in h[k]] for k in ['A','B']}
 assert len(sample['rows'])==12
 for index,s in enumerate(sample['rows']):
  seed=index//2;eps=[F(1,1000),F(1,100)][index%2];raw=hashlib.sha256(('msc-math/thesis/near-hko-validation/v1:'+str(seed)).encode()).digest();assert s['seed']==seed and F(s['epsilon'])==eps and s['direction_digest']==raw.hex();coords=[v+eps*F(byte%9-4,4) for v,byte in zip([x for k in ['A','B'] for u in base[k] for x in u],raw)];assert [F(x) for k in ['A','B'] for u in s[k] for x in u]==coords
 pred=json.loads((d/'predictions.json').read_text());out=json.loads((d/'results.json').read_text());assert sha(d/'predictions.json')==out['predictions_sha256'];assert pred['sample_sha256']==out['sample_sha256']==sha(d/'sample.json');assert pred['plan_sha256']==out['plan_sha256']==sha(d/'PLAN.md');assert pred['analyze_sha256']==out['analyze_sha256']==sha(d/'analyze.py');assert len(pred['rows'])==12
 checks=[];obstruction=None
 for index,s in enumerate(pred['rows']):
  for k,v in sample['rows'][index].items():assert s[k]==v
  assert s['status']=='OK';a=[list(map(F,u)) for u in s['A']];b=[list(map(F,u)) for u in s['B']];ca=vertices(a);cb=vertices(b);assert len(ca)==len(cb)==5
  for v in s['variants']:lp(a,b,v)
  qr=rounding(a,b,s['variants'],s['rounding'],ca,cb);volume=area(a)*area(b)
  if index>=len(out['rows']):continue
  row=out['rows'][index]
  for k,v in s.items():assert row[k]==v
  table=[]
  for v in s['variants']:
   for ia,alpha in enumerate(ca):
    for ib,gamma in enumerate(cb):table.append(dict(shadow_sign=v['shadow_sign'],A_vertex=ia,B_vertex=ib,alpha=alpha,gamma=gamma,word=v['word'],Q=q(a,b,v['word'],alpha,gamma)))
  assert canon(table)==canon(row['truth']['oracle_candidates']);qt=max(v['Q'] for v in table);ties=[v for v in table if v['Q']==qt];assert canon(ties)==canon(row['truth']['all_ties']);assert F(row['truth']['Q'])==qt and F(row['truth']['capacity'])==1/(2*qt);assert F(row['volume'])==volume and F(row['ratio'])==1/(8*qt*qt*volume);assert row['exact_recovery']==(qr==qt);assert F(row['capacity_over_true'])==qt/qr and F(row['relative_capacity_error'])==qt/qr-1;assert row['at_least_one_percent']==(qt/qr>=F(101,100));assert qr<=qt
  mech=row['mechanism'];assert canon(ca)==canon(mech['closure_vertices_A']) and canon(cb)==canon(mech['closure_vertices_B'])
  for v in s['variants']:
   W=list(map(F,v['W']));key=str(v['shadow_sign']);counts={}
   for factor,cv in [('A',ca),('B',cb)]:
    vals=[]
    for k in range(5):
     vals0=[W[i*5+k] for i in range(5)] if factor=='A' else W[k*5:k*5+5];mass=sum(vals0)
     if mass:
      weights=[x/mass for x in vals0];vals.append(dict(index=k,weights=weights,support=support(vals0),closure_vertex=next((i for i,c in enumerate(cv) if c==weights),None)))
    assert canon(vals)==canon(mech['conditionals'][key][factor]);counts[factor]=sorted({v['closure_vertex'] for v in vals if v['closure_vertex'] is not None});assert counts[factor]==mech['conditional_vertex_coverage'][key][factor]
   marginalA=[sum(W[i*5+j] for j in range(5)) for i in range(5)];marginalB=[sum(W[i*5+j] for i in range(5)) for j in range(5)];assert mech['rank_one_by_shadow'][key]==(W==[x*y for x in marginalA for y in marginalB])
  for v,winner in zip(ties,mech['winning_pairs']):
   assert winner['shadow_sign']==v['shadow_sign'] and winner['A_vertex']==v['A_vertex'] and winner['B_vertex']==v['B_vertex'];assert winner['A_support']==support(v['alpha']) and winner['B_support']==support(v['gamma']);assert winner['A_conditionally_seen']==(v['A_vertex'] in mech['conditional_vertex_coverage'][str(v['shadow_sign'])]['A']);assert winner['B_conditionally_seen']==(v['B_vertex'] in mech['conditional_vertex_coverage'][str(v['shadow_sign'])]['B'])
  checks.append(dict(name=s['name'],exact_recovery=qr==qt,ratio=float(F(row['ratio'])),above_one=F(row['ratio'])>1,capacity_error=float(qt/qr-1),all_pair_word_evaluations=len(table)))
  if qr!=qt:
   assert index==len(out['rows'])-1 and len(ties)==2 and {v['shadow_sign'] for v in ties}=={1,-1};win=ties[0];assert all(v['A_vertex']==win['A_vertex'] and v['B_vertex']==win['B_vertex'] for v in ties)
   for v in ties:
    key=str(v['shadow_sign']);assert v['A_vertex'] not in mech['conditional_vertex_coverage'][key]['A'] and v['B_vertex'] not in mech['conditional_vertex_coverage'][key]['B']
   best=s['rounding']['best'];alpha=list(map(F,best['alpha']));gamma=list(map(F,best['gamma']));assert best['kind']=='row' and best['index']==0 and best['shadow_sign']==1 and alpha==win['alpha'];good=win['gamma'];bad=cb[4];lam=gamma[2]/good[2];assert 0<lam<1 and gamma==[lam*x+(1-lam)*y for x,y in zip(good,bad)];qbad=q(a,b,best['word'],alpha,bad);assert qbad<qt and qr==lam*qt+(1-lam)*qbad
   stability_certificate=stability(a,b,s['variants']);obstruction=dict(name=s['name'],unique_winning_supports=dict(A=support(win['alpha']),B=support(win['gamma'])),both_words_same_unique_pair=True,stability=stability_certificate,winning_factors_absent_from_both_shadows_conditionals=True,best_rounded_candidate=dict(shadow_sign=1,kind='row',index=0),best_fixed_B_support=support(gamma),good_B_support=support(good),bad_B_support=support(bad),good_B_mixture_weight=lam,bad_B_mixture_weight=1-lam,Q_true=qt,Q_bad_pair=qbad,Q_round=qr,Q_gap_identity=(1-lam)*(qt-qbad),relative_capacity_error=qt/qr-1,geometry_statement='Best candidate already has winning A but fixes a strict mixture of winning B{1,2,4} and losing B{1,3,4}. Uniqueness of maximizing vertex pair plus absence of either pure winning conditional forces every one-response candidate strictly below the true Q.')
   assert qt-qr==obstruction['Q_gap_identity']
 assert len(checks)==out['scored_count'];assert out['unscored_names']==[s['name'] for s in pred['rows'][len(checks):]];assert obstruction is not None
 (d/'obstruction.json').write_text(json.dumps(serial(obstruction),indent=2)+'\n')
 result=dict(status='PASS',frozen_inputs=12,predictions_checked=12,exact_LP_certificates_checked=24,rounded_candidates_checked=sum(s['rounding']['candidate_count'] for s in pred['rows']),scored=len(checks),oracle_pair_word_evaluations=sum(v['all_pair_word_evaluations'] for v in checks),exact_recoveries=sum(v['exact_recovery'] for v in checks),above_one=sum(v['above_one'] for v in checks),unscored=out['unscored_names'],checks=checks,obstruction_verified=True,hashes={k:sha(d/k) for k in ['PLAN.md','freeze.py','sample.json','frozen-manifest.json','analyze.py','predictions.json','results.json','verify.py','obstruction.json']})
 (d/'verification.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(result,indent=2))
if __name__=='__main__':main()
