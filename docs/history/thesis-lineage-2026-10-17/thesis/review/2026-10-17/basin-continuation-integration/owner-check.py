#!/usr/bin/env python3
"""Independent exact polynomial/HK audit of the selected radial continuation."""
import argparse,hashlib,importlib.util,json,math,time
from fractions import Fraction as F
from itertools import combinations
from pathlib import Path

def trim(a):
 a=list(a)
 while len(a)>1 and a[-1]==0:a.pop()
 return a

def add(a,b):return trim([(a[i] if i<len(a) else F(0))+(b[i] if i<len(b) else F(0)) for i in range(max(len(a),len(b)))])
def scale(a,c):return trim([c*x for x in a])
def sub(a,b):return add(a,scale(b,-1))
def mul(a,b):
 c=[F(0)]*(len(a)+len(b)-1)
 for i,x in enumerate(a):
  for j,y in enumerate(b):c[i+j]+=x*y
 return trim(c)
def peval(a,t):
 z=F(0)
 for x in reversed(a):z=z*t+x
 return z

def bernstein(p,l,h):
 n=len(p)-1;u=[F(0)]*(n+1)
 for k,x in enumerate(p):
  for j in range(k+1):u[j]+=x*math.comb(k,j)*l**(k-j)*(h-l)**j
 return [sum(u[k]*F(math.comb(j,k),math.comb(n,k)) for k in range(j+1)) for j in range(n+1)]
def signcert(p,l,h):
 b=bernstein(p,l,h)
 if min(b)>0:return 1,b
 if max(b)<0:return -1,b
 raise AssertionError(('nonuniform Bernstein sign',list(map(str,p)),list(map(str,b))))

def det(a,b):return sub(mul(a[0],b[1]),mul(a[1],b[0]))
def dot(a,b):return add(mul(a[0],b[0]),mul(a[1],b[1]))
def closure(fac,ids,l,h):
 i,j,k=ids;raw=[det(fac[j],fac[k]),det(fac[k],fac[i]),det(fac[i],fac[j])];den=add(add(raw[0],raw[1]),raw[2]);ds,db=signcert(den,l,h);cert=[]
 for x in raw:
  s,b=signcert(x,l,h);assert s==ds;cert.append(b)
 weights=[[F(0)] for _ in fac]
 for i,x in zip(ids,raw):weights[i]=x
 return weights,den,dict(support=ids,denominator=den,denominator_sign=ds,denominator_bernstein=db,raw_weight_bernstein=cert)
def qraw(a,b,w,wa,wb):
 pos={v:i for i,v in enumerate(w)};z=[F(0)]
 for i,x in enumerate(a):
  for j,y in enumerate(b):z=add(z,scale(mul(mul(wa[i],wb[j]),dot(x,y)),F(1 if pos[i]<pos[len(a)+j] else -1,4)))
 return z

def ser(x):
 if isinstance(x,F):return str(x)
 if isinstance(x,dict):return {k:ser(v) for k,v in x.items()}
 if isinstance(x,(tuple,list)):return [ser(v) for v in x]
 return x

