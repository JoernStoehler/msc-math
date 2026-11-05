#!/usr/bin/env python3
"""One frozen sector row-product LP; exact reconstruction if feasible."""
import contextlib
import importlib.util
import io
import json
from pathlib import Path
import sys
import time

start = time.monotonic()
cached = ['/home/joern/.cache/uv/archive-v0/RcJQTM1A-Mr1_OFP',
          '/home/joern/.cache/uv/archive-v0/BdkG5We34Xm1t-b1']
sys.path[:0] = cached
import numpy as np
import scipy
from scipy.optimize import linprog

source, checker, output = map(Path, sys.argv[1:4])
sys.argv = [str(checker), str(source)]
spec = importlib.util.spec_from_file_location('exact_control', checker)
m = importlib.util.module_from_spec(spec)
with contextlib.redirect_stdout(io.StringIO()):
    spec.loader.exec_module(m)
F, dot = m.F, m.dot
rows = list(m.rows)
tau2 = m.tau2
rows += [(F(),F(),F(),F(),-tau2,F(1),F()),
         (F(),F(),F(),tau2,tau2,F(-1),tau2)]
active = [i for i, r in enumerate(rows) if dot(r, m.points[0]) == 0]
assert active == m.active_sets[0] and len(active) == 20
pairs = sorted({tuple(sorted((i,j))) for i in active for j in range(612)})
basis = [(i,j) for i in range(7) for j in range(i,7)]
flt = lambda a: float(a.a)+float(a.b)*np.sqrt(5.)
rf = np.array([[flt(x) for x in r] for r in rows])
left, right = np.array(pairs).T
matrix = np.array([rf[left,i]*rf[right,j] +
                   (rf[left,j]*rf[right,i] if i != j else 0)
                   for i,j in basis])
target_exact = [m.q[i][j]*(1 if i == j else 2) for i,j in basis]
target = np.array([flt(x) for x in target_exact])
result = linprog(np.zeros(len(pairs)), A_eq=matrix, b_eq=target,
                 bounds=(0,None), method='highs', options={
                     'time_limit':30., 'primal_feasibility_tolerance':1e-9,
                     'dual_feasibility_tolerance':1e-9})
receipt = dict(source_sha256=m.hashlib.sha256(source.read_bytes()).hexdigest(),
               scipy_version=scipy.__version__, numpy_version=np.__version__,
               package_paths=cached, solver='scipy.optimize.linprog(method=highs)',
               time_limit_seconds=30, status=int(result.status),
               message=result.message, row_count=612, active_count=len(active),
               product_columns=len(pairs), coefficient_equations=len(basis),
               exact_certificate=False)
if result.x is not None:
    support = np.flatnonzero(result.x > 0).tolist()
    receipt['positive_column_count'] = len(support)
    receipt['maximum_float_coefficient_residual'] = float(np.max(np.abs(matrix@result.x-target)))
    receipt['floating_support'] = [dict(pair=pairs[k], value=float(result.x[k])) for k in support]
    exact_columns = [[rows[pairs[k][0]][i]*rows[pairs[k][1]][j] +
                      (rows[pairs[k][0]][j]*rows[pairs[k][1]][i] if i != j else F())
                      for i,j in basis] for k in support]
    augmented = [[col[i] for col in exact_columns]+[target_exact[i]] for i in range(28)]
    pivot_row = 0
    pivots = {}
    for j in range(len(support)):
        choices = [i for i in range(pivot_row,28) if augmented[i][j] != 0]
        if not choices:
            continue
        k = choices[0]
        augmented[pivot_row],augmented[k] = augmented[k],augmented[pivot_row]
        a=augmented[pivot_row][j]
        augmented[pivot_row] = [x/a for x in augmented[pivot_row]]
        for i in range(28):
            if i != pivot_row:
                a=augmented[i][j]
                augmented[i]=[x-a*y for x,y in zip(augmented[i],augmented[pivot_row])]
        pivots[j]=pivot_row
        pivot_row += 1
    consistent=all(any(x != 0 for x in row[:-1]) or row[-1] == 0 for row in augmented)
    weights=[F() for _ in support]
    for j,i in pivots.items():
        weights[j]=augmented[i][-1]
    reconstructed = consistent and all(x.sign()>=0 for x in weights) and all(
        sum((weights[k]*exact_columns[k][i] for k in range(len(support))),F()) == target_exact[i]
        for i in range(28))
    receipt['exact_reconstruction_consistent']=consistent
    receipt['exact_reconstruction_nonnegative']=all(x.sign()>=0 for x in weights)
    receipt['exact_certificate']=reconstructed
    if reconstructed:
        receipt['exact_terms']=[dict(pair=pairs[support[k]], coefficient=w.data())
                                for k,w in enumerate(weights) if w != 0]
receipt['elapsed_seconds']=time.monotonic()-start
output.write_text(json.dumps(receipt,indent=2)+'\n')
print(json.dumps({k:v for k,v in receipt.items() if k not in ['floating_support','exact_terms']},indent=2))
