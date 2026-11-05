#!/usr/bin/env python3
"""Exact rational enclosure + general-position certificate; no third-party deps."""
import argparse
from fractions import Fraction as F
from itertools import combinations
from math import isqrt
from pathlib import Path
import json
import random

class I:
    def __init__(self, lo, hi=None):
        self.lo, self.hi = F(lo), F(lo if hi is None else hi)
    def __add__(self, other):
        other = other if isinstance(other, I) else I(other)
        return I(self.lo + other.lo, self.hi + other.hi)
    __radd__ = __add__
    def __neg__(self): return I(-self.hi, -self.lo)
    def __sub__(self, other): return self + (-other if isinstance(other,I) else -I(other))
    def __rsub__(self, other): return I(other) + (-self)
    def __mul__(self, other):
        other = other if isinstance(other,I) else I(other)
        p=[a*b for a in [self.lo,self.hi] for b in [other.lo,other.hi]]
        return I(min(p), max(p))
    __rmul__ = __mul__
    def __truediv__(self, other):
        other = other if isinstance(other,I) else I(other)
        if other.lo <= 0 <= other.hi: raise ValueError('zero interval denominator')
        return self * I(1/other.hi, 1/other.lo)
    def sqrt(self):
        if self.lo < 0: raise ValueError('negative radicand')
        den = 2**70
        lo = isqrt(self.lo.numerator*den*den//self.lo.denominator)
        hi = isqrt(self.hi.numerator*den*den//self.hi.denominator)+1
        return I(F(lo,den),F(hi,den))

def rank(a):
    a=[list(map(F,row)) for row in a]; row=0
    for col in range(len(a[0])):
        pivot=next((i for i in range(row,len(a)) if a[i][col]),None)
        if pivot is None: continue
        a[row],a[pivot]=a[pivot],a[row]; s=a[row][col]
        a[row]=[v/s for v in a[row]]
        for i in range(row+1,len(a)):
            s=a[i][col]; a[i]=[u-s*v for u,v in zip(a[i],a[row])]
        row+=1
        if row==len(a): break
    return row

def solve(a):
    a=[list(map(F,row))+[F(1)] for row in a]
    for j in range(4):
        p=next(i for i in range(j,4) if a[i][j]); a[j],a[p]=a[p],a[j]
        t=a[j][j]; a[j]=[v/t for v in a[j]]
        for i in range(4):
            if i!=j:
                t=a[i][j]; a[i]=[u-t*v for u,v in zip(a[i],a[j])]
    return tuple(row[-1] for row in a)

def main():
    p=argparse.ArgumentParser(); p.add_argument('--output-dir',type=Path,required=True); args=p.parse_args()
    args.output_dir.mkdir(parents=True,exist_ok=True)
    z=I(5).sqrt(); h=(1+z)/4
    co=[I(1),(z-1)/4,-(z+1)/4,-(z+1)/4,(z-1)/4]
    si=[I(0),(10+2*z).sqrt()/4,(10-2*z).sqrt()/4,-(10-2*z).sqrt()/4,-(10+2*z).sqrt()/4]
    b=[[co[i]/h,si[i]/h,I(0),I(0)] for i in range(5)]
    b += [[I(0),I(0),si[i]/h,-co[i]/h] for i in range(5)]
    den=2**20; rng=random.Random(20261016)
    nums=[[round((x.lo+x.hi)*den/2)+rng.randint(-9,9) for x in row] for row in b]
    a=[[F(x,den) for x in row] for row in nums]
    err=max(sum(max(abs(x-y.lo),abs(x-y.hi)) for x,y in zip(row,base)) for row,base in zip(a,b))
    eps=F(1,10000)
    if not err < eps: raise AssertionError('row error budget exceeded')
    full=sum(rank([a[i] for i in ids])==4 for ids in combinations(range(10),4))
    if full != 210: raise AssertionError('not in linear general position')
    vertices=[]; incidences=[]
    for ids in combinations(range(10),4):
        v=solve([a[i] for i in ids]); dots=[sum(x*y for x,y in zip(row,v)) for row in a]
        if max(dots)<=1:
            vertices.append(v); incidences.append([i for i,t in enumerate(dots) if t==1])
    if len(set(vertices))!=len(vertices): raise AssertionError('nonsimple or duplicate vertex')
    facet_counts=[]
    for j in range(10):
        vs=[v for v,ids in zip(vertices,incidences) if j in ids]
        if len(vs)<4 or rank([[x-y for x,y in zip(v,vs[0])] for v in vs[1:]])!=3:
            raise AssertionError('row is not a genuine facet')
        facet_counts.append(len(vs))
    sigma_lo=(3+z.lo)/5
    lower=sigma_lo*((1-eps)/(1+eps))**4
    if lower<=1: raise AssertionError('not certified counterexample')
    fixture={'coordinate_order':['q1','q2','p1','p2'],'denominator':den,'row_numerators':nums,'dual_vertices':[[float(x) for x in row] for row in a]}
    summary={'arithmetic':'fractions.Fraction + integer sqrt enclosures','all_210_four_row_subsets_independent':True,'genuine_facets':10,'vertices':len(vertices),'facet_vertex_counts':facet_counts,'row_l1_error_upper_bound':str(err),'chosen_error_bound':str(eps),'hko_ratio_lower_bound':str(sigma_lo),'inclusion_ratio_lower_bound':str(lower),'inclusion_ratio_lower_bound_display':float(lower),'non_affine_cartesian_product':True,'meaning':'analytic lower ratio bound via inclusion, not a capacity solver output'}
    (args.output_dir/'fixture.json').write_text(json.dumps(fixture,indent=2)+'\n')
    (args.output_dir/'certificate.json').write_text(json.dumps(summary,indent=2)+'\n')
    print(json.dumps({k:v for k,v in summary.items() if not k.endswith('bound')},indent=2))
if __name__=='__main__': main()