def main():
 p=argparse.ArgumentParser();p.add_argument('--patterns',type=Path,required=True);p.add_argument('--out',type=Path,required=True);o=p.parse_args();r=o.patterns;ct=time.process_time();l=F(1,100);h=F(1,20)
 hp=r/'patterns/upper-tail/conditional-coupling/hko-exact.json';hko=json.loads(hp.read_text());base={k:[[F(v['rational'])+F(v['sqrt5'])*F(2236068,1000000) for v in row] for row in hko[k]] for k in ['A','B']};raw=hashlib.sha256(b'msc-math/thesis/lp-seeded-response/validation/v1:0').digest();direction=[F(v%9-4,4) for v in raw[:20]];fac={k:[[ [base[k][i][j],direction[(0 if k=='A' else 10)+2*i+j]] for j in range(2)] for i in range(5)] for k in ['A','B']};a=fac['A'];b=fac['B']
 sp=importlib.util.spec_from_file_location('root_prior_independent',r/'patterns/upper-tail/near-hko-transfer/verify.py');m=importlib.util.module_from_spec(sp);sp.loader.exec_module(m)
 def at(f,t):return [[peval(x,t) for x in row] for row in f]
 pred=json.loads((r/'patterns/upper-tail/lp-seeded-response/predictions.json').read_text());src=pred['rows'][0];assert src['name']=='lpseed_near_hko_s0_e100'
 assert at(a,l)==[[F(x) for x in row] for row in src['A']];assert at(b,l)==[[F(x) for x in row] for row in src['B']]
 # Certify both angular orders unchanged by a strict sign certificate for every
 # determinant, plus every y coordinate where nonzero. Fixed negative reversal.
 wordcert={};words={}
 for sg in [1,-1]:
  vv=a+[[scale(u[1],-sg),scale(u[0],sg)] for u in b];cert=[]
  for i in range(10):
   ys,yb=signcert(vv[i][1],l,h)
   for j in range(i+1,10):
    ds,db=signcert(det(vv[i],vv[j]),l,h);cert.append(dict(i=i,j=j,sign=ds,bernstein=db))
  word=m.word(at(a,l),at(b,l),sg);assert word==m.word(at(a,h),at(b,h),sg);words[sg]=word;wordcert[str(sg)]=cert
 # Competitor chosen from inherited exact global witness at t=1/100.
 truth=json.loads((r/'patterns/upper-tail/lp-seeded-response/results.json').read_text());win=truth['rows'][0]['winners'][0];ci=[i for i,v in enumerate(win['alpha']) if F(v)];cj=[i for i,v in enumerate(win['gamma']) if F(v)];comp_sign=win['shadow_sign'];assert (ci,cj,comp_sign)==([0,2,3],[1,2,4],-1)
 twa,tad,tcA=closure(a,[0,2,4],l,h);twb,tbd,tcB=closure(b,[0,1,3],l,h);cwa,cad,ccA=closure(a,ci,l,h);cwb,cbd,ccB=closure(b,cj,l,h)
 cn=qraw(a,b,words[comp_sign],cwa,cwb);cd=mul(cad,cbd);tns={sg:qraw(a,b,words[sg],twa,twb) for sg in [1,-1]};td=mul(tad,tbd);gaps={}
 for sg,tn in tns.items():
  gn=sub(mul(cn,td),mul(tn,cd));gd=mul(cd,td);ns,nb=signcert(gn,l,h);ds,db=signcert(gd,l,h);assert ns==ds
  values=[]
  for t in [l,(l+h)/2,h]:
   aa=at(a,t);bb=at(b,t);alpha=[peval(x,t)/peval(tad,t) for x in twa];gamma=[peval(x,t)/peval(tbd,t) for x in twb];ca=[peval(x,t)/peval(cad,t) for x in cwa];cb=[peval(x,t)/peval(cbd,t) for x in cwb];qt=m.q(aa,bb,words[sg],alpha,gamma);qc=m.q(aa,bb,words[comp_sign],ca,cb);assert qt==peval(tn,t)/peval(td,t);assert qc==peval(cn,t)/peval(cd,t);assert qc-qt==peval(gn,t)/peval(gd,t)>0;values.append(dict(t=t,competitor=qc,target=qt,gap=qc-qt))
  gaps[str(sg)]=dict(numerator=gn,denominator=gd,numerator_bernstein=nb,denominator_bernstein=db,direct_HK_checks=values)
 # Verify target remains strict coordinate maximum for the negative word.
 geometry=[];response_gaps=[]
 for name,ff,tw,other,tdfactor,odfactor in [('A',a,twa,twb,tad,tbd),('B',b,twb,twa,tbd,tad)]:
  for i,j,k in combinations(range(5),3):
   dif=lambda x,y:[sub(u,v) for u,v in zip(x,y)]
   sg,bb=signcert(det(dif(ff[i],ff[k]),dif(ff[j],ff[k])),l,h)
   geometry.append(dict(factor=name,triple=[i,j,k],sign=sg,bernstein=bb))
  vs=m.vertices(at(ff,l));assert len(vs)==5
  for vv in vs:
   ids=[i for i,x in enumerate(vv) if x]
   if ids==([0,2,4] if name=='A' else [0,1,3]):continue
   vw,vd,vc=closure(ff,ids,l,h)
   vn=qraw(a,b,words[-1],vw,other) if name=='A' else qraw(a,b,words[-1],other,vw)
   # Compare target numerator/tdfactor against competitor numerator/vd;
   # the shared opposite-factor denominator is retained explicitly.
   gn=sub(mul(tns[-1],vd),mul(vn,tdfactor));gd=mul(mul(tdfactor,vd),odfactor)
   ns,nb=signcert(gn,l,h);ds,db=signcert(gd,l,h);assert ns==ds
   response_gaps.append(dict(factor=name,alternative_support=ids,numerator=gn,denominator=gd,numerator_bernstein=nb,denominator_bernstein=db))
 assert len(response_gaps)==8
 out=dict(status='PASS',geometry_affine_signs=geometry,strict_coordinate_response_gaps=response_gaps,interval=[l,h],direction_digest=raw.hex(),source_supports=dict(targetA=[0,2,4],targetB=[0,1,3],competitorA=ci,competitorB=cj),closure_certificates=[tcA,tcB,ccA,ccB],words=words,angular_determinant_certificates=wordcert,competitor_sign=comp_sign,gaps=gaps,scope='Strict domination of both target shadow values throughout closed interval; no LP/seed-path or extractor-exactness continuation claimed.',cpu_seconds=time.process_time()-ct,inputs={str(x.relative_to(r)):hashlib.sha256(x.read_bytes()).hexdigest() for x in [hp,r/'patterns/upper-tail/lp-seeded-response/predictions.json',r/'patterns/upper-tail/lp-seeded-response/results.json',r/'patterns/upper-tail/near-hko-transfer/verify.py',r/'patterns/upper-tail/mixed-area-order/REPORT.md']})
 out['check_sha256']=hashlib.sha256(Path(__file__).read_bytes()).hexdigest()
 o.out.parent.mkdir(parents=True,exist_ok=True);o.out.write_text(json.dumps(ser(out),indent=2)+'\n');print('PASS whole interval both target shadows dominated; CPU',out['cpu_seconds'])
if __name__=='__main__':main()
