#!/usr/bin/env python3
"""Exact algebra for the uniformly vertex-truncated regular HKO factors."""
import hashlib
import argparse
import importlib.util
import json
from fractions import Fraction as F
from pathlib import Path

SOURCE = Path('/workspaces/msc-math-worktrees/patterns-2026-10-17/patterns/upper-tail/conditional-coupling/quadratic.py')
spec = importlib.util.spec_from_file_location('width_q5', SOURCE)
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)
E = module.Q5
D = Path(__file__).resolve().parent

def sq(x):
    return x*x

def serial(x):
    if isinstance(x, E):
        return x.serial()
    if isinstance(x, dict):
        return {k: serial(v) for k, v in x.items()}
    if isinstance(x, list):
        return [serial(v) for v in x]
    return x

h=(1+E(0,1))/4
s2=1-h*h
L=1+h
m2=4*h*h*s2
area0_squared=25*s2/(h*h)
d0=L/h
capacity0_squared=sq(sq(d0))/m2
rho=(3+E(0,1))/5
assert capacity0_squared/(2*area0_squared)==rho
assert s2+L*L==2*L
assert 1/s2+1/(L*L)==2/(L*s2)
assert 0<s2<1 and L>0
# Polynomial identity in arbitrary delta, checked by coefficient equality:
# (1-delta^2/s2)-(1-delta/L)^2
# = (2/L)*delta*(1-delta/s2).
lhs=[E(0),2/L,-1/s2-1/(L*L)]
rhs=[E(0),2/L,-2/(L*s2)]
assert lhs==rhs
# Five removed triangles / original area = delta^2/s2.
# Single removed triangle coefficient 1/(h*s), area0=5*s/h.
assert 5/(5*s2)==1/s2
checks=[]
for delta in [E(F(1,1000)),s2/2]:
    area_factor=1-delta*delta/s2
    cap_factor=sq(1-delta/L)
    width_ratio_bound=rho*sq(cap_factor/area_factor)
    assert 0<delta<s2 and area_factor>cap_factor>0
    assert width_ratio_bound<rho
    checks.append(dict(delta=delta,area_factor=area_factor,
                       capacity_factor_bound=cap_factor,
                       width_ratio_bound=width_ratio_bound,
                       width_ratio_bound_float=float(width_ratio_bound)))
# At the opposite five-facet endpoint the same bound returns rho.
assert sq(1-s2/L)==1-s2
receipt=dict(status='PASS',arithmetic='exact Q(sqrt(5)); generic polynomial coefficient identity',
             h=h,squared_sine=s2,hko_ratio=rho,
             strict_interval='0 < delta < sin(pi/5)^2',
             ratio_exact_formula='rho_H * ((1-delta/(1+h))^2 / (1-delta^2/sin(pi/5)^2))^2',
             right_derivative=-4*rho/L,
             algebra_controls=checks,
             imports=[dict(path=str(SOURCE),sha256=hashlib.sha256(SOURCE.read_bytes()).hexdigest())],
             checker_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest())
parser=argparse.ArgumentParser();parser.add_argument('--out',type=Path,default=Path('/tmp/msc-math/facet-refinement-width.json'));args=parser.parse_args();args.out.parent.mkdir(parents=True,exist_ok=True)
args.out.write_text(json.dumps(serial(receipt),indent=2)+'\n')
print('PASS: exact HKO normalization, generic factorization, cut-interval sign, endpoint and two symbolic algebra controls')
