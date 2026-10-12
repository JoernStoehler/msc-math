"""Independent complete equality census audit after accepted gap proofs."""
from review_signs import *
import hashlib

def main():
 start=time.perf_counter();opened=json.loads((root/'open-proof.json').read_text());closed=json.loads((root/'closed-proof.json').read_text());eq=json.loads((root/'equality.json').read_text())
 assert eq['open_proof_sha256']==hashlib.sha256((root/'open-proof.json').read_bytes()).hexdigest()
 assert eq['closed_proof_sha256']==hashlib.sha256((root/'closed-proof.json').read_bytes()).hexdigest()
 ids={c['query'] for r in opened['segments'] for c in r['endpoint_checks'] if 'query' in c}
 assert ids=={r['query'] for r in eq['full_root_checks']} and len(ids)==eq['endpoint_query_count']
 for record in eq['full_root_checks']:
  query=opened['queries'][record['query']];p=list(map(m.decd,query['polynomial']));assert p!=[z]
  left,right=decv(query['left']),decv(query['right']);count=count_open(p,left,right)
  assert count==record['full_root_query']['count']==0
 assert not eq['interior_endpoint_zero_queries'] and not eq['identically_zero_open_endpoint_polynomials']
 assert not any(c['identically_zero_gap'] for c in closed['checks']) and not eq['flat_zero_closed_fibers']
 # Each open minimum is an endpoint minimum, so strict endpoint gaps imply strict
 # entire x-fiber gaps even when the quadratic becomes linear/constant.
 assert all(r['minimum_mode'] in ('concave/linear','singleton endpoint','endpoint derivative lower','endpoint derivative upper') for r in opened['segments'])
 bodies={};parameters=set()
 for rec in closed['equality']:
  u,x=decv(rec['u']),decv(rec['x']);h=list(map(decv,rec['supports']))
  key=(u.key(),x.key(),tuple(v.key() for v in h));bodies.setdefault(key,[]).append(rec['owner']);parameters.add((u.key(),x.key()))
 observed={}
 for rec in eq['equality_bodies']:
  u,x=decv(rec['u']),decv(rec['x']);h=list(map(decv,rec['supports']))
  key=(u.key(),x.key(),tuple(v.key() for v in h));assert key not in observed;observed[key]=rec['owners']
 assert observed==bodies
 assert parameters=={(decv(r['u']).key(),decv(r['x']).key()) for r in eq['parameter_classes']}
 assert len(bodies)==len(parameters)==1
 u,x=next(iter(parameters));assert u==V(E(0,'1/5')).key() and x==V(one).key()
 print(json.dumps({'status':'PASS complete finite-candidate equality census','full_endpoint_root_checks':len(ids),'open_interior_equality':False,'distinct_closed_bodies':len(bodies),'parameter':'x=1,u=1/sqrt(5)','elapsed_seconds':round(time.perf_counter()-start,3),'physical_controls_pending':True,'writes':False}))
if __name__=='__main__':main()
