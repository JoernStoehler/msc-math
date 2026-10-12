"""Resolve one representation ambiguity: fixed widths+area+tenfits versus odd shape modes."""
import hashlib
import json
from time import perf_counter
import numpy as np
from collection import ROOT, geometry, body_id, hull
from evaluator import evaluate, templates


def support_boundary(n,m,a):
    theta=2*np.pi*np.arange(n)/n
    u=np.column_stack([np.cos(theta),np.sin(theta)])
    v=np.column_stack([-np.sin(theta),np.cos(theta)])
    h=1+a*np.cos(m*theta);hp=-m*a*np.sin(m*theta)
    return h[:,None]*u+hp[:,None]*v


def measure(p,source):
    p=hull(p);evaluation=evaluate(p);g=geometry(p)
    fits=[]
    for f in evaluation['fits']:
        keep={k:f[k] for k in ['template_id','scale','translation','active_facets','primal_violation','dual_stationarity_residual','duality_gap']}
        keep['nonzero_dual_weights']=[(i,y) for i,y in enumerate(f['dual_weights']) if abs(y)>1e-12]
        fits.append(keep)
    return {'body_id':body_id(p),'source':source,'vertices':p.tolist(),
            'geometry':g,'capacity':evaluation['capacity'],'sys':evaluation['sys'],
            'fits':fits,'facet_normals':evaluation['facet_normals'],'facet_offsets':evaluation['facet_offsets']}


def placement_checks(rows):
    output=[]
    for row in rows:
        if row['source']['family']!='odd_support_harmonic': continue
        m=row['source']['m'];a=row['source']['a'];errors=[];midpoints=[]
        for f,t in zip(row['fits'],templates()):
            T=np.array(t['vertices'])
            distances=np.linalg.norm(T[:,None,:]-T[None,:,:],axis=2)
            i,j=np.unravel_index(np.argmax(distances),distances.shape)
            u=(T[j]-T[i])/distances[i,j];v=np.array([-u[1],u[0]])
            theta=np.arctan2(u[1],u[0])
            expected=a*np.cos(m*theta)*u-m*a*np.sin(m*theta)*v
            actual=np.array(f['translation'])+f['scale']*(T[i]+T[j])/2
            errors.append(float(np.linalg.norm(expected-actual)));midpoints.append(actual.tolist())
        output.append({'m':m,'n':row['source']['n'],'max_midpoint_error':max(errors),
                       'diameter_midpoints':midpoints})
    return output


def run():
    start=perf_counter();rows=[];checks=[]
    a3=.025
    expected_area=np.pi*(1-4*a3*a3)
    expected_fit=2*(1+np.cos(np.pi/5))
    for m in [3,5,7]:
        a=a3*np.sqrt(8/(m*m-1))
        assert (m*m-1)*a<1
        assert (m+1)*a+np.tan(np.pi/5)<1
        for n in [360,720]:
            row=measure(support_boundary(n,m,a),{'family':'odd_support_harmonic','formula':'h(theta)=1+a*cos(m*theta)',
                         'm':m,'a':float(a),'a_squared_exact':f'1/{200*(m*m-1)}','n':n,
                         'relation':'(m*m-1)*a*a=1/200; equal smooth area and width',
                         'status':'inscribed polygon approximation; smooth identities proved separately'})
            rows.append(row)
            checks.append({'m':m,'n':n,'body_id':row['body_id'],'fit_max_error':max(abs(f['scale']-expected_fit) for f in row['fits']),
                           'area_error':abs(row['geometry']['area']-expected_area),
                           'width_error':max(abs(w-2) for w in row['geometry']['width_field']),
                           'covariance_trace':row['geometry']['covariance_trace'],
                           'b3':row['geometry']['affine_skew_tensor_norm2'],
                           'b4':row['geometry']['affine_radial_fourth_moment'],
                           'polar_product':row['geometry']['centroid_polar_area_product']})
    # Cross-line acquisition is useful here solely to compare a radial deformation
    # with the exactly fixed-width support deformations, not to optimize a bound.
    source_path=ROOT.parent/'relation-discovery/moment-adverse-polygons.jsonl'
    source=json.loads(source_path.read_text().splitlines()[0])
    row=measure(np.array(source['vertices']),{'family':'cross_line_radial_control','parent_body_id':source['body_id'],
                'source_path':str(source_path),'source_sha256':hashlib.sha256(source_path.read_bytes()).hexdigest(),
                'construction':source['construction'],'a_exact':source['a_exact']})
    rows.append(row)
    out=ROOT/'harmonic-fibers-v1.jsonl'
    out.write_text(''.join(json.dumps(row,separators=(',',':'))+'\n' for row in rows))
    result={'status':'smooth identities derived; polygon measurements floating point',
            'count':len(rows),'seconds':perf_counter()-start,'expected_smooth_area':expected_area,
            'expected_all_ten_smooth_fits':expected_fit,'checks':checks,
            'placement_midpoint_checks':placement_checks(rows),
            'cross_line_radial_control':{'body_id':row['body_id'],'capacity_over_A2':row['capacity']/row['geometry']['A2'],
                                        'width_min':min(row['geometry']['width_field']),
                                        'width_max':max(row['geometry']['width_field'])},
            'script_sha256':hashlib.sha256((ROOT/'harmonic_fibers.py').read_bytes()).hexdigest(),
            'data_sha256':hashlib.sha256(out.read_bytes()).hexdigest()}
    (ROOT/'harmonic-fibers-v1-summary.json').write_text(json.dumps(result,indent=2)+'\n')
    print(json.dumps(result,indent=2))

if __name__=='__main__':run()
