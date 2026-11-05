#!/usr/bin/env python3
"""Pure rational polynomial certificate for one fixed s0 continuation interval."""
import argparse,hashlib,importlib.util,json,math,time
from fractions import Fraction as F
from itertools import combinations
from pathlib import Path

def p(x):return [F(x)]
def trim(a):
 while len(a)>1 and not a[-1]:a.pop()
 return a
def add(a,b):return trim([(a[i] if i<len(a) else 0)+(b[i] if i<len(b) else 0) for i in range(max(len(a),len(b)))])
def neg(a):return [-x for x in a]
def sub(a,b):return add(a,neg(b))
def mul(a,b):
 z=[F(0)]*(len(a)+len(b)-1)
 for i,x in enumerate(a):
  for j,y in enumerate(b):z[i+j]+=x*y
 return trim(z)
def scale(a,c):return trim([x*c for x in a])
def dot(a,b):return add(mul(a[0],b[0]),mul(a[1],b[1]))
def det(a,b):return sub(mul(a[0],b[1]),mul(a[1],b[0]))
def ev(a,t):
 z=F(0)
 for x in reversed(a):z=z*t+x
 return z
def ser(x):
 if isinstance(x,F):return str(x)
 if isinstance(x,dict):return {k:ser(v) for k,v in x.items()}
 if isinstance(x,(list,tuple)):return [ser(v) for v in x]
 return x
def sha(path):return hashlib.sha256(path.read_bytes()).hexdigest()
def bernstein(a,lo,hi):
 # Power coefficients of a(lo+(hi-lo)*u), then degree-n Bernstein coefficients.
 n=len(a)-1;c=[sum(a[j]*math.comb(j,k)*lo**(j-k)*(hi-lo)**k for j in range(k,n+1)) for k in range(n+1)]
 return [sum(c[k]*F(math.comb(i,k),math.comb(n,k)) for k in range(i+1)) for i in range(n+1)]
def positive(a,lo,hi,depth=0):
 bs=bernstein(a,lo,hi)
 if min(bs)>0:return [dict(interval=[lo,hi],bernstein=bs,lower=min(bs))]
 assert depth<16,('unproved strict sign',lo,hi,bs)
 mid=(lo+hi)/2
 return positive(a,lo,mid,depth+1)+positive(a,mid,hi,depth+1)
def vertex(fac,sup):
 i,j,k=sup;nums=[p(0) for _ in fac];nums[i]=det(fac[j],fac[k]);nums[j]=det(fac[k],fac[i]);nums[k]=det(fac[i],fac[j]);den=add(add(nums[i],nums[j]),nums[k]);sgn=1 if ev(den,F(1,100))>0 else -1
 return [scale(x,sgn) for x in nums],scale(den,sgn)
def score(a,b,w,alpha,gamma):
 an,ad=alpha;bn,bd=gamma;pos={v:k for k,v in enumerate(w)};num=p(0)
 for i in range(5):
  for j in range(5):num=add(num,scale(mul(mul(an[i],bn[j]),dot(a[i],b[j])),F(1 if pos[i]<pos[5+j] else -1,4)))
 return num,mul(ad,bd)
def difference(x,y):return sub(mul(x[0],y[1]),mul(y[0],x[1])),mul(x[1],y[1])

