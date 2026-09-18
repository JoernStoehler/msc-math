# /// script
# requires-python = ">=3.11"
# dependencies = ["numpy>=2", "scipy>=1.14"]
# ///
"""Construct orientations preserving both volume and vertex covariance spectra."""
import itertools
import json
from pathlib import Path
import numpy as np
import orientation_panel as p

out=Path(__file__).resolve().parent/'artifacts'
bodies=[json.loads(line) for line in (out/'bodies.jsonl').read_text().splitlines()]
contrasts=[]
for body in bodies:
    vertices=np.array(body['vertices'])
    cv=np.cov(vertices,rowvar=False,bias=True)
    quadratic=np.array([[-np.trace(cv@left@cv@right) for right in p.FORMS] for left in p.FORMS])
    eigenvalues,eigenframe=np.linalg.eigh(quadratic)
    if np.ptp(eigenvalues)<1e-10:continue
    # Signs in this eigenframe preserve both |u| and u^T A u exactly.
    # det(Cv) also stays fixed, so both Williamson frequencies stay fixed.
    # Several fixed magnitudes avoid accidental vanishing of odd cross terms.
    best=None
    for weights in [[1,1,1],[1,2,3],[3,1,2],[2,3,1]]:
        z=np.sqrt(np.array(weights)/sum(weights))
        candidates=[]
        for signs in itertools.product([-1,1],repeat=3):
            u=eigenframe@(np.array(signs)*z)
            record=p.measure(vertices,np.array(body['polar_vertices']),np.array(body['volume_fourth_moment']),
                body['ridges'],body['volume'],np.array(body['volume_covariance']),u)
            candidates.append(record)
        low=min(candidates,key=lambda r:r['volume_pairing_kurtosis'])
        high=max(candidates,key=lambda r:r['volume_pairing_kurtosis'])
        span=high['volume_pairing_kurtosis']/low['volume_pairing_kurtosis']-1
        if best is None or span>best['bulk_fourth_fractional_span']:
            best=dict(body_id=body['body_id'],body_name=body['name'],population=body['population'],
                second_pairing_quadratic=quadratic,eigenvalues=eigenvalues,eigenframe=eigenframe,weights=weights,
                low=low,high=high,bulk_fourth_fractional_span=span,
                vertex_rho_difference=abs(high['vertex_covariance_rho']-low['vertex_covariance_rho']),
                status='analytic level-set construction evaluated in f64; not a capacity statement')
    assert best['vertex_rho_difference']<2e-11
    contrasts.append(best)
    print(body['name'],best['bulk_fourth_fractional_span'],best['vertex_rho_difference'])
p.save(out/'matched-covariance-contrasts.json',dict(
    definition='u in SO4/U2 sphere; flip signs in eigencoordinates of A_ij=-tr(Cv Omega_i Cv Omega_j); preserve vertex Williamson spectrum and volume covariance I',
    contrasts=contrasts))
