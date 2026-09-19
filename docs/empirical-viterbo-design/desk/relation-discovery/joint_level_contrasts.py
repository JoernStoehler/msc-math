#!/usr/bin/env python3
"""Target-free joint descriptor fibers on the native orientation collection.

Preserve the vertex Williamson spectrum analytically along two great circles,
then solve for repeated bulk fourth moments. Numerical witnesses, not exact
certificates. Native geometry and tensors are never modified or regenerated.
"""
import argparse
import hashlib
import importlib.util
import json
from pathlib import Path

import numpy as np
from scipy.optimize import brentq


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def basis(theta):
    theta=np.asarray(theta)
    return np.array([np.ones_like(theta),np.cos(2*theta),np.sin(2*theta),
                     np.cos(4*theta),np.sin(4*theta)]).T


def main():
    arg=argparse.ArgumentParser()
    arg.add_argument('--bodies',type=Path,required=True)
    arg.add_argument('--measurement-code',type=Path,required=True)
    arg.add_argument('--out',type=Path,required=True)
    args=arg.parse_args()
    spec=importlib.util.spec_from_file_location('native_orientation_measurement',args.measurement_code)
    module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module)
    bodies=[json.loads(line) for line in args.bodies.read_text().splitlines()]
    results=[];controls=[]
    for body in bodies:
        vertices=np.array(body['vertices']);centered=vertices-vertices.mean(0)
        cv=centered.T@centered/len(vertices)
        forms=module.FORMS
        A=np.array([[-np.trace(cv@left@cv@right) for right in forms] for left in forms])
        eigenvalues,E=np.linalg.eigh(A)
        if np.ptp(eigenvalues)<1e-10:
            controls.append({'body_id':body['body_id'],'body_name':body['name'],
                'status':'scalar quadratic; already covered by owner exact controls'})
            continue
        lo,mid,hi=eigenvalues
        alpha=np.sqrt((hi-mid)/(hi-lo));beta=np.sqrt((mid-lo)/(hi-lo))
        tensor=np.array(body['volume_fourth_moment'])
        covariance=np.array(body['volume_covariance'])
        polar=np.array(body['polar_vertices'])
        def orient(theta,sign):
            return E@np.array([alpha*np.cos(theta),np.sin(theta),sign*beta*np.cos(theta)])
        def bulk_fourth(u):
            omega=np.einsum('a,aij->ij',u,forms)
            return module.pairing_fourth(tensor,omega)
        models={};fit_errors=[]
        train=np.arange(16)*np.pi/16
        checks=(np.arange(31)+.371)*np.pi/31
        for sign in (-1,1):
            values=np.array([bulk_fourth(orient(t,sign)) for t in train])
            coeff=np.linalg.lstsq(basis(train),values,rcond=None)[0]
            models[sign]=coeff
            fit_errors.extend(abs(float(basis(t)@coeff)-bulk_fourth(orient(t,sign))) for t in checks)
        assert max(fit_errors)<2e-10
        grid=np.linspace(0,np.pi,129)
        observed=np.concatenate([basis(grid[:-1])@models[s] for s in (-1,1)])
        target_levels=np.quantile(observed,[.125,.25,.375,.5,.625,.75,.875])
        level_results=[];best=None
        for level_index,target in enumerate(target_levels):
            records=[]
            for sign in (-1,1):
                coeff=models[sign]
                def residual(t): return float(basis(t)@coeff-target)
                roots=[]
                for left,right in zip(grid[:-1],grid[1:]):
                    fleft,fright=residual(left),residual(right)
                    if fleft==0:roots.append(left)
                    if fleft*fright<0:
                        roots.append(brentq(residual,left,right,xtol=5e-15,rtol=1e-14))
                for theta in roots:
                    u=orient(theta,sign)
                    if any(abs(abs(np.dot(u,np.array(r['orientation'])))-1)<1e-10 for r in records):
                        continue
                    r=module.measure(vertices,polar,tensor,body['ridges'],body['volume'],covariance,u)
                    r.update(circle_sign=sign,theta=theta,
                             vertex_quadratic=float(u@A@u),
                             quartic_target=float(target),
                             quartic_direct_residual=r['volume_pairing_fourth']-float(target))
                    records.append(json.loads(json.dumps(r,default=module.jsonable)))
            if len(records)<2:
                level_results.append({'index':level_index,'roots':len(records),'status':'insufficient distinct roots'})
                continue
            low=min(records,key=lambda r:r['dual_reciprocal_over_volume_sqrt'])
            high=max(records,key=lambda r:r['dual_reciprocal_over_volume_sqrt'])
            span=high['dual_reciprocal_over_volume_sqrt']/low['dual_reciprocal_over_volume_sqrt']-1
            rhos=[r['vertex_covariance_rho'] for r in records]
            fourths=[r['volume_pairing_kurtosis'] for r in records]
            ridge=[r['ridge_area_sum_over_volume_sqrt'] for r in records]
            entry={'index':level_index,'quartic_target':float(target),'roots':len(records),
                   'dual_fractional_span':span,'rho_range':float(np.ptp(rhos)),
                   'bulk_kurtosis_range':float(np.ptp(fourths)),
                   'ridge_fractional_span':max(ridge)/min(ridge)-1,
                   'low_dual':low,'high_dual':high}
            assert entry['rho_range']<2e-10
            assert entry['bulk_kurtosis_range']<2e-10
            level_results.append(entry)
            if best is None or span>best['dual_fractional_span']:best=entry
        assert best is not None
        results.append({'body_id':body['body_id'],'body_name':body['name'],
            'population':body['population'],'eigenvalues':eigenvalues.tolist(),
            'eigenframe':E.tolist(),'vertex_quadratic':A.tolist(),
            'fourth_fourier_coefficients':{str(k):v.tolist() for k,v in models.items()},
            'fourth_fourier_max_validation_error':max(fit_errors),
            'selected':best,'levels':level_results,
            'status':'analytic covariance level construction and numerical quartic roots; no capacity claim'})
        print(body['name'],len(best['low_dual']['orientation']),best['dual_fractional_span'],best['rho_range'],best['bulk_kurtosis_range'])
    output={'schema':'joint-covariance-fourth-fibers-v1',
        'bodies_path':str(args.bodies),'bodies_sha256':sha(args.bodies),
        'measurement_code_path':str(args.measurement_code),'measurement_code_sha256':sha(args.measurement_code),
        'method':'u=E(alpha*cos(theta),sin(theta),+-beta*cos(theta)); uTAu=middle eigenvalue; solve quartic field levels',
        'selection':'seven interior empirical quantiles of the two circles; retain largest dual separation; all levels kept',
        'excluded_controls':controls,'contrasts':results,'capacity_calls':0,
        'claim_status':'numerical controlled witnesses; no exact geometry certification or population inference'}
    args.out.write_text(json.dumps(output,indent=2)+'\n')


if __name__=='__main__':main()
