from fractions import Fraction as F
from itertools import combinations
from pathlib import Path
import json,hashlib,sys
R=Path.cwd()

def dot(a,b):return sum(x*y for x,y in zip(a,b))
def cross(a,b):return a[0]*b[1]-a[1]*b[0]
def vertices(a):
 z=a[0][0]*0;n=len(a);out=[]
 for i,j in combinations(range(n),2):
  if cross(a[i],a[j])!=0:continue
  k=next((k for k in range(2) if a[i][k]!=a[j][k]),None)
  if k is None:continue
  x=a[j][k]/(a[j][k]-a[i][k]);y=1-x
  if x<=0 or y<=0:continue
  v=[z]*n;v[i]=x;v[j]=y;out.append(v)
 for i,j,k in combinations(range(n),3):
  nums=[cross(a[j],a[k]),cross(a[k],a[i]),cross(a[i],a[j])];den=sum(nums)
  if not den:continue
  w=[x/den for x in nums]
  if min(w)<=0:continue
  v=[z]*n
  for t,x in zip([i,j,k],w):v[t]=x
  out.append(v)
 for v in out:
  assert sum(v)==1 and min(v)>=0 and all(sum(v[i]*a[i][j] for i in range(n))==0 for j in range(2))
 assert out
 return out

def hk(a,b,word,alpha,gamma):
 v=[[*x,0,0] for x in a]+[[0,0,*x] for x in b];w=[x/2 for x in alpha+gamma]
 return sum(w[i]*w[j]*(v[i][0]*v[j][2]+v[i][1]*v[j][3]-v[i][2]*v[j][0]-v[i][3]*v[j][1]) for k,i in enumerate(word) for j in word[k+1:])

def recompute(row,parse):
 a=[[parse(x) for x in v] for v in row['A']];b=[[parse(x) for x in v] for v in row['B']];va=vertices(a);vb=vertices(b);n=len(a);m=len(b);z=a[0][0]*0;candidates=[];counts={};variants=[]
 for v in row['variants']:
  W=list(map(parse,v['W']));word=v['word'];pos={k:i for i,k in enumerate(word)}
  H=[[dot(a[i],b[j])*(1 if pos[i]<pos[n+j] else -1)/4 for j in range(m)] for i in range(n)]
  assert min(W)>=0 and sum(W)==1
  assert all(sum(W[i*m+j]*b[j][k] for j in range(m))==0 for i in range(n) for k in range(2))
  assert all(sum(W[i*m+j]*a[i][k] for i in range(n))==0 for j in range(m) for k in range(2))
  cur=[]
  for i in range(n):
   mass=sum(W[i*m:(i+1)*m])
   if not mass:continue
   gamma=[x/mass for x in W[i*m:(i+1)*m]];coeff=[dot(H[t],gamma) for t in range(n)];alpha=max(va,key=lambda x:dot(x,coeff));q=dot(alpha,coeff);assert hk(a,b,word,alpha,gamma)==q
   cur.append((q,alpha,gamma,word,'row',i,v['shadow_sign']))
  for j in range(m):
   mass=sum(W[i*m+j] for i in range(n))
   if not mass:continue
   alpha=[W[i*m+j]/mass for i in range(n)];coeff=[sum(alpha[i]*H[i][t] for i in range(n)) for t in range(m)];gamma=max(vb,key=lambda x:dot(x,coeff));q=dot(gamma,coeff);assert hk(a,b,word,alpha,gamma)==q
   cur.append((q,alpha,gamma,word,'column',j,v['shadow_sign']))
  candidates+=cur;counts[str(v['shadow_sign'])]=len(cur)
 best=max(candidates,key=lambda x:x[0]);assert best[0]>0
 return {'Q':best[0],'C':1/(2*best[0]),'best':best,'counts':counts,'vertices':[len(va),len(vb)],'all':candidates}


def check_reported(source,reported,parse):
 rebuilt=recompute(source,parse);a=[[parse(x) for x in v] for v in source['A']];b=[[parse(x) for x in v] for v in source['B']];n=len(a);m=len(b)
 expected={(x[6],x[4],x[5]):x for x in rebuilt['all']};seen=set()
 for c in reported['candidates']:
  key=(c['shadow_sign'],c['kind'],c['index']);assert key in expected and key not in seen;seen.add(key);e=expected[key]
  alpha=list(map(parse,c['alpha']));gamma=list(map(parse,c['gamma']));q=parse(c['Q']);assert q==e[0]
  assert min(alpha+gamma)>=0 and sum(alpha)==sum(gamma)==1
  assert all(sum(alpha[i]*a[i][k] for i in range(n))==0 and sum(gamma[j]*b[j][k] for j in range(m))==0 for k in range(2))
  assert c['word']==e[3] and hk(a,b,c['word'],alpha,gamma)==q
  if c['kind']=='row':assert gamma==e[2]
  else:assert alpha==e[1]
  vv=next(x for x in source['variants'] if x['shadow_sign']==c['shadow_sign']);w=list(map(parse,vv['W']));mass=sum(w[c['index']*m:(c['index']+1)*m]) if c['kind']=='row' else sum(w[i*m+c['index']] for i in range(n))
  assert parse(c['condition_mass'])==mass
 assert seen==set(expected) and reported['candidate_count']==len(seen) and reported['closure_vertex_counts']==rebuilt['vertices']
 assert parse(reported['best']['Q'])==rebuilt['Q'] and parse(reported['capacity'])==rebuilt['C']
 return rebuilt

