#!/usr/bin/env python3
"""Check both fixed-support shadow objectives on the already known coordinate trap.
Reads one registered geometry row and the retained response packet. No search.
"""
import argparse,hashlib,importlib.util,json
from fractions import Fraction as F
from pathlib import Path
p=argparse.ArgumentParser();p.add_argument('--source',type=Path,required=True);p.add_argument('--response',type=Path,required=True);p.add_argument('--out',type=Path,required=True);args=p.parse_args()
sp=importlib.util.spec_from_file_location('independent',Path(__file__).with_name('check.py'));m=importlib.util.module_from_spec(sp);sp.loader.exec_module(m)
assert hashlib.sha256(args.source.read_bytes()).hexdigest()=='66bf82010e92e0f26b0df226f4e6c0eef05d21eb22a0967c7f669530f6545736'
for line in args.source.open():
 r=json.loads(line)
 if r['name']=='random_5x5_s1_64':break
else:raise Exception('missing control')
v=[list(map(F,u)) for u in r['dual_vertices_rational']];a=[u[:2] for u in v[:r['k']]];b=[u[2:] for u in v[r['k']:]]
s=json.loads(args.response.read_text());rr=next(z for z in s['rows'] if z['name']==r['name']);out=[]
for variant in rr['selection']['variants']:
 aa=list(map(F,variant['alpha']));gg=list(map(F,variant['gamma']));x=[tuple(w*u for u in av) for w,av in zip(aa,a) if w];y=[tuple(w*u for u in bv) for w,bv in zip(gg,b) if w]
 qs=[m.mixed(x,y,sign) for sign in [1,-1]]
 assert max(qs)==F(variant['Q'])
 out.append(dict(original_sign=variant['shadow_sign'],original_Q=variant['Q'],positive_shadow_Q=str(qs[0]),negative_shadow_Q=str(qs[1]),maximum_Q=str(max(qs)),same_as_original=True))
receipt=dict(status='PASS',name=r['name'],source_sha256=hashlib.sha256(args.source.read_bytes()).hexdigest(),response_sha256=hashlib.sha256(args.response.read_bytes()).hexdigest(),geometry=r['dual_vertices_rational'],variants=out)
args.out.write_text(json.dumps(receipt,indent=2)+'\n');print(json.dumps(dict(status='PASS',name=r['name'],both_final_pairs_are_order_optimal=True)))
