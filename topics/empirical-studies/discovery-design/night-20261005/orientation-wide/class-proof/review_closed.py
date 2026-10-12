"""Independent exhaustive original/internal closed-fiber arithmetic audit."""
from review_signs import *
from itertools import combinations
import hashlib

def main():
 start=time.perf_counter();form=json.loads((root/'formulas.json').read_text());joint=json.loads((root.parent/'joint-denominators'/'joint-denominators.json').read_text());events=json.loads((root/'feasibility-events.json').read_text());out=json.loads((root/'closed-proof.json').read_text())
 assert out['formula_sha256']==hashlib.sha256((root/'formulas.json').read_bytes()).hexdigest()
 assert out['feasibility_events_sha256']==hashlib.sha256((root/'feasibility-events.json').read_bytes()).hexdigest()
 lookup={(f['vector'],tuple(f['fit_triple'])):i for i,f in enumerate(form['formulas'])}
 observations={(json.dumps(c['owner'],sort_keys=True),c['formula']):c for c in out['checks']};matched=set();empty=0;equalitycount=0;tailcount=0
 def verify(fi,u,lo,hi,owner):
  nonlocal empty,equalitycount,tailcount
  f=form['formulas'][fi];res=[(list(map(m.decd,c)),list(map(m.decd,a))) for c,a in f['residuals']]
  for c,a in res:
   cv,av=evaluate(c,u),evaluate(a,u)
   if av.sign()==0:
    if cv.sign()<0:empty+=1;return
   else:
    at=-cv/av
    if av.sign()>0 and (at-lo).sign()>0:lo=at
    if av.sign()<0 and (hi is None or (at-hi).sign()<0):hi=at
   if hi is not None and (hi-lo).sign()<0:empty+=1;return
  if hi is not None and hi.sign()<=0:empty+=1;return
  key=(json.dumps(owner,sort_keys=True),fi);record=observations[key];matched.add(key)
  assert decv(record['u'])==u and decv(record['lower'])==lo
  assert (record['upper'] is None)==(hi is None)
  if hi is not None:assert decv(record['upper'])==hi
  q=[evaluate(list(map(m.decd,p)),u) for p in f['gap']]
  assert q==list(map(decv,record['q']))
  assert record['identically_zero_gap']==all(v.sign()==0 for v in q)
  tests=[('lower',lo)]+([] if hi is None else [('upper',hi)])
  if q[2].sign()>0:
   at=-q[1]/(2*q[2])
   if (at-lo).sign()>=0 and (hi is None or (hi-at).sign()>=0):tests.append(('stationary',at))
  if hi is None:tailcount+=1;assert q[2].sign()>0 or (q[2].sign()==0 and q[1].sign()>=0)
  if lo.sign()==0:assert q[0].sign()>=0
  tests=[(label,x) for label,x in tests if x.sign()!=0]
  assert len(tests)==len(record['tests'])
  for (label,x),test in zip(tests,record['tests']):
   value=q[0]+q[1]*x+q[2]*x*x
   assert label==test['label'] and x==decv(test['x']) and value==decv(test['gap'])
   assert value.sign()==test['sign'] and value.sign()>=0
   edges=[(evaluate(c,u)+evaluate(a,u)*x).sign() for c,a in res[5:]]
   assert edges==test['edge_signs']
   if value.sign()==0:equalitycount+=1;assert all(s>0 for s in edges)
 for domain in form['closed_domains']:
  fi,ci=domain['incidence'];fiber=joint['joint']['closed_fibers'][fi];cell=fiber['cells'][ci];u=decv(fiber['u']);vi=domain['vectors'][0]
  for l,rv in enumerate(cell['vector']):
   actual=list(map(m.decd,joint['circuits'][l]['candidates'][rv['candidate_ids'][0]]['coefficients']));p=list(map(m.decd,form['vectors'][vi][l]))
   assert (V(p[0]-actual[0])+u*(p[2]-actual[2])).sign()==0
   assert (V(p[1]-actual[1])+u*(p[3]-actual[3])).sign()==0
  lo=decv(cell['left']);hi=None if cell['right'] is None else decv(cell['right'])
  for ids in combinations(range(5),3):verify(lookup[vi,ids],u,lo,hi,['original',fi,ci])
 for entry in events['event_domains']:
  di,fi=entry['domain'],entry['formula'];si,ti,ci=form['open_domains'][di]['incidence'];strip=joint['joint']['topology_strips'][si];cell=strip['substrips'][ti]['cells'][ci]
  for data in entry['internal_cuts']:
   u=decv(data)
   def bound(rec):
    if rec is None:return None
    row=strip['curves'][rec[0]];return evaluate(list(map(m.decd,row['numerator'])),u)/evaluate(list(map(m.decd,row['denominator'])),u)
   lo=V(z) if cell['left_boundary'] is None else bound(cell['left_boundary']);hi=bound(cell['right_boundary'])
   verify(fi,u,lo,hi,['internal',di,fi,data])
 assert matched==set(observations) and empty==out['empty_fibers'] and not out['failures']
 assert equalitycount==len(out['equality'])
 print(json.dumps({'status':'PASS independent original/internal closed-fiber gap arithmetic','checked_fibers':len(matched),'empty_fibers':empty,'equality_occurrences':equalitycount,'infinite_tails':tailcount,'elapsed_seconds':round(time.perf_counter()-start,3),'writes':False}))
if __name__=='__main__':main()
