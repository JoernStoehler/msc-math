import json
from pathlib import Path
import numpy as np
from scipy.optimize import linprog
p=Path('/tmp/msc-math/tensor-discriminator/20261022/results.json')
r=json.loads(p.read_text())
out=[]
for case in r['cases']:
    g=np.asarray(case['four_generators']).reshape(-1,4)
    x=np.asarray(case['worst']['order']['tensor']).reshape(4)
    aeq=np.vstack((g.T,np.ones(len(g))))
    beq=np.r_[x,1.0]
    sol=linprog(np.zeros(len(g)),A_eq=aeq,b_eq=beq,bounds=(0,None),method='highs')
    direction=np.asarray(case['worst']['separating_direction']).reshape(4)
    support=float(np.max(g @ direction))
    value=float(x @ direction)
    out.append({'pair':case['pair'],'lp_success':bool(sol.success),'lp_status':int(sol.status),'lp_message':sol.message,'lp_equality_max_abs_residual':float(np.max(np.abs(aeq@sol.x-beq))) if sol.success else None,'lp_nonzero_count':int(np.count_nonzero(sol.x>1e-9)) if sol.success else None,'lp_min_weight':float(np.min(sol.x)) if sol.success else None,'direct_support_gap_value_minus_support':value-support,'qhull_max_facet_residual':case['worst']['max_facet_residual']})
target=Path('/tmp/msc-math/tensor-discriminator/20261022/lp_crosscheck.json')
if target.exists(): raise FileExistsError(target)
target.write_text(json.dumps(out,indent=2)+'\n')
print(json.dumps(out,indent=2))
