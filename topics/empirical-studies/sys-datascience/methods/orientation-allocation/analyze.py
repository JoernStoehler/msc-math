"""Reconcile frozen memberships, controls and charged work; never evaluates targets."""
import json, math, hashlib
from pathlib import Path
p=Path(__file__).resolve().parent
a=p/'artifacts'
inputs=[json.loads(x) for x in (a/'inputs.jsonl').read_text().splitlines()]
results=[json.loads(x) for x in (a/'results.jsonl').read_text().splitlines()]
assert len(inputs)==124 and len({x['id'] for x in inputs})==124
assert len({x['id'] for x in results})==len(results)
by={x['id']:x for x in results}
assert set(by)<=set(x['id'] for x in inputs)
frozen=json.loads((a/'freeze.json').read_text())
assert hashlib.sha256((a/'inputs.jsonl').read_bytes()).hexdigest()==frozen['input_sha256']
blocks=[];curves=[];controls=[]
for block in ['3x3-b0','3x3-b1','4x4-b0','4x4-b1']:
    rows=[x for x in inputs if x['block']==block]
    arm_results={}
    for arm in ['iid','orientation']:
        members=sorted([(m['step'],x) for x in rows for m in x['memberships'] if m['arm']==arm])
        assert [i for i,x in members]==list(range(16))
        elapsed=0.;best=None;ok=0;failures=[]
        for step,x in members:
            r=by.get(x['id']);elapsed+=x['proposal_ms']
            if r is None: failures.append({'id':x['id'],'status':'not_completed'});break
            assert r['input']==x
            assert r['evaluator']['input_file_sha256']==frozen['input_sha256']
            elapsed+=r['wall_ms']
            if r['status']=='ok':
                assert math.isclose(r['sys'],r['capacity']**2/(2*r['volume']),rel_tol=1e-12)
                assert r['capacity_lower']<=r['capacity']<=r['capacity_upper']
                assert math.isclose(r['volume'],x['target_free_volume'],rel_tol=1e-12)
                best=r['sys'] if best is None else max(best,r['sys']);ok+=1
            else: failures.append({'id':x['id'],'stage':r.get('error_stage'),'error':r.get('error_type')})
            curves.append({'block':block,'arm':arm,'step':step,'charged_calls':step+1,'charged_ms':elapsed,'best_sys':best,'status':r['status']})
        arm_results[arm]={'valid':ok,'complete':ok==16,'best_sys':best,'charged_ms':elapsed,'failures':failures}
    base=by.get(block+'-iid-0');u2=by.get(block+'-orientation-1')
    control={'block':block,'pass':False}
    if base and u2 and base['status']==u2['status']=='ok':
        delta=u2['sys']-base['sys'];control.update(delta_sys=delta,relative_volume_change=u2['volume']/base['volume']-1.,**{'pass':abs(delta)<=1e-7 and abs(u2['volume']/base['volume']-1.)<=1e-8})
    controls.append(control)
    complete=all(x['complete'] for x in arm_results.values()) and control['pass']
    blocks.append({'block':block,'arms':arm_results,'complete':complete,'orientation_minus_iid_best':arm_results['orientation']['best_sys']-arm_results['iid']['best_sys'] if complete else None})
complete=len(results)==124 and all(x['complete'] for x in blocks)
summary={'schema':'orientation-allocation-result-v1','complete':complete,'expected_unique':124,'evaluated_rows':len(results),'valid_rows':sum(x['status']=='ok' for x in results),'charged_memberships':128,'blocks':blocks,'u2_controls':controls,'unique_evaluator_wall_ms':sum(x['wall_ms'] for x in results),'unique_proposal_ms':sum(x['proposal_ms'] for x in inputs),'equal_bucket_mean_best_difference':sum(x['orientation_minus_iid_best'] for x in blocks)/4 if complete else None,'boundary':'Finite four-block comparison; capacity bounds do not certify f64 volume or ratio. Full-grid scientific verdict requires all successes and controls. Shared-start cost charged to both arms. Proposal timings include generation/rejection or transformation plus exact geometry validation; compilation, panel serialization and analysis excluded.'}
(a/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
(a/'curves.jsonl').write_text(''.join(json.dumps(x)+'\n' for x in curves))
print(json.dumps(summary,indent=2))
