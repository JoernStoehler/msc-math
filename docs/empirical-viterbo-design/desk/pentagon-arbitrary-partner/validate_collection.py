"""Validation separate from discovery; records numerical rather than exact certificates."""
import json
from pathlib import Path
import numpy as np
from collection import ROOT, checks, two_bounce, Q, geometry, moments
from evaluator import evaluate

checks()
fixtures=[(np.array([[-2.,-1],[2,-1],[2,1],[-2,1]]),np.array([[-3.,-5],[3,-5],[3,5],[-3,5]]),20.),
          (np.array([[0.,0],[1,0],[0,1]]),np.array([[0.,1],[1,1],[1,0],[0,-1],[-1,-1],[-1,0]]),2.)]
records=[{'check':'two_bounce_fixture_'+str(i),'error':abs(two_bounce(p,q)[0]-expected)} for i,(p,q,expected) in enumerate(fixtures)]
base=[json.loads(x) for x in (ROOT/'collection-v1.jsonl').read_text().splitlines()]
follow=[json.loads(x) for x in (ROOT/'symmetrization-v1.jsonl').read_text().splitlines()]
allrows=base+follow
records.append({'check':'capacity_upper_ceiling','error':max(max(0,r['scalars']['capacity_over_A2']-1) for r in allrows)})
records.append({'check':'LP_primal_dual_residuals','error':max(max(f['primal_violation'],f['dual_stationarity_residual'],f['duality_gap']) for r in allrows for f in r['fits'])})
lookup={r['body_id']:r for r in base}
pathrows=[r for r in allrows if r['parent_id'] is not None]
records.append({'check':'fixed_width_paths_A2','error':max(abs(r['scalars']['A2']/lookup[r['parent_id']]['scalars']['A2']-1) for r in pathrows)})
records.append({'check':'fixed_width_paths_directional_widths','error':max(float(np.max(np.abs(np.array(r['fields']['width_field'])-lookup[r['parent_id']]['fields']['width_field']))) for r in pathrows)})
# Mixed-area identity: A(K_t)=A(K)+t(1-t)[A(K-K)-4A(K)].
records.append({'check':'symmetrization_area_identity','error':max(abs(r['scalars']['area']-(lookup[r['parent_id']]['scalars']['area']+r['source']['t']*(1-r['source']['t'])*(lookup[r['parent_id']]['scalars']['difference_area']-4*lookup[r['parent_id']]['scalars']['area']))) for r in pathrows)})
symmetric=[r for r in allrows if r['scalars']['difference_area_excess']<1e-10]
records.append({'check':'symmetric_partner_lemma','error':max(abs(r['scalars']['capacity_over_A2']-1) for r in symmetric)})
records.append({'check':'central_sys_bound','error':max(max(0,r['scalars']['sys']-(10+6*np.sqrt(5))/25) for r in symmetric)})
# One irregular shape: affine-invariant moment summaries and centroid-polar product.
p=np.array(base[0]['vertices']);a=geometry(p);H=np.array([[1.2,.7],[-.2,.9]]);b=geometry(p@H.T+[3,-2])
for feature in ['affine_skew_tensor_norm2','affine_radial_fourth_moment','centroid_polar_area_product','difference_area_excess']:
 records.append({'check':'affine_'+feature,'error':abs(a[feature]-b[feature])})
assert max(r['error'] for r in records)<1e-9,records
result={'status':'passed_float_checks_not_rigorous_certificate','row_count':len(allrows),'symmetric_count':len(symmetric),'checks':records}
(ROOT/'validation-v1.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps(result,indent=2))
