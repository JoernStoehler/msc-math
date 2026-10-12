"""Adaptive follow-up: fixed-width paths starting with a strict two-bounce gap."""
import hashlib
import json
from pathlib import Path
from time import perf_counter
import numpy as np
from scipy.optimize import linprog
from scipy.spatial import ConvexHull
from collection import ROOT, geometry, body_id, hull, difference, two_bounce, Q, VERSION
from evaluator import evaluate, templates, regular


def path(p,t):
    return hull(((1-t)*p[:,None,:]-t*p[None,:,:]).reshape(-1,2))


def plateau_threshold(p):
    """First t in [0,1/2] for which all A2-scaled templates fit K_t."""
    A=ConvexHull(difference(p)).equations[:,:2]
    hplus=np.max(A@p.T,axis=1); hminus=np.max(-A@p.T,axis=1)
    ceiling,_=two_bounce(p)
    records=[]
    for template in templates():
        T=np.array(template['vertices']);support=np.max(A@T.T,axis=1)
        C=np.column_stack([A,-(hminus-hplus)])
        lp=linprog([0.,0.,1.], A_ub=C,b_ub=hplus-ceiling*support,
                   bounds=[(None,None),(None,None),(0.,.5)],method='highs')
        if not lp.success: raise RuntimeError(lp.message)
        records.append({'template_id':template['id'],'first_t':float(lp.x[2]),
                        'translation':lp.x[:2].tolist(),
                        'slacks':lp.ineqlin.residual.tolist(),
                        'dual_weights':(-lp.ineqlin.marginals).tolist()})
    return {'threshold':max(r['first_t'] for r in records),'template_thresholds':records,
            'A2':ceiling,'facet_normals':A.tolist(),'hplus':hplus.tolist(),'hminus':hminus.tolist()}


def measure(name,p,family,source,parent=None):
    result=evaluate(p);g=geometry(p)
    fields={k:g.pop(k) for k in ['support_field','width_field','polar_vertices','centered_monomials','covariance']}
    g.update(capacity=result['capacity'],sys=result['sys'],capacity_over_A2=result['capacity']/g['A2'],
             fit_max_over_min=max(f['scale'] for f in result['fits'])/result['capacity'])
    return {'body_id':body_id(np.array(result['vertices'])),'name':name,'family':family,
            'parent_id':parent,'source':source,'measurement_version':VERSION,
            'vertices':result['vertices'],'scalars':g,'fields':fields,'fits':result['fits'],
            'facet_normals':result['facet_normals'],'facet_offsets':result['facet_offsets']}


def run():
    start=perf_counter()
    original=[json.loads(line) for line in (ROOT/'collection-v1.jsonl').read_text().splitlines()]
    names=['disk_8','gaussian_8','halfplanes_5','cover_3','regular_3_0.1700']
    rows=[];summaries=[]
    for name in names:
        base=next(r for r in original if r['name']==name);p=np.array(base['vertices'])
        threshold=plateau_threshold(p)
        summary={'name':name,'body_id':base['body_id'],**threshold,'grid':[]}
        for t in np.linspace(0,.5,11):
            row=measure(f'{name}_sym_{t:.3f}',path(p,t),'strict_gap_symmetrization',
                        {'t':float(t),'operation':'(1-t)K+t(-K)'},base['body_id'])
            rows.append(row)
            summary['grid'].append({'t':float(t),'capacity_over_A2':row['scalars']['capacity_over_A2'],
                                    'sys':row['scalars']['sys'],'body_id':row['body_id']})
        tau=threshold['threshold']
        for delta in [-1e-4,0.,1e-4]:
            t=max(0,min(.5,tau+delta))
            row=measure(f'{name}_threshold_{delta:+.4f}',path(p,t),'plateau_threshold_check',
                        {'t':float(t),'delta_from_threshold':delta,'operation':'(1-t)K+t(-K)'},base['body_id'])
            rows.append(row)
            summary.setdefault('threshold_checks',[]).append({'delta':delta,'capacity_over_A2':row['scalars']['capacity_over_A2']})
        summaries.append(summary)
    # Retain the adverse symmetric controls used while interpreting the first collection.
    for j in range(3):
        p=difference(np.array(templates()[j]['vertices']))/2
        rows.append(measure(f'half_template_difference_{j}',p,'adverse_symmetric_control',{'template_id':templates()[j]['id']}))
    for n in [4,6,8,10,12]:
        for angle in [0.,.1,.2,.3]:
            rows.append(measure(f'symmetric_regular_{n}_{angle}',regular(n,angle),'adverse_symmetric_control',{'n':n,'angle':angle}))
    # Universal symmetric reference body B=1/2(Q-Q)^polar.
    D=difference(Q);H=ConvexHull(D)
    B=hull(H.equations[:,:2]/(-H.equations[:,2,None]))/2
    brow=measure('universal_symmetric_reference',B,'derived_control',{'construction':'1/2 (Q-Q)^polar'})
    rows.append(brow)
    out=ROOT/'symmetrization-v1.jsonl'
    out.write_text(''.join(json.dumps(row,separators=(',',':'))+'\n' for row in rows))
    result={'status':'exploratory floating-point plus separately documented deductions','row_count':len(rows),
            'seconds':perf_counter()-start,'selected_parent_names':names,'paths':summaries,
            'B_capacity':brow['scalars']['capacity'],'B_A2':brow['scalars']['A2'],
            'script_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
            'parent_data_sha256':hashlib.sha256((ROOT/'collection-v1.jsonl').read_bytes()).hexdigest(),
            'data_sha256':hashlib.sha256(out.read_bytes()).hexdigest()}
    (ROOT/'symmetrization-v1-summary.json').write_text(json.dumps(result,indent=2)+'\n')
    print(json.dumps({'rows':len(rows),'seconds':result['seconds'],'paths':[{k:v for k,v in r.items() if k in ['name','threshold','threshold_checks','grid']} for r in summaries]},indent=2))

if __name__=='__main__':run()
