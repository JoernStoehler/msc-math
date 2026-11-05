#!/usr/bin/env python3
"""Independent primal/dual reflected-cover + full selected-word check."""
import argparse,hashlib,itertools,json,time
from fractions import Fraction as F
from functools import cmp_to_key
from pathlib import Path

def dot(u,v):return sum(x*y for x,y in zip(u,v))
def det(u,v):return u[0]*v[1]-u[1]*v[0]
def solve(A,b):
 M=[list(u)+[v] for u,v in zip(A,b)];n=len(M)
 for j in range(n):
  k=next(i for i in range(j,n) if M[i][j]);M[j],M[k]=M[k],M[j];q=M[j][j];M[j]=[v/q for v in M[j]]
  for i in range(n):
   if i!=j:q=M[i][j];M[i]=[v-q*w for v,w in zip(M[i],M[j])]
 return [r[-1] for r in M]
def cmp(u,v):
 a=int(u[1]<0 or(u[1]==0 and u[0]<0));b=int(v[1]<0 or(v[1]==0 and v[0]<0))
 if a!=b:return a-b
 d=det(u,v);return (d<0)-(d>0)
def geometry(a):
 vv=[]
 for i,j in itertools.combinations(range(len(a)),2):
  if det(a[i],a[j])==0:continue
  v=solve([a[i],a[j]],[F(1),F(1)])
  if all(dot(u,v)<=1 for u in a) and v not in vv:vv.append(v)
 center=[sum(v[k] for v in vv)/len(vv) for k in [0,1]];vv.sort(key=cmp_to_key(lambda u,v:cmp([x-y for x,y in zip(u,center)],[x-y for x,y in zip(v,center)])))
 area=abs(sum(det(vv[i],vv[(i+1)%len(vv)]) for i in range(len(vv))))/2
 H=[max(-dot(u,v) for v in vv) for u in a]
 edges=[[y-x for x,y in zip(vv[i],vv[(i+1)%len(vv)])] for i in range(len(vv))];dd=[det(edges[i],edges[(i+1)%len(edges)]) for i in range(len(edges))]
 return vv,area,H,len(set(dd))>1
def closures(a):
 out=[];n=len(a)
 for ids in itertools.combinations(range(n),3):
  try:w=solve([[F(1)]*3,[a[i][0] for i in ids],[a[i][1] for i in ids]],[F(1),F(0),F(0)])
  except StopIteration:continue
  if min(w)<=0:continue
  v=[F(0)]*n
  for i,x in zip(ids,w):v[i]=x
  out.append(v)
 for i,j in itertools.combinations(range(n),2):
  if det(a[i],a[j]) or dot(a[i],a[j])>=0:continue
  k=next(k for k in [0,1] if a[i][k]);x=-a[j][k]/(a[i][k]-a[j][k]);v=[F(0)]*n;v[i]=x;v[j]=1-x;out.append(v)
 return out