def dual_check(row):
 a=[[F(x) for x in v] for v in row['A']];b=[[F(x) for x in v] for v in row['B']];n=len(a);m=len(b)
 for v in row['variants']:
  word=v['word'];pos={k:i for i,k in enumerate(word)};obj=[dot(a[i],b[j])*(1 if pos[i]<pos[n+j] else -1)/4 for i in range(n) for j in range(m)]
  A=[[F(1)]*(n*m)]
  for i in range(n):
   for k in range(2):A.append([b[j][k] if ii==i else F(0) for ii in range(n) for j in range(m)])
  for j in range(m):
   for k in range(2):A.append([a[i][k] if jj==j else F(0) for i in range(n) for jj in range(m)])
  y=list(map(F,v['dual']));w=list(map(F,v['W']));sl=[sum(A[t][j]*y[t] for t in range(len(A)))-obj[j] for j in range(n*m)]
  assert sl==list(map(F,v['dual_slack'])) and min(sl)>=0 and sum(obj[j]*w[j] for j in range(n*m))==F(v['Q'])==y[0]

if __name__=='__main__':
 import argparse
 p=argparse.ArgumentParser();p.add_argument('--repo',type=Path,default=R);p.add_argument('--result',type=Path,required=True);p.add_argument('--out',type=Path,required=True);args=p.parse_args();R=args.repo
 sys.path.insert(0,str(R/'patterns/upper-tail/conditional-coupling'));from quadratic import Q5
 source_path=R/'patterns/upper-tail/conditional-coupling/results.json';hko_path=R/'patterns/upper-tail/conditional-coupling/hko-exact.json'
 data=json.load(open(source_path));hko=json.load(open(hko_path));new=json.load(open(args.result));out=[];ranks=[];candidate_count=0
 for name,sha in new['sources'].items():assert hashlib.sha256((R/name).read_bytes()).hexdigest()==sha
 for source,r in zip(data['rows'],new['rows']):
  assert source['name']==r['name'];dual_check(source);x=check_reported(source,r['rounding'],F);candidate_count+=len(x['all']);old=F(source['feasible_capacity_upper']);lower=F(source['capacity_lower']);assert x['C']>=lower
  assert F(r['combined_capacity'])==min(old,x['C']) and F(r['capacity_lower'])==lower and F(r['incumbent_capacity'])==old
  assert r['exact_capacity']==(min(old,x['C'])==lower)
  out.append({'name':source['name'],'Q':str(x['Q']),'capacity':str(x['C']),'prior_upper':str(old),'lower':str(lower),'exact_capacity':x['C']==lower,'cut':float(1-x['C']/old),'counts':x['counts'],'closure_vertices':x['vertices']})
  n=len(source['A']);m=len(source['B'])
  for v in source['variants']:
   w=list(map(F,v['W']));a=[sum(w[i*m:(i+1)*m]) for i in range(n)];b=[sum(w[i*m+j] for i in range(n)) for j in range(m)];rankone=all(w[i*m+j]==a[i]*b[j] for i in range(n) for j in range(m));ranks.append({'name':source['name'],'sign':v['shadow_sign'],'rank_one':rankone})
 # The original exact LP certificates were independently checked in the previous packet;
 # this rechecks all rational primal/dual data and checks all new algebraic candidates.
 x=check_reported(hko,new['hko'],Q5.parse);assert x['Q']==Q5(1)/10;candidate_count+=len(x['all'])
 # Strict 5x6 relaxation obstruction: only max-sign LP matters; a maximizing product
 # may choose an A closure vertex. Its common tight B columns cannot close.
 r=data['rows'][1];a=[[F(x) for x in v] for v in r['A']];b=[[F(x) for x in v] for v in r['B']];v=max(r['variants'],key=lambda v:F(v['Q']));assert F(v['Q'])>min(F(t['Q']) for t in r['variants'])
 sl=list(map(F,v['dual_slack']));neighbors=[{j for j in range(len(b)) if sl[i*len(b)+j]==0} for i in range(len(a))];graph=[]
 for alpha in vertices(a):
  ids=[i for i,t in enumerate(alpha) if t];common=sorted(set.intersection(*(neighbors[i] for i in ids)));assert len(common)<=2
  if len(common)==2:assert cross(b[common[0]],b[common[1]])!=0
  if len(common)==1:assert b[common[0]]!=[0,0]
  graph.append({'A_support':ids,'common_tight_B':common,'pair_determinant':str(cross(b[common[0]],b[common[1]])) if len(common)==2 else None})
 result={'status':'PASS','rational_candidates':candidate_count-len(x['all']),'algebraic_candidates':len(x['all']),'direct4D_checks':candidate_count,'rational_LP_duals':6,'rows':out,'coupling_factorizations':ranks,'HKO_Q':x['Q'].serial(),'HKO_capacity':x['C'].serial(),'strict_gap_5x6':{'max_shadow_sign':v['shadow_sign'],'tight_graph':graph},'inputs_sha256':{str(p):hashlib.sha256(p.read_bytes()).hexdigest() for p in [source_path,hko_path,args.result,Path(__file__)]}}
 args.out.write_text(json.dumps(result,indent=2)+'\n');print(json.dumps({k:result[k] for k in ['status','rational_candidates','algebraic_candidates','direct4D_checks','rational_LP_duals']},indent=2))
