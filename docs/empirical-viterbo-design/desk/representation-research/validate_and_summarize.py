# /// script
# requires-python = ">=3.11"
# dependencies = ["numpy>=2", "scipy>=1.14"]
# ///
"""Independent metamorphic checks and an interpretable panel summary."""
import json
from pathlib import Path
import numpy as np
from scipy.linalg import expm
from scipy.stats import spearmanr
import orientation_panel as p

HERE=Path(__file__).resolve().parent
OUT=HERE/'artifacts'
bodies=[json.loads(line) for line in (OUT/'bodies.jsonl').read_text().splitlines()]
rows=[json.loads(line) for line in (OUT/'measurements.jsonl').read_text().splitlines()]
rng=np.random.default_rng(2026091803)
h=rng.normal(size=(4,4));h=(h+h.T)/8
symplectic=expm(p.J@h)
assert np.max(np.abs(symplectic.T@p.J@symplectic-p.J))<1e-13
keys=['dual_reciprocal_over_volume_sqrt','primal_pairing_max_over_volume_sqrt','vertex_pairing_kurtosis','volume_pairing_kurtosis','ridge_area_sum_over_volume_sqrt','ridge_area_max_share','vertex_covariance_rho','vertex_covariance_nu1_over_volume_sqrt','vertex_covariance_nu2_over_volume_sqrt']
checks=[];native=[];summaries=[]
for body in bodies:
    vertices=np.array(body['vertices'])
    transformed=1.7*vertices@symplectic.T+np.array([.17,-.11,.23,.08])
    vol,center,cov,_=p.moments(transformed)
    transformed-=center
    tensor=p.fourth_moment(transformed)
    dual,_,ridges=p.facet_data(transformed)
    measurement=p.measure(transformed,dual,tensor,ridges,vol,cov,np.array([1.,0,0]))
    base=next(r for r in rows if r['body_id']==body['body_id'] and r['orientation_id']==0)
    errors={key:abs(measurement[key]-base[key])/max(abs(base[key]),1e-15) for key in keys}
    assert max(errors.values())<2e-9,(body['name'],errors)
    checks.append(dict(body_name=body['name'],relative_errors=errors))
    # Preserve the original body as a separate population view. This avoids
    # treating non-symplectic whitening as removal of a harmless gauge.
    original=np.array(body['source_vertices'])-np.array(body['source_centroid'])
    vol,center,cov,_=p.moments(original)
    original-=center
    dual,_,ridges=p.facet_data(original)
    tensor=p.fourth_moment(original)
    measurement=p.measure(original,dual,tensor,ridges,vol,cov,np.array([1.,0,0]))
    measurement.update(body_id=body['body_id'],body_name=body['name'],population=body['population'],
        geometry_view='original centered source; no whitening',volume=vol,volume_covariance=cov)
    native.append(measurement)
    rs=[r for r in rows if r['body_id']==body['body_id']]
    fields={}
    for key in keys:
        low=min(rs,key=lambda r:r[key]);high=max(rs,key=lambda r:r[key])
        fields[key]=dict(min=low[key],max=high[key],fractional_span=high[key]/low[key]-1,
            min_orientation_id=low['orientation_id'],max_orientation_id=high['orientation_id'])
    correlations={}
    for left,right in [('volume_pairing_kurtosis','dual_reciprocal_over_volume_sqrt'),('volume_pairing_kurtosis','ridge_area_sum_over_volume_sqrt'),('vertex_pairing_kurtosis','volume_pairing_kurtosis')]:
        correlations[left+'__'+right]=None if fields[left]['fractional_span']<1e-10 or fields[right]['fractional_span']<1e-10 else float(spearmanr([r[left] for r in rs],[r[right] for r in rs]).statistic)
    summaries.append(dict(body_name=body['name'],population=body['population'],ranges=fields,within_body_spearman=correlations))

# Independent exact formula comparisons (the rational derivation is separate).
cube=[r for r in rows if r['body_name']=='cube']
calibration_errors={
 'cube_kurtosis':max(abs(r['volume_pairing_kurtosis']-(2.7+.36*np.sum(np.array(r['orientation'])**4))) for r in cube),
 'cube_dual':max(abs(r['dual_reciprocal_over_volume_sqrt']-1/(4*max(abs(x) for x in r['orientation']))) for r in cube),
 'cube_ridge':max(abs(r['ridge_area_sum_over_volume_sqrt']-8*sum(abs(x) for x in r['orientation'])) for r in cube),
 'simplex_kurtosis':max(abs(r['volume_pairing_kurtosis']-6723/1568) for r in rows if r['body_name']=='simplex'),
 'cell24_kurtosis':max(abs(r['volume_pairing_kurtosis']-3645000/1399489) for r in rows if r['body_name']=='24cell')}
assert max(calibration_errors.values())<2e-11
p.save(OUT/'validation.json',dict(symplectic_map=symplectic,translation=[.17,-.11,.23,.08],scale=1.7,
    metamorphic_checks=checks,exact_formula_absolute_errors=calibration_errors))
p.save(OUT/'summary.json',dict(body_count=len(bodies),orientation_count_per_body=128,
    capacity_evaluations=0,summary=summaries,
    interpretation='controlled covariance-blind orientation responses, not independent random bodies; ranges are sampled except exact named controls'))
with (OUT/'native-measurements.jsonl').open('w') as stream:
    for row in native:stream.write(json.dumps(row,default=p.jsonable,sort_keys=True)+'\n')
print(json.dumps(calibration_errors,indent=2))
for summary in summaries:
    print(summary['body_name'],summary['within_body_spearman'])