def main():
 p=argparse.ArgumentParser();p.add_argument('--repo',type=Path,required=True);p.add_argument('--directory',type=Path,required=True);p.add_argument('--source',type=Path,required=True);o=p.parse_args();d=o.directory;t=time.perf_counter();src=o.repo/'patterns/upper-tail/contact-stability';geom={r['name']:(r['A'],r['B']) for r in json.loads((src/'sample.json').read_text())['rows']};gc=json.loads((src/'global-counterexample-input.json').read_text());geom[gc['name']]=(gc['A'],gc['B']);r=json.loads((d/'minkowski-results.json').read_text());missing={x['name'] for x in r['rows']}-set(geom)
 for line in o.source.open():
  x=json.loads(line)
  if x['name'] in missing:
   v=x['dual_vertices_rational'];geom[x['name']]=([u[:2] for u in v[:x['k']]],[u[2:] for u in v[x['k']:]])
 checks=[];words=0
 for row in r['rows']:
  a,b=([[F(z) for z in u] for u in fac] for fac in geom[row['name']]);f=row['feature'];vol=F(1);nonregular=[];faceweights=[];primalcert=[]
  for u,fac in [(a,f['A_face']),(b,f['B_face'])]:
   vv,area,H,nr=geometry(u);vol*=area;nonregular.append(nr);cc=closures(u);obj=[dot(w,H) for w in cc];lam=max(obj);expected=[v for v,x in zip(cc,obj) if x==lam];retained=[[F(z) for z in w] for w in fac['weights']];assert expected==retained;assert lam==F(fac['asymmetry']);assert H==list(map(F,fac['opposite_supports']));assert len(expected)==1
   al=expected[0];ids=[i for i,x in enumerate(al) if x];assert len(ids)==3
   c=solve([[*u[i],F(-1)] for i in ids],[-H[i] for i in ids]);assert c[2]==lam;slack=[lam-dot(v,c[:2])-hh for v,hh in zip(u,H)];assert min(slack)>=0 and all(slack[i]>0 for i in range(len(u)) if i not in ids)
   primalcert.append(dict(c=[str(x) for x in c[:2]],asymmetry=str(lam),contact_indices=ids,slacks=[str(x) for x in slack]));faceweights.append(al)
  al,ga=faceweights;n=len(a);labels=[i for i,x in enumerate(al) if x]+[n+j for j,x in enumerate(ga) if x];P=[u+[F(0),F(0)] for u in a]+[[F(0),F(0)]+u for u in b];W=[x/2 for x in al+ga]
  def q(word):
   return sum(W[i]*W[j]*(P[i][0]*P[j][2]+P[i][1]*P[j][3]-P[i][2]*P[j][0]-P[i][3]*P[j][1]) for k,i in enumerate(word) for j in word[k+1:])
  value=max(q([labels[0],*tail]) for tail in itertools.permutations(labels[1:]));words+=120;assert value==F(f['witness']['Q']);assert q(f['witness']['word'])==value;cap=1/(2*value);ratio=cap*cap/(2*vol);assert cap==F(f['capacity_bound']) and ratio==F(f['ratio_bound']) and vol==F(f['volume'])
  above_HKO=5*ratio-3>0 and (5*ratio-3)**2>5
  checks.append(dict(name=row['name'],non_affine_regular_factors=nonregular,primal_cover_certificates=primalcert,ratio_bound_above_HKO_exact=above_HKO,selected_capacity=str(cap),selected_ratio=str(ratio)))
 # This precise nonregular constructed geometry alone refutes an HKO ceiling.
 target=next(x for x in checks if x['name']=='contact_full_s0_e3of1000');assert all(target['non_affine_regular_factors']) and target['ratio_bound_above_HKO_exact'];counter=next(x for x in checks if x['name']==gc['name']);truth=json.loads((src/'global-counterexample.json').read_text());assert F(counter['selected_capacity'])==F(truth['exact_capacity']) and F(counter['selected_ratio'])==F(truth['exact_ratio'])
 out=dict(status='PASS',rational_geometries=len(checks),independent_full_selected_words=words,all_reflected_cover_LPs_unique_dual_vertex_and_exact_primal_dual_equality=True,nonregular_above_HKO_counterexample='contact_full_s0_e3of1000',global_counterexample_capacity_exactly_recovered=True,checks=checks,seconds=time.perf_counter()-t,input_hashes={n:hashlib.sha256((Path(__file__).with_name(n) if n=='minkowski.py' else d/n).read_bytes()).hexdigest() for n in ['minkowski.py','minkowski-results.json','MINKOWSKI-PLAN.md']},verifier_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest());(d/'minkowski-verification.json').write_text(json.dumps(out,indent=2)+'\n');print(json.dumps({k:v for k,v in out.items() if k not in ['checks','input_hashes']},indent=2))
if __name__=='__main__':main()
