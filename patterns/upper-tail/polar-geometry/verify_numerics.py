#!/usr/bin/env python3
"""Meaningful cross-checks of centering, symplectic covariance and affine Mahler invariance."""
import argparse,json,hashlib
from pathlib import Path
import numpy as np
from analyze import mat,feature,J
ap=argparse.ArgumentParser();ap.add_argument('--source',type=Path,required=True);ap.add_argument('--out',type=Path,required=True);z=ap.parse_args();rr=[]
B=np.array([[.3,.2],[.2,-.4]]);S=np.block([[np.eye(2),B],[np.zeros((2,2)),np.eye(2)]]);T=np.diag([2,.7,.5,1/.7]);shift=np.array([.06,-.04,.03,.05]);assert np.max(abs(S@J@S.T-J))<1e-14;assert np.max(abs(T@J@T.T-J))<1e-14
for fn in ['random.jsonl','random-product.jsonl']:
 with (z.source/fn).open() as f:
  for index in range(3):
   r=json.loads(next(f));a=mat(r['dual_vertices_rational']);v=mat(r['vertices_rational']);old=feature(a,v,r['volume'])
   for name,M,scale in [('symplectic_shear',S,1.),('symplectic_stretch',T,1.),('dilation',np.eye(4),1.7)]:
    M=M*scale;vv=v@M.T+shift;aa=a@np.linalg.inv(M);aa=aa/(1+aa@shift)[:,None];new=feature(aa,vv,r['volume']*abs(np.linalg.det(M)));ae=abs(new['alpha_root_volume']/old['alpha_root_volume']-1);me=abs(new['mahler']/old['mahler']-1);assert ae<1e-8 and me<1e-8;rr.append(dict(name=r['name'],intervention=name,alpha_root_volume_relative_error=ae,mahler_relative_error=me))
   M=np.array([[1,.3,.2,0],[0,1,0,-.1],[0,0,1,.4],[0,0,0,1.2]]);vv=v@M.T;aa=a@np.linalg.inv(M);new=feature(aa,vv,r['volume']*abs(np.linalg.det(M)));me=abs(new['mahler']/old['mahler']-1);assert me<1e-8;rr.append(dict(name=r['name'],intervention='general_affine_mahler_only',mahler_relative_error=me))
out=dict(checks=rr,max_symplectic_alpha_relative_error=max(r.get('alpha_root_volume_relative_error',0) for r in rr),max_mahler_relative_error=max(r['mahler_relative_error'] for r in rr));z.out.write_text(json.dumps(out,indent=2)+'\n');print(json.dumps({k:v for k,v in out.items() if k!='checks'}))
