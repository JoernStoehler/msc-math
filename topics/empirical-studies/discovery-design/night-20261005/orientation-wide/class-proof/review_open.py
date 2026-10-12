"""Independent audit of every open gap sign certificate and minimum implication."""
from review_signs import *
import hashlib

def exactquotient(p,q):
 quotient,remainder=divrem(p,q);assert remainder==[z];return quotient

def oddpart(p):
 """Parity via successive multiplicity layers, separate from producer Yun loop."""
 odd=[one];p=monic(p)
 while degree(p)>0:
  repeated=gcd(p,derivative(p));layer=exactquotient(p,repeated)
  overlap=gcd(odd,layer)
  odd=exactquotient(mul(odd,layer),mul(overlap,overlap));p=repeated
 return monic(odd)

def main():
 start=time.perf_counter();out=json.loads((root/'open-proof.json').read_text());clip=json.loads((root/'clipped.json').read_text());form=json.loads((root/'formulas.json').read_text())
 assert out['clipped_sha256']==hashlib.sha256((root/'clipped.json').read_bytes()).hexdigest()
 accepted=[];passed=0
 for qi,query in enumerate(out['queries']):
  p=list(map(m.decd,query['polynomial']));left,right=decv(query['left']),decv(query['right']);cert=query['certificate']
  if p==[z]:assert cert=={'passed':True,'identically_zero':True};accepted.append((p,left,right,True,None));passed+=1;continue
  odd=oddpart(p);assert odd==list(map(m.decd,cert['odd_part']))
  count=count_open(odd,left,right);assert count==cert['odd_root_query']['count']
  # Independently verify original reported sequence via its exact remainder identities.
  chain=[list(map(m.decd,c)) for c in cert['odd_root_query']['chain']]
  assert chain==sturm(exactquotient(odd,gcd(odd,derivative(odd))))
  target=cert['target'];assert target in (-1,1)
  probe=decv(cert['probe']);assert compare(left,probe)<0 and compare(probe,right)<0
  ps=evaluate(p,probe).sign()*target;ends=[evaluate(p,left).sign()*target,evaluate(p,right).sign()*target]
  assert ps==cert['probe_sign'] and ends==cert['endpoint_signs']
  ok=count==0 and ps>0 and min(ends)>=0;assert ok==cert['passed'];passed+=ok
  accepted.append((p,left,right,ok,target))
 def referenced(ref):return [z] if ref.get('zero') else scale(list(map(m.decd,clip['polynomials'][int(ref['key'])]['coefficients'])),m.decd(ref['multiplier']))
 def checkquery(qi,p,left,right,target):
  qp,ql,qr,passed,qt=accepted[qi];assert qp==p and ql==left and qr==right and passed
  assert p==[z] or qt==target
 modes={}
 for result in out['segments']:
  t=clip['open_segments'][result['segment']];left,right=decv(t['left']),decv(t['right']);u=decv(t['sample_u']);q=list(map(lambda p:list(map(m.decd,p)),form['formulas'][t['formula']]['gap']))
  assert len(t['checks'])==len(result['endpoint_checks'])
  for check,proof in zip(t['checks'],result['endpoint_checks']):
   assert proof['label']==check['label']
   if 'query' in proof:checkquery(proof['query'],referenced(check['gap']),left,right,1)
   else:
    assert proof['mode']=='zero-edge strict primary bound <=1'
    label=check['bound']['label'];assert label[0]=='residual' and label[1]>=5
    c,a=map(lambda p:list(map(m.decd,p)),form['formulas'][t['formula']]['residuals'][label[1]])
    n,d=map(lambda p:list(map(m.decd,p)),(check['bound']['numerator'],check['bound']['denominator']))
    assert add(mul(c,d),mul(a,n))==[z]
  mode=result['minimum_mode'];modes[mode]=modes.get(mode,0)+1
  if mode=='singleton endpoint':assert t['singleton']
  elif mode=='concave/linear':checkquery(result['minimum_query'],referenced(t['q2']),left,right,-1)
  elif mode.startswith('endpoint derivative '):
   label=mode.rsplit(' ',1)[1];check=next(c for c in t['checks'] if c['label']==label)
   d=list(map(m.decd,check['bound']['denominator']));dsign=evaluate(d,u).sign();assert dsign!=0
   assert count_open(d,left,right)==0
   target=dsign if label=='lower' else -dsign
   checkquery(result['minimum_query'],referenced(check['station_membership']),left,right,target)
  elif mode=='stationary numerator nonnegative':checkquery(result['minimum_query'],referenced(t['stationary_gap']),left,right,1)
  else:assert False,mode
  assert result['passed']
 assert not out['failed_segments'] and len(out['segments'])==len(clip['open_segments'])
 assert all(t['upper'] is not None for t in clip['open_segments'])
 print(json.dumps({'status':'PASS independent open gap certificates and minimum implications','queries':len(accepted),'passed_queries':passed,'segments':len(out['segments']),'minimum_modes':modes,'elapsed_seconds':round(time.perf_counter()-start,3),'writes':False}))
if __name__=='__main__':main()
