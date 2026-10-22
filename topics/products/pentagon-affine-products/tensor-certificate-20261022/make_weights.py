import hashlib
import json
from pathlib import Path
import numpy as np
from scipy.linalg import qr
from scipy.optimize import linprog
SRC=Path('/tmp/msc-math/tensor-discriminator/20261022/results.json')
OUT=Path('/tmp/msc-math/tensor-certificate/20261022/weights.json')
if OUT.exists(): raise FileExistsError(OUT)
r=json.loads(SRC.read_text())
c=next(x for x in r['cases'] if x['pair']==[5,7])
g=np.asarray(c['four_generators']).reshape(-1,4)
a=np.vstack([g.T,np.ones(len(g))])
_,_,piv=qr(a,pivoting=True)
basis=piv[:5].tolist()
assert np.linalg.matrix_rank(a[:,basis])==5
floor=1e-5
alpha=1-len(g)*floor
D=10**12
rows=[]
for k,o in enumerate(c['six_tensors']):
    t=np.asarray(o['tensor']).reshape(4)
    sol=linprog(np.zeros(len(g)),A_eq=a,b_eq=np.r_[t/alpha,1],bounds=(0,None),method='highs')
    if not sol.success: raise RuntimeError((k,sol.message))
    w=floor+alpha*sol.x
    rounded=np.rint(w*D).astype(np.int64)
    delta=np.linalg.solve(a[:,basis],np.r_[t,1]-a@(rounded/D))
    corrected=rounded/D
    corrected[basis]+=delta
    if np.min(corrected)<1e-6: raise RuntimeError(('small corrected weight',k,float(np.min(corrected))))
    rows.append({'a_triple':o['a_triple'],'b_triple':o['b_triple'],'a_order':o['a_order'],'b_order':o['b_order'],'weights_numerators':rounded.tolist()})
obj={'source_results':str(SRC),'source_results_sha256':hashlib.sha256(SRC.read_bytes()).hexdigest(),'pair':[5,7],'basis_indices':basis,'denominator':D,'floor_target':floor,'numeric_condition_number':float(np.linalg.cond(a[:,basis])),'rows':rows,'numeric_min_corrected_weight':min(float(min(np.asarray(row['weights_numerators'])/D)) for row in rows)}
OUT.write_text(json.dumps(obj,separators=(',',':'))+'\n')
print('rows',len(rows),'basis',basis,'condition',obj['numeric_condition_number'],'min original rational weight',obj['numeric_min_corrected_weight'])
