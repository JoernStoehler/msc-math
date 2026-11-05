#!/usr/bin/env python3
"""Same-row fold baselines; no method choice or new geometry."""
import argparse,json,gzip,math
from pathlib import Path
import numpy as np
from analyze import metrics,designs
ap=argparse.ArgumentParser();ap.add_argument('--features',type=Path,required=True);ap.add_argument('--out',type=Path,required=True);z=ap.parse_args();rr=[json.loads(l) for l in gzip.open(z.features,'rt')];rr=[r for r in rr if r['ridge'] is not None];y=np.log([r['sys'] for r in rr]);b=np.array([r['bucket'] for r in rr]);R=np.log([r['ridge'] for r in rr]);o=np.ones(len(rr));tail=np.zeros(len(rr),bool)
for g in sorted(set(b)):
 ii=np.flatnonzero(b==g);tail[ii[np.argsort(y[ii],kind='stable')[-math.ceil(.1*len(ii)):]]]=True
X=designs(np.log([r['alpha_root_volume'] for r in rr]),np.log([r['mahler'] for r in rr]));X.update(constant=o[:,None],ridge_linear=np.c_[o,R],ridge_quadratic=np.c_[o,R,R*R]);out={}
for name,x in X.items():
 p=np.empty(len(y))
 for g in sorted(set(b)):
  te=b==g;p[te]=x[te]@np.linalg.lstsq(x[~te],y[~te],rcond=None)[0]
 out[name]={'all':metrics(y,p),'upper_decile':metrics(y[tail],p[tail])}
z.out.write_text(json.dumps(dict(contract='Same14334 rows; two old covariance-excluded rows lack the name-to-polyid ridge join and are excluded uniformly here only',scores=out),indent=2)+'\n');print(json.dumps({k:out[k] for k in ['constant','ridge_linear','ridge_quadratic']},indent=2))
