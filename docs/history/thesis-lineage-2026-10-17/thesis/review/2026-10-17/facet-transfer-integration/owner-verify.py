#!/usr/bin/env python3
"""Independent Gaussian closure/4D-response/prefix-oracle audit of added-facet transfer."""
import hashlib,importlib.util,json,sys,functools,time
from fractions import Fraction as F
from pathlib import Path
T=Path('/workspaces/msc-math-worktrees/thesis-2026-10-16')
P=Path('/workspaces/msc-math-worktrees/patterns-2026-10-17')
D=Path(__file__).parent

def module(name,path):
 s=importlib.util.spec_from_file_location(name,path);m=importlib.util.module_from_spec(s);s.loader.exec_module(m);return m

def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
g=module('independent_geom',T/'thesis/review/2026-10-17/near-hko-transfer-integration/owner-verify.py')
rp=module('independent_replay',T/'thesis/review/2026-10-17/lp-seeded-response-integration/owner-verify.py')

def hull_area(a):
 pts=set()
 for u,v in __import__('itertools').combinations(a,2):
  d=g.cross(u,v)
  if d:
   q=((v[1]-u[1])/d,(u[0]-v[0])/d)
   if all(g.dot(w,q)<=1 for w in a):pts.add(q)
 assert len(pts)==len(a)
 def chain(seq):
  out=[]
  for p in seq:
   while len(out)>=2 and g.cross((out[-1][0]-out[-2][0],out[-1][1]-out[-2][1]),(p[0]-out[-1][0],p[1]-out[-1][1]))<=0:out.pop()
   out.append(p)
  return out
 h=chain(sorted(pts))[:-1]+chain(sorted(pts,reverse=True))[:-1];assert len(h)==len(a)
 for u in a:assert sum(g.dot(u,p)==1 for p in h)==2
 cv=g.closures(a);av=[sum(v[i] for v in cv)/len(cv) for i in range(len(a))];g.feasible(a,av);assert min(av)>0
 return sum(g.cross(h[i],h[(i+1)%len(h)]) for i in range(len(h)))/2

def angle_order(a):
 def cmp(i,j):
  x,y=a[i],a[j];hx=int(x[1]<0 or (x[1]==0 and x[0]<0));hy=int(y[1]<0 or (y[1]==0 and y[0]<0))
  if hx!=hy:return hx-hy
  d=g.cross(x,y)
  return -1 if d>0 else 1 if d<0 else (i>j)-(i<j)
 return sorted(range(len(a)),key=functools.cmp_to_key(cmp))

def main():
 t=time.process_time();freeze=json.loads((D/'root-freeze.json').read_text())
 for name,digest in freeze.items():assert sha(D/name)==digest,name
 sample=json.loads((D/'sample.json').read_text());pred=json.loads((D/'predictions.json').read_text());results=json.loads((D/'results.json').read_text())
 hko=json.loads((P/'patterns/upper-tail/conditional-coupling/hko-exact.json').read_text());base=[[F(v['rational'])+F(v['sqrt5'])*F(2236068,1000000) for v in u] for u in hko['A']+hko['B']]
 assert len(sample['rows'])==len(pred['rows'])==12
 scored={x['name']:x for x in results['rows']};assert [x['name'] for x in results['rows']]==[x['name'] for x in sample['rows'][:len(scored)]]
 counts={'LPs':0,'trajectories':0,'strict_updates':0,'prefix_pairs':0};out=[]
 for index,(s,p) in enumerate(zip(sample['rows'],pred['rows'])):
  seed=index//2;delta=F(1,[1000,100][index%2]);raw=hashlib.sha256(('msc-math/thesis/facet-transfer/v1:'+str(seed)).encode()).digest()
  coords=[[base[j][k]+F(1,100)*F(raw[2*j+k]%9-4,4) for k in range(2)] for j in range(10)]
  expected=[]
  for a,which in [(coords[:5],seed%5),(coords[5:],(2*seed+1)%5)]:
   ids=angle_order(a);u,v=a[ids[which]],a[ids[(which+1)%5]];new=[(u[k]+v[k])/(2*(1-delta)) for k in range(2)]
   vertex=g.solve([u,v],[F(1),F(1)],2);assert vertex is not None and g.dot(new,vertex)>1
   expected.append(a+[new])
  a,b=[[list(map(F,u)) for u in s[k]] for k in ['A','B']];assert [a,b]==expected
  assert p['name']==s['name'] and p['A']==s['A'] and p['B']==s['B'] and p['status']=='OK'
  volume=hull_area(a)*hull_area(b);re=rp.replay(g,a,b,p['variants']);e=p['extraction'];assert len(re['candidates'])==e['candidate_count']==len(e['seeds'])
  for ac,cl in zip(re['candidates'],e['seeds']):
   assert (ac['shadow_sign'],ac['kind'],ac['index'])==(cl['shadow_sign'],cl['kind'],cl['index'])
   assert ac['alpha']==list(map(F,cl['terminal_alpha'])) and ac['gamma']==list(map(F,cl['terminal_gamma'])) and ac['Q']==F(cl['Q'])
   assert len(cl['states'])==2+ac['strict_updates'] and F(cl['states'][0]['Q'])==ac['one_response_Q']
   for (x,y,q),st in zip(ac['states'],cl['states'][1:]):assert x==list(map(F,st['alpha'])) and y==list(map(F,st['gamma'])) and q==F(st['Q'])
  assert re['best']['Q']==F(e['best']['Q']);counts['LPs']+=len(p['variants']);counts['trajectories']+=len(re['candidates']);counts['strict_updates']+=sum(z['strict_updates'] for z in re['candidates'])
  if s['name'] not in scored:continue
  row=scored[s['name']];ca,cb=g.closures(a),g.closures(b);truth=max(g.prefix_value(a,b,x,y) for x in ca for y in cb);counts['prefix_pairs']+=len(ca)*len(cb);qp=re['best']['Q'];cap=1/(2*truth);ratio=cap*cap/(2*volume)
  assert F(row['Q_true'])==truth and F(row['Q_predicted'])==qp and F(row['ratio'])==ratio and F(row['capacity_true'])==cap and F(row['volume'])==volume
  assert row['exact_recovery']==(truth==qp) and F(row['relative_capacity_error'])==truth/qp-1
  out.append({'name':s['name'],'ratio':str(ratio),'ratio_display':float(ratio),'exact':truth==qp,'capacity_error':str(truth/qp-1),'closure_counts':[len(ca),len(cb)]})
 receipt={'status':'PASS','predicted':12,'scored':len(out),'above_one':sum(F(x['ratio'])>1 for x in out),'recoveries':sum(x['exact'] for x in out),'counts':counts,'rows':out,'cpu_seconds':time.process_time()-t,'hashes':{k:sha(D/k) for k in ['PLAN.md','sample.json','predictions.json','results.json']},'checker_sha256':sha(Path(__file__)),'scope':'Independent frozen construction, genuine six-facet geometry, LP dual and deterministic response audit; capacity by all Gaussian closure pairs and independent A-prefix oracle.'}
 (D/'root-verification.json').write_text(json.dumps(receipt,indent=2)+'\n');print(json.dumps({k:v for k,v in receipt.items() if k!='rows'},indent=2))
if __name__=='__main__':main()
