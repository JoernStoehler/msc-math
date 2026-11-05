#!/usr/bin/env python3
import importlib.util,json,hashlib,time
from pathlib import Path
from fractions import Fraction as F
D=Path(__file__).parent;sp=importlib.util.spec_from_file_location('spectral_probe',D/'replay.py');p=importlib.util.module_from_spec(sp);sp.loader.exec_module(p)
s=F(223606797749979,100000000000000);a=[[F(1),F(0)],[(s-1)/4,F(1)],[-(s+1)/4,(s-1)/2],[-(s+1)/4,-(s-1)/2],[(s-1)/4,-F(1)]];b=[[-v[1],v[0]] for v in a];a[0][1]=F(1,1000000);row=dict(name='spectral_one_tangent',A=a,B=b);z=p.select(a,b);delta=F(1,100000);lower=5/(1+delta)**2;upper=5/(1-delta)**2;assert lower*lower/(2*z['volume'])>1;assert upper<F(5001,1000);assert z['capacity']>F(5015,1000)
out=dict(input=row,selection=z,delta=delta,capacity_lower=lower,capacity_upper=upper,ratio_lower=lower*lower/(2*z['volume']),capacity_rule_miss_lower=F(5015,5001)-1,strict_tests=dict(true_capacity_below_5_001=True,selected_capacity_above_5_015=True,true_ratio_above_one=True),plan_sha256=hashlib.sha256((D/'PLAN.md').read_bytes()).hexdigest());(D/'obstruction.json').write_text(json.dumps(p.serial(out),indent=2)+'\n');print('spectral',float(z['capacity']),'true bracket',float(lower),float(upper),'ratio lower',float(lower*lower/(2*z['volume'])))