def main():
 ap=argparse.ArgumentParser();ap.add_argument('--repo',type=Path,required=True);ap.add_argument('--out',type=Path,required=True);o=ap.parse_args();start=time.process_time();lo=F(1,100);hi=F(1,20)
 paths=['patterns/upper-tail/lp-seeded-response/sample.json','patterns/upper-tail/lp-seeded-response/predictions.json','patterns/upper-tail/lp-seeded-response/results.json','patterns/upper-tail/lp-seeded-response/freeze.py','patterns/upper-tail/seed-coverage/REPORT.md','patterns/upper-tail/seed-coverage/verification.json','patterns/upper-tail/near-hko-transfer/verify.py','patterns/upper-tail/local-completion/REPORT.md','topics/products/product-qp-six-facet-reduction.tex','patterns/upper-tail/mixed-area-order/REPORT.md']
 source_hashes={x:sha(o.repo/x) for x in paths};pred=json.loads((o.repo/paths[1]).read_text())['rows'];s=pred[0];end=pred[1];assert s['name']=='lpseed_near_hko_s0_e100' and end['name']=='lpseed_near_hko_s0_e20'
 sp=importlib.util.spec_from_file_location('independent',o.repo/paths[6]);m=importlib.util.module_from_spec(sp);sp.loader.exec_module(m)
 a=[];b=[]
 for key,target in [('A',a),('B',b)]:
  for x,y in zip(s[key],end[key]):
   target.append([[F(xx)-lo*(F(yy)-F(xx))/(hi-lo),(F(yy)-F(xx))/(hi-lo)] for xx,yy in zip(x,y)])
 # Independently bind the whole line to the prior freeze's base and digest direction.
 h=json.loads((o.repo/'patterns/upper-tail/conditional-coupling/hko-exact.json').read_text());raw=hashlib.sha256(b'msc-math/thesis/lp-seeded-response/validation/v1:0').digest();coords=[x for fac in [a,b] for v in fac for x in v];base=[F(v['rational'])+F(2236068,1000000)*F(v['sqrt5']) for key in ['A','B'] for u in h[key] for v in u]
 assert all(x==[z,F(byte%9-4,4)] for x,z,byte in zip(coords,base,raw));source_hashes['patterns/upper-tail/conditional-coupling/hko-exact.json']=sha(o.repo/'patterns/upper-tail/conditional-coupling/hko-exact.json')
 aa=[[ev(v,lo) for v in row] for row in a];bb=[[ev(v,lo) for v in row] for row in b];ca0=m.vertices(aa);cb0=m.vertices(bb);supportsA=[m.support(x) for x in ca0];supportsB=[m.support(x) for x in cb0];ca=[vertex(a,z) for z in supportsA];cb=[vertex(b,z) for z in supportsB];target=(supportsA.index([0,2,4]),supportsB.index([0,1,3]));competitor=(1,3)
 # Every vertex-support sign and every infeasible triple remains unchanged.
 certificates={};polynomials={}
 def certify(name,poly):
  polynomials[name]=poly;certificates[name]=positive(poly,lo,hi)
 for key,fac,vs in [('A',a,ca),('B',b,cb)]:
  for j,(ns,de) in enumerate(vs):
   certify(f'{key}_vertex{j}_denominator',de)
   for i,n in enumerate(ns):
    if n!=[0]:certify(f'{key}_vertex{j}_weight{i}',n)
  for i,j in combinations(range(5),2):
   pp=det(fac[i],fac[j]);sgn=1 if ev(pp,lo)>0 else -1;certify(f'{key}_pairdet{i}{j}',scale(pp,sgn))
  for sup in combinations(range(5),3):
   ns,de=vertex(fac,sup);certify(f'{key}_triple{str(sup)}_denominator',de)
   if list(sup) not in (supportsA if key=='A' else supportsB):
    bad=next(i for i in sup if ev(ns[i],lo)<0);certify(f'{key}_triple{str(sup)}_negativeweight{bad}',neg(ns[bad]))
 # All shadow rays retain their circular order and chosen linear cut.
 for sign in [1,-1]:
  vec=a+[[scale(v[1],-sign),scale(v[0],sign)] for v in b]
  for i,j in combinations(range(10),2):
   pp=det(vec[i],vec[j]);sgn=1 if ev(pp,lo)>0 else -1;certify(f'shadow{sign}_raydet{i}_{j}',scale(pp,sgn))
  for i,v in enumerate(vec):
   pp=v[1];sgn=1 if ev(pp,lo)>0 else -1;certify(f'shadow{sign}_cut{i}',scale(pp,sgn))
 negword=next(v['word'] for v in s['variants'] if v['shadow_sign']==-1);qt=score(a,b,negword,ca[target[0]],cb[target[1]]);qc=score(a,b,negword,ca[competitor[0]],cb[competitor[1]]);gap=difference(qc,qt);certify('competitor_minus_target_numerator',gap[0]);certify('competitor_minus_target_denominator',gap[1])
 positiveword=next(v['word'] for v in s['variants'] if v['shadow_sign']==1);qp=score(a,b,positiveword,ca[target[0]],cb[target[1]]);crossgap=difference(qc,qp);certify('competitor_minus_positive_target_numerator',crossgap[0]);certify('competitor_minus_positive_target_denominator',crossgap[1])
 for i in range(5):
  if i!=target[0]:
   g=difference(qt,score(a,b,negword,ca[i],cb[target[1]]));certify(f'target_A_responsegap{i}',g[0])
 for j in range(5):
  if j!=target[1]:
   g=difference(qt,score(a,b,negword,ca[target[0]],cb[j]));certify(f'target_B_responsegap{j}',g[0])
 # Direct, independent HK endpoint equality binds polynomial expansion and labels.
 endpoint=[]
 for t in [lo,hi]:
  af=[[ev(v,t) for v in row] for row in a];bf=[[ev(v,t) for v in row] for row in b];av=m.vertices(af);bv=m.vertices(bf);assert [m.support(z) for z in av]==supportsA and [m.support(z) for z in bv]==supportsB;assert m.word(af,bf,-1)==negword
  assert m.area(af)>0 and m.area(bf)>0
  source=s if t==lo else end
  for variant in source['variants']:m.lp(af,bf,variant)
  vals=[]
  for pair,q in [(target,qt),(competitor,qc)]:
   exact=m.q(af,bf,negword,av[pair[0]],bv[pair[1]]);assert exact==ev(q[0],t)/ev(q[1],t);vals.append(exact)
  positive_exact=m.q(af,bf,positiveword,av[target[0]],bv[target[1]]);assert positive_exact==ev(qp[0],t)/ev(qp[1],t)
  endpoint.append(dict(t=t,target_Q=vals[0],positive_target_Q=positive_exact,competitor_Q=vals[1],gap=vals[1]-vals[0],gap_to_positive_target=vals[1]-positive_exact,existing_exact_LP_certificates_replayed=2))
 out=dict(status='PASS',claim='On the entire fixed radial interval [1/100,1/20], negative-word A024/B013 remains a strict coordinate basin but is strictly dominated by feasible A023/B124, in both angular words, hence in every HK word by the imported two-shadow theorem; it cannot become globally winning regardless of seed capture.',parameter_interval=[lo,hi],A=a,B=b,A_supports=supportsA,B_supports=supportsB,negative_word=negword,target_pair=target,competitor_pair=competitor,competitor_supports=[supportsA[competitor[0]],supportsB[competitor[1]]],target_score=qt,positive_target_score=qp,positive_word=positiveword,competitor_score=qc,polynomials=polynomials,certificates=certificates,endpoints=endpoint,source_hashes=source_hashes,check_sha256=sha(Path(__file__)),cpu_seconds=time.process_time()-start)
 o.out.parent.mkdir(parents=True,exist_ok=True);o.out.write_text(json.dumps(ser(out),indent=2)+'\n');print('PASS',len(certificates),'strict polynomial signs; CPU',out['cpu_seconds']);print('endpoints',[(float(z['gap'])) for z in endpoint])
if __name__=='__main__':main()
