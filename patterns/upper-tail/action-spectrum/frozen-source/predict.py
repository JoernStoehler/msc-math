#!/usr/bin/env python3
import json,sys,hashlib,math,importlib.util,time
from fractions import Fraction as F
from pathlib import Path
HERE=Path(__file__).resolve().parent
s=importlib.util.spec_from_file_location('probe',HERE/'probe.py');m=importlib.util.module_from_spec(s);s.loader.exec_module(m)
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def serial(v):
 if isinstance(v,F):return str(v)
 if isinstance(v,dict):return {k:serial(x) for k,x in v.items()}
 if isinstance(v,(list,tuple)):return [serial(x) for x in v]
 return v
source=Path('/tmp/msc-math/upper-tail-20261017/contact/sample.json');sample=json.loads(source.read_text());h=math.cos(math.pi/5);theta=math.pi/10;volh=(5*math.sin(2*math.pi/5)/2)**2
ah=[(math.cos((2*i+1)*math.pi/5)/h,math.sin((2*i+1)*math.pi/5)/h) for i in range(5)];bh=[(math.cos((2*i+1)*math.pi/5+theta)/h,math.sin((2*i+1)*math.pi/5+theta)/h) for i in range(5)];mh=m.score(ah,bh,1e-11);Lh=max(z['L'] for z in mh);m2h=max(z['m2'] for z in mh);Bh=1/(8*volh*Lh*Lh);sysh=(3+math.sqrt(5))/5
start=time.perf_counter();rows=[]
for row in sample['rows']:
 a=[[F(x) for x in v] for v in row['A']];b=[[F(x) for x in v] for v in row['B']];mom=m.score(a,b);volume=m.sh.old.area(a)*m.sh.old.area(b);L=max(z['L'] for z in mom);mm=max(z['m2'] for z in mom);B=1/(8*volume*L*L)
 rows.append({k:row[k] for k in row if k not in ['A','B']}|dict(moments=mom,volume=volume,L=L,rigorous_bound=B,predicted_bound_scaled=float(B)*sysh/Bh,predicted_rms=sysh*volh*m2h/(float(volume)*float(mm))))
out=dict(protocol='fixed moment tensor predictions; no pairwise action enumeration or fresh truth',sample_sha256=sha(source),code_sha256={f:sha(HERE/f) for f in ['PLAN.md','probe.py','predict.py']},calibration=dict(HKO_moments=mh,HKO_volume=volh,HKO_bound=Bh,HKO_ratio=sysh),rows=rows,elapsed_seconds=time.perf_counter()-start)
p=HERE/'predictions.json';assert not p.exists();p.write_text(json.dumps(serial(out),indent=2)+'\n');print(json.dumps(dict(rows=len(rows),elapsed_seconds=out['elapsed_seconds'],prediction_sha256=sha(p),first=rows[0]['name'])))
