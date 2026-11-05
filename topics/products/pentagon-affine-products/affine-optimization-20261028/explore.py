# /// script
# dependencies = ["numpy", "scipy"]
# ///
"""Exploratory floating edge enumeration; no completeness certificate."""
import itertools
import json
from pathlib import Path
import numpy as np
from scipy.spatial import HalfspaceIntersection

OUT = Path('/tmp/msc-math/autonomous-20261028/optimizer')
OUT.mkdir(parents=True, exist_ok=True)
labels = [(i, j, e) for i in range(5) for j in range(7) for e in (1, -1)]
norm = lambda n, i: np.array([np.cos(2*np.pi*i/n), np.sin(2*np.pi*i/n)])
rows = np.array([e*np.outer(norm(5,i), norm(7,j)).ravel() for i,j,e in labels])
# Rotate and negate any active contact to (0,0,+); all preserve det.
fixed = labels.index((0,0,1))
C = rows[:,1:]
d = 1-rows[:,0]
edges = []
for a,b in itertools.combinations([k for k in range(70) if k != fixed], 2):
    Q = C[[a,b]]
    direction = np.cross(Q[0], Q[1])
    if np.linalg.norm(direction)<1e-10:
        continue
    origin = np.linalg.lstsq(Q,d[[a,b]], rcond=None)[0]
    slope = C@direction
    residual = d-C@origin
    if np.any(residual[np.abs(slope)<1e-10]<-1e-8):
        continue
    upper = min(residual[slope>1e-10]/slope[slope>1e-10], default=np.inf)
    lower = max(residual[slope< -1e-10]/slope[slope< -1e-10], default=-np.inf)
    if upper-lower<1e-9:
        continue
    endpoints = [np.r_[1, origin+t*direction] for t in (lower,upper)]
    A,B = endpoints
    N = B-A
    det = lambda m: m[0]*m[3]-m[1]*m[2]
    qa = det(N)
    qb = A[0]*N[3]+N[0]*A[3]-A[1]*N[2]-N[1]*A[2]
    ts = [0.,1.]
    if qa < -1e-12 and 0 < -qb/(2*qa) < 1:
        ts.append(-qb/(2*qa))
    best = max([(det(A+t*N),float(t),A+t*N) for t in ts],key=lambda z:z[0])
    edges.append(dict(contacts=[labels[fixed],labels[a],labels[b]],value=best[0],t=best[1],matrix=best[2].tolist(),endpoints=[A.tolist(),B.tolist()],qa=qa))
edges.sort(key=lambda r:-r['value'])
(OUT/'floating-edges.json').write_text(json.dumps(edges,indent=2))
print('feasible triple lines',len(edges),'stationary winners',sum(0<e['t']<1 for e in edges))
for e in edges[:20]: print(e)
print('vertex values',sorted(set(round(e['value'],12) for e in edges if e['t'] in (0.,1.)),reverse=True))
